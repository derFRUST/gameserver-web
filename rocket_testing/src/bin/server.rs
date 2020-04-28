#![feature(decl_macro, proc_macro_hygiene)]

use dotenv::dotenv;
use rocket::{http::Method, response::content, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

use rocket_testing::db::establish_connection;
use rocket_testing::model::{create_schema, Context, Schema};

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() -> Result<(), Error> {
    dotenv().ok();

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .manage(Context {
            pool: establish_connection(),
        })
        .manage(create_schema())
        .mount("/", rocket::routes![graphiql, post_graphql_handler])
        .attach(cors)
        .launch();

    Ok(())
}
