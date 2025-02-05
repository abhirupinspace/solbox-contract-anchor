use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod utils;
pub mod state;
pub mod instructions; 

use instructions::initialize::*;
use instructions::register_user::*;
use instructions::place_referral::*;

declare_id!("DxyA3qMY73rf99uMRiucJcgQi6m3q3Luf3YuxSFM77Nh");

#[program]
pub mod solbox_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing the contract...");
        initialize(ctx)
    }

    pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
        register_user(ctx)
    }

    pub fn place_referral(ctx: Context<PlaceReferral>) -> Result<()> {
        place_referral(ctx)
    }
}