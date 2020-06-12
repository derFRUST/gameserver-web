use juniper::{Executor, FieldResult};

use super::graphql::{Context, QueryTrail, ServerPayloadFields};
use super::server::Server;

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
