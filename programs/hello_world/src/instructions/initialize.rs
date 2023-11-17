use crate::state::{InitTokenParams, TokenState};
use anchor_lang::{accounts::signer, prelude::*};
use anchor_spl::{
    token::Token,
    token_2022::{initialize_mint, InitializeMint},
};
use mpl_token_metadata;
use spl_token_2022::extension::transfer_fee::instruction::TransferFeeInstruction;

#[derive(Accounts)]
#[instruction(
    params: InitTokenParams
)]

pub struct InitiazeTokenContext<'info> {
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [b"mint",payer.key().as_ref()],
        bump,
        payer = payer,
        space =  8+8 +32
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    pub signers: Signer<'info>,
}
#[derive(Accounts)]
pub struct SetTrasnferFees<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program_id: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub authority: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub signers: Signer<'info>,
}

pub fn handler(
    ctx: Context<InitiazeTokenContext>,
    tax: u16,
    max_tax: u64,
    deciamls: u8,
) -> Result<()> {
    let seeds = vec!["mint".as_bytes(), b"mint".as_ref()];
    // let signer = vec![seeds.as_slice()];
    let _ini_tx = initialize_mint(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            InitializeMint {
                mint: ctx.accounts.mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[seeds.as_slice()],
        ),
        deciamls,
        &ctx.accounts.authority.key(),
        Some(&ctx.accounts.authority.key()),
    );

    let _set_config_tx = set_config(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            SetTrasnferFees {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                signers: ctx.accounts.signers.clone(),
            },
            &[seeds.as_slice()],
        ),
        tax,
        max_tax,
    );
    Ok(())
}
pub fn set_config<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, SetTrasnferFees<'info>>,
    transfer_fee_basis_points: u16,
    maximum_fee: u64,
) -> Result<()> {
    let aut = ctx.accounts.authority.key();
    let mint = ctx.accounts.mint.key();
    let t_p_id = ctx.accounts.token_program_id.key();
    let signers = ctx.accounts.signers.key();
    let ix = spl_token_2022::extension::transfer_fee::instruction::set_transfer_fee(
        &t_p_id,
        &mint,
        &aut,
        &[&signers],
        transfer_fee_basis_points,
        maximum_fee,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.token_program_id,
            ctx.accounts.mint,
            ctx.accounts.authority,
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

// pub fn set_metadata() {
// spl_token_2022::extension::token_metadata
// }
