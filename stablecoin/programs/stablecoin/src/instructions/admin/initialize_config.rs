use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::Mint};

use crate::{constants::{LIQUIDATION_BONUS, LIQUIDATION_THRESHOLD, MINT_DECIMALS, MIN_HEALTH_FACTOR, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT}, states::ConfigState};


pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {

    *ctx.accounts.config_account = ConfigState {
        authority: ctx.accounts.authority.key(),
        mint_account: ctx.accounts.mint_account.key(),
        liquidation_threshold:LIQUIDATION_THRESHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        min_health_factor: MIN_HEALTH_FACTOR,
        bump: ctx.bumps.config_account,
        bump_mint_account: ctx.bumps.mint_account

    };
    

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