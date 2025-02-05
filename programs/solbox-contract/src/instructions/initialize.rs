use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub value: u64,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let data = &mut ctx.accounts.data_account;
    data.value = 0;
    Ok(())
}
