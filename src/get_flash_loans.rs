use crate::abi::{self};
use crate::contract;
use crate::eth::Block;
use crate::pb;
use crate::TRACKED_CONTRACT;
use substreams::Hex;
use substreams_ethereum::Event;

pub fn get_flash_loans(
    blk: &Block,
) -> Result<Vec<pb::contract::v1::FlashLoan>, substreams::errors::Error> {
    Ok(blk
        .receipts()
        .flat_map(|view| {
            view.receipt
                .logs
                .iter()
                .filter(|log| log.address == TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::contract::events::FlashLoan::match_and_decode(log) {
                        return Some(contract::FlashLoan {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            asset: Hex::encode(event.asset),
                            initiator: Hex::encode(event.initiator),
                            interest_rate_mode: event.interest_rate_mode.to_u64(),
                            premium: event.premium.to_string(),
                            referral_code: event.referral_code.to_u64(),
                            target: Hex::encode(event.target),
                        });
                    }

                    None
                })
        })
        .collect())
}
