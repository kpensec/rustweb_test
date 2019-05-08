#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate juniper;
extern crate juniper_rocket;
extern crate serde;
extern crate serde_json;
extern crate redis;
extern crate futures;

mod graphql;
mod vagrant;
mod database;
mod ctx;
mod model;
mod routes;

use ctx::{Ctx};
use database::{Database};
use graphql::{build_schema};
use rocket_contrib::serve::{StaticFiles};

impl juniper::Context for Database {}

fn main() {
    let ctx = Ctx::new();
    let redis_routes = routes![routes::redis_get, routes::redis_set];
    let graphql_routes = routes![routes::graphiql, routes::get_graphql_handler, routes::post_graphql_handler];
    let vm_routes = routes![routes::vagrant_status, routes::vagrant_up, routes::vagrant_down];
    rocket::ignite()
        .manage(ctx)
        .manage(build_schema())
        .mount("/graphql", graphql_routes)
        .mount("/vm", vm_routes)
        .mount("/redis", redis_routes)
        .mount("/", StaticFiles::from("./www/build"))
        .launch();
}

