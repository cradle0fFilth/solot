use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::{accounts::Metadata as MetadataAccount, types::DataV2},
        CreateMetadataAccountsV3, Metadata,
    },
    token::{burn, mint_to, Burn, Mint, MintTo, Token, TokenAccount},
};
use solana_program::{pubkey, pubkey::Pubkey};

declare_id!("GCDAMzKMKeoX4U8HR4Leop868pygJ5nFpYpnCbwsoiGd");

const ADMIN_PUBKEY: Pubkey = pubkey!("D2pRGmGrgkUKWvv5KXUcGo7R7bGtg6m6yqSYr2x29fZA");

const INIT_SOLOT: u64 = 10000;

#[program]
pub mod solot {
    use super::*;

    pub fn initialize_lottery(ctx: Context<InitiaLizeLottery>) -> Result<()> {
        let solot_data = &mut ctx.accounts.solot_data;
        solot_data.solot_total_count = 0;
        solot_data.total_ticket = 0;
        solot_data.unredeemed_ticket = 0;
        solot_data.prize_pool = 0;
        solot_data.total_players = 0;

        // mint 10000 solot to the pool
        let seeds = b"reward";
            let bump = ctx.bumps.token_mint;
            let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];
        // CPI Context
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.solot_token_pool_account.to_account_info(),
                authority: ctx.accounts.token_mint.to_account_info(),
            },
            signer, // pda signer
        );
        let amount = (INIT_SOLOT)
        .checked_mul(10u64.pow(ctx.accounts.token_mint.decimals as u32))
        .unwrap();
        mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn create_mint(ctx: Context<CreateMint>, uri: String, name: String, symbol: String) -> Result<()> {
        let seeds = b"reward";
        let bump = ctx.bumps.token_mint;
        let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];
        // CPI Context
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(), // the metadata account being created
                mint: ctx.accounts.token_mint.to_account_info(), // the mint account of the metadata account
                mint_authority: ctx.accounts.token_mint.to_account_info(), // the mint authority of the mint account
                update_authority: ctx.accounts.token_mint.to_account_info(), // the update authority of the metadata account
                payer: ctx.accounts.admin.to_account_info(), // the payer for creating the metadata account
                system_program: ctx.accounts.system_program.to_account_info(), // the system program account, required when creating new accounts
                rent: ctx.accounts.rent.to_account_info(), // the rent sysvar account
            },
            signer, // pda signer
        );
        let data_v2 = DataV2 {
            name: name,
            symbol: symbol,
            uri: uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(
            cpi_ctx, // cpi context
            data_v2, // token metadata
            true,    // is_mutable
            true,    // update_authority_is_signer
            None,    // collection details
        )?;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitiaLizeLottery<'info> {
    #[account(mut, address = ADMIN_PUBKEY)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + 32)]
    pub solot_data: Account<'info, SolotData>,

    #[account(
        mut,
        seeds = [b"reward"],
        bump,
    )]
    pub token_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user
    )]
    pub solot_token_pool_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut, address = ADMIN_PUBKEY)]
    pub admin: Signer<'info>,
    #[account(
        init,
        seeds = [b"reward"],
        bump,
        payer = admin,
        mint::decimals = 9,
        mint::authority = token_mint,

    )]
    pub token_mint: Account<'info, Mint>,

    ///CHECK: Using "address" constraint to validate metadata account address, this account is created via CPI in the instruction
    #[account(
        mut,
        address = MetadataAccount::find_pda(&token_mint.key()).0,
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct SolotData {
    pub solot_total_count: u64,
    pub total_ticket: u32,
    pub unredeemed_ticket: u32,
    pub prize_pool: u64,
    pub total_players: u32,
}

