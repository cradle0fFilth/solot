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

const ADMIN_PUBKEY: Pubkey = pubkey!("");
const INITIAL_SOLOT_NUM:u8 = 1000;

#[program]:
pub mod solot {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
