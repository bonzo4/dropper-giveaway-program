use anchor_lang::prelude::*;

use crate::state::SplGiveaway;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct SetSplGiveawayWinnersOptions {
    giveaway_id: u64,
    winner_keys: Vec<Pubkey>,
}

pub fn set_spl_giveaway_winners(
    ctx: Context<SetSplGiveawayWinners>,
    options: SetSplGiveawayWinnersOptions,
) -> Result<()> {
    let giveaway = &mut ctx.accounts.giveaway;

    giveaway.winners = Some(options.winner_keys);

    Ok(())
}

#[derive(Accounts)]
#[instruction(options: SetSplGiveawayWinnersOptions)]
pub struct SetSplGiveawayWinners<'info> {
    #[account(
        mut,
        constraint=signer.key().to_string() == "FNSeGdeCFkULxGd7vSmWqBrQHN6XseCXBp51yXEjhSQQ",
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"spl_giveaway".as_ref(), &options.giveaway_id.to_le_bytes()],
        bump,
        constraint=giveaway.winners.is_none() && options.winner_keys.len() as u64 == giveaway.winners_amount
    )]
    pub giveaway: Account<'info, SplGiveaway>,
    pub system_program: Program<'info, System>,
}
