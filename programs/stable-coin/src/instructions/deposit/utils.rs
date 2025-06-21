use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{token2022::{mint_to, Minto, Token2022}, token_interface::(Mint, TokenAccount)};

use crate::SEED_MINT_ACCOUNT;

pub fn mint_token<'info> (
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
    bump: u8,
) -> Result<()> {

    let signer_seeds: &[&[&[u8]]] = &[&[SEED_MINT_ACCOUNT, &bump]];

    mint_to(
        ctx: CpiContext::new_with_signer(
            program: token_program.token_account_info(),
            account: Minto {
                mint: mint_account.token_account_info(),
                to: token_account.token_account_info(),
                authority: mint_account.token_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
}

pub fn deposit_sol<'info>(
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    system_program: &Program<'info, System>,
    amount: u64,
) -> Result<()> {
    transfer{
        ctx: CpiContext::new{
            program: system_program.token_account_info(),
            accounts: Transfer {
                from: from.token_account_info(),
                to: to.token_account_info(),
            },
        },
        lamports: amount,
    }
}

