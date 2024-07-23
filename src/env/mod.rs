pub mod input;
pub mod jito_auth;

use std::{
    fs::{self, File},
    io::Write,
};

use input::{mint_input, private_key_input};
use log::info;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct BackrunAccount {
    pub id: String,
    pub account: Pubkey,
}

#[derive(Debug, Clone, Serialize)]
pub struct PoolDataSettings {
    #[serde(rename = "RPC-URL")]
    pub rpc_url: String,

    #[serde(rename = "BLOCK-ENGINE-URL")]
    pub block_engine_url: String,

    #[serde(rename = "DEPLOYER-PRIVATE-KEY")]
    pub deployer_key: String,

    #[serde(rename = "BUYER-PRIVATE-KEY")]
    pub buyer_key: String,

    #[serde(rename = "TOKEN-MINT")]
    pub token_mint: String,

    #[serde(rename = "MARKET-ADDRESS")]
    pub market_id: String,

    #[serde(rename = "POOL-ID")]
    pub pool_id: String,

    #[serde(rename = "LUT-KEY")]
    pub lut_key: String,

    #[serde(rename = "VOLUME-LUT-KEY")]
    pub volume_lut_key: String,
}

#[derive(Deserialize, Serialize, Clone, Default)]
struct HelperSettings {
    #[serde(rename = "RPC-URL")]
    rpc_url: String,

    #[serde(rename = "BLOCK-ENGINE-URL")]
    pub block_engine_url: String,

    #[serde(rename = "DEPLOYER-PRIVATE-KEY")]
    deployer_key: String,

    #[serde(rename = "BUYER-PRIVATE-KEY")]
    buyer_key: String,

    #[serde(rename = "TOKEN-MINT")]
    token_mint: String,

    #[serde(rename = "MARKET-ADDRESS")]
    market_id: String,

    #[serde(rename = "POOL-ID")]
    pool_id: String,

    #[serde(rename = "LUT-KEY")]
    lut_key: String,

    #[serde(rename = "VOLUME-LUT-KEY")]
    volume_lut_key: String,
}

pub async fn load_minter_settings() -> eyre::Result<PoolDataSettings> {
    let args = match fs::read_to_string("settings.json") {
        Ok(args) => args,
        Err(_) => {
            info!("Settings file not found, creating a new one");
            // Create a new settings.json file with default settings
            let default_settings = HelperSettings {
                rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
                block_engine_url: "https://ny.mainnet.block-engine.jito.wtf".to_string(),
                deployer_key: "".to_string(),
                buyer_key: "".to_string(),
                token_mint: "".to_string(),
                market_id: "".to_string(),
                pool_id: "".to_string(),
                lut_key: "".to_string(),
                volume_lut_key: "".to_string(),
            };
            let default_settings_json = serde_json::to_string_pretty(&default_settings).unwrap();
            let mut file = File::create("settings.json").unwrap();
            file.write_all(default_settings_json.as_bytes()).unwrap();

            "".to_string()
        }
    };

    let mut helper_settings: HelperSettings = match serde_json::from_str(&args) {
        Ok(settings) => settings,
        Err(_) => {
            // If the file is empty, use default settings
            HelperSettings::default()
        }
    };

    // If any field is empty, ask the user to fill it
    if helper_settings.deployer_key.is_empty() {
        helper_settings.deployer_key = private_key_input("Deployer Private Key").await.unwrap();
    }
    if helper_settings.buyer_key.is_empty() {
        helper_settings.buyer_key = private_key_input("Buyer Private Key").await.unwrap();
    }
    if helper_settings.token_mint.is_empty() {
        helper_settings.token_mint = (mint_input("Token Mint").await).to_string();
    }
    if helper_settings.market_id.is_empty() {
        helper_settings.market_id = (mint_input("Market ID").await).to_string();
    }

    // Save the updated settings to the file
    let default_settings_json = serde_json::to_string_pretty(&helper_settings).unwrap();
    let mut file = File::create("settings.json").unwrap();
    file.write_all(default_settings_json.as_bytes()).unwrap();

    Ok(PoolDataSettings {
        rpc_url: helper_settings.rpc_url,
        block_engine_url: helper_settings.block_engine_url,
        market_id: helper_settings.market_id,
        token_mint: helper_settings.token_mint,
        deployer_key: helper_settings.deployer_key,
        buyer_key: helper_settings.buyer_key,
        pool_id: helper_settings.pool_id,
        lut_key: helper_settings.lut_key,
        volume_lut_key: helper_settings.volume_lut_key,
    })
}
