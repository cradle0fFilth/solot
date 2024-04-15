use crate::constants::{ADMIN_PUBKEY, INIT_SOLOT};
use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct InitiaLizeLottery<'info> {
    #[account(mut, address = ADMIN_PUBKEY)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + 32)]
    pub solot_data: Account<'info, SolotData>,
    #[account(init, payer = user, space = 8 + LossLotteryTickets::LEN)]
    pub loss_lottery_tickets: Box<Account<'info, LossLotteryTickets>>,
    #[account(init, payer = user, space = 8 + WinLotteryTickets::LEN)]
    pub win_lottery_tickets: Box<Account<'info, WinLotteryTickets>>,

    #[account(
        mut,
        seeds = [b"reward"],
        bump,
    )]
    // PDA
    pub token_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user
    )]
    pub solot_token_pool_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitiaLizeLottery<'info> {
    pub fn handler(ctx: Context<InitiaLizeLottery>) -> Result<()> {
        let solot_data = &mut ctx.accounts.solot_data;
        solot_data.total_ticket = 0;
        solot_data.prize_pool = 0;

        // mint 10000 solot to the pool
        let seeds = b"reward";
        let bump = ctx.bumps.token_mint;
        let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];
        mint_solot_token(&ctx.accounts.token_program.to_account_info(), &ctx.accounts.token_mint,
            &ctx.accounts.solot_token_pool_account.to_account_info(),  signer, INIT_SOLOT)?;
        Ok(())
    }
}
