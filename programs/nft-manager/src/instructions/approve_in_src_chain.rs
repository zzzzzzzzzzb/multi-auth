use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
#[instruction(src_nft: Pubkey, token_id: u64, src_chain_id: u64)]
pub struct ApproveInSrcChainContext<'info> {
    #[account(
        seeds = [b"mint", token_id.to_le_bytes().as_ref()],
        bump
    )]
    pub nft_mint: Account<'info, Mint>,

    // 用户的 Token 账户（存储 NFT 所有权）
    #[account(
        associated_token::mint = nft_mint,
        associated_token::authority = sender,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        space = 8 + 32 + 8 + 8 + 8 + 1 + 8 + 1, // 账户空间计算
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

    /// CHECK:
    /// 该字段由程序初始化时自动设置，通过 PDA 派生确保唯一性
    #[account(mut, signer)]
    pub sender: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn approve_in_src_chain(
    ctx: Context<ApproveInSrcChainContext>,
    nft: Pubkey,
    token_id: u64,
    to_chain_id: u64,
    auth_opt: bool,
    fee_ratio: u64,
) -> Result<()> {
    require!(to_chain_id != CHAIN_ID, NftManagerError::InvalidChainID,);
    let nft_owner = ctx.accounts.token_account.owner.key();

    require!(
        ctx.accounts.sender.key() == nft_owner,
        NftManagerError::InvalidNFTOwner
    );

    require!(fee_ratio < FEE_FACTOR, NftManagerError::InvalidReeRatio);

    let auth_datas = &mut ctx.accounts.auth_datas_account;

    auth_datas.nft_address = nft;
    auth_datas.token_id = token_id;
    auth_datas.src_chain_id = CHAIN_ID;
    auth_datas.to_chain_id = to_chain_id;
    auth_datas.auth_opt = auth_opt;
    auth_datas.fee_ratio = fee_ratio;

    let fee_receiver = &mut ctx.accounts.fee_receivers_in_src_chain;
    fee_receiver.receiver = ctx.accounts.sender.key();
    // slot
    let current_slot = Clock::get()?.slot;
    fee_receiver.height = current_slot;

    msg!(
        "approve in src chain: {}, {}, {}, {}, {}, {}, {}, {}, {}",
        nft,
        ctx.accounts.sender.key(),
        token_id,
        CHAIN_ID,
        to_chain_id,
        nft_owner,
        auth_opt,
        fee_ratio,
        current_slot
    );
    Ok(())
}
