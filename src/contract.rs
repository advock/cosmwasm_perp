use std::borrow::Borrow;
use std::path::PrefixComponent;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
    Uint128, Uint256, WasmMsg,
};
// use cw2::set_contract_version;
use crate::state::{
    ConditionalOrder, ConditionalOrderTypes, LimitOrder, PositionState, Synth, TradeType,
    EXECUTEORDER, LIMITORDER, MARGIN, POSITION, TASKID,
};

use crate::error::{self, ContractError};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{
    check_if_order_is_executed, getPosition, get_asset_info, get_max_magrin_for_asset,
    get_synth_price, query_TaskIDs, user_balance, Quote_asset_size,
};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, Cw20ReceiveMsg};
use cw20_base::{self};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:comdex-perp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

pub const CMST_ADDR: &str = "CMST";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

pub fn add_stablecoin(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let msg = to_binary(&Cw20ExecuteMsg::Transfer {
        recipient: _env.contract.address.to_string(),
        amount: amount,
    })?;

    let transfer_msg = WasmMsg::Execute {
        contract_addr: CMST_ADDR.to_string(),
        msg: msg,
        funds: vec![],
    };
    MARGIN.save(_deps.storage, _info.sender.clone(), &amount)?;

    let response = Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "transfer")
        .add_attribute("sender", _info.sender.to_string());

    Ok(response)
}

pub fn TradeSynth(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    order: ConditionalOrder,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // get leverage
    // find price
    // allot syth to user

    let asset: Synth = get_asset_info(order.marketkey);
    let userBalance = user_balance(_deps.as_ref(), _info.sender.clone())?;

    if userBalance < order.initial_margin {
        return unimplemented!();
    }

    let Max_margin = get_max_magrin_for_asset(order.marketkey);

    if Max_margin < order.margin {
        return unimplemented!();
    }

    let trade_value = order.margin * order.initial_margin;

    let size = Quote_asset_size(trade_value, order.marketkey);

    match order.trade_type {
        TradeType::Long => match order.conditional_Order {
            ConditionalOrderTypes::Limit => {
                if !order.limit_Price.is_none() {
                    let limit_price = order.clone().limit_Price.unwrap();
                    if asset.Lastprice <= limit_price {
                        _buyasset(
                            _deps,
                            _env,
                            _info.sender.clone(),
                            order.margin,
                            trade_value,
                            asset.Lastprice,
                            size,
                        );
                    } else {
                        // place order
                        let LimitOrder = LimitOrder {
                            user: _info.sender,
                            margin: order.margin,
                            limitPrice: order.limit_Price.unwrap(),
                            tradeType: TradeType::Long,
                        };

                        placeOrder(_deps, _env, LimitOrder);
                    }
                } else {
                    unimplemented!()
                }
            }

            ConditionalOrderTypes::Market => {
                let price = _buyasset(
                    _deps,
                    _env,
                    _info.sender.clone(),
                    order.margin,
                    trade_value,
                    asset.Lastprice,
                    size,
                );
            }

            ConditionalOrderTypes::Stop => {}
        },
        TradeType::Short => match order.conditional_Order {
            ConditionalOrderTypes::Limit => {
                if !order.limit_Price.is_none() {
                    let limit_price = order.limit_Price.unwrap();
                    if asset.Lastprice <= limit_price {
                        _buyasset(
                            _deps,
                            _env,
                            _info.sender.clone(),
                            order.margin,
                            trade_value,
                            order.limit_Price.unwrap(),
                            size,
                        );
                    } else {
                        // _orderBuyAsset(_deps, _env, order, _info.sender);
                    }
                } else {
                    unimplemented!()
                }
            }

            ConditionalOrderTypes::Market => {
                _buyasset(
                    _deps,
                    _env,
                    _info.sender.clone(),
                    order.margin,
                    asset.Lastprice * order.initial_margin,
                    asset.Lastprice,
                    size,
                );
            }

            ConditionalOrderTypes::Stop => {}
        },
    }

    Err(ContractError::Unauthorized {})
}

pub fn _buyasset(
    deps: DepsMut,
    _env: Env,
    user: Addr,
    margin: Uint128,
    trade_value: Uint128,
    price: Uint128,
    size: Uint128,
) {
    let mut tasks = query_TaskIDs(deps.as_ref(), user).unwrap();
    let taskID = tasks.last().unwrap() + 1;

    EXECUTEORDER.save(deps.storage, taskID, &true);

    let position = PositionState {
        taskID: taskID,
        tradeType: TradeType::Long,
        lastFundingIndex: unimplemented!(),
        margin: margin,
        buy_price: price,
        size: size,
    };
    POSITION.save(deps.storage, user, &position);
}

pub fn placeOrder(_deps: DepsMut, _env: Env, LimitOrder: LimitOrder) {
    let mut tasks = query_TaskIDs(_deps.as_ref(), LimitOrder.user.clone()).unwrap();
    let taskID = tasks.last().unwrap() + 1;
    tasks.push(taskID);
    TASKID.save(_deps.storage, &tasks);

    EXECUTEORDER.save(_deps.storage, taskID, &false);

    LIMITORDER.save(_deps.storage, taskID, &LimitOrder);
}

#[cfg(test)]
mod tests {}
