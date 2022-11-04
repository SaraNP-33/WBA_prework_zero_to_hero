use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//This is how we communicaate with your contract

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePoll{ //{ExecuteMsg ::CreatePoll{poll_id:"1", question:"something", options:["1", "2", "3"]}}
        poll_id: String,
        question: String,
        options: Vec<String>,
    },
    Vote{
        poll_id: String,
        vote: String,
    },
    // How I think Delete Poll and Revoke poll would be written
    // DeletePoll{
    //     poll_id: String,
    // },
    // RevokeVote{
    //     poll_id:String,
    //     option: String,
    // }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    CustomMsg { val: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponse {
    val: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
