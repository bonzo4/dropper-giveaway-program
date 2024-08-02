use anchor_lang::prelude::*;
use instructions::*;

declare_id!("7voKU6J5NiZ947AJjksL3WNRPQQC1TYUyatDai7TiGRH");

pub mod errors;
pub mod instructions;
pub mod state;

#[program]
pub mod dropper_contract {

    use super::*;

    pub fn create_spl_giveaway(
        ctx: Context<CreateSplGivewaway>,
        options: CreateSplGiveawayOptions,
    ) -> Result<()> {
        instructions::create_spl_giveaway(ctx, options)
    }

    pub fn create_sol_giveaway(
        ctx: Context<CreateSolGiveaway>,
        options: CreateSolGiveawayOptions,
    ) -> Result<()> {
        instructions::create_sol_giveaway(ctx, options)
    }

    pub fn payout_spl_giveaway(
        ctx: Context<PayoutSplGiveaway>,
        options: PayoutSplGiveawayOptions,
    ) -> Result<()> {
        instructions::payout_spl_giveaway(ctx, options)
    }

    pub fn payout_sol_giveaway(
        ctx: Context<PayoutSolGiveaway>,
        _options: PayoutSolGiveawayOptions,
    ) -> Result<()> {
        instructions::payout_sol_giveaway(ctx)
    }

    pub fn claim_spl_giveaway(
        ctx: Context<ClaimSplGiveaway>,
        options: ClaimSplGiveawayOptions,
    ) -> Result<()> {
        instructions::claim_spl_giveaway(ctx, options)
    }

    pub fn claim_sol_giveaway(
        ctx: Context<ClaimSolGiveaway>,
        _options: ClaimSolGiveawayOptions,
    ) -> Result<()> {
        instructions::claim_sol_giveaway(ctx)
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
