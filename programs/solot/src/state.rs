use anchor_lang::prelude::*;

#[account]
pub struct SolotData {
    pub solot_total_count: u64,
    pub total_ticket: u32,
    pub unredeemed_ticket: u32,
    pub prize_pool: u64,
    pub total_players: u32,
}

#[account]
pub struct UnredeemedTickets {
    pub unredeemed_tickets: Vec<Ticket>,
}

impl UnredeemedTickets {}

#[account]
pub struct RewardTickets {
    pub reward_tickets: Vec<Ticket>,
}

impl RewardTickets {}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Ticket {
    pub ticket_number: u32,
    pub numbers: [u8; 3],
    pub letter: u8,
}
