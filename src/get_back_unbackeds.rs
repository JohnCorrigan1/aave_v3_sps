use crate::abi::{self};
use crate::contract as contract2;
use crate::eth::Block;
use crate::pb;
use crate::TRACKED_CONTRACT;
use substreams::Hex;
use substreams_ethereum::Event;

pub fn get_back_unbackeds(
    blk: &Block,
) -> Result<Vec<pb::contract::v1::BackUnbacked>, substreams::errors::Error> {
    return Ok(blk
        .receipts()
        .flat_map(|view| {
            view.receipt
                .logs
                .iter()
                .filter(|log| log.address == TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::contract::events::BackUnbacked::match_and_decode(log)
                    {
                        return Some(contract2::BackUnbacked {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            backer: event.backer,
                            fee: event.fee.to_string(),
                            reserve: event.reserve,
                        });
                    }

                    None
                })
        })
        .collect());
}
