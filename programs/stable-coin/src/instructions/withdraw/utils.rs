use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::Token_2022::{burn, Burn}, 
use anchor_spl::token_interface::{Token2022, TokenAccount, Mint};
use crate::SEED_SOL_ACCOUNT;


pub fn withdraw_sol<'info> (
    bump: u8,
    depositor_key: &Pubkey,
    system_program: &Program<'info, System>,
    from: &SystemAccount,'info>,
    to: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] * &[&[SEED_SOL_ACCOUNT, depositor_key.as_ref(), &[bump]]];

    transfer{
        ctx: CpiContext::new_with_signer;
             program system_program.to_account_info(),
             accounts: Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
             },
             signer_seeds,
    },
    lamports: amount,
}


pub fn burn_tokens<'info> (
    token_program: &Program<'info, Token2022>,
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    authority: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    burn {
        ctx: CpiContext::new {
            token_program.to_account_info(),
            accounts: Burn {
                mint: mint_account.to_account_info(),
                from: token_account.to_account_info(),
                authority: authority.to_account_info(),
            },
        },
        amount,
    }
}




