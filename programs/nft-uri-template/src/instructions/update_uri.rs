use crate::state::{AdminInfo, AuthedInfo, AuthedSigner, NftTemplateError, CHAIN_ID, FEE_FACTOR};
use anchor_lang::solana_program::keccak::hashv as keccak;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3, update_metadata_accounts_v2,
            mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3, Metadata,
            UpdateMetadataAccountsV2,
        },
        token::{approve, mint_to, transfer, Approve, Mint, MintTo, Token, TokenAccount, Transfer},
    },
};

// TODO remove
declare_program!(nft_manager);
use nft_manager::accounts::{AuthData, AuthStatusAccount};
use nft_manager::types::AuthStatus;

#[derive(Accounts)]
pub struct UpdateURIContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account()]
    pub auth_status_account: Account<'info, AuthStatusAccount>,
    #[account()]
    pub auth_datas_account: Account<'info, AuthData>,
    #[account()]
    pub authed_info_account: Account<'info, AuthedInfo>,
    #[account()]
    pub signers_account: Account<'info, AuthedSigner>,
    /// CHECK: Validate address by deriving pda
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
}

pub fn update_uri(
    ctx: Context<UpdateURIContext>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let auth_info = &mut ctx.accounts.authed_info_account;
    let auth_status = &mut ctx.accounts.auth_status_account;
    let auth_data = &mut ctx.accounts.auth_datas_account;
    match auth_status.auth_status {
        AuthStatus::Authed => {
            require!(auth_data.auth_opt, NftTemplateError::AuthOPTIsFalse);
        }
        _ => return Err(Error::from(NftTemplateError::AuthError)),
    };
    update_metadata_accounts_v2(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            UpdateMetadataAccountsV2 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                update_authority: ctx.accounts.sender_token_account.to_account_info(),
            },
        ),
        None,
        Some(DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        }),
        None,
        None,
    )?;
    Ok(())
}

pub fn update_uri_sig(
    ctx: Context<UpdateURIContext>,
    name: String,
    symbol: String,
    uri: String,
    authed_signer: Pubkey,
    sig: [u8; 64],
) -> Result<()> {
    let msg_hash = keccak(&[
        ctx.accounts.sender_token_account.key().as_ref(),
        name.as_bytes(),
        symbol.as_bytes(),
        uri.as_bytes(),
    ]);
    let pk = secp256k1_recover(msg_hash.as_ref(), 0, sig.as_ref())
        .map_err(|err| NftTemplateError::InvalidSignature)?;
    require!(
        keccak(&[pk.0.as_ref()]).0 == authed_signer.as_ref(),
        NftTemplateError::InvalidSigner
    );
    update_uri(ctx, name, symbol, uri)
}
