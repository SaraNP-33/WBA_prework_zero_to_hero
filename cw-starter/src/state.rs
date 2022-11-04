// allows structs to be serialized and deserialized to and from JSON
use schemars::JsonSchema;
//provide the serialization described above.
use serde::{Deserialize, Serialize};

//Cosmos address, under the hood it is simply a string
use cosmwasm_std::Addr;
//helper provided by storage plus. It effectively means we can store an item in storage. In this case, the STATE variable is an Item that stores a singular State struct.
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr
}
pub struct Poll{
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>
}
pub struct Ballot{
    pub option: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
