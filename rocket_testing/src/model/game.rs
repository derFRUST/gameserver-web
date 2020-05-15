use juniper::{Executor, FieldResult, ID};

use super::graphql::{Context, GameFields};
use super::id::Id;

#[derive(Queryable)]
pub struct Game {
    id: Id,
    name: String,
    image: String, // TODO: Change to Option<String>?
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
