use crate::abi::{self};
use crate::contract;
use crate::eth::Block;
use crate::pb;
use crate::TRACKED_CONTRACT;
use substreams::Hex;
use substreams_ethereum::Event;

pub fn get_borrows(
    blk: &Block,
) -> Result<Vec<pb::contract::v1::Borrow>, substreams::errors::Error> {
    Ok(blk
        .receipts()
        .flat_map(|view| {
            view.receipt
                .logs
                .iter()
                .filter(|log| log.address == TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::contract::events::Borrow::match_and_decode(log) {
                        return Some(contract::Borrow {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            borrow_rate: event.borrow_rate.to_string(),
                            interest_rate_mode: event.interest_rate_mode.to_u64(),
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
