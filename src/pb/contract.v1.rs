// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub back_unbackeds: ::prost::alloc::vec::Vec<BackUnbacked>,
    #[prost(message, repeated, tag="2")]
    pub borrows: ::prost::alloc::vec::Vec<Borrow>,
    #[prost(message, repeated, tag="3")]
    pub flash_loans: ::prost::alloc::vec::Vec<FlashLoan>,
    #[prost(message, repeated, tag="4")]
    pub isolation_mode_total_debt_updateds: ::prost::alloc::vec::Vec<IsolationModeTotalDebtUpdated>,
    #[prost(message, repeated, tag="5")]
    pub liquidation_calls: ::prost::alloc::vec::Vec<LiquidationCall>,
    #[prost(message, repeated, tag="6")]
    pub mint_unbackeds: ::prost::alloc::vec::Vec<MintUnbacked>,
    #[prost(message, repeated, tag="7")]
    pub minted_to_treasuries: ::prost::alloc::vec::Vec<MintedToTreasury>,
    #[prost(message, repeated, tag="8")]
    pub rebalance_stable_borrow_rates: ::prost::alloc::vec::Vec<RebalanceStableBorrowRate>,
    #[prost(message, repeated, tag="9")]
    pub repays: ::prost::alloc::vec::Vec<Repay>,
    #[prost(message, repeated, tag="10")]
    pub reserve_data_updateds: ::prost::alloc::vec::Vec<ReserveDataUpdated>,
    #[prost(message, repeated, tag="11")]
    pub reserve_used_as_collateral_disableds: ::prost::alloc::vec::Vec<ReserveUsedAsCollateralDisabled>,
    #[prost(message, repeated, tag="12")]
    pub reserve_used_as_collateral_enableds: ::prost::alloc::vec::Vec<ReserveUsedAsCollateralEnabled>,
    #[prost(message, repeated, tag="13")]
    pub supplies: ::prost::alloc::vec::Vec<Supply>,
    #[prost(message, repeated, tag="14")]
    pub swap_borrow_rate_modes: ::prost::alloc::vec::Vec<SwapBorrowRateMode>,
    #[prost(message, repeated, tag="15")]
    pub upgradeds: ::prost::alloc::vec::Vec<Upgraded>,
    #[prost(message, repeated, tag="16")]
    pub user_e_mode_sets: ::prost::alloc::vec::Vec<UserEModeSet>,
    #[prost(message, repeated, tag="17")]
    pub withdraws: ::prost::alloc::vec::Vec<Withdraw>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackUnbacked {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub reserve: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub backer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Borrow {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub reserve: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub on_behalf_of: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag="9")]
    pub interest_rate_mode: u64,
    #[prost(string, tag="10")]
    pub borrow_rate: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub referral_code: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlashLoan {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub target: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub initiator: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub asset: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag="9")]
    pub interest_rate_mode: u64,
    #[prost(string, tag="10")]
    pub premium: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub referral_code: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsolationModeTotalDebtUpdated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub asset: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub total_debt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidationCall {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub collateral_asset: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub debt_asset: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub debt_to_cover: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub liquidated_collateral_amount: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub liquidator: ::prost::alloc::string::String,
    #[prost(bool, tag="11")]
    pub receive_a_token: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintUnbacked {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub reserve: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub on_behalf_of: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag="9")]
    pub referral_code: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintedToTreasury {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub reserve: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub amount_minted: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebalanceStableBorrowRate {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub reserve: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub user: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Repay {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub reserve: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub repayer: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub use_a_tokens: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveDataUpdated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub reserve: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub liquidity_rate: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub stable_borrow_rate: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub variable_borrow_rate: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub liquidity_index: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub variable_borrow_index: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveUsedAsCollateralDisabled {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub reserve: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub user: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveUsedAsCollateralEnabled {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub reserve: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub user: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Supply {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub reserve: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub on_behalf_of: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag="9")]
    pub referral_code: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapBorrowRateMode {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub reserve: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub interest_rate_mode: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Upgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEModeSet {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="6")]
    pub category_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Withdraw {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub reserve: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
