use anchor_lang::prelude::*;

declare_id!("7vRG9KT98AqMV2cKcgkQ1WTjMqADBNgXJkkNWSjXtJbe");

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
    pub fn mint_ticket(ctx: Context<MintTicket>, ticket_type: u8) -> Result<()> {
        mint_ticket::MintTicket::handler(ctx, ticket_type)
    }
    pub fn redeem(ctx: Context<Redeem>, ticket_id: u32) -> Result<()> {
        redeem::Redeem::handler(ctx, ticket_id)
    }
}
