use anchor_lang::prelude::*;
use crate::(Config, SEED_CONFIG_ACCOUNT);

#[derive(Accounts)]
pub struct updateConfig<'info> {
    #[account(
        mut,
        seeds = SEED_CONFIG_ACCOUNT,
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,
}

pub fn process_update_config(ctx: Context<updateConfig>, mint_health_factor: u64) -> Result<()> {
    let config_account: Config = &mut ctx.accounts.config_account;

    config_account.mint_health_factor = mint_health_factor;

    Ok(())
}


