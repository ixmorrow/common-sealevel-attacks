use anchor_lang::prelude::*;

declare_id!("Cft4eTTrt4sJU4Ar35rUQHx6PSXfJju3dixmvApzhWws");

#[program]
pub mod owner_check {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.data.user = ctx.accounts.payer.key();
        msg!("User: {}", ctx.accounts.data.user.to_string());
        Ok(())
    }

    pub fn insecure(ctx: Context<Unchecked>) -> Result<()> {
        let account_data = ctx.accounts.data.try_borrow_data()?;
        let mut account_data_slice: &[u8] = &account_data;
        let account_state = Data::try_deserialize(&mut account_data_slice)?;
        msg!("User: {}", account_state.user.to_string());
        msg!("Program Owner: {}", ctx.accounts.data.owner.to_string());
        Ok(())
    }

    pub fn secure(ctx: Context<Unchecked>) -> Result<()> {
        if ctx.accounts.data.owner != ctx.program_id {
            return Err(ProgramError::IllegalOwner.into());
        }
        let account_data = ctx.accounts.data.try_borrow_data()?;
        let mut account_data_slice: &[u8] = &account_data;
        let account_state = Data::try_deserialize(&mut account_data_slice)?;
        msg!("User: {}", account_state.user.to_string());
        msg!("Program Owner: {}", ctx.accounts.data.owner.to_string());
        Ok(())
    }

    pub fn recommended(ctx: Context<Checked>) -> Result<()> {
        msg!("User: {}", &ctx.accounts.data.user.to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32
    )]
    pub data: Account<'info, Data>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unchecked<'info> {
    /// CHECK:
    data: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Checked<'info> {
    data: Account<'info, Data>,
}

#[account]
pub struct Data {
    user: Pubkey,
}
