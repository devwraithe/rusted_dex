use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("5g41rWoDcyNQm8TR2zpbfTH3P2ZyVTfBjeQ4Rk3cN2az");

const DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod rusted_dex {
    use anchor_spl::token;

    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        token_b: Pubkey,
        token_a: Pubkey,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        pool.token_a = token_a;
        pool.token_b = token_b;

        let balance = ctx.accounts.pool.to_account_info().lamports();
        msg!("balance in lamports on pool initialization is {}", balance);

        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let user_token_a = &mut ctx.accounts.user_token_a;
        let user_token_b = &mut ctx.accounts.user_token_b;
        let pool_token_a = &mut ctx.accounts.pool_token_a;
        let pool_token_b = &mut ctx.accounts.pool_token_b;
        let lp_token_mint = &mut ctx.accounts.lp_token_mint;
        let user_lp_token = &mut ctx.accounts.user_lp_token;

        let reserve_a = pool.reserve_a as u128;
        let reserve_b = pool.reserve_b as u128;

        if reserve_a > 0 && reserve_b > 0 {
            let ratio_a = (amount_a as u128) * reserve_b;
            let ratio_b = (amount_b as u128) * reserve_a;

            if ratio_a != ratio_b {
                return Err(error!(ErrorCode::InvalidLiquidityRatio));
            }
        }

        let trf_a = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: user_token_a.to_account_info(),
                to: pool_token_a.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );

        let trf_b = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: user_token_b.to_account_info(),
                to: pool_token_b.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );

        let trf_mint = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: lp_token_mint.to_account_info(),
                to: user_lp_token.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );

        let _ = token::transfer(trf_a, amount_a)?;
        let _ = token::transfer(trf_b, amount_b)?;

        pool.reserve_a = amount_a;
        pool.reserve_b = amount_b;

        let lp_tokens_to_mint = if reserve_a == 0 || reserve_b == 0 {
            amount_a + amount_b
        } else {
            let total_supply = lp_token_mint.supply;
            ((amount_a as u128 * total_supply as u128) / reserve_a) as u64
        };

        let _ = token::mint_to(trf_mint, lp_tokens_to_mint)?;

        Ok(())
    }

    pub fn swap_token(ctx: Context<SwapToken>, params: SwapTokenParams) -> Result<()> {
        let amount_in = params.amount_in;
        let minimum_output_amount = params.minimum_output_amount;

        let pool_input_balance = ctx.accounts.pool_input_token_account.amount;
        let pool_output_balance = ctx.accounts.pool_output_token_account.amount;

        require!(
            pool_input_balance > 0 && pool_output_balance > 0,
            ErrorCode::InsufficientLiquidity
        );

        let amount_out = (pool_output_balance as u128 * amount_in as u128)
            / (pool_input_balance as u128 + amount_in as u128);

        require!(
            amount_out as u64 >= minimum_output_amount,
            ErrorCode::SlippageExceeded
        );

        let amount_out = amount_out as u64;

        let cpi_input_accounts = token::Transfer {
            from: ctx.accounts.user_input_token_account.to_account_info(),
            to: ctx.accounts.pool_input_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_input_accounts,
        );

        token::transfer(cpi_context, amount_in)?;

        let cpi_output_accounts = token::Transfer {
            from: ctx.accounts.pool_output_token_account.to_account_info(),
            to: ctx.accounts.user_output_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_output_accounts,
        );

        token::transfer(cpi_context, amount_out)?;

        let balance = ctx.accounts.pool.to_account_info().lamports();
        msg!("balance in lamports after swapping token is {}", balance);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, seeds = [b"pool", user.key().as_ref()], bump, payer = user, space = PoolState::SPACE)]
    pub pool: Account<'info, PoolState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut, seeds = [b"pool", user.key().as_ref()], bump)]
    pub pool: Account<'info, PoolState>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_lp_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SwapToken<'info> {
    #[account(mut, seeds = [b"pool", user.key().as_ref()], bump)]
    pub pool: Account<'info, PoolState>,
    #[account(mut)]
    pub user: Signer<'info>, // pool_authority
    #[account(mut)]
    pub user_input_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_output_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_input_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_output_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct SwapTokenParams {
    pub amount_in: u64,
    pub minimum_output_amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct PoolState {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
}

impl PoolState {
    pub const SPACE: usize = DISCRIMINATOR_SIZE + PoolState::INIT_SPACE;
}

#[error_code]
pub enum ErrorCode {
    #[msg("invalid liquidity ratio")]
    InvalidLiquidityRatio,
    #[msg("insufficient liquidity in the pool")]
    InsufficientLiquidity,
    #[msg("slippage exceeded the allowed minimum output.")]
    SlippageExceeded,
}
