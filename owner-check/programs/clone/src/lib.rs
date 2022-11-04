use anchor_lang::prelude::*;

declare_id!("5SmitLhxfKa9F7FeW84veXYKWUfsxvujh7fGf1gTY6Ju");

#[program]
pub mod clone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.data.user = ctx.accounts.payer.key();
        msg!("User: {}", ctx.accounts.data.user.to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32
    )]
    pub data: Account<'info, Data>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Data {
    user: Pubkey,
}
