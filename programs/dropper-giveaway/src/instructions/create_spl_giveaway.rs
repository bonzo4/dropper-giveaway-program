use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::state::SplGiveaway;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct CreateSplGiveawayOptions {
    giveaway_id: u64,
    token_address: Pubkey,
    winners_amount: u64,
    reward_amount: u64,
}

pub fn create_spl_giveaway(
    ctx: Context<CreateSplGivewaway>,
    options: CreateSplGiveawayOptions,
) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let dropper_vault = &ctx.accounts.dropper_vault;
    let giveaway = &mut ctx.accounts.giveaway;
    let giveaway_vault = &ctx.accounts.giveaway_vault;
    let token_payer_account = &ctx.accounts.token_payer_account;
    let token_mint = &ctx.accounts.token_mint;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;

    let bump = ctx.bumps.giveaway;

    // intialize giveaway
    giveaway.bump = bump;
    giveaway.token_address = options.token_address;
    giveaway.winners_amount = options.winners_amount;
    giveaway.reward_amount = options.reward_amount;
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

    //transfer spl context
    let spl_transfer_ctx = TransferChecked {
        from: token_payer_account.to_account_info(),
        to: giveaway_vault.to_account_info(),
        mint: token_mint.to_account_info(),
        authority: signer.to_account_info(),
    };

    // transfer spl instruction
    let _ = transfer_checked(
        CpiContext::new(token_program.to_account_info(), spl_transfer_ctx),
        options.reward_amount * options.winners_amount,
        token_mint.decimals,
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(options: CreateSplGiveawayOptions)]
pub struct CreateSplGivewaway<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        constraint=dropper_vault.key().to_string() == "89LabAxMY6Bn9ak1Uz5LfQZtNybtFhpARatkm7wQHrJE"
    )]
    pub dropper_vault: SystemAccount<'info>,
    #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=signer
    )]
    pub token_payer_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = signer,
        space = SplGiveaway::SIZE,
        seeds = [b"spl_giveaway".as_ref(), &options.giveaway_id.to_le_bytes()],
        bump,
    )]
    pub giveaway: Account<'info, SplGiveaway>,
    #[account(
        init,
        payer = signer,
        seeds = [b"spl_giveawat_vault".as_ref(), giveaway.key().as_ref()],
        bump,
        token::mint=token_mint,
        token::authority=giveaway
    )]
    pub giveaway_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint=token_mint.key()==options.token_address,
    )]
    pub token_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
