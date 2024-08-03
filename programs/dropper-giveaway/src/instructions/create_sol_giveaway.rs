use std::ops::Mul;

use crate::state::SolGiveaway;
use anchor_lang::{prelude::*, system_program};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct CreateSolGiveawayOptions {
    giveaway_id: u64,
    winners_amount: u64,
    lamports_amount: u64,
}

pub fn create_sol_giveaway(
    ctx: Context<CreateSolGiveaway>,
    options: CreateSolGiveawayOptions,
) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let dropper_vault = &ctx.accounts.dropper_vault;
    let giveaway = &mut ctx.accounts.giveaway;
    let system_program = &ctx.accounts.system_program;

    let bump = ctx.bumps.giveaway;

    giveaway.bump = bump;
    giveaway.winners_amount = options.winners_amount;
    giveaway.lamports_amount = options.lamports_amount;
    giveaway.winners = None;

    // transfer sol context
    let transfer_ctx = system_program::Transfer {
        from: signer.to_account_info(),
        to: dropper_vault.to_account_info(),
    };

    // transfer sol instruction
    let _ = system_program::transfer(
        CpiContext::new(system_program.to_account_info(), transfer_ctx),
        10_u64.pow(8_u32),
    );

    // transfer sol context
    let pda_transfer_ctx = system_program::Transfer {
        from: signer.to_account_info(),
        to: giveaway.to_account_info(),
    };

    // transfer sol instruction
    let _ = system_program::transfer(
        CpiContext::new(system_program.to_account_info(), pda_transfer_ctx),
        options.lamports_amount.mul(options.winners_amount),
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(options: CreateSolGiveawayOptions)]
pub struct CreateSolGiveaway<'info> {
    #[account(mut, signer)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        address=pubkey!("89LabAxMY6Bn9ak1Uz5LfQZtNybtFhpARatkm7wQHrJE")
    )]
    pub dropper_vault: SystemAccount<'info>,
    #[account(
        init,
        payer = signer,
        space = SolGiveaway::SIZE,
        seeds = [b"sol_giveaway".as_ref(), &options.giveaway_id.to_le_bytes()],
        bump,
    )]
    pub giveaway: Account<'info, SolGiveaway>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
