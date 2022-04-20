use anchor_lang::prelude::*;

#[error_code]
pub enum CommonError {
    #[msg("Wrong reserve owner. Must be a system account.")]
    WrongReserveOwener,
    #[msg("Reserve must have no data, but has data.")]
    NonEmptyReserveData,
}
