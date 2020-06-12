use std::fmt::Debug;
use std::io;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Integer;

#[derive(Debug, Clone, Eq, PartialEq, Hash, AsExpression, FromSqlRow)]
#[sql_type = "Integer"]
pub struct Id {
    id: String,
}

impl Id {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn to_string(&self) -> &String {
        &self.id
    }
}

impl<DB: Backend> ToSql<Integer, DB> for Id
where
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        self.id.to_string().parse::<i32>()?.to_sql(out)
    }
}

impl<DB: Backend> FromSql<Integer, DB> for Id
where
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        Ok(Self {
            id: i32::from_sql(bytes)?.to_string().into(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ServerStatus {
    Stopped,
    Starting,
    Started,
    Stopping,
}

implEnumString!(ServerStatus; [
    ServerStatus::Stopped => "STOPPED",
    ServerStatus::Starting => "STARTING",
    ServerStatus::Started => "STARTED",
    ServerStatus::Stopping => "STOPPING"
]);
implEnumToSqlText!(ServerStatus);

pub mod request {
    use super::*;

    pub trait GameserverCreate: Debug {
        fn name(&self) -> String;
        fn game(&self) -> Id;
    }

    pub trait GameserverUpdate: Debug {
        fn id(&self) -> Id;
        fn name(&self) -> String;
        fn game(&self) -> Id;
    }
}

pub mod response {
    use super::*;

    pub trait GameserverData: Debug {
        fn id(&self) -> &Id;
        fn name(&self) -> &String;
        fn game(&self) -> &Id;
        fn status(&self) -> &ServerStatus;
    }

    pub trait GameData: Debug {
        fn id(&self) -> &Id;
        fn name(&self) -> &String;
        fn image(&self) -> &String;
    }
}
