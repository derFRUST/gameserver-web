use std::convert::TryFrom;

use super::graphql::ServerStatus;

impl TryFrom<String> for ServerStatus {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match &s[..] {
            "STARTING" => Ok(ServerStatus::Starting),
            "STARTED" => Ok(ServerStatus::Started),
            "STOPPING" => Ok(ServerStatus::Stopping),
            "STOPPED" => Ok(ServerStatus::Stopped),
            _ => Err(format!("Invalid server status: {}", s)),
        }
    }
}

impl From<&ServerStatus> for String {
    fn from(status: &ServerStatus) -> Self {
        match status {
            ServerStatus::Stopped => String::from("STOPPED"),
            ServerStatus::Starting => String::from("STARTING"),
            ServerStatus::Started => String::from("STARTED"),
            ServerStatus::Stopping => String::from("STOPPING"),
        }
    }
}

implEnumToSqlText!(ServerStatus);
