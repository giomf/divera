pub mod schema;

use crate::{create_url, handle_response, send, Error, ACCESS_KEY_HEADER};
use schema::response;

const ENDPOINT_USERS: &str = "users";

pub fn users(access_token: &str) -> Result<Vec<response::User>, Error> {
    let url = create_url(ENDPOINT_USERS);
    let request = reqwest::blocking::Client::new()
        .get(url)
        .query(&[(ACCESS_KEY_HEADER, access_token)]);

    let response = send(request)?;
    let login = handle_response(response)?;
    Ok(login)
}
