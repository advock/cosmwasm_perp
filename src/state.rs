use cosmwasm_std::{
    Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Uint256,
};

use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConditionalOrder {
    pub marketkey: u128,
    pub initial_margin: Uint128,
    pub margin: Uint128,
    pub limit_Price: Option<Uint128>,
    pub conditional_Order: ConditionalOrderTypes,
    pub trade_type: TradeType,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ConditionalOrderTypes {
    Limit,
    Stop,
    Market,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TradeType {
    Long,
    Short,
}

pub const CONDITIONALORDERS: Map<u128, ConditionalOrder> = Map::new("conditionalOrders");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PositionState {
    pub taskID: u128,
    pub tradeType: TradeType,
    pub lastFundingIndex: u128,
    pub margin: Uint128,
    pub buy_price: Uint128,
    pub size: Uint128,
}

pub const POSITION: Map<Addr, PositionState> = Map::new("position");

pub const MARGIN: Map<Addr, Uint128> = Map::new("initialMargin");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Synth {
    pub id: Addr,
    pub asset: String,
    pub Lastprice: Uint128,
}

pub const TASKID: Item<Vec<u128>> = Item::new("taskId's");

pub const ORDERS: Map<Addr, Vec<u128>> = Map::new("orders");

pub const EXECUTEORDER: Map<u128, bool> = Map::new("ExecutedOrders");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LimitOrder {
    pub user: Addr,
    pub margin: Uint128,
    pub limitPrice: Uint128,
    pub tradeType: TradeType,
}

pub const LIMITORDER: Map<u128, LimitOrder> = Map::new("limitorders");
