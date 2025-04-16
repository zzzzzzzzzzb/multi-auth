use anchor_lang::prelude::*;

mod instructions;
mod state;
use instructions::*;
use state::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("FtLq7ygXbDBZ57x6iWDhk9MhATNkeqGTBirctTztWT7C");

#[program]
pub mod multi_auth_program {
    use super::*;

    pub fn register(
        ctx: Context<Register>,
        src_nft: Pubkey,
        src_token_id: u64,
        src_chain_id: u64,
    ) -> Result<()> {
        register::register(ctx, src_nft, src_token_id, src_chain_id)
    }

    pub fn unregister(
        ctx: Context<UnRegister>,
        src_nft: Pubkey,
        src_token_id: u64,
        src_chain_id: u64,
    ) -> Result<()> {
        unregister::unregister(ctx, src_nft, src_token_id, src_chain_id)
    }
}
