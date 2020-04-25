use crate::model::*;

#[juniper::object(
    Context = Database,
)]
impl<'a> &'a dyn Game {
    fn name(&self) -> &str {
        self.name()
    }
    fn image(&self) -> &str {
        self.image()
    }
}

#[juniper::object(
    Context = Database,
)]
impl<'a> &'a dyn Server {
    fn name(&self) -> &str {
        self.name()
    }
    fn game(&self, ctx: &Database) -> Option<&dyn Game> {
        ctx.get_game(self.game())
    }
    fn status(&self) -> &ServerStatus {
        self.status()
    }
}

pub struct Query;

#[juniper::object(
    Context = Database,
)]
impl Query {
    fn games(database: &Database) -> Vec<&dyn Game> {
        database.get_games()
    }
    fn game(database: &Database, id: String) -> Option<&dyn Game> {
        database.get_game(&id)
    }
    fn servers(database: &Database) -> Vec<&dyn Server> {
        database.get_servers()
    }
}
