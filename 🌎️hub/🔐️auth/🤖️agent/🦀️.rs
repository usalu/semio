//! 🤖️ `hub.auth.agent` — the delegated agent credential: a signed-in human hands an AI agent a
//! scoped, revocable capability, and the agent exchanges it for a session in which it acts inside
//! ONE space as its own principal, never as the human.
//!
//! Three routes, all of them event-sourced through the same log sessions use
//! (`crate::directory::AGENT_DELEGATED_EVENT` / `AGENT_DELEGATION_REVOKED_EVENT` /
//! `AGENT_SESSION_ISSUED_EVENT`): `POST /auth/agent-delegations` (create, bearer = the delegating
//! human's session), `GET /auth/agent-delegations?space=<id>` (list), `DELETE
//! /auth/agent-delegations/{id}` (revoke) and `POST /auth/agent-sessions` (exchange the delegation
//! for an agent session, bearer = the delegation capability itself).
//!
//! The delegation token is shown exactly once, at creation. It never travels in argv, in an
//! environment variable, or in a URL: `semio-os-mcp --hub` reads it from a credential file or a
//! stdin handle (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs`).

use serde::{Deserialize, Serialize};

use crate::directory::AgentSessionPreflight;
use crate::directory::model::{AgentAudience, AgentDelegationRow};

/// 🛣️ The three delegation routes and the exchange route.
pub const AGENT_DELEGATION_ROUTE: &str = "/auth/agent-delegations";
pub const AGENT_SESSION_ROUTE: &str = "/auth/agent-sessions";

pub const AGENT_DELEGATION_CREATE_SCHEMA: &str = "semio.hub.auth.agent-delegation-create/v1";
pub const AGENT_DELEGATION_RECEIPT_SCHEMA: &str = "semio.hub.auth.agent-delegation-receipt/v1";
pub const AGENT_DELEGATION_LIST_SCHEMA: &str = "semio.hub.auth.agent-delegation-list/v1";
pub const AGENT_SESSION_REQUEST_SCHEMA: &str = "semio.hub.auth.agent-session/v1";

/// 📏️ Request bounds. Both bodies are tiny and closed; anything larger is refused by axum's
/// `DefaultBodyLimit` before a handler runs, exactly like credential sign-in.
pub const AGENT_DELEGATION_REQUEST_MAX_BYTES: usize = 1024;
pub const AGENT_SESSION_REQUEST_MAX_BYTES: usize = 1024;

/// ⏳️ Delegation lifetime bounds. A delegation is deliberately shorter-lived than a browser
/// session's year-long ceiling: an unattended credential that outlives the work it was minted for
/// is the whole risk this scope exists to bound.
pub const DEFAULT_DELEGATION_TTL_SECS: i64 = 7 * 24 * 60 * 60;
pub const MIN_DELEGATION_TTL_SECS: i64 = 60;
pub const MAX_DELEGATION_TTL_SECS: i64 = 90 * 24 * 60 * 60;

/// ⏳️ The lifetime of one agent *session* minted from a delegation. Short on purpose: the agent
/// re-exchanges, and every re-exchange re-reads the delegation's revocation state.
pub const AGENT_SESSION_TTL_SECS: i64 = 60 * 60;

//#region 🔖️Wire
/// 📝️ The exact `POST /auth/agent-delegations` body.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateAgentDelegationRequestV1 {
    pub schema: String,
    pub space_id: String,
    pub agent_label: String,
    pub audience: AgentAudience,
    pub ttl_secs: i64,
}

/// ✅️ One bounds-checked delegation request. Constructing this is the only way past the bounds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedCreateAgentDelegationV1 {
    space_id: String,
    agent_label: String,
    audience: AgentAudience,
    ttl_secs: i64,
}

impl VerifiedCreateAgentDelegationV1 {
    pub fn space_id(&self) -> &str {
        &self.space_id
    }

    pub fn agent_label(&self) -> &str {
        &self.agent_label
    }

    pub fn audience(&self) -> AgentAudience {
        self.audience
    }

    pub fn ttl_secs(&self) -> i64 {
        self.ttl_secs
    }
}

impl CreateAgentDelegationRequestV1 {
    /// 🛂️ Holds the decoded body against `$defs/CreateAgentDelegationRequestV1`'s bounds.
    pub fn verify(self) -> Result<VerifiedCreateAgentDelegationV1, AgentErrorCodeV1> {
        if self.schema != AGENT_DELEGATION_CREATE_SCHEMA {
            return Err(AgentErrorCodeV1::MalformedRequest);
        }
        if !(1..=256).contains(&self.space_id.len()) || self.space_id.chars().any(char::is_control) {
            return Err(AgentErrorCodeV1::MalformedRequest);
        }
        let agent_label = self.agent_label.trim().to_string();
        if !(1..=crate::directory::AGENT_LABEL_MAX_BYTES).contains(&agent_label.len()) || agent_label.chars().any(char::is_control) {
            return Err(AgentErrorCodeV1::MalformedRequest);
        }
        if !(MIN_DELEGATION_TTL_SECS..=MAX_DELEGATION_TTL_SECS).contains(&self.ttl_secs) {
            return Err(AgentErrorCodeV1::MalformedRequest);
        }
        Ok(VerifiedCreateAgentDelegationV1 { space_id: self.space_id, agent_label, audience: self.audience, ttl_secs: self.ttl_secs })
    }
}

/// 🎁️ The exact 201 body. `token` is the one-time plaintext delegation capability — the only time
/// it is ever readable. `agentPrincipalId` is the actor string every later edit, presence row and
/// undo entry carries, so the caller can show the human exactly who it just created.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDelegationReceiptV1 {
    pub schema: String,
    pub delegation_id: String,
    pub agent_principal_id: String,
    pub agent_label: String,
    pub space_id: String,
    pub audience: AgentAudience,
    pub expires_at_ms: i64,
    pub token: String,
}

/// 📋️ One delegation as the delegation UI reads it. No token, no selector, no digest.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDelegationSummaryV1 {
    pub delegation_id: String,
    pub agent_principal_id: String,
    pub agent_label: String,
    pub space_id: String,
    pub audience: AgentAudience,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    pub revoked: bool,
    /// 🕰️ When this delegation last minted an agent session, `null` if it never has. Always
    /// serialized, so the delegation UI reads "never used" as a state rather than as an absent key
    /// it has to guess about.
    pub last_used_at_ms: Option<i64>,
}

impl AgentDelegationSummaryV1 {
    pub fn from_row(row: AgentDelegationRow) -> Self {
        Self {
            agent_principal_id: agent_principal_id(&row.delegation_id),
            delegation_id: row.delegation_id,
            agent_label: row.agent_label,
            space_id: row.space_id,
            audience: row.audience,
            created_at_ms: row.created_at_ms,
            expires_at_ms: row.expires_at_ms,
            revoked: row.revoked,
            last_used_at_ms: row.last_used_at_ms,
        }
    }
}

/// 📋️ The exact `GET /auth/agent-delegations` body.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDelegationListV1 {
    pub schema: String,
    pub delegations: Vec<AgentDelegationSummaryV1>,
}

impl AgentDelegationListV1 {
    pub fn new(delegations: Vec<AgentDelegationSummaryV1>) -> Self {
        Self { schema: AGENT_DELEGATION_LIST_SCHEMA.into(), delegations }
    }
}

/// 📝️ The exact `POST /auth/agent-sessions` body. The delegation token is the `Authorization`
/// bearer, never a body field: it is a capability, and it travels where capabilities travel.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AgentSessionRequestV1 {
    pub schema: String,
    pub audience: AgentAudience,
    pub agent_instance_id: String,
}

impl AgentSessionRequestV1 {
    /// 🛂️ `agentInstanceId` reuses `deviceInstanceId`'s charset so one agent process is
    /// distinguishable from another under the same delegation.
    pub fn verify(&self) -> Result<(), AgentErrorCodeV1> {
        if self.schema != AGENT_SESSION_REQUEST_SCHEMA {
            return Err(AgentErrorCodeV1::MalformedRequest);
        }
        if !(1..=128).contains(&self.agent_instance_id.len()) || !self.agent_instance_id.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-')) {
            return Err(AgentErrorCodeV1::MalformedRequest);
        }
        Ok(())
    }
}

/// 🎁️ The exact `POST /auth/agent-sessions` 200 body. `token` is a `SessionCapability` exactly like
/// credential sign-in mints, so every existing hub route authenticates an agent with no special
/// case — what differs is the session's kind, and therefore its presence row and its actor.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSessionMintResponseV1 {
    pub schema: String,
    pub token: String,
    pub agent_principal_id: String,
    pub agent_label: String,
    pub space_id: String,
    pub audience: AgentAudience,
    pub expires_at_ms: i64,
}

/// 🛑️ Every agent-delegation failure a client can observe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentErrorCodeV1 {
    MalformedRequest,
    InvalidDelegation,
    DelegationRevoked,
    DelegationExpired,
    Forbidden,
    RateLimited,
    DirectoryUnavailable,
}

impl AgentErrorCodeV1 {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MalformedRequest => "malformed-request",
            Self::InvalidDelegation => "invalid-delegation",
            Self::DelegationRevoked => "delegation-revoked",
            Self::DelegationExpired => "delegation-expired",
            Self::Forbidden => "forbidden",
            Self::RateLimited => "rate-limited",
            Self::DirectoryUnavailable => "directory-unavailable",
        }
    }

    /// 🌐️ The exact HTTP status this code is served with.
    pub fn status(self) -> u16 {
        match self {
            Self::MalformedRequest => 400,
            Self::InvalidDelegation => 401,
            Self::DelegationRevoked | Self::DelegationExpired | Self::Forbidden => 403,
            Self::RateLimited => 429,
            Self::DirectoryUnavailable => 503,
        }
    }
}

/// 🛑️ The exact non-2xx body, spelled like `AuthErrorV1` so one client parser reads both.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentErrorV1 {
    pub schema: String,
    pub error: AgentErrorCodeV1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<u64>,
}

pub const AGENT_ERROR_SCHEMA: &str = "semio.hub.auth.agent-error/v1";

impl AgentErrorV1 {
    pub fn new(error: AgentErrorCodeV1) -> Self {
        Self { schema: AGENT_ERROR_SCHEMA.into(), error, retry_after_ms: None }
    }
}
//#endregion 🔖️Wire

//#region 🔖️Principal
/// 🤖️ The actor string an agent principal carries everywhere a human carries `user:<id>#<session>`.
/// It is derived from the delegation id alone, so it is stable across the agent's sessions and
/// across hub restarts, and it is structurally impossible to confuse with a human actor.
pub fn agent_principal_id(delegation_id: &str) -> String {
    format!("agent:{delegation_id}")
}

/// 🤖️ Whether an actor string names an agent principal. The inverse of [`agent_principal_id`]'s
/// prefix, used by per-actor undo ownership and by the roster.
pub fn is_agent_principal(actor: &str) -> bool {
    actor.starts_with("agent:")
}

/// 🤖️ How an agent is NAMED wherever a human reads an actor — a roster row, the connection log, an
/// attribution line. The identity itself is untouched: the hub keeps minting an opaque, per-session
/// `hub.v1.<sha256>` socket actor, which is exactly what makes an agent's edits a different actor
/// from the delegating human's and keeps per-actor undo separating them. Only the rendering changes,
/// from "the human whose authority it borrows" to "the agent its human named".
pub fn agent_actor_display(agent_label: &str) -> String {
    format!("agent:{agent_label}")
}
//#endregion 🔖️Principal

//#region 🔖️Decision
/// ⚖️ What one agent-session exchange should do, before anything is minted. Pure: the caller
/// supplies the preflight the directory's own law produced and the audience it asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AgentSessionDecisionV1 {
    Mint { ttl_secs: i64 },
    Refuse(AgentErrorCodeV1),
}

/// ⚖️ Translates the directory's preflight into the client-observable outcome. `Denied` — which
/// covers "no such delegation", "wrong secret" and "audience does not match" alike — is one
/// `invalid-delegation`, so a delegation id can never be probed for existence. A caller that
/// already proved the secret is told the truth about revocation and expiry, because it needs to
/// know whether to stop or to ask its human for a new credential.
pub fn decide_agent_session(preflight: AgentSessionPreflight) -> AgentSessionDecisionV1 {
    match preflight {
        AgentSessionPreflight::Mint => AgentSessionDecisionV1::Mint { ttl_secs: AGENT_SESSION_TTL_SECS },
        AgentSessionPreflight::Revoked => AgentSessionDecisionV1::Refuse(AgentErrorCodeV1::DelegationRevoked),
        AgentSessionPreflight::Expired => AgentSessionDecisionV1::Refuse(AgentErrorCodeV1::DelegationExpired),
        AgentSessionPreflight::Denied => AgentSessionDecisionV1::Refuse(AgentErrorCodeV1::InvalidDelegation),
    }
}
//#endregion 🔖️Decision

#[cfg(test)]
#[path = "../🧪️tests/🤖️agent-unit/🦀️.rs"]
mod tests;
