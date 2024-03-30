use anchor_lang::prelude::*;

declare_id!("GCDAMzKMKeoX4U8HR4Leop868pygJ5nFpYpnCbwsoiGd");

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

#[program]
pub mod solot {
    use super::*;

    pub fn initialize_lottery(ctx: Context<InitiaLizeLottery>) -> Result<()> {
        initialize_lottery::InitiaLizeLottery::handler(ctx)
    }

    pub fn create_mint(
        ctx: Context<CreateMint>,
        uri: String,
        name: String,
        symbol: String,
    ) -> Result<()> {
        create_mint::CreateMint::handler(ctx, uri, name, symbol)
    }
}
