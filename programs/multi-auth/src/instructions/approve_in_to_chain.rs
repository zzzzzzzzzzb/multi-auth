use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::hashv as keccak;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
// use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
#[instruction(approve_signer: [u8; 32], src_nft: Pubkey, token_id: u64, src_chain_id: u64)]
pub struct ApproveInToChainContext<'info> {
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
            b"signers",
            approve_signer.as_ref(),
        ],
        bump,
    )]
    pub signers_account: Account<'info, ApproveSigner>,

    /// CHECK:
    /// 该字段由程序初始化时自动设置，通过 PDA 派生确保唯一性
    #[account(mut, signer)]
    pub sender: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn approve_in_to_chain(
    ctx: Context<ApproveInToChainContext>,
    fee_receiver: Pubkey,
    height: u64,
    signature: [u8; 64],
) -> Result<()> {
    let auth_datas = &mut ctx.accounts.auth_datas_account;
    let fee_receiver_in_to_chain = &mut ctx.accounts.fee_receivers_in_to_chain;

    require!(
        auth_datas.to_chain_id == CHAIN_ID,
        NftManagerError::InvalidToChainID
    );
    require!(
        ctx.accounts.sender.key() == fee_receiver,
        NftManagerError::InvalidFeeReceiver
    );
    require!(
        height > fee_receiver_in_to_chain.height,
        NftManagerError::HeightCheckFailed
    );

    let msg_hash = keccak(&[
        auth_datas.nft_address.as_ref(),
        auth_datas.token_id.to_le_bytes().as_ref(),
        auth_datas.src_chain_id.to_le_bytes().as_ref(),
        auth_datas.to_chain_id.to_le_bytes().as_ref(),
        auth_datas.auth_opt.to_string().as_bytes().as_ref(),
        auth_datas.fee_ratio.to_le_bytes().as_ref(),
        fee_receiver.as_ref(),
        height.to_le_bytes().as_ref(),
    ]);
    let pk = secp256k1_recover(msg_hash.as_ref(), 0, signature.as_ref())
        .map_err(|err| NftManagerError::InvalidSignature)?;
    require!(
        keccak(&[pk.0.as_ref()]).0 == ctx.accounts.signers_account.approve_signer,
        NftManagerError::InvalidSigner
    );

    fee_receiver_in_to_chain.height = height;
    fee_receiver_in_to_chain.receiver = fee_receiver;

    msg!(
        "approve_in_to_chain: {}, {}, {}, {}, {}, {}",
        auth_datas.nft_address,
        auth_datas.token_id,
        auth_datas.src_chain_id,
        auth_datas.to_chain_id,
        fee_receiver,
        auth_datas.auth_opt
    );
    Ok(())
}
