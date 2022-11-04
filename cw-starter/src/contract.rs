#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};


const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    //setting our Config
    //unwrap our Option field and if it is null sets it to info.sender.to_string() which is the message sender's address as a string.
   let admin =msg.admin.unwrap_or(info.sender.to_string());
   //It then validates the address by passing it as a &str to the deps.api.addr_validate() function. This validates if an address is valid and throws an error otherwise. The ? is an error handler
   let validated_admin=deps.api.addr_validate(&admin)?;
   //The next line creates a Config struct with our validated admin address as the admin. We have to clone the validated address to avoid moving values.
   let config = Config {
    admin: validated_admin.clone(),
   };
   //stores it in our CONFIG storage
   CONFIG.save(deps.storage, &config)?;
   //return line indicated by no ;. This returns a success using the Ok and Result structure.
   Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
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

#[cfg(test)]
mod tests {}
