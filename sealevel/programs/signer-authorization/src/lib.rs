use anchor_lang::prelude::*;

declare_id!("9xCciLpuCeYivkpFcNaW6doRQ3XfbbQ7G6cGitJij9vQ");

#[program]
pub mod signer_authorization {
    use super::*;

    pub fn insecure(ctx: Context<Insecure>) -> Result<()> {
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }

    pub fn secure(ctx: Context<Secure>) -> Result<()> {
        if !ctx.accounts.authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature.into());
        }
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }

    pub fn recommended(ctx: Context<Recommended>) -> Result<()> {
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Insecure<'info> {
    /// CHECK:
    authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Secure<'info> {
    /// CHECK:
    authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Recommended<'info> {
    authority: Signer<'info>,
}
