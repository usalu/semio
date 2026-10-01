//! 🛂️ Hub-owned delegation grammar, session request, response, and refusal mapping.
use semio_framework_os_mcp::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp::agent_credential::{CredentialExchangeProtocolV1, CredentialExchangeRequestV1, DelegatedCredentialV1, DelegatedSessionGrantV1};

struct SecretDocument(serde_json::Value);
impl std::ops::Deref for SecretDocument { type Target = serde_json::Value; fn deref(&self) -> &Self::Target { &self.0 } }
impl Drop for SecretDocument {
    fn drop(&mut self) { if let Some(serde_json::Value::String(token)) = self.0.get_mut("token") { unsafe { token.as_bytes_mut() }.fill(0); } }
}
struct SecretToken(String);
impl std::ops::Deref for SecretToken { type Target = str; fn deref(&self) -> &str { &self.0 } }
impl Drop for SecretToken { fn drop(&mut self) { unsafe { self.0.as_bytes_mut() }.fill(0); } }
impl SecretToken { fn take(&mut self) -> String { std::mem::take(&mut self.0) } }

/// 🧬️ The exact credential schema authored by Hub delegation commands.
pub const AGENT_CREDENTIAL_SCHEMA: &str = "semio.hub.agent-credential/v1";

/// 🔌️ Supplies the Hub protocol to the outward MCP executable's explicit inventory.
pub fn hub_agent_credential_protocol_v1() -> CredentialExchangeProtocolV1 {
    CredentialExchangeProtocolV1 { id: "hub.agent-delegation/v1", credential_schema: AGENT_CREDENTIAL_SCHEMA, decode: decode_hub_agent_credential_v1, request: exchange_request_v1, decode_grant: decode_bound_grant_v1, decode_error: agent_exchange_error }
}

/// 📄️ Decodes only the Hub's bounded delegation document.
pub fn decode_hub_agent_credential_v1(bytes: &[u8]) -> Result<DelegatedCredentialV1, GatewayError> {
    let malformed = |what: &str| GatewayError::new(GatewayErrorCode::InputInvalid, format!("agent credential is malformed: {what}"));
    if bytes.is_empty() || bytes.len() > 16 * 1024 { return Err(malformed("empty or larger than 16 KiB")); }
    let document = SecretDocument(serde_json::from_slice(bytes).map_err(|_| malformed("not JSON"))?);
    let object = document.as_object().ok_or_else(|| malformed("not an object"))?;
    let mut fields: Vec<&str> = object.keys().map(String::as_str).collect(); fields.sort_unstable();
    if fields != ["audience", "hubOrigin", "schema", "spaceId", "token"] { return Err(malformed("fields are not closed")); }
    let field = |name: &str| document.get(name).and_then(serde_json::Value::as_str).map(str::to_string).ok_or_else(|| malformed(name));
    if field("schema")? != AGENT_CREDENTIAL_SCHEMA { return Err(malformed("wrong schema")); }
    let origin = field("hubOrigin")?; let scope = field("spaceId")?;
    if origin.chars().count() > 1024 || scope.chars().count() > 256 { return Err(malformed("origin or scope exceeds its schema bound")); }
    let audience = field("audience")?; let mut token = SecretToken(field("token")?);
    if !matches!(audience.as_str(), "read" | "edit") || !valid_delegation_token(&token) { return Err(malformed("invalid audience or delegation token")); }
    DelegatedCredentialV1::new(origin, scope, audience, token.take())
}

fn decode_bound_grant_v1(bytes: &[u8], credential: &DelegatedCredentialV1) -> Result<DelegatedSessionGrantV1, GatewayError> {
    let grant = decode_agent_session_grant(bytes)?;
    let document = SecretDocument(serde_json::from_slice(bytes).map_err(|_| GatewayError::new(GatewayErrorCode::InputInvalid, "invalid Hub grant"))?);
    if grant.space_id != credential.scope_id() || document.get("audience").and_then(serde_json::Value::as_str) != Some(credential.audience()) { return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, "Hub grant does not match the credential's scope and audience")); }
    Ok(grant)
}

fn exchange_request_v1(credential: &DelegatedCredentialV1, instance: &str) -> Result<CredentialExchangeRequestV1, GatewayError> {
    CredentialExchangeRequestV1::new("/auth/agent-sessions".into(), agent_session_request_body(credential.audience(), instance))
}

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
    .expect("Hub agent request strings serialize")
}

pub fn decode_agent_session_grant(bytes: &[u8]) -> Result<DelegatedSessionGrantV1, GatewayError> {
    let malformed = || GatewayError::new(GatewayErrorCode::Internal, "the Hub agent-session response is invalid");
    if bytes.is_empty() || bytes.len() > 16 * 1024 { return Err(malformed()); }
    let document = SecretDocument(serde_json::from_slice(bytes).map_err(|_| malformed())?);
    let object = document.as_object().ok_or_else(malformed)?;
    let mut keys: Vec<&str> = object.keys().map(String::as_str).collect(); keys.sort_unstable();
    if keys != ["agentLabel", "agentPrincipalId", "audience", "expiresAtMs", "schema", "spaceId", "token"] { return Err(malformed()); }
    let field = |name: &str| document.get(name).and_then(serde_json::Value::as_str).map(str::to_string).ok_or_else(malformed);
    let mut token = SecretToken(field("token")?);
    let hex = |part: &str| part.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    let parts: Vec<&str> = token.split('.').collect();
    if document.get("schema").and_then(serde_json::Value::as_str) != Some("semio.hub.auth.agent-session/v1") || parts.len() != 4 || parts[0] != "session" || parts[1] != "v1" || parts[2].len() != 32 || parts[3].len() != 64 || !hex(parts[2]) || !hex(parts[3]) || !field("agentPrincipalId")?.starts_with("agent:") || field("agentPrincipalId")?.len() <= "agent:".len() || field("agentLabel")?.is_empty() || field("agentLabel")?.chars().count() > 64 || field("spaceId")?.is_empty() || field("spaceId")?.chars().count() > 256 || !matches!(field("audience")?.as_str(), "read" | "edit") || document.get("expiresAtMs").and_then(serde_json::Value::as_i64).is_none_or(|value| value < 0) { return Err(malformed()); }
    Ok(DelegatedSessionGrantV1 {
        token: token.take(),
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
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
