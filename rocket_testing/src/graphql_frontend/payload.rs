use juniper::{Executor, FieldResult, ID};

use super::graphql::{Context, DeleteServerPayloadFields, QueryTrail, ServerPayloadFields};
use super::server::Server;

pub struct DeleteServerPayload {
    pub id: ID,
}

impl DeleteServerPayloadFields for DeleteServerPayload {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<&ID> {
        Ok(&self.id)
    }
}

pub struct ServerPayload {
    pub server: Server,
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
