use anchor_lang::prelude::*;

declare_id!("5657x5XSmUukd1jfQLZ5C6W2r6gBKNaHv14xMdh8xFA4");

#[program]
pub mod type_cosplay_recommended {
    use super::*;

    pub fn initialize_type_one(ctx: Context<InitializeTypeOne>) -> Result<()> {
        ctx.accounts.user.user_one = ctx.accounts.payer.key();
        msg!("User: {}", ctx.accounts.user.user_one.to_string());
        Ok(())
    }

    pub fn initialize_type_two(ctx: Context<InitializeTypeTwo>) -> Result<()> {
        ctx.accounts.user.user_two = ctx.accounts.payer.key();
        msg!("User: {}", ctx.accounts.user.user_two.to_string());
        Ok(())
    }

    pub fn recommended(ctx: Context<Checked>) -> Result<()> {
        msg!("Account: {}", ctx.accounts.user.key().to_string());
        msg!("User: {}", ctx.accounts.user.user_one.to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeTypeOne<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32
    )]
    pub user: Account<'info, TypeOne>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeTypeTwo<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32
    )]
    pub user: Account<'info, TypeTwo>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Checked<'info> {
    user: Account<'info, TypeOne>,
}

#[account]
pub struct TypeOne {
    user_one: Pubkey,
}

#[account]
pub struct TypeTwo {
    user_two: Pubkey,
}
