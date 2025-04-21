use crate::state::{AdminInfo, AuthedInfo, NftTemplateError, CHAIN_ID, FEE_FACTOR};
use anchor_lang::solana_program::keccak::hashv as keccak;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3,
            mpl_token_metadata::types::DataV2, CreateMasterEditionV3, CreateMetadataAccountsV3,
            Metadata,
        },
        token::{approve, mint_to, transfer, Approve, Mint, MintTo, Token, TokenAccount, Transfer},
    },
};

declare_program!(nft_manager);
use nft_manager::accounts::{AuthData, AuthStatusAccount, FeeReceiver};
use nft_manager::cpi::accounts::Charge;
use nft_manager::cpi::charge;
use nft_manager::program::NftManager;
use nft_manager::types::AuthStatus;

declare_program!(mint_nft);
use mint_nft::cpi::accounts::MintNft as MintNftAccount;
use mint_nft::cpi::mint_nft;
use mint_nft::program::MintNft as MintNftProgram;

#[derive(Accounts)]
#[instruction(src_nft: Pubkey, token_id: u64, src_token_id: u64, src_chain_id: u64)]
pub struct MintContext<'info> {
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + 8 + 32 + 8 + 8 + 1,
        seeds = [
            b"authed_info",
            src_nft.as_ref(),
            &src_chain_id.to_le_bytes(),
            &src_token_id.to_le_bytes(),
        ],
        bump,
    )]
    pub authed_info_account: Account<'info, AuthedInfo>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"data",
            src_nft.as_ref(),
            &token_id.to_le_bytes(),
            &src_chain_id.to_le_bytes(),
        ],
        bump,
    )]
    pub auth_datas_account: Account<'info, AuthData>,

    #[account(
        mut,
        seeds = [
            b"status",
            owner.key().as_ref(),
            src_nft.as_ref(),
            &src_token_id.to_le_bytes(),
            &src_chain_id.to_le_bytes(),
        ],
        bump,
    )]
    pub auth_status_account: Account<'info, AuthStatusAccount>,

    #[account(
        mut,
        seeds = [
            b"src_chain",
            src_nft.as_ref(),
            &token_id.to_le_bytes(),
        ],
        bump,
    )]
    pub fee_receivers_in_src_chain: Account<'info, FeeReceiver>,

    #[account(
        mut,
        seeds = [
            b"admin_receiver",
        ],
        bump,
    )]
    pub admin_receiver_account: Account<'info, AdminInfo>,

    #[account(
        mut,
        seeds = [
            b"to_chain",
            src_nft.as_ref(),
            &token_id.to_le_bytes(),
            &src_chain_id.to_le_bytes(),
        ],
        bump,
    )]
    pub fee_receivers_in_to_chain: Account<'info, FeeReceiver>,

    #[account(mut)]
    pub sender: AccountInfo<'info>,
    pub recipient: SystemAccount<'info>,

    pub receiver: AccountInfo<'info>,
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
        associated_token::authority = sender,
    )]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = sender,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub fee_token_context: FeeTokenContext<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub nft_manager_program: Program<'info, NftManager>,
    pub nft_minter_program: Program<'info, MintNftProgram>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct FeeTokenContext<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    pub recipient: SystemAccount<'info>,
    pub owner: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = sender,
    )]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

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
    let auth_data = &mut ctx.accounts.auth_datas_account;
    let auth_status = &mut ctx.accounts.auth_status_account;
    let fee_ratio = auth_data.fee_ratio;

    let _receiver;
    match auth_status.auth_status {
        AuthStatus::Authed => {
            require!(auth_data.auth_opt, NftTemplateError::AuthOPTIsFalse);
            if auth_data.src_chain_id == CHAIN_ID {
                _receiver = ctx.accounts.fee_receivers_in_src_chain.receiver;
            } else {
                _receiver = ctx.accounts.fee_receivers_in_to_chain.receiver;
            }
            // SOL
            if fee_token == Pubkey::new_from_array([0u8; 32]) {
                require!(
                    ctx.accounts.sender.lamports() >= price,
                    NftTemplateError::InsufficientFunds
                );
                let cpi_ctx = CpiContext::new(
                    ctx.accounts.nft_manager_program.to_account_info(),
                    Charge {
                        auth_datas_account: ctx.accounts.auth_datas_account.to_account_info(),
                        fee_receivers_in_to_chain: ctx
                            .accounts
                            .fee_receivers_in_to_chain
                            .to_account_info(),
                        sender: ctx.accounts.sender.to_account_info(),
                        recipient: ctx.accounts.recipient.to_account_info(),
                        mint_account: ctx.accounts.mint_account.to_account_info(),
                        sender_token_account: ctx.accounts.sender_token_account.to_account_info(),
                        recipient_token_account: ctx
                            .accounts
                            .recipient_token_account
                            .to_account_info(),
                        token_program: ctx.accounts.token_program.to_account_info(),
                        associated_token_program: ctx
                            .accounts
                            .associated_token_program
                            .to_account_info(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                    },
                );
                charge(
                    cpi_ctx,
                    fee_token,
                    price,
                    src_nft,
                    src_token_id,
                    src_chain_id,
                )?;
                let fee_amount = price
                    .checked_mul(fee_ratio)
                    .ok_or(NftTemplateError::MathOverFlow)?
                    .checked_div(FEE_FACTOR)
                    .ok_or(NftTemplateError::MathOverFlow)?;
                **ctx
                    .accounts
                    .sender
                    .to_account_info()
                    .try_borrow_mut_lamports()? -= fee_amount;
                **ctx
                    .accounts
                    .receiver
                    .to_account_info()
                    .try_borrow_mut_lamports()? += fee_amount;
            } else {
                // SPL
                let balance_before = ctx.accounts.fee_token_context.sender_token_account.amount;

                transfer(
                    CpiContext::new(
                        ctx.accounts.token_program.to_account_info(),
                        Transfer {
                            from: ctx
                                .accounts
                                .fee_token_context
                                .sender_token_account
                                .to_account_info(),
                            to: ctx
                                .accounts
                                .fee_token_context
                                .owner_token_account
                                .to_account_info(),
                            authority: ctx.accounts.fee_token_context.owner.to_account_info(),
                        },
                    ),
                    price,
                )?;

                let fee_amount = price
                    .checked_mul(fee_ratio)
                    .ok_or(NftTemplateError::MathOverFlow)?
                    .checked_div(FEE_FACTOR)
                    .ok_or(NftTemplateError::MathOverFlow)?;

                approve(
                    CpiContext::new(
                        ctx.accounts.token_program.to_account_info(),
                        Approve {
                            to: ctx
                                .accounts
                                .fee_token_context
                                .sender_token_account
                                .to_account_info(),
                            delegate: ctx.accounts.nft_manager_program.to_account_info(),
                            authority: ctx.accounts.fee_token_context.sender.to_account_info(),
                        },
                    ),
                    fee_amount,
                )?;

                let cpi_ctx = CpiContext::new(
                    ctx.accounts.nft_manager_program.to_account_info(),
                    Charge {
                        auth_datas_account: ctx.accounts.auth_datas_account.to_account_info(),
                        fee_receivers_in_to_chain: ctx
                            .accounts
                            .fee_receivers_in_to_chain
                            .to_account_info(),
                        sender: ctx.accounts.sender.to_account_info(),
                        recipient: ctx.accounts.recipient.to_account_info(),
                        mint_account: ctx.accounts.mint_account.to_account_info(),
                        sender_token_account: ctx.accounts.sender_token_account.to_account_info(),
                        recipient_token_account: ctx
                            .accounts
                            .recipient_token_account
                            .to_account_info(),
                        token_program: ctx.accounts.token_program.to_account_info(),
                        associated_token_program: ctx
                            .accounts
                            .associated_token_program
                            .to_account_info(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                    },
                );
                charge(
                    cpi_ctx,
                    fee_token,
                    price,
                    src_nft,
                    src_token_id,
                    src_chain_id,
                )?;
                let balance_after = ctx.accounts.fee_token_context.sender_token_account.amount;
                if balance_after > balance_before
                    && ctx.accounts.receiver.key() != Pubkey::new_from_array([0u8; 32])
                {
                    transfer(
                        CpiContext::new(
                            ctx.accounts.token_program.to_account_info(),
                            Transfer {
                                from: ctx
                                    .accounts
                                    .fee_token_context
                                    .sender_token_account
                                    .to_account_info(),
                                to: ctx
                                    .accounts
                                    .fee_token_context
                                    .recipient_token_account
                                    .to_account_info(),
                                authority: ctx.accounts.fee_token_context.sender.to_account_info(),
                            },
                        ),
                        balance_after - balance_before, // Transfer amount, adjust for decimals
                    )?;
                }
            }

            // mint_nft
            let mint_cpi_ctx = CpiContext::new(
                ctx.accounts.nft_minter_program.to_account_info(),
                MintNftAccount {
                    payer: ctx.accounts.owner.to_account_info(),
                    metadata_account: ctx.accounts.metadata_account.to_account_info(),
                    edition_account: ctx.accounts.edition_account.to_account_info(),
                    mint_account: ctx.accounts.mint_account.to_account_info(),
                    associated_token_account: ctx
                        .accounts
                        .associated_token_account
                        .to_account_info(),
                    associated_token_program: ctx
                        .accounts
                        .associated_token_program
                        .to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    token_metadata_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            );
            mint_nft(mint_cpi_ctx, nft_name, nft_symbol, nft_uri)?;
            *ctx.accounts.authed_info_account = AuthedInfo {
                nft: ctx.accounts.associated_token_account.key(),
                src_nft,
                src_token_id,
                src_chain_id,
            };

            Ok(())
        }

        _ => Err(Error::from(NftTemplateError::AuthError)),
    }
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
    let msg_hash = keccak(&[
        authed_signer.as_ref(),
        fee_token.as_ref(),
        price.to_le_bytes().as_ref(),
        src_nft.as_ref(),
        src_token_id.to_le_bytes().as_ref(),
        src_chain_id.to_le_bytes().as_ref(),
        ctx.accounts.owner.to_account_info().key.as_ref(),
        nonce.to_le_bytes().as_ref(),
    ]);
    let pk = secp256k1_recover(msg_hash.as_ref(), 0, signature.as_ref())
        .map_err(|err| NftTemplateError::InvalidSignature)?;
    require!(
        keccak(&[pk.0.as_ref()]).0 == authed_signer.as_ref(),
        NftTemplateError::InvalidSigner
    );

    mint_with_auth(
        ctx,
        fee_token,
        price,
        src_nft,
        src_token_id,
        src_chain_id,
        nft_name,
        nft_symbol,
        nft_uri,
    )?;
    Ok(())
}
