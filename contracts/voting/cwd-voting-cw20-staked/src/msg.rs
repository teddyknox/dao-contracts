use cosmwasm_std::{Decimal, Uint128};
use cw20::Cw20Coin;
use cw20_base::msg::InstantiateMarketingInfo;
use cw20_vest::msg::Schedule;
use cw_utils::Duration;
use cwd_macros::{active_query, info_query, token_query, voting_query};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StakingInfo {
    Existing {
        staking_contract_address: String,
    },
    New {
        staking_code_id: u64,
        unstaking_duration: Option<Duration>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum VestingInfo {
    Existing {
        vesting_contract_address: String,
    },
    New(VestingInfoNew),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VestingInfoNew {
    pub vesting_code_id: u64,
    pub schedules: Vec<Schedule>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum TokenInfo {
    Existing {
        address: String,
        staking_contract: StakingInfo,
        vesting_contract: Option<VestingInfo>,
    },
    New {
        code_id: u64,
        label: String,

        name: String,
        symbol: String,
        decimals: u8,
        initial_balances: Vec<Cw20Coin>,
        marketing: Option<InstantiateMarketingInfo>,
        staking_code_id: u64,
        unstaking_duration: Option<Duration>,
        initial_dao_balance: Option<Uint128>,
        vesting_info: Option<VestingInfoNew>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActiveThreshold {
    AbsoluteCount { count: Uint128 },
    Percentage { percent: Decimal },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub token_info: TokenInfo,
    pub active_threshold: Option<ActiveThreshold>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateActiveThreshold {
        new_threshold: Option<ActiveThreshold>,
    },
}

#[voting_query]
#[info_query]
#[token_query]
#[active_query]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    StakingContract {},
    VestingContract {},
    Dao {},
    ActiveThreshold {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ActiveThresholdResponse {
    pub active_threshold: Option<ActiveThreshold>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}
