use crate::db::ConnectionPool;
use diesel::prelude::*;
use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

use crate::schema::servers;

graphql_schema_from_file!("../graphql/schema.graphql");

#[derive(Queryable)]
pub struct Game {
    id: i32,
    name: String,
    image: String,
}

#[derive(Identifiable, Queryable)]
pub struct Server {
    id: i32,
    name: String,
    game_id: i32,
    status: String,
}

impl GameFields for Game {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.id)
    }

    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }

    fn field_image(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.image)
    }
}

impl ServerFields for Server {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.id)
    }

    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }

    fn field_game(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, Game, juniper_from_schema::Walked>,
    ) -> FieldResult<Game> {
        executor.context().get_game(self.game_id)
    }

    fn field_status(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.status)
    }
}

pub struct Context {
    pub pool: ConnectionPool,
}

impl juniper::Context for Context {}

impl Context {
    pub fn get_game(&self, game_id: i32) -> FieldResult<Game> {
        use crate::schema::games::dsl::*;
        let connection = self.pool.get().unwrap();
        Ok(games.filter(id.eq(game_id)).first(&connection)?)
    }

    pub fn get_server(&self, server_id: i32) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;
        let connection = self.pool.get().unwrap();
        Ok(servers.filter(id.eq(server_id)).first(&connection)?)
    }

    pub fn update_server(&self, server_update: ServerUpdate) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;
        let connection = self.pool.get().unwrap();
        diesel::update(servers.find(server_update.id))
            .set((
                name.eq(server_update.name),
                game_id.eq(server_update.game_id),
            ))
            .execute(&connection)?;
        Ok(servers.filter(id.eq(server_update.id)).first(&connection)?)
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
    fn field_start_stop_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, Server, Walked>,
        server_id: i32,
    ) -> FieldResult<Server> {
        let server = executor.context().get_server(server_id)?;
        println!("Current Status: {}", server.status);
        Ok(server)
    }

    fn field_update_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, Server, Walked>,
        server_update: ServerUpdate,
    ) -> FieldResult<Server> {
        executor.context().update_server(server_update)
    }
}

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation)
}
