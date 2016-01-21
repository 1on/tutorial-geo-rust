extern crate iron;
extern crate postgres;
extern crate rustc_serialize;

use application::Application;
use model;
use std::sync::Mutex;
use std::result::Result;
use iron::*;
use iron::mime::{Mime, TopLevel, SubLevel};
use self::postgres::Connection;
use self::rustc_serialize::json;

pub fn before_request_secured(request: &mut Request) -> Result<i32, String> {
    /*let url = request.url.clone().into_generic_url();
    let query = url.query_pairs().unwrap_or(vec![]);

    let mut api_key_checked = false;
    for (key, value) in query {
        if key == "apiKey" {
            let token = model::token::Token::find_by_token(&*connection.lock().unwrap(), value);
            api_key_checked = token.is_some();
            break;
        }
    }

    if !api_key_checked {
        panic!("No apiKey");
    } else {
        return Ok(0);
    }*/
    return Ok(0);
}

pub fn get_country_list_action(request: &mut Request) -> IronResult<Response> {
    /*// before_request(connection, request);

    let url = request.url.clone().into_generic_url();
    let query = url.query_pairs().unwrap_or(vec![]);
    let request_data = model::request::Request::<model::request::FilterCountry>::new(query);
    // match request_data.filter.validate() {
    //     Some(violations) => {return response_with_violations(violations)},
    //     None => {},
    // }


    let countries = model::country::Country::find_all(connection, request_data);

    let json_records;
    if let Ok(json) = json::encode(&countries) {
        json_records = Some(json);
    } else {
        return Ok(Response::with((status::InternalServerError, "couldn't convert records to JSON")));
    }*/

    let content_type = Mime(TopLevel::Application, SubLevel::Json, Vec::new());

    Ok(Response::with((content_type, status::Ok, "json_records.unwrap()")))
}

// fn response_with_violations(violations: Vec<(String, String)>) -> IronResult<Response> {

// }