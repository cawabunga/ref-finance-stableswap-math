use crate::ref_finance::TARGET_DECIMAL;

pub fn amounts_to_c_amounts(amounts: &Vec<u128>, token_decimals: &Vec<u8>) -> Vec<u128> {
    let mut c_amounts = amounts.clone();
    for (index, value) in token_decimals.iter().enumerate() {
        if *value <= TARGET_DECIMAL {
            let factor = 10_u128
                .checked_pow((TARGET_DECIMAL - value) as u32)
                .unwrap();
            c_amounts[index] = c_amounts[index].checked_mul(factor).unwrap();
        } else {
            let factor = 10_u128
                .checked_pow((value - TARGET_DECIMAL) as u32)
                .unwrap();
            c_amounts[index] = c_amounts[index].checked_div(factor).unwrap();
        }
    }
    c_amounts
}
