use std::convert::TryFrom;

use super::graphql::ServerStatus;

implEnumString!(ServerStatus; [
    ServerStatus::Stopped => "STOPPED",
    ServerStatus::Starting => "STARTING",
    ServerStatus::Started => "STARTED",
    ServerStatus::Stopping => "STOPPING"
]);
implEnumToSqlText!(ServerStatus);
