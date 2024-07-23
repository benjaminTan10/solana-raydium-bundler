use std::str::FromStr;

use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address;

use crate::env::PoolDataSettings;

use super::{
    instruction::{swap, AmmKeys, MarketPubkeys},
    pool_ixs::AMM_PROGRAM,
};

pub fn swap_ixs(
    server_data: PoolDataSettings,
    amm_keys: AmmKeys,
    market_keys: MarketPubkeys,
    wallet: &Keypair,
    amount_in: u64,
    out: bool,
    user_token_source: Pubkey,
) -> eyre::Result<Instruction> {
    // let buyer_keypair = Keypair::from_base58_string(&server_data.buyer_key);
    let buyer_wallet = wallet;
    // if out {
    //     buyer_wallet = &buyer_keypair;
    // }

    let user_token_destination = get_associated_token_address(
        &buyer_wallet.pubkey(),
        &Pubkey::from_str(&server_data.token_mint)?,
    );

    // build swap instruction
    let build_swap_instruction = swap(
        &AMM_PROGRAM,
        &amm_keys,
        &market_keys,
        &buyer_wallet.pubkey(),
        &user_token_source,
        &user_token_destination,
        amount_in,
        1,
        out,
    )?;

    Ok(build_swap_instruction)
}
