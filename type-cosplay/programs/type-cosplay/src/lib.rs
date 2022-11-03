use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("4EAyrs6uzEXvKb8DPEsQrmBTD4j3CUYdutGvjqBnBQh2");

#[program]
pub mod type_cosplay {
    use super::*;

    pub fn initialize_type_one(ctx: Context<Initialize>) -> Result<()> {
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

        let mut account = TypeOne::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();

        account.user_one = ctx.accounts.payer.key();
        account.serialize(&mut *ctx.accounts.user.data.borrow_mut())?;

        msg!("User: {}", account.user_one.to_string());
        Ok(())
    }

    pub fn initialize_type_two(ctx: Context<Initialize>) -> Result<()> {
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

        let mut account = TypeTwo::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();

        account.user_two = ctx.accounts.payer.key();
        account.serialize(&mut *ctx.accounts.user.data.borrow_mut())?;

        msg!("User: {}", account.user_two.to_string());
        Ok(())
    }

    pub fn insecure(ctx: Context<Unchecked>) -> Result<()> {
        let account = TypeOne::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();
        msg!("Account: {}", ctx.accounts.user.key().to_string());
        msg!("User: {}", account.user_one.to_string());
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

#[derive(Accounts)]
pub struct Unchecked<'info> {
    /// CHECK:
    user: AccountInfo<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TypeOne {
    user_one: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TypeTwo {
    user_two: Pubkey,
}
