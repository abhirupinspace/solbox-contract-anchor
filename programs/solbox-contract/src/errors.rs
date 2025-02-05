use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("No available slot for referral placement.")]
    NoAvailableSlot,
}
