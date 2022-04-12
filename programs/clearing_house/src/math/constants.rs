// PRECISIONS
pub const AMM_RESERVE_PRECISION: u128 = 10_000_000_000_000; //expo = -13;
pub const MARK_PRICE_PRECISION: u128 = 10_000_000_000; //expo = -10
pub const QUOTE_PRECISION: u128 = 1_000_000; // expo = -6
pub const FUNDING_PAYMENT_PRECISION: u128 = 10_000; // expo = -4
pub const MARGIN_PRECISION: u128 = 10_000; // expo = -4
pub const PEG_PRECISION: u128 = 1_000; //expo = -3
pub const PRICE_SPREAD_PRECISION: i128 = 10_000; // expo = -4
pub const PRICE_SPREAD_PRECISION_U128: u128 = 10_000; // expo = -4

// PRECISION CONVERSIONS
pub const PRICE_TO_PEG_PRECISION_RATIO: u128 = MARK_PRICE_PRECISION / PEG_PRECISION; // expo: 7
pub const PRICE_TO_PEG_QUOTE_PRECISION_RATIO: u128 = MARK_PRICE_PRECISION / QUOTE_PRECISION; // expo: 4
pub const AMM_TO_QUOTE_PRECISION_RATIO: u128 = AMM_RESERVE_PRECISION / QUOTE_PRECISION; // expo: 7
pub const AMM_TO_QUOTE_PRECISION_RATIO_I128: i128 =
    (AMM_RESERVE_PRECISION / QUOTE_PRECISION) as i128; // expo: 7
pub const AMM_TIMES_PEG_TO_QUOTE_PRECISION_RATIO: u128 =
    AMM_RESERVE_PRECISION * PEG_PRECISION / QUOTE_PRECISION; // expo: 10
pub const QUOTE_TO_BASE_AMT_FUNDING_PRECISION: i128 =
    (AMM_RESERVE_PRECISION * MARK_PRICE_PRECISION * FUNDING_PAYMENT_PRECISION / QUOTE_PRECISION)
        as i128; // expo: 21
pub const PRICE_TO_QUOTE_PRECISION_RATIO: u128 = MARK_PRICE_PRECISION / QUOTE_PRECISION; // expo: 4
pub const MARK_PRICE_TIMES_AMM_TO_QUOTE_PRECISION_RATIO: u128 =
    MARK_PRICE_PRECISION * AMM_TO_QUOTE_PRECISION_RATIO; // expo 17

pub const FUNDING_EXCESS_TO_QUOTE_RATIO: i128 =
    (MARK_PRICE_PRECISION * AMM_RESERVE_PRECISION / QUOTE_PRECISION) as i128; // expo 11

pub const AMM_TIMES_PEG_PRECISION: i128 = (AMM_RESERVE_PRECISION * PEG_PRECISION) as i128; // expo 16
pub const AMM_RESERVE_PRECISION_I128: i128 = (AMM_RESERVE_PRECISION) as i128;

// FEE REBATES
pub const SHARE_OF_FEES_ALLOCATED_TO_CLEARING_HOUSE_NUMERATOR: u128 = 1;
pub const SHARE_OF_FEES_ALLOCATED_TO_CLEARING_HOUSE_DENOMINATOR: u128 = 2;
pub const UPDATE_K_ALLOWED_PRICE_CHANGE: u128 = MARK_PRICE_PRECISION / 100_000; //.00001

// TIME PERIODS
pub const ONE_HOUR: i64 = 3600;
pub const TWENTYFOUR_HOUR: i64 = 3600 * 24;

// FEES
pub const DEFAULT_FEE_NUMERATOR: u128 = 10;
pub const DEFAULT_FEE_DENOMINATOR: u128 = 10000;
pub const DEFAULT_DISCOUNT_TOKEN_FIRST_TIER_MINIMUM_BALANCE: u64 = 1_000_000_000_000; // 1000
pub const DEFAULT_DISCOUNT_TOKEN_FIRST_TIER_DISCOUNT_NUMERATOR: u128 = 20;
pub const DEFAULT_DISCOUNT_TOKEN_FIRST_TIER_DISCOUNT_DENOMINATOR: u128 = 100;
pub const DEFAULT_DISCOUNT_TOKEN_SECOND_TIER_MINIMUM_BALANCE: u64 = 100_000_000_000;
pub const DEFAULT_DISCOUNT_TOKEN_SECOND_TIER_DISCOUNT_NUMERATOR: u128 = 15;
pub const DEFAULT_DISCOUNT_TOKEN_SECOND_TIER_DISCOUNT_DENOMINATOR: u128 = 100;
pub const DEFAULT_DISCOUNT_TOKEN_THIRD_TIER_MINIMUM_BALANCE: u64 = 10_000_000_000;
pub const DEFAULT_DISCOUNT_TOKEN_THIRD_TIER_DISCOUNT_NUMERATOR: u128 = 10;
pub const DEFAULT_DISCOUNT_TOKEN_THIRD_TIER_DISCOUNT_DENOMINATOR: u128 = 100;
pub const DEFAULT_DISCOUNT_TOKEN_FOURTH_TIER_MINIMUM_BALANCE: u64 = 1_000_000_000;
pub const DEFAULT_DISCOUNT_TOKEN_FOURTH_TIER_DISCOUNT_NUMERATOR: u128 = 5;
pub const DEFAULT_DISCOUNT_TOKEN_FOURTH_TIER_DISCOUNT_DENOMINATOR: u128 = 100;
pub const DEFAULT_REFERRER_REWARD_NUMERATOR: u128 = 5;
pub const DEFAULT_REFERRER_REWARD_DENOMINATOR: u128 = 100;
pub const DEFAULT_REFEREE_DISCOUNT_NUMERATOR: u128 = 5;
pub const DEFAULT_REFEREE_DISCOUNT_DENOMINATOR: u128 = 100;

// CONSTRAINTS
pub const MAX_LIQUIDATION_SLIPPAGE: i128 = 100; // expo = -2
pub const MAX_LIQUIDATION_SLIPPAGE_U128: u128 = 100; // expo = -2
pub const MAX_MARK_TWAP_DIVERGENCE: u128 = 5_000; // expo = -3
pub const MAXIMUM_MARGIN_RATIO: u32 = MARGIN_PRECISION as u32;
pub const MINIMUM_MARGIN_RATIO: u32 = MARGIN_PRECISION as u32 / 50;

// FORMULAIC REPEG / K
pub const K_PCT_SCALE: i128 = 10000; // expo = -4

// hardcoded scale bounds for a single update (.1% increase and .09% decrease)
pub const K_PCT_LOWER_BOUND: i128 = 9991;
pub const K_PCT_UPPER_BOUND: i128 = 10010;
