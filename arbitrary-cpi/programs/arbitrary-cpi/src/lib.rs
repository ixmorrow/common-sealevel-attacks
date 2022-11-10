use anchor_lang::prelude::*;
use callee_program::cpi::accounts::Initialize;
use callee_program::program::CalleeProgram;
// use callee_program::{self, Data};

declare_id!("HvdkrQW4ZY6nJ9u3sqXQdkHeSChXQ8f3Sj3vJwaitut5");

#[program]
pub mod arbitrary_cpi {
    use super::*;

    pub fn cpi(ctx: Context<Cpi>) -> Result<()> {
        let cpi_program = ctx.accounts.callee_program.to_account_info();
        let cpi_accounts = Initialize {
            data: ctx.accounts.data.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        callee_program::cpi::initialize(cpi_ctx).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Cpi<'info> {
    #[account(mut)]
    pub data: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub callee_program: Program<'info, CalleeProgram>,
}
