use anchor_lang::prelude::*;

declare_id!("C6J4ttv9YtPCqrtL563KZuqC4yvHtQ1YeUpfKYF5FwoY");

#[program]
pub mod callee_program {
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
