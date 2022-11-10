use anchor_lang::prelude::*;

declare_id!("9kQK8xVskuvF8e1NfThkKrfcVZu6HNiCWF2b9C9C7SXg");

#[program]
pub mod callee_program_fake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub data: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Data {
    pub data: u64,
}
