use anchor_lang::prelude::*;
use crate::state::*;


#[derive(Accounts)]
pub struct MintTicket<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub solot_data: Account<'info, SolotData>,            // 是否需要额外的约束？
    #[account(init, payer = player,  space = 8 + 32)]
    pub player_ticket: Account<'info, PlayerTicket>,
    #[account(mut)]
    pub loss_tickets: Account<'info, LossLotteryTickets>,
    #[account(mut)]
    pub win_tickets: Account<'info, WinLotteryTickets>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintTicket<'info> {
    pub fn mint_ticket(ctx: Context<MintTicket>) -> Result<()> {
        // 处理ticket pirce， 0.1 SOL

        // 创建一个Ticket
        let solot_data = &mut ctx.accounts.solot_data;
        let player_ticket = &mut ctx.accounts.player_ticket;

        player_ticket.authority = ctx.accounts.player.key();
        player_ticket.player_ticket = Ticket::new(solot_data.total_ticket, [1,2,3], b'a');
        // 用 switchboard 程序来随机生成 字母& 数字
        // RequestRandomness::request_randomness();
        // generate_ticket_field(&mut mint_ticket.ticket);
        msg!("ticket create successfully");
        ctx.accounts.loss_tickets.add_ticket(player_ticket.player_ticket.clone());

        // 开奖
        draw_lottery(&mut ctx.accounts.loss_tickets,
            &mut ctx.accounts.win_tickets, &mut solot_data.prize_pool)?;
        solot_data.total_ticket += 1;
        // todo：操作price_pool
        Ok(())
    }
}


fn draw_lottery(loss_tickets: &mut Account<LossLotteryTickets>, win_tickets: &mut Account<WinLotteryTickets>, prize_pool: &mut u64) -> Result<()> {
    // todo： switchboard 随机ticket用于开奖号码
    let draw_ticket = Ticket::new(0, [1,2,3], b'a');
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