use anchor_lang::prelude::*;

pub fn log_referral_placement(referrer: Pubkey, new_user: Pubkey) {
    msg!("New referral placed: {} under {}", new_user, referrer);
}
