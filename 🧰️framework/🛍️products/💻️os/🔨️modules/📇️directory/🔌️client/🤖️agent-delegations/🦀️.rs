//! 🤖️ The asking human's own agent delegations in one space — `GET /auth/agent-delegations?space=<id>` — read as the live agent
//! principals a shell names when it asks the agent-bridge supervisor for its scoped offer (ticket 26/09/23, G12 × WG11
//! session 14c). The Rust twin of `📇️directory/🤖️delegations/🟦️.ts` (`parseAgentDelegationListV1`, `agentPrincipalIdV1`) plus
//! ShellHost's offer-scope filter; both answer `🤖️delegations/🧫️fixtures/📋️agent-delegation-list.json`.

use super::{encode_url_component, DirectoryClient, DirectoryClientError, DirectoryTransport, HttpMethod};
use semio_framework_pack_json::Value;
use semio_framework_async::OperationContext;

/// 📏️ The hub's own listing page bound (`AGENT_DELEGATION_PAGE_MAX`); rows past it are never read.
pub const AGENT_DELEGATION_PAGE_MAX: usize = 256;
/// 📏️ The listing answer's byte bound, checked before it is decoded.
pub const AGENT_DELEGATION_LIST_MAX_BYTES: usize = 256 * 1024;
const AGENT_LABEL_MAX_UNITS: usize = 128;
const IDENTIFIER_MAX_UNITS: usize = 256;
const SAFE_INTEGER_MAX: u64 = 9_007_199_254_740_991;

/// 🤖️ `semio_hub::auth::agent::agent_principal_id`'s twin: the actor string of one delegation's agent.
pub fn agent_principal_id(delegation_id: &str) -> String {
    format!("agent:{delegation_id}")
}

/// 🔤️ A bounded, control-free, non-empty text field, measured in UTF-16 units as the TypeScript twin measures it.
fn bounded_text<'a>(row: &'a Value, key: &str, max: usize) -> Option<&'a str> {
    row.get(key).and_then(Value::as_str).filter(|value| !value.is_empty() && value.encode_utf16().count() <= max && !value.chars().any(char::is_control))
}

/// ⏱️ A safe-integer millisecond field (`Number.isSafeInteger`).
fn safe_ms(row: &Value, key: &str) -> Option<i64> {
    row.get(key).and_then(Value::as_i64).filter(|value| value.unsigned_abs() <= SAFE_INTEGER_MAX)
}

/// 📥️ The principals of the live delegations one listing answer names at `now_ms`, in listing order: a malformed row is
/// dropped, never the listing, and a revoked or expired row names no principal — exactly what ShellHost's `offerScope` keeps.
pub fn live_agent_principals(body: &str, now_ms: i64) -> Vec<String> {
    let Ok(value) = semio_framework_pack_json::parse(body, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return Vec::new() };
    let Some(rows) = value.get("delegations").and_then(Value::as_array) else { return Vec::new() };
    rows.iter()
        .take(AGENT_DELEGATION_PAGE_MAX)
        .filter_map(|row| {
            let delegation_id = bounded_text(row, "delegationId", IDENTIFIER_MAX_UNITS)?;
            bounded_text(row, "agentLabel", AGENT_LABEL_MAX_UNITS)?;
            bounded_text(row, "spaceId", IDENTIFIER_MAX_UNITS)?;
            safe_ms(row, "createdAtMs")?;
            let expires_at_ms = safe_ms(row, "expiresAtMs")?;
            matches!(row.get("audience").and_then(Value::as_str), Some("read" | "edit")).then_some(())?;
            let revoked = row.get("revoked").and_then(Value::as_bool) == Some(true);
            (!revoked && expires_at_ms > now_ms).then(|| agent_principal_id(delegation_id))
        })
        .collect()
}

impl<T: DirectoryTransport> DirectoryClient<T> {
    /// 🤖️ The live agent principals the asking human delegated in `space_id` at `now_ms` — one bounded
    /// `GET /auth/agent-delegations?space=<id>` read with this client's own session capability, which never leaves it.
    pub async fn live_agent_delegation_principals(&self, ctx: &OperationContext, space_id: &str, now_ms: i64) -> Result<Vec<String>, DirectoryClientError> {
        if space_id.is_empty() || space_id.encode_utf16().count() > IDENTIFIER_MAX_UNITS || space_id.chars().any(char::is_control) {
            return Err(DirectoryClientError::Decode("agent delegation listing names no space".into()));
        }
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let path = format!("/auth/agent-delegations?space={}", encode_url_component(space_id));
        let response = self.transport.http(ctx, HttpMethod::Get, &self.url(&path), bearer, None).await?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            200 => {}
            401 => return Err(DirectoryClientError::Unauthorized),
            status => return Err(DirectoryClientError::Http { status, body: String::new() }),
        }
        if response.body.len() > AGENT_DELEGATION_LIST_MAX_BYTES {
            return Err(DirectoryClientError::Decode("agent delegation listing exceeds its bound".into()));
        }
        let body = String::from_utf8(response.body).map_err(|error| DirectoryClientError::Decode(error.to_string()))?;
        Ok(live_agent_principals(&body, now_ms))
    }
}
