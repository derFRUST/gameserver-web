use crate::context::Database;

pub struct Query;

#[juniper::object(
    Context = Database,
)]
impl Query {
    #[graphql()]
    fn test(database: &Database) -> &str {
        "Hello World"
    }
}
