use anchor_lang::prelude::*;
use instructions::*;

declare_id!("8s9yegeQK6UJPJq25jquj7vQQx6yJJtKZ7CFLjUaj5cL");

pub mod errors;
pub mod instructions;
pub mod state;

#[program]
pub mod dropper_giveaway {

    use super::*;

    pub fn claim_sol_giveaway(
        ctx: Context<ClaimSolGiveaway>,
        _giveaway_id: u64,
        _creator_key: Pubkey,
    ) -> Result<()> {
        instructions::claim_sol_giveaway(ctx)
    }

    pub fn claim_spl_giveaway(
        ctx: Context<ClaimSplGiveaway>,
        giveaway_id: u64,
        creator_key: Pubkey,
    ) -> Result<()> {
        instructions::claim_spl_giveaway(ctx, giveaway_id, creator_key)
    }

    pub fn create_sol_giveaway(
        ctx: Context<CreateSolGiveaway>,
        options: CreateSolGiveawayOptions,
    ) -> Result<()> {
        instructions::create_sol_giveaway(ctx, options)
    }

    pub fn create_spl_giveaway(
        ctx: Context<CreateSplGivewaway>,
        options: CreateSplGiveawayOptions,
    ) -> Result<()> {
        instructions::create_spl_giveaway(ctx, options)
    }

    pub fn payout_sol_giveaway(
        ctx: Context<PayoutSolGiveaway>,
        _giveaway_id: u64,
        _creator_key: Pubkey,
    ) -> Result<()> {
        instructions::payout_sol_giveaway(ctx)
    }

    pub fn payout_spl_giveaway(
        ctx: Context<PayoutSplGiveaway>,
        giveaway_id: u64,
        creator_key: Pubkey,
    ) -> Result<()> {
        instructions::payout_spl_giveaway(ctx, giveaway_id, creator_key)
    }

    pub fn repo_sol_giveaway(
        ctx: Context<RepoSolGiveaway>,
        _giveaway_id: u64,
        _creator_key: Pubkey,
    ) -> Result<()> {
        instructions::repo_sol_giveaway(ctx)
    }

    pub fn repo_spl_giveaway(
        ctx: Context<RepoSplGiveaway>,
        giveaway_id: u64,
        creator_key: Pubkey,
    ) -> Result<()> {
        instructions::repo_spl_giveaway(ctx, giveaway_id, creator_key)
    }

    pub fn set_sol_giveaway_winners(
        ctx: Context<SetSolGiveawayWinners>,
        options: SetSolGiveawayWinnersOptions,
    ) -> Result<()> {
        instructions::set_sol_giveaway_winners(ctx, options)
    }

    pub fn set_spl_giveaway_winners(
        ctx: Context<SetSplGiveawayWinners>,
        options: SetSplGiveawayWinnersOptions,
    ) -> Result<()> {
        instructions::set_spl_giveaway_winners(ctx, options)
    }
}
