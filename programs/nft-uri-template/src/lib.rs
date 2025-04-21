mod state;
mod instructions;

use anchor_lang::prelude::*;

declare_id!("CvwbcAmiPvUwyvCaFM24K23RuKPckLbUvSFj1KxxwP9Y");

#[program]
pub mod nft_uri_template {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
