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
mod tests {
    use cosmwasm_std::attr; // helper to construct an attribute e.g. ("action", "instantiate")
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info}; // mock functions to mock an environment, message info, dependencies
    use crate::contract::instantiate; // the contract instantiate function
    use crate::msg::InstantiateMsg; // our instantate method

    // Two fake addresses we will use to mock_info
    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";
    //need this to make fn and the variables work
    #[test]
    fn test_instantiate(){
         // Mock the dependencies, must be mutable so we can pass it as a mutable, empty vector means our contract has no balance
         let mut deps = mock_dependencies();
         // Mock the contract environment, contains the block info, contract address, etc.
         let env = mock_env();
         // Mock the message info, ADDR1 will be the sender, the empty vec means we sent no funds.
         let info = mock_info(ADDR1, &vec![]);

          // Create a message where we (the sender) will be an admin
        let msg = InstantiateMsg { admin: None };
        // Call instantiate, unwrap to assert success
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        //a macro used in testing to assert two values are the same. It will panic otherwise and fail the test.
        assert_eq!(
            //A Response object has a member variable which is a vector of attributes (Vec<Attribute>)
            // in this case we access it using res.attributes
             //we want to assert that this is equal to another vector of attributes
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)]
        )
    }
}
