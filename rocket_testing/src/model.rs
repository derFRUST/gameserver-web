use crate::db::ConnectionPool;
use diesel::prelude::*;
use juniper::{Executor, FieldResult, ID};
use juniper_from_schema::graphql_schema_from_file;

use crate::schema::servers;

graphql_schema_from_file!("../graphql/schema.graphql");

#[derive(Queryable)]
pub struct Game {
    id: i32,
    name: String,
    image: String, // TODO: Change to Option<String>?
}

// TODO: Use From instead of Into
impl Into<String> for ServerStatus {
    fn into(self) -> String {
        match self {
            ServerStatus::Stopped => String::from("stopped"),
            ServerStatus::Starting => String::from("starting"),
            ServerStatus::Started => String::from("started"),
            ServerStatus::Stopping => String::from("stopping"),
        }
    }
}

impl Into<ServerStatus> for String {
    fn into(self) -> ServerStatus {
        match &self[..] {
            "starting" => ServerStatus::Starting,
            "started" => ServerStatus::Started,
            "stopping" => ServerStatus::Stopping,
            _ => ServerStatus::Stopped,
        }
    }
}

#[derive(Identifiable, Queryable)]
pub struct Server {
    id: i32,
    name: String,
    game_id: i32,
    status: String,
}

pub struct ServerPayload {
    server: Server,
}

impl GameFields for Game {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<ID> {
        Ok(self.id.to_string().into())
    }

    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }

    fn field_image(&self, _: &Executor<'_, Context>) -> FieldResult<Option<String>> {
        Ok(Some(self.image.clone()))
    }
}

impl ServerFields for Server {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<ID> {
        Ok(self.id.to_string().into())
    }

    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }

    fn field_game(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, Game, juniper_from_schema::Walked>,
    ) -> FieldResult<Game> {
        executor.context().get_game(self.game_id.to_string().into())
    }

    fn field_status(&self, _: &Executor<'_, Context>) -> FieldResult<ServerStatus> {
        Ok(self.status.clone().into())
    }
}

impl ServerPayloadFields for ServerPayload {
    fn field_server(
        &self,
        _: &Executor<'_, Context>,
        _: &QueryTrail<'_, Server, juniper_from_schema::Walked>,
    ) -> FieldResult<&Server> {
        Ok(&self.server)
    }
}

pub struct Context {
    pub pool: ConnectionPool,
}

impl juniper::Context for Context {}

impl Context {
    pub fn get_game(&self, input_id: ID) -> FieldResult<Game> {
        use crate::schema::games::dsl::*;
        let input_id: i32 = input_id.to_string().parse()?;
        let connection = self.pool.get().unwrap();
        Ok(games.filter(id.eq(input_id)).first(&connection)?)
    }

    pub fn get_server(&self, input_id: ID) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;
        let input_id: i32 = input_id.to_string().parse()?;
        let connection = self.pool.get().unwrap();
        Ok(servers.filter(id.eq(input_id)).first(&connection)?)
    }

    pub fn update_server(&self, input: UpdateServerInput) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;
        let input_id: i32 = input.id.to_string().parse()?;
        let input_game_id: i32 = input.game_id.to_string().parse()?;
        let connection = self.pool.get().unwrap();
        diesel::update(servers.find(input_id))
            .set((name.eq(input.name), game_id.eq(input_game_id)))
            .execute(&connection)?;
        Ok(servers.filter(id.eq(input_id)).first(&connection)?)
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_games(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, Game, Walked>,
    ) -> FieldResult<Vec<Game>> {
        use crate::schema::games::dsl::*;
        let connection = executor.context().pool.get().unwrap();
        Ok(games.load(&connection)?)
    }

    fn field_servers(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, Server, Walked>,
    ) -> FieldResult<Vec<Server>> {
        use crate::schema::servers::dsl::*;
        let connection = executor.context().pool.get().unwrap();
        Ok(servers.load(&connection)?)
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_start_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, ServerPayload, Walked>,
        input: StartServerInput,
    ) -> FieldResult<ServerPayload> {
        let server = executor.context().get_server(input.id)?;
        println!("Current Status: {}", server.status);
        Ok(ServerPayload { server })
    }

    fn field_stop_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, ServerPayload, Walked>,
        input: StopServerInput,
    ) -> FieldResult<ServerPayload> {
        let server = executor.context().get_server(input.id)?;
        println!("Current Status: {}", server.status);
        Ok(ServerPayload { server })
    }

    fn field_update_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, ServerPayload, Walked>,
        input: UpdateServerInput,
    ) -> FieldResult<ServerPayload> {
        Ok(ServerPayload {
            server: executor.context().update_server(input)?,
        })
    }
}

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation)
}
