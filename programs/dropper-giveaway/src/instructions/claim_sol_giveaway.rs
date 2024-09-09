use anchor_lang::{prelude::*, system_program};

use crate::state::SolGiveaway;

pub fn claim_sol_giveaway(ctx: Context<ClaimSolGiveaway>) -> Result<()> {
    let giveaway = &mut ctx.accounts.giveaway;
    let signer = &ctx.accounts.signer;
    let signer_key = ctx.accounts.signer.key;

    // Remove winner from the winners list
    giveaway.remove_winner(signer_key)?;

    let _ = giveaway.sub_lamports(giveaway.lamports_amount);
    let _ = signer.add_lamports(giveaway.lamports_amount);

    Ok(())
}

#[derive(Accounts)]
#[instruction(_giveaway_id: u64, _creator_key: Pubkey)]
pub struct ClaimSolGiveaway<'info> {
    #[account(mut, signer)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"sol_giveaway".as_ref(), &_giveaway_id.to_le_bytes(), &_creator_key.as_ref()],
        bump,
        constraint=giveaway.winners.is_some()
    )]
    pub giveaway: Account<'info, SolGiveaway>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
