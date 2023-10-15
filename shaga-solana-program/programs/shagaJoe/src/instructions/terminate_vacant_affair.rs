use crate::{errors::*, seeds::*, states::*};
use anchor_lang::prelude::*;
use clockwork_sdk::{cpi::thread_delete, state::Thread};

#[derive(Accounts)]
pub struct TerminateVacantAffairAccounts<'info> {
    /// checked below if signer == client or thread
    #[account(mut)]
    pub signer: Signer<'info>,
    /// checked below if signer == client or thread
    #[account(mut)]
    pub authority: SystemAccount<'info>,
    /// checked below if signer == client or thread
    #[account(mut, has_one = authority @ ShagaErrorCode::UnauthorizedAffairCreation, seeds = [SEED_LENDER, affair.authority.as_ref()], bump)]
    pub lender: Account<'info, Lender>,
    // /// Verify that only this thread can execute the ThreadTick Instruction
    // #[account(signer, constraint = thread.authority.eq(&thread_authority.key()))]
    // pub thread: Account<'info, Thread>,
    #[account(mut, seeds = [SEED_AFFAIR, affair.authority.as_ref()], bump)]
    pub affair: Account<'info, Affair>,
    #[account(mut, seeds = [SEED_AFFAIR_LIST], bump)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(mut, seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    /// CHECK: checked below
    #[account(mut)]
    pub affair_clockwork_thread: UncheckedAccount<'info>,
    /// The Thread Admin
    /// The authority that was used as a seed to derive the thread address
    /// `thread_authority` should equal `thread.thread_authority`
    /// CHECK: via seeds
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
}

pub fn handle_vacant_affair_termination(ctx: Context<TerminateVacantAffairAccounts>) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let vault = &ctx.accounts.vault;
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let signer = &ctx.accounts.signer;
    let thread_authority = &ctx.accounts.thread_authority;
    let authority = &ctx.accounts.authority;
    let affair_clockwork_thread = &ctx.accounts.affair_clockwork_thread;
    let clockwork_program = &ctx.accounts.clockwork_program;

    // check if signer is the client
    if affair_account.authority != signer.key() {
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
        if !thread_signer.authority.eq(&thread_authority.key()) {
            msg!("Invalid clockwork thread rental termination key.");
            return Err(ShagaErrorCode::InvalidSigner.into());
        }
    } else {
        // TODO: figure out if we should delete the thread if the thread executed the instruction
        let borrow_affair_account = affair_account.clone();

        let (thread_id, _bump) = Pubkey::find_program_address(
            &[
                SEED_THREAD,
                thread_authority.key().as_ref(),
                borrow_affair_account.key().as_ref(),
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
        if clockwork_thread_computed.key() != affair_clockwork_thread.key() {
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
                close_to: authority.to_account_info(),
                thread: affair_clockwork_thread.to_account_info(),
            },
            binding_seeds,
        );

        thread_delete(cpi_ctx)?;
    }
    if affair_account.rental.is_some() {
        msg!("Invalid instruction there is an on going rental.");
        return Err(ShagaErrorCode::InvalidTerminationInstruction.into());
    }

    // Remove the affair from the list of active affairs
    let affair_pubkey = affair_account.key();
    affairs_list_account.remove_affair(affair_pubkey);

    // handled by anchor
    affair_account.close(vault.to_account_info())?;

    // check if lender has some sols to retrieve.
    let lender_account_info = &mut ctx.accounts.lender.to_account_info();
    let lender_rent = Rent::get()?.minimum_balance(lender_account_info.data_len());
    let lender_balance = lender_account_info.lamports() - lender_rent;
    if lender_balance > 0 {
        let authority_account_info = &mut ctx.accounts.authority.to_account_info();
        let mut authority_lamports = authority_account_info.try_borrow_mut_lamports()?;
        let mut lender_lamports = lender_account_info.try_borrow_mut_lamports()?;

        **lender_lamports -= lender_balance;
        **authority_lamports += lender_balance;
    }

    Ok(())
}
