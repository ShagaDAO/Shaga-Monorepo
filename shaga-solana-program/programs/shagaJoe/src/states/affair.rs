// states/affair.rs

use crate::errors::ShagaErrorCode;
use crate::seeds::SEED_AFFAIR;
use anchor_lang::prelude::*;

#[derive(InitSpace, Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AffairState {
    Unavailable,
    Available,
}

impl Default for AffairState {
    fn default() -> Self {
        AffairState::Available
    }
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Affair {
    pub authority: Pubkey,
    pub client: Pubkey,
    pub rental: Option<Pubkey>,
    #[max_len(15)]
    pub ip_address: String, // puffed to 15 characters (max)
    #[max_len(64)]
    pub cpu_name: String, // puffed to 64 characters (max)
    #[max_len(64)]
    pub gpu_name: String, // puffed to 64 characters (max)
    pub total_ram_mb: u32,
    // in LAMPORTS_PER_SOL
    pub sol_per_hour: u64,
    pub affair_state: AffairState,
    pub affair_termination_time: u64,
    pub active_rental_start_time: u64,
    pub due_rent_amount: u64,
    //pub active_locked_amount: u64,
}

impl Default for Affair {
    fn default() -> Self {
        Self {
            authority: Pubkey::default(),
            client: Pubkey::default(),
            rental: Option::from(Pubkey::default()),
            ip_address: "".to_string(),
            cpu_name: "".to_string(),
            gpu_name: "".to_string(),
            total_ram_mb: 0,
            sol_per_hour: 0,
            affair_state: AffairState::default(),
            affair_termination_time: 0,
            active_rental_start_time: 0,
            due_rent_amount: 0,
            //active_locked_amount: 0,
        }
    }
}

impl Affair {
    pub fn join(&mut self, rental_key: Pubkey) -> Result<()> {
        if self.affair_state != AffairState::Available {
            msg!("Affair is not available for joining.");
            return Err(ShagaErrorCode::AffairAlreadyJoined.into());
        }

        self.rental = Some(rental_key);
        self.affair_state = AffairState::Unavailable;
        Ok(())
    }

    pub fn pda(owner: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_AFFAIR, owner.as_ref()], &crate::ID)
    }

    pub fn size() -> usize {
        8 + Affair::INIT_SPACE
    }

    pub fn can_join(&self) -> bool {
        self.affair_state == AffairState::Available
    }

    pub fn deserialize_data(src: &[u8]) -> Result<Affair> {
        let mut p = src;
        let affair = Affair::try_deserialize(&mut p)?;
        Ok(affair)
    }
}
