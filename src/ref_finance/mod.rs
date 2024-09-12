use near_sdk::json_types::ValidAccountId;
use near_sdk::{AccountId, Balance, Timestamp};

pub use crate::ref_finance::admin_fee::AdminFees;
use crate::ref_finance::errors::*;
use crate::ref_finance::math::{Fees, StableSwap, SwapResult, MAX_AMP, MIN_AMP};
use crate::ref_finance::utils::FEE_DIVISOR;

mod admin_fee;
mod errors;
mod math;
mod utils;

pub const MIN_DECIMAL: u8 = 1;
pub const MAX_DECIMAL: u8 = 24;
pub const TARGET_DECIMAL: u8 = 18;
pub const MIN_RESERVE: u128 = 1_000_000_000_000_000;

pub struct StableSwapPool {
    /// List of tokens in the pool.
    pub token_account_ids: Vec<AccountId>,
    /// Each decimals for tokens in the pool
    pub token_decimals: Vec<u8>,
    /// token amounts in comparable decimal.
    pub c_amounts: Vec<Balance>,
    /// Fee charged for swap (gets divided by FEE_DIVISOR).
    pub total_fee: u32,
    /// Initial amplification coefficient.
    pub init_amp_factor: u128,
    /// Target for ramping up amplification coefficient.
    pub target_amp_factor: u128,
    /// Initial amplification time.
    pub init_amp_time: Timestamp,
    /// Stop ramp up amplification time.
    pub stop_amp_time: Timestamp,
}

impl StableSwapPool {
    pub fn new(
        token_account_ids: Vec<ValidAccountId>,
        token_decimals: Vec<u8>,
        amp_factor: u128,
        total_fee: u32,
    ) -> Self {
        for decimal in token_decimals.clone().into_iter() {
            assert!(decimal <= MAX_DECIMAL, "{}", ERR60_DECIMAL_ILLEGAL);
            assert!(decimal >= MIN_DECIMAL, "{}", ERR60_DECIMAL_ILLEGAL);
        }
        assert!(
            amp_factor >= MIN_AMP && amp_factor <= MAX_AMP,
            "{}",
            ERR61_AMP_ILLEGAL
        );
        assert!(total_fee < FEE_DIVISOR, "{}", ERR62_FEE_ILLEGAL);
        Self {
            token_account_ids: token_account_ids.iter().map(|a| a.clone().into()).collect(),
            token_decimals,
            c_amounts: vec![0u128; token_account_ids.len()],
            total_fee,
            init_amp_factor: amp_factor,
            target_amp_factor: amp_factor,
            init_amp_time: 0,
            stop_amp_time: 0,
        }
    }

    fn amounts_to_c_amounts(&self, amounts: &Vec<u128>) -> Vec<u128> {
        let mut c_amounts = amounts.clone();
        for (index, value) in self.token_decimals.iter().enumerate() {
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

    fn amount_to_c_amount(&self, amount: u128, index: usize) -> u128 {
        let value = self.token_decimals.get(index).unwrap();
        if *value <= TARGET_DECIMAL {
            let factor = 10_u128
                .checked_pow((TARGET_DECIMAL - value) as u32)
                .unwrap();
            amount.checked_mul(factor).unwrap()
        } else {
            let factor = 10_u128
                .checked_pow((value - TARGET_DECIMAL) as u32)
                .unwrap();
            amount.checked_div(factor).unwrap()
        }
    }

    fn c_amount_to_amount(&self, c_amount: u128, index: usize) -> u128 {
        let value = self.token_decimals.get(index).unwrap();
        if *value <= TARGET_DECIMAL {
            let factor = 10_u128
                .checked_pow((TARGET_DECIMAL - value) as u32)
                .unwrap();
            c_amount.checked_div(factor).unwrap()
        } else {
            let factor = 10_u128
                .checked_pow((value - TARGET_DECIMAL) as u32)
                .unwrap();
            c_amount.checked_mul(factor).unwrap()
        }
    }

    fn get_invariant(&self) -> StableSwap {
        StableSwap::new(
            self.init_amp_factor,
            self.target_amp_factor,
            1, // env::block_timestamp(),
            self.init_amp_time,
            self.stop_amp_time,
        )
    }

    /// Returns token index for given token account_id.
    fn token_index(&self, token_id: &AccountId) -> usize {
        self.token_account_ids
            .iter()
            .position(|id| id == token_id)
            .expect(ERR63_MISSING_TOKEN)
    }

    /// Returns number of tokens in outcome, given amount.
    /// Tokens are provided as indexes into token list for given pool.
    /// All tokens are comparable tokens
    fn internal_get_return(
        &self,
        token_in: usize,
        amount_in: Balance,
        token_out: usize,
        fees: &AdminFees,
    ) -> SwapResult {
        // make amounts into comparable-amounts
        let c_amount_in = self.amount_to_c_amount(amount_in, token_in);

        self.get_invariant()
            .swap_to(
                token_in,
                c_amount_in,
                token_out,
                &self.c_amounts,
                &Fees::new(self.total_fee, &fees),
            )
            .expect(ERR70_SWAP_OUT_CALC_ERR)
    }

    /// Swap `token_amount_in` of `token_in` token into `token_out` and return how much was received.
    /// Assuming that `token_amount_in` was already received from `sender_id`.
    pub fn swap(
        &mut self,
        token_in: &AccountId,
        amount_in: Balance,
        token_out: &AccountId,
        fees: &AdminFees,
    ) -> Balance {
        assert_ne!(token_in, token_out, "{}", ERR71_SWAP_DUP_TOKENS);
        let in_idx = self.token_index(token_in);
        let out_idx = self.token_index(token_out);
        let result = self.internal_get_return(in_idx, amount_in, out_idx, &fees);
        let amount_swapped = self.c_amount_to_amount(result.amount_swapped, out_idx);

        amount_swapped
    }
}
