// Library for serialization of messages
use serde::{Deserialize, Serialize};

// Define Struct for response
#[derive(Serialize, Deserialize)]
pub struct QueryResp {
    pub message: String,
}

// Define a query input messages structure
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetMessage { name: String }, // Hello World <name>
    GetCount {},                    // Counter value
}

// Execution input message to increment the counter
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
}

// Query output message to get the counter value
#[derive(Serialize, Deserialize)]
pub struct CountResp {
    pub count: u32,
}