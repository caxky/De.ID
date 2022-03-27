use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StudentAccreditation {
    pub admin: Addr,
    pub studentname: String,
    pub studentid: i32,
    pub universitites: Vec<String>,
    pub degrees: Vec<String>,
}

pub const STUDENTACC: Item<StudentAccreditation> = Item::new("studentacc");
