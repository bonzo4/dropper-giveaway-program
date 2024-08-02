use anchor_lang::prelude::*;

use crate::{errors::DropperError, state::SolGiveaway};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct PayoutSolGiveawayOptions {
    giveaway_id: u64,
    winner_key: Pubkey,
}

pub fn payout_sol_giveaway(ctx: Context<PayoutSolGiveaway>) -> Result<()> {
    let giveaway = &mut ctx.accounts.giveaway;
    let winner_account_key = ctx.accounts.winner_account.key;

    // Remove winner from the winners list
    {
        let winners = giveaway.winners.as_mut().ok_or(DropperError::Error)?;

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
#[instruction(_options: PayoutSolGiveawayOptions)]
pub struct PayoutSolGiveaway<'info> {
    #[account(
        mut,
        constraint=signer.key().to_string() == "FNSeGdeCFkULxGd7vSmWqBrQHN6XseCXBp51yXEjhSQQ",
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        constraint=_options.winner_key==winner_account.key()
    )]
    pub winner_account: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"sol_giveaway".as_ref(), &_options.giveaway_id.to_le_bytes()],
        bump,
        constraint=giveaway.winners.is_some() && giveaway.winners.as_ref().unwrap().contains(signer.key)
    )]
    pub giveaway: Account<'info, SolGiveaway>,
    pub system_program: Program<'info, System>,
}
