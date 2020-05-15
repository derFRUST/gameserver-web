use crate::db::ConnectionPool;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::prelude::*;
use diesel::row;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use juniper::{Executor, FieldResult, ID};
use juniper_from_schema::graphql_schema_from_file;
use std::convert::TryFrom;
use std::fmt;
use std::io;

use crate::types;

use crate::schema::servers;

graphql_schema_from_file!("../graphql/schema.graphql");

#[derive(Queryable)]
pub struct Game {
    id: types::Id,
    name: String,
    image: String, // TODO: Change to Option<String>?
}

impl TryFrom<String> for ServerStatus {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match &s[..] {
            "starting" => Ok(ServerStatus::Starting),
            "started" => Ok(ServerStatus::Started),
            "stopping" => Ok(ServerStatus::Stopping),
            "stopped" => Ok(ServerStatus::Stopped),
            _ => Err(format!("Invalid server status: {}", s)),
        }
    }
}

impl From<&ServerStatus> for String {
    fn from(status: &ServerStatus) -> Self {
        match status {
            ServerStatus::Stopped => String::from("stopped"),
            ServerStatus::Starting => String::from("starting"),
            ServerStatus::Started => String::from("started"),
            ServerStatus::Stopping => String::from("stopping"),
        }
    }
}

impl fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl<DB: Backend> ToSql<Text, DB> for ServerStatus
where
    String: ToSql<Text, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        String::from(self).to_sql(out)
    }
}

impl<DB: Backend> FromSql<Text, DB> for ServerStatus
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        Ok(ServerStatus::try_from(String::from_sql(bytes)?)?)
    }
}

impl<DB: Backend> FromSqlRow<Text, DB> for ServerStatus
where
    String: FromSql<Text, DB>,
{
    fn build_from_row<T: row::Row<DB>>(row: &mut T) -> deserialize::Result<Self> {
        Self::from_sql(row.take())
    }
}

impl<DB: Backend> Queryable<Text, DB> for ServerStatus
where
    String: FromSql<Text, DB>,
{
    type Row = Self;

    fn build(row: Self::Row) -> Self {
        row
    }
}

#[derive(Identifiable, Queryable)]
pub struct Server {
    id: types::Id,
    name: String,
    game_id: types::Id,
    status: ServerStatus,
}

pub struct ServerPayload {
    server: Server,
}

impl GameFields for Game {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<&ID> {
        Ok(self.id.as_ref())
    }

    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }

    fn field_image(&self, _: &Executor<'_, Context>) -> FieldResult<Option<String>> {
        Ok(Some(self.image.clone()))
    }
}

impl ServerFields for Server {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<&ID> {
        Ok(self.id.as_ref())
    }

    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }

    fn field_game(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, Game, juniper_from_schema::Walked>,
    ) -> FieldResult<Game> {
        executor.context().get_game(&self.game_id)
    }

    fn field_status(&self, _: &Executor<'_, Context>) -> FieldResult<&ServerStatus> {
        Ok(&self.status)
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
    pub fn get_game(&self, input_id: &types::Id) -> FieldResult<Game> {
        use crate::schema::games::dsl::*;
        let connection = self.pool.get().unwrap();
        Ok(games.filter(id.eq(input_id)).first(&connection)?)
    }

    pub fn get_server(&self, input_id: types::Id) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;
        let connection = self.pool.get().unwrap();
        Ok(servers.filter(id.eq(input_id)).first(&connection)?)
    }

    pub fn update_server(&self, input: UpdateServerInput) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;
        let input_id: types::Id = input.id.into();
        let input_game_id: types::Id = input.game_id.into();
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
