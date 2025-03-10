use anchor_lang::prelude::*;


//Collateral State
#[account]
#[derive(InitSpace)]
pub struct CollateralState {
    pub depositer: Pubkey,
    pub sol_account: Pubkey,
    pub token_account: Pubkey,
    pub lamport_balance: u64,
    pub amount_minted: u64,
    pub bump: u8,
    pub bump_sol: u64,
    pub is_initialized: bool,
}