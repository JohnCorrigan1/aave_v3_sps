use crate::abi::{self};
use crate::contract;
use crate::eth::Block;
use crate::pb;
use crate::TRACKED_CONTRACT;
use substreams::Hex;
use substreams_ethereum::Event;

pub fn get_reserve_data_updateds(
    blk: &Block,
) -> Result<Vec<pb::contract::v1::ReserveDataUpdated>, substreams::errors::Error> {
    Ok(blk
        .receipts()
        .flat_map(|view| {
            view.receipt
                .logs
                .iter()
                .filter(|log| log.address == TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) =
                        abi::contract::events::ReserveDataUpdated::match_and_decode(log)
                    {
                        return Some(contract::ReserveDataUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            liquidity_index: event.liquidity_index.to_string(),
                            liquidity_rate: event.liquidity_rate.to_string(),
                            reserve: event.reserve,
                            stable_borrow_rate: event.stable_borrow_rate.to_string(),
                            variable_borrow_index: event.variable_borrow_index.to_string(),
                            variable_borrow_rate: event.variable_borrow_rate.to_string(),
                        });
                    }

                    None
                })
        })
        .collect())
}
