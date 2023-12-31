use crate::contract;
use crate::state::LimitOrder;
use crate::state::PositionState;
use crate::state::Synth;
use crate::state::LIMITORDER;
use crate::state::ORDERS;
use crate::state::TASKID;
use crate::state::{EXECUTEORDER, MARGIN, POSITION};
use crate::ContractError;
use cosmwasm_std::to_binary;
use cosmwasm_std::BalanceResponse;
use cosmwasm_std::Uint128;
use cosmwasm_std::{
    Addr, BankQuery, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response, StdError,
    StdResult, Uint256, WasmQuery,
};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, Cw20ReceiveMsg};
use serde::{Deserialize, Serialize};

pub const CMST_ADDR: &str = "CMST";

pub fn _getCMSTrate(deps: Deps, marketID: Addr) -> Option<(Uint256, Uint256)> {
    unimplemented!()
}

pub fn get_CMST_balance_of_user(deps: Deps, env: Env) -> StdResult<BalanceResponse> {
    // Create a QueryRequest to get the contract's balance for a specific token
    let contract_address_this = env.contract.address.to_string();

    let query_msg = QueryRequest::Wasm(WasmQuery::Raw {
        contract_addr: CMST_ADDR.to_string(),
        key: to_binary(&Cw20QueryMsg::Balance {
            address: contract_address_this,
        })?,
    });

    let balance_response: BalanceResponse = deps.querier.query(&query_msg)?;
    Ok(balance_response)
}

pub fn user_balance(deps: Deps, address: Addr) -> StdResult<Uint128> {
    let res = MARGIN.may_load(deps.storage, address.clone())?;
    match res {
        Some(val) => Ok(val),
        None => Err(StdError::NotFound {
            kind: format!(
                "Unable to load balance of position with address: {}",
                address
            ),
        }),
    }
}

pub fn getPosition(deps: Deps, address: Addr) -> StdResult<PositionState> {
    //position = _getPerpMarket(deps, marketKey);

    let res = POSITION.may_load(deps.storage, address.clone())?;
    match res {
        Some(val) => Ok(val),
        None => Err(StdError::NotFound {
            kind: format!("Unable to load position with address: {}", address),
        }),
    }
}

pub fn Quote_asset_size_at_market_price(amount: Uint128, synth: Synth) -> Uint128 {
    let price = synth.Lastprice;

    let size = amount / price;

    size
}

pub fn Quote_asset_size_at_limit_price(amount: Uint128, limit_price: Uint128) -> Uint128 {
    let size = amount / limit_price;

    size
}

pub fn modifyAccountMargin(amount: i128) {
    if (amount > 0) {
        unimplemented!()
    } else {
        unimplemented!()
    }
}

pub fn get_asset_info(id: u128) -> Synth {
    unimplemented!()
}

pub fn get_synth_price(id: u128) -> Uint128 {
    unimplemented!()
}

pub fn get_max_magrin_for_asset(id: u128) -> Uint128 {
    unimplemented!()
}

pub fn check_if_order_is_executed(deps: Deps, taskID: u128) -> StdResult<bool> {
    let res = EXECUTEORDER.may_load(deps.storage, taskID)?;

    match res {
        Some(val) => Ok(val),
        None => Err(StdError::NotFound {
            kind: format!("Unable to load orders with taskID: {}", taskID),
        }),
    }
}

pub fn query_TaskIDs(deps: Deps, address: Addr) -> StdResult<Vec<u128>> {
    let res: Option<Vec<u128>> = TASKID.may_load(deps.storage)?;

    match res {
        Some(val) => Ok(val),
        None => Err(StdError::NotFound {
            kind: format!("Unable to load orders with address: {}", address),
        }),
    }
}

pub fn query_executed_orders(deps: Deps, taskId: u128) -> StdResult<bool> {
    let res = EXECUTEORDER.may_load(deps.storage, taskId)?;

    match res {
        Some(val) => Ok(val),
        None => Err(StdError::NotFound {
            kind: format!("Unable to load orders with taskID: {}", taskId),
        }),
    }
}

pub fn query_limit_order(deps: Deps, taskID: u128) -> StdResult<LimitOrder> {
    let res = LIMITORDER.may_load(deps.storage, taskID)?;

    match res {
        Some(val) => Ok(val),
        None => Err(StdError::NotFound {
            kind: format!("Unable to load orders with taskID: {}", taskID),
        }),
    }
}
