use std::{fs::File, io::Write};

use solana_address_lookup_table_program::instruction::create_lookup_table;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::env::PoolDataSettings;

pub async fn create_lut(mut pool_data: PoolDataSettings) -> eyre::Result<(Instruction, Pubkey)> {
    println!("Creating LUT");
    let buyer_key = Keypair::from_base58_string(&pool_data.buyer_key);

    let rpc_client = RpcClient::new(pool_data.rpc_url.clone());

    let recent_slot = match rpc_client.get_slot().await {
        Ok(slot) => slot,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e.into());
        }
    };

    let (lut, lut_key) = create_lookup_table(buyer_key.pubkey(), buyer_key.pubkey(), recent_slot);

    pool_data.lut_key = lut_key.to_string();
    let mut file = File::create("settings.json")?;
    file.write_all(serde_json::to_string_pretty(&pool_data)?.as_bytes())?;

    Ok((lut, lut_key))
}
