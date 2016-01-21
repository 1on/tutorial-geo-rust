pub struct Request<T: Filter> {
    pub filter: T
}

impl<T: Filter> Request<T> {
    pub fn new(parameters: Vec<(String, String)>) -> Request<T> {
        let filter = T::new(parameters);

        Request {
            filter: filter
        }
    }
}

pub struct FilterCountry {
    pub name: Option<String>,
    pub iso: Option<String>,
    pub iso3: Option<String>,
    pub fips: Option<String>,
}

pub trait Filter {
    fn new(parameters: Vec<(String, String)>) -> Self;
    fn validate(&self) -> Option<Vec<(String, String)>>;
}

impl Filter for FilterCountry {
    fn new(parameters: Vec<(String, String)>) -> FilterCountry {
        let mut filter = FilterCountry {
            name: None,
            iso: None,
            iso3: None,
            fips: None,
        };

        for (key, value) in parameters {
            match &*key {
                "name"  => {filter.name = Some(value.clone())},
                "iso"   => {filter.iso = Some(value.clone())},
                "iso3"  => {filter.iso3 = Some(value.clone())},
                "fips"  => {filter.fips = Some(value.clone())},
                _ => {continue;}
            }
        }

        return filter;
    }

    fn validate(&self) -> Option<Vec<(String, String)>> {
        let mut violations: Vec<(String, String)> = Vec::new();

        match self.iso.as_ref() {
            Some(iso) => {
                if iso.len() != 2 {
                    violations.push(("name".to_owned(), "Len must be = 2".to_owned()));
                }
            },
            None => {}
        }

        if violations.len() > 0 {
            return Some(violations);
        } else {
            return None;
        }
    }
}