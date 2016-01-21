use model::request::Request;
use model::request::FilterCountry;
use postgres::rows::Row;
use postgres::Connection;
use std::sync::Mutex;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Country {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub iso: Option<String>
}

impl Country {
    pub fn new() -> Country {
        Country {
            id: None,
            name: None,
            iso: None,
        }
    }

    pub fn hydrate(country: Option<Country>, alias: String, row: Row) -> Country {
        let mut country = match country {
            Some(country) => country,
            None => Country::new(),
        };

        country.id = Some(row.get(&((alias.clone() + "_id")[..])));
        country.name = Some(row.get(&((alias.clone() + "_name")[..])));
        country.iso = Some(row.get(&((alias.clone() + "_iso")[..])));

        return country;
    }

    pub fn find_all(db: &Mutex<Connection>, request: Request<FilterCountry>) -> Vec<Country> {
        let db = &*db.lock().unwrap();
        let mut result: Vec<Country> = Vec::new();

        let mut query = "
            SELECT
                country.id as country_id,
                country.name as country_name,
                country.iso as country_iso
            FROM geo_country country".to_string();
        if request.filter.name.is_some() {
            query = query + " WHERE country.name = '" + &request.filter.name.unwrap() + "'";
        }

        let stmt = db.prepare(&query).unwrap();
        for row in &stmt.query(&[]).unwrap() {
            result.push(Country::hydrate(None, "country".to_owned(), row));
        }

        return result;
    }
}