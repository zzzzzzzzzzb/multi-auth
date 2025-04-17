use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(src_nft: Pubkey, token_id: u64, chain_id: u64)]
pub struct TransferWrapperContext<'info> {
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

pub fn transfer_wrapper(
    ctx: Context<TransferWrapperContext>,
    nft: Pubkey,
    token_id: u64,
    to: Pubkey,
    to_chains_id: Vec<u64>,
) -> Result<()> {
    // nft转账
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        1,
    )?;

    let fee_receivers_in_src_chain = &mut ctx.accounts.fee_receivers_in_src_chain;
    let fee_receivers_in_to_chain = &mut ctx.accounts.fee_receivers_in_to_chain;

    let receiver = ctx.accounts.recipient_token_account.key();
    let current_slot = Clock::get()?.slot;

    fee_receivers_in_src_chain.receiver = receiver;
    fee_receivers_in_src_chain.height = current_slot;

    for id in to_chains_id.clone() {
        if id == CHAIN_ID {
            fee_receivers_in_to_chain.receiver = receiver;
            fee_receivers_in_to_chain.height = current_slot;
        }
    }

    msg!(
        "transfer_wrapper: {}, {}, {}, {}, {}, {}",
        nft,
        ctx.accounts.sender_token_account.to_account_info().key(),
        ctx.accounts.recipient_token_account.to_account_info().key(),
        token_id,
        CHAIN_ID,
        current_slot
    );

    Ok(())
}
