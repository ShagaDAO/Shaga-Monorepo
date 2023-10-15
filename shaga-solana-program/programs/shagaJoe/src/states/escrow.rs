use crate::seeds::SEED_ESCROW;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Escrow {
    pub locked_amount: u64,
}

impl Escrow {
    pub fn size() -> usize {
        8 + Escrow::INIT_SPACE
    }

    pub fn pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_ESCROW], &crate::ID)
    }
    pub fn deserialize_data(src: &[u8]) -> Result<Escrow> {
        let mut p = src;
        let escrow = Escrow::try_deserialize(&mut p)?;
        Ok(escrow)
    }
}
