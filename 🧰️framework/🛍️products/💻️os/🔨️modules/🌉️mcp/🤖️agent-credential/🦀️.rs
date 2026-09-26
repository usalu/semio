//! 🤖️ The agent's own hub credential: how `semio-os-mcp --hub <url> --space <id>` authenticates as
//! an **agent principal** rather than as the human who started it.
//!
//! The secret never travels in argv, in an environment variable or in a URL — the three carriers
//! `🏗️bootstrap/🦀️.rs`'s process-entry seal exists to keep hub authority out of (M1). It is read
//! from a file the human wrote (`--credential-file <path>`, mode 0600), or from an inherited file descriptor
//! (`--credential-fd <n>`), and it is wiped as soon as it has been exchanged.
//!
//! The file is the one-time receipt of `POST /auth/agent-delegations`. This module reads it,
//! exchanges it at `POST /auth/agent-sessions` for an ordinary session capability, and hands that
//! to the same `HeadlessWorkspace::open_hub` a `dev s`-spawned gateway uses — so nothing downstream
//! of the credential needs to know an agent is driving.

use crate::{GatewayError, GatewayErrorCode};

/// 🧬️ The exact credential-file shape, written once by whoever created the delegation.
pub const AGENT_CREDENTIAL_SCHEMA: &str = "semio.hub.agent-credential/v1";

/// 📏️ The largest credential file this process will read. A delegation receipt is a few hundred
/// bytes; anything larger is a mistake or an attack, never a credential.
pub const AGENT_CREDENTIAL_MAX_BYTES: u64 = 16 * 1024;

/// 🤖️ One agent credential, decoded and bounds-checked. `Debug` never prints the token and `Drop`
/// wipes it, exactly like `LocalHubCredential`.
pub struct AgentCredentialV1 {
    hub_origin: String,
    space_id: String,
    audience: String,
    token: String,
}

impl std::fmt::Debug for AgentCredentialV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("AgentCredentialV1").field("hub_origin", &self.hub_origin).field("space_id", &self.space_id).field("audience", &self.audience).field("token", &"<redacted>").finish()
    }
}

impl Drop for AgentCredentialV1 {
    fn drop(&mut self) {
        let bytes = unsafe { self.token.as_bytes_mut() };
        bytes.fill(0);
    }
}

impl AgentCredentialV1 {
    pub fn hub_origin(&self) -> &str {
        &self.hub_origin
    }

    pub fn space_id(&self) -> &str {
        &self.space_id
    }

    pub fn audience(&self) -> &str {
        &self.audience
    }

    /// 🔓️ The delegation token. Only the exchange call reads this, and only once.
    pub fn expose_for_exchange(&self) -> &str {
        &self.token
    }

    /// 📄️ Decodes one credential document. Every bound is checked here so a malformed file is a
    /// typed refusal at startup rather than a confusing `401` three calls later.
    pub fn decode(bytes: &[u8]) -> Result<Self, GatewayError> {
        let malformed = |what: &str| GatewayError::new(GatewayErrorCode::InputInvalid, format!("agent credential is malformed: {what}"));
        if bytes.is_empty() || bytes.len() as u64 > AGENT_CREDENTIAL_MAX_BYTES {
            return Err(malformed("empty or larger than 16 KiB"));
        }
        let document: serde_json::Value = serde_json::from_slice(bytes).map_err(|_| malformed("not JSON"))?;
        let field = |name: &str| document.get(name).and_then(serde_json::Value::as_str).map(str::to_string).ok_or_else(|| malformed(&format!("`{name}` is missing or not a string")));
        if field("schema")? != AGENT_CREDENTIAL_SCHEMA {
            return Err(malformed("`schema` is not `semio.hub.agent-credential/v1`"));
        }
        let hub_origin = field("hubOrigin")?;
        let space_id = field("spaceId")?;
        let audience = field("audience")?;
        let token = field("token")?;
        if hub_origin.is_empty() || space_id.is_empty() {
            return Err(malformed("`hubOrigin` and `spaceId` must be non-empty"));
        }
        if !matches!(audience.as_str(), "read" | "edit") {
            return Err(malformed("`audience` must be `read` or `edit`"));
        }
        if !valid_delegation_token(&token) {
            return Err(malformed("`token` is not a `delegation.v1.<32 hex>.<64 hex>` capability"));
        }
        Ok(Self { hub_origin, space_id, audience, token })
    }

    /// 📂️ Reads the credential from a path. The file must be regular, within the size bound and
    /// readable by its owner alone (POSIX mode, Windows DACL — `🔐️owner-only`): a credential a group or the world can read is not a
    /// credential, and refusing it at startup is the only moment anyone will notice.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_file(path: &std::path::Path) -> Result<Self, GatewayError> {
        let metadata = std::fs::metadata(path).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, format!("agent credential `{}` is unreadable: {error}", path.display())))?;
        if !metadata.is_file() {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("agent credential `{}` is not a regular file", path.display())));
        }
        if metadata.len() > AGENT_CREDENTIAL_MAX_BYTES {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("agent credential `{}` is larger than 16 KiB", path.display())));
        }
        crate::owner_only::verify_owner_only(path).map_err(|reason| GatewayError::new(GatewayErrorCode::PermissionDenied, format!("agent credential `{}` {reason}", path.display())))?;
        let mut bytes = std::fs::read(path).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, format!("agent credential `{}` is unreadable: {error}", path.display())))?;
        let credential = Self::decode(&bytes);
        bytes.fill(0);
        credential
    }

    /// 🔢️ Reads the credential from an inherited file descriptor — the stdin-handle shape, for a
    /// supervisor that would rather pipe the credential than leave it on disk at all. The
    /// descriptor is read to EOF and closed; `0` is admissible only in `http` mode, where this
    /// process's stdin is not the MCP framing channel.
    #[cfg(unix)]
    pub fn read_fd(fd: i32) -> Result<Self, GatewayError> {
        use std::io::Read;
        use std::os::fd::FromRawFd;
        if fd < 0 {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "agent credential descriptor must be non-negative"));
        }
        let mut file = unsafe { std::fs::File::from_raw_fd(fd) };
        let mut bytes = Vec::new();
        let read = file.by_ref().take(AGENT_CREDENTIAL_MAX_BYTES + 1).read_to_end(&mut bytes);
        let outcome = match read {
            Ok(_) if bytes.len() as u64 <= AGENT_CREDENTIAL_MAX_BYTES => Self::decode(&bytes),
            Ok(_) => Err(GatewayError::new(GatewayErrorCode::InputInvalid, "agent credential descriptor delivered more than 16 KiB")),
            Err(error) => Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("agent credential descriptor is unreadable: {error}"))),
        };
        bytes.fill(0);
        outcome
    }
}

/// 🔑️ `delegation.v1.<32 lower-hex selector>.<64 lower-hex secret>` — the exact shape
/// `AgentDelegationCapability::expose_once` produces, checked here so a truncated paste fails at
/// startup instead of as an opaque `401`.
pub fn valid_delegation_token(value: &str) -> bool {
    let lower_hex = |bytes: &[u8]| bytes.iter().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte));
    let bytes = value.as_bytes();
    bytes.len() == 111 && bytes.get(..14) == Some(b"delegation.v1.".as_slice()) && bytes.get(14..46).is_some_and(lower_hex) && bytes.get(46) == Some(&b'.') && bytes.get(47..111).is_some_and(lower_hex)
}

/// 🧾️ The `POST /auth/agent-sessions` request body this process sends. `agentInstanceId`
/// distinguishes one MCP process from another under the same delegation, so two agents sharing a
/// credential still get separate sessions and separate presence rows.
pub fn agent_session_request_body(audience: &str, agent_instance_id: &str) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "schema": "semio.hub.auth.agent-session/v1",
        "audience": audience,
        "agentInstanceId": agent_instance_id,
    }))
    .unwrap_or_default()
}

/// 🎁️ The one field of the exchange response this process keeps, plus the identity it reports.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentSessionGrantV1 {
    pub token: String,
    pub agent_principal_id: String,
    pub agent_label: String,
    pub space_id: String,
    pub expires_at_ms: i64,
}

/// 🎁️ Decodes the `POST /auth/agent-sessions` 200 body. A `4xx` body is decoded by
/// [`agent_exchange_error`] instead, so a refusal never silently reads as a grant.
pub fn decode_agent_session_grant(bytes: &[u8]) -> Result<AgentSessionGrantV1, GatewayError> {
    let malformed = || GatewayError::new(GatewayErrorCode::Internal, "the hub's agent-session response is not `semio.hub.auth.agent-session/v1`");
    let document: serde_json::Value = serde_json::from_slice(bytes).map_err(|_| malformed())?;
    let field = |name: &str| document.get(name).and_then(serde_json::Value::as_str).map(str::to_string).ok_or_else(malformed);
    Ok(AgentSessionGrantV1 {
        token: field("token")?,
        agent_principal_id: field("agentPrincipalId")?,
        agent_label: field("agentLabel")?,
        space_id: field("spaceId")?,
        expires_at_ms: document.get("expiresAtMs").and_then(serde_json::Value::as_i64).ok_or_else(malformed)?,
    })
}

/// 🛑️ Turns a non-200 exchange into a typed gateway error that names the hub's own refusal code,
/// so an operator reads `delegation-revoked` rather than `401`.
pub fn agent_exchange_error(status: u16, body: &[u8]) -> GatewayError {
    let code = serde_json::from_slice::<serde_json::Value>(body).ok().and_then(|document| document.get("error").and_then(serde_json::Value::as_str).map(str::to_string));
    let detail = code.unwrap_or_else(|| format!("http-{status}"));
    let gateway_code = match detail.as_str() {
        "delegation-revoked" | "delegation-expired" | "forbidden" => GatewayErrorCode::PermissionDenied,
        "rate-limited" | "directory-unavailable" => GatewayErrorCode::PluginUnavailable,
        _ => GatewayErrorCode::PermissionDenied,
    };
    let error = GatewayError::new(gateway_code, format!("the hub refused this agent delegation: {detail}"));
    if matches!(detail.as_str(), "rate-limited" | "directory-unavailable") { error.retryable() } else { error }
}

#[cfg(test)]
#[path = "../🧪️tests/🤖️agent-credential/🦀️.rs"]
mod tests;
