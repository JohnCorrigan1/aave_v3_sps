mod abi;
mod get_back_unbackeds;
mod get_borrows;
mod get_flash_loans;
mod isolation_mode_total_debt_updateds;
mod liquidation_calls;
mod mint_unbackeds;
mod minted_to_treasuries;
mod pb;
mod rebalance_stable_borrow_rates;
mod repays;
mod reserve_data_updateds;
mod reserve_used_as_collateral_enableds;
mod reserve_used_as_collerteral_disableds;
mod supplies;
mod swap_borrow_rate_modes;
mod upgrades;
mod user_e_mode_sets;
mod withdraws;
use hex_literal::hex;
use pb::{contract::v1 as contract, sf::substreams::v1::module::input::Store};
use substreams::store::{StoreAdd, StoreAddBigInt};
use substreams_ethereum::pb::eth::v2 as eth;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

const TRACKED_CONTRACT: [u8; 20] = hex!("87870bca3f3fd6335c3f4ce8392d69350b4fa4e2");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    Ok(contract::Events {
        back_unbackeds: get_back_unbackeds::get_back_unbackeds(&blk)?,
        borrows: get_borrows::get_borrows(&blk)?,
        flash_loans: get_flash_loans::get_flash_loans(&blk)?,
        isolation_mode_total_debt_updateds:
            isolation_mode_total_debt_updateds::isolation_mode_total_debt_updateds(&blk)?,
        liquidation_calls: liquidation_calls::get_liquidation_calls(&blk)?,
        mint_unbackeds: mint_unbackeds::get_mint_unbackeds(&blk)?,
        minted_to_treasuries: minted_to_treasuries::get_minted_to_treasuries(&blk)?,
        rebalance_stable_borrow_rates:
            rebalance_stable_borrow_rates::get_rebalanced_stable_borrow_rates(&blk)?,
        repays: repays::get_repays(&blk)?,
        reserve_data_updateds: reserve_data_updateds::get_reserve_data_updateds(&blk)?,
        reserve_used_as_collateral_disableds:
            reserve_used_as_collerteral_disableds::get_reserve_used_as_collateral_disableds(&blk)?,
        reserve_used_as_collateral_enableds:
            reserve_used_as_collateral_enableds::get_reserve_used_as_collateral_enableds(&blk)?,
        supplies: supplies::get_supplies(&blk)?,
        swap_borrow_rate_modes: swap_borrow_rate_modes::get_swap_borrow_rate_modes(&blk)?,
        upgradeds: upgrades::get_upgrades(&blk)?,
        user_e_mode_sets: user_e_mode_sets::get_user_e_mode_sets(&blk)?,
        withdraws: withdraws::get_withdraws(&blk)?,
    })
}

fn token_store(
    events: contract::Events,
    s: StoreAddBigInt,
) -> Result<(), substreams::errors::Error> {
    Ok(())
}

fn store_events(
    events: contract::Events,
    s: StoreAddBigInt,
) -> Result<(), substreams::errors::Error> {
    Ok(())
}
