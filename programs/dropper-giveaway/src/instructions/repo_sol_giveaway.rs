use anchor_lang::{prelude::*, system_program};

use crate::{errors::DropperError, state::SolGiveaway};

pub fn repo_sol_giveaway(ctx: Context<RepoSolGiveaway>) -> Result<()> {
    let giveaway = &mut ctx.accounts.giveaway;
    let destination_account = &mut ctx.accounts.destination_account;

    let winners = giveaway.winners.as_ref().ok_or(DropperError::Error)?;

    let leftover_winners = winners.len() as u64;

    require!(leftover_winners > 0, DropperError::NotAWinner);

    // Perform lamport transfers
    giveaway.sub_lamports(giveaway.lamports_amount * leftover_winners)?;
    destination_account.add_lamports(giveaway.lamports_amount * leftover_winners)?;

    giveaway.winners = Some(vec![]);

    Ok(())
}

#[derive(Accounts)]
#[instruction(_giveaway_id: u64)]
pub struct RepoSolGiveaway<'info> {
    #[account(
        mut,
        signer,
        constraint=signer.key().to_string() == "FNSeGdeCFkULxGd7vSmWqBrQHN6XseCXBp51yXEjhSQQ",
    )]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub destination_account: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"sol_giveaway".as_ref(), &_giveaway_id.to_le_bytes()],
        bump,
        constraint=giveaway.winners.is_some()
    )]
    pub giveaway: Account<'info, SolGiveaway>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
