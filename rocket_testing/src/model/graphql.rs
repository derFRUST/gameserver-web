use diesel::prelude::*;
use juniper::{graphql_value, Executor, FieldError, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

use crate::db::ConnectionPool;

use super::game::Game;
use super::id::Id;
use super::payload::ServerPayload;
use super::server::Server;

graphql_schema_from_file!("../graphql/schema.graphql");

pub struct Context {
    pub pool: ConnectionPool,
}

impl juniper::Context for Context {}

impl Context {
    pub fn get_game(&self, input_id: &Id) -> FieldResult<Game> {
        use crate::schema::games::dsl::*;
        let connection = self.pool.get().unwrap();
        Ok(games.filter(id.eq(input_id)).first(&connection)?)
    }

    pub fn get_server(&self, input_id: &Id) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;
        let connection = self.pool.get().unwrap();
        Ok(servers.filter(id.eq(input_id)).first(&connection)?)
    }

    pub fn start_server(&self, input_id: &Id) -> FieldResult<Server> {
        let mut server = self.get_server(&input_id)?;

        println!("Current Status: {}", server.status);
        if server.status == ServerStatus::Stopped {
            use crate::schema::servers::dsl::*;
            let connection = self.pool.get().unwrap();
            server.status = ServerStatus::Starting;
            diesel::update(servers.find(&input_id))
                .set(status.eq(server.status.to_string()))
                .execute(&connection)?;

            // TODO: actually start server
        }
        println!("New Status: {}", server.status);

        match server.status {
            ServerStatus::Stopping => Err(FieldError::new(
                "Unexpected server status: Stopping",
                graphql_value!({ "type": "STATUS_UNEXPECTED" }),
            )),
            _ => Ok(server),
        }
    }

    pub fn stop_server(&self, input_id: &Id) -> FieldResult<Server> {
        let mut server = self.get_server(&input_id)?;

        println!("Current Status: {}", server.status);
        if server.status == ServerStatus::Started || server.status == ServerStatus::Starting {
            use crate::schema::servers::dsl::*;
            let connection = self.pool.get().unwrap();
            server.status = ServerStatus::Stopped; // TODO: set to Stopping when stopping is implemented
            diesel::update(servers.find(&input_id))
                .set(status.eq(server.status.to_string()))
                .execute(&connection)?;

            // TODO: actually stop server
        }
        println!("New Status: {}", server.status);
        Ok(server)
    }

    pub fn update_server(&self, input: UpdateServerInput) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;
        let input_id: Id = input.id.into();
        let input_game_id: Id = input.game_id.into();
        let connection = self.pool.get().unwrap();
        diesel::update(servers.find(&input_id))
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
        input_id: Option<juniper::ID>,
    ) -> FieldResult<Vec<Server>> {
        use crate::schema::servers::dsl::*;
        let connection = executor.context().pool.get().unwrap();
        Ok(match input_id {
            Some(input_id) => servers
                .filter(id.eq(Id::from(input_id)))
                .load(&connection)?,
            None => servers.load(&connection)?,
        })
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_create_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, ServerPayload, Walked>,
        input: CreateServerInput,
    ) -> FieldResult<ServerPayload> {
        use crate::schema::servers::dsl::*;
        let connection = executor.context().pool.get().unwrap();
        let result = diesel::insert_into(servers)
            .values((
                name.eq(input.name),
                game_id.eq(Id::from(input.game_id)),
                status.eq(ServerStatus::Stopped.to_string()),
            ))
            .execute(&connection)?;
        println!("{:?}", result);

        // TODO: alternative idea to using auto increment IDs:
        // Use random GUID for identification instead.
        Ok(ServerPayload {
            server: servers
                .filter(id.eq(Id::from(juniper::ID::from("1".to_string())))) // FIXME: find correct server
                .first(&connection)?,
        })
    }

    fn field_start_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, ServerPayload, Walked>,
        input: StartServerInput,
    ) -> FieldResult<ServerPayload> {
        Ok(ServerPayload {
            server: executor.context().start_server(&input.id.into())?,
        })
    }

    fn field_stop_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, ServerPayload, Walked>,
        input: StopServerInput,
    ) -> FieldResult<ServerPayload> {
        Ok(ServerPayload {
            server: executor.context().stop_server(&input.id.into())?,
        })
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
