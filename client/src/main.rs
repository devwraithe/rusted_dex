use anchor_client::{Client, Cluster};
use clap::{Parser, Subcommand};
use rusted_dex::{accounts, instruction};
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    signer::{EncodableKey, Signer},
    system_instruction, system_program, transaction,
};
use std::{error::Error, rc::Rc};

#[derive(Parser)]
#[command(name = "rusted_dex")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    InitializePool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // create client
    let cluster = Cluster::Devnet;
    let supplier = Rc::new(Keypair::read_from_file(
        "/Users/admin/.config/solana/id.json",
    )?);
    let payer = Rc::new(Keypair::new());
    let client = Client::new(cluster, payer.clone());
    add_sol(&client, &supplier, &payer.pubkey())?;

    // create program
    let program = client.program(rusted_dex::ID)?;

    let (pool_addr, _) =
        Pubkey::find_program_address(&[b"pool", payer.pubkey().as_ref()], &rusted_dex::ID);
    let token_a = Pubkey::new_unique();
    let token_b = Pubkey::new_unique();

    match &cli.command {
        Commands::InitializePool => {
            println!("initializing the pool...");

            // initialize pool
            let init_tx = program
                .request()
                .accounts(accounts::InitializePool {
                    pool: pool_addr,
                    user: program.payer(),
                    system_program: system_program::ID,
                })
                .args(instruction::InitializePool { token_a, token_b })
                .signer(&payer)
                .send()
                .map_err(|e| format!("❌initialize pool failed: {}", e))?;
            println!("✅initialize pool successful");

            let tx_url = format!("https://explorer.solana.com/tx/{}?cluster=devnet", init_tx);
            println!("🔗init pool url: {}", tx_url);
        }
    }

    Ok(())
}

fn add_sol(
    client: &Client<Rc<Keypair>>,
    sender: &Keypair,
    recipient: &Pubkey,
) -> Result<(), Box<dyn Error>> {
    let one_sol = 1_000_000_000.0;
    let amount = 100_000_000;

    println!(
        "transferring {} SOL to {}...",
        amount as f64 / one_sol,
        recipient
    );

    let program = client.program(rusted_dex::ID)?;

    let transaction = {
        let blockhash = program.rpc().get_latest_blockhash()?;
        let instructions = vec![system_instruction::transfer(
            &sender.pubkey(),
            recipient,
            amount,
        )];

        transaction::Transaction::new_signed_with_payer(
            &instructions,
            Some(&sender.pubkey()),
            &[sender],
            blockhash,
        )
    };

    let signature = program.rpc().send_and_confirm_transaction(&transaction)?;

    let sender_balance = program.rpc().get_balance(&sender.pubkey())?;
    let recipient_balance = program.rpc().get_balance(recipient)?;

    println!(
        "✅transferred {} SOL from {} to {}",
        amount as f64 / one_sol,
        sender.pubkey(),
        recipient
    );
    println!(
        "sender's balance: {} lamports",
        sender_balance as f64 / one_sol
    );
    println!(
        "recipient's balance: {} lamports",
        recipient_balance as f64 / one_sol
    );

    let tx_url = format!(
        "https://explorer.solana.com/tx/{}?cluster=devnet",
        signature,
    );
    println!("🔗tx sol url: {}", tx_url);

    Ok(())
}
