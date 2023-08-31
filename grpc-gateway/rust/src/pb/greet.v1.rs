// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GreetRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GreetResponse {
    #[prost(string, tag="1")]
    pub greeting: ::prost::alloc::string::String,
}
/// ヘルスチェックレスポンス
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResponse {
    #[prost(string, tag="1")]
    pub msg: ::prost::alloc::string::String,
}
include!("greet.v1.tonic.rs");
// @@protoc_insertion_point(module)