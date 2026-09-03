//! 🪪️ Native identity bootstrap from an owned one-shot local credential endpoint.

use super::client::{DirectoryClient, DirectoryTransport, LocalHubCredential};
use semio_framework_async::OperationContext;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::{Arc, OnceLock};

//#region 🔖️Identity
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Identity {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub hub_base_url: String,
    pub issued_at_ms: i64,
}

pub fn actor_id(identity: &Identity, session_id: &str) -> String {
    format!("user:{}#{session_id}", identity.user_id)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityStatus {
    Online,
    Offline,
}

#[derive(Clone, Debug)]
pub struct IdentityOutcome {
    pub identity: Identity,
    pub status: IdentityStatus,
    pub credential: Arc<LocalHubCredential>,
}

#[derive(Clone, Copy, Debug)]
pub struct IdentityError;

impl std::fmt::Display for IdentityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("local credential unavailable")
    }
}

impl std::error::Error for IdentityError {}
//#endregion 🔖️Identity

//#region 🔖️Environment
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentityEnv {
    pub data_dir: Option<std::path::PathBuf>,
}

impl IdentityEnv {
    pub fn from_process_env() -> Option<Self> {
        (std::env::var("S_LOCAL_CREDENTIAL_FD").ok().as_deref() == Some("3")).then(|| Self { data_dir: std::env::var("S_DATA_DIR").ok().map(std::path::PathBuf::from) })
    }
}
//#endregion 🔖️Environment

//#region 🔖️Bootstrap
#[cfg(not(target_arch = "wasm32"))]
struct ClaimedLocalHubCredential {
    client_class: String,
    result: Result<Arc<LocalHubCredential>, IdentityError>,
}

#[cfg(not(target_arch = "wasm32"))]
static CLAIMED_LOCAL_HUB_CREDENTIAL: OnceLock<ClaimedLocalHubCredential> = OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| i64::try_from(duration.as_millis()).unwrap_or(i64::MAX))
}

/// 🪪️ Claims, seals, consumes, and closes fd3 before any plugin or renderer activation.
#[cfg(not(target_arch = "wasm32"))]
pub fn claim_inherited_local_hub_credential(expected_class: &str) -> Result<Arc<LocalHubCredential>, IdentityError> {
    claim_local_hub_credential_with(&CLAIMED_LOCAL_HUB_CREDENTIAL, expected_class, LocalHubCredential::read_inherited)
}

#[cfg(not(target_arch = "wasm32"))]
fn claim_local_hub_credential_with(claimed: &OnceLock<ClaimedLocalHubCredential>, expected_class: &str, read: impl FnOnce(&str) -> Result<LocalHubCredential, super::client::DirectoryClientError>) -> Result<Arc<LocalHubCredential>, IdentityError> {
    let claimed = claimed.get_or_init(|| ClaimedLocalHubCredential { client_class: expected_class.to_string(), result: read(expected_class).map(Arc::new).map_err(|_| IdentityError) });
    if claimed.client_class != expected_class {
        return Err(IdentityError);
    }
    claimed.result.clone()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn claimed_local_hub_credential(expected_class: &str) -> Option<Arc<LocalHubCredential>> {
    CLAIMED_LOCAL_HUB_CREDENTIAL.get().filter(|claimed| claimed.client_class == expected_class).and_then(|claimed| claimed.result.as_ref().ok()).cloned()
}

/// 🪪️ Resolves non-secret identity from the credential already claimed at process entry.
#[cfg(not(target_arch = "wasm32"))]
pub async fn restore_claimed<T: DirectoryTransport>(ctx: &OperationContext, transport: T, expected_class: &str) -> Result<IdentityOutcome, IdentityError> {
    let credential = claimed_local_hub_credential(expected_class).ok_or(IdentityError)?;
    let client = DirectoryClient::authenticated(transport, credential.clone());
    let session = client.me(ctx).await.map_err(|_| IdentityError)?;
    Ok(IdentityOutcome { identity: Identity { user_id: session.user_id, email: session.email, display_name: session.display_name, hub_base_url: client.base_url().to_string(), issued_at_ms: now_ms() }, status: IdentityStatus::Online, credential })
}
//#endregion 🔖️Bootstrap

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Barrier, OnceLock};

    #[test]
    fn identity_is_non_secret_and_actor_is_server_subject_derived() {
        let identity = Identity { user_id: "u-amara".to_string(), email: "amara@semio.dev".to_string(), display_name: "Amara".to_string(), hub_base_url: "http://127.0.0.1:8787".to_string(), issued_at_ms: 0 };
        assert_eq!(actor_id(&identity, "sess-1"), "user:u-amara#sess-1");
        assert!(!crate::os_pack::json::to_json_string(&identity).contains("token"));
    }

    #[test]
    fn credential_claim_is_single_read_class_bound_and_terminal_on_failure() {
        let claimed = Arc::new(OnceLock::new());
        let starts = Arc::new(Barrier::new(3));
        let reads = Arc::new(AtomicUsize::new(0));
        let mut callers = Vec::new();
        for _ in 0..2 {
            let claimed = claimed.clone();
            let starts = starts.clone();
            let reads = reads.clone();
            callers.push(std::thread::spawn(move || {
                starts.wait();
                claim_local_hub_credential_with(&claimed, "native", |_| {
                    reads.fetch_add(1, Ordering::SeqCst);
                    Ok(LocalHubCredential::test("http://127.0.0.1:8787", &format!("session.v1.{}.{:064}", "a".repeat(32), 1)))
                })
            }));
        }
        starts.wait();
        let first = callers.remove(0).join().unwrap().unwrap();
        let second = callers.remove(0).join().unwrap().unwrap();
        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(reads.load(Ordering::SeqCst), 1);
        assert!(claim_local_hub_credential_with(&claimed, "mcp", |_| unreachable!()).is_err());

        let failed = OnceLock::new();
        let failed_reads = AtomicUsize::new(0);
        for _ in 0..2 {
            assert!(claim_local_hub_credential_with(&failed, "native", |_| {
                failed_reads.fetch_add(1, Ordering::SeqCst);
                Err(super::super::client::DirectoryClientError::Unauthorized)
            })
            .is_err());
        }
        assert_eq!(failed_reads.load(Ordering::SeqCst), 1);
    }
}
//#endregion 🧪️Tests
