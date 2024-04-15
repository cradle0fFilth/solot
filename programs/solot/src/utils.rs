use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::state::*;
use crate::constants::*;
use anchor_spl::token::{mint_to, MintTo, Mint};

// transfer sol between solot_data account and player account
pub fn solot_transfer_sol<'info>(from: &AccountInfo<'info>, to: &AccountInfo<'info>, amount: u64, system_program: &AccountInfo<'info>) -> Result<()> {
    transfer(
        CpiContext::new(
            system_program.clone(),
            Transfer {
                from:from.clone(),
                to: to.clone(),
            }),
            amount,
    )?;
    Ok(())
}

// Proportional transfer
pub fn pda_proportional_transfer<'info>(ticket_associated_account: &mut Account<'info, TicketAssociatedAccount>,
    solot_data: &mut Account<'info, SolotData>) -> Result<()>{
    let mut amount = ticket_associated_account.get_lamports();
    if solot_data.prize_pool <= PRIZE_POLL_THRESHOLD_ONE {
        amount = (amount/5)*3;         // 60%
    } else if (solot_data.prize_pool > PRIZE_POLL_THRESHOLD_ONE) && (solot_data.prize_pool <= PRIZE_POLL_THRESHOLD_TWO) {
        amount = amount/2;         // 50%
    } else if (solot_data.prize_pool > PRIZE_POLL_THRESHOLD_TWO) && (solot_data.prize_pool <= PRIZE_POLL_THRESHOLD_THREE) {
        amount = (amount/5)*2;     // 40%
    } else {
        amount = (amount/10)*3;    // 30%
    }
    ticket_associated_account.sub_lamports(amount)?;
    solot_data.add_lamports(amount)?;
    solot_data.prize_pool = solot_data.prize_pool.checked_add(amount).unwrap();
    Ok(())
}


// mint solot token to specified account
pub fn mint_solot_token<'info>(token_program: &AccountInfo<'info>, mint: &Account<'info, Mint>, to: &AccountInfo<'info>, signer: &[&[&[u8]]], amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new_with_signer(
        token_program.clone(),
        MintTo {
            mint: mint.to_account_info().clone(),
            to: to.clone(),
            authority: mint.to_account_info().clone(),
        },
        signer, // pda signer
    );
    let amount = (amount)
        .checked_mul(10u64.pow(mint.decimals as u32))
        .unwrap();
    mint_to(cpi_ctx, amount)?;
    Ok(())
}