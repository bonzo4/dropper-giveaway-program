use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::{errors::DropperError, state::SplGiveaway};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ClaimSplGiveawayOptions {
    giveaway_id: u64,
}

pub fn claim_spl_giveaway(
    ctx: Context<ClaimSplGiveaway>,
    options: ClaimSplGiveawayOptions,
) -> Result<()> {
    let giveaway = &mut ctx.accounts.giveaway;
    let giveaway_vault = &ctx.accounts.giveaway_vault;
    let winner_token_account = &ctx.accounts.winner_token_accout;
    let token_mint = &ctx.accounts.token_mint;
    let token_program = &ctx.accounts.token_program;
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

    // transfer spl context
    let spl_transfer_ctx = TransferChecked {
        from: giveaway_vault.to_account_info(),
        to: winner_token_account.to_account_info(),
        authority: giveaway.to_account_info(),
        mint: token_mint.to_account_info(),
    };

    let bump = ctx.bumps.giveaway;
    let seeds = vec![bump];
    let binding = &options.giveaway_id.to_le_bytes();
    let seeds = vec![b"spl_giveaway".as_ref(), binding, seeds.as_slice()];
    let seeds = vec![seeds.as_slice()];
    let seeds = seeds.as_slice();

    let ctx_with_signer =
        CpiContext::new_with_signer(token_program.to_account_info(), spl_transfer_ctx, seeds);

    let _ = transfer_checked(ctx_with_signer, giveaway.reward_amount, token_mint.decimals);

    Ok(())
}

#[derive(Accounts)]
#[instruction(options: ClaimSplGiveawayOptions)]
pub struct ClaimSplGiveaway<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init_if_needed,
        payer=signer,
        associated_token::mint = token_mint,
        associated_token::authority = signer,
    )]
    pub winner_token_accout: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"spl_giveaway".as_ref(), &options.giveaway_id.to_le_bytes()],
        bump,
        constraint=giveaway.winners.is_some() && giveaway.winners.as_ref().unwrap().contains(signer.key)
    )]
    pub giveaway: Account<'info, SplGiveaway>,
    #[account(
        mut,
        seeds = [b"spl_giveawat_vault".as_ref(), giveaway.key().as_ref()],
        bump,
        token::mint=token_mint,
        token::authority=giveaway
    )]
    pub giveaway_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint=token_mint.key()==giveaway.token_address,
    )]
    pub token_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
