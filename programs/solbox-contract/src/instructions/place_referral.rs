use anchor_lang::prelude::*;

use super::register_user::UserAccount;

#[derive(Accounts)]
pub struct PlaceReferral<'info> {
    #[account(mut)]
    pub referrer: Signer<'info>,
    #[account(mut)]
    pub referee: Account<'info, UserAccount>,
}

pub fn handler(ctx: Context<PlaceReferral>) -> Result<()> {
    let _referrer = &ctx.accounts.referrer;
    let _referee = &ctx.accounts.referee;
    Ok(())
}
