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