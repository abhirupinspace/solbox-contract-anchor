use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub authority: Pubkey,       // Wallet address of the user
    pub referrer: Option<Pubkey>, // Referrer (None if root user)
    pub referrals: Vec<Pubkey>,  // List of referrals
}

impl UserAccount {
    pub fn new(authority: Pubkey, referrer: Option<Pubkey>) -> Self {
        Self {
            authority,
            referrer,
            referrals: vec![],
        }
    }

    pub fn can_accept_referral(&self) -> bool {
        self.referrals.len() < crate::constants::MAX_DIRECT_REFERRALS
    }
}
