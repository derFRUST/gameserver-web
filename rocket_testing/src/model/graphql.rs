use diesel::prelude::*;
use juniper::{Executor, FieldResult};
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

    pub fn get_server(&self, input_id: Id) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;
        let connection = self.pool.get().unwrap();
        Ok(servers.filter(id.eq(input_id)).first(&connection)?)
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
        let server = executor.context().get_server(input.id.into())?;
        println!("Current Status: {}", server.status);
        Ok(ServerPayload { server })
    }

    fn field_stop_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, ServerPayload, Walked>,
        input: StopServerInput,
    ) -> FieldResult<ServerPayload> {
        let server = executor.context().get_server(input.id.into())?;
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
