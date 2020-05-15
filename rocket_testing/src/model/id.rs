use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use std::hash::{Hash, Hasher};
use std::io;

#[derive(Debug, Clone, PartialEq, Eq, AsExpression, FromSqlRow)]
#[sql_type = "Integer"]
pub struct Id {
    id: juniper::ID,
}

impl AsRef<juniper::ID> for Id {
    fn as_ref(&self) -> &juniper::ID {
        &self.id
    }
}

impl From<juniper::ID> for Id {
    fn from(id: juniper::ID) -> Self {
        Id { id }
    }
}

impl Hash for Id {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
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
        Ok(Id {
            id: i32::from_sql(bytes)?.to_string().into(),
        })
    }
}
