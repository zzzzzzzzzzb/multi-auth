use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(src_nft: Pubkey, src_token_id: u64, src_chain_id: u64)]
pub struct RegisterContext<'info> {
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + 1 + 32 + 8 + 8 + 32 + 1, // 账户空间计算
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

    // // 支付账户
    // #[account(mut)]
    // pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn register(
    ctx: Context<RegisterContext>,
    src_nft: Pubkey,
    src_token_id: u64,
    src_chain_id: u64,
) -> Result<()> {
    let auth_account = &mut ctx.accounts.auth_status_account;

    // 检查状态
    require!(
        auth_account.auth_status != AuthStatus::Rejected,
        NftManagerError::AuthRejected
    );

    // 更新状态
    auth_account.auth_status = AuthStatus::Authed;
    auth_account.user = ctx.accounts.owner.key();
    auth_account.src_nft = src_nft;
    auth_account.src_token_id = src_token_id;
    auth_account.src_chain_id = src_chain_id;

    msg!(
        "register success:{} {} {} {}",
        auth_account.user,
        auth_account.src_chain_id,
        auth_account.src_nft,
        auth_account.src_token_id
    );

    Ok(())
}
