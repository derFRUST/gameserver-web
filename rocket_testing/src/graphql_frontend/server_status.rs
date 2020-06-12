use super::graphql::ServerStatus;
use crate::gameserver::model;

impl From<&model::ServerStatus> for ServerStatus {
    fn from(status: &model::ServerStatus) -> Self {
        match status {
            model::ServerStatus::Stopped => ServerStatus::Stopped,
            model::ServerStatus::Starting => ServerStatus::Starting,
            model::ServerStatus::Started => ServerStatus::Started,
            model::ServerStatus::Stopping => ServerStatus::Stopping,
        }
    }
}
