use crate::state::*;
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

const FEE_FACTOR: u64 = 10_000;

#[derive(Accounts)]
#[instruction(src_nft: Pubkey, src_token_id: u64, src_chain_id: u64)]
pub struct ChargeContext<'info> {
    #[account(
        init_if_needed,
        payer = sender,
        space = 8 + 32 + 8 + 8 + 8 + 1 + 8 + 1, // 账户空间计算
        seeds = [
            src_nft.as_ref(),
            &src_token_id.to_le_bytes(),
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
            src_nft.as_ref(),
            &src_token_id.to_le_bytes(),
            &src_chain_id.to_le_bytes(),
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

pub fn charge(
    ctx: Context<ChargeContext>,
    fee_assert: Pubkey,
    price: u64,
    src_nft: Pubkey,
    src_token_id: u64,
    src_chain_id: u64,
) -> Result<()> {
    let fee_receiver = &mut ctx.accounts.fee_receivers_in_to_chain;
    let data = &mut ctx.accounts.auth_datas_account;

    require!(
        fee_receiver.receiver == ctx.accounts.recipient.key(),
        NftManagerError::ReceiverNotMatch,
    );

    require!(
        fee_receiver.receiver != Pubkey::new_from_array([0u8; 32]),
        NftManagerError::ChargeInvalidReceiver
    );

    // SOL
    if fee_assert == Pubkey::new_from_array([0u8; 32]) {
        let fee_amount = price
            .checked_mul(data.fee_ratio)
            .ok_or(NftManagerError::MathOverflow)?
            .checked_div(FEE_FACTOR)
            .ok_or(NftManagerError::MathOverflow)?;

        require!(
            ctx.accounts.sender.lamports() >= fee_amount,
            NftManagerError::InsufficientFunds
        );
        if fee_receiver.receiver != Pubkey::new_from_array([0u8; 32]) {
            **ctx
                .accounts
                .sender
                .to_account_info()
                .try_borrow_mut_lamports()? -= fee_amount;
            **ctx
                .accounts
                .recipient
                .to_account_info()
                .try_borrow_mut_lamports()? += fee_amount;
        }
    } else {
        //SPL 代币
        if fee_receiver.receiver != Pubkey::new_from_array([0u8; 32]) {
            let fee_amount = price
                .checked_mul(data.fee_ratio)
                .ok_or(NftManagerError::MathOverflow)?
                .checked_div(FEE_FACTOR)
                .ok_or(NftManagerError::MathOverflow)?;

            // Invoke the transfer instruction on the token program
            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.sender_token_account.to_account_info(),
                        to: ctx.accounts.recipient_token_account.to_account_info(),
                        authority: ctx.accounts.sender.to_account_info(),
                    },
                ),
                fee_amount * 10u64.pow(ctx.accounts.mint_account.decimals as u32), // Transfer amount, adjust for decimals
            )?;
        }
    }

    Ok(())
}
