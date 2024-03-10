mod abi;
pub mod get_back_unbackeds;
pub mod get_borrows;
pub mod get_flash_loans;
pub mod graph_out;
pub mod isolation_mode_total_debt_updateds;
pub mod liquidation_calls;
pub mod mint_unbackeds;
pub mod minted_to_treasuries;
pub mod pb;
pub mod rebalance_stable_borrow_rates;
pub mod repays;
pub mod reserve_data_updateds;
pub mod reserve_used_as_collateral_enableds;
pub mod reserve_used_as_collerteral_disableds;
pub mod supplies;
pub mod swap_borrow_rate_modes;
pub mod upgrades;
pub mod user_e_mode_sets;
pub mod withdraws;
use hex_literal::hex;
use pb::{contract::v1 as contract, pool_config::v1 as poolConfig};
use substreams::{
    store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto},
    Hex,
};

use substreams_ethereum::pb::eth::v2 as eth;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

const TRACKED_CONTRACT: [u8; 20] = hex!("87870bca3f3fd6335c3f4ce8392d69350b4fa4e2");
const POOL_CONFIG_CONTRACT: [u8; 20] = hex!("64b761D848206f447Fe2dd461b0c635Ec39EbB27");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_reserve_initializations(
    blk: eth::Block,
) -> Result<poolConfig::ReserveInitializations, substreams::errors::Error> {
    let reserve_initializations: Vec<_> = blk
        .events::<abi::pool_config::events::ReserveInitialized>(&[&POOL_CONFIG_CONTRACT])
        .map(|(event, _log)| poolConfig::ReserveInitialization {
            asset: Hex::encode(event.asset),
            a_token: Hex::encode(event.a_token),
            stable_debt_token: Hex::encode(event.stable_debt_token),
            variable_debt_token: Hex::encode(event.variable_debt_token),
            interest_rate_strategy_address: Hex::encode(event.interest_rate_strategy_address),
        })
        .collect();

    Ok(poolConfig::ReserveInitializations {
        reserve_initializations,
    })
}

#[substreams::handlers::store]
fn store_tokens(
    reserve_initializations: poolConfig::ReserveInitializations,
    s: StoreSetProto<poolConfig::Token>,
) {
    for reserve in reserve_initializations.reserve_initializations {
        let token = get_calls(reserve.asset.clone());
        let a_token = get_calls(reserve.a_token.clone());
        s.set(
            0,
            &reserve.asset,
            &poolConfig::Token {
                asset: reserve.asset.to_string(),
                name: token.name,
                symbol: token.symbol,
                decimals: token.decimals,
                a_token: reserve.a_token.to_string(),
                a_name: a_token.name,
                a_symbol: a_token.symbol,
                a_decimals: a_token.decimals,
            },
        );
    }
}

#[substreams::handlers::map]
fn map_tokens(
    reserve_initializations: poolConfig::ReserveInitializations,
    token_store: StoreGetProto<poolConfig::Token>,
) -> Result<poolConfig::Tokens, substreams::errors::Error> {
    let tokens: Vec<_> = reserve_initializations
        .reserve_initializations
        .iter()
        .map(|reserve| token_store.get_at(0, &reserve.asset).unwrap())
        .collect();

    Ok(poolConfig::Tokens { tokens })
}
struct Token {
    name: String,
    symbol: String,
    decimals: u64,
}

fn get_calls(address: String) -> Token {
    let batch = substreams_ethereum::rpc::RpcBatch::new();
    let response = batch
        .add(
            abi::erc20::functions::Decimals {},
            Hex::decode(&address).unwrap(),
        )
        .add(
            abi::erc20::functions::Name {},
            Hex::decode(&address).unwrap(),
        )
        .add(
            abi::erc20::functions::Symbol {},
            Hex::decode(&address).unwrap(),
        )
        .execute()
        .unwrap()
        .responses;

    Token {
        decimals: substreams_ethereum::rpc::RpcBatch::decode::<_, abi::erc20::functions::Decimals>(
            &response[0],
        )
        .unwrap()
        .to_u64(),
        name: substreams_ethereum::rpc::RpcBatch::decode::<_, abi::erc20::functions::Name>(
            &response[1],
        )
        .unwrap()
        .to_string(),
        symbol: substreams_ethereum::rpc::RpcBatch::decode::<_, abi::erc20::functions::Symbol>(
            &response[2],
        )
        .unwrap()
        .to_string(),
    }
}

// fn get_decimals(address: String) -> substreams::scalar::BigInt {
//     let decimals = eth_call(address);
// }
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
