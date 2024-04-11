use anchor_lang::prelude::*;

declare_id!("9B8Up2vzn7yrcmkm2dGG8dYX6UBzFe6tue4gbuvgbf3u");

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

#[program]
pub mod solot {
    use super::*;

    pub fn create_mint(
        ctx: Context<CreateMint>,
        uri: String,
        name: String,
        symbol: String,
    ) -> Result<()> {
        create_mint::CreateMint::handler(ctx, uri, name, symbol)
    }

    pub fn initialize_lottery(ctx: Context<InitiaLizeLottery>) -> Result<()> {
        initialize_lottery::InitiaLizeLottery::handler(ctx)
    }
    pub fn mint_ticket(ctx: Context<MintTicket>) -> Result<()> {
        mint_ticket::MintTicket::handler(ctx)
    }
    pub fn claim_price(ctx: Context<Redeem>, ticket_id: u32, amount: u64) -> Result<()> {
        redeem::Redeem::handler(ctx, ticket_id, amount)
    }
}
