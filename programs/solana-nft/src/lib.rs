use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("EG3uhAwhJssBfby1fBaYKEs5dc8kPMQJSfU9A7RkEe86");

pub mod contexts;

pub use contexts::*;

#[program]
pub mod mint_nft {
    use super::*;

    pub fn mint_nft(
        ctx: Context<CreateToken>,
        nft_name: String,
        nft_symbol: String,
        nft_uri: String
    ) -> Result<()> {
        ctx.accounts.mint_nft(nft_name, nft_symbol, nft_uri)
    }
}