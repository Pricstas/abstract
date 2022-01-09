use cw_controllers::Admin;
use cw_storage_plus::{Map, U32Key};

pub const ADMIN: Admin = Admin::new("admin");

// Map with composite keys
// module name + version = code_id
// We can interate over the map giving just the prefix to get all the versions
pub const MODULE_CODE_IDS: Map<(&str, &str), u64> = Map::new("module_code_ids");


// Maps OS ID to the address of it's manager contract
pub const OS_ADDRESSES: Map<U32Key, String> = Map::new("os_addresses");
