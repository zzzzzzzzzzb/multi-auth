use crate::state::{AdminInfo, AuthedInfo, NftTemplateError, CHAIN_ID, FEE_FACTOR};
use anchor_lang::solana_program::keccak::hashv as keccak;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use anchor_spl::metadata::update_metadata_accounts_v2;
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3,
            mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3, Metadata,
            UpdateMetadataAccountsV2,
        },
        token::{approve, mint_to, transfer, Approve, Mint, MintTo, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
pub struct UpdateURIContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub recipient: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = signer,
    )]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn update_uri(ctx: Context<UpdateURIContext>, new_data: DataV2) -> Result<()> {
    update_metadata_accounts_v2(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            UpdateMetadataAccountsV2 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                update_authority: ctx.accounts.sender_token_account.to_account_info(),
            },
        ),
        None,
        Some(new_data),
        None,
        None,
    )?;
    Ok(())
}

pub fn update_uri_sig(
    ctx: Context<UpdateURIContext>,
    new_data: DataV2,
    auth_signer: Pubkey,
    sig: [u8; 64],
) -> Result<()> {
    Ok(())
}
