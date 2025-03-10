use anchor_lang::prelude::*;

declare_id!("Co32qABaBxtzVNmSmQ82v5MXzcCvRvUb23FmfW6Ko7JR");

pub mod states;
pub mod constants;
pub mod instructions;

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
