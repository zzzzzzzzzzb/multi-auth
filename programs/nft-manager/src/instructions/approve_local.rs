use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
#[instruction(approve_signer: [u8; 32], src_nft: Pubkey, token_id: u64, src_chain_id: u64)]
pub struct ApproveLocalContext<'info> {
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
            &src_chain_id.to_le_bytes(),
        ],
        bump,
    )]
    pub fee_receivers_in_to_chain: Account<'info, FeeReceiver>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = sender,
    )]
    pub sender_token_account: Account<'info, TokenAccount>,

    /// CHECK:
    /// 该字段由程序初始化时自动设置，通过 PDA 派生确保唯一性
    #[account(mut, signer)]
    pub sender: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn approve_local(
    ctx: Context<ApproveLocalContext>,
    nft: Pubkey,
    token_id: u64,
    auth_opt: bool,
    fee_ratio: u64,
) -> Result<()> {
    let nft_owner = ctx.accounts.token_account.owner.key();

    require!(
        ctx.accounts.sender_token_account.to_account_info().key() == nft_owner,
        NftManagerError::InvalidNFTOwner
    );
    let auth_datas = &mut ctx.accounts.auth_datas_account;
    auth_datas.nft_address = nft;
    auth_datas.token_id = token_id;
    auth_datas.src_chain_id = CHAIN_ID;
    auth_datas.to_chain_id = CHAIN_ID;
    auth_datas.fee_ratio = fee_ratio;
    auth_datas.auth_opt = auth_opt;

    let current_slot = Clock::get()?.slot;
    let fee_receivers_in_src_chain = &mut ctx.accounts.fee_receivers_in_src_chain;
    fee_receivers_in_src_chain.receiver = nft_owner;
    fee_receivers_in_src_chain.height = current_slot;

    let fee_receivers_in_to_chain = &mut ctx.accounts.fee_receivers_in_to_chain;
    fee_receivers_in_to_chain.receiver = nft_owner;
    fee_receivers_in_to_chain.height = current_slot;

    msg!(
        "approve_local: {}, {}, {}, {}",
        nft,
        token_id,
        nft_owner,
        auth_opt
    );
    Ok(())
}
