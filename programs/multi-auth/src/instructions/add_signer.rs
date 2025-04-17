use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Approve;

#[derive(Accounts)]
#[instruction()]
pub struct AddSignerContext<'info> {
    #[account(
        init_if_needed,
        payer = sender,
        space = 8 + 32 + 1, // 账户空间计算
        seeds = [
            b"signers",
        ],
        bump,
    )]
    pub signers_account: Account<'info, ApproveSigner>,

    #[account(mut, signer)]
    pub sender: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn add_signer(
    ctx: Context<AddSignerContext>,
    new_signer: Pubkey,
) -> Result<()> {
    // let signers = &mut ctx.accounts.signers_account;
    // signers.approve_signers.push(new_signer);
    // 
    // Pubkey::find_program_address()

    Ok(())
}
