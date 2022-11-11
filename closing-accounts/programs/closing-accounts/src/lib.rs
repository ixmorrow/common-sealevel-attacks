use anchor_lang::prelude::*;
use std::io::{Cursor, Write};
use std::ops::DerefMut;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod closing_accounts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.user.authority = ctx.accounts.authority.key();

        Ok(())
    }

    pub fn insecure_close(ctx: Context<Close>) -> Result<()> {
        let dest_starting_lamports = ctx.accounts.destination.lamports();

        **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(ctx.accounts.user.to_account_info().lamports())
            .unwrap();
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() = 0;

        Ok(())
    }

    pub fn insecure_close_still(ctx: Context<Close>) -> Result<()> {
        let account = ctx.accounts.user.to_account_info();

        let dest_starting_lamports = ctx.accounts.destination.lamports();

        **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(account.lamports())
            .unwrap();
        **account.lamports.borrow_mut() = 0;

        let mut data = account.try_borrow_mut_data()?;
        for byte in data.deref_mut().iter_mut() {
            *byte = 0;
        }

        Ok(())
    }

    pub fn insecure_close_still_still(ctx: Context<Close>) -> Result<()> {
        let dest_starting_lamports = ctx.accounts.destination.lamports();

        let account = ctx.accounts.user.to_account_info();
        **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(account.lamports())
            .unwrap();
        **account.lamports.borrow_mut() = 0;

        let mut data = account.try_borrow_mut_data()?;
        for byte in data.deref_mut().iter_mut() {
            *byte = 0;
        }

        let dst: &mut [u8] = &mut data;
        let mut cursor = std::io::Cursor::new(dst);
        cursor
            .write_all(&anchor_lang::__private::CLOSED_ACCOUNT_DISCRIMINATOR)
            .unwrap();

        Ok(())
    }

    pub fn force_defund(ctx: Context<ForceDefund>) -> Result<()> {
        let account = &ctx.accounts.user;

        let data = account.try_borrow_data()?;
        assert!(data.len() > 8);

        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&data[0..8]);
        if discriminator != anchor_lang::__private::CLOSED_ACCOUNT_DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData.into());
        }

        let dest_starting_lamports = ctx.accounts.destination.lamports();

        **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(account.lamports())
            .unwrap();
        **account.lamports.borrow_mut() = 0;

        Ok(())
    }

    pub fn recommended_close(ctx: Context<RecommendedClose>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8+32)]
    user: Account<'info, User>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    user: Account<'info, User>,
    destination: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct ForceDefund<'info> {
    #[account(mut)]
    /// CHECK:
    user: UncheckedAccount<'info>,
    destination: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct RecommendedClose<'info> {
    #[account(mut, close = destination)]
    user: Account<'info, User>,
    #[account(mut)]
    destination: SystemAccount<'info>,
}

#[account]
pub struct User {
    authority: Pubkey,
}
