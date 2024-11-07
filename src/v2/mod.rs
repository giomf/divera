#![allow(unused)]

pub mod schema;

use crate::{create_url, handle_response, send, Error, ACCESS_KEY_HEADER};
use schema::{request, response};

const ENDPOINT_LOGIN: &str = "v2/auth/login";
const ENDPOINT_PULL_ALL: &str = "v2/pull/all";
const ENDPOINT_REPORTS: &str = "v2/reporttypes/reports";
const ENDPOINT_REPORTTYPES: &str = "v2/reporttypes";

pub fn login(username: &str, password: &str) -> Result<response::Login, Error> {
    let url = create_url(ENDPOINT_LOGIN);
    let body = request::LoginRequest {
        login: request::Login {
            username: username.to_string(),
            password: password.to_string(),
            jwt: false,
        },
    };
    let request = reqwest::blocking::Client::new().post(url).json(&body);

    let response = send(request)?;
    let login = handle_response(response)?;
    Ok(login)
}

pub fn report_types(access_token: &str) -> Result<response::ReportTypes, Error> {
    let url = create_url(ENDPOINT_REPORTTYPES);
    let request = reqwest::blocking::Client::new()
        .get(url)
        .query(&[(ACCESS_KEY_HEADER, access_token)]);

    let response = send(request)?;
    Ok(handle_response(response)?)
}

pub fn reports(access_token: &str, report_type: i64) -> Result<response::Reports, Error> {
    let url = create_url(ENDPOINT_REPORTS);
    let request = reqwest::blocking::Client::new().get(url).query(&[
        (ACCESS_KEY_HEADER, access_token),
        ("id", &report_type.to_string()),
    ]);

    let response = send(request)?;
    Ok(handle_response(response)?)
}

pub fn pull_all(access_token: &str) -> Result<response::All, Error> {
    let url = create_url(ENDPOINT_PULL_ALL);
    let request = reqwest::blocking::Client::new()
        .get(url)
        .query(&[(ACCESS_KEY_HEADER, access_token)]);

    let response = send(request)?;
    Ok(handle_response(response)?)
}
