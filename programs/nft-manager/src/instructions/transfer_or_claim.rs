use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::hashv as keccak;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(approve_signer: [u8; 32], src_nft: Pubkey, token_id: u64, chain_id: u64)]
pub struct TransferOrClaimContext<'info> {
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

    #[account(
        mut,
        seeds = [
            b"signers",
            approve_signer.as_ref(),
        ],
        bump,
    )]
    pub signers_account: Account<'info, ApproveSigner>,

    #[account(mut)]
    pub sender: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn transfer_or_claim(
    ctx: Context<TransferOrClaimContext>,
    src_nft: Pubkey,
    src_token_id: u64,
    src_chain_id: u64,
    to_chain_id: u64,
    fee_receiver: Pubkey,
    src_height: u64,
    signature: [u8; 64],
) -> Result<()> {
    require!(to_chain_id == CHAIN_ID, NftManagerError::InvalidToChainID);
    require!(
        fee_receiver == ctx.accounts.sender.key(),
        NftManagerError::InvalidFeeReceiver
    );

    let msg_hash = keccak(&[
        src_nft.as_ref(),
        src_token_id.to_le_bytes().as_ref(),
        src_chain_id.to_le_bytes().as_ref(),
        to_chain_id.to_le_bytes().as_ref(),
        fee_receiver.as_ref(),
        src_height.to_le_bytes().as_ref(),
    ]);
    let pk = secp256k1_recover(msg_hash.as_ref(), 0, signature.as_ref())
        .map_err(|err| NftManagerError::InvalidSignature)?;
    require!(
        keccak(&[pk.0.as_ref()]).0 == ctx.accounts.signers_account.approve_signer,
        NftManagerError::InvalidSigner
    );

    let fee_receiver_in_to_chain = &mut ctx.accounts.fee_receivers_in_to_chain;
    fee_receiver_in_to_chain.receiver = fee_receiver;
    fee_receiver_in_to_chain.height = Clock::get()?.slot;

    msg!(
        "transfer_or_claim: {}, {}, {}, {}, {} ,{}",
        src_nft,
        src_token_id,
        src_chain_id,
        to_chain_id,
        fee_receiver,
        src_height,
    );

    Ok(())
}
