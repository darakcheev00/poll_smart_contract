pub mod contract; // all logic
mod error; // where you define errors for contract, 404 not found can be defined
pub mod helpers; // test helpers
pub mod msg; // define msg endpoints (messages make up transactions)
pub mod state; // a state that you may store
// state is where you store data

// entry point for all the modules

// define all the files hereyyyy

pub use crate::error::ContractError;
