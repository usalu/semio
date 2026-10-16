//! 👥️ `presence` — document scope keys, presence leases and colour leases.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use directory::os_directory::DirectoryPresenceActor;
use semio_hub_artifact_creation::ArtifactCreationCommitAuthorityV1;
use semio_hub_artifact_authority::AuthorityOperationControl;
use directory::os_directory::DocumentScope;
use semio_hub_directory_contract::IdentityVerificationControl;
use semio_hub_inference_runtime_contract::GisMapApprovalIngressAuthorityV1;
use semio_hub_lag_rebootstrap::RebootstrapTransferControl;
use std::collections::BTreeMap;
use db::storage::PayloadStorage as _;
use protocol::ArtifactId as ProtocolArtifactId;

//#region 🔖️State
/// 🎫️ Unambiguous v1 key for the flat DB/fanout catalogs: ASCII `v1:`, both UTF-8 byte
/// lengths in decimal, separators, then the exact adjacent UTF-8 scope payloads. Both lengths make
/// colon-containing and non-ASCII identifiers structural without a fallback decoder.
pub fn document_scope_key_v1(scope: &DocumentScope) -> String {
    format!("v1:{}:{}:{}{}", scope.space_id.len(), scope.document_id.len(), scope.space_id, scope.document_id)
}

pub fn db_artifact_id(scope: &DocumentScope) -> ProtocolArtifactId {
    ProtocolArtifactId(document_scope_key_v1(scope))
}

pub fn db_core_document_id(id: &ProtocolArtifactId) -> db::ArtifactId {
    db::ArtifactId(id.0.clone())
}

/// 👤️ One private server-local owner slot; only the matching live socket may refresh, expire, or remove it.
pub struct PresenceLeaseSlot {
    pub socket_live_id: String,
    pub expires_at: tokio::time::Instant,
    pub user_id: Option<String>,
    pub connected_at_ms: i64,
    pub label: Option<String>,
    pub role: Option<String>,
    pub document_surface: Option<String>,
    pub color: u8,
    /// 🤖️ The principal kind the hub AUTHENTICATED, never one a client claimed. An agent session
    /// (`AuthSessionKind::Agent`, minted from a delegation at `POST /auth/agent-sessions`) is an
    /// `Agent` peer; everything else is `Human`. Normalized onto every outbound roster entry
    /// alongside `user_id`/`role`/`color`, so a human session can never impersonate an agent and an
    /// agent can never hide as a human.
    pub principal_kind: protocol::PresencePrincipalKind,
    pub peer: Option<Vec<u8>>,
}

/// 🪪️ Exactly the roster fields the hub AUTHENTICATED, cloned out of a `PresenceLeaseSlot` so the
/// canonical identity-only peer can be encoded without holding the presence map across an await.
/// It is the row a live socket contributes on its own, with every app-owned ephemeral absent —
/// `refresh_document_presence` stamps these same fields onto a beating peer, so a stripped row and a
/// beating row differ only in the ephemerals the client owns.
pub struct PresenceIdentityV1 {
    pub connected_at_ms: i64,
    pub label: Option<String>,
    pub user_id: Option<String>,
    pub role: Option<String>,
    pub color: u8,
    pub document_surface: Option<String>,
    pub principal_kind: protocol::PresencePrincipalKind,
}

impl PresenceIdentityV1 {
    pub fn of(slot: &PresenceLeaseSlot) -> Self {
        Self {
            connected_at_ms: slot.connected_at_ms,
            label: slot.label.clone(),
            user_id: slot.user_id.clone(),
            role: slot.role.clone(),
            color: slot.color,
            document_surface: slot.document_surface.clone(),
            principal_kind: slot.principal_kind,
        }
    }

    pub async fn encode(&self, actor: &str) -> Vec<u8> {
        protocol::encode_presence_peer(&protocol::PresencePeer {
            actor: actor.to_string(),
            connected_at_ms: self.connected_at_ms,
            label: self.label.clone(),
            user_id: self.user_id.clone(),
            role: self.role.clone(),
            color: Some(self.color),
            surface: self.document_surface.clone(),
            presence_pack: None,
            drag_ghost_json: None,
            interaction: None,
            views: Vec::new(),
            ui: None,
            tool_run: None,
            principal_kind: Some(self.principal_kind),
            active_tool: None,
            history_edit: None,
            typing: Vec::new(),
        })
        .await
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PresenceLeaseTransition {
    Published,
    NoChange,
    Rejected,
    Unavailable,
}

pub struct PresenceSnapshot {
    pub peers: Vec<Vec<u8>>,
    pub actors: Vec<DirectoryPresenceActor>,
}

/// 🗂️ The member-directory presence projection each document last published, held by the presence publication
/// gate: a peer beat that only moves its pointer, camera or caret changes the document roster (every delta goes to the
/// document's sockets) but not the projection (actor, user, surface, colour), so it is not republished to every
/// member's directory socket. A document whose roster emptied is forgotten after publishing the empty projection, and a
/// document that never had a directory row publishes none.
#[derive(Default)]
pub struct PresenceDirectoryProjections {
    pub published: BTreeMap<String, Vec<DirectoryPresenceActor>>,
}

#[cfg(any(test, feature = "testkit"))]
pub struct TestPresenceClock {
    pub(crate) origin: tokio::time::Instant,
    pub now_ms: std::sync::atomic::AtomicU64,
    pub gate_ticks: std::sync::atomic::AtomicBool,
    pub(crate) tick_admitted: tokio::sync::Semaphore,
    pub tick_release: tokio::sync::Semaphore,
    pub(crate) tick_evaluated: tokio::sync::Semaphore,
}

#[cfg(any(test, feature = "testkit"))]
impl TestPresenceClock {
    pub fn new() -> Self {
        Self {
            origin: tokio::time::Instant::now(),
            now_ms: std::sync::atomic::AtomicU64::new(0),
            gate_ticks: std::sync::atomic::AtomicBool::new(false),
            tick_admitted: tokio::sync::Semaphore::new(0),
            tick_release: tokio::sync::Semaphore::new(0),
            tick_evaluated: tokio::sync::Semaphore::new(0),
        }
    }

    pub fn now(&self) -> tokio::time::Instant {
        self.origin + std::time::Duration::from_millis(self.now_ms.load(std::sync::atomic::Ordering::SeqCst))
    }

    pub fn advance_to(&self, now_ms: u64) {
        self.now_ms.store(now_ms, std::sync::atomic::Ordering::SeqCst);
    }

    pub async fn evaluate_tick(&self, ungate: bool) {
        tokio::time::timeout(std::time::Duration::from_secs(5), self.tick_admitted.acquire()).await.expect("presence tick admission deadline").expect("presence tick admission").forget();
        if ungate {
            self.gate_ticks.store(false, std::sync::atomic::Ordering::Release);
        }
        self.tick_release.add_permits(1);
        tokio::time::timeout(std::time::Duration::from_secs(5), self.tick_evaluated.acquire()).await.expect("presence tick evaluation deadline").expect("presence tick evaluation").forget();
    }
}

/// 🎨️ One actor's held palette index within a space, ref-counted across that actor's
/// concurrently open document sockets in the same space (contract §C7.3: "A's second document socket
/// keeps 0").
pub struct ColorLease {
    pub index: u8,
    pub refs: u32,
}

/// 🌈️ One space's live session-color leases (contract §C7.3) — never persisted, rebuilt from
/// nothing on hub restart, mirroring `presence`'s own ephemeral law.
#[derive(Default)]
pub struct SpaceColors {
    pub by_actor: BTreeMap<String, ColorLease>,
}

pub const PRESENCE_LEASE_TTL_MS: u64 = 15_000;
//#endregion 🔖️State
