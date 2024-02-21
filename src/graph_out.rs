use std::str::FromStr;
use substreams::scalar::BigDecimal;
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = EntityChangesTables::new();

    // Loop over all the abis events to create changes
    events.back_unbackeds.into_iter().for_each(|evt| {
        tables
            .create_row(
                "back_unbacked",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("backer", Hex(&evt.backer).to_string())
            .set("fee", BigDecimal::from_str(&evt.fee).unwrap())
            .set("reserve", Hex(&evt.reserve).to_string());
    });
    events.borrows.into_iter().for_each(|evt| {
        tables
            .create_row("borrow", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set(
                "borrow_rate",
                BigDecimal::from_str(&evt.borrow_rate).unwrap(),
            )
            .set("interest_rate_mode", evt.interest_rate_mode)
            .set("on_behalf_of", Hex(&evt.on_behalf_of).to_string())
            .set("referral_code", evt.referral_code)
            .set("reserve", Hex(&evt.reserve).to_string())
            .set("user", Hex(&evt.user).to_string());
    });
    events.flash_loans.into_iter().for_each(|evt| {
        tables
            .create_row(
                "flash_loan",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("asset", Hex(&evt.asset).to_string())
            .set("initiator", Hex(&evt.initiator).to_string())
            .set("interest_rate_mode", evt.interest_rate_mode)
            .set("premium", BigDecimal::from_str(&evt.premium).unwrap())
            .set("referral_code", evt.referral_code)
            .set("target", Hex(&evt.target).to_string());
    });
    events
        .isolation_mode_total_debt_updateds
        .into_iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "isolation_mode_total_debt_updated",
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
                .set("evt_block_time", evt.evt_block_time.unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set("asset", Hex(&evt.asset).to_string())
                .set("total_debt", BigDecimal::from_str(&evt.total_debt).unwrap());
        });
    events.liquidation_calls.into_iter().for_each(|evt| {
        tables
            .create_row(
                "liquidation_call",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("collateral_asset", Hex(&evt.collateral_asset).to_string())
            .set("debt_asset", Hex(&evt.debt_asset).to_string())
            .set(
                "debt_to_cover",
                BigDecimal::from_str(&evt.debt_to_cover).unwrap(),
            )
            .set(
                "liquidated_collateral_amount",
                BigDecimal::from_str(&evt.liquidated_collateral_amount).unwrap(),
            )
            .set("liquidator", Hex(&evt.liquidator).to_string())
            .set("receive_a_token", evt.receive_a_token)
            .set("user", Hex(&evt.user).to_string());
    });
    events.mint_unbackeds.into_iter().for_each(|evt| {
        tables
            .create_row(
                "mint_unbacked",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("on_behalf_of", Hex(&evt.on_behalf_of).to_string())
            .set("referral_code", evt.referral_code)
            .set("reserve", Hex(&evt.reserve).to_string())
            .set("user", Hex(&evt.user).to_string());
    });
    events.minted_to_treasuries.into_iter().for_each(|evt| {
        tables
            .create_row(
                "minted_to_treasury",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set(
                "amount_minted",
                BigDecimal::from_str(&evt.amount_minted).unwrap(),
            )
            .set("reserve", Hex(&evt.reserve).to_string());
    });
    events
        .rebalance_stable_borrow_rates
        .into_iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "rebalance_stable_borrow_rate",
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
                .set("evt_block_time", evt.evt_block_time.unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set("reserve", Hex(&evt.reserve).to_string())
                .set("user", Hex(&evt.user).to_string());
        });
    events.repays.into_iter().for_each(|evt| {
        tables
            .create_row("repay", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("repayer", Hex(&evt.repayer).to_string())
            .set("reserve", Hex(&evt.reserve).to_string())
            .set("use_a_tokens", evt.use_a_tokens)
            .set("user", Hex(&evt.user).to_string());
    });
    events.reserve_data_updateds.into_iter().for_each(|evt| {
        tables
            .create_row(
                "reserve_data_updated",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set(
                "liquidity_index",
                BigDecimal::from_str(&evt.liquidity_index).unwrap(),
            )
            .set(
                "liquidity_rate",
                BigDecimal::from_str(&evt.liquidity_rate).unwrap(),
            )
            .set("reserve", Hex(&evt.reserve).to_string())
            .set(
                "stable_borrow_rate",
                BigDecimal::from_str(&evt.stable_borrow_rate).unwrap(),
            )
            .set(
                "variable_borrow_index",
                BigDecimal::from_str(&evt.variable_borrow_index).unwrap(),
            )
            .set(
                "variable_borrow_rate",
                BigDecimal::from_str(&evt.variable_borrow_rate).unwrap(),
            );
    });
    events
        .reserve_used_as_collateral_disableds
        .into_iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "reserve_used_as_collateral_disabled",
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
                .set("evt_block_time", evt.evt_block_time.unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set("reserve", Hex(&evt.reserve).to_string())
                .set("user", Hex(&evt.user).to_string());
        });
    events
        .reserve_used_as_collateral_enableds
        .into_iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "reserve_used_as_collateral_enabled",
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
                .set("evt_block_time", evt.evt_block_time.unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set("reserve", Hex(&evt.reserve).to_string())
                .set("user", Hex(&evt.user).to_string());
        });
    events.supplies.into_iter().for_each(|evt| {
        tables
            .create_row("supply", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("on_behalf_of", Hex(&evt.on_behalf_of).to_string())
            .set("referral_code", evt.referral_code)
            .set("reserve", Hex(&evt.reserve).to_string())
            .set("user", Hex(&evt.user).to_string());
    });
    events.swap_borrow_rate_modes.into_iter().for_each(|evt| {
        tables
            .create_row(
                "swap_borrow_rate_mode",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("interest_rate_mode", evt.interest_rate_mode)
            .set("reserve", Hex(&evt.reserve).to_string())
            .set("user", Hex(&evt.user).to_string());
    });
    events.upgradeds.into_iter().for_each(|evt| {
        tables
            .create_row("upgraded", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });
    events.user_e_mode_sets.into_iter().for_each(|evt| {
        tables
            .create_row(
                "user_e_mode_set",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("category_id", evt.category_id)
            .set("user", Hex(&evt.user).to_string());
    });
    events.withdraws.into_iter().for_each(|evt| {
        tables
            .create_row("withdraw", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("reserve", Hex(&evt.reserve).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("user", Hex(&evt.user).to_string());
    });

    Ok(tables.to_entity_changes())
}
