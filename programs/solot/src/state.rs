use anchor_lang::prelude::*;

#[account]
pub struct SolotData {
    pub total_ticket: u32,
    pub prize_pool: u64,
}

#[account]
pub struct LossLotteryTickets {
    pub loss_lottery_tickets: Vec<Ticket>,
}

impl LossLotteryTickets {
    // 增删查 todo
    pub const LEN: usize = 400 * (4 + 4 + 3 + 1);
    pub fn add_ticket(&mut self, ticket: Ticket){
        self.loss_lottery_tickets.push(ticket);
    }
}


// #[account]
// pub struct PlayerTicket{
//     pub authority: Pubkey,
//     pub player_ticket: Ticket,
// }

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Ticket {
    pub authority: Pubkey,
    pub ticket_id: u32, // 4
    pub numbers: [u8; 3], // 3
    pub letter: u8, // 1
}

impl Ticket {
    pub fn new(authority: Pubkey, ticket_id: u32, numbers: [u8; 3], letter: u8) -> Self {
        Ticket {
            authority,
            ticket_id,
            numbers,
            letter,
        }
    }
    pub fn compare_ticket(&self, draw_ticket: &Ticket) -> u32 {
        let mut equal_counts  = 0;
        if self.letter != draw_ticket.letter {
            return equal_counts;
        }
        for i in 0..3 {
            if self.numbers[i] == draw_ticket.numbers[i] {
                equal_counts += 1;
            }
        }
        equal_counts
    }
}

#[account]
pub struct WinLotteryTickets {
    pub win_lottery_tickets: Vec<WinTicket>,
}

impl WinLotteryTickets {
    pub const LEN: usize = 400 * (4 + 4 + 2 + 8 + 8);
    pub fn add_ticket(&mut self, ticket: WinTicket){
        self.win_lottery_tickets.push(ticket);
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct WinTicket {
    pub authority: Pubkey,
    pub ticket_id: u32, // 4
    pub reward_type: RewardType, // 1 + 1
    // keep lamports
    pub solot_reward: u64, // 8
    pub sol_reward: u64, // 8
}

impl WinTicket {
    pub fn new(authority: Pubkey, ticket_id: u32, reward_type: RewardType, solot_reward: u64, sol_reward: u64) -> Self {
        WinTicket {
            authority,
            ticket_id,
            reward_type,
            solot_reward,
            sol_reward,
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum RewardType {
    None,
    Reward1,
    Reward2,
    Reward3,
}