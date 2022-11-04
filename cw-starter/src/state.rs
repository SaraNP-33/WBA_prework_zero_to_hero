// allows structs to be serialized and deserialized to and from JSON
use schemars::JsonSchema;
//provide the serialization described above.
use serde::{Deserialize, Serialize};

//Cosmos address, under the hood it is simply a string
use cosmwasm_std::Addr;
//helper provided by storage plus. It effectively means we can store an item in storage. In this case, the STATE variable is an Item that stores a singular State struct.
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Poll {
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Ballot {
    pub option: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

// A map with a String key and Poll value.
// The key will be a UUID generated clientside
pub const POLLS: Map<String, Poll> = Map::new("polls");

//We're going to use a composite key.his composite key will be in the format of (Addr, String). Where Addr is the address of the voter and String is the Poll UUID this vote is for.
pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");
