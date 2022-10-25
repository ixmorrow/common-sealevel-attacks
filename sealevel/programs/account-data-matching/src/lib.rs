use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_spl::token::TokenAccount;
use spl_token::state::Account as SplTokenAccount;

declare_id!("5JcpLgkgWp97fy6BBfNJfqTdfgJw2DntQx17NTajMCYq");

#[program]
pub mod account_data_matching {
    use super::*;

    pub fn insecure(ctx: Context<Insecure>) -> Result<()> {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        msg!("Your account balance is: {}", token.amount);
        Ok(())
    }

    pub fn secure(ctx: Context<Secure>) -> Result<()> {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        if ctx.accounts.authority.key != &token.owner {
            return Err(ProgramError::InvalidAccountData.into());
        }
        msg!("Your account balance is: {}", token.amount);
        Ok(())
    }

    pub fn recommended(ctx: Context<Recommended>) -> Result<()> {
        msg!("Your account balance is: {}", ctx.accounts.token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Insecure<'info> {
    /// CHECK:
    token: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Secure<'info> {
    /// CHECK:
    token: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Recommended<'info> {
    #[account(constraint = authority.key == &token.owner)]
    token: Account<'info, TokenAccount>,
    authority: Signer<'info>,
}
