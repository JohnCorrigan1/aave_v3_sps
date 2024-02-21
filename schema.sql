CREATE TABLE IF NOT EXISTS back_unbacked (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    "backer" VARCHAR(40),
    "fee" DECIMAL,
    "reserve" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS borrow (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    "borrow_rate" DECIMAL,
    "interest_rate_mode" INT,
    "on_behalf_of" VARCHAR(40),
    "referral_code" INT,
    "reserve" VARCHAR(40),
    "user" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS flash_loan (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    "asset" VARCHAR(40),
    "initiator" VARCHAR(40),
    "interest_rate_mode" INT,
    "premium" DECIMAL,
    "referral_code" INT,
    "target" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS isolation_mode_total_debt_updated (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "asset" VARCHAR(40),
    "total_debt" DECIMAL
);
CREATE TABLE IF NOT EXISTS liquidation_call (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "collateral_asset" VARCHAR(40),
    "debt_asset" VARCHAR(40),
    "debt_to_cover" DECIMAL,
    "liquidated_collateral_amount" DECIMAL,
    "liquidator" VARCHAR(40),
    "receive_a_token" BOOL,
    "user" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS mint_unbacked (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    "on_behalf_of" VARCHAR(40),
    "referral_code" INT,
    "reserve" VARCHAR(40),
    "user" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS minted_to_treasury (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount_minted" DECIMAL,
    "reserve" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS rebalance_stable_borrow_rate (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "reserve" VARCHAR(40),
    "user" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS repay (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    "repayer" VARCHAR(40),
    "reserve" VARCHAR(40),
    "use_a_tokens" BOOL,
    "user" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS reserve_data_updated (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "liquidity_index" DECIMAL,
    "liquidity_rate" DECIMAL,
    "reserve" VARCHAR(40),
    "stable_borrow_rate" DECIMAL,
    "variable_borrow_index" DECIMAL,
    "variable_borrow_rate" DECIMAL
);
CREATE TABLE IF NOT EXISTS reserve_used_as_collateral_disabled (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "reserve" VARCHAR(40),
    "user" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS reserve_used_as_collateral_enabled (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "reserve" VARCHAR(40),
    "user" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS supply (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    "on_behalf_of" VARCHAR(40),
    "referral_code" INT,
    "reserve" VARCHAR(40),
    "user" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS swap_borrow_rate_mode (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "interest_rate_mode" INT,
    "reserve" VARCHAR(40),
    "user" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS upgraded (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "implementation" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS user_e_mode_set (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "category_id" INT,
    "user" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS withdraw (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    "reserve" VARCHAR(40),
    "to" VARCHAR(40),
    "user" VARCHAR(40)
);
