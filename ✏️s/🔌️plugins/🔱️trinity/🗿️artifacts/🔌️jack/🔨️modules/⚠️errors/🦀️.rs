//! ⚠️ Jack errors: the `TrinityRamError` of graph snapshots, manifest validation and mutations with its `Display`, `Error` and `From` conversions.

use semio_framework_graph_manifest::ManifestValidationError;

//#region ⚠️ Errors
/// ⚠️ Trinity graph snapshot, manifest-validation, and mutation errors.
#[derive(Debug)]
pub enum TrinityRamError {
    /// 🧬️ JSON (de)serialization failure.
    Json(String),
    ValueRefusal(semio_framework_value::ValueError),
    /// 🧭️ VCS store/dispatch failure.
    Vcs(vcs::VcsError),
    /// 🧬️ Persisted mutation diff rejection.
    MutationApply(protocol::MutationApplyError),
    /// 📜️ Compile-time manifest validation failure (path-qualified).
    Manifest(ManifestValidationError),
    SchemaMismatch {
        expected: &'static str,
        actual: String,
    },
    UnknownManifestId(String),
    ManifestMissing,
    NodeNotFound(String),
    EdgeNotFound(String),
    NodeAlreadyExists(String),
    EdgeAlreadyExists(String),
    InvalidSourcePortKey(String),
    InvalidTargetPortKey(String),
    SourceNodeNotFound(String),
    TargetNodeNotFound(String),
    PortKindNotDeclaredOnFixture {
        node_id: String,
        port_kind: String,
        node_kind: String,
    },
    PortKindNotDeclaredOnMutation {
        node_id: String,
        port_id: String,
        port_kind: String,
        node_kind: String,
    },
    UnknownNodeKind {
        kind: String,
    },
    UnknownEdgeKind {
        kind: String,
    },
    UnknownPortKind {
        kind: String,
    },
    UnknownEntityKind {
        path: String,
    },
    UnknownPropertyAtPath {
        path: String,
        key: String,
    },
    PropertyTypeMismatch {
        path: String,
        name: String,
        value_type: String,
    },
    UnknownPropertyInBag {
        path: String,
        key: String,
    },
    /// 📏️ A `set-query` beyond [`JACK_QUERY_MAXIMUM_BYTES`].
    QueryTooLarge {
        bytes: usize,
        maximum: usize,
    },
}

impl std::fmt::Display for TrinityRamError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::ValueRefusal(error) => write!(formatter,"{error}"),
            Self::Vcs(error) => write!(formatter, "{error}"),
            Self::MutationApply(error) => write!(formatter, "{error}"),
            Self::Manifest(error) => write!(formatter, "{}: {}", error.path, error.message),
            Self::SchemaMismatch { expected, actual } => write!(formatter, "expected schema {expected}, got {actual}"),
            Self::UnknownManifestId(id) => write!(formatter, "unknown manifest id {id}"),
            Self::ManifestMissing => formatter.write_str("snapshot missing manifest or manifestId"),
            Self::NodeNotFound(id) => write!(formatter, "node {id} not found"),
            Self::EdgeNotFound(id) => write!(formatter, "edge {id} not found"),
            Self::NodeAlreadyExists(id) => write!(formatter, "node {id} already exists"),
            Self::EdgeAlreadyExists(id) => write!(formatter, "edge {id} already exists"),
            Self::InvalidSourcePortKey(key) => write!(formatter, "invalid source port key {key}"),
            Self::InvalidTargetPortKey(key) => write!(formatter, "invalid target port key {key}"),
            Self::SourceNodeNotFound(id) => write!(formatter, "source node {id} not found"),
            Self::TargetNodeNotFound(id) => write!(formatter, "target node {id} not found"),
            Self::PortKindNotDeclaredOnFixture { node_id, port_kind, node_kind } => write!(formatter, "nodes/{node_id}/ports/{port_kind}: port kind {port_kind} not declared on node kind {node_kind}"),
            Self::PortKindNotDeclaredOnMutation { node_id, port_id, port_kind, node_kind } => write!(formatter, "nodes/{node_id}/ports/{port_id}: port kind {port_kind} not declared on node kind {node_kind}"),
            Self::UnknownNodeKind { kind } => write!(formatter, "nodes/{kind}: unknown node kind {kind:?}"),
            Self::UnknownEdgeKind { kind } => write!(formatter, "edges/{kind}: unknown edge kind {kind:?}"),
            Self::UnknownPortKind { kind } => write!(formatter, "ports/{kind}: unknown port kind {kind:?}"),
            Self::UnknownEntityKind { path } => write!(formatter, "{path}: unknown kind"),
            Self::UnknownPropertyAtPath { path, key } => write!(formatter, "{path}: unknown property {key:?}"),
            Self::PropertyTypeMismatch { path, name, value_type } => write!(formatter, "{path}/{name}: property type mismatch for {value_type}"),
            Self::UnknownPropertyInBag { path, key } => write!(formatter, "{path}/{key}: unknown property {key:?}"),
            Self::QueryTooLarge { bytes, maximum } => write!(formatter, "query: {bytes} bytes exceed the {maximum}-byte bound"),
        }
    }
}

impl std::error::Error for TrinityRamError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Vcs(error) => std::error::Error::source(error),
            Self::MutationApply(error) => std::error::Error::source(error),
            _ => None,
        }
    }
}

impl From<semio_framework_pack_json::JsonError> for TrinityRamError {
    fn from(error: semio_framework_pack_json::JsonError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<semio_framework_value::ValueError> for TrinityRamError {
    fn from(error: semio_framework_value::ValueError) -> Self {
        Self::ValueRefusal(error)
    }
}

impl From<vcs::VcsError> for TrinityRamError {
    fn from(error: vcs::VcsError) -> Self {
        Self::Vcs(error)
    }
}

impl From<protocol::MutationApplyError> for TrinityRamError {
    fn from(error: protocol::MutationApplyError) -> Self {
        Self::MutationApply(error)
    }
}

/// 🔀️ [`ManifestValidationError`] carries no `std::error::Error` impl of its own (plain path/message struct), so this conversion remains explicit.
impl From<ManifestValidationError> for TrinityRamError {
    fn from(error: ManifestValidationError) -> Self {
        Self::Manifest(error)
    }
}
//#endregion ⚠️ Errors
