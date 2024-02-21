use crate::abi::{self};
use crate::contract;
use crate::eth::Block;
use crate::pb;
use crate::TRACKED_CONTRACT;
use substreams::Hex;
use substreams_ethereum::Event;

pub fn get_minted_to_treasuries(
    blk: &Block,
) -> Result<Vec<pb::contract::v1::MintedToTreasury>, substreams::errors::Error> {
    Ok(blk
        .receipts()
        .flat_map(|view| {
            view.receipt
                .logs
                .iter()
                .filter(|log| log.address == TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) =
                        abi::contract::events::MintedToTreasury::match_and_decode(log)
                    {
                        return Some(contract::MintedToTreasury {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount_minted: event.amount_minted.to_string(),
                            reserve: Hex::encode(event.reserve),
                        });
                    }

                    None
                })
        })
        .collect())
}
