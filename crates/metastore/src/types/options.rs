use super::{FromOptionalField, ProtoConvError};
use crate::proto::arrow;
use crate::proto::options;
use datafusion::arrow::datatypes::DataType;
use proptest_derive::Arbitrary;
use std::fmt;

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct InternalColumnDefinition {
    pub name: String,
    pub nullable: bool,
    // TODO: change proptest strategy to select random DataType
    #[proptest(value("DataType::Utf8"))]
    pub arrow_type: DataType,
}

impl InternalColumnDefinition {
    /// Create a vec of column definitions.
    ///
    /// Tuples are in the form of:
    /// (name, datatype, nullable)
    pub fn from_tuples<C, N>(cols: C) -> Vec<InternalColumnDefinition>
    where
        C: IntoIterator<Item = (N, DataType, bool)>,
        N: Into<String>,
    {
        cols.into_iter()
            .map(|(name, arrow_type, nullable)| InternalColumnDefinition {
                name: name.into(),
                nullable,
                arrow_type,
            })
            .collect()
    }
}

impl TryFrom<options::InternalColumnDefinition> for InternalColumnDefinition {
    type Error = ProtoConvError;
    fn try_from(value: options::InternalColumnDefinition) -> Result<Self, Self::Error> {
        let arrow_type: DataType = value.arrow_type.as_ref().required("arrow_type")?;
        Ok(InternalColumnDefinition {
            name: value.name,
            nullable: value.nullable,
            arrow_type,
        })
    }
}

// TODO: Try to make this just `From`. Would require some additional conversions
// for the arrow types.
impl TryFrom<InternalColumnDefinition> for options::InternalColumnDefinition {
    type Error = ProtoConvError;
    fn try_from(value: InternalColumnDefinition) -> Result<Self, Self::Error> {
        let arrow_type = arrow::ArrowType::try_from(&value.arrow_type)?;
        Ok(options::InternalColumnDefinition {
            name: value.name,
            nullable: value.nullable,
            arrow_type: Some(arrow_type),
        })
    }
}

// Database options

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub enum DatabaseOptions {
    Internal(DatabaseOptionsInternal),
    Debug(DatabaseOptionsDebug),
    Postgres(DatabaseOptionsPostgres),
    BigQuery(DatabaseOptionsBigQuery),
    Mysql(DatabaseOptionsMysql),
    Mongo(DatabaseOptionsMongo),
    Snowflake(DatabaseOptionsSnowflake),
}

impl DatabaseOptions {
    pub const INTERNAL: &str = "internal";
    pub const DEBUG: &str = "debug";
    pub const POSTGRES: &str = "postgres";
    pub const BIGQUERY: &str = "bigquery";
    pub const MYSQL: &str = "mysql";
    pub const MONGO: &str = "mongo";
    pub const SNOWFLAKE: &str = "snowflake";

    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseOptions::Internal(_) => Self::INTERNAL,
            DatabaseOptions::Debug(_) => Self::DEBUG,
            DatabaseOptions::Postgres(_) => Self::POSTGRES,
            DatabaseOptions::BigQuery(_) => Self::BIGQUERY,
            DatabaseOptions::Mysql(_) => Self::MYSQL,
            DatabaseOptions::Mongo(_) => Self::MONGO,
            DatabaseOptions::Snowflake(_) => Self::SNOWFLAKE,
        }
    }
}

impl fmt::Display for DatabaseOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<options::database_options::Options> for DatabaseOptions {
    type Error = ProtoConvError;
    fn try_from(value: options::database_options::Options) -> Result<Self, Self::Error> {
        Ok(match value {
            options::database_options::Options::Internal(v) => {
                DatabaseOptions::Internal(v.try_into()?)
            }
            options::database_options::Options::Debug(v) => DatabaseOptions::Debug(v.try_into()?),
            options::database_options::Options::Postgres(v) => {
                DatabaseOptions::Postgres(v.try_into()?)
            }
            options::database_options::Options::Bigquery(v) => {
                DatabaseOptions::BigQuery(v.try_into()?)
            }
            options::database_options::Options::Mysql(v) => DatabaseOptions::Mysql(v.try_into()?),
            options::database_options::Options::Mongo(v) => DatabaseOptions::Mongo(v.try_into()?),
            options::database_options::Options::Snowflake(v) => {
                DatabaseOptions::Snowflake(v.try_into()?)
            }
        })
    }
}

impl TryFrom<options::DatabaseOptions> for DatabaseOptions {
    type Error = ProtoConvError;
    fn try_from(value: options::DatabaseOptions) -> Result<Self, Self::Error> {
        value.options.required("options")
    }
}

impl From<DatabaseOptions> for options::database_options::Options {
    fn from(value: DatabaseOptions) -> Self {
        match value {
            DatabaseOptions::Internal(v) => options::database_options::Options::Internal(v.into()),
            DatabaseOptions::Debug(v) => options::database_options::Options::Debug(v.into()),
            DatabaseOptions::Postgres(v) => options::database_options::Options::Postgres(v.into()),
            DatabaseOptions::BigQuery(v) => options::database_options::Options::Bigquery(v.into()),
            DatabaseOptions::Mysql(v) => options::database_options::Options::Mysql(v.into()),
            DatabaseOptions::Mongo(v) => options::database_options::Options::Mongo(v.into()),
            DatabaseOptions::Snowflake(v) => {
                options::database_options::Options::Snowflake(v.into())
            }
        }
    }
}

impl From<DatabaseOptions> for options::DatabaseOptions {
    fn from(value: DatabaseOptions) -> Self {
        options::DatabaseOptions {
            options: Some(value.into()),
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct DatabaseOptionsInternal {}

impl TryFrom<options::DatabaseOptionsInternal> for DatabaseOptionsInternal {
    type Error = ProtoConvError;
    fn try_from(_value: options::DatabaseOptionsInternal) -> Result<Self, Self::Error> {
        Ok(DatabaseOptionsInternal {})
    }
}

impl From<DatabaseOptionsInternal> for options::DatabaseOptionsInternal {
    fn from(_value: DatabaseOptionsInternal) -> Self {
        options::DatabaseOptionsInternal {}
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct DatabaseOptionsDebug {}

impl TryFrom<options::DatabaseOptionsDebug> for DatabaseOptionsDebug {
    type Error = ProtoConvError;
    fn try_from(_value: options::DatabaseOptionsDebug) -> Result<Self, Self::Error> {
        Ok(DatabaseOptionsDebug {})
    }
}

impl From<DatabaseOptionsDebug> for options::DatabaseOptionsDebug {
    fn from(_value: DatabaseOptionsDebug) -> Self {
        options::DatabaseOptionsDebug {}
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct DatabaseOptionsPostgres {
    pub connection_string: String,
}

impl TryFrom<options::DatabaseOptionsPostgres> for DatabaseOptionsPostgres {
    type Error = ProtoConvError;
    fn try_from(value: options::DatabaseOptionsPostgres) -> Result<Self, Self::Error> {
        Ok(DatabaseOptionsPostgres {
            connection_string: value.connection_string,
        })
    }
}

impl From<DatabaseOptionsPostgres> for options::DatabaseOptionsPostgres {
    fn from(value: DatabaseOptionsPostgres) -> Self {
        options::DatabaseOptionsPostgres {
            connection_string: value.connection_string,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct DatabaseOptionsBigQuery {
    pub service_account_key: String,
    pub project_id: String,
}

impl TryFrom<options::DatabaseOptionsBigQuery> for DatabaseOptionsBigQuery {
    type Error = ProtoConvError;
    fn try_from(value: options::DatabaseOptionsBigQuery) -> Result<Self, Self::Error> {
        Ok(DatabaseOptionsBigQuery {
            service_account_key: value.service_account_key,
            project_id: value.project_id,
        })
    }
}

impl From<DatabaseOptionsBigQuery> for options::DatabaseOptionsBigQuery {
    fn from(value: DatabaseOptionsBigQuery) -> Self {
        options::DatabaseOptionsBigQuery {
            service_account_key: value.service_account_key,
            project_id: value.project_id,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct DatabaseOptionsMysql {
    pub connection_string: String,
}

impl TryFrom<options::DatabaseOptionsMysql> for DatabaseOptionsMysql {
    type Error = ProtoConvError;
    fn try_from(value: options::DatabaseOptionsMysql) -> Result<Self, Self::Error> {
        Ok(DatabaseOptionsMysql {
            connection_string: value.connection_string,
        })
    }
}

impl From<DatabaseOptionsMysql> for options::DatabaseOptionsMysql {
    fn from(value: DatabaseOptionsMysql) -> Self {
        options::DatabaseOptionsMysql {
            connection_string: value.connection_string,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct DatabaseOptionsMongo {
    pub connection_string: String,
}

impl TryFrom<options::DatabaseOptionsMongo> for DatabaseOptionsMongo {
    type Error = ProtoConvError;
    fn try_from(value: options::DatabaseOptionsMongo) -> Result<Self, Self::Error> {
        Ok(DatabaseOptionsMongo {
            connection_string: value.connection_string,
        })
    }
}

impl From<DatabaseOptionsMongo> for options::DatabaseOptionsMongo {
    fn from(value: DatabaseOptionsMongo) -> Self {
        options::DatabaseOptionsMongo {
            connection_string: value.connection_string,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct DatabaseOptionsSnowflake {
    pub account_name: String,
    pub login_name: String,
    pub password: String,
    pub database_name: String,
    pub warehouse: String,
    pub role_name: String,
}

impl TryFrom<options::DatabaseOptionsSnowflake> for DatabaseOptionsSnowflake {
    type Error = ProtoConvError;
    fn try_from(value: options::DatabaseOptionsSnowflake) -> Result<Self, Self::Error> {
        Ok(DatabaseOptionsSnowflake {
            account_name: value.account_name,
            login_name: value.login_name,
            password: value.password,
            database_name: value.database_name,
            warehouse: value.warehouse,
            role_name: value.role_name,
        })
    }
}

impl From<DatabaseOptionsSnowflake> for options::DatabaseOptionsSnowflake {
    fn from(value: DatabaseOptionsSnowflake) -> Self {
        options::DatabaseOptionsSnowflake {
            account_name: value.account_name,
            login_name: value.login_name,
            password: value.password,
            database_name: value.database_name,
            warehouse: value.warehouse,
            role_name: value.role_name,
        }
    }
}

// Table options

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub enum TableOptions {
    Internal(TableOptionsInternal),
    Debug(TableOptionsDebug),
    Postgres(TableOptionsPostgres),
    BigQuery(TableOptionsBigQuery),
    Mysql(TableOptionsMysql),
    Local(TableOptionsLocal),
    Gcs(TableOptionsGcs),
    S3(TableOptionsS3),
    Mongo(TableOptionsMongo),
    Snowflake(TableOptionsSnowflake),
}

impl TableOptions {
    pub const INTERNAL: &str = "internal";
    pub const DEBUG: &str = "debug";
    pub const POSTGRES: &str = "postgres";
    pub const BIGQUERY: &str = "bigquery";
    pub const MYSQL: &str = "mysql";
    pub const LOCAL: &str = "local";
    pub const GCS: &str = "gcs";
    pub const S3_STORAGE: &str = "s3";
    pub const MONGO: &str = "mongo";
    pub const SNOWFLAKE: &str = "snowflake";

    pub const fn new_internal(columns: Vec<InternalColumnDefinition>) -> TableOptions {
        TableOptions::Internal(TableOptionsInternal { columns })
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TableOptions::Internal(_) => Self::INTERNAL,
            TableOptions::Debug(_) => Self::DEBUG,
            TableOptions::Postgres(_) => Self::POSTGRES,
            TableOptions::BigQuery(_) => Self::BIGQUERY,
            TableOptions::Mysql(_) => Self::MYSQL,
            TableOptions::Local(_) => Self::LOCAL,
            TableOptions::Gcs(_) => Self::GCS,
            TableOptions::S3(_) => Self::S3_STORAGE,
            TableOptions::Mongo(_) => Self::MONGO,
            TableOptions::Snowflake(_) => Self::SNOWFLAKE,
        }
    }
}

impl fmt::Display for TableOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<options::table_options::Options> for TableOptions {
    type Error = ProtoConvError;
    fn try_from(value: options::table_options::Options) -> Result<Self, Self::Error> {
        Ok(match value {
            options::table_options::Options::Internal(v) => TableOptions::Internal(v.try_into()?),
            options::table_options::Options::Debug(v) => TableOptions::Debug(v.try_into()?),
            options::table_options::Options::Postgres(v) => TableOptions::Postgres(v.try_into()?),
            options::table_options::Options::Bigquery(v) => TableOptions::BigQuery(v.try_into()?),
            options::table_options::Options::Mysql(v) => TableOptions::Mysql(v.try_into()?),
            options::table_options::Options::Local(v) => TableOptions::Local(v.try_into()?),
            options::table_options::Options::Gcs(v) => TableOptions::Gcs(v.try_into()?),
            options::table_options::Options::S3(v) => TableOptions::S3(v.try_into()?),
            options::table_options::Options::Mongo(v) => TableOptions::Mongo(v.try_into()?),
            options::table_options::Options::Snowflake(v) => TableOptions::Snowflake(v.try_into()?),
        })
    }
}

impl TryFrom<options::TableOptions> for TableOptions {
    type Error = ProtoConvError;
    fn try_from(value: options::TableOptions) -> Result<Self, Self::Error> {
        value.options.required("options")
    }
}

impl TryFrom<TableOptions> for options::table_options::Options {
    type Error = ProtoConvError;
    fn try_from(value: TableOptions) -> Result<Self, Self::Error> {
        Ok(match value {
            TableOptions::Internal(v) => options::table_options::Options::Internal(v.try_into()?),
            TableOptions::Debug(v) => options::table_options::Options::Debug(v.into()),
            TableOptions::Postgres(v) => options::table_options::Options::Postgres(v.into()),
            TableOptions::BigQuery(v) => options::table_options::Options::Bigquery(v.into()),
            TableOptions::Mysql(v) => options::table_options::Options::Mysql(v.into()),
            TableOptions::Local(v) => options::table_options::Options::Local(v.into()),
            TableOptions::Gcs(v) => options::table_options::Options::Gcs(v.into()),
            TableOptions::S3(v) => options::table_options::Options::S3(v.into()),
            TableOptions::Mongo(v) => options::table_options::Options::Mongo(v.into()),
            TableOptions::Snowflake(v) => options::table_options::Options::Snowflake(v.into()),
        })
    }
}

impl TryFrom<TableOptions> for options::TableOptions {
    type Error = ProtoConvError;
    fn try_from(value: TableOptions) -> Result<Self, Self::Error> {
        Ok(options::TableOptions {
            options: Some(value.try_into()?),
        })
    }
}
#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct TableOptionsInternal {
    pub columns: Vec<InternalColumnDefinition>,
}

impl TryFrom<options::TableOptionsInternal> for TableOptionsInternal {
    type Error = ProtoConvError;
    fn try_from(value: options::TableOptionsInternal) -> Result<Self, Self::Error> {
        Ok(TableOptionsInternal {
            columns: value
                .columns
                .into_iter()
                .map(|col| col.try_into())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<TableOptionsInternal> for options::TableOptionsInternal {
    type Error = ProtoConvError;
    fn try_from(value: TableOptionsInternal) -> Result<Self, Self::Error> {
        Ok(options::TableOptionsInternal {
            columns: value
                .columns
                .into_iter()
                .map(|col| col.try_into())
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct TableOptionsDebug {
    pub table_type: String,
}

impl TryFrom<options::TableOptionsDebug> for TableOptionsDebug {
    type Error = ProtoConvError;
    fn try_from(value: options::TableOptionsDebug) -> Result<Self, Self::Error> {
        Ok(TableOptionsDebug {
            table_type: value.table_type,
        })
    }
}

impl From<TableOptionsDebug> for options::TableOptionsDebug {
    fn from(value: TableOptionsDebug) -> Self {
        options::TableOptionsDebug {
            table_type: value.table_type,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct TableOptionsPostgres {
    pub connection_string: String,
    pub schema: String,
    pub table: String,
}

impl TryFrom<options::TableOptionsPostgres> for TableOptionsPostgres {
    type Error = ProtoConvError;
    fn try_from(value: options::TableOptionsPostgres) -> Result<Self, Self::Error> {
        Ok(TableOptionsPostgres {
            connection_string: value.connection_string,
            schema: value.schema,
            table: value.table,
        })
    }
}

impl From<TableOptionsPostgres> for options::TableOptionsPostgres {
    fn from(value: TableOptionsPostgres) -> Self {
        options::TableOptionsPostgres {
            connection_string: value.connection_string,
            schema: value.schema,
            table: value.table,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct TableOptionsBigQuery {
    pub service_account_key: String,
    pub project_id: String,
    pub dataset_id: String,
    pub table_id: String,
}

impl TryFrom<options::TableOptionsBigQuery> for TableOptionsBigQuery {
    type Error = ProtoConvError;
    fn try_from(value: options::TableOptionsBigQuery) -> Result<Self, Self::Error> {
        Ok(TableOptionsBigQuery {
            service_account_key: value.service_account_key,
            project_id: value.project_id,
            dataset_id: value.dataset_id,
            table_id: value.table_id,
        })
    }
}

impl From<TableOptionsBigQuery> for options::TableOptionsBigQuery {
    fn from(value: TableOptionsBigQuery) -> Self {
        options::TableOptionsBigQuery {
            service_account_key: value.service_account_key,
            project_id: value.project_id,
            dataset_id: value.dataset_id,
            table_id: value.table_id,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct TableOptionsMysql {
    pub connection_string: String,
    pub schema: String,
    pub table: String,
}

impl TryFrom<options::TableOptionsMysql> for TableOptionsMysql {
    type Error = ProtoConvError;
    fn try_from(value: options::TableOptionsMysql) -> Result<Self, Self::Error> {
        Ok(TableOptionsMysql {
            connection_string: value.connection_string,
            schema: value.schema,
            table: value.table,
        })
    }
}

impl From<TableOptionsMysql> for options::TableOptionsMysql {
    fn from(value: TableOptionsMysql) -> Self {
        options::TableOptionsMysql {
            connection_string: value.connection_string,
            schema: value.schema,
            table: value.table,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct TableOptionsLocal {
    pub location: String,
}

impl TryFrom<options::TableOptionsLocal> for TableOptionsLocal {
    type Error = ProtoConvError;
    fn try_from(value: options::TableOptionsLocal) -> Result<Self, Self::Error> {
        Ok(TableOptionsLocal {
            location: value.location,
        })
    }
}

impl From<TableOptionsLocal> for options::TableOptionsLocal {
    fn from(value: TableOptionsLocal) -> Self {
        options::TableOptionsLocal {
            location: value.location,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct TableOptionsGcs {
    pub service_account_key: String,
    pub bucket: String,
    pub location: String,
}

impl TryFrom<options::TableOptionsGcs> for TableOptionsGcs {
    type Error = ProtoConvError;
    fn try_from(value: options::TableOptionsGcs) -> Result<Self, Self::Error> {
        Ok(TableOptionsGcs {
            service_account_key: value.service_account_key,
            bucket: value.bucket,
            location: value.location,
        })
    }
}

impl From<TableOptionsGcs> for options::TableOptionsGcs {
    fn from(value: TableOptionsGcs) -> Self {
        options::TableOptionsGcs {
            service_account_key: value.service_account_key,
            bucket: value.bucket,
            location: value.location,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct TableOptionsS3 {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub region: String,
    pub bucket: String,
    pub location: String,
}

impl TryFrom<options::TableOptionsS3> for TableOptionsS3 {
    type Error = ProtoConvError;
    fn try_from(value: options::TableOptionsS3) -> Result<Self, Self::Error> {
        Ok(TableOptionsS3 {
            access_key_id: value.access_key_id,
            secret_access_key: value.secret_access_key,
            region: value.region,
            bucket: value.bucket,
            location: value.location,
        })
    }
}

impl From<TableOptionsS3> for options::TableOptionsS3 {
    fn from(value: TableOptionsS3) -> Self {
        options::TableOptionsS3 {
            access_key_id: value.access_key_id,
            secret_access_key: value.secret_access_key,
            region: value.region,
            bucket: value.bucket,
            location: value.location,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct TableOptionsMongo {
    pub connection_string: String,
    pub database: String,
    pub collection: String,
}

impl TryFrom<options::TableOptionsMongo> for TableOptionsMongo {
    type Error = ProtoConvError;
    fn try_from(value: options::TableOptionsMongo) -> Result<Self, Self::Error> {
        Ok(TableOptionsMongo {
            connection_string: value.connection_string,
            database: value.database,
            collection: value.collection,
        })
    }
}

impl From<TableOptionsMongo> for options::TableOptionsMongo {
    fn from(value: TableOptionsMongo) -> Self {
        options::TableOptionsMongo {
            connection_string: value.connection_string,
            database: value.database,
            collection: value.collection,
        }
    }
}

#[derive(Debug, Clone, Arbitrary, PartialEq, Eq)]
pub struct TableOptionsSnowflake {
    pub account_name: String,
    pub login_name: String,
    pub password: String,
    pub database_name: String,
    pub warehouse: String,
    pub role_name: String,
    pub schema_name: String,
    pub table_name: String,
}

impl TryFrom<options::TableOptionsSnowflake> for TableOptionsSnowflake {
    type Error = ProtoConvError;
    fn try_from(value: options::TableOptionsSnowflake) -> Result<Self, Self::Error> {
        Ok(TableOptionsSnowflake {
            account_name: value.account_name,
            login_name: value.login_name,
            password: value.password,
            database_name: value.database_name,
            warehouse: value.warehouse,
            role_name: value.role_name,
            schema_name: value.schema_name,
            table_name: value.table_name,
        })
    }
}

impl From<TableOptionsSnowflake> for options::TableOptionsSnowflake {
    fn from(value: TableOptionsSnowflake) -> Self {
        options::TableOptionsSnowflake {
            account_name: value.account_name,
            login_name: value.login_name,
            password: value.password,
            database_name: value.database_name,
            warehouse: value.warehouse,
            role_name: value.role_name,
            schema_name: value.schema_name,
            table_name: value.table_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::arbitrary::any;
    use proptest::proptest;

    proptest! {
        #[test]
        fn roundtrip_table_options(expected in any::<TableOptions>()) {
            let p: options::TableOptions = expected.clone().try_into().unwrap();
            let got: TableOptions = p.try_into().unwrap();
            assert_eq!(expected, got);
        }
    }

    proptest! {
        #[test]
        fn roundtrip_connection_options(expected in any::<DatabaseOptions>()) {
            let p: options::DatabaseOptions = expected.clone().into();
            let got: DatabaseOptions = p.try_into().unwrap();
            assert_eq!(expected, got);
        }
    }
}