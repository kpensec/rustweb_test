use ctx::Ctx;
use graphql::Schema;
use rocket::State;
use rocket::response::{content, Redirect};
use vagrant;
use vagrant::{MachinesStatus};
use rocket_contrib::json::Json;

#[get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(context: State<Ctx>, request: juniper_rocket::GraphQLRequest, schema: State<Schema>) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(context: State<Ctx>, request: juniper_rocket::GraphQLRequest, schema: State<Schema>) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

// vm routes
#[get("/status")]
pub fn vagrant_status() -> Json<MachinesStatus> {
    Json(vagrant::get_status())
}

#[get("/up/<name>")]
pub fn vagrant_up(name: String) -> String {
    vagrant::power_on(name)
}

#[get("/down/<name>")]
pub fn vagrant_down(name: String) -> String {
    vagrant::power_off(name)
}


#[get("/get")]
pub fn redis_get(ctx: State<Ctx>) -> String {
   match ctx.foo() {
       Ok(v) => v.to_string(),
       Err(e) => e.to_string()
   }
}


#[get("/set/<v>")]
pub fn redis_set(ctx: State<Ctx>, v: i32) -> String {
    match ctx.bar(v) {
        Ok(()) => "Operation success",
        Err(_) => "Error..."
    }.to_string()
}

