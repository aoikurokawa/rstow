use anchor_lang::prelude::*;

#[error_code]
pub enum CommonError {
    #[msg("Wrong reserve owner. Must be a system account.")]
    WrongReserveOwener,
    #[msg("Reserve must have no data, but has data.")]
    NonEmptyReserveData,
    #[msg("Invalid initial reserve lamports")]
    InvalidInitialReserveLamports,
    #[msg("Zero validator chunk size")]
    ZeroValidatorChunkSize,
    #[msg("Too big validator chunk size")]
    TooBigValidatorChunkSize,
    #[msg("Zero credit chunk size")]
    ZeroCreditChunkSize,
    #[msg("Too big credit chunk size")]
    TooBigCreditChunkSize,
    #[msg("Too low credit fee")]
    TooLowCreditFee,
    #[msg("Invalid mint authority")]
    InvalidMintAuthority,
    #[msg("Non empty initial mint supply")]
    MintHasInitialSupply,
    #[msg("Invalid owner fee state")]
    InvalidOwnerFeeState,

    #[msg("1910 Invalid program id. For using program from another account please update in the code.")]
    InvalidProgramId = 6116,

    #[msg("FFA0 Unexpected account")]
    UncheckedAccount = 65140,

    #[msg("CACF Calculation failure")]
    CalculationFailure = 51619,

    #[msg("B3AA You can't deposit a stake-account with lockup")]
    AccountWithLockup = 45694,

    #[msg("2000 Number too low")]
    NumberTooLow = 7892,
    #[msg("2001 Number to high")]
    NumberTooHigh = 7893,

    #[msg("1100 Fee too high")]
    FeeTooHigh = 4052,

    #[msg("1101 Min fee > Max fee")]
    FeeWrongWayRound = 4053,

    #[msg("1102 Liquidity target too low")]
    LiquidityTargetTooLow = 4054,

    #[msg("1103 Ticket not due. Wait more epochs")]
    TicketNotDue = 4055,

    #[msg("1104 Ticket not ready. Wait a few hours and try again")]
    TicketNotReady = 4056,

    #[msg("1105 Wrong Ticket beneficiary")]
    WrongBeneficiary = 4057,

    #[msg("1199 Insufficient Liquidity in the liquidity pool")]
    InsufficientLiquidity = 4205,

    #[msg("BAD1 Invalid validator")]
    InvalidValidator = 47525,
}
