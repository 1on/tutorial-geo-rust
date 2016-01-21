extern crate postgres;

use std::collections::HashMap;
use std::str::FromStr;
use self::postgres::{Connection, ConnectTarget, ConnectParams, SslMode, UserInfo};

pub struct Database {
    connect_params : ConnectParams,
    pub connection: Option<Connection>
}

impl Database {
    pub fn new(parameters: &HashMap<String, String>) -> Database {

        let connect_params = ConnectParams {
            target: ConnectTarget::Tcp(parameters.get("host").unwrap().to_owned()),
            port: Some(FromStr::from_str(parameters.get("port").unwrap()).unwrap()),
            user: Some(UserInfo {
                user: parameters.get("user").unwrap().clone(),
                password: Some(parameters.get("password").unwrap().clone())
            }),
            database: Some(parameters.get("database").unwrap().clone()),
            options: vec![]
        };

        Database {
            connect_params: connect_params,
            connection: None
        }
    }

    pub fn connect(&mut self) {
        println!("Connect");
        self.connection =
            Some(Connection::connect(
                self.connect_params.clone(),
                SslMode::None)
            .unwrap());
    }
}
