use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("A4eTQheZwwx7GRm441KfXnsK4ZCf3RDbx4Dxmv8p9a2z");

#[program]
pub mod clone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let space = 32;
        let lamports = Rent::get()?.minimum_balance(space as usize);

        let ix = anchor_lang::solana_program::system_instruction::create_account(
            &ctx.accounts.payer.key(),
            &ctx.accounts.user.key(),
            lamports,
            space,
            &ctx.program_id,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let mut account = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();

        account.user = ctx.accounts.payer.key();
        account.serialize(&mut *ctx.accounts.user.data.borrow_mut())?;

        msg!("Public Key: {}", account.user.to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    pub user: Pubkey,
}
