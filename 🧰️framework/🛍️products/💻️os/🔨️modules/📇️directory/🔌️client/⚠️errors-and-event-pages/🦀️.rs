//! ⚠️ Errors and event pages of the directory client: the client error with its conversions, the canonical command receipt, administration page and event page records, the event page acknowledgement and the bootstrap transition. See [`super`].

use super::*;

//#region 🔖️Errors
#[derive(Debug)]
pub enum DirectoryClientError {
    Transport(TransportError),
    Decode(String),
    Unauthorized,
    Http {
        status: u16,
        body: String,
    },
    /// 🛑️ `ctx.cancel` was already cancelled BEFORE this call ever reached the transport — checked
    /// in `DirectoryClient::request_json`/`DirectoryStream::recv` up front, so a cancelled caller
    /// never even builds a request. `TransportError::Cancelled` (via the `Transport` variant above)
    /// is the other half: cancelled WHILE the transport call was in flight.
    Cancelled,
}

impl std::fmt::Display for DirectoryClientError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport(error) => std::fmt::Display::fmt(error, formatter),
            Self::Decode(error) => write!(formatter, "decode: {error}"),
            Self::Unauthorized => formatter.write_str("unauthorized"),
            Self::Http { status, body } => write!(formatter, "http {status}: {body}"),
            Self::Cancelled => formatter.write_str("cancelled"),
        }
    }
}

impl std::error::Error for DirectoryClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
            _ => None,
        }
    }
}

impl From<TransportError> for DirectoryClientError {
    fn from(error: TransportError) -> Self {
        Self::Transport(error)
    }
}

impl From<ValueError> for DirectoryClientError {
    fn from(error: ValueError) -> Self {
        Self::Decode(error.to_string())
    }
}
//#endregion 🔖️Errors

//#region 🔖️Wire
/// 📮️ One canonical `POST /directory/commands` completion, preserving the exact response bytes the
/// receipt digest covers. The public command boundary is closed: no `DslValue` and no open
/// `unknown` crosses it.
#[derive(Clone, Debug, PartialEq)]
pub struct CanonicalDirectoryCommandReceiptV1 {
    pub canonical_json: String,
    pub receipt: DirectoryCommandReceiptV1,
}

/// 🪪️ `GET /auth/sessions/me`'s exact server-bound public authority.
pub type SessionView = DirectorySessionAuthorityV1;

/// 🎫️ `POST /auth/sessions`'s body. Wire is snake_case (`token`, `user_id`), NOT this contract's
/// general camelCase convention: this route predates the wave (`🌎️hub/📦️bin.rs`'s
/// `CreateAuthSessionResponse` has no `rename_all`) and §C2 marks it "unchanged" — the client
/// matches the ACTUAL wire, not the convention.
#[derive(Clone, Debug, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub struct SessionMintResponse {
    pub token: String,
    pub user_id: String,
}

/// 🏛️ One administration page plus the exact response bytes its receipt covers.
#[derive(Clone, Debug, PartialEq)]
pub struct CanonicalDirectorySpaceAdministrationPageV1 {
    pub(super) canonical_json: String,
    pub(super) page: DirectorySpaceAdministrationPageV1,
}

impl CanonicalDirectorySpaceAdministrationPageV1 {
    pub fn canonical_json(&self) -> &str {
        &self.canonical_json
    }
    pub fn page(&self) -> &DirectorySpaceAdministrationPageV1 {
        &self.page
    }
    pub fn receipt_sha256(&self) -> &str {
        self.page.receipt_sha256()
    }
}

/// 📄️ Immutable original page bytes plus only the authenticated header needed for ACK ordering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalDirectoryEventPageV1 {
    pub(super) canonical_json: String,
    pub(super) session_binding_sha256: String,
    pub(super) authorization_generation: u64,
    pub(super) after_seq_exclusive: u64,
    pub(super) through_seq_inclusive: u64,
    pub(super) has_more: bool,
    pub(super) receipt_sha256: String,
}

impl CanonicalDirectoryEventPageV1 {
    /// 🧪️ Builds a page from explicit header values so ordering laws can present pages the authenticated decoder never produced.
    #[cfg(any(test, feature = "testkit"))]
    pub fn from_header_parts(canonical_json: String, session_binding_sha256: String, authorization_generation: u64, after_seq_exclusive: u64, through_seq_inclusive: u64, has_more: bool, receipt_sha256: String) -> Self {
        Self { canonical_json, session_binding_sha256, authorization_generation, after_seq_exclusive, through_seq_inclusive, has_more, receipt_sha256 }
    }

    pub fn canonical_json(&self) -> &str {
        &self.canonical_json
    }
    pub fn session_binding_sha256(&self) -> &str {
        &self.session_binding_sha256
    }
    pub fn authorization_generation(&self) -> u64 {
        self.authorization_generation
    }
    pub fn after_seq_exclusive(&self) -> u64 {
        self.after_seq_exclusive
    }
    pub fn through_seq_inclusive(&self) -> u64 {
        self.through_seq_inclusive
    }
    pub fn has_more(&self) -> bool {
        self.has_more
    }
    pub fn receipt_sha256(&self) -> &str {
        &self.receipt_sha256
    }

    /// ✅️ Captures the exact non-secret header a retained Home action must acknowledge.
    pub fn acknowledgement(&self, bootstrap_epoch: u64) -> DirectoryEventPageAckV1 {
        DirectoryEventPageAckV1 {
            bootstrap_epoch,
            session_binding_sha256: self.session_binding_sha256.clone(),
            authorization_generation: self.authorization_generation,
            through_seq_inclusive: self.through_seq_inclusive,
            receipt_sha256: self.receipt_sha256.clone(),
        }
    }
}

/// 🧾️ Exact retained-Home acknowledgement for one still-owned directory page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectoryEventPageAckV1 {
    pub bootstrap_epoch: u64,
    pub session_binding_sha256: String,
    pub authorization_generation: u64,
    pub through_seq_inclusive: u64,
    pub receipt_sha256: String,
}

/// 🚦️ Next I/O boundary after one accepted page acknowledgement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectoryBootstrapTransition {
    Fetch { after: u64 },
    Live { since: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DirectoryBootstrapPhase {
    Fetching,
    AwaitingAck,
    Live,
    Closed,
}

/// 🧭️ Sole native-shell owner of the fetch → retained Home ACK → live cursor.
pub struct DirectoryEventPageBootstrapV1 {
    bootstrap_epoch: u64,
    acknowledged_through: u64,
    pub(super) pending: Option<CanonicalDirectoryEventPageV1>,
    phase: DirectoryBootstrapPhase,
}

impl DirectoryEventPageBootstrapV1 {
    pub fn new(bootstrap_epoch: u64, after: u64) -> Result<Self, DirectoryClientError> {
        if bootstrap_epoch > DOCUMENT_OPEN_MAX_SAFE_INTEGER || after > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
            return Err(DirectoryClientError::Decode("directory bootstrap owner is not wire-safe".into()));
        }
        Ok(Self { bootstrap_epoch, acknowledged_through: after, pending: None, phase: DirectoryBootstrapPhase::Fetching })
    }

    pub fn bootstrap_epoch(&self) -> u64 {
        self.bootstrap_epoch
    }

    pub fn after(&self) -> u64 {
        self.acknowledged_through
    }

    pub fn present(&mut self, page: CanonicalDirectoryEventPageV1) -> Result<DirectoryEventPageAckV1, DirectoryClientError> {
        if self.phase != DirectoryBootstrapPhase::Fetching || page.after_seq_exclusive != self.acknowledged_through || page.through_seq_inclusive < self.acknowledged_through {
            return Err(DirectoryClientError::Decode("directory bootstrap page ordering mismatch".into()));
        }
        let acknowledgement = page.acknowledgement(self.bootstrap_epoch);
        self.pending = Some(page);
        self.phase = DirectoryBootstrapPhase::AwaitingAck;
        Ok(acknowledgement)
    }

    pub fn acknowledge(&mut self, acknowledgement: &DirectoryEventPageAckV1) -> Result<DirectoryBootstrapTransition, DirectoryClientError> {
        let page = self.pending.as_ref().ok_or_else(|| DirectoryClientError::Decode("directory bootstrap has no pending page".into()))?;
        if self.phase != DirectoryBootstrapPhase::AwaitingAck
            || acknowledgement.bootstrap_epoch != self.bootstrap_epoch
            || acknowledgement.session_binding_sha256 != page.session_binding_sha256
            || acknowledgement.authorization_generation != page.authorization_generation
            || acknowledgement.through_seq_inclusive != page.through_seq_inclusive
            || acknowledgement.receipt_sha256 != page.receipt_sha256
        {
            return Err(DirectoryClientError::Decode("directory bootstrap acknowledgement mismatch".into()));
        }
        self.acknowledged_through = page.through_seq_inclusive;
        let has_more = page.has_more;
        self.pending = None;
        self.phase = if has_more { DirectoryBootstrapPhase::Fetching } else { DirectoryBootstrapPhase::Live };
        Ok(if has_more { DirectoryBootstrapTransition::Fetch { after: self.acknowledged_through } } else { DirectoryBootstrapTransition::Live { since: self.acknowledged_through } })
    }

    pub fn reject(&mut self, bootstrap_epoch: u64, receipt_sha256: &str) -> Result<u64, DirectoryClientError> {
        let page = self.pending.as_ref().ok_or_else(|| DirectoryClientError::Decode("directory bootstrap has no pending page".into()))?;
        if self.phase != DirectoryBootstrapPhase::AwaitingAck || bootstrap_epoch != self.bootstrap_epoch || receipt_sha256 != page.receipt_sha256 {
            return Err(DirectoryClientError::Decode("directory bootstrap rejection mismatch".into()));
        }
        self.pending = None;
        self.phase = DirectoryBootstrapPhase::Fetching;
        Ok(self.acknowledged_through)
    }

    pub fn wake(&mut self, rebootstrap: bool) -> Option<u64> {
        if self.phase != DirectoryBootstrapPhase::Live {
            return None;
        }
        if rebootstrap {
            self.acknowledged_through = 0;
        }
        self.phase = DirectoryBootstrapPhase::Fetching;
        Some(self.acknowledged_through)
    }

    pub fn close(&mut self) {
        self.pending = None;
        self.phase = DirectoryBootstrapPhase::Closed;
    }
}
//#endregion 🔖️Wire
