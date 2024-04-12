use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;


// Proportional transfer
pub fn proportional_transfer<'info>(ticket_associated_account: &mut Account<'info, TicketAssociatedAccount>,
    solot_data: &mut Account<'info, SolotData>) -> Result<()>{
    let mut amount = ticket_associated_account.get_lamports();
    if solot_data.prize_pool < PRIZE_POLL_THRESHOLD_ONE {
        amount = amount/2;         // 50%
    } else if (solot_data.prize_pool >= PRIZE_POLL_THRESHOLD_ONE) && (solot_data.prize_pool < PRIZE_POLL_THRESHOLD_TWO) {
        amount = (amount/5)*2;     // 40%
    } else {
        amount = (amount/10)*3;    // 30%
    }
    ticket_associated_account.sub_lamports(amount)?;
    solot_data.add_lamports(amount)?;
    solot_data.prize_pool = solot_data.prize_pool.checked_add(amount).unwrap();
    Ok(())
}