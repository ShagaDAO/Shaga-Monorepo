use crate::{errors::*, seeds::*, states::*};
use anchor_lang::prelude::*;
use clockwork_sdk::{cpi::thread_delete, state::Thread};
use solana_program::native_token::Sol;

#[derive(PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum RentalTerminationAuthority {
    Clockwork,
    Client,
    // handled in terminate_affair instructions
    // TerminateAffair,
}

#[derive(Accounts)]
pub struct EndRentalAccounts<'info> {
    /// checked below if signer == client or thread
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: checked below
    #[account(mut)]
    pub client: UncheckedAccount<'info>,
    /// CHECK: checked below
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: AccountInfo<'info>,
    #[account(mut, seeds = [SEED_LENDER, affair.authority.as_ref()], bump)]
    pub lender: Account<'info, Lender>,
    #[account(mut, seeds = [SEED_AFFAIR, affair.authority.as_ref()], bump)]
    pub affair: Account<'info, Affair>,
    #[account(mut, seeds = [SEED_AFFAIR_LIST], bump)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(mut, seeds = [SEED_ESCROW, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut, seeds = [SEED_RENTAL, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub rental: Account<'info, Rental>,
    #[account(mut, seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    /// CHECK: checked below
    #[account(mut)]
    pub rental_clockwork_thread: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
}

/// can be done by either the client, clockwork, or affair authority
pub fn handle_ending_rental(
    ctx: Context<EndRentalAccounts>,
    termination_by: RentalTerminationAuthority,
) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &mut ctx.accounts.rental;
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let lender_account = &ctx.accounts.lender;
    let client = &ctx.accounts.client;
    let vault = &ctx.accounts.vault;
    let signer = &ctx.accounts.signer;
    let thread_authority = &ctx.accounts.thread_authority;
    let rental_clockwork_thread = &ctx.accounts.rental_clockwork_thread;
    let clockwork_program = &ctx.accounts.clockwork_program;

    // check if signer is the client
    if client.key() != signer.key() || termination_by == RentalTerminationAuthority::Clockwork {
        // check if signer is thread. if it is not then fail early.
        // serialize the signer into a thread or fail.
        let thread_data = &mut &**signer.try_borrow_data()?;
        let thread_signer_result = Thread::try_deserialize(thread_data);
        let thread_signer = if thread_signer_result.is_ok() {
            thread_signer_result.unwrap()
        } else {
            msg!("Could not deserialize clockwork thread termination key.");
            return Err(ShagaErrorCode::InvalidSigner.into());
        };
        if rental_account.rental_clockwork_thread != signer.key()
            && !thread_signer.authority.eq(&thread_authority.key())
        {
            msg!("Invalid clockwork thread rental termination key.");
            return Err(ShagaErrorCode::InvalidTerminationTime.into());
        }
    }
    // fail early if rental does not exist
    if affair_account.rental.is_none() {
        msg!("No rental found. possibly already terminated or ended by the client.");
        return Err(ShagaErrorCode::InvalidTerminationTime.into());
    }

    // TODO: figure out if we should delete the thread if the thread executed the instruction
    let (thread_id, _bump) = Pubkey::find_program_address(
        &[
            SEED_THREAD,
            thread_authority.key().as_ref(),
            rental_account.key().as_ref(),
        ],
        ctx.program_id,
    );
    let thread_id_vec: Vec<u8> = thread_id.to_bytes().to_vec();

    // Step 6: Fetch the bump seed associated with the authority
    let (clockwork_thread_computed, _bump) = Pubkey::find_program_address(
        &[
            SEED_THREAD,
            thread_authority.key().as_ref(),
            thread_id_vec.as_slice().as_ref(),
        ],
        &clockwork_program.key(),
    );
    if clockwork_thread_computed.key() != rental_clockwork_thread.key() {
        msg!("Invalid clockwork thread affair termination key.");
        return Err(ShagaErrorCode::InvalidTerminationTime.into());
    }

    let ta_bump = *ctx.bumps.get("thread_authority").unwrap();
    let cpi_signer: &[&[u8]] = &[SEED_AUTHORITY_THREAD, &[ta_bump]];
    let binding_seeds = &[cpi_signer];
    // Step 7: Create the termination thread
    let cpi_ctx = CpiContext::new_with_signer(
        clockwork_program.to_account_info(),
        clockwork_sdk::cpi::ThreadDelete {
            authority: thread_authority.to_account_info(),
            close_to: client.to_account_info(),
            thread: rental_clockwork_thread.to_account_info(),
        },
        binding_seeds,
    );

    thread_delete(cpi_ctx)?;

    // Step 1: Calculate the actual time server was used (in hours)
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // if current_time is larger than or equal to rental_termination_time that means the actual payment
    // is the total escrowed amount.
    if current_time >= rental_account.rental_termination_time {
        let lender_account_info = &mut lender_account.to_account_info();
        let escrow_account_info = &mut escrow_account.to_account_info();

        let mut escrow_lamports = escrow_account_info.try_borrow_mut_lamports()?;
        let mut lender_lamports = lender_account_info.try_borrow_mut_lamports()?;
        let actual_payment = rental_account.rent_amount;
        **escrow_lamports -= actual_payment;
        **lender_lamports += actual_payment;
    } else {
        let scaling_factor = 100_u64;
        let actual_time =
            (current_time as f64 - affair_account.active_rental_start_time as f64) / 3600.0;
        let scaled_rental_duration = (actual_time * scaling_factor as f64) as u64;

        let actual_payment = scaled_rental_duration
            .checked_mul(affair_account.sol_per_hour)
            .ok_or(ShagaErrorCode::NumericalOverflow)?
            .checked_div(scaling_factor)
            .ok_or(ShagaErrorCode::NumericalOverflow)?;
        let refund_amount = affair_account
            .due_rent_amount
            .checked_sub(actual_payment)
            .ok_or(ShagaErrorCode::NumericalOverflow)?;

        let client_account_info = &mut client.to_account_info();
        let lender_account_info = &mut lender_account.to_account_info();
        let escrow_account_info = &mut escrow_account.to_account_info();

        let mut escrow_lamports = escrow_account_info.try_borrow_mut_lamports()?;
        let mut lender_lamports = lender_account_info.try_borrow_mut_lamports()?;
        let mut client_lamports = client_account_info.try_borrow_mut_lamports()?;

        **escrow_lamports -= refund_amount + actual_payment;
        **lender_lamports += actual_payment;
        **client_lamports += refund_amount;

        msg!("refund_amount: {}", Sol(refund_amount));
        msg!("actual_payment: {}", Sol(actual_payment));
        msg!("escrow_lamports: {}", Sol(**escrow_lamports));
    }

    // Step 5: Update lender karma points based on who terminated the affair
    let lender_account = &mut ctx.accounts.lender;
    lender_account.give_thumbs_up();

    // Step 6: Update affair state to indicate it's Available
    affair_account.affair_state = AffairState::Available;
    affair_account.rental = None;
    affair_account.client = Pubkey::default();

    // Step 7: Add Affair Back to Affair List
    let affair_pubkey = affair_account.key();
    affairs_list_account.register_affair(affair_pubkey)?;
    escrow_account.locked_amount = 0;

    // since rent ended and we already transfered the total.
    // we can close these accounts below.
    escrow_account.close(vault.to_account_info())?;
    rental_account.close(vault.to_account_info())?;

    Ok(())
}
