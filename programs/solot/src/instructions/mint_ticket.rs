use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use solana_program::native_token::LAMPORTS_PER_SOL;
use crate::state::*;
use crate::constants::*;
use crate::error::*;
use crate::utils::*;
#[derive(Accounts)]
pub struct MintTicket<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub solot_data: Account<'info, SolotData>,
    #[account(init,
    seeds = [(solot_data.total_ticket + 1).to_be_bytes().as_ref(), player.key().as_ref()],
    bump,
    payer = player,
    space = std::mem::size_of::<TicketAssociatedAccount>() + 8)]
    pub ticket_associated_account: Account<'info, TicketAssociatedAccount>,
    #[account(mut)]
    pub loss_tickets: Account<'info, LossLotteryTickets>,
    #[account(mut)]
    pub win_tickets: Account<'info, WinLotteryTickets>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintTicket<'info> {
    pub fn handler(ctx: Context<MintTicket>, ticket_type: u8) -> Result<()> {
        let solot_data = &mut ctx.accounts.solot_data;
        let ticket_associated_account = &mut ctx.accounts.ticket_associated_account;
        ticket_associated_account.ticket_id = solot_data.total_ticket + 1;
        // 处理ticket pirce
        let ticket_price = match ticket_type {
            TICKET_TYPE_GATEWAY => {
                msg!("gateway ticket");
                (1u64).checked_mul(LAMPORTS_PER_SOL/10)
            },
            TICKET_TYPE_TRIUMPH => {
                msg!("triumph ticket");
                (6u64).checked_mul(LAMPORTS_PER_SOL/10)
            },
            TICKET_TYPE_VOYAGE => {
                msg!("voyage ticket");
                (18u64).checked_mul(LAMPORTS_PER_SOL/10)
            }
            _ => {
                msg!("invalid ticket type");
                return err!(SolotError::InvalidArgument);
            }
        };
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.player.to_account_info(),
                    to: ticket_associated_account.to_account_info(),
                }),
                ticket_price.unwrap(),
        )?;
        // 按比例将sol 转入奖池中， ticket_associated_account账户中剩下的sol暂存，用于swap
        proportional_transfer(ticket_associated_account, solot_data)?;
        // todo： swap

        // 创建一个Ticket
        let player_ticket = Ticket::new(ticket_associated_account.ticket_id, ticket_type,[1,2,3], b'a');
        // 用 switchboard 程序来随机生成 字母& 数字
        // RequestRandomness::request_randomness();
        // generate_ticket_field(&mut mint_ticket.ticket);
        msg!("ticket create successfully");
        ctx.accounts.loss_tickets.add_ticket(player_ticket.clone());

        // 开奖
        draw_lottery(&mut ctx.accounts.loss_tickets,
            &mut ctx.accounts.win_tickets, &mut solot_data.prize_pool)?;
        solot_data.total_ticket += 1;
        Ok(())
    }
}


fn draw_lottery(loss_tickets: &mut Account<LossLotteryTickets>, win_tickets: &mut Account<WinLotteryTickets>, prize_pool: &mut u64) -> Result<()> {
    // todo： switchboard 随机ticket用于开奖号码
    let draw_ticket = Ticket::new(0, 0, [1,2,3], b'a');
    // 比对结果, 根据中奖ticket的id 和中奖类型，重新构建win_ticket放入win_tickets数组里， 同时将原来的tiket从loss_tickets里删除
    loss_tickets.loss_lottery_tickets.retain(|element|{
        match element.compare_ticket(&draw_ticket) {
            0 => {
                msg!("loss lottery");
                true
            }
            1 => {
                msg!("win lottery, match 1 number");
                win_tickets.add_ticket(WinTicket::new(element.ticket_id, RewardType::Reward3, 500, 0));
                false
            }
            2 => {
                msg!("win lottery, match 2 number");
                win_tickets.add_ticket(WinTicket::new(element.ticket_id, RewardType::Reward2, 1500, 0));
                false
            }
            3 => {
                msg!("win lottery, match 3 number");
                win_tickets.add_ticket(WinTicket::new(element.ticket_id, RewardType::Reward1, 0, (*prize_pool)/5));
                false
            }
            _ => {
                msg!("ivalid value!");
                false
            }
        }
    });
    Ok(())
}

// fn generate_ticket_filed (ticket: &mut Ticket) {
//     let vrf_result = VrfResult::new();
//     ConsumeRandomness::consume_randomness(&mut vrf_result);
//     let random_number1 = vrf_result[0]%100 as u8;
//     let random_number2 = vrf_result[1]%100 as u8;
//     let random_number3 = vrf_result[2]%100 as u8;
//     let random_letter: char = (b'a' + vrf_result[3]%26 as u8) as char;
//     ticket.numbers = [random_number1, random_number2, random_number3];
//     ticket.letter = random_letter;
// }