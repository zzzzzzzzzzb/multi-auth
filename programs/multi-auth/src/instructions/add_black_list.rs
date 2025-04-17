use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(user: Pubkey, src_nft: Pubkey, src_token_id: u64, src_chain_id: u64)]
pub struct AddBlackListContext<'info> {
    // TODO
    #[account(
        mut,
        seeds = [
            src_nft.as_ref(),
            &src_token_id.to_le_bytes(),
            &src_chain_id.to_le_bytes(),
        ],
        bump,
    )]
    pub fee_receivers_in_to_chain: Account<'info, FeeReceiver>,

    #[account(
        mut,
        seeds = [
            user.as_ref(),
            src_nft.as_ref(),
            &src_token_id.to_le_bytes(),
            &src_chain_id.to_le_bytes(),
        ],
        bump,
    )]
    pub auth_status_account: Account<'info, AuthStatusAccount>,

    /// CHECK:
    /// 该字段由程序初始化时自动设置，通过 PDA 派生确保唯一性
    #[account(signer)]
    pub sender: AccountInfo<'info>,

    #[account()]
    pub payer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn add_black_list(
    ctx: Context<AddBlackListContext>,
    user: Pubkey,
    src_nft: Pubkey,
    src_token_id: u64,
    src_chain_id: u64,
) -> Result<()> {
    let auth_account = &mut ctx.accounts.auth_status_account;

    require!(
        ctx.accounts.fee_receivers_in_to_chain.receiver == ctx.accounts.sender.key(),
        NftManagerError::InvalidNFTOwner,
    );

    auth_account.auth_status = AuthStatus::Rejected;
    auth_account.user = user;
    auth_account.src_nft = src_nft;
    auth_account.src_token_id = src_token_id;
    auth_account.src_chain_id = src_chain_id;

    Ok(())
}
