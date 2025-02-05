use anchor_lang::prelude::*;
use crate::state::user::UserAccount;

pub fn find_next_slot(
    accounts: &mut Vec<UserAccount>,
    referrer: &mut UserAccount,
    new_user: Pubkey
) -> Option<Pubkey> {
    if referrer.can_accept_referral() {
        referrer.referrals.push(new_user);
        return Some(referrer.authority);
    }
    
    for referral in &referrer.referrals {
        let mut ref_mut = accounts.iter_mut().find(|acc| acc.authority == *referral);
        if let Some(ref mut downline) = ref_mut {
            if downline.can_accept_referral() {
                downline.referrals.push(new_user);
                return Some(downline.authority);
            }
        }
    }
    
    None
}
