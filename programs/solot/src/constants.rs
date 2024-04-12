use solana_program::{pubkey, pubkey::Pubkey};
use solana_program::native_token::LAMPORTS_PER_SOL;

pub const ADMIN_PUBKEY: Pubkey = pubkey!("9X5mTq4reQS1Yexs5TeF4novs8a2LQRUyRWvCkH123wa");

pub const INIT_SOLOT: u64 = 10000;

pub const PRIZE_POLL_THRESHOLD_ONE: u64 = 1500*LAMPORTS_PER_SOL;
pub const PRIZE_POLL_THRESHOLD_TWO: u64 = 3000*LAMPORTS_PER_SOL;


// ticket types
pub const TICKET_TYPE_GATEWAY: u8 = 1 << 0;         // Fully random winning, price 0.1 sol
pub const TICKET_TYPE_VOYAGE: u8 = 1 << 1;          // Only wins 2+1 or 3+1, price 0.6 sol
pub const TICKET_TYPE_TRIUMPH: u8 = 1 << 2;         // Only wins 3+1, price 1.8 sol