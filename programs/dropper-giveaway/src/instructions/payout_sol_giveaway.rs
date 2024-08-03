use anchor_lang::{prelude::*, solana_program::system_program};

use crate::{errors::DropperError, state::SolGiveaway};

pub fn payout_sol_giveaway(ctx: Context<PayoutSolGiveaway>) -> Result<()> {
    let giveaway = &mut ctx.accounts.giveaway;
    let winner_account_key = ctx.accounts.winner_account.key;

    // Remove winner from the winners list
    {
        let winners = giveaway.winners.as_mut().ok_or(DropperError::Error)?;

        require!(
            winners.contains(winner_account_key),
            DropperError::NotAWinner
        );

        if let Some(index) = winners.iter().position(|x| x == winner_account_key) {
            winners.remove(index);
        } else {
            return Err(DropperError::Error.into());
        }
    }

    // Perform lamport transfers
    giveaway.sub_lamports(giveaway.lamports_amount)?;
    ctx.accounts
        .winner_account
        .add_lamports(giveaway.lamports_amount)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(_giveaway_id: u64)]
pub struct PayoutSolGiveaway<'info> {
    #[account(
        mut,
        signer,
        constraint=signer.key().to_string() == "FNSeGdeCFkULxGd7vSmWqBrQHN6XseCXBp51yXEjhSQQ",
    )]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub winner_account: SystemAccount<'info>,
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
