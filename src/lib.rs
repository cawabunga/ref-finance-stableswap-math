mod ref_finance;
mod utils;

use crate::ref_finance::{AdminFees, StableSwapPool};
use crate::utils::amounts_to_c_amounts;
use js_sys::BigInt;
use near_sdk::json_types::ValidAccountId;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = getAmountOut)]
pub fn get_amount_out(
    token_decimals: Vec<u8>,
    amounts: Vec<BigInt>,
    amp_factor: BigInt,
    total_fee: u32,
    token_in_index: usize,
    token_out_index: usize,
    amount_in: BigInt,
) -> BigInt {
    let token_account_ids: Vec<ValidAccountId> = token_decimals
        .iter()
        .enumerate()
        .map(|(i, _)| format!("token_{}", i).try_into().unwrap())
        .collect();

    let amounts = amounts
        .into_iter()
        .map(|bi| format!("{}", bi).parse().unwrap())
        .collect::<Vec<u128>>();

    let amp_factor = format!("{}", amp_factor).parse().unwrap();
    let amount_in = format!("{}", amount_in).parse().unwrap();

    let c_amounts = amounts_to_c_amounts(&amounts, &token_decimals);
    let mut pool = StableSwapPool::new(
        token_account_ids.clone(),
        token_decimals,
        amp_factor,
        total_fee,
    );
    pool.c_amounts = c_amounts;

    let amount_out = pool.swap(
        &token_account_ids
            .get(token_in_index)
            .unwrap()
            .clone()
            .into(),
        amount_in,
        &token_account_ids
            .get(token_out_index)
            .unwrap()
            .clone()
            .into(),
        &AdminFees::new(0),
    );

    amount_out.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_get_amount_out() {
        let result = get_amount_out(
            vec![6, 6, 18],
            vec![
                BigInt::from(719240775791_u64),
                BigInt::from(485261247671_u64),
                "990759998116457852477754".parse().unwrap(),
            ],
            BigInt::from(240),
            5,
            0,
            1,
            BigInt::from(10000000000_u64),
        );

        assert_eq!(result, 9992301437_u64);
    }
}
