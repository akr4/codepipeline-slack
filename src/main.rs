mod handler;
mod slack;

use failure::{Compat, Error};
use lambda_runtime::{lambda, Context};
use serde_derive::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(Deserialize, Debug)]
pub struct CloudWatchEvent {
    pub detail: CodePipelineEvent,
}

#[derive(Deserialize, Debug)]
pub struct CodePipelineEvent {
    pub pipeline: String,
    #[serde(rename = "execution-id")]
    pub execution_id: String,
    pub state: String,
}

#[derive(Serialize)]
pub struct Output {
    pub message: String,
}

fn main() -> Result<(), Box<dyn StdError>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(my_handler_wrapper);
    Ok(())
}

fn my_handler_wrapper(e: CloudWatchEvent, c: Context) -> Result<Output, Compat<Error>> {
    handler::my_handler(&e, &c).map_err(|e| e.compat())
}
