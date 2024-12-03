use anchor_lang::prelude::*;

declare_id!("5sRS7S7T8CR4LfeqHYJSBw1smKoeNmEEM3XLP1jhcLGZ");

#[program]
pub mod rusted_dex {
    use super::*;

    // creates a pool liquidity for two tokens
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        token_a: Pubkey,
        token_b: Pubkey,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.token_a = token_a; // access pool account
        pool.token_b = token_b;
        pool.reserve_a = 0;
        pool.reserve_b = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 8 + 8)]
    pub pool: Account<'info, Pool>, // new pool account
    #[account(mut)]
    pub user: Signer<'info>, // user initializing the pool
    pub system_program: Program<'info, System>, // required for account creation
}

#[account]
pub struct Pool {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
}
