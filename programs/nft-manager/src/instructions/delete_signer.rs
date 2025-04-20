use crate::state::ApproveSigner;
use crate::state::*;
use anchor_lang::prelude::*;

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
    pub signers_account: Account<'info, ApproveSigner>,

    #[account(mut, signer)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn delete_signer(ctx: Context<DeleteSignerContext>) -> Result<()> {
    Ok(())
}
