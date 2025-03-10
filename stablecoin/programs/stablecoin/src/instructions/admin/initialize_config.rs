use anchor_lang::prelude::*;

use crate::{ConfigState, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT, MINT_DECIMALS};

pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {

}


#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    
    #[account(mut)]
    pub authority: Signer<'info>

    #[account(
        init,
        payer=authority,
        space = 8 + ConfigState::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT].
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
    pub systen_program: Program<'info, System>
}