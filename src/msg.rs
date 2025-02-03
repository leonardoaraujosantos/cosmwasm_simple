use serde::{Deserialize, Serialize};

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