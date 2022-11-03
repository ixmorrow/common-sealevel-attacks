use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_spl::token::TokenAccount;
use spl_token::state::Account as SplTokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod account_data_matching {
    use super::*;

    pub fn insecure(ctx: Context<Unchecked>) -> Result<()> {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        msg!("Account owner is: {}", token.owner);
        Ok(())
    }

    pub fn secure(ctx: Context<Unchecked>) -> Result<()> {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        if ctx.accounts.authority.key != &token.owner {
            return Err(ProgramError::InvalidAccountData.into());
        }
        msg!("Account owner is: {}", token.owner);
        Ok(())
    }

    pub fn recommended(ctx: Context<Checked>) -> Result<()> {
        msg!("Account owner is: {}", ctx.accounts.token.owner);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Unchecked<'info> {
    /// CHECK:
    token: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Checked<'info> {
    #[account(constraint = authority.key == &token.owner)]
    token: Account<'info, TokenAccount>,
    authority: Signer<'info>,
}
