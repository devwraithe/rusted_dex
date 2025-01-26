use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
    },
    Client, Cluster, Program,
};

pub fn initialize_client() -> (Client<&'static Keypair>, &'static Keypair) {
    let anchor_wallet =
        std::env::var("ANCHOR_WALLET").expect("Environment variable ANCHOR WALLET is not set");

    let payer: &'static Keypair = Box::leak(Box::new(
        read_keypair_file(&anchor_wallet).expect("Failed to read keypair file"),
    ));

    let client = Client::new_with_options(Cluster::Localnet, payer, CommitmentConfig::confirmed());

    (client, payer)
}

pub fn get_program(client: &Client<&'static Keypair>) -> Program<&'static Keypair> {
    client
        .program(rusted_dex::ID)
        .expect("Failed to initialize program client")
}

pub fn setup_pool_addresses() -> (
    Pubkey,
    Pubkey,
    Pubkey,
    &'static Keypair,
    Program<&'static Keypair>,
) {
    let (client, payer) = initialize_client();
    let _program = get_program(&client);

    let token_a = Pubkey::new_unique();
    let token_b = Pubkey::new_unique();

    let (pool_addr, _) =
        Pubkey::find_program_address(&[b"pool", payer.pubkey().as_ref()], &rusted_dex::ID);

    (pool_addr, token_a, token_b, payer, _program)
}
