use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(ticket_id: u32)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub win_tickets: Account<'info, WinLotteryTickets>,
    #[account(mut,
        seeds = [&ticket_id.to_be_bytes(), player.key().as_ref()],
        bump,)]
    pub ticket_associated_account: Account<'info, TicketAssociatedAccount>,
    #[account(
        mut,
        seeds = [b"reward"],
        bump,
    )]
    // PDA
    pub token_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = player,
        associated_token::mint = token_mint,
        associated_token::authority = player
    )]
    pub solot_token_pool_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Redeem<'info> {
    pub fn handler(ctx: Context<Redeem>, ticket_id: u32, amount: u64) -> Result<()> {
        let win_ticket_vec = &mut ctx.accounts.win_tickets.win_lottery_tickets;

        // 遍历中奖彩票信息，找到指定的彩票
        for win_ticket in win_ticket_vec.iter_mut() {
            if win_ticket.ticket_id == ticket_id {
                match win_ticket.reward_type {
                    RewardType::Reward1 => {
                        require_gte!(win_ticket.sol_reward, amount, SolotError::ExceededWinningAmount);
                        // 向中奖彩票的持有者转账sol
                    }

                    RewardType::Reward2 => {
                        require_gte!(win_ticket.solot_reward, amount, SolotError::ExceededWinningAmount);
                        // mint solot 到player 的关联代币账户

                    }

                    RewardType::Reward3 => {

                    }
                    _ => {
                        msg!("Invalid reward type")
                    }
                }
            }
        }
        Ok(())
    }
}