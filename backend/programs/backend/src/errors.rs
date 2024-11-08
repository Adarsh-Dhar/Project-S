use anchor_lang::prelude::*;

#[error_code]
pub enum LendingError {
    #[msg("Insufficient collateral for loan")]
    InsufficientCollateral,
    #[msg("Invalid repayment amount")]
    InvalidRepaymentAmount,
    #[msg("Position is not liquidatable")]
    PositionNotLiquidatable,
    #[msg("Oracle price is stale")]
    StaleOraclePrice,
    #[msg("Math operation overflow")]
    MathOverflow,
} 