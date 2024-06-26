use solana_program::{pubkey, pubkey::Pubkey};
use solana_program::native_token::LAMPORTS_PER_SOL;

pub const ADMIN_PUBKEY: Pubkey = pubkey!("6T9ajVYoL13jeNp9FCMoU9s4AEBaNFJpHvXptUz1MGag");

pub const INIT_SOLOT: u64 = 10000;

pub const PRIZE_POLL_THRESHOLD_ONE: u64 = 500*LAMPORTS_PER_SOL;
pub const PRIZE_POLL_THRESHOLD_TWO: u64 = 1000*LAMPORTS_PER_SOL;
pub const PRIZE_POLL_THRESHOLD_THREE: u64 = 1500*LAMPORTS_PER_SOL;


// ticket types
pub const TICKET_TYPE_STANDARD: u8 = 1 << 0;         // Fully random winning, price 0.1 sol
pub const TICKET_TYPE_SPECIAL: u8 = 1 << 1;          // Only wins 2+1 or 3+1, price 0.6 sol