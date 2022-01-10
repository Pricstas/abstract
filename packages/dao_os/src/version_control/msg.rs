use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddCodeId {
        module: String,
        version: String,
        code_id: u64,
    },
    RemoveCodeId {
        module: String,
        version: String,
    },
    AddOs {
        os_id: u32,
        os_manager_address: String,
    },
    RemoveOs {
        os_id: u32,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Queries assets based on name
    QueryEnabledModules { os_address: String },
    /// Queries address of OS manager module
    QueryOsAddress { os_id: u32 },
}
