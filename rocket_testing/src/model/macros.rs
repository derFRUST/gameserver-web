#[macro_export]
macro_rules! implEnumToSqlText {
    ( $t:ty ) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", String::from(self))
            }
        }

        impl<DB: diesel::backend::Backend> diesel::serialize::ToSql<diesel::sql_types::Text, DB>
            for $t
        where
            String: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
        {
            fn to_sql<W>(
                &self,
                out: &mut diesel::serialize::Output<W, DB>,
            ) -> diesel::serialize::Result
            where
                W: std::io::Write,
            {
                String::from(self).to_sql(out)
            }
        }

        impl<DB: diesel::backend::Backend> diesel::deserialize::FromSql<diesel::sql_types::Text, DB>
            for $t
        where
            String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
        {
            fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
                Ok(<$t>::try_from(String::from_sql(bytes)?)?)
            }
        }

        impl<DB: diesel::backend::Backend>
            diesel::deserialize::FromSqlRow<diesel::sql_types::Text, DB> for $t
        where
            String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
        {
            fn build_from_row<T: diesel::row::Row<DB>>(
                row: &mut T,
            ) -> diesel::deserialize::Result<Self> {
                diesel::deserialize::FromSql::from_sql(row.take())
            }
        }

        impl<DB: diesel::backend::Backend>
            diesel::deserialize::Queryable<diesel::sql_types::Text, DB> for $t
        where
            String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
        {
            type Row = Self;

            fn build(row: Self::Row) -> Self {
                row
            }
        }
    };
}
