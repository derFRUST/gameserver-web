use juniper::{Executor, FieldResult, ID};

use super::game::Game;
use super::graphql::{Context, QueryTrail, ServerFields, ServerStatus};

use crate::gameserver::model::response::GameserverData;

pub type Server = Box<dyn GameserverData>;

impl ServerFields for Server {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<ID> {
        Ok(ID::from(self.id()))
    }

    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(self.name())
    }

    fn field_game(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, Game, juniper_from_schema::Walked>,
    ) -> FieldResult<Game> {
        let context = executor.context();
        Ok(context.gameserver_view.game_data(&self.game()))
    }

    fn field_status(&self, _: &Executor<'_, Context>) -> FieldResult<ServerStatus> {
        Ok(self.status().into())
    }
}
