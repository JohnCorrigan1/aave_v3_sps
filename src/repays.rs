use crate::abi::{self};
use crate::contract;
use crate::eth::Block;
use crate::pb;
use crate::TRACKED_CONTRACT;
// use std::str::FromStr;
use substreams::Hex;
use substreams_ethereum::Event;

pub fn get_repays(blk: &Block) -> Result<Vec<pb::contract::v1::Repay>, substreams::errors::Error> {
    Ok(blk
        .receipts()
        .flat_map(|view| {
            view.receipt
                .logs
                .iter()
                .filter(|log| log.address == TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::contract::events::Repay::match_and_decode(log) {
                        return Some(contract::Repay {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            repayer: Hex::encode(event.repayer),
                            reserve: Hex::encode(event.reserve),
                            use_a_tokens: event.use_a_tokens,
                            user: Hex::encode(event.user),
                        });
                    }

                    None
                })
        })
        .collect())
}

// #[substreams::handlers::store]
// pub fn store_repays(events: contract::Events, s: substreams::store::StoreAddBigInt) {
//     for repay in events.repays {
//         s.add(
//             0,
//             repay.reserve,
//             substreams::scalar::BigInt::from_str(&repay.amount)
//                 .unwrap_or(substreams::scalar::BigInt::zero()),
//         );
//     }
// }
