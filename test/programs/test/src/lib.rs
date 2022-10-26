use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data = &mut ctx.accounts.data_account;
        data.user = ctx.accounts.payer.key();
        Ok(())
    }

    pub fn update(ctx: Context<Update>) -> Result<()> {
        let data = &mut ctx.accounts.data_account;
        data.user = ctx.accounts.payer.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + 32)]
    pub data_account: Account<'info, Data>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, constraint = payer.key == &data_account.user)]
    pub data_account: Account<'info, Data>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct Data {
    pub user: Pubkey,
}
