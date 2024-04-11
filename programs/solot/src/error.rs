use anchor_lang::prelude::*;

#[error_code]
pub enum SolotError {
    #[msg("Invalid argument")]
    InvalidArgument,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Exceeded winning amount")]
    ExceededWinningAmount,
}