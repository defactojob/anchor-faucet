#![feature(proc_macro_hygiene)]

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

#[program]
pub mod parrot {
    use anchor_spl::token::MintTo;

    use super::*;
    pub fn initialize(ctx: Context<Initialize>, nonce: u8) -> ProgramResult {
        let faucet = &mut ctx.accounts.faucet;
        // faucet.token_program = ctx.accounts.token_program.into();

        faucet.mint = ctx.accounts.mint.key.clone();
        faucet.nonce = nonce;

        Ok(())
    }

    pub fn drip(ctx: Context<Drip>) -> ProgramResult {
        let seeds = &[
            ctx.accounts.faucet.to_account_info().key.as_ref(),
            &[ctx.accounts.faucet.nonce],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.receiver.to_account_info(),
            authority: ctx.accounts.mint_auth.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.clone();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer);
        token::mint_to(cpi_ctx, 100)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    faucet: ProgramAccount<'info, Faucet>,
    mint: AccountInfo<'info>,

    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Drip<'info> {
    #[account()]
    faucet: ProgramAccount<'info, Faucet>,

    #[account(mut)]
    mint: CpiAccount<'info, Mint>,

    // what's the point with this annotation?
    #[account(seeds = [faucet.to_account_info().key.as_ref(), &[faucet.nonce]])]
    // #[account()]
    mint_auth: AccountInfo<'info>,

    #[account(mut)]
    receiver: CpiAccount<'info, TokenAccount>,

    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
}

#[account]
pub struct Faucet {
    pub mint: Pubkey,
    // signer nonce
    pub nonce: u8,
}