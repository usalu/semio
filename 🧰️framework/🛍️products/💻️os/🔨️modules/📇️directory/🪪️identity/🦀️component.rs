//! 🪪️ Native session mint-or-restore helper (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-
//! COLLABORATIVE-STUDIOS, contract §C0/§C3). This is a SIBLING of, not the same thing as, the
//! CQRS `os.config.identity` config facet lane 1-C owns at
//! `💻️os/🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️component.ts` (self-contained there, per 1-C's
//! own `📓️w1-c-report.md` design-decision note — `💻️os/🎚️config/**` is peer-leased to 1-C per this
//! ticket's `📋️ownership-and-handoffs.md` §A) — that facet is the browser/React shell's persisted-
//! local-only op log. The wgpu NATIVE shell reads `S_*` env vars directly instead
//! (`📓️scout-client.md` §4/§7: "wgpu native reads `S_*` directly"), so it needs a plain restore-
//! or-mint-then-cache helper, not an event log — this module is that helper, built on
//! `../🔌️client/🦀️component.rs`'s `DirectoryClient`. The `Identity` shape below mirrors contract
//! §C3's `Identity { userId, email, displayName, hubBaseUrl, sessionToken, issuedAtMs }` field-
//! for-field (re-declared here for the same reason `🧬️schema/🦀️component.rs`'s header re-declares
//! `DirectorySpaceKind` — the owning module is not reachable from this lease). Cross-checked
//! field-for-field against 1-C's independently-built TS `DirectoryClient`
//! (`SessionView`/`SessionMintResponse` shapes, `HUB_RECONNECT_MIN_MS`/`MAX_MS`) — no drift, per
//! 1-C's own report.

use super::client::{DirectoryClient, DirectoryClientError, DirectoryTransport};
use semio_framework_async::OperationContext;

//#region 🔖️Identity
/// 🪪️ One restored-or-minted session, matching contract §C3's `Identity` field-for-field.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub hub_base_url: String,
    pub session_token: String,
    pub issued_at_ms: i64,
}

/// 🎭️ `user:{userId}#{sessionId}` (contract §C0) — `session_id` is the caller's own per-tab/
/// per-process id (e.g. wgpu's `session.instance_id`), never derived here.
pub fn actor_id(identity: &Identity, session_id: &str) -> String {
    format!("user:{}#{session_id}", identity.user_id)
}

/// 📶️ Whether `mint_or_restore` reached the hub this call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityStatus {
    Online,
    Offline,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentityOutcome {
    pub identity: Identity,
    pub status: IdentityStatus,
}

#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("hub unreachable and no cached identity for {0}")]
    Unavailable(String),
}
//#endregion 🔖️Identity

//#region 🔖️Env
/// 🌱️ `S_HUB_URL` / `S_USER` / `S_DATA_DIR` (contract §C0). `data_dir` is optional — a caller
/// with no `S_DATA_DIR` (e.g. an e2e harness) simply gets no cache, minting fresh every boot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentityEnv {
    pub hub_url: String,
    pub user_email: String,
    pub data_dir: Option<std::path::PathBuf>,
}

impl IdentityEnv {
    pub fn from_process_env() -> Option<Self> {
        let hub_url = std::env::var("S_HUB_URL").ok()?;
        let user_email = std::env::var("S_USER").ok()?;
        let data_dir = std::env::var("S_DATA_DIR").ok().map(std::path::PathBuf::from);
        Some(Self { hub_url, user_email, data_dir })
    }
}
//#endregion 🔖️Env

//#region 🔖️Cache
#[cfg(not(target_arch = "wasm32"))]
mod cache {
    use super::Identity;
    use std::path::{Path, PathBuf};

    fn path(data_dir: &Path) -> PathBuf {
        data_dir.join("os").join("🪪️identity.json")
    }

    pub fn load(data_dir: &Path) -> Option<Identity> {
        let bytes = std::fs::read(path(data_dir)).ok()?;
        serde_json::from_slice(&bytes).ok()
    }

    pub fn save(data_dir: &Path, identity: &Identity) {
        let target = path(data_dir);
        let Some(parent) = target.parent() else { return };
        if std::fs::create_dir_all(parent).is_err() {
            return;
        }
        if let Ok(bytes) = serde_json::to_vec_pretty(identity) {
            let _ = std::fs::write(target, bytes);
        }
    }
}

/// 🌉️ Documented seam (matches this lane's brief): the browser wgpu build has no filesystem, and
/// production browser sessions restore identity through lane 1-C's `os.config.identity` facet
/// instead — this cache never runs there. A future in-wasm host wires a real cache (IndexedDB via
/// `web_sys`) here; kept so `mint_or_restore` stays one cross-target function.
#[cfg(target_arch = "wasm32")]
mod cache {
    use super::Identity;
    pub fn load(_data_dir: &std::path::Path) -> Option<Identity> {
        None
    }
    pub fn save(_data_dir: &std::path::Path, _identity: &Identity) {}
}
//#endregion 🔖️Cache

//#region 🔖️MintOrRestore
/// ⏰️ Millisecond wall-clock read, native `SystemTime` / wasm32 `js_sys::Date` — same split
/// `🏪️store/🔄️sync`'s own `now_ms` already uses.
#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
}

#[cfg(target_arch = "wasm32")]
fn now_ms() -> i64 {
    js_sys::Date::now() as i64
}

fn persist(env: &IdentityEnv, identity: &Identity) {
    if let Some(data_dir) = &env.data_dir {
        cache::save(data_dir, identity);
    }
}

/// 🚪️ Boot flow (contract §C3): restore the cached identity and confirm it with
/// `GET /auth/sessions/me`; a clean 401 falls through to `POST /auth/sessions {email}`; any
/// OTHER transport failure degrades to the last cached identity marked `Offline` rather than
/// blocking or panicking. With no cache at all, a mint failure has nothing to degrade to and
/// surfaces `IdentityError::Unavailable`. `ctx` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
/// RUNTIME) is threaded into BOTH hub calls unchanged — cancelling it (e.g. on process shutdown,
/// mid-boot) stops identity bootstrap the same way it stops any other directory request, rather
/// than this path inventing its own bolted-on cancellation. The caller owns `ctx`'s lifetime
/// (typically one scoped to "this boot attempt" or "this process"), same as every other
/// `DirectoryClient` call site.
pub async fn mint_or_restore<T: DirectoryTransport>(ctx: &OperationContext, client: &DirectoryClient<T>, env: &IdentityEnv) -> Result<IdentityOutcome, IdentityError> {
    let cached = env.data_dir.as_deref().and_then(cache::load);

    if let Some(cached_identity) = &cached {
        client.set_token(Some(cached_identity.session_token.clone()));
        match client.me(ctx).await {
            Ok(session) => {
                let identity = Identity {
                    user_id: session.user_id,
                    email: session.email,
                    display_name: session.display_name,
                    hub_base_url: env.hub_url.clone(),
                    session_token: cached_identity.session_token.clone(),
                    issued_at_ms: cached_identity.issued_at_ms,
                };
                persist(env, &identity);
                return Ok(IdentityOutcome { identity, status: IdentityStatus::Online });
            }
            Err(DirectoryClientError::Unauthorized) => {}
            Err(_) => return Ok(IdentityOutcome { identity: cached_identity.clone(), status: IdentityStatus::Offline }),
        }
    }

    match client.mint_session(ctx, &env.user_email).await {
        Ok(minted) => {
            let identity = Identity {
                user_id: minted.user_id,
                email: env.user_email.clone(),
                display_name: env.user_email.clone(),
                hub_base_url: env.hub_url.clone(),
                session_token: minted.token,
                issued_at_ms: now_ms(),
            };
            client.set_token(Some(identity.session_token.clone()));
            persist(env, &identity);
            Ok(IdentityOutcome { identity, status: IdentityStatus::Online })
        }
        Err(_) => cached.map(|identity| IdentityOutcome { identity, status: IdentityStatus::Offline }).ok_or_else(|| IdentityError::Unavailable(env.user_email.clone())),
    }
}
//#endregion 🔖️MintOrRestore

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::super::client::test_support::FakeTransport;
    use super::super::client::HttpResponse;
    use super::*;
    use semio_framework_async::{CancelToken, TraceId};

    fn env(data_dir: &std::path::Path) -> IdentityEnv {
        IdentityEnv { hub_url: "http://hub.local".to_string(), user_email: "amara@semio.dev".to_string(), data_dir: Some(data_dir.to_path_buf()) }
    }

    fn root_ctx() -> OperationContext {
        OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: CancelToken::root(), capability: None }
    }

    #[test]
    fn actor_id_matches_contract_grammar() {
        let identity = Identity { user_id: "u-amara".to_string(), email: "amara@semio.dev".to_string(), display_name: "Amara".to_string(), hub_base_url: "http://hub.local".to_string(), session_token: "tok".to_string(), issued_at_ms: 0 };
        assert_eq!(actor_id(&identity, "sess-1"), "user:u-amara#sess-1");
    }

    #[test]
    fn no_cache_mints_a_fresh_session() {
        let dir = tempfile::tempdir().expect("tempdir");
        let transport = FakeTransport::default();
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!({ "token": "tok-new", "user_id": "u-1" })));
        let client = DirectoryClient::new(transport.clone(), "http://hub.local");

        let outcome = futures_lite::future::block_on(mint_or_restore(&root_ctx(), &client, &env(dir.path()))).expect("mints");
        assert_eq!(outcome.status, IdentityStatus::Online);
        assert_eq!(outcome.identity.session_token, "tok-new");
        assert_eq!(transport.requests.lock().unwrap().len(), 1, "restore is skipped entirely with no cache");
        assert_eq!(cache::load(dir.path()).expect("cached").session_token, "tok-new");
    }

    #[test]
    fn valid_cache_restores_without_minting() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cached = Identity { user_id: "u-1".to_string(), email: "amara@semio.dev".to_string(), display_name: "Amara".to_string(), hub_base_url: "http://hub.local".to_string(), session_token: "tok-old".to_string(), issued_at_ms: 111 };
        cache::save(dir.path(), &cached);
        let transport = FakeTransport::default();
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!({ "userId": "u-1", "email": "amara@semio.dev", "displayName": "Amara", "expiresAt": 999 })));
        let client = DirectoryClient::new(transport.clone(), "http://hub.local");

        let outcome = futures_lite::future::block_on(mint_or_restore(&root_ctx(), &client, &env(dir.path()))).expect("restores");
        assert_eq!(outcome.status, IdentityStatus::Online);
        assert_eq!(outcome.identity.session_token, "tok-old", "restore keeps the cached token, /me only confirms it");
        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].url, "http://hub.local/auth/sessions/me");
    }

    #[test]
    fn expired_cache_falls_through_to_mint() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cached = Identity { user_id: "u-1".to_string(), email: "amara@semio.dev".to_string(), display_name: "Amara".to_string(), hub_base_url: "http://hub.local".to_string(), session_token: "tok-expired".to_string(), issued_at_ms: 111 };
        cache::save(dir.path(), &cached);
        let transport = FakeTransport::default();
        transport.push_response(Ok(HttpResponse { status: 401, body: Vec::new() }));
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!({ "token": "tok-fresh", "user_id": "u-1" })));
        let client = DirectoryClient::new(transport.clone(), "http://hub.local");

        let outcome = futures_lite::future::block_on(mint_or_restore(&root_ctx(), &client, &env(dir.path()))).expect("mints after 401");
        assert_eq!(outcome.status, IdentityStatus::Online);
        assert_eq!(outcome.identity.session_token, "tok-fresh");
        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests.len(), 2, "one failed /me, one successful mint");
        assert_eq!(requests[1].url, "http://hub.local/auth/sessions");
    }

    #[test]
    fn unreachable_hub_degrades_to_cached_identity_offline() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cached = Identity { user_id: "u-1".to_string(), email: "amara@semio.dev".to_string(), display_name: "Amara".to_string(), hub_base_url: "http://hub.local".to_string(), session_token: "tok-old".to_string(), issued_at_ms: 111 };
        cache::save(dir.path(), &cached);
        let transport = FakeTransport::default();
        transport.push_response(Err(super::super::client::TransportError::Io("connection refused".to_string())));
        let client = DirectoryClient::new(transport, "http://hub.local");

        let outcome = futures_lite::future::block_on(mint_or_restore(&root_ctx(), &client, &env(dir.path()))).expect("degrades, never errors, while a cache exists");
        assert_eq!(outcome.status, IdentityStatus::Offline);
        assert_eq!(outcome.identity, cached, "the stale identity is returned as-is, never mutated");
    }

    #[test]
    fn unreachable_hub_with_no_cache_is_unavailable_not_a_panic() {
        let dir = tempfile::tempdir().expect("tempdir");
        let transport = FakeTransport::default();
        transport.push_response(Err(super::super::client::TransportError::Io("connection refused".to_string())));
        let client = DirectoryClient::new(transport, "http://hub.local");

        let error = futures_lite::future::block_on(mint_or_restore(&root_ctx(), &client, &env(dir.path()))).expect_err("no cache and no hub leaves nothing to restore");
        assert!(matches!(error, IdentityError::Unavailable(email) if email == "amara@semio.dev"));
    }
}
//#endregion 🧪️Tests
