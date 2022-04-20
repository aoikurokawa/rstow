use crate::error::CommonError;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

pub fn check_min_amount(amount: u64, min_amount: u64, action_name: u64) -> Result<()> {
    if amount >= min_amount {
        Ok(())
    } else {
        msg!(
            "{}: Number too low {} is (min is {})",
            action_name,
            amount,
            min_amount
        );
        Err(CommonError::NumberTooLow.into())
    }
}
