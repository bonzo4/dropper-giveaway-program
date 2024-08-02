use anchor_lang::prelude::*;

use super::{vec_size, DISCRIMINATOR_SIZE, PUB_KEY_SIZE, U64_SIZE, U8_SIZE};

#[account]
pub struct SolGiveaway {
    pub bump: u8,
    pub winners_amount: u64,
    pub lamports_amount: u64,
    pub winners: Option<Vec<Pubkey>>,
}

impl SolGiveaway {
    pub const SIZE: usize =
        DISCRIMINATOR_SIZE + U8_SIZE + (U64_SIZE * 2) + vec_size(PUB_KEY_SIZE, 5);
}
