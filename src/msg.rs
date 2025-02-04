use serde::{Deserialize, Serialize};
use prost::Message;

#[derive(Serialize, Deserialize)]
pub struct QueryResp {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetMessage { name: String },
    GetCount {},
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    PushOracleResult {
        job_id: u64,
        results_json: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct CountResp {
    pub count: u32,
}

#[derive(Message)]
pub struct MsgOraclePushResult {
    #[prost(string, tag = "1")]
    pub creator: String,
    #[prost(uint64, tag = "2")]
    pub job_id: u64,
    #[prost(string, tag = "3")]
    pub results_json: String,
}

/// MsgExec allows an account to execute messages using an authorization.
/// https://github.com/cosmos/cosmos-sdk/blob/fe1c8af22ae3768ffa840c6caaf8ff0cb27149b5/x/authz/proto/cosmos/authz/v1beta1/tx.proto#L63
#[derive(Message)]
pub struct MsgExec {
    /// The account executing the message (must have been granted authorization).
    #[prost(string, tag = "1")]
    pub grantee: String,

    /// A list of messages to execute (wrapped in `Any` format).
    #[prost(message, repeated, tag = "2")]
    pub msgs: Vec<Any>,
}

/// `Any` represents a Cosmos SDK message with a type URL and binary payload.
#[derive(Message)]
pub struct Any {
    #[prost(string, tag = "1")]
    pub type_url: String,

    #[prost(bytes, tag = "2")]
    pub value: Vec<u8>,
}