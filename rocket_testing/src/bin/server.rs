#![feature(decl_macro, proc_macro_hygiene)]

use dotenv::dotenv;
use rocket::{http::Method, response::content, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

use rocket_testing::db::establish_connection;
use rocket_testing::graphql_frontend::{create_schema, Context, Schema};

use rocket_testing::gameserver::interactor::GameserverInteractor;

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

    let interactor = Box::leak(Box::new(GameserverInteractor::new(establish_connection())));

    rocket::ignite()
        .manage(Context {
            gameserver_control: interactor,
            gameserver_view: interactor,
        })
        .manage(create_schema())
        .mount("/", rocket::routes![graphiql, post_graphql_handler])
        .attach(cors)
        .launch();

    Ok(())
}
