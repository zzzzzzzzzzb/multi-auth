use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::hashv as keccak;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;

#[derive(Accounts)]
#[instruction(approve_signer: [u8; 32])]
pub struct AddSignerContext<'info> {
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + 32 + 1, // 账户空间计算
        seeds = [
            b"signers",
            approve_signer.as_ref(),
        ],
        bump,
    )]
    pub signers_account: Account<'info, ApproveSigner>,

    #[account(mut, signer)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifySignerContext {}

pub fn add_signer(ctx: Context<AddSignerContext>, new_signer: [u8; 32]) -> Result<()> {
    let signers = &mut ctx.accounts.signers_account;
    signers.approve_signer = new_signer;

    Ok(())
}

pub fn verify_signature(
    _ctx: Context<VerifySignerContext>,
    signer: [u8; 32],
    msg: String,
    signature: [u8; 64],
) -> Result<()> {
    let msg_hash = keccak(&[msg.as_ref()]);
    let pk = secp256k1_recover(msg_hash.as_ref(), 0, signature.as_ref())
        .map_err(|err| NftManagerError::InvalidSignature)?;
    require!(
        keccak(&[pk.0.as_ref()]).0 == signer,
        NftManagerError::InvalidSigner
    );
    Ok(())
}
