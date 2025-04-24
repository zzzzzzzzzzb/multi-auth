use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::hashv as keccak;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;

#[derive(Accounts)]
#[instruction(authed_signer: [u8; 32])]
pub struct AddSignerContext<'info> {
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + 32 + 1, // 账户空间计算
        seeds = [
            b"authed_signers",
            authed_signer.as_ref(),
        ],
        bump,
    )]
    pub signers_account: Account<'info, AuthedSigner>,

    #[account(mut, signer)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn add_signer(ctx: Context<AddSignerContext>, new_signer: [u8; 32]) -> Result<()> {
    *ctx.accounts.signers_account = AuthedSigner {
        authed_signer: new_signer,
    };
    Ok(())
}

#[derive(Accounts)]
#[instruction(approve_signer: [u8; 32])]
pub struct DeleteSignerContext<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [
            b"signers",
            approve_signer.as_ref(),
        ],
        bump,
    )]
    pub signers_account: Account<'info, AuthedSigner>,

    #[account(mut, signer)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn delete_signer(ctx: Context<DeleteSignerContext>) -> Result<()> {
    Ok(())
}