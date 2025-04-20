use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        metadata::{
            Metadata,
        },
        token::{Mint, Token, TokenAccount},
    },
};

declare_id!("3QcwJAXvEFgp7ehUGj4YhK5d3n3MZoDp4Sgxe8wJ9cf8");

declare_program!(mint_nft);
use mint_nft::cpi::accounts::MintNft as MintNftAccount;
use mint_nft::cpi::mint_nft;
use mint_nft::program::MintNft as MintNftProgram;

// declare_program!(multi_auth_program);


#[program]
pub mod nft_template {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        nft_name: String,
        nft_symbol: String,
        nft_uri: String,
    ) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.mint_nft.to_account_info(),
            MintNftAccount {
                payer: ctx.accounts.payer.to_account_info(),
                metadata_account: ctx.accounts.metadata_account.to_account_info(),
                edition_account: ctx.accounts.edition_account.to_account_info(),
                mint_account: ctx.accounts.mint_account.to_account_info(),
                associated_token_account: ctx.accounts.associated_token_account.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                token_metadata_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );
        mint_nft(cpi_ctx, nft_name, nft_symbol, nft_uri)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub mint_nft: Program<'info, MintNftProgram>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Validate address by deriving pda
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

    // Create new mint account, NFTs have 0 decimals
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    // Create associated token account, if needed
    // This is the account that will hold the NFT
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
