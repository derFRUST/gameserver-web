#[macro_export]
/// Automatically implement conversion functions to and from strings.
macro_rules! implEnumString {
    ( $t:ty; [ $( $e:path => $s:expr ),* ]) => {
        // borrowing version
        impl From<&$t> for String {
            fn from(item: &$t) -> Self {
                match item {
                    $(
                        $e => String::from($s),
                    )*
                }
            }
        }

        // moving version
        impl From<$t> for String {
            fn from(item: $t) -> Self {
                match item {
                    $(
                        $e => String::from($s),
                    )*
                }
            }
        }

        // slice version
        impl std::convert::TryFrom<&str> for $t {
            type Error = String;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                match s {
                    $(
                        $s => Ok($e),
                    )*
                    _ => Err(format!("Invalid value {} for type {}!", s, stringify!($t))),
                }
            }
        }

        // borrowing version
        impl std::convert::TryFrom<&String> for $t {
            type Error = String;

            fn try_from(s: &String) -> Result<Self, Self::Error> {
                Self::try_from(&s[..])
            }
        }

        // moving version
        impl std::convert::TryFrom<String> for $t {
            type Error = String;

            fn try_from(s: String) -> Result<Self, Self::Error> {
                Self::try_from(&s)

            }
        }
    };
}

#[macro_export]
macro_rules! implEnumToSqlText {
    ( $t:ty ) => {
        use std::convert::TryFrom;

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

#[cfg(test)]
mod tests {
    mod stringify_enum {
        use std::convert::TryFrom;

        #[derive(Debug, PartialEq, Eq)]
        enum MyTestEnum {
            ElementA,
            ElementB,
        }

        implEnumString!(MyTestEnum; [MyTestEnum::ElementA => "A", MyTestEnum::ElementB => "B"]);

        #[test]
        fn test_to_string() {
            // borrowed version
            let element = MyTestEnum::ElementA;
            let string = String::from(&element);
            assert_eq!(string, "A");

            // owned version
            assert_eq!(String::from(MyTestEnum::ElementB), "B");
        }

        #[test]
        fn test_from_string() -> Result<(), String> {
            // borrowed version
            let string = "A".to_owned();
            let element = MyTestEnum::try_from(&string);
            assert_eq!(element?, MyTestEnum::ElementA);

            // owned version
            assert_eq!(MyTestEnum::try_from("B".to_owned())?, MyTestEnum::ElementB);

            // slice version
            assert_eq!(MyTestEnum::try_from("B")?, MyTestEnum::ElementB);

            // invalid value
            let result = MyTestEnum::try_from("C");
            assert!(result.is_err());
            assert_eq!(
                String::from(result.unwrap_err()),
                "Invalid value C for type MyTestEnum!"
            );

            Ok(())
        }
    }
}
