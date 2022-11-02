use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod signer_authorization {
    use super::*;

    pub fn insecure(ctx: Context<Unchecked>) -> Result<()> {
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }

    pub fn secure(ctx: Context<Unchecked>) -> Result<()> {
        if !ctx.accounts.authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature.into());
        }
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }

    pub fn recommended(ctx: Context<Checked>) -> Result<()> {
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Unchecked<'info> {
    /// CHECK:
    authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Checked<'info> {
    authority: Signer<'info>,
}
