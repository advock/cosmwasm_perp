#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
    Uint128, Uint256, WasmMsg,
};
// use cw2::set_contract_version;
use crate::state::{
    ConditionalOrder, ConditionalOrderTypes, PositionState, Synth, TradeType, LIMITORDERS, MARGIN,
    POSITION,
};

use crate::error::{self, ContractError};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{
    getPosition, get_CMST_balance, get_asset_info, get_max_magrin_for_asset, get_synth_price,
    query_limit_orders, user_balance,
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
    MARGIN.save(_deps.storage, _info.sender, &amount)?;

    let response = Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "transfer")
        .add_attribute("sender", _info.sender.to_string());

    Ok(response)
}

pub fn TradeSynth(
    _deps: Deps,
    _env: Env,
    _info: MessageInfo,
    order: ConditionalOrder,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // get leverage
    // find price
    // allot syth to user

    let asset: Synth = get_asset_info(order.marketkey);
    let userBalance = user_balance(_deps, _info.sender)?;

    if userBalance < order.marginDelta {
        return unimplemented!();
    }

    let Max_margin = get_max_magrin_for_asset(order.marketkey);

    if Max_margin < order.margin {
        return unimplemented!();
    }

    let total_trade_value = order.margin * order.marginDelta;

    match order.tradeType {
        TradeType::Long => match order.conditionalOrder {
            ConditionalOrderTypes::Limit => {
                if !order.limitPrice.is_none() {
                    let limit_price = order.limitPrice.unwrap();
                    if asset.Lastprice <= limit_price {
                        _buyasset(_env, order.marketkey, total_trade_value);
                    } else {
                        _orderBuyAsset(_deps, _env, order, _info.sender);
                    }
                } else {
                    unimplemented!()
                }
            }

            ConditionalOrderTypes::Market => {
                _buyasset(_env, order.marketkey, asset.Lastprice * order.margin)
            }

            ConditionalOrderTypes::Stop => {}
        },
        TradeType::Short => match order.conditionalOrder {
            ConditionalOrderTypes::Limit => {
                if !order.limitPrice.is_none() {
                    let limit_price = order.limitPrice.unwrap();
                    if asset.Lastprice <= limit_price {
                        _buyasset(_env, order.marketkey, total_trade_value);
                    } else {
                        _orderBuyAsset(_deps, _env, order, _info.sender);
                    }
                } else {
                    unimplemented!()
                }
            }

            ConditionalOrderTypes::Market => {
                _buyasset(_env, order.marketkey, asset.Lastprice * order.margin)
            }

            ConditionalOrderTypes::Stop => {}
        },
    }

    Err(ContractError::Unauthorized {})
}

pub fn _buyasset(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    order: ConditionalOrder,
    amount: Uint128,
) {
    // allot the amount of asset to user and all liquidation level

    let position = PositionState {
        taskID: order.taskID,
        lastFundingIndex: 1,
        margin: order.margin,
        lastPrice: unimplemented!(),
        size: unimplemented!(),
    };
    POSITION.save(deps.storage, info.sender, &position);
}

pub fn _orderBuyAsset(_deps: DepsMut, _env: Env, order: ConditionalOrder, address: Addr) {
    let mut orders = query_limit_orders(_deps.as_ref(), address).unwrap();

    orders.push(order.taskID);

    LIMITORDERS.save(_deps.storage, address, &orders);
}

pub fn _sellasset(_env: Env, id: u128, amount: Uint128) {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
