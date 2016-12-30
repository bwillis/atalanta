extern crate iron;
#[macro_use] extern crate juniper;
extern crate mount;
extern crate router;
extern crate staticfile;

use iron::prelude::*;
use juniper::iron_handlers::GraphQLHandler;

mod schema;
mod models;
use schema::QueryRoot;

use models::database::context_factory;

use mount::Mount;
use staticfile::Static;

use std::path::Path;

fn main() {
    let mut mount = Mount::new();
    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        QueryRoot,
        ()
    );

    mount
        .mount("/graphql", graphql_endpoint)
        .mount("/", Static::new(Path::new("target/public")));

    Iron::new(mount).http("0.0.0.0:3456").unwrap();
}
