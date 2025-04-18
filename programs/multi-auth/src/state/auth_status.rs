use anchor_lang::prelude::*;

pub const FEE_FACTOR: u64 = 10_000;

pub const CHAIN_ID: u64 = 1; // TODO

#[error_code]
pub enum NftManagerError {
    #[msg("Auth rejected")]
    AuthRejected,
    #[msg("Charge invalid receiver")]
    ChargeInvalidReceiver,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("InsufficientFunds")]
    InsufficientFunds,
    #[msg("Receiver does not match")]
    ReceiverNotMatch,
    #[msg("Invalid nft owner")]
    InvalidNFTOwner,
    #[msg("Invalid chain id")]
    InvalidChainID,
    #[msg("Invalid to_chain id")]
    InvalidToChainID,
    #[msg("NftManager: feeRatio too high")]
    InvalidReeRatio,
    #[msg("NftManager: invalid fee receiver")]
    InvalidFeeReceiver,
    #[msg("NftManager: invalid signature")]
    InvalidSignature,
    #[msg("NftManager: invalid signer")]
    InvalidSigner,
    #[msg("NftManager: height check failed")]
    HeightCheckFailed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AuthStatus {
    Uninitialized,
    Authed,
    Rejected,
}

#[account]
pub struct AuthStatusAccount {
    pub auth_status: AuthStatus,
    pub src_nft: Pubkey,
    pub src_token_id: u64,
    pub src_chain_id: u64,
    pub user: Pubkey,
}
