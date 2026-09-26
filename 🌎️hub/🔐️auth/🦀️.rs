//! 🔐️ Scope `hub.auth` — credential sign-in, session minting and the rate limiter in front of the
//! hub's unauthenticated and cheap-to-abuse routes.
//!
//! Schema authority: [`🔣️.json`](🧬️schema/🔣️.json)
//! (`https://json.schemas.assets.semio-tech.com/hub/auth/schema.json`), TypeScript mirror:
//! [`🟦️.ts`](🧬️schema/🟦️.ts). This module is the Rust decoder authority for
//! `CredentialSignInRequestV1`, `SessionMintResponseV1` and `AuthErrorV1`; the route that mounts it
//! is `POST /auth/sessions` in `🏗️bootstrap/🦀️.rs`.
//!
//! Sessions are never CRUD rows here: minting goes through
//! [`crate::directory::HubDirectory::issue_auth_session`] (which appends a `session-issued` fact),
//! revocation through `revoke_auth_session` (`session-revoked`), and every sign-in attempt —
//! admitted or refused — appends a `credential-sign-in` fact through
//! [`crate::directory::HubDirectory::append_credential_audit`].

use serde::{Deserialize, Serialize};

#[path = "🔑️password/🦀️.rs"]
pub mod password;

#[path = "🚦️rate-limit/🦀️.rs"]
pub mod rate_limit;

#[path = "🤖️agent/🦀️.rs"]
pub mod agent;

#[path = "🛡️access-policy/🦀️.rs"]
pub mod access_policy;

/// 🧬️ The scope id and `$id` every `hub.auth` export resolves under.
pub const SCHEMA_SCOPE: &str = "hub.auth";
pub const SCHEMA_ID: &str = "https://json.schemas.assets.semio-tech.com/hub/auth/schema.json";

/// 🎫️ The hub's only credential sign-in route and its exact request bound.
pub const SESSION_MINT_ROUTE: &str = "/auth/sessions";
pub const SIGN_IN_REQUEST_MAX_BYTES: usize = 1024;
pub const CREDENTIAL_SIGN_IN_SCHEMA: &str = "semio.hub.auth.credential-sign-in/v1";
pub const AUTH_ERROR_SCHEMA: &str = "semio.hub.auth.error/v1";

/// 🪪️ Session introspection — the one route that carries the session's own `expiresAt`, so a client
/// never has to guess a deadline the mint response deliberately withholds.
pub const SESSION_ME_ROUTE: &str = "/auth/sessions/me";

/// 🔁️ Self-service password change. The caller proves the current password on top of a live bearer,
/// so a stolen session alone can never take an account over.
pub const CREDENTIAL_ROUTE: &str = "/auth/credentials";
pub const CREDENTIAL_CHANGE_SCHEMA: &str = "semio.hub.auth.credential-change/v1";
pub const CREDENTIAL_CHANGE_REQUEST_MAX_BYTES: usize = 1024;

/// 🪪️ The identity provider every password-credential session is issued under, so a directory
/// revocation by identity can target exactly the credential-minted sessions.
pub const CREDENTIAL_IDENTITY_PROVIDER: &str = "credential.password.v1";

/// 🧾️ The durable fact kinds this scope appends to the authentication log.
pub const CREDENTIAL_SIGN_IN_EVENT: &str = "credential-sign-in";
pub const CREDENTIAL_CHANGED_EVENT: &str = "credential-changed";

/// ⏳️ Default browser session lifetime, and the window a deployment may configure.
pub const DEFAULT_SESSION_TTL_SECS: i64 = 12 * 60 * 60;
pub const MIN_SESSION_TTL_SECS: i64 = 60;
pub const MAX_SESSION_TTL_SECS: i64 = 31_536_000;

//#region 🔖️Wire
/// 🖥️ Which kind of client is asking — carried into the session's durable `peer_class`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthClientClassV1 {
    Browser,
    Native,
    Cli,
}

impl AuthClientClassV1 {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Browser => "browser",
            Self::Native => "native",
            Self::Cli => "cli",
        }
    }
}

/// 📝️ The exact `POST /auth/sessions` body. `Debug` never prints the password.
#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CredentialSignInRequestV1 {
    pub schema: String,
    pub email: String,
    pub password: String,
    pub device_instance_id: String,
    pub client_class: AuthClientClassV1,
}

impl std::fmt::Debug for CredentialSignInRequestV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CredentialSignInRequestV1")
            .field("schema", &self.schema)
            .field("email", &"[REDACTED]")
            .field("password", &"[REDACTED]")
            .field("deviceInstanceId", &self.device_instance_id)
            .field("clientClass", &self.client_class)
            .finish()
    }
}

/// ✅️ One bounds-checked sign-in request. Constructing this is the only way past the wire bounds.
#[derive(Clone)]
pub struct VerifiedCredentialSignInV1 {
    email: String,
    password: String,
    device_instance_id: String,
    client_class: AuthClientClassV1,
}

impl std::fmt::Debug for VerifiedCredentialSignInV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("VerifiedCredentialSignInV1").field("email", &"[REDACTED]").field("password", &"[REDACTED]").field("deviceInstanceId", &self.device_instance_id).field("clientClass", &self.client_class).finish()
    }
}

impl VerifiedCredentialSignInV1 {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn device_instance_id(&self) -> &str {
        &self.device_instance_id
    }

    pub fn client_class(&self) -> AuthClientClassV1 {
        self.client_class
    }
}

impl CredentialSignInRequestV1 {
    /// 🛂️ Holds the decoded body against `$defs/CredentialSignInRequestV1`'s bounds; the email is
    /// lowercased exactly once, here, so lookup and rate-limit subject always agree.
    pub fn verify(self) -> Result<VerifiedCredentialSignInV1, AuthErrorCodeV1> {
        if self.schema != CREDENTIAL_SIGN_IN_SCHEMA {
            return Err(AuthErrorCodeV1::MalformedRequest);
        }
        let email = self.email.trim().to_lowercase();
        if !(3..=254).contains(&email.len()) || !admissible_email(&email) {
            return Err(AuthErrorCodeV1::MalformedRequest);
        }
        if !(password::PASSWORD_MIN_BYTES..=password::PASSWORD_MAX_BYTES).contains(&self.password.len()) || self.password.chars().any(|character| character.is_control()) {
            return Err(AuthErrorCodeV1::MalformedRequest);
        }
        if !(1..=128).contains(&self.device_instance_id.len()) || !self.device_instance_id.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-')) {
            return Err(AuthErrorCodeV1::MalformedRequest);
        }
        Ok(VerifiedCredentialSignInV1 { email, password: self.password, device_instance_id: self.device_instance_id, client_class: self.client_class })
    }
}

fn admissible_email(email: &str) -> bool {
    let mut parts = email.split('@');
    let Some(local) = parts.next() else { return false };
    let Some(domain) = parts.next() else { return false };
    parts.next().is_none() && !local.is_empty() && !domain.is_empty() && !email.chars().any(|character| character.is_control() || character == ' ')
}

/// 🔁️ The exact `POST /auth/credentials` body. `Debug` never prints either password.
#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CredentialChangeRequestV1 {
    pub schema: String,
    pub current_password: String,
    pub new_password: String,
}

impl std::fmt::Debug for CredentialChangeRequestV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("CredentialChangeRequestV1").field("schema", &self.schema).field("currentPassword", &"[REDACTED]").field("newPassword", &"[REDACTED]").finish()
    }
}

/// ✅️ One bounds-checked credential change. Constructing this is the only way past the wire bounds.
#[derive(Clone)]
pub struct VerifiedCredentialChangeV1 {
    current_password: String,
    new_password: String,
}

impl std::fmt::Debug for VerifiedCredentialChangeV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("VerifiedCredentialChangeV1").field("currentPassword", &"[REDACTED]").field("newPassword", &"[REDACTED]").finish()
    }
}

impl VerifiedCredentialChangeV1 {
    pub fn current_password(&self) -> &str {
        &self.current_password
    }

    pub fn new_password(&self) -> &str {
        &self.new_password
    }
}

impl CredentialChangeRequestV1 {
    /// 🛂️ Holds the decoded body against `$defs/CredentialChangeRequestV1`'s bounds. A new password
    /// equal to the current one is refused here rather than silently rewriting the same secret under
    /// a fresh salt, which would journal a `credential-changed` fact that changed nothing.
    pub fn verify(self) -> Result<VerifiedCredentialChangeV1, AuthErrorCodeV1> {
        if self.schema != CREDENTIAL_CHANGE_SCHEMA {
            return Err(AuthErrorCodeV1::MalformedRequest);
        }
        for secret in [&self.current_password, &self.new_password] {
            if !(password::PASSWORD_MIN_BYTES..=password::PASSWORD_MAX_BYTES).contains(&secret.len()) || secret.chars().any(|character| character.is_control()) {
                return Err(AuthErrorCodeV1::MalformedRequest);
            }
        }
        if self.current_password == self.new_password {
            return Err(AuthErrorCodeV1::MalformedRequest);
        }
        Ok(VerifiedCredentialChangeV1 { current_password: self.current_password, new_password: self.new_password })
    }
}

/// 🎁️ The exact 200 body. Field names are snake_case on purpose: this is what the os client's
/// `SessionMintResponse` decoder (`📇️directory/🔌️client/🦀️.rs`) already reads, and it is strict.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionMintResponseV1 {
    pub token: String,
    pub user_id: String,
}

/// 🛑️ Every credential sign-in failure the client can observe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthErrorCodeV1 {
    MalformedRequest,
    InvalidCredentials,
    CredentialSignInDisabled,
    RateLimited,
    DirectoryUnavailable,
}

impl AuthErrorCodeV1 {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MalformedRequest => "malformed-request",
            Self::InvalidCredentials => "invalid-credentials",
            Self::CredentialSignInDisabled => "credential-sign-in-disabled",
            Self::RateLimited => "rate-limited",
            Self::DirectoryUnavailable => "directory-unavailable",
        }
    }

    /// 🌐️ The exact HTTP status this code is served with.
    pub fn status(self) -> u16 {
        match self {
            Self::MalformedRequest => 400,
            Self::InvalidCredentials => 401,
            Self::CredentialSignInDisabled => 403,
            Self::RateLimited => 429,
            Self::DirectoryUnavailable => 503,
        }
    }

    /// 🧾️ The durable reason code this refusal is journaled under.
    pub fn reason_code(self) -> &'static str {
        self.as_str()
    }
}

/// 🛑️ The exact non-200 body.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthErrorV1 {
    pub schema: String,
    pub error: AuthErrorCodeV1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<u64>,
}

impl AuthErrorV1 {
    pub fn new(error: AuthErrorCodeV1) -> Self {
        Self { schema: AUTH_ERROR_SCHEMA.into(), error, retry_after_ms: None }
    }

    pub fn rate_limited(retry_after_ms: u64) -> Self {
        Self { schema: AUTH_ERROR_SCHEMA.into(), error: AuthErrorCodeV1::RateLimited, retry_after_ms: Some(retry_after_ms) }
    }
}
//#endregion 🔖️Wire

//#region 🔖️Policy
/// 🛂️ A deployment's credential sign-in posture. Fail-closed: a hub only issues sessions to
/// password credentials when it was explicitly told to, so a development hub keeps minting through
/// the local-bootstrap pipe alone and `/readyz` keeps reporting `publicSessionIssuance: false`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CredentialSignInPolicyV1 {
    enabled: bool,
    session_ttl_secs: i64,
    mint_iterations: u32,
}

impl Default for CredentialSignInPolicyV1 {
    fn default() -> Self {
        Self { enabled: false, session_ttl_secs: DEFAULT_SESSION_TTL_SECS, mint_iterations: password::DEFAULT_ITERATIONS }
    }
}

impl CredentialSignInPolicyV1 {
    /// 🔓️ An explicitly enabled policy, bounds-checked.
    pub fn enabled(session_ttl_secs: i64, mint_iterations: u32) -> Result<Self, AuthPolicyError> {
        if !(MIN_SESSION_TTL_SECS..=MAX_SESSION_TTL_SECS).contains(&session_ttl_secs) {
            return Err(AuthPolicyError::SessionTtlOutOfBounds);
        }
        if !(password::MIN_ITERATIONS..=password::MAX_ITERATIONS).contains(&mint_iterations) {
            return Err(AuthPolicyError::IterationsOutOfBounds);
        }
        Ok(Self { enabled: true, session_ttl_secs, mint_iterations })
    }

    /// 🌱️ `OS_HUB_CREDENTIAL_SIGN_IN` (`true`/`1` to enable), `OS_HUB_SESSION_TTL_SECONDS`,
    /// `OS_HUB_PASSWORD_ITERATIONS`. Absent means disabled, an unreadable value is a boot failure.
    pub fn from_env() -> Result<Self, AuthPolicyError> {
        let enabled = match std::env::var("OS_HUB_CREDENTIAL_SIGN_IN").ok().as_deref() {
            None | Some("") | Some("false") | Some("0") => false,
            Some("true") | Some("1") => true,
            Some(_) => return Err(AuthPolicyError::UnreadableFlag),
        };
        if !enabled {
            return Ok(Self::default());
        }
        let ttl = match std::env::var("OS_HUB_SESSION_TTL_SECONDS").ok().filter(|value| !value.is_empty()) {
            Some(value) => value.parse::<i64>().map_err(|_| AuthPolicyError::SessionTtlOutOfBounds)?,
            None => DEFAULT_SESSION_TTL_SECS,
        };
        let iterations = match std::env::var("OS_HUB_PASSWORD_ITERATIONS").ok().filter(|value| !value.is_empty()) {
            Some(value) => value.parse::<u32>().map_err(|_| AuthPolicyError::IterationsOutOfBounds)?,
            None => password::DEFAULT_ITERATIONS,
        };
        Self::enabled(ttl, iterations)
    }

    pub fn is_enabled(self) -> bool {
        self.enabled
    }

    pub fn session_ttl_secs(self) -> i64 {
        self.session_ttl_secs
    }

    pub fn mint_iterations(self) -> u32 {
        self.mint_iterations
    }
}

/// 🛑️ Why a configured policy is not admissible.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthPolicyError {
    UnreadableFlag,
    SessionTtlOutOfBounds,
    IterationsOutOfBounds,
}

impl std::fmt::Display for AuthPolicyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::UnreadableFlag => "OS_HUB_CREDENTIAL_SIGN_IN must be true, false, 1, or 0",
            Self::SessionTtlOutOfBounds => "OS_HUB_SESSION_TTL_SECONDS must be 60..=31536000",
            Self::IterationsOutOfBounds => "OS_HUB_PASSWORD_ITERATIONS must be 1000..=999999999",
        })
    }
}

impl std::error::Error for AuthPolicyError {}
//#endregion 🔖️Policy

//#region 🔖️Decision
/// 🙋️ Exactly what the decision needs to know about the claimed account — never the whole record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CredentialSubjectV1 {
    pub user_id: String,
    pub password_hash: Option<String>,
}

/// ⚖️ The outcome of one sign-in attempt, before any session is minted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CredentialSignInDecisionV1 {
    Mint { user_id: String, ttl_secs: i64 },
    Refuse(AuthErrorCodeV1),
}

/// ⚖️ The whole credential law, pure and clock-free: policy, account presence, credential shape and
/// password equality all collapse into ONE observable refusal, so no caller can enumerate accounts.
pub fn decide_credential_sign_in(policy: CredentialSignInPolicyV1, request: &VerifiedCredentialSignInV1, subject: Option<&CredentialSubjectV1>) -> CredentialSignInDecisionV1 {
    if !policy.is_enabled() {
        return CredentialSignInDecisionV1::Refuse(AuthErrorCodeV1::CredentialSignInDisabled);
    }
    let stored = subject.and_then(|subject| subject.password_hash.as_deref().and_then(|encoded| password::PasswordCredentialV1::parse(encoded).ok()).map(|credential| (subject, credential)));
    let Some((subject, credential)) = stored else {
        password::PasswordCredentialV1::absent().verify(request.password());
        return CredentialSignInDecisionV1::Refuse(AuthErrorCodeV1::InvalidCredentials);
    };
    if credential.verify(request.password()) {
        CredentialSignInDecisionV1::Mint { user_id: subject.user_id.clone(), ttl_secs: policy.session_ttl_secs() }
    } else {
        CredentialSignInDecisionV1::Refuse(AuthErrorCodeV1::InvalidCredentials)
    }
}

/// ⚖️ The outcome of one credential change, before anything is written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CredentialChangeDecisionV1 {
    Apply { user_id: String, mint_iterations: u32 },
    Refuse(AuthErrorCodeV1),
}

/// ⚖️ The whole credential-change law, pure and clock-free. The bearer already proved *which*
/// principal is asking, so there is nothing to enumerate here — but "this account has no password
/// credential at all" and "the supplied current password is wrong" still collapse into the same
/// `invalid-credentials`, because a principal without a credential must go through the operator
/// bootstrap verb (`os-hub credential set`), never through a route reachable over the network.
pub fn decide_credential_change(policy: CredentialSignInPolicyV1, request: &VerifiedCredentialChangeV1, subject: &CredentialSubjectV1) -> CredentialChangeDecisionV1 {
    if !policy.is_enabled() {
        return CredentialChangeDecisionV1::Refuse(AuthErrorCodeV1::CredentialSignInDisabled);
    }
    let Some(encoded) = subject.password_hash.as_deref() else { return CredentialChangeDecisionV1::Refuse(AuthErrorCodeV1::InvalidCredentials) };
    let Ok(credential) = password::PasswordCredentialV1::parse(encoded) else { return CredentialChangeDecisionV1::Refuse(AuthErrorCodeV1::InvalidCredentials) };
    if credential.verify(request.current_password()) {
        CredentialChangeDecisionV1::Apply { user_id: subject.user_id.clone(), mint_iterations: policy.mint_iterations() }
    } else {
        CredentialChangeDecisionV1::Refuse(AuthErrorCodeV1::InvalidCredentials)
    }
}
//#endregion 🔖️Decision

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
