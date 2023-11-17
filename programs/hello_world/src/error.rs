use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // Define error codes and their corresponding error messages.
    #[msg("The Escrow account is yet to reach the unlock time")]
    EscrowNotReady,
}
