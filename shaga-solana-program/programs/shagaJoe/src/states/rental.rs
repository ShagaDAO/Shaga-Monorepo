// states/rental.rs

use crate::seeds::SEED_RENTAL;
use anchor_lang::prelude::*;
// TODO: make it possible to extend the rental, if the affair_termination_time allows it

#[account]
#[derive(InitSpace, Debug)]
pub struct Rental {
    pub client: Pubkey,
    pub affair: Pubkey,
    pub rent_amount: u64,
    pub rental_start_time: u64,
    pub rental_termination_time: u64,
    pub rental_clockwork_thread: Pubkey,
}

impl Default for Rental {
    fn default() -> Self {
        Self {
            client: Pubkey::default(),
            affair: Pubkey::default(),
            rent_amount: 0,
            rental_start_time: 0,
            rental_termination_time: 0,
            rental_clockwork_thread: Pubkey::default(),
        }
    }
}

impl Rental {
    pub fn initialize(
        &mut self,
        client: Pubkey,
        affair: Pubkey,
        rent_amount: u64,
        rental_start_time: u64,
        rental_termination_time: u64,
        rental_clockwork_thread: Pubkey,
    ) {
        self.client = client;
        self.affair = affair;
        self.rent_amount = rent_amount;
        self.rental_start_time = rental_start_time;
        self.rental_termination_time = rental_termination_time;
        self.rental_clockwork_thread = rental_clockwork_thread;
    }

    pub fn pda(affair: Pubkey, client: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_RENTAL, affair.as_ref(), client.as_ref()], &crate::ID)
    }

    pub fn size() -> usize {
        8 + Rental::INIT_SPACE
    }

    pub fn deserialize_data(src: &[u8]) -> Result<Rental> {
        let mut p = src;
        let rental = Rental::try_deserialize(&mut p)?;
        Ok(rental)
    }
}
