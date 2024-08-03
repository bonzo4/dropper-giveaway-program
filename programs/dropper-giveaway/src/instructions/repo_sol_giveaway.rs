use anchor_lang::prelude::*;

use crate::{errors::DropperError, state::SolGiveaway};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct RepoSolGiveawayOptions {
    giveaway_id: u64,
    destination_key: Pubkey,
}

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
#[instruction(_options: RepoSolGiveawayOptions)]
pub struct RepoSolGiveaway<'info> {
    #[account(
        mut,
        constraint=signer.key().to_string() == "FNSeGdeCFkULxGd7vSmWqBrQHN6XseCXBp51yXEjhSQQ",
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        constraint=_options.destination_key==destination_account.key()
    )]
    pub destination_account: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"sol_giveaway".as_ref(), &_options.giveaway_id.to_le_bytes()],
        bump,
        constraint=giveaway.winners.is_some()
    )]
    pub giveaway: Account<'info, SolGiveaway>,
    pub system_program: Program<'info, System>,
}
