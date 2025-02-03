use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
    CosmosMsg, AnyMsg, 
};
mod msg;
mod state;
use crate::msg::{ExecuteMsg, QueryMsg, CountResp, QueryResp, MsgOraclePushResult};
use crate::state::{initialize_counter, load_counter, increment_counter};
use prost::Message; 

#[entry_point]
pub fn instantiate(deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    initialize_counter(deps.storage)?;
    Ok(Response::new())
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment{} => execute_increment(deps),
        ExecuteMsg::PushOracleResult { job_id, results_json } => {
            execute_push_oracle_result(job_id, results_json, info.sender.to_string())
        }
    }
}

fn execute_increment(deps: DepsMut) -> StdResult<Response> {
    let new_count = increment_counter(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "increment")
        .add_attribute("new_count", new_count.to_string()))
}

fn execute_push_oracle_result(job_id: u64, results_json: String, sender: String) -> StdResult<Response> {
    let msg = MsgOraclePushResult {
        creator: sender,
        job_id,
        results_json,
    };
    
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap(); // Encode to protobuf bytes

    let msg = CosmosMsg::Any(AnyMsg {
        type_url: "/aminichain.apigateway.MsgOraclePushResult".to_string(),
        value: Binary::from(buf),
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "push_oracle_result")
        .add_attribute("job_id", job_id.to_string()))
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMessage { name } => query_message(name),
        QueryMsg::GetCount {} => query_count(deps),
    }
}

fn query_message(name: String) -> StdResult<Binary> {
    let message = format!("Hello World {}", name);
    let resp = QueryResp { message };
    to_json_binary(&resp)
}

fn query_count(deps: Deps) -> StdResult<Binary> {
    let count = load_counter(deps.storage)?;
    let resp = CountResp { count };
    to_json_binary(&resp)
}