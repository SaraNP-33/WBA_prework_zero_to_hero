use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Too many poll options")]
    TooManyOptions {},

    #[error("Poll not found with ID={poll_id:?}")]
    PollNotFound { poll_id: String },

    #[error("Poll with ID={poll_id:?} has no option \"{bad_option:?}\"")]
    PollOptionNotFound { poll_id: String, bad_option: String },
}
