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
        ctx: Context<RegisterContext>,
        src_nft: Pubkey,
        src_token_id: u64,
        src_chain_id: u64,
    ) -> Result<()> {
        register::register(ctx, src_nft, src_token_id, src_chain_id)
    }

    pub fn unregister(
        ctx: Context<UnRegisterContext>,
        src_nft: Pubkey,
        src_token_id: u64,
        src_chain_id: u64,
    ) -> Result<()> {
        unregister::unregister(ctx, src_nft, src_token_id, src_chain_id)
    }

    pub fn charge(
        ctx: Context<ChargeContext>,
        fee_assert: Pubkey,
        price: u64,
        src_nft: Pubkey,
        src_token_id: u64,
        src_chain_id: u64,
    ) -> Result<()> {
        charge::charge(ctx, fee_assert, price, src_nft, src_token_id, src_chain_id)
    }

    pub fn add_black_list(
        ctx: Context<AddBlackListContext>,
        user: Pubkey,
        src_nft: Pubkey,
        src_token_id: u64,
        src_chain_id: u64,
    ) -> Result<()> {
        add_black_list::add_black_list(ctx, user, src_nft, src_token_id, src_chain_id)
    }

    pub fn remove_black_list(
        ctx: Context<RemoveBlackListContext>,
        user: Pubkey,
        src_nft: Pubkey,
        src_token_id: u64,
        src_chain_id: u64,
    ) -> Result<()> {
        remove_black_list::remove_black_list(ctx, user, src_nft, src_token_id, src_chain_id)
    }
}
