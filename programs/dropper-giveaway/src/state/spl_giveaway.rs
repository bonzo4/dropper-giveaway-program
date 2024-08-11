use anchor_lang::prelude::*;

use crate::errors::DropperError;

use super::{vec_size, DISCRIMINATOR_SIZE, PUB_KEY_SIZE, U64_SIZE, U8_SIZE};

#[account]
pub struct SplGiveaway {
    pub bump: u8,
    pub token_address: Pubkey,
    pub winners_amount: u64,
    pub reward_amount: u64,
    pub winners: Option<Vec<Pubkey>>,
}

impl SplGiveaway {
    pub const SIZE: usize = DISCRIMINATOR_SIZE
        + U8_SIZE
        + PUB_KEY_SIZE
        + (U64_SIZE * 2)
        + vec_size(PUB_KEY_SIZE, 5)
        + 8;

    pub fn remove_winner(&mut self, signer_key: &Pubkey) -> Result<()> {
        let winners = self.winners.as_mut().ok_or(DropperError::Error)?;

        require!(winners.contains(signer_key), DropperError::NotAWinner);

        if let Some(index) = winners.iter().position(|x| x == signer_key) {
            winners.remove(index);
        } else {
            return Err(DropperError::Error.into());
        }

        Ok(())
    }
}
