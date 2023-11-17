use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod minion_token {
    use super::*;
    pub fn initialize_token(
        ctx: Context<InitiazeTokenContext>,
        tax: u16,
        max_tax: u64,
        decimals: u64,
    ) -> Result<()> {
        initialize::handler(ctx, tax, max_tax, decimals.try_into().unwrap())
    }
}
