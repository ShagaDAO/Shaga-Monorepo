use crate::{seeds::*, states::*};

use anchor_lang::prelude::*;

pub fn handle_lender_initialization(ctx: Context<InitializeLender>) -> Result<()> {
    let lender_account = &mut ctx.accounts.lender;

    // not needed since it would fail in anchor if account already exists.
    // if lender_account.authority != Pubkey::default() {
    //     return Err(ShagaErrorCode::InvalidLender.into());
    // }

    let lender_object = Lender::default();
    lender_account.set_inner(lender_object);
    lender_account.authority = ctx.accounts.payer.key();

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeLender<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, space = Lender::size(), seeds = [SEED_LENDER, payer.key().as_ref()], bump)]
    pub lender: Account<'info, Lender>,
    pub system_program: Program<'info, System>,
}
