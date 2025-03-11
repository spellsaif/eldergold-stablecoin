use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::Mint};

use crate::{constants::{MINT_DECIMALS, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT}, states::ConfigState};


pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {

    let config_account = &mut ctx.accounts.config_account;
    config_account.authority = ctx.accounts.authority.key();
    

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
    
    //Programs
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>
}