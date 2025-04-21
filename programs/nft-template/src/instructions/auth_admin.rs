use crate::state::AdminInfo;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitAdminAndReceiverContext<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 1,
        seeds = [
            b"admin_receiver",
        ],
        bump,
    )]
    pub admin_receiver_account: Account<'info, AdminInfo>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn init_admin_and_receiver(
    ctx: Context<InitAdminAndReceiverContext>,
    admin: Pubkey,
    recv: Pubkey,
) -> Result<()> {
    *ctx.accounts.admin_receiver_account = AdminInfo {
        auth_admin: admin,
        receiver: recv,
    };
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateAdminAndReceiverContext<'info> {
    #[account(
        mut,
        seeds = [
            b"admin_receiver",
        ],
        bump,
    )]
    pub admin_receiver_account: Account<'info, AdminInfo>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn update_admin(ctx: Context<UpdateAdminAndReceiverContext>, admin: Pubkey) -> Result<()> {
    let info = &mut ctx.accounts.admin_receiver_account;
    let old_admin = info.auth_admin.clone();
    info.auth_admin = admin;
    msg!("update admin, {}, {}", old_admin, admin);
    Ok(())
}

pub fn update_receiver(
    ctx: Context<UpdateAdminAndReceiverContext>,
    receiver: Pubkey,
) -> Result<()> {
    let info = &mut ctx.accounts.admin_receiver_account;
    let old_receiver = info.receiver;
    info.receiver = receiver;
    msg!("update receiver, {}, {}", old_receiver, receiver);
    Ok(())
}
