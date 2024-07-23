use serde::{Deserialize, Serialize};
use solana_program::pubkey;
use solana_sdk::pubkey::Pubkey;

pub const SOLC_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct LIQUIDITY_STATE_LAYOUT_V4 {
    pub status: u64,
    pub nonce: u64,
    pub maxOrder: u64,
    pub depth: u64,
    pub baseDecimal: u64,
    pub quoteDecimal: u64,
    pub state: u64,
    pub resetFlag: u64,
    pub minSize: u64,
    pub volMaxCutRatio: u64,
    pub amountWaveRatio: u64,
    pub baseLotSize: u64,
    pub quoteLotSize: u64,
    pub minPriceMultiplier: u64,
    pub maxPriceMultiplier: u64,
    pub systemDecimalValue: u64,
    pub minSeparateNumerator: u64,
    pub minSeparateDenominator: u64,
    pub tradeFeeNumerator: u64,
    pub tradeFeeDenominator: u64,
    pub pnlNumerator: u64,
    pub pnlDenominator: u64,
    pub swapFeeNumerator: u64,
    pub swapFeeDenominator: u64,
    pub baseNeedTakePnl: u64,
    pub quoteNeedTakePnl: u64,
    pub quoteTotalPnl: u64,
    pub baseTotalPnl: u64,
    pub poolOpenTime: u64,
    pub punishPcAmount: u64,
    pub punishCoinAmount: u64,
    pub orderbookToInitTime: u64,
    // u128('poolTotalDepositPc'),
    // u128('poolTotalDepositCoin'),
    pub swapBaseInAmount: u128,
    pub swapQuoteOutAmount: u128,
    pub swapBase2QuoteFee: u64,
    pub swapQuoteInAmount: u128,
    pub swapBaseOutAmount: u128,
    pub swapQuote2BaseFee: u64,
    // amm vault
    pub baseVault: Pubkey,
    pub quoteVault: Pubkey,
    // mint
    pub baseMint: Pubkey,
    pub quoteMint: Pubkey,
    pub lpMint: Pubkey,
    // market
    pub openOrders: Pubkey,
    pub marketId: Pubkey,
    pub marketProgramId: Pubkey,
    pub targetOrders: Pubkey,
    pub withdrawQueue: Pubkey,
    pub lpVault: Pubkey,
    pub owner: Pubkey,
    // true circulating supply without lock up
    pub lpReserve: u64,
    pub padding: [u64; 3],
}

impl LIQUIDITY_STATE_LAYOUT_V4 {
    pub fn decode(input: &mut &[u8]) -> eyre::Result<Self> {
        let mut s = Self::default();
        s.status = Self::unpack_u64(input)?;
        s.nonce = Self::unpack_u64(input)?;
        s.maxOrder = Self::unpack_u64(input)?;
        s.depth = Self::unpack_u64(input)?;
        s.baseDecimal = Self::unpack_u64(input)?;
        s.quoteDecimal = Self::unpack_u64(input)?;
        s.state = Self::unpack_u64(input)?;
        s.resetFlag = Self::unpack_u64(input)?;
        s.minSize = Self::unpack_u64(input)?;
        s.volMaxCutRatio = Self::unpack_u64(input)?;
        s.amountWaveRatio = Self::unpack_u64(input)?;
        s.baseLotSize = Self::unpack_u64(input)?;
        s.quoteLotSize = Self::unpack_u64(input)?;
        s.minPriceMultiplier = Self::unpack_u64(input)?;
        s.maxPriceMultiplier = Self::unpack_u64(input)?;
        s.systemDecimalValue = Self::unpack_u64(input)?;
        s.minSeparateNumerator = Self::unpack_u64(input)?;
        s.minSeparateDenominator = Self::unpack_u64(input)?;
        s.tradeFeeNumerator = Self::unpack_u64(input)?;
        s.tradeFeeDenominator = Self::unpack_u64(input)?;
        s.pnlNumerator = Self::unpack_u64(input)?;
        s.pnlDenominator = Self::unpack_u64(input)?;
        s.swapFeeNumerator = Self::unpack_u64(input)?;
        s.swapFeeDenominator = Self::unpack_u64(input)?;
        s.baseNeedTakePnl = Self::unpack_u64(input)?;
        s.quoteNeedTakePnl = Self::unpack_u64(input)?;
        s.quoteTotalPnl = Self::unpack_u64(input)?;
        s.baseTotalPnl = Self::unpack_u64(input)?;
        s.poolOpenTime = Self::unpack_u64(input)?;
        s.punishPcAmount = Self::unpack_u64(input)?;
        s.punishCoinAmount = Self::unpack_u64(input)?;
        s.orderbookToInitTime = Self::unpack_u64(input)?;
        // u128('poolTotalDepositPc'),
        // u128('poolTotalDepositCoin'),
        s.swapBaseInAmount = Self::unpack_u128(input)?;
        s.swapQuoteOutAmount = Self::unpack_u128(input)?;
        s.swapBase2QuoteFee = Self::unpack_u64(input)?;
        s.swapQuoteInAmount = Self::unpack_u128(input)?;
        s.swapBaseOutAmount = Self::unpack_u128(input)?;
        s.swapQuote2BaseFee = Self::unpack_u64(input)?;
        // amm vault

        s.baseVault = Self::unpack_pubkey(input)?;
        s.quoteVault = Self::unpack_pubkey(input)?;
        // mint
        s.baseMint = Self::unpack_pubkey(input)?;
        s.quoteMint = Self::unpack_pubkey(input)?;
        s.lpMint = Self::unpack_pubkey(input)?;
        // market
        s.openOrders = Self::unpack_pubkey(input)?;
        s.marketId = Self::unpack_pubkey(input)?;
        s.marketProgramId = Self::unpack_pubkey(input)?;
        s.targetOrders = Self::unpack_pubkey(input)?;
        s.withdrawQueue = Self::unpack_pubkey(input)?;
        s.lpVault = Self::unpack_pubkey(input)?;
        s.owner = Self::unpack_pubkey(input)?;
        // true circulating supply without lock up
        s.lpReserve = Self::unpack_u64(input)?;
        s.padding = [
            Self::unpack_u64(input)?,
            Self::unpack_u64(input)?,
            Self::unpack_u64(input)?,
        ];
        Ok(s)
    }
    fn unpack_u64(input: &mut &[u8]) -> eyre::Result<u64> {
        use std::io::Read;

        let mut buf = [0u8; 8];
        input.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    fn unpack_u128(input: &mut &[u8]) -> eyre::Result<u128> {
        use std::io::Read;

        let mut buf = [0u8; 16];
        input.read_exact(&mut buf)?;
        Ok(u128::from_le_bytes(buf))
    }
    fn unpack_pubkey(input: &mut &[u8]) -> eyre::Result<Pubkey> {
        use std::io::Read;

        let mut buf = [0u8; 32];
        input.read_exact(&mut buf)?;
        Ok(Pubkey::new_from_array(buf))
    }
}

pub async fn program_address(program_id: &Pubkey) -> eyre::Result<Pubkey> {
    let buffer = vec![97, 109, 109, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121];
    let seeds = &[&buffer[..]];

    let (key, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    Ok(key)
}
