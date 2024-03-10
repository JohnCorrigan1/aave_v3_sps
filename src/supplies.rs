use crate::abi::{self};
use crate::contract;
use crate::eth::Block;
use crate::pb;
use crate::TRACKED_CONTRACT;
use std::str::FromStr;
use substreams::{
    store::{StoreAdd, StoreAddBigInt, StoreNew},
    Hex,
};
use substreams_ethereum::Event;

pub fn get_supplies(
    blk: &Block,
) -> Result<Vec<pb::contract::v1::Supply>, substreams::errors::Error> {
    Ok(blk
        .receipts()
        .flat_map(|view| {
            view.receipt
                .logs
                .iter()
                .filter(|log| log.address == TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::contract::events::Supply::match_and_decode(log) {
                        return Some(contract::Supply {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            on_behalf_of: Hex::encode(event.on_behalf_of),
                            referral_code: event.referral_code.to_u64(),
                            reserve: Hex::encode(event.reserve),
                            user: Hex::encode(event.user),
                        });
                    }

                    None
                })
        })
        .collect())
}

#[substreams::handlers::store]
pub fn store_supplies(events: contract::Events, s: substreams::store::StoreAddBigInt) {
    for supply in events.supplies {
        s.add(
            0,
            supply.reserve,
            substreams::scalar::BigInt::from_str(&supply.amount)
                .unwrap_or(substreams::scalar::BigInt::zero()),
        );
    }

    for withdrawl in events.withdraws {
        s.add(
            0,
            withdrawl.reserve,
            substreams::scalar::BigInt::from_str(&withdrawl.amount)
                .unwrap_or(substreams::scalar::BigInt::zero())
                * -1,
        );
    }

    for liquidation in events.liquidation_calls {
        s.add(
            0,
            &format!("{}:{}", liquidation.user, liquidation.collateral_asset),
            substreams::scalar::BigInt::from_str(&liquidation.liquidated_collateral_amount)
                .unwrap_or(substreams::scalar::BigInt::zero())
                * -1,
        );
    }
}
