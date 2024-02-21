use crate::abi::{self};
use crate::contract;
use crate::eth::Block;
use crate::pb;
use crate::TRACKED_CONTRACT;
use substreams::Hex;
use substreams_ethereum::Event;

pub fn get_mint_unbackeds(
    blk: &Block,
) -> Result<Vec<pb::contract::v1::MintUnbacked>, substreams::errors::Error> {
    Ok(blk
        .receipts()
        .flat_map(|view| {
            view.receipt
                .logs
                .iter()
                .filter(|log| log.address == TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::contract::events::MintUnbacked::match_and_decode(log)
                    {
                        return Some(contract::MintUnbacked {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            on_behalf_of: event.on_behalf_of,
                            referral_code: event.referral_code.to_u64(),
                            reserve: event.reserve,
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect())
}
