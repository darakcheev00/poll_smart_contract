use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::Poll;

// how do we communicate with our contract

// fires instantiate route
// instantiation - create new instance of a contract
// the constructor
#[cw_serde]
pub struct InstantiateMsg {
    pub admin_address: String // why is this a string and not Addr? Because we need to validate that it's an address. (Addr doesnt validate)
}

// write routes (POST, PUT)
#[cw_serde]
pub enum ExecuteMsg {
    CreatePoll { //ExecuteMsg::CreatePoll {question: "do you love sparK?"}
        question: String,
    },
    Vote {
        question: String, // which question are we answering?
        choice: String // 'yes' or 'no'
    }
}

// read routes (GET)
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetPollResponse)]
    GetPoll {
        question: String,
    },
    #[returns(RegResponse)]
    GetConfig {}
}


#[cw_serde]
pub struct GetPollResponse {
    pub poll: Option<Poll>, // either null or a poll
}

#[cw_serde]
pub struct RegResponse {}