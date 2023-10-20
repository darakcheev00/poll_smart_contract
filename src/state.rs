use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub admin_address: Addr, // wallet address

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Poll {
    pub question: String,
    pub yes_votes: u64,
    pub no_votes: u64,
}

pub const CONFIG: Item<Config> = Item::new("config"); // this is stored on chain

// string -> Poll
// "do you like spark ibc?"-> Poll{
//                                 question: "do you liek spark ibc?",
                            //     yes_votes: 100,
                            //     no_votes: 0,
                            // }
pub const POLLS: Map<String, Poll> = Map::new("polls"); // maps string to poll