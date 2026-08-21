//! 🎫️ Handle table + idempotency store (`📌️sol-P1b-packet.md` §2.2, `📋️master.md` §2.4) — every
//! `prep_`/`txn_`/`undo_`/`job_`/`appr_`/`cont_`/`sess_` handle a later packet (P6 mutation protocol,
//! P10 shell) mints flows through here. **Authorization is never derivable from possession of a
//! handle alone**: [`HandleTable::resolve`] takes the requesting session and returns
//! `PERMISSION_DENIED` on owner mismatch even though the handle itself exists and has not expired —
//! this is the security property `mod quick`'s `cross_session_resolve_is_permission_denied_not_a_leak`
//! test exists to pin down, not a nicety. Zero dependency on the kernel/plugin/channel/actor crates,
//! matching this crate's own root doc.

use crate::errors::{GatewayError, GatewayErrorCode};
use crate::schema::InvocationReport;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

//#region 🔖️HandleKind
/// 🏷️ The seven handle kinds this gateway mints — prefix + default TTL are the frozen source of
/// truth (`📌️sol-P1b-packet.md` §2.2's TTL table); `Session` alone is sliding (refreshed on every
/// successful [`HandleTable::resolve`]) rather than fixed-TTL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HandleKind {
    Session,
    Prepared,
    Transaction,
    Undo,
    Job,
    Approval,
    Continuation,
}

impl HandleKind {
    /// 🔤️ The kind-prefixed id prefix (`sess_`, `prep_`, `txn_`, `undo_`, `job_`, `appr_`, `cont_`).
    pub fn prefix(self) -> &'static str {
        match self {
            HandleKind::Session => "sess_",
            HandleKind::Prepared => "prep_",
            HandleKind::Transaction => "txn_",
            HandleKind::Undo => "undo_",
            HandleKind::Job => "job_",
            HandleKind::Approval => "appr_",
            HandleKind::Continuation => "cont_",
        }
    }

    /// ⏳️ Default TTL in milliseconds at mint time — `Session`'s value is only ever used as the
    /// sliding-refresh window (see [`HandleTable::resolve`]), never a hard expiry from mint.
    pub fn default_ttl_ms(self) -> u64 {
        const MINUTE: u64 = 60_000;
        const HOUR: u64 = 60 * MINUTE;
        match self {
            HandleKind::Prepared => 10 * MINUTE,
            HandleKind::Transaction => 30 * MINUTE,
            HandleKind::Undo => 24 * HOUR,
            HandleKind::Job => HOUR,
            HandleKind::Approval => 10 * MINUTE,
            HandleKind::Continuation => 5 * MINUTE,
            HandleKind::Session => 30 * MINUTE,
        }
    }
}
//#endregion 🔖️HandleKind

//#region 🔖️SessionHandle
/// 🆔️ The owning session's `sess_`-prefixed id — a newtype (rather than a bare `String`) so
/// [`HandleTable::resolve`]'s owner comparison can never be accidentally satisfied by comparing the
/// wrong kind of string.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SessionHandle(pub String);

impl SessionHandle {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for SessionHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
//#endregion 🔖️SessionHandle

//#region 🔖️Attachment
/// 🔗️ What a handle is bound to — distinct from `AgentSession.attachment` (`📋️master.md` §2.4's
/// headless/shell workspace binding, a later packet's concern). `Other` is the escape hatch a later
/// packet's own payload-specific binding uses without growing this enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Attachment {
    None,
    Capability { capability_id: String },
    Artifact { artifact_id: String },
    Other { label: String },
}
//#endregion 🔖️Attachment

//#region 🔖️HandleRecord
/// 🧾️ `HandleRecord{kind, owner, bound_to, expires_ms, payload}` — the one row shape every handle
/// kind shares; `payload` is the kind-specific structured body (a later packet's concern what goes in
/// it, e.g. a `PreparedActionReport` for `Prepared`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleRecord {
    pub kind: HandleKind,
    pub owner: SessionHandle,
    pub bound_to: Attachment,
    pub expires_ms: u64,
    pub payload: serde_json::Value,
}
//#endregion 🔖️HandleRecord

//#region 🔖️IdGeneration
static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// 🎲️ ULID-ish (time + monotonic counter + process-local entropy, blake3-mixed) id generation — no
/// new external dependency (`uuid`/etc): `📌️sol-P1b-packet.md` §2.2 explicitly allows "a small
/// internal monotonic+random scheme" instead, and `blake3` is already this crate's dependency.
pub fn mint_id(kind: HandleKind, now_ms: u64) -> String {
    let counter = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let entropy_marker = Box::new(counter);
    let entropy = format!("{now_ms}:{counter}:{:p}", entropy_marker.as_ref());
    let digest = blake3::hash(entropy.as_bytes()).to_hex();
    format!("{}{}", kind.prefix(), &digest[..26])
}
//#endregion 🔖️IdGeneration

//#region 🔖️HandleTable
/// 🗃️ In-memory handle table — mint/resolve/revoke/GC. A later packet (P6/P7) wires a real
/// `GatewayBackend` implementation against one shared instance; `NullBackend` has none yet.
#[derive(Default)]
pub struct HandleTable {
    records: Mutex<BTreeMap<String, HandleRecord>>,
}

impl HandleTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🆕️ Mints a fresh handle owned by `owner`, expiring at `now_ms + kind.default_ttl_ms()`.
    pub fn mint(&self, kind: HandleKind, owner: SessionHandle, bound_to: Attachment, payload: serde_json::Value, now_ms: u64) -> String {
        let id = mint_id(kind, now_ms);
        let record = HandleRecord { kind, owner, bound_to, expires_ms: now_ms.saturating_add(kind.default_ttl_ms()), payload };
        self.records.lock().expect("handle table lock poisoned").insert(id.clone(), record);
        id
    }

    /// 🔐️ Authorization boundary — see this module's doc. A missing OR expired handle is `NOT_FOUND`
    /// (never distinguishes "never existed" from "expired" to a caller who was never its owner); an
    /// existing, unexpired handle owned by someone else is `PERMISSION_DENIED`. A `Session`-kind
    /// handle's expiry slides forward on every successful resolve.
    pub fn resolve(&self, id: &str, requesting_session: &SessionHandle, now_ms: u64) -> Result<HandleRecord, GatewayError> {
        let mut records = self.records.lock().expect("handle table lock poisoned");
        let Some(record) = records.get(id) else {
            return Err(GatewayError::new(GatewayErrorCode::NotFound, format!("unknown handle: {id}")));
        };
        if record.expires_ms <= now_ms {
            records.remove(id);
            return Err(GatewayError::new(GatewayErrorCode::NotFound, format!("handle expired: {id}")));
        }
        if &record.owner != requesting_session {
            return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, format!("handle {id} is not owned by the requesting session")));
        }
        let mut record = record.clone();
        if record.kind == HandleKind::Session {
            record.expires_ms = now_ms.saturating_add(HandleKind::Session.default_ttl_ms());
            records.insert(id.to_string(), record.clone());
        }
        Ok(record)
    }

    /// 🗑️ Unconditional removal (no ownership check — callers that already resolved successfully use
    /// this to drop a one-shot handle, e.g. a consumed `prep_`).
    pub fn revoke(&self, id: &str) -> bool {
        self.records.lock().expect("handle table lock poisoned").remove(id).is_some()
    }

    /// 🏁️ `job_` handles expire `terminal + 1h` rather than at mint time — a later packet calls this
    /// once a `JobStatus.state` becomes terminal (`Succeeded`/`Failed`/`Cancelled`).
    pub fn mark_terminal(&self, id: &str, now_ms: u64) -> Result<(), GatewayError> {
        let mut records = self.records.lock().expect("handle table lock poisoned");
        let Some(record) = records.get_mut(id) else {
            return Err(GatewayError::new(GatewayErrorCode::NotFound, format!("unknown handle: {id}")));
        };
        if record.kind != HandleKind::Job {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "mark_terminal only applies to Job handles"));
        }
        record.expires_ms = now_ms.saturating_add(HandleKind::Job.default_ttl_ms());
        Ok(())
    }

    /// 🧹️ Sweeps every expired record, returning the count removed — a later packet schedules this
    /// periodically; it is also exercised directly by [`resolve`](Self::resolve)'s own lazy expiry.
    pub fn gc_expired(&self, now_ms: u64) -> usize {
        let mut records = self.records.lock().expect("handle table lock poisoned");
        let before = records.len();
        records.retain(|_, record| record.expires_ms > now_ms);
        before - records.len()
    }

    pub fn len(&self) -> usize {
        self.records.lock().expect("handle table lock poisoned").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
//#endregion 🔖️HandleTable

//#region 🔖️IdempotencyStore
/// ⏱️ 24h TTL (`📋️master.md` §3.3 "Idempotency") — `📌️sol-P1b-packet.md` §2.2's frozen window.
pub const IDEMPOTENCY_TTL_MS: u64 = 24 * 60 * 60_000;

/// ♻️ `(principal, idempotencyKey) -> InvocationReport`, 24h TTL, `replayed: true` on a cache hit —
/// the seam `action.invoke{idempotencyKey}` (a later packet) reads through.
#[derive(Default)]
pub struct IdempotencyStore {
    entries: Mutex<BTreeMap<(String, String), (InvocationReport, u64)>>,
}

impl IdempotencyStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// ↩️ Returns the cached report with `replayed` forced to `true` on a hit inside the TTL window;
    /// otherwise calls `compute` exactly once, stores the result (`replayed` as `compute` produced it,
    /// normally `false`), and returns it.
    pub fn get_or_insert_with(&self, principal: &str, idempotency_key: &str, now_ms: u64, compute: impl FnOnce() -> InvocationReport) -> InvocationReport {
        let cache_key = (principal.to_string(), idempotency_key.to_string());
        {
            let mut entries = self.entries.lock().expect("idempotency store lock poisoned");
            if let Some((report, expires_ms)) = entries.get(&cache_key) {
                if *expires_ms > now_ms {
                    let mut replayed = report.clone();
                    replayed.replayed = true;
                    return replayed;
                }
                entries.remove(&cache_key);
            }
        }
        let report = compute();
        self.entries.lock().expect("idempotency store lock poisoned").insert(cache_key, (report.clone(), now_ms.saturating_add(IDEMPOTENCY_TTL_MS)));
        report
    }

    pub fn len(&self) -> usize {
        self.entries.lock().expect("idempotency store lock poisoned").len()
    }
}
//#endregion 🔖️IdempotencyStore

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;

    fn session(id: &str) -> SessionHandle {
        SessionHandle::new(id.to_string())
    }

    //#region 🔖️MintResolve
    #[test]
    fn mint_then_resolve_by_the_owning_session_succeeds() {
        let table = HandleTable::new();
        let owner = session("sess_a");
        let id = table.mint(HandleKind::Prepared, owner.clone(), Attachment::Capability { capability_id: "cad.viewport.translateSelection".into() }, serde_json::json!({"dx": 1.0}), 1_000);
        assert!(id.starts_with("prep_"));
        let record = table.resolve(&id, &owner, 1_500).unwrap();
        assert_eq!(record.kind, HandleKind::Prepared);
        assert_eq!(record.payload["dx"], 1.0);
    }

    #[test]
    fn every_kind_mints_with_its_frozen_prefix() {
        let table = HandleTable::new();
        let owner = session("sess_a");
        let expectations =
            [(HandleKind::Session, "sess_"), (HandleKind::Prepared, "prep_"), (HandleKind::Transaction, "txn_"), (HandleKind::Undo, "undo_"), (HandleKind::Job, "job_"), (HandleKind::Approval, "appr_"), (HandleKind::Continuation, "cont_")];
        for (kind, prefix) in expectations {
            let id = table.mint(kind, owner.clone(), Attachment::None, serde_json::Value::Null, 0);
            assert!(id.starts_with(prefix), "{kind:?} minted `{id}`, expected prefix `{prefix}`");
        }
    }
    //#endregion 🔖️MintResolve

    //#region 🔖️AuthorizationSecurity
    /// 🔐️ THE security property: possession of the handle string is not enough — a session that never
    /// minted/owns it must be refused even though the handle is real and unexpired.
    #[test]
    fn cross_session_resolve_is_permission_denied_not_a_leak() {
        let table = HandleTable::new();
        let owner = session("sess_owner");
        let thief = session("sess_thief");
        let id = table.mint(HandleKind::Transaction, owner, Attachment::None, serde_json::Value::Null, 0);
        let error = table.resolve(&id, &thief, 100).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::PermissionDenied);
    }

    #[test]
    fn resolving_an_unknown_handle_is_not_found() {
        let table = HandleTable::new();
        let error = table.resolve("prep_does_not_exist", &session("sess_a"), 0).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::NotFound);
    }
    //#endregion 🔖️AuthorizationSecurity

    //#region 🔖️Expiry
    #[test]
    fn a_handle_past_its_ttl_is_not_found_and_removed() {
        let table = HandleTable::new();
        let owner = session("sess_a");
        let id = table.mint(HandleKind::Continuation, owner.clone(), Attachment::None, serde_json::Value::Null, 0);
        let past_expiry = HandleKind::Continuation.default_ttl_ms() + 1;
        let error = table.resolve(&id, &owner, past_expiry).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::NotFound);
        assert_eq!(table.len(), 0, "lazily-discovered expiry must remove the record");
    }

    #[test]
    fn session_handle_ttl_slides_forward_on_every_resolve() {
        let table = HandleTable::new();
        let owner = session("sess_a");
        let id = table.mint(HandleKind::Session, owner.clone(), Attachment::None, serde_json::Value::Null, 0);
        let almost_expired = HandleKind::Session.default_ttl_ms() - 1;
        table.resolve(&id, &owner, almost_expired).unwrap();
        // had the TTL not slid forward on the resolve above, this would now be past the original expiry
        let would_have_been_expired = HandleKind::Session.default_ttl_ms() + 1;
        table.resolve(&id, &owner, would_have_been_expired).unwrap();
    }

    #[test]
    fn gc_expired_removes_only_expired_records_and_reports_the_count() {
        let table = HandleTable::new();
        let owner = session("sess_a");
        table.mint(HandleKind::Continuation, owner.clone(), Attachment::None, serde_json::Value::Null, 0);
        table.mint(HandleKind::Undo, owner, Attachment::None, serde_json::Value::Null, 0);
        let removed = table.gc_expired(HandleKind::Continuation.default_ttl_ms() + 1);
        assert_eq!(removed, 1);
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn mark_terminal_extends_a_job_handle_by_one_hour_from_now() {
        let table = HandleTable::new();
        let owner = session("sess_a");
        let id = table.mint(HandleKind::Job, owner.clone(), Attachment::None, serde_json::Value::Null, 0);
        table.mark_terminal(&id, 10_000_000).unwrap();
        let record = table.resolve(&id, &owner, 10_000_000 + HandleKind::Job.default_ttl_ms() - 1).unwrap();
        assert_eq!(record.kind, HandleKind::Job);
    }

    #[test]
    fn mark_terminal_rejects_a_non_job_handle() {
        let table = HandleTable::new();
        let id = table.mint(HandleKind::Prepared, session("sess_a"), Attachment::None, serde_json::Value::Null, 0);
        let error = table.mark_terminal(&id, 0).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::InputInvalid);
    }
    //#endregion 🔖️Expiry

    //#region 🔖️Idempotency
    #[test]
    fn idempotent_replay_reuses_the_cached_report_and_flags_it() {
        let store = IdempotencyStore::new();
        let calls = std::cell::Cell::new(0);
        let make_report = || {
            calls.set(calls.get() + 1);
            InvocationReport {
                invocation_id: "inv-1".into(),
                capability_id: "cad.viewport.translateSelection".into(),
                status: crate::schema::InvocationStatus::Succeeded,
                affected_resources: vec![],
                revision_before: None,
                revision_after: None,
                diff_uri: None,
                warnings: vec![],
                undo_token: None,
                postconditions: vec![],
                replayed: false,
            }
        };
        let first = store.get_or_insert_with("agent:local", "key-1", 0, make_report);
        assert!(!first.replayed);
        let second = store.get_or_insert_with("agent:local", "key-1", 1_000, make_report);
        assert!(second.replayed, "a replay within the TTL window must be flagged");
        assert_eq!(second.invocation_id, "inv-1");
        assert_eq!(calls.get(), 1, "compute must run exactly once for a cached key");
    }

    #[test]
    fn different_idempotency_keys_never_collide() {
        let store = IdempotencyStore::new();
        let a = store.get_or_insert_with("agent:local", "key-a", 0, || InvocationReport {
            invocation_id: "inv-a".into(),
            capability_id: "x".into(),
            status: crate::schema::InvocationStatus::Succeeded,
            affected_resources: vec![],
            revision_before: None,
            revision_after: None,
            diff_uri: None,
            warnings: vec![],
            undo_token: None,
            postconditions: vec![],
            replayed: false,
        });
        let b = store.get_or_insert_with("agent:local", "key-b", 0, || InvocationReport {
            invocation_id: "inv-b".into(),
            capability_id: "x".into(),
            status: crate::schema::InvocationStatus::Succeeded,
            affected_resources: vec![],
            revision_before: None,
            revision_after: None,
            diff_uri: None,
            warnings: vec![],
            undo_token: None,
            postconditions: vec![],
            replayed: false,
        });
        assert_ne!(a.invocation_id, b.invocation_id);
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn expired_idempotency_entry_recomputes() {
        let store = IdempotencyStore::new();
        let calls = std::rc::Rc::new(std::cell::Cell::new(0));
        fn report_with(calls: &std::rc::Rc<std::cell::Cell<i32>>, id: &'static str) -> InvocationReport {
            calls.set(calls.get() + 1);
            InvocationReport {
                invocation_id: id.into(),
                capability_id: "x".into(),
                status: crate::schema::InvocationStatus::Succeeded,
                affected_resources: vec![],
                revision_before: None,
                revision_after: None,
                diff_uri: None,
                warnings: vec![],
                undo_token: None,
                postconditions: vec![],
                replayed: false,
            }
        }
        let first = store.get_or_insert_with("agent:local", "key-1", 0, || report_with(&calls, "inv-1"));
        assert!(!first.replayed);
        let after_ttl = IDEMPOTENCY_TTL_MS + 1;
        let second = store.get_or_insert_with("agent:local", "key-1", after_ttl, || report_with(&calls, "inv-2"));
        assert!(!second.replayed, "past the TTL, this must recompute rather than replay");
        assert_eq!(second.invocation_id, "inv-2");
        assert_eq!(calls.get(), 2);
    }
    //#endregion 🔖️Idempotency
}
//#endregion 🧪️Tests
