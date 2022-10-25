use anchor_lang::prelude::*;
// use anchor_lang::solana_program::program_error::ProgramError;
// use anchor_lang::solana_program::program_pack::Pack;
use anchor_spl::token::{Mint, TokenAccount};
use solana_program::{
    program_error::ProgramError,
    program_option::COption,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use spl_token::state::Account as SplTokenAccount;
declare_id!("7rLYxDkCFsTkvPbbK6YiPi6SoqCxaSkRSbR5VDkUqv2Y");

#[program]
pub mod owner_check {

    use super::*;

    pub fn insecure(ctx: Context<Insecure>) -> Result<()> {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        if ctx.accounts.authority.key != &token.owner {
            return Err(ProgramError::InvalidAccountData.into());
        }
        msg!("Your account balance is: {}", token.amount);
        Ok(())
    }

    pub fn secure(ctx: Context<Secure>) -> Result<()> {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        if ctx.accounts.token.owner != &spl_token::ID {
            return Err(ProgramError::InvalidAccountData.into());
        }
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

    // pub fn init(ctx: Context<Init>) -> Result<()> {
    //     let mut test = ctx.accounts.test.load_init()?;
    //     *test = Test::default();
    //     test.mint = ctx.accounts.mint.key();
    //     test.owner = ctx.accounts.authority.key();
    //     test.amount = 1;
    //     test.state = AccountState::Initialized;
    //     // test.is_initialized = true;
    //     Ok(())
    // }
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

// #[derive(Accounts)]
// pub struct Init<'info> {
//     #[account(init, payer=authority, space=165)]
//     // #[account(init, payer=authority, space=std::mem::size_of::<Test>())]
//     test: AccountLoader<'info, Test>,
//     #[account(mut)]
//     authority: Signer<'info>,
//     mint: Account<'info, Mint>,
//     pub system_program: Program<'info, System>,
// }

// #[repr(packed)]
// #[account(zero_copy)]
// #[derive(Default)]
// pub struct Test {
//     // pub is_initialized: bool,
//     pub mint: Pubkey,
//     pub owner: Pubkey,
//     pub amount: u64,
//     pub delegate: COption<Pubkey>,
//     pub state: AccountState,
//     pub is_native: COption<u64>,
//     pub delegated_amount: u64,
//     pub close_authority: COption<Pubkey>,
// }

// impl IsInitialized for Test {
//     fn is_initialized(&self) -> bool {
//         self.state != AccountState::Uninitialized
//     }
// }

// /// Account state.
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, AnchorDeserialize, AnchorSerialize)]
// pub enum AccountState {
//     Uninitialized,
//     Initialized,
//     Frozen,
// }

// impl Default for AccountState {
//     fn default() -> Self {
//         AccountState::Initialized
//     }
// }
