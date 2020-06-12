use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

use super::game::Game;
use super::payload::ServerPayload;

use super::server::Server;

use crate::gameserver::control::GameserverControl;
use crate::gameserver::model;
use crate::gameserver::model::request::{GameserverCreate, GameserverUpdate};
use crate::gameserver::view::GameserverView;

graphql_schema_from_file!("../graphql/schema.graphql");

pub struct Context {
    pub gameserver_view: &'static dyn GameserverView,
    pub gameserver_control: &'static dyn GameserverControl,
}

impl juniper::Context for Context {}

pub struct Query;

impl QueryFields for Query {
    fn field_games(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, Game, Walked>,
    ) -> FieldResult<Vec<Game>> {
        let context = executor.context();
        Ok(context.gameserver_view.game_list())
    }

    fn field_servers(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, Server, Walked>,
        input_id: Option<juniper::ID>,
    ) -> FieldResult<Vec<Server>> {
        let context = executor.context();
        Ok(match input_id {
            Some(input_id) => {
                let input_id = crate::gameserver::model::Id::new(input_id.to_string());
                vec![context.gameserver_view.server_data(&input_id)]
            }
            None => context.gameserver_view.server_list(),
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
        let context = executor.context();
        let id = context.gameserver_control.create_server(&input);

        Ok(ServerPayload {
            server: context.gameserver_view.server_data(&id),
        })
    }

    fn field_start_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, ServerPayload, Walked>,
        input: StartServerInput,
    ) -> FieldResult<ServerPayload> {
        let context = executor.context();
        let id = input.id.into();
        context.gameserver_control.start_server(&id); // TODO: add failure
        Ok(ServerPayload {
            server: context.gameserver_view.server_data(&id),
        })
    }

    fn field_stop_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, ServerPayload, Walked>,
        input: StopServerInput,
    ) -> FieldResult<ServerPayload> {
        let context = executor.context();
        let id = input.id.into();
        context.gameserver_control.stop_server(&id); // TODO: add failure
        Ok(ServerPayload {
            server: context.gameserver_view.server_data(&id),
        })
    }

    fn field_update_server(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, ServerPayload, Walked>,
        input: UpdateServerInput,
    ) -> FieldResult<ServerPayload> {
        let context = executor.context();
        context.gameserver_control.update_server(&input); // TODO: add failure
        Ok(ServerPayload {
            server: context.gameserver_view.server_data(&input.id.into()),
        })
    }
}

impl GameserverUpdate for UpdateServerInput {
    fn id(&self) -> model::Id {
        (&self.id).into()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn game(&self) -> model::Id {
        (&self.game_id).into()
    }
}

impl GameserverCreate for CreateServerInput {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn game(&self) -> model::Id {
        (&self.game_id).into()
    }
}

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation)
}
