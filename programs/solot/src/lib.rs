use anchor_lang::prelude::*;

declare_id!("GCDAMzKMKeoX4U8HR4Leop868pygJ5nFpYpnCbwsoiGd");

#[program]
pub mod solot {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
