use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{constants::{MINT_DECIMALS, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT}, states::ConfigState};


pub fn process_initialize_config(_ctx: Context<InitializeConfig>) -> Result<()> {

    Ok(())
}


#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer=authority,
        space = 8 + ConfigState::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump
    )]
    pub config_account: Account<'info, ConfigState>,

    #[account(
        init,
        payer=authority,
        seeds=[SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}