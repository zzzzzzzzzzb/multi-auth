use anchor_lang::prelude::*;

#[account]
pub struct AuthedInfo {
    pub nft: Pubkey,
    pub src_nft: Pubkey,
    pub src_token_id: u64,
    pub src_chain_id: u64,
}
