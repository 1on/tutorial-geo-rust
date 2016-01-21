extern crate ini;
extern crate iron;
extern crate router;
extern crate mount;
extern crate rustc_serialize;
extern crate postgres;

mod db;
mod model;
mod controller;
mod application;

use application::Application;
use db::Database;
use std::sync::{Arc, Mutex};
use iron::*;

fn main() {
    println!("Hello, world!");

    let mut application = Application::new();

    Iron::new(init_routing(application)).http("localhost:3000").unwrap();
}

fn init_routing(application: Application) -> mount::Mount {

    let mut router = router::Router::new();
    {
        router.get("countries",
            move |request: &mut Request|
            controller::get_country_list_action(request));
    }

    let mut chain_secure = Chain::new(router);
    chain_secure.link_before(application);

    let mut root = mount::Mount::new();
    root.mount("/api/", chain_secure);

    return root;
}