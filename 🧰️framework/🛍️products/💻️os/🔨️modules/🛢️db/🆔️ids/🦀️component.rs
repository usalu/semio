//! 🆔 Db identity types, DbError, and resource limits.

//#region 🔖️Ids
/// @emoji 🪪️ A document's identity, decoupled from `protocol::ArtifactId` (see module doc) but
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

/// @emoji 👤️ An actor's (author's) identity, decoupled from `protocol::ActorId` — see
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

/// @emoji 🔁️ A document actor's supervision generation (bumped on every restart by `db_actor`'s
/// `OneForOne`/`OneForAll`/`Escalate` supervision). `ArtifactHandle` (the `db` facade's stable
/// API) carries one alongside its mailbox sender so a handle obtained before a restart fails
/// loudly (`DbError::StaleGeneration`) instead of silently talking to a dead mailbox.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct GenerationId(pub u64);

impl GenerationId {
    /// @emoji 🌱️ The generation of a freshly spawned actor that has never restarted.
    pub const INITIAL: GenerationId = GenerationId(0);

    /// @emoji ⏭️ The next generation after a supervised restart.
    pub fn next(self) -> GenerationId {
        GenerationId(self.0 + 1)
    }
}
//#endregion 🔖️Ids

//#region 🔖️Errors
/// @emoji 🚨️ The one error type every `db_*` public fn returns; never leaks `std::io::Error` (or
/// any other foreign error type) — every crate below `db_artifact` wraps its own errors into this.
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum DbError {
    #[error("io error: {0}")]
    Io(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("already exists: {0}")]
    AlreadyExists(String),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("limit exceeded: {0}")]
    LimitExceeded(&'static str),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("fenced: expected epoch {expected}, got {actual}")]
    Fenced { expected: u64, actual: u64 },
    #[error("stale generation: expected {expected:?}, got {actual:?}")]
    StaleGeneration { expected: GenerationId, actual: GenerationId },
    #[error("unavailable: {0}")]
    Unavailable(String),
    #[error("timeout: {0}")]
    Timeout(String),
    #[error("corrupt: {0}")]
    Corrupt(String),
    #[error("closed")]
    Closed,
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("not implemented: {0}")]
    Unimplemented(&'static str),
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<pack::PackError> for DbError {
    /// @emoji 🔀️ `db_wal`/`db_snapshot` sit directly on top of `pack`/`protocol`'s `.spr`/`.spk`
    /// containers; this lets them use `?` without hand-rolling the same mapping repeatedly.
    /// Corruption-flavored `PackError` variants map to `DbError::Corrupt`, resource-flavored ones
    /// to `DbError::LimitExceeded`/`Io`, and schema mismatches to `DbError::InvalidArgument`.
    fn from(err: pack::PackError) -> Self {
        match err {
            pack::PackError::Io(message) => DbError::Io(message),
            pack::PackError::LimitExceeded(what) => DbError::LimitExceeded(what),
            pack::PackError::Schema(message) => DbError::InvalidArgument(message),
            other => DbError::Corrupt(other.to_string()),
        }
    }
}
//#endregion 🔖️Errors

//#region 🔖️Limits
/// @emoji 🛡️ Corruption/resource-hardening ceilings the `db` family validates against before
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

impl Default for DbLimits {
    fn default() -> Self {
        Self { max_command_bytes: 8 * 1024 * 1024, max_batch_commands: 4_096, max_payload_bytes: 256 * 1024 * 1024, max_query_bytes: 4 * 1024 * 1024, max_mailbox_depth: 65_536, max_open_artifacts: 100_000, max_preview_ttl_ms: 5 * 60 * 1_000 }
    }
}

/// @emoji 📏️ Validates `len` against `max` BEFORE the caller allocates anything sized by it —
/// shared by every length check across the `db` family so the "validate before allocating"
/// invariant has exactly one implementation to audit.
pub fn check_len(len: u64, max: u64, what: &'static str) -> Result<(), DbError> {
    if len > max {
        return Err(DbError::LimitExceeded(what));
    }
    Ok(())
}
//#endregion 🔖️Limits

#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️Ids
    #[test]
    fn document_id_and_actor_id_convert_and_display() {
        let document: ArtifactId = "doc-1".into();
        assert_eq!(document.to_string(), "doc-1");
        assert_eq!(document, ArtifactId::from("doc-1".to_string()));

        let actor: ActorId = "actor-1".into();
        assert_eq!(actor.to_string(), "actor-1");
    }

    #[test]
    fn generation_id_next_is_strictly_monotonic() {
        let g0 = GenerationId::INITIAL;
        let g1 = g0.next();
        let g2 = g1.next();
        assert!(g0 < g1);
        assert!(g1 < g2);
        assert_eq!(g0, GenerationId(0));
        assert_eq!(g2, GenerationId(2));
    }
    //#endregion 🔖️Ids

    //#region 🔖️Errors
    #[test]
    fn pack_error_conversion_never_panics_and_maps_by_category() {
        let corrupt: DbError = pack::PackError::BadMagic.into();
        assert!(matches!(corrupt, DbError::Corrupt(_)));

        let limit: DbError = pack::PackError::LimitExceeded("too big").into();
        assert_eq!(limit, DbError::LimitExceeded("too big"));

        let io: DbError = pack::PackError::Io("disk full".to_string()).into();
        assert_eq!(io, DbError::Io("disk full".to_string()));

        let schema: DbError = pack::PackError::Schema("bad field".to_string()).into();
        assert_eq!(schema, DbError::InvalidArgument("bad field".to_string()));
    }
    //#endregion 🔖️Errors

    //#region 🔖️Limits
    #[test]
    fn check_len_rejects_over_limit_before_any_allocation_would_happen() {
        assert!(check_len(10, 100, "test").is_ok());
        assert_eq!(check_len(101, 100, "test"), Err(DbError::LimitExceeded("test")));
    }
    //#endregion 🔖️Limits
}
