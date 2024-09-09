use anchor_lang::{prelude::*, system_program};

use crate::state::SplGiveaway;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct SetSplGiveawayWinnersOptions {
    giveaway_id: u64,
    winner_keys: Vec<Pubkey>,
    creator_key: Pubkey,
}

pub fn set_spl_giveaway_winners(
    ctx: Context<SetSplGiveawayWinners>,
    options: SetSplGiveawayWinnersOptions,
) -> Result<()> {
    let giveaway = &mut ctx.accounts.giveaway;

    let winner_dif = giveaway.winners_amount - options.winner_keys.len() as u64;

    let temp_winners = vec![Pubkey::default(); winner_dif as usize];

    let winner_keys = options.winner_keys.iter().chain(temp_winners.iter());

    giveaway.winners = Some(winner_keys.cloned().collect());

    Ok(())
}

#[derive(Accounts)]
#[instruction(options: SetSplGiveawayWinnersOptions)]
pub struct SetSplGiveawayWinners<'info> {
    #[account(
        mut,
        signer,
        constraint=signer.key().to_string() == "FNSeGdeCFkULxGd7vSmWqBrQHN6XseCXBp51yXEjhSQQ",
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"spl_giveaway".as_ref(), &options.giveaway_id.to_le_bytes(), &options.creator_key.as_ref()],
        bump,
        constraint=giveaway.winners.is_none() && options.winner_keys.len() as u64 <= giveaway.winners_amount
    )]
    pub giveaway: Account<'info, SplGiveaway>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
