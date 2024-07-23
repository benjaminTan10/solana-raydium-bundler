use std::str::FromStr;

use rand::{rngs::StdRng, Rng, SeedableRng};
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signature::Keypair, system_instruction,
};

pub fn auth_keypair() -> Keypair {
    let bytes_auth_vec = vec![
        170, 102, 199, 216, 226, 201, 23, 43, 26, 120, 207, 73, 110, 164, 116, 178, 255, 140, 255,
        218, 189, 56, 60, 156, 217, 54, 187, 126, 163, 9, 162, 105, 7, 82, 19, 78, 31, 45, 211, 21,
        169, 244, 1, 88, 110, 145, 211, 13, 133, 99, 16, 32, 105, 253, 55, 213, 94, 124, 237, 195,
        235, 255, 7, 72,
    ];
    let bytes_auth = bytes_auth_vec.as_slice();
    let auth_keypair = Keypair::from_bytes(bytes_auth).unwrap();
    auth_keypair
}

pub fn jito_tip_inx(source: Pubkey, destination: Pubkey, priority: u64) -> Instruction {
    let ix = system_instruction::transfer(&source, &destination, priority);
    ix
}

pub fn tip_txn(source: Pubkey, destination: Pubkey, priority: u64) -> Instruction {
    let ix = system_instruction::transfer(&source, &destination, priority);
    ix
}

pub fn tip_program_id() -> Pubkey {
    let auth = Pubkey::from_str("T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt").unwrap();

    return auth;
}

pub fn jito_tip_acc() -> Pubkey {
    let tip_accounts = generate_tip_accounts(&tip_program_id());
    let mut rng = StdRng::from_entropy();
    let jito_tip_acc = tip_accounts[rng.gen_range(0..tip_accounts.len())];

    return jito_tip_acc;
}

pub fn generate_tip_accounts(tip_program_pubkey: &Pubkey) -> Vec<Pubkey> {
    let tip_pda_0 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_0"], tip_program_pubkey).0;
    let tip_pda_1 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_1"], tip_program_pubkey).0;
    let tip_pda_2 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_2"], tip_program_pubkey).0;
    let tip_pda_3 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_3"], tip_program_pubkey).0;
    let tip_pda_4 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_4"], tip_program_pubkey).0;
    let tip_pda_5 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_5"], tip_program_pubkey).0;
    let tip_pda_6 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_6"], tip_program_pubkey).0;
    let tip_pda_7 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_7"], tip_program_pubkey).0;

    vec![
        tip_pda_0, tip_pda_1, tip_pda_2, tip_pda_3, tip_pda_4, tip_pda_5, tip_pda_6, tip_pda_7,
    ]
}
