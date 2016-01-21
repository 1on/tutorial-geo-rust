extern crate postgres;

use std::collections::HashMap;
use db::Database;
use ini::Ini;
use iron::*;
use std::sync::{Arc, Mutex};
use self::postgres::Connection;
use model;
use controller;
use std;
use std::fmt::{self, Debug};

pub struct Application {
    configuration: Configuration,
    database: Arc<Mutex<Connection>>
}

impl Application {
    pub fn new() -> Application {
        let configuration = Configuration::load();
        let mut database = Database::new(&configuration.database_parameters);
        database.connect();
        let mut database = database.connection.unwrap();
        let mut database = Arc::new(Mutex::new(database));

        Application {
            configuration: configuration,
            database: database
        }
    }
}

#[derive(Debug)]
struct StringError(String);

impl std::error::Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl BeforeMiddleware for Application {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        let connection = &*self.database;

        let url = request.url.clone().into_generic_url();
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
            Err(IronError::new(StringError("No apiKey".to_string()), (
                status::BadRequest,
                "qwerqw"
            )))
        } else {
            Ok(())
        }
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        Err(err)
    }
}

struct Configuration {
    general_parameters: HashMap<String, String>,
    database_parameters: HashMap<String, String>
}

impl Configuration {
    fn load() -> Configuration {
        let config = Ini::load_from_file("config.ini").unwrap();

        let mut general_parameters = HashMap::new();
        let params = config.section(None::<String>);
        if params != None {
            for (key, value) in params.unwrap() {
                general_parameters.insert(key.to_owned(), value.to_owned());
            }
        }

        let params = config.section(Some("Database")).unwrap();
        let mut database_parameters = HashMap::new();
        for (key, value) in params {
            database_parameters.insert(key.to_owned(), value.to_owned());
        }

        Configuration {
            general_parameters: general_parameters,
            database_parameters: database_parameters
        }
    }
}