// pub use switchboard_v2::{VrfAccountData, VrfRequestRandomness};
// use anchor_lang::solana_program::clock;


// #[derive(Clone, AnchorSerialize, AnchorDeserialize)]
// pub struct RequestResultParams {
//     pub permission_bump: u8,
//     pub switchboard_state_bump: u8,
// }

// #[derive(Accounts)]
// #[instruction(params: VrfRequestRandomnessParams)]
// pub struct RequestRandomness<'info> {
//     #[account(signer)]
//     pub authority: AccountInfo<'info>,
//     #[account(mut)]
//     pub vrf: AccountInfo<'info>,
//     #[account(mut)]
//     pub oracle_queue: AccountInfo<'info>,
//     pub queue_authority: AccountInfo<'info>,
//     pub data_buffer: AccountInfo<'info>,
//     #[account(
//         mut,
//         seeds = [
//             b"PermissionAccountData",
//             queue_authority.key().as_ref(),
//             oracle_queue.key().as_ref(),
//             vrf.key().as_ref()
//         ],
//         bump = params.permission_bump
//     )]
//     pub permission: AccountInfo<'info>,
//     #[account(mut, constraint = escrow.owner == program_state.key())]
//     pub escrow: Account<'info, TokenAccount>,
//     #[account(mut, constraint = payer_wallet.owner == payer_authority.key())]
//     pub payer_wallet: Account<'info, TokenAccount>,
//     #[account(signer)]
//     pub payer_authority: AccountInfo<'info>,
//     pub recent_blockhashes: AccountInfo<'info>,
//     #[account(seeds = [b"STATE"], bump = params.state_bump)]
//     pub program_state: AccountInfo<'info>,
//     pub token_program: AccountInfo<'info>,
// }

// impl<'info> RequestRandomness<'info> {
//     pub fn request_randomness(ctx: &Context(self), params: &VrfRequestRandomnessParams) -> Result<()> {
//         let switchboard_program = ctx.accounts.switchboard_program.to_account_info();
//         let vrf_request_randomness = VrfRequestRandomness {
//             authority: ctx.accounts.vrf_state.to_account_info(),
//             vrf: ctx.accounts.vrf.to_account_info(),
//             oracle_queue: ctx.accounts.oracle_queue.to_account_info(),
//             queue_authority: ctx.accounts.queue_authority.to_account_info(),
//             data_buffer: ctx.accounts.data_buffer.to_account_info(),
//             permission: ctx.accounts.permission.to_account_info(),
//             escrow: ctx.accounts.switchboard_escrow.clone(),
//             payer_wallet: ctx.accounts.payer_wallet.clone(),
//             payer_authority: ctx.accounts.user.to_account_info(),
//             recent_blockhashes: ctx.accounts.recent_blockhashes.to_account_info(),
//             program_state: ctx.accounts.program_state.to_account_info(),
//             token_program: ctx.accounts.token_program.to_account_info(),
//         };
//         msg!("requesting randomness...");
//         vrf_request_randomness.invoke_signed(
//             switchboard_program,
//             params.switchboard_state_bump,
//             params.permission_bump,
//             state_seeds,
//         )?;
//         Ok(())
//     }
// }


// #[derive(Accounts)]
// pub struct ConsumeRandomness<'info> {
//     // vrf client state
//     #[account]
//     pub vrf_auth: AccountLoader<'info, VrfClientState>,
//     // switchboard vrf account
//     #[account(
//         mut,
//         constraint = vrf.load()?.authority == vrf_auth.key() @ EscrowErrorCode::InvalidVrfAuthorityError
//     )]
//     pub vrf: AccountLoader<'info, VrfAccountData>
// }

// impl<'info> ConsumeRandomness<'info> {
//     pub fn consume_randomness(ctx: &Context(self), vrf_result: &mut VrfResult) -> Result<()> {
//         msg!("Consuming randomness!");

//         // load the vrf account data
//         let vrf = ctx.accounts.vrf.load()?;
//         // use the get_result method to fetch the randomness results
//         let result_buffer = vrf.get_result()?;

//         // check if result buff is all 0's
//         if result_buffer == [0u8; 32] {
//             msg!("vrf buffer empty");
//             return Ok(());
//         }

//         msg!("Result buffer is {:?}", result_buffer);
//         vrf_result.result_buffer = result_buffer;
//         vrf_result.last_timestamp = Clock::get()?.unix_timestamp;
//         Ok(())
//     }
// }

// #[repr(packed)]
// #[account(zero_copy)]
// pub struct VrfResult {
//     pub result_buffer: [u8; 32],
//     pub last_timestamp: i64,
// }

// impl VrfResult {
//     pub fn new(&self) -> Self {
//         VrfResult {
//             result_buffer: [0u8; 32],
//             last_timestamp: 0,
//         }
//     }
// }