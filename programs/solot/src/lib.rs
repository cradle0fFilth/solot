use anchor_lang::prelude::*;

declare_id!("GCDAMzKMKeoX4U8HR4Leop868pygJ5nFpYpnCbwsoiGd");

#[program]
pub mod solot {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let solot_program = &mut ctx.accounts.solot_program;       
        solot_program.solot_total_count = 0;
        solot_program.total_ticket = 0;
        solot_program.unredeemed_ticket = 0;          
        solot_program.prize_pool = 0;
        solot_program.total_players = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + 32)]
    pub solot_program: Account<'info, SolotProgram>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct SolotProgram {
    pub solot_total_count: u64,
    pub total_ticket: u32,
    pub unredeemed_ticket: u32,
    pub prize_pool: u64,
    pub total_players: u32,
}

