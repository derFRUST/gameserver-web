use juniper::{Executor, FieldResult, ID};

use crate::schema::servers;

use super::game::Game;
use super::graphql::{Context, QueryTrail, ServerFields, ServerStatus};
use super::id::Id;

#[derive(Identifiable, Queryable)]
pub struct Server {
    id: Id,
    name: String,
    game_id: Id,
    pub status: ServerStatus,
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
