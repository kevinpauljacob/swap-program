use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer { 
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub amount_offered: u64,
    pub token_mint_b: Pubkey,
    pub amount_requested: u64,
    pub expiration: i64,
    pub is_active: bool,
    pub bump: u8,
}

