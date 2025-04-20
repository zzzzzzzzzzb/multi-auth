use anchor_lang::prelude::*;

#[account]
pub struct AuthData {
    pub nft_address: Pubkey,
    pub token_id: u64,
    pub src_chain_id: u64,
    pub to_chain_id: u64,
    pub auth_opt: bool,
    pub fee_ratio: u64,
}
