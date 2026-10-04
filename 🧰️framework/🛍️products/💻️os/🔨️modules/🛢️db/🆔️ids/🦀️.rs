//! 🆔 Db identity types, DbError, and resource limits.

//#region 🔖️Ids
/// 🪪️ A document's identity, decoupled from `protocol::ArtifactId` (see module doc) but
/// sharing its single-`String` shape so conversions at the `db`/`protocol` boundary are lossless.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ArtifactId(pub String);

impl std::fmt::Display for ArtifactId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ArtifactId {
    fn from(value: &str) -> Self {
        ArtifactId(value.to_string())
    }
}

impl From<String> for ArtifactId {
    fn from(value: String) -> Self {
        ArtifactId(value)
    }
}

/// 👤️ An actor's (author's) identity, decoupled from `protocol::ActorId` — see
/// `ArtifactId`'s doc for the shared-shape conversion rationale.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ActorId(pub String);

impl std::fmt::Display for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ActorId {
    fn from(value: &str) -> Self {
        ActorId(value.to_string())
    }
}

impl From<String> for ActorId {
    fn from(value: String) -> Self {
        ActorId(value)
    }
}

/// 🔁️ A document actor's supervision generation (bumped on every restart by `db_actor`'s
/// `OneForOne`/`OneForAll`/`Escalate` supervision). `ArtifactHandle` (the `db` facade's stable
/// API) carries one alongside its mailbox sender so a handle obtained before a restart fails
/// loudly (`DbError::StaleGeneration`) instead of silently talking to a dead mailbox.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct GenerationId(pub u64);

impl GenerationId {
    /// 🌱️ The generation of a freshly spawned actor that has never restarted.
    pub const INITIAL: GenerationId = GenerationId(0);

    /// ⏭️ The next generation after a supervised restart.
    // 🚫️async: E1 pure accessor, only consumed by sync `#[test] fn` assertions — see R9
    pub fn next(self) -> GenerationId {
        GenerationId(self.0 + 1)
    }
}
//#endregion 🔖️Ids

//#region 🔖️Errors
/// 🚨️ The one error type every `db_*` public fn returns; never leaks `std::io::Error` (or
/// any other foreign error type) — every crate below `db_artifact` wraps its own errors into this.
#[derive(Debug, Clone, PartialEq)]
pub enum DbError {
    Io(String),
    NotFound(String),
    AlreadyExists(String),
    InvalidArgument(String),
    LimitExceeded(&'static str),
    Conflict(String),
    Fenced {
        expected: u64,
        actual: u64,
    },
    StaleGeneration {
        expected: GenerationId,
        actual: GenerationId,
    },
    Unavailable(String),
    Timeout(String),
    Corrupt(String),
    Closed,
    Unauthorized(String),
    Unimplemented(&'static str),
    Internal(String),
    /// ⚖️ `db_artifact::ArtifactEngine::submit`'s outcome-step gate (contract
    /// `MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C9): `policy` rejected the
    /// batch's `worst` graded `Severity` before anything reached the WAL — `messages` is every
    /// graded `protocol::MutationMessage` the caller (`db_engine`/`semio_hub`) needs to explain why,
    /// carried end-to-end rather than flattened to a string (the one deliberate exception to this
    /// enum's usual protocol-free identity types, since a rejection's whole point is to surface the
    /// graded messages verbatim to the submitter).
    Rejected {
        policy: protocol::MergePolicy,
        worst: semio_framework_diagnostic::Severity,
        messages: Vec<protocol::MutationMessage>,
    },
}

impl DbError {
    /// ⏳️ Whether the error says nothing about the batch itself — the engine could not take it now (unavailable, timed out,
    /// closed, fenced by a newer epoch or generation, an I/O fault) — so its author resends it; every other error is a
    /// verdict on the batch, final for it. The hub's refusal message and its rebuild decision both read this one predicate.
    pub fn is_transient(&self) -> bool {
        matches!(self, Self::Unavailable(_) | Self::Timeout(_) | Self::Closed | Self::Fenced { .. } | Self::StaleGeneration { .. } | Self::Io(_))
    }
}

impl std::fmt::Display for DbError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(message) => write!(formatter, "io error: {message}"),
            Self::NotFound(message) => write!(formatter, "not found: {message}"),
            Self::AlreadyExists(message) => write!(formatter, "already exists: {message}"),
            Self::InvalidArgument(message) => write!(formatter, "invalid argument: {message}"),
            Self::LimitExceeded(message) => write!(formatter, "limit exceeded: {message}"),
            Self::Conflict(message) => write!(formatter, "conflict: {message}"),
            Self::Fenced { expected, actual } => write!(formatter, "fenced: expected epoch {expected}, got {actual}"),
            Self::StaleGeneration { expected, actual } => write!(formatter, "stale generation: expected {expected:?}, got {actual:?}"),
            Self::Unavailable(message) => write!(formatter, "unavailable: {message}"),
            Self::Timeout(message) => write!(formatter, "timeout: {message}"),
            Self::Corrupt(message) => write!(formatter, "corrupt: {message}"),
            Self::Closed => formatter.write_str("closed"),
            Self::Unauthorized(message) => write!(formatter, "unauthorized: {message}"),
            Self::Unimplemented(message) => write!(formatter, "not implemented: {message}"),
            Self::Internal(message) => write!(formatter, "internal error: {message}"),
            Self::Rejected { worst, .. } => write!(formatter, "submit rejected by merge policy: worst={worst:?}"),
        }
    }
}

impl std::error::Error for DbError {}

impl From<pack::PackError> for DbError {
    /// 🔀️ `db_wal`/`db_snapshot` sit directly on top of `pack`/`protocol`'s `.spr`/`.spk`
    /// containers; this lets them use `?` without hand-rolling the same mapping repeatedly.
    /// Corruption-flavored `PackError` variants map to `DbError::Corrupt`, resource-flavored ones
    /// to `DbError::LimitExceeded`/`Io`, and schema mismatches to `DbError::InvalidArgument`.
    fn from(err: pack::PackError) -> Self {
        match err {
            pack::PackError::Io(message) => DbError::Io(message),
            pack::PackError::LimitExceeded(what) => DbError::LimitExceeded(what),
            pack::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message)) => DbError::InvalidArgument(message),
            other => DbError::Corrupt(other.to_string()),
        }
    }
}
//#endregion 🔖️Errors

//#region 🔖️Limits
/// 🛡️ Corruption/resource-hardening ceilings the `db` family validates against before
/// allocating (mirrors `pack::PackLimits`'s stated invariant) — every decoder/mailbox/query
/// path in the family checks a length against these before growing a buffer.
#[derive(Clone, Debug)]
pub struct DbLimits {
    pub max_command_bytes: u64,
    pub max_batch_commands: u32,
    pub max_payload_bytes: u64,
    pub max_query_bytes: u64,
    pub max_mailbox_depth: u32,
    pub max_open_artifacts: u32,
    pub max_preview_ttl_ms: u64,
}

/// 📦️ `max_batch_commands` is the wire's declared document-backbone batch maximum
/// (`protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES`): an authority never refuses, for its own
/// capacity, a batch the wire declares legal (ticket 26/09/23 session 14, C12 P1 — a link cut's whole
/// outbox arrives as one batch).
impl Default for DbLimits {
    fn default() -> Self {
        Self { max_command_bytes: 8 * 1024 * 1024, max_batch_commands: DECLARED_BATCH_COMMANDS, max_payload_bytes: 256 * 1024 * 1024, max_query_bytes: 4 * 1024 * 1024, max_mailbox_depth: 65_536, max_open_artifacts: 100_000, max_preview_ttl_ms: 5 * 60 * 1_000 }
    }
}

const DECLARED_BATCH_COMMANDS: u32 = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES as u32;
const _: () = assert!(DECLARED_BATCH_COMMANDS as usize == protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES);

/// 📏️ Validates `len` against `max` BEFORE the caller allocates anything sized by it —
/// shared by every length check across the `db` family so the "validate before allocating"
/// invariant has exactly one implementation to audit.
// 🚫️async: E1 pure accessor called from bounded storage executors — see R9
pub fn check_len(len: u64, max: u64, what: &'static str) -> Result<(), DbError> {
    if len > max {
        return Err(DbError::LimitExceeded(what));
    }
    Ok(())
}
//#endregion 🔖️Limits

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
