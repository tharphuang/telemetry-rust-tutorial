/// backend
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Func {
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub func_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub arg: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub cost_time: i64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Callback {
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub calllback: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub arg: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub cost_time: i64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageType {
    FuncMsg = 0,
    CallbackMsg = 1,
}
impl MessageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MessageType::FuncMsg => "FuncMsg",
            MessageType::CallbackMsg => "CallbackMsg",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FuncMsg" => Some(Self::FuncMsg),
            "CallbackMsg" => Some(Self::CallbackMsg),
            _ => None,
        }
    }
}
