#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate juniper;
extern crate juniper_rocket;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod graphql;
mod vagrant;
mod database;
mod ctx;
mod model;
mod routes;

use ctx::Ctx;
use database::Database;
use graphql::{build_schema};

impl juniper::Context for Database {}


fn main() {
    let ctx = Ctx::new();
    rocket::ignite()
        .manage(ctx)
        .manage(build_schema())
        .mount(
            "/",
            routes![
                routes::graphiql,
                routes::get_graphql_handler,
                routes::post_graphql_handler,
            ]
        )
        .mount(
            "/vm",
            routes![
                routes::vagrant_status,
                routes::vagrant_up,
                routes::vagrant_down,
            ]
        )
        .launch();
}
