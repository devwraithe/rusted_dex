// // add liquidity
// let amount_a = 1000;
// let amount_b = 2000;
// program
//     .request()
//     .accounts(accounts::AddLiquidity {
//         pool: pool_account.pubkey(),
//         user: payer.pubkey(),
//     })
//     .args(instruction::AddLiquidity { amount_a, amount_b })
//     .signer(&payer)
//     .send()?;
// println!("✅add liquidity successful");