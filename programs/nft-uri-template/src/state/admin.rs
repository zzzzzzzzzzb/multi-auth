use anchor_lang::prelude::*;

#[account]
pub struct AdminInfo {
    pub auth_admin: Pubkey,
    pub receiver: Pubkey,
}

#[error_code]
pub enum NftTemplateError {
    #[msg("NFT Template: Auth rejected")]
    AuthError,
    #[msg("NFT Template: AuthOPTIsFalse")]
    AuthOPTIsFalse,
    #[msg("NFT Template: InsufficientFunds")]
    InsufficientFunds,
    #[msg("NFT Template: MathOverflow")]
    MathOverFlow,
    #[msg("NFT Template: Invalid Signature")]
    InvalidSignature,
    #[msg("NFT Template: Invalid Signer")]
    InvalidSigner,
    #[msg("NFT Template: OnlyOwner")]
    OnlyOwner,
}

pub const CHAIN_ID: u64 = 1; // TODO

pub const FEE_FACTOR: u64 = 10_000;
