use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(src_nft: Pubkey, token_id: u64, chain_id: u64)]
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
    sigs: &[u8],
) -> Result<()> {
    Ok(())
}