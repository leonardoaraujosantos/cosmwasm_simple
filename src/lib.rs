use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
    CosmosMsg, AnyMsg,
};
mod msg;
mod state;
use crate::msg::{ExecuteMsg, QueryMsg, CountResp, QueryResp, MsgOraclePushResult, MsgExec, Any};
use crate::state::{initialize_counter, load_counter, increment_counter};
use prost::Message; 

#[entry_point]
pub fn instantiate(deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    initialize_counter(deps.storage)?;
    Ok(Response::new())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment{} => execute_increment(deps),
        ExecuteMsg::PushOracleResult { job_id, results_json } => {
            execute_push_oracle_result(job_id, results_json, info.sender.to_string(), env)
        }
    }
}

fn execute_increment(deps: DepsMut) -> StdResult<Response> {
    let new_count = increment_counter(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "increment")
        .add_attribute("new_count", new_count.to_string()))
}

fn execute_push_oracle_result(job_id: u64, results_json: String, sender: String, env: Env) -> StdResult<Response> {
    // ✅ Create a Protobuf message for `MsgOraclePushResult` that will be executed by the blockchain
    let msg = MsgOraclePushResult {
        creator: sender.clone(),
        job_id,
        results_json,
    };
    // Encode MsgOraclePushResult as Protobuf of type Any
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();

    // Preprare Custom message Call to /aminichain.apigateway.MsgOraclePushResult
    let oracle_msg = Any {  
        type_url: "/aminichain.apigateway.MsgOraclePushResult".to_string(),
        value: buf,
    };

    // ✅ Wrap `oracle_msg` inside `MsgExec` for Authz execution
    // 🔹 The grantee is the contract itself, allowing it to execute the message with delegated permission.
    let exec_msg = MsgExec {
        grantee: env.contract.address.to_string(),
        msgs: vec![oracle_msg],  // ✅ Use Protobuf Any inside MsgExec
    };
    
    // Encode MsgExec as Protobuf of type Any
    let mut exec_buf = Vec::new();
    exec_msg.encode(&mut exec_buf).unwrap();

    // Preprare Authz message Call to /cosmos.authz.v1beta1.MsgExec
    // 🔹 This is still Protobuf-encoded
    let msg_exec = CosmosMsg::Any(AnyMsg { 
        type_url: "/cosmos.authz.v1beta1.MsgExec".to_string(),
        value: Binary::from(exec_buf),
    });

    Ok(Response::new()
        .add_message(msg_exec)
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