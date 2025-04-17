use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(src_nft: Pubkey, token_id: u64, chain_id: u64)]
pub struct ClaimContext<'info> {
    #[account(
        init_if_needed,
        payer = sender,
        space = 8 + 32 + 8 + 8 + 8 + 1 + 8 + 1, // 账户空间计算
        seeds = [
            b"data",
            src_nft.as_ref(),
            &token_id.to_le_bytes(),
            &chain_id.to_le_bytes(),
        ],
        bump,
    )]
    pub auth_datas_account: Account<'info, AuthData>,

    #[account(
        init_if_needed,
        payer = sender,
        space = 8 + 32 + 8 + 1, // 账户空间计算
        seeds = [
            b"src_chain",
            src_nft.as_ref(),
            &token_id.to_le_bytes(),
        ],
        bump,
    )]
    pub fee_receivers_in_src_chain: Account<'info, FeeReceiver>,

    #[account(
        init_if_needed,
        payer = sender,
        space = 8 + 32 + 8 + 1, // 账户空间计算
        seeds = [
            b"to_chain",
            src_nft.as_ref(),
            &token_id.to_le_bytes(),
            &chain_id.to_le_bytes(),
        ],
        bump,
    )]
    pub fee_receivers_in_to_chain: Account<'info, FeeReceiver>,

    #[account(mut)]
    pub sender: Signer<'info>,
    pub recipient: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    // 用户的 Token 账户（存储 NFT 所有权）
    #[account(
        associated_token::mint = mint_account,
        associated_token::authority = sender,
    )]
    pub token_account: Account<'info, TokenAccount>,

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

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn claim(
    ctx: Context<ClaimContext>,
    nft: Pubkey,
    token_id: u64,
    to_chains_id: Vec<u64>,
) -> Result<()> {
    let nft_owner = ctx.accounts.token_account.owner.key();

    require!(
        ctx.accounts.sender_token_account.to_account_info().key() == nft_owner,
        NftManagerError::InvalidNFTOwner
    );
    require!(
        ctx.accounts.sender_token_account.to_account_info().key()
            != ctx.accounts.fee_receivers_in_src_chain.receiver,
        NftManagerError::InvalidFeeReceiver
    );

    let fee_receivers_in_src_chain = &mut ctx.accounts.fee_receivers_in_src_chain;
    let fee_receivers_in_to_chain = &mut ctx.accounts.fee_receivers_in_to_chain;

    let current_slot = Clock::get()?.slot;

    fee_receivers_in_src_chain.receiver = nft_owner;
    fee_receivers_in_src_chain.height = current_slot;

    for id in to_chains_id.clone() {
        if id == CHAIN_ID {
            fee_receivers_in_to_chain.receiver = nft_owner;
            fee_receivers_in_to_chain.height = current_slot;
        }
    }

    msg!(
        "claim: {}, {}, {}, {}, {}",
        nft,
        nft_owner,
        token_id,
        CHAIN_ID,
        current_slot
    );

    Ok(())
}
