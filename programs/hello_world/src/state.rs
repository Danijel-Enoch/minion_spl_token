use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct TokenState {
    pub authority: Pubkey,
    pub initialized: bool,
    pub router_contract: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}
