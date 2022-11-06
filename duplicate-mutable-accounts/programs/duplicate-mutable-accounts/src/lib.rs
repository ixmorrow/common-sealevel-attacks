use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod duplicate_mutable_accounts {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Data: {}", ctx.accounts.user.data.to_string());
        Ok(())
    }

    pub fn insecure(ctx: Context<Update>, a: u64, b: u64) -> Result<()> {
        let user_a = &mut ctx.accounts.user_a;
        let user_b = &mut ctx.accounts.user_b;

        user_a.data = a;
        user_b.data = b;

        msg!("Data A: {}", ctx.accounts.user_a.data.to_string());
        msg!("Data B: {}", ctx.accounts.user_b.data.to_string());
        Ok(())
    }

    pub fn secure(ctx: Context<Update>, a: u64, b: u64) -> Result<()> {
        if ctx.accounts.user_a.key() == ctx.accounts.user_b.key() {
            return Err(ProgramError::InvalidArgument.into());
        }
        let user_a = &mut ctx.accounts.user_a;
        let user_b = &mut ctx.accounts.user_b;

        user_a.data = a;
        user_b.data = b;

        msg!("Data A: {}", ctx.accounts.user_a.data.to_string());
        msg!("Data B: {}", ctx.accounts.user_b.data.to_string());
        Ok(())
    }

    pub fn recommended(ctx: Context<UpdateChecked>, a: u64, b: u64) -> Result<()> {
        let user_a = &mut ctx.accounts.user_a;
        let user_b = &mut ctx.accounts.user_b;

        user_a.data = a;
        user_b.data = b;

        msg!("Data A: {}", ctx.accounts.user_a.data.to_string());
        msg!("Data B: {}", ctx.accounts.user_b.data.to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 8
    )]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    user_a: Account<'info, User>,
    #[account(mut)]
    user_b: Account<'info, User>,
}

#[derive(Accounts)]
pub struct UpdateChecked<'info> {
    #[account(constraint = user_a.key() != user_b.key())]
    user_a: Account<'info, User>,
    user_b: Account<'info, User>,
}

#[account]
pub struct User {
    data: u64,
}
