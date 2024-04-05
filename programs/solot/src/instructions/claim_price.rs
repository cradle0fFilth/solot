use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;

#[derive(Accounts)]
pub struct ClaimPrice<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub win_tickets: Account<'info, WinLotteryTickets>,
}

impl<'info> ClaimPrice<'info> {
    pub fn handler(ctx: Context<ClaimPrice>, ticket_id: u32, amount: u64) -> Result<()> {
        let win_ticket_vec = &mut ctx.accounts.win_tickets.win_lottery_tickets;

        // 遍历中奖彩票信息，找到指定的彩票
        for win_ticket in win_ticket_vec.iter_mut() {
            if win_ticket.ticket_id == ticket_id {
                // 检查ticket 所有者是否和本指令发起者一致
                require_keys_eq!(ctx.accounts.player.key(), win_ticket.authority, SolotError::Unauthorized);
            }
        }
        Ok(())
    }
}