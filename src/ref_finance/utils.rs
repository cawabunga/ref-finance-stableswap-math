use near_sdk::json_types::U128;
use uint::construct_uint;

/// Fee divisor, allowing to provide fee in bps.
pub const FEE_DIVISOR: u32 = 10_000;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

/// Volume of swap on the given token.
pub struct SwapVolume {
    pub input: U128,
    pub output: U128,
}

impl Default for SwapVolume {
    fn default() -> Self {
        Self {
            input: U128(0),
            output: U128(0),
        }
    }
}

pub fn u128_ratio(a: u128, num: u128, denom: u128) -> u128 {
    (U256::from(a) * U256::from(num) / U256::from(denom)).as_u128()
}
