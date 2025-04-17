use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(src_nft: Pubkey, src_token_id: u64, src_chain_id: u64)]
pub struct UnRegisterContext<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [
            b"status",
            owner.key().as_ref(),
            src_nft.as_ref(),
            &src_token_id.to_le_bytes(),
            &src_chain_id.to_le_bytes(),
        ],
        bump,
    )]
    pub auth_status_account: Account<'info, AuthStatusAccount>,

    // pub src_nft: Account<'info, Mint>, // 假设为 SPL Token 的 Mint 账户
    /// CHECK:
    /// 该字段由程序初始化时自动设置，通过 PDA 派生确保唯一性
    #[account(mut, signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn unregister(
    ctx: Context<UnRegisterContext>,
    _src_nft: Pubkey,
    _src_token_id: u64,
    _src_chain_id: u64,
) -> Result<()> {
    let auth_account = &mut ctx.accounts.auth_status_account;

    // 检查状态
    require!(
        auth_account.auth_status != AuthStatus::Rejected,
        NftManagerError::AuthRejected
    );
    // 自动close账户
    Ok(())
}
