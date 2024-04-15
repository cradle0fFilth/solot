use anchor_lang::prelude::*;
use crate::state::*;
use crate::utils::*;
use crate::constants::*;
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
    pub solot_data: Account<'info, SolotData>,
    #[account(mut)]
    pub win_tickets: Box<Account<'info, WinLotteryTickets>>,
    #[account(mut,
        seeds = [&ticket_id.to_be_bytes(), player.key().as_ref()],
        bump,
        close = player)]
    pub ticket_associated_account: Account<'info, TicketAssociatedAccount>,
    // service charge account
    #[account(mut, address = ADMIN_PUBKEY)]
    /// CHECK: This should force the wallet to only be the ADMIN wallet
    pub admin_wallet: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"reward"],
        bump,
    )]
    // PDA
    pub token_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = ADMIN_PUBKEY,
    )]
    pub solot_token_pool_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = player,
        associated_token::mint = token_mint,
        associated_token::authority = player
    )]
    pub player_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Redeem<'info> {
    pub fn handler(ctx: Context<Redeem>, ticket_id: u32) -> Result<()> {
        let win_ticket_vec = &mut ctx.accounts.win_tickets.win_lottery_tickets;
        let solot_data = &mut ctx.accounts.solot_data;

        // 遍历中奖彩票信息，找到指定的彩票
        for win_ticket in win_ticket_vec.iter_mut() {
            if win_ticket.ticket_id == ticket_id {
                match win_ticket.reward_type {
                    RewardType::Reward1 => {
                        // 收取 12% 作为服务费
                        let service_charge = win_ticket.solot_reward.checked_mul(12).unwrap() / 100;
                        // 转入admin 账户
                        solot_transfer_sol(&solot_data.to_account_info(), &ctx.accounts.admin_wallet.to_account_info(),
                            service_charge, &ctx.accounts.system_program.to_account_info())?;
                        // 向中奖彩票的持有者转账sol
                        let final_reward = win_ticket.sol_reward - service_charge;
                        solot_transfer_sol(&solot_data.to_account_info(), &ctx.accounts.player.to_account_info(),
                            final_reward, &ctx.accounts.system_program.to_account_info())?;
                        msg!("Transfer sol to player, prize pool: {}, solot account balance: {}", solot_data.prize_pool, solot_data.get_lamports());
                    }

                    RewardType::Reward2 | RewardType::Reward3 => {
                        let seeds = b"reward";
                        let bump = ctx.bumps.token_mint;
                        let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];
                        // 收取 12% 作为服务费
                        let service_charge = win_ticket.solot_reward.checked_mul(12).unwrap() / 100;
                        // 转入 solot admin 关联代币账户
                        mint_solot_token(&ctx.accounts.token_program.to_account_info(), &ctx.accounts.token_mint,
                            &ctx.accounts.solot_token_pool_account.to_account_info(), signer, service_charge)?;
                        // mint solot 到player 的关联代币账户
                        let final_solot_reward = win_ticket.solot_reward - service_charge;
                        mint_solot_token(&ctx.accounts.token_program.to_account_info(), &ctx.accounts.token_mint,
                            &ctx.accounts.player_token_account.to_account_info(), signer, final_solot_reward)?;
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