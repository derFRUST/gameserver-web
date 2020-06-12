use diesel::prelude::*;

use crate::db::ConnectionPool;
use crate::schema::games;
use crate::schema::servers;

use super::control::GameserverControl;
use super::model::request::{GameserverCreate, GameserverUpdate};
use super::model::response::{GameData, GameserverData};
use super::model::{Id, ServerStatus};
use super::view::GameserverView;

pub struct GameserverInteractor {
    pool: ConnectionPool,
}

impl GameserverInteractor {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "servers"]
pub struct Gameserver {
    id: Id,
    name: String,
    game_id: Id,
    pub status: ServerStatus,
}

impl GameserverData for Gameserver {
    fn id(&self) -> &Id {
        &self.id
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn game(&self) -> &Id {
        &self.game_id
    }
    fn status(&self) -> &ServerStatus {
        &self.status
    }
}
#[derive(Debug, Identifiable, Queryable)]
struct Game {
    id: Id,
    name: String,
    image: String,
}

impl GameData for Game {
    fn id(&self) -> &Id {
        &self.id
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn image(&self) -> &String {
        &self.image
    }
}

impl GameserverControl for GameserverInteractor {
    fn create_server(&self, create: &dyn GameserverCreate) -> Id {
        use crate::schema::servers::dsl::*;
        let connection = self.pool.get().unwrap();
        diesel::insert_into(servers)
            .values((
                name.eq(create.name()),
                game_id.eq(Id::from(create.game())),
                status.eq(ServerStatus::Stopped.to_string()),
            ))
            .execute(&connection)
            .unwrap();

        // TODO: alternative idea to using auto increment IDs:
        // Use random GUID for identification instead.
        Id::new("1".to_string())
    }

    fn start_server(&self, input_id: &Id) {
        let server = self.server_data(input_id);

        if server.status() == &ServerStatus::Stopped {
            use crate::schema::servers::dsl::*;
            let connection = self.pool.get().unwrap();
            diesel::update(servers.find(&input_id))
                .set(status.eq(ServerStatus::Starting.to_string()))
                .execute(&connection)
                .unwrap(); // TODO: remove unwrap

            // TODO: actually start server
        }

        // TODO: can be reused for Result<>
        // match server.status {
        //     ServerStatus::Stopping => Err(FieldError::new(
        //         "Unexpected server status: Stopping",
        //         graphql_value!({ "type": "STATUS_UNEXPECTED" }),
        //     )),
        //     _ => Ok(server),
        // }
    }

    fn stop_server(&self, input_id: &Id) {
        let server = self.server_data(input_id);

        if server.status() == &ServerStatus::Started || server.status() == &ServerStatus::Starting {
            use crate::schema::servers::dsl::*;
            let connection = self.pool.get().unwrap();
            diesel::update(servers.find(&input_id))
                .set(status.eq(ServerStatus::Stopped.to_string())) // TODO: set to Stopping when stopping is implemented
                .execute(&connection)
                .unwrap(); // TODO: remove unwrap

            // TODO: actually stop server
        }
    }

    fn update_server(&self, update: &dyn GameserverUpdate) {
        use crate::schema::servers::dsl::*;
        let connection = self.pool.get().unwrap();
        diesel::update(servers.find(&update.id()))
            .set((name.eq(update.name()), game_id.eq(update.game())))
            .execute(&connection)
            .unwrap();
    }
}

impl GameserverView for GameserverInteractor {
    fn server_data(&self, input_id: &Id) -> Box<dyn GameserverData> {
        use crate::schema::servers::dsl::*;
        let connection = self.pool.get().unwrap();
        let server: Gameserver = servers.filter(id.eq(input_id)).first(&connection).unwrap(); // TODO: remove unwrap
        Box::new(server)
    }

    fn server_list(&self) -> Vec<Box<dyn GameserverData>> {
        use crate::schema::servers::dsl::*;
        let connection = self.pool.get().unwrap();
        let server_list: Vec<Gameserver> = servers.load(&connection).unwrap(); // TODO: remove unwrap

        server_list
            .into_iter()
            .map(|s| Box::new(s) as Box<dyn GameserverData>)
            .collect()
    }

    fn game_data(&self, input_id: &Id) -> Box<dyn GameData> {
        use crate::schema::games::dsl::*;
        let connection = self.pool.get().unwrap();
        let game: Game = games.filter(id.eq(input_id)).first(&connection).unwrap(); // TODO: remove unwrap
        Box::new(game)
    }

    fn game_list(&self) -> Vec<Box<dyn GameData>> {
        use crate::schema::games::dsl::*;
        let connection = self.pool.get().unwrap();
        let game_list: Vec<Game> = games.load(&connection).unwrap(); // TODO: remove unwrap

        game_list
            .into_iter()
            .map(|s| Box::new(s) as Box<dyn GameData>)
            .collect::<Vec<_>>()
    }
}
