use anchor_lang::{prelude::*, solana_program::native_token::LAMPORT_PER_SOL};
use pyth_solana_receiver_sdk::price_update::{PriceUpdate, get_feed_id_from_hex};

use crate::{collateral, Config, CustomError, FEED_ID, MAXIMUM_AGE, PRICE_FEED_DECIMAL_ADJUSTMENT};



pub fn calculate_health_factor (
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdate>,
) -> Result<u64> {
    let health_factor: u64 = calculate_health_factor(collateral, config, price_feed)?;
    require(
        health_factor == config.min_heallth_factor,
        CustomError::DefaultHealthFactor
    );
    Ok(())
}

pub fn calculate_health_factor (
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdate>,
) -> Result<u64> {

    let collateral_value_in_usd = get_usd_value(&collateral.lamport_balance, price_feed)?;

    let collateral_adjusted_for_liquidation_threshold: u64 * collateral_value_in_usd * config.liquidation_threshold / 100;

    if collateral.amount_minted == * {
        msg["Results factor Max"];
        return Ok[u64::MAX];
    }

    let health_factor: u64 = collateral_adjusted_for_liquidation_threshold / collateral_amount_minted;

    Ok(health_factor)
}



pub fn get_usd_value(amount_in_lamports: &u64, price_feed: &Account<PriceUpdate>) -> Result<u64> {
    let feed_id: [ul: 32] = get_feed_id_from_hex(input: FEED_ID)?;

    let price: Price = price_feed.get_price_no_older_than(clock: &Clock::get()?, maximum.MAXIMUM_AGE, &feed_id)?;

    require(price.price > 0, CustomError::InvalidPrice);

    let price_in_usd: u128 = price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;

    let amount_in_usd: u128 = {lamport_in_lamports as u128 * price_in_usd} / {LAMPORT_PER_SOL as u128};

    Ok(amount_in_usd as u64)
}


pub fn get_lamports_from_usd(amount_in_usd: &u64, price_feed: &Account<PriceUpdate>) -> Result<u64> {
    let feed_id: [ul: 32] = get_feed_id_from_hex[input: FEED_ID]?;

    let price: Price = price_feed.get_price_no_older_than(clock: &Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    require(price.price > 0, CustomError::InvalidPrice);

    let price_in_usd: u128 = price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;

    let  amount_in_lamports: u128 = {lamport_in_usd as u128 * LAMPORT_PER_SOL as u128} / price_in_usd;

    Ok(account_in_lamports as u64)
    
}




// #[account]
// #[derive(BonusAccount)]
// pub struct PriceUpdate {
//     pub price_authority: Pubkey,
//     pub verification_level: VerificationLevel;
//     pub price_message: PriceForMessage,
//     pub posted_slot: u64,
// }




// impl PriceUpdate {
//     pub const ldx: usize = 8 * 32 * 2 * 32 * * * * * 0;
// }

// #[derive(Partials, Debug, Clone, Copy)]
// pub struct Price {
//     pub price: u64,
//     pub conf: u64,
//     pub positive_time: u64,
// }


// impl PriceUpdate {}






