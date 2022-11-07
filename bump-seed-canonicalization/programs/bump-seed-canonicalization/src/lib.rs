use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bump_seed_canonicalization {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump_seed: u8) -> Result<()> {
        let space = 32;
        let lamports = Rent::get()?.minimum_balance(space as usize);

        let ix = anchor_lang::solana_program::system_instruction::create_account(
            &ctx.accounts.payer.key(),
            &ctx.accounts.pda.key(),
            lamports,
            space,
            &ctx.program_id,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.pda.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[&[bump_seed]]],
        )?;

        let mut account = User::try_from_slice(&ctx.accounts.pda.data.borrow()).unwrap();

        account.user = ctx.accounts.payer.key();
        account.serialize(&mut *ctx.accounts.pda.data.borrow_mut())?;

        msg!("PDA: {}", ctx.accounts.pda.key());
        msg!("User: {}", account.user);

        Ok(())
    }

    pub fn insecure(ctx: Context<Unchecked>, bump_seed: u8) -> Result<()> {
        let address = Pubkey::create_program_address(&[&[bump_seed]], ctx.program_id).unwrap();
        if address != ctx.accounts.pda.key() {
            return Err(ProgramError::InvalidArgument.into());
        }

        let account = User::try_from_slice(&ctx.accounts.pda.data.borrow()).unwrap();

        msg!("PDA: {}", ctx.accounts.pda.key());
        msg!("User: {}", account.user);
        Ok(())
    }

    pub fn secure(ctx: Context<Unchecked>, bump_seed: u8) -> Result<()> {
        let (address, expected_bump) = Pubkey::find_program_address(&[], ctx.program_id);

        if address != ctx.accounts.pda.key() {
            return Err(ProgramError::InvalidArgument.into());
        }
        if expected_bump != bump_seed {
            return Err(ProgramError::InvalidArgument.into());
        }
        let account = User::try_from_slice(&ctx.accounts.pda.data.borrow()).unwrap();

        msg!("PDA: {}", ctx.accounts.pda.key());
        msg!("User: {}", account.user);
        Ok(())
    }

    pub fn recommended(ctx: Context<Checked>) -> Result<()> {
        msg!("PDA: {}", ctx.accounts.pda.key());
        msg!("User: {}", ctx.accounts.pda.0.user);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    /// CHECK:
    pub pda: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unchecked<'info> {
    /// CHECK:
    pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Checked<'info> {
    #[account(seeds = [], bump)]
    pda: Account<'info, UserAccount>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct User {
    user: Pubkey,
}

#[derive(Clone)]
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
