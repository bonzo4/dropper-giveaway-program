use anchor_lang::prelude::*;

use crate::{errors::DropperError, state::SolGiveaway};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ClaimSolGiveawayOptions {
    giveaway_id: u64,
}

pub fn claim_sol_giveaway(ctx: Context<ClaimSolGiveaway>) -> Result<()> {
    let giveaway = &mut ctx.accounts.giveaway;
    let signer = &ctx.accounts.signer;
    let signer_key = ctx.accounts.signer.key;

    // Remove winner from the winners list
    {
        let winners = giveaway.winners.as_mut().ok_or(DropperError::Error)?;

        if let Some(index) = winners.iter().position(|x| x == signer_key) {
            winners.remove(index);
        } else {
            return Err(DropperError::Error.into());
        }
    }

    let _ = giveaway.sub_lamports(giveaway.lamports_amount);
    let _ = signer.add_lamports(giveaway.lamports_amount);

    Ok(())
}

#[derive(Accounts)]
#[instruction(_options: ClaimSolGiveawayOptions)]
pub struct ClaimSolGiveaway<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"sol_giveaway".as_ref(), &_options.giveaway_id.to_le_bytes()],
        bump,
        constraint=giveaway.winners.is_some() && giveaway.winners.as_ref().unwrap().contains(signer.key)
    )]
    pub giveaway: Account<'info, SolGiveaway>,
    pub system_program: Program<'info, System>,
}
