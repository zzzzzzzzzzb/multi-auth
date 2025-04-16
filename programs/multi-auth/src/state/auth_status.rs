use anchor_lang::prelude::*;

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
    #[msg("ReceiverNotMatch")]
    ReceiverNotMatch,
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
    // pub src_nft: String,
    pub src_token_id: u64,
    pub src_chain_id: u64,
    pub owner: Pubkey,
    // pub bump: u8,
}
