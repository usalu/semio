//! 🪪️ Native identity bootstrap from an owned one-shot local credential endpoint.

use super::client::{DirectoryClient, DirectoryTransport, LocalHubCredential};
use semio_framework_async::OperationContext;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;

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

#[derive(Debug)]
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
fn now_ms() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| i64::try_from(duration.as_millis()).unwrap_or(i64::MAX))
}

/// 🪪️ Native-only like `LocalHubCredential::read_inherited` — the credential arrives on an inherited
/// pipe, which no wasm target has.
#[cfg(not(target_arch = "wasm32"))]
pub async fn restore_inherited<T: DirectoryTransport>(ctx: &OperationContext, transport: T) -> Result<IdentityOutcome, IdentityError> {
    let credential = Arc::new(LocalHubCredential::read_inherited("native").map_err(|_| IdentityError)?);
    let client = DirectoryClient::authenticated(transport, credential.clone());
    let session = client.me(ctx).await.map_err(|_| IdentityError)?;
    Ok(IdentityOutcome {
        identity: Identity { user_id: session.user_id, email: session.email, display_name: session.display_name, hub_base_url: client.base_url().to_string(), issued_at_ms: now_ms() },
        status: IdentityStatus::Online,
        credential,
    })
}
//#endregion 🔖️Bootstrap

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_non_secret_and_actor_is_server_subject_derived() {
        let identity = Identity { user_id: "u-amara".to_string(), email: "amara@semio.dev".to_string(), display_name: "Amara".to_string(), hub_base_url: "http://127.0.0.1:8787".to_string(), issued_at_ms: 0 };
        assert_eq!(actor_id(&identity, "sess-1"), "user:u-amara#sess-1");
        assert!(!crate::os_pack::json::to_json_string(&identity).contains("token"));
    }
}
//#endregion 🧪️Tests
