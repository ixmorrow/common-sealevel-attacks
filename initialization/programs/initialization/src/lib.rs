use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use std::ops::DerefMut;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
#[program]
pub mod initialization {
    use super::*;

    pub fn insecure(ctx: Context<Unchecked>) -> Result<()> {
        let mut user = UserInsecure::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();

        user.authority = ctx.accounts.authority.key();

        let mut storage = ctx.accounts.user.try_borrow_mut_data()?;
        user.serialize(storage.deref_mut()).unwrap();
        Ok(())
    }

    pub fn secure(ctx: Context<Unchecked>) -> Result<()> {
        let mut user = UserSecure::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();
        msg!("Account owner is: {}", user.discriminator);

        if user.discriminator {
            return Err(ProgramError::InvalidAccountData.into());
        }

        user.authority = ctx.accounts.authority.key();
        user.discriminator = true;

        let mut storage = ctx.accounts.user.try_borrow_mut_data()?;
        user.serialize(storage.deref_mut()).unwrap();
        Ok(())
    }

    pub fn recommended(ctx: Context<Checked>) -> Result<()> {
        ctx.accounts.user.authority = ctx.accounts.authority.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Unchecked<'info> {
    #[account(mut)]
    /// CHECK:
    user: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Checked<'info> {
    #[account(init, payer = authority, space = 8+32)]
    user: Account<'info, UserRecommended>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UserInsecure {
    authority: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UserSecure {
    discriminator: bool,
    authority: Pubkey,
}

#[account]
pub struct UserRecommended {
    authority: Pubkey,
}
