use crate::{errors::*, states::*};
use anchor_lang::prelude::*;

pub fn check_can_start_rental(affair: &Affair) -> Result<()> {
    if !affair.can_join() {
        msg!("The is already rented");
        return Err(ShagaErrorCode::AffairAlreadyJoined.into());
    } else {
        Ok(())
    }
}

pub fn check_client_already_in_affair(affair: &Affair, client_key: &Pubkey) -> Result<()> {
    if affair.client == *client_key {
        msg!("Client already has rental active");
        return Err(ShagaErrorCode::ClientAlreadyInAffair.into());
    } else {
        Ok(())
    }
}

pub fn check_sufficient_funds_in_escrow(escrow: &Escrow, rent_amount: u64) -> Result<()> {
    if escrow.locked_amount < rent_amount {
        msg!("Insufficient funds.");
        return Err(ShagaErrorCode::InsufficientFunds.into());
    } else {
        Ok(())
    }
}
