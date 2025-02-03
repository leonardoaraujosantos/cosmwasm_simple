use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
mod msg;
mod state;
use crate::msg::{ExecuteMsg, QueryMsg, CountResp, QueryResp};
use crate::state::{initialize_counter, load_counter, increment_counter};

// Entrypoint para Instantiation (inicializa o contador como 0)
#[entry_point]
pub fn instantiate(deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    initialize_counter(deps.storage)?;
    Ok(Response::new())
}

// Entry point para Execução (incrementa o contador)
#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, _info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment{} => execute_increment(deps),
    }
}

// Função que incrementa o contador
fn execute_increment(deps: DepsMut) -> StdResult<Response> {
    let new_count = increment_counter(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "increment")
        .add_attribute("new_count", new_count.to_string()))
}

// Entry point para Queries (retorna o valor do contador ou a mensagem personalizada)
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMessage { name } => query_message(name),
        QueryMsg::GetCount{} => query_count(deps),
    }
}

// Função que retorna "Hello World <name>"
fn query_message(name: String) -> StdResult<Binary> {
    let message = format!("Hello World {}", name);
    let resp = QueryResp { message };
    to_json_binary(&resp)
}

// Função que retorna o valor do contador
fn query_count(deps: Deps) -> StdResult<Binary> {
    let count = load_counter(deps.storage)?;
    let resp = CountResp { count };
    
    Ok(to_json_binary(&resp)?)
}