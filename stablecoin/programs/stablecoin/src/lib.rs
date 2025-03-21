use anchor_lang::prelude::*;

declare_id!("Co32qABaBxtzVNmSmQ82v5MXzcCvRvUb23FmfW6Ko7JR");

pub mod states;
pub mod constants;
pub mod instructions;

pub use instructions::*;


#[program]
pub mod stablecoin {

    use crate::instructions::process_initialize_config;

    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx:Context<UpdateConfig>, min_health_factor:u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }
}

