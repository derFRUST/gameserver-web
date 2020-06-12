use juniper::{Executor, FieldResult, ID};

use super::graphql::{Context, GameFields};

use crate::gameserver::model::response::GameData;

pub type Game = Box<dyn GameData>;

impl GameFields for Game {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<ID> {
        Ok(ID::from(self.id()))
    }

    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(self.name())
    }

    fn field_image(&self, _: &Executor<'_, Context>) -> FieldResult<Option<String>> {
        Ok(Some(self.image().clone()))
    }
}
