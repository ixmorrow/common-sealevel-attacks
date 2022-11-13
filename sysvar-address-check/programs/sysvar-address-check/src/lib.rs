use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sysvar_address_check {
    use super::*;

    pub fn insecure(ctx: Context<Unchecked>) -> Result<()> {
        msg!("Rent Key -> {}", ctx.accounts.rent.key().to_string());
        Ok(())
    }

    pub fn secure(ctx: Context<Unchecked>) -> Result<()> {
        require_eq!(ctx.accounts.rent.key(), sysvar::rent::ID);
        msg!("Rent Key -> {}", ctx.accounts.rent.key().to_string());
        Ok(())
    }

    pub fn recommended(ctx: Context<Checked>) -> Result<()> {
        msg!("Rent Key -> {}", ctx.accounts.rent.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Unchecked<'info> {
    /// CHECK:
    rent: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Checked<'info> {
    rent: Sysvar<'info, Rent>,
}
