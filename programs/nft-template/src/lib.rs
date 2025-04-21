mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("3QcwJAXvEFgp7ehUGj4YhK5d3n3MZoDp4Sgxe8wJ9cf8");

#[program]
pub mod nft_template {
    use super::*;

    pub fn mint_with_auth(
        ctx: Context<MintContext>,
        fee_token: Pubkey,
        price: u64,
        src_nft: Pubkey,
        src_token_id: u64,
        src_chain_id: u64,
        nft_name: String,
        nft_symbol: String,
        nft_uri: String,
    ) -> Result<()> {
        mint::mint_with_auth(
            ctx,
            fee_token,
            price,
            src_nft,
            src_token_id,
            src_chain_id,
            nft_name,
            nft_symbol,
            nft_uri,
        )
    }

    pub fn mint_with_sig(
        ctx: Context<MintContext>,
        fee_token: Pubkey,
        price: u64,
        src_nft: Pubkey,
        src_token_id: u64,
        src_chain_id: u64,
        nonce: u64,
        nft_name: String,
        nft_symbol: String,
        nft_uri: String,
        authed_signer: Pubkey,
        signature: [u8; 64],
    ) -> Result<()> {
        mint::mint_with_sig(
            ctx,
            fee_token,
            price,
            src_nft,
            src_token_id,
            src_chain_id,
            nonce,
            nft_name,
            nft_symbol,
            nft_uri,
            authed_signer,
            signature,
        )
    }

    pub fn init_admin_and_receiver(
        ctx: Context<InitAdminAndReceiverContext>,
        admin: Pubkey,
        recv: Pubkey,
    ) -> Result<()> {
        auth_admin::init_admin_and_receiver(ctx, admin, recv)
    }

    pub fn update_admin(ctx: Context<UpdateAdminAndReceiverContext>, admin: Pubkey) -> Result<()> {
        auth_admin::update_admin(ctx, admin)
    }

    pub fn update_recv(ctx: Context<UpdateAdminAndReceiverContext>, recv: Pubkey) -> Result<()> {
        auth_admin::update_receiver(ctx, recv)
    }
}
