use anchor_lang::prelude::*;

mod instructions;
mod state;
use instructions::*;
use state::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("AF9dYWSbe2T2EXqjCJNKRC2tLiPkiaduv1G4sTkqjaea");

#[program]
pub mod nft_manager {
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

    pub fn add_signer(ctx: Context<AddSignerContext>, signer: [u8; 32]) -> Result<()> {
        add_signer::add_signer(ctx, signer)
    }

    pub fn remove_signer(ctx: Context<DeleteSignerContext>, signer: [u8; 32]) -> Result<()> {
        delete_signer::delete_signer(ctx)
    }

    pub fn approve_in_src_chain(
        ctx: Context<ApproveInSrcChainContext>,
        nft: Pubkey,
        token_id: u64,
        to_chain_id: u64,
        auth_opt: bool,
        fee_ratio: u64,
    ) -> Result<()> {
        approve_in_src_chain::approve_in_src_chain(
            ctx,
            nft,
            token_id,
            to_chain_id,
            auth_opt,
            fee_ratio,
        )
    }

    pub fn transfer_wrapper(
        ctx: Context<TransferWrapperContext>,
        nft: Pubkey,
        token_id: u64,
        to: Pubkey,
        to_chains_id: Vec<u64>,
    ) -> Result<()> {
        transfer_wrapper::transfer_wrapper(ctx, nft, token_id, to, to_chains_id)
    }

    pub fn claim(
        ctx: Context<ClaimContext>,
        nft: Pubkey,
        token_id: u64,
        to_chains_id: Vec<u64>,
    ) -> Result<()> {
        claim::claim(ctx, nft, token_id, to_chains_id)
    }

    pub fn approve_in_to_chain(
        ctx: Context<ApproveInToChainContext>,
        fee_receiver: Pubkey,
        height: u64,
        signature: [u8; 64],
    ) -> Result<()> {
        approve_in_to_chain::approve_in_to_chain(ctx, fee_receiver, height, signature)
    }

    pub fn approve_local(
        ctx: Context<ApproveLocalContext>,
        nft: Pubkey,
        token_id: u64,
        auth_opt: bool,
        fee_ratio: u64,
    ) -> Result<()> {
        approve_local::approve_local(ctx, nft, token_id, auth_opt, fee_ratio)
    }
}
