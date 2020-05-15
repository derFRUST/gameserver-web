use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::prelude::*;
use diesel::row;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use std::convert::TryFrom;
use std::fmt;
use std::io;

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

impl fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl<DB: Backend> ToSql<Text, DB> for ServerStatus
where
    String: ToSql<Text, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        String::from(self).to_sql(out)
    }
}

impl<DB: Backend> FromSql<Text, DB> for ServerStatus
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        Ok(ServerStatus::try_from(String::from_sql(bytes)?)?)
    }
}

impl<DB: Backend> FromSqlRow<Text, DB> for ServerStatus
where
    String: FromSql<Text, DB>,
{
    fn build_from_row<T: row::Row<DB>>(row: &mut T) -> deserialize::Result<Self> {
        Self::from_sql(row.take())
    }
}

impl<DB: Backend> Queryable<Text, DB> for ServerStatus
where
    String: FromSql<Text, DB>,
{
    type Row = Self;

    fn build(row: Self::Row) -> Self {
        row
    }
}
