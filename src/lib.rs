pub mod v2;

use reqwest::{
    blocking::{RequestBuilder, Response},
    Error as ReqwestError, Url,
};
use serde::{de::DeserializeOwned, Deserialize};
use std::fmt::Debug;
use thiserror::Error as ThisError;

const BASE_URL: &str = "https://app.divera247.com/api/";
const ACCESS_KEY_HEADER: &str = "accesskey";

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Request error")]
    Request(#[from] ReqwestError),
}

#[derive(Clone, Debug, Deserialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

fn create_url(endpoint: &str) -> Url {
    Url::parse(BASE_URL).unwrap().join(endpoint).unwrap()
}

fn send(request: RequestBuilder) -> Result<Response, Error> {
    let response = request.send()?;
    log::debug!("Response headers: {:#?}", &response);
    Ok(response)
}

fn handle_response<T: DeserializeOwned + Debug>(response: Response) -> Result<T, Error> {
    let response: SuccessResponse<T> = response.json()?;
    log::debug!("Response body: {:#?}", response);
    Ok(response.data)
}
