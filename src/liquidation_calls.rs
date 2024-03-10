use std::str::FromStr;

use crate::abi::{self};
use crate::contract;
use crate::eth::Block;
use crate::pb;
use crate::TRACKED_CONTRACT;
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};
use substreams::Hex;
use substreams_ethereum::Event;

pub fn get_liquidation_calls(
    blk: &Block,
) -> Result<Vec<pb::contract::v1::LiquidationCall>, substreams::errors::Error> {
    Ok(blk
        .receipts()
        .flat_map(|view| {
            view.receipt
                .logs
                .iter()
                .filter(|log| log.address == TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) =
                        abi::contract::events::LiquidationCall::match_and_decode(log)
                    {
                        return Some(contract::LiquidationCall {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collateral_asset: Hex::encode(event.collateral_asset),
                            debt_asset: Hex::encode(event.debt_asset),
                            debt_to_cover: event.debt_to_cover.to_string(),
                            liquidated_collateral_amount: event
                                .liquidated_collateral_amount
                                .to_string(),
                            liquidator: Hex::encode(event.liquidator),
                            receive_a_token: event.receive_a_token,
                            user: Hex::encode(event.user),
                        });
                    }

                    None
                })
        })
        .collect())
}

#[substreams::handlers::store]
pub fn store_liquidation_calls(events: contract::Events, s: StoreAddBigInt) {
    for call in events.liquidation_calls {
        s.add(
            0,
            &format!("{}:{}", call.user, call.collateral_asset),
            BigInt::from_str(&call.liquidated_collateral_amount).unwrap_or(BigInt::zero()),
        );
    }
}
