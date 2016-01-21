extern crate postgres;

use self::postgres::rows::Row;
use self::postgres::Connection;

pub struct Token {
    pub id: Option<i32>,
    pub api_key: Option<String>
}

impl Token {
    pub fn new() -> Token {
        Token {
            id: None,
            api_key: None
        }
    }

    pub fn hydrate(token: Option<Token>, alias: String, row: Row) -> Token {
        let mut token = match token {
            Some(token) => token,
            None => Token::new(),
        };

        token.id = Some(row.get(&((alias.clone() + "_id")[..])));
        token.api_key = Some(row.get(&((alias.clone() + "_token")[..])));

        return token;
    }

    pub fn find_by_token(db: &Connection, api_key: String) -> Option<Token> {
        let mut query = "
            SELECT
                api_key.id as api_key_id,
                api_key.token as api_key_token
            FROM geo_api_key api_key
            WHERE
                api_key.token = $1".to_string();

        let stmt = db.prepare(&query).unwrap();
        let row = stmt.query(&[&api_key]).unwrap();

        if row.len() == 1 {
            return Some(Token::hydrate(None, "api_key".to_owned(), row.get(0)));
        } else {
            return None;
        }
    }
}