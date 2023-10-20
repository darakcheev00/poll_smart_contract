#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use crate::state::{Config, CONFIG, Poll, POLLS};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetPollResponse};



// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tut-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
// env reads from the cargo.toml


// api endpoints

// can only create these entrypoints and three others:
// migrate (move versions), reply(if you want to reply from a message), sudo(governance contracts. rarely used) 

// instantiate
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env, // stores env variables around contract
    _info: MessageInfo, // metadata, who sent the message, what are they paying
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // deps = dependancies to our contract. gave it deps.storage so that name and version could be stored in the storage
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // _env.block.chain_id
    // _info.sender = person who sent the message
    // _info.funds = list of coins (transaction fund for trans fee)

    // this will error out if user gives an invalid address
    let validated_admin_address = deps.api.addr_validate(&msg.admin_address)?;

    // give config the validated address
    let config = Config {
        admin_address: validated_admin_address
    };

    // assert success
    CONFIG.save(deps.storage, &config)?;

    // Result<Response>
    Ok(Response::new().add_attribute("action", "instantiate"))
}

// write data
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll { question } => execute_create_poll(deps, &env, &info, question),
        ExecuteMsg::Vote {question, choice } => execute_vote(deps, &env, &info, question, choice),
    }

}


fn execute_vote(deps: DepsMut, _env: &Env, _info: &MessageInfo, question: String, choice: String)-> Result<Response,ContractError>{
    if !POLLS.has(deps.storage, question.clone()){
        // if poll doesnt have question that the person is answering, error out and tell them that poll doesnt exist
        return Err(ContractError::CustomError { val: "Poll does not exist".to_string() });
    }

    let mut poll = POLLS.load(deps.storage, question.clone())?;

    match choice.as_str() {
        "yes" => poll.yes_votes += 1,
        "no" => poll.no_votes += 1,
        _ => return Err(ContractError::CustomError { val: "choice doesnt exist".to_string() })
    }

    POLLS.save(deps.storage, question, &poll)?;
    Ok(Response::new().add_attribute("action", "vote"))

}

// function to just create a poll which starts with 0,0
fn execute_create_poll(deps: DepsMut, _env: &Env, _info: &MessageInfo, question: String ) -> Result<Response, ContractError>{
    
    if POLLS.has(deps.storage, question.clone()){
        // if it does, we want to error
        return Err(ContractError::CustomError {val: "key already taken".to_string() });
    }
    
    let poll = Poll {question: question.clone(), yes_votes: 0, no_votes: 0};
    POLLS.save(deps.storage, question, &poll)?;

    Ok(Response::new().add_attribute("action", "create_poll"))
}


// read data
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // read only, cant modify data with a query
    match msg {
        QueryMsg::GetPoll { question } => query_get_poll(deps, &_env, question),
        QueryMsg::GetConfig {} => to_binary(&CONFIG.load(deps.storage)?)
    }
}

fn query_get_poll(deps: Deps, _env: &Env, question: String) -> StdResult<Binary>{
    let poll = POLLS.may_load(deps.storage, question)?;
    to_binary(&GetPollResponse{poll})
}




#[cfg(test)]
mod tests {
    use cosmwasm_std::attr;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use crate::contract::{instantiate, execute, query};
    use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, GetPollResponse};
    use cosmwasm_std::{from_binary, Addr};
    use crate::state::{Poll, Config};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1",&[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string() //String::from("addr1")
        };

        let resp = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(resp.attributes, vec![
            attr("action", "instantiate")
        ]);

        let msg = QueryMsg::GetConfig {};
        let resp = query(deps.as_ref(), env, msg).unwrap();
        let config: Config = from_binary(&resp).unwrap();
        assert_eq!(config, Config {
            admin_address: Addr::unchecked("addr1")
        });
    }

    #[test]
    fn test_create_poll(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg{
            admin_address: "addr1".to_string()
        };

        // before you execute a contract you need to instantiate it
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        let msg = ExecuteMsg::CreatePoll {
            question: "Do you like grapes?".to_string()
        };

        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(resp.attributes, vec![
            attr("action","create_poll")
        ]);

        let msg = QueryMsg::GetPoll { question: "Do you like grapes?".to_string() };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let get_polls_response: GetPollResponse = from_binary(&resp).unwrap();
        assert_eq!(get_polls_response, GetPollResponse{ poll: 
            Some(
                Poll { 
                    question: "Do you like grapes?".to_string(), 
                    yes_votes: 0, 
                    no_votes: 0
                }
            )
        });

        let msg = ExecuteMsg::CreatePoll {
            question: "Do you like grapes?".to_string()
        };

        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    }


    #[test]
    fn test_vote(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg{
            admin_address: "addr1".to_string()
        };

        // before you execute a contract you need to instantiate it
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        // before you vote you must create a poll
        let msg = ExecuteMsg::CreatePoll {
            question: "Do you like grapes?".to_string()
        };

        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        // Vote on a poll that DNE
        let msg = ExecuteMsg::Vote { 
            question: "this poll doesnt exist".to_string(), 
            choice: "yes".to_string()
        };

        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        // Vote with an invalid choice
        let msg = ExecuteMsg::Vote {
            question: "Do you like grapes?".to_string(),
            choice: "abc".to_string()
        };

        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        // Vote yes
        let msg = ExecuteMsg::Vote {
            question: "Do you like grapes?".to_string(),
            choice: "yes".to_string()
        };

        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(resp.attributes, vec![
            attr("action","vote")
        ]);

        // query state to see if the response has been saved
        let msg = QueryMsg::GetPoll { question: "Do you like grapes?".to_string() };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let get_poll_resp: GetPollResponse = from_binary(&resp).unwrap();
        assert_eq!(get_poll_resp, GetPollResponse{ poll: 
            Some(
                Poll{
                    question: "Do you like grapes?".to_string(),
                    yes_votes: 1,
                    no_votes: 0
                }
            )
        });


    
        // Vote no
        let msg = ExecuteMsg::Vote {
            question: "Do you like grapes?".to_string(),
            choice: "no".to_string()
        };

        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    }

    
}
