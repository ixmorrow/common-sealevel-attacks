use anchor_lang::prelude::*;
// use anchor_lang::solana_program::borsh::try_from_slice_unchecked;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("BRAn5brMcPV69i6uH3jpaG9FcDEVYo5rGMyAPsMwzWEE");

#[program]
pub mod owner_check {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
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

        let mut account = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();

        account.user = ctx.accounts.payer.key();
        account.serialize(&mut *ctx.accounts.user.data.borrow_mut())?;

        msg!("Public Key: {}", account.user.to_string());
        Ok(())
    }

    pub fn insecure(ctx: Context<Unchecked>) -> Result<()> {
        let account = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();
        msg!("Public Key: {}", account.user.to_string());
        Ok(())
    }

    pub fn secure(ctx: Context<Unchecked>) -> Result<()> {
        if ctx.accounts.user.owner != ctx.program_id {
            return Err(ProgramError::InvalidAccountData.into());
        }
        let account = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();
        msg!("Public Key: {}", account.user.to_string());
        Ok(())
    }

    pub fn recommended(ctx: Context<Checked>) -> Result<()> {
        let account = &ctx.accounts.user;
        msg!("Public Key: {:?}", account.0.user);
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

#[derive(Accounts)]
pub struct Checked<'info> {
    /// CHECK:
    user: Account<'info, UserAccount>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct User {
    pub user: Pubkey,
}

#[derive(Clone, Debug)]
pub struct UserAccount(User);

impl anchor_lang::Owner for UserAccount {
    fn owner() -> Pubkey {
        ID
    }
}

impl anchor_lang::AccountSerialize for UserAccount {}

impl anchor_lang::AccountDeserialize for UserAccount {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        User::deserialize(buf).map(Self).map_err(Into::into)
    }

    fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
        Self::try_deserialize_unchecked(buf)
    }
}
