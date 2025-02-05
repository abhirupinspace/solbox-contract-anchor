use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub authority: Pubkey,
}

pub fn handler(ctx: Context<RegisterUser>) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    user.authority = *ctx.accounts.user.key;
    Ok(())
}
