// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveInitializations {
    #[prost(message, repeated, tag="1")]
    pub reserve_initializations: ::prost::alloc::vec::Vec<ReserveInitialization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveInitialization {
    #[prost(string, tag="1")]
    pub asset: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub a_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub stable_debt_token: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub variable_debt_token: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub interest_rate_strategy_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tokens {
    #[prost(message, repeated, tag="1")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub asset: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub a_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub a_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub a_symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub decimals: u64,
    #[prost(uint64, tag="8")]
    pub a_decimals: u64,
}
// @@protoc_insertion_point(module)
