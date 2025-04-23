use crate::state::{AdminInfo, AuthedInfo, NftTemplateError, CHAIN_ID, FEE_FACTOR};
use {
    anchor_lang::{
        prelude::*,
        solana_program::{
            keccak::hashv as keccak,
            secp256k1_recover::secp256k1_recover,
            system_instruction,
        },
    },
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

#[derive(Accounts)]
#[instruction(src_nft: Pubkey, token_id: u64, src_token_id: u64, src_chain_id: u64)]
pub struct MintContext<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        init_if_needed,
        payer = sender,
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
    #[account(
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
            sender.key().as_ref(),
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
            b"to_chain",
            src_nft.as_ref(),
            &token_id.to_le_bytes(),
            &src_chain_id.to_le_bytes(),
        ],
        bump,
    )]
    pub fee_receivers_in_to_chain: Account<'info, FeeReceiver>,
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
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    pub nft_manager_recv: AccountInfo<'info>,

    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,

    // nft mint
    #[account(mut)]
    pub mint_ata: Account<'info, Mint>,
    #[account(mut)]
    pub sender_nft_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recv_nft_ata: Account<'info, TokenAccount>,
    
    pub nft_manager_program: Program<'info, NftManager>,
    pub token_program: Program<'info, Token>,
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

    match auth_status.auth_status {
        AuthStatus::Authed => {
            require!(auth_data.auth_opt, NftTemplateError::AuthOPTIsFalse);
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
                        recipient: ctx.accounts.recipient_token_account.to_account_info(),
                        sender_token_account: ctx.accounts.sender_token_account.to_account_info(),
                        recipient_token_account: ctx
                            .accounts
                            .recipient_token_account
                            .to_account_info(),
                        token_program: ctx.accounts.token_program.to_account_info(),
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

            } else {
                // SPL
                let balance_before = ctx.accounts.sender_token_account.amount;

                transfer(
                    CpiContext::new(
                        ctx.accounts.token_program.to_account_info(),
                        Transfer {
                            from: ctx
                                .accounts
                                .sender_token_account
                                .to_account_info(),
                            to: ctx
                                .accounts
                                .recipient_token_account
                                .to_account_info(),
                            authority: ctx.accounts.sender.to_account_info(),
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
                                .sender_token_account
                                .to_account_info(),
                            delegate: ctx.accounts.nft_manager_program.to_account_info(),
                            authority: ctx.accounts.sender.to_account_info(),
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
                        recipient: ctx.accounts.recipient_token_account.to_account_info(),
                        sender_token_account: ctx.accounts.sender_token_account.to_account_info(),
                        recipient_token_account: ctx
                            .accounts
                            .recipient_token_account
                            .to_account_info(),
                        token_program: ctx.accounts.token_program.to_account_info(),
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
                let balance_after = ctx.accounts.sender_token_account.amount;
                if balance_after > balance_before
                    && ctx.accounts.receiver.key() != Pubkey::new_from_array([0u8; 32])
                {
                    transfer(
                        CpiContext::new(
                            ctx.accounts.token_program.to_account_info(),
                            Transfer {
                                from: ctx
                                    .accounts
                                    .sender_token_account
                                    .to_account_info(),
                                to: ctx
                                    .accounts
                                    .recipient_token_account
                                    .to_account_info(),
                                authority: ctx.accounts.sender.to_account_info(),
                            },
                        ),
                        balance_after - balance_before, // Transfer amount, adjust for decimals
                    )?;
                }
            }

            // mint_nft
            mint_to(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        mint: ctx.accounts.mint_ata.to_account_info(),
                        to: ctx.accounts.recv_nft_ata.to_account_info(),
                        authority: ctx.accounts.receiver.to_account_info(),
                    },
                ),
                1,
            )?;
            *ctx.accounts.authed_info_account = AuthedInfo {
                nft: ctx.accounts.mint_ata.key(),
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
        ctx.accounts.receiver.to_account_info().key.as_ref(),
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
