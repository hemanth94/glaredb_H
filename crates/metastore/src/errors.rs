#[derive(thiserror::Error, Debug)]
pub enum MetastoreError {
    #[error("Duplicate name: {0}")]
    DuplicateName(String),

    #[error("Duplicate object names found during load; name {name}, schema: {schema}, first: {first}, second: {second}")]
    DuplicateNameFoundDuringLoad {
        name: String,
        schema: u32,
        first: u32,
        second: u32,
    },

    #[error("Builtin object persisted when it shouldn't have been: {0:?}")]
    BuiltinObjectPersisted(crate::types::catalog::EntryMeta),

    #[error("Missing database catalog: {0}")]
    MissingCatalog(uuid::Uuid),

    #[error("Missing schema: {0}")]
    MissingNamedSchema(String),

    #[error("Missing database object; schema: {schema}, name: {name}")]
    MissingNamedObject { schema: String, name: String },

    #[error("Missing entry: {0}")]
    MissingEntry(u32),

    #[error("Catalog version mismatch; have: {have}, need: {need}")]
    VersionMismtatch { have: u64, need: u64 },

    #[error("Invalid database id: {0:?}")]
    InvalidDatabaseId(Vec<u8>),

    #[error("Schema {schema} has non-zero parent: {parent}")]
    SchemaHasNonZeroParent { schema: u32, parent: u32 },

    #[error("Object {object} has invalid parent id: {parent}")]
    ObjectHasInvalidParentId { object: u32, parent: u32 },

    #[error(transparent)]
    Storage(#[from] crate::storage::StorageError),

    #[error(transparent)]
    ProtoConv(#[from] crate::types::ProtoConvError),
}

pub type Result<T, E = MetastoreError> = std::result::Result<T, E>;

impl From<MetastoreError> for tonic::Status {
    fn from(value: MetastoreError) -> Self {
        tonic::Status::from_error(Box::new(value))
    }
}