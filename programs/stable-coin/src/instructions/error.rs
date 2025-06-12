use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Invalid health factor")]
    DefaultHealthFactor,
    #[msg("Can not liquidate a healthy account")]
    AboveMinimumFactor,
}



