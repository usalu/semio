//! 🤖️ wgpu twin of `🤖️AgentDelegations` and of the contract `📇️directory/🤖️delegations/🟦️.ts`: the human-facing half
//! of the delegated agent credential — create a scoped, revocable delegation, download its credential file once, see what
//! is outstanding, withdraw one behind a confirmation.
//!
//! 🧬️ Routes, schemas, bounds, lifetimes, control ids/icons and the en/de label table are read from the language-neutral
//! `📇️directory/🤖️delegations/🔣️.json` (shape `🧬️.schema.json`) the React side is held to by a law; behaviour is held to the
//! shared vectors `🧪️tests/🧫️fixtures/📐️agent-delegation-contract-vectors.json`. Pure: no transport here — the shell turns
//! in `📒️directory` send the requests this module builds and hand the answers back to its parsers.
//!
//! 🚪️ Host doors: the download verb hands the credential file to the shell's save door (native save dialog, browser
//! download), the MCP install verb posts the install request to the page's loopback route and a host that serves none
//! (native, a static deployment) answers the contract's `unavailable` phase, as React does.

use crate::hub_connection::{action, button, descriptor, input, one_arg, select, stack, tagged_row, text_row};
use semio_framework::IconName;
use semio_ui_declaration::ui::UiNode;
use semio_ui_locale::Locale;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

const CONTRACT_SOURCE: &str = include_str!("../../../../../../../../../🔨️modules/📇️directory/🤖️delegations/🔣️.json");

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Bounds {
    label_max_bytes: usize,
    min_ttl_secs: u64,
    max_ttl_secs: u64,
    page_max: usize,
    identifier_max: usize,
}

#[derive(Deserialize)]
struct Control {
    id: String,
    icon: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Contract {
    paths: HashMap<String, String>,
    schemas: HashMap<String, String>,
    bounds: Bounds,
    ttl_choices_secs: Vec<u64>,
    default_ttl_index: usize,
    default_audience: String,
    controls: HashMap<String, Control>,
    fields: HashMap<String, String>,
    labels: HashMap<String, HashMap<String, String>>,
}

fn contract() -> &'static Contract {
    static CONTRACT: OnceLock<Contract> = OnceLock::new();
    CONTRACT.get_or_init(|| serde_json::from_str(CONTRACT_SOURCE).expect("the shared agent-delegations contract is valid JSON of its schema"))
}

fn control(name: &str) -> (&'static str, IconName) {
    let control = &contract().controls[name];
    (control.id.as_str(), IconName::from(control.icon.as_str()))
}

//#region 🌐️Labels
/// 🌐️ One label of the contract's table, in `locale`.
pub fn agent_label(locale: Locale, key: &str) -> &'static str {
    let row = match locale {
        Locale::En => "en",
        Locale::De => "de",
    };
    contract().labels.get(row).and_then(|labels| labels.get(key)).map_or("", String::as_str)
}

/// 🌐️ A label with its `{{name}}`-style placeholders filled.
pub fn agent_label_with(locale: Locale, key: &str, values: &[(&str, &str)]) -> String {
    values.iter().fold(agent_label(locale, key).to_string(), |text, (name, value)| text.replace(&format!("{{{{{name}}}}}"), value))
}
//#endregion 🌐️Labels

//#region 🎯️Scope
/// 🎯️ What an agent may do — a closed two-value enum, `read` or `edit`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentAudience {
    Read,
    Edit,
}

impl AgentAudience {
    pub const ALL: [Self; 2] = [Self::Read, Self::Edit];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Edit => "edit",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|audience| audience.as_str() == value)
    }
}

/// 🤖️ The actor string of one delegation's agent.
pub fn agent_principal_id(delegation_id: &str) -> String {
    format!("agent:{delegation_id}")
}

/// 🔑️ The exact delegation token shape: `delegation.v1.<32 lower-hex>.<64 lower-hex>`.
pub fn is_agent_delegation_token(value: &str) -> bool {
    let hex = |part: &str, len: usize| part.len() == len && part.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'));
    value.strip_prefix("delegation.v1.").and_then(|rest| rest.split_once('.')).is_some_and(|(left, right)| hex(left, 32) && hex(right, 64))
}

/// ⏳️ The lifetimes offered: a day, a week, a month.
pub fn agent_ttl_choices_secs() -> &'static [u64] {
    &contract().ttl_choices_secs
}
//#endregion 🎯️Scope

//#region 📋️Rows
/// 📋️ One delegation as the hub lists it — never a token, never a selector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentDelegationSummary {
    pub delegation_id: String,
    pub agent_label: String,
    pub space_id: String,
    pub audience: AgentAudience,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    pub revoked: bool,
    pub last_used_at_ms: Option<i64>,
}

/// 🚦️ What a row is right now, derived from the summary and a clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentDelegationState {
    Live,
    Expired,
    Revoked,
}

impl AgentDelegationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
        }
    }
}

/// 📋️ A summary with its derived state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentDelegationRow {
    pub summary: AgentDelegationSummary,
    pub state: AgentDelegationState,
}

/// 🚦️ Revoked, else expired once the deadline passed, else live.
pub fn agent_delegation_state(summary: &AgentDelegationSummary, now_ms: i64) -> AgentDelegationState {
    if summary.revoked {
        AgentDelegationState::Revoked
    } else if summary.expires_at_ms <= now_ms {
        AgentDelegationState::Expired
    } else {
        AgentDelegationState::Live
    }
}

/// 📇️ Live first, then expired, then revoked; newest first; ties by id — deterministic on every device.
pub fn agent_delegation_rows(summaries: &[AgentDelegationSummary], now_ms: i64) -> Vec<AgentDelegationRow> {
    let rank = |state: AgentDelegationState| state as u8;
    let mut rows: Vec<AgentDelegationRow> = summaries.iter().map(|summary| AgentDelegationRow { state: agent_delegation_state(summary, now_ms), summary: summary.clone() }).collect();
    rows.sort_by(|left, right| rank(left.state).cmp(&rank(right.state)).then(right.summary.created_at_ms.cmp(&left.summary.created_at_ms)).then(left.summary.delegation_id.cmp(&right.summary.delegation_id)));
    rows
}

/// 🗑️ Whether withdrawing would still change anything: everything but a revoked row.
pub fn agent_delegation_revocable(row: &AgentDelegationRow) -> bool {
    row.state != AgentDelegationState::Revoked
}

/// 🕰️ A locale-independent `YYYY-MM-DD HH:MM UTC`, `—` for a non-integer instant.
pub fn agent_delegation_instant(milliseconds: Option<i64>) -> String {
    let Some(ms) = milliseconds.filter(|ms| ms.unsigned_abs() <= 9_007_199_254_740_991) else { return "—".to_string() };
    let days = ms.div_euclid(86_400_000);
    let rest = ms.rem_euclid(86_400_000);
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 { shifted_month + 3 } else { shifted_month - 9 };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02} {:02}:{:02} UTC", rest / 3_600_000, (rest % 3_600_000) / 60_000)
}
//#endregion 📋️Rows

//#region 🏗️Requests
fn identifier_ok(value: &str) -> bool {
    !value.is_empty() && value.encode_utf16().count() <= contract().bounds.identifier_max && !value.chars().any(char::is_control)
}

fn encode_component(value: &str) -> String {
    value.bytes().fold(String::new(), |mut out, byte| {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')') {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
        out
    })
}

fn json_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

/// 🏗️ The create route (`POST`).
pub fn agent_delegation_create_path() -> &'static str {
    &contract().paths["delegations"]
}

/// 📋️ The listing path for one space.
pub fn agent_delegation_list_path(space_id: &str) -> Result<String, &'static str> {
    identifier_ok(space_id).then(|| format!("{}?space={}", contract().paths["delegations"], encode_component(space_id))).ok_or("directory.delegations.invalid-space")
}

/// 🚫️ The revoke path of one delegation (`POST`, empty body).
pub fn agent_delegation_revoke_path(delegation_id: &str) -> Result<String, &'static str> {
    identifier_ok(delegation_id).then(|| format!("{}/{}/revoke", contract().paths["delegations"], encode_component(delegation_id))).ok_or("directory.delegations.invalid-delegation")
}

/// 🏗️ The exact create body, held against the hub's own bounds so a refusal happens here with a cause.
pub fn create_agent_delegation_body(space_id: &str, agent_label: &str, audience: AgentAudience, ttl_secs: u64) -> Result<String, &'static str> {
    if !identifier_ok(space_id) {
        return Err("directory.delegations.invalid-space");
    }
    let label = agent_label.trim();
    if label.is_empty() || label.len() > contract().bounds.label_max_bytes || label.chars().any(char::is_control) {
        return Err("directory.delegations.invalid-label");
    }
    if ttl_secs < contract().bounds.min_ttl_secs || ttl_secs > contract().bounds.max_ttl_secs {
        return Err("directory.delegations.invalid-ttl");
    }
    Ok(format!(r#"{{"schema":{},"spaceId":{},"agentLabel":{},"audience":{},"ttlSecs":{ttl_secs}}}"#, json_string(&contract().schemas["create"]), json_string(space_id), json_string(label), json_string(audience.as_str())))
}

/// 🎁️ The one-time creation receipt; `token` is readable exactly once.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentDelegationReceipt {
    pub delegation_id: String,
    pub agent_label: String,
    pub space_id: String,
    pub audience: AgentAudience,
    pub expires_at_ms: i64,
    pub token: String,
}

fn bounded_text<'a>(row: &'a serde_json::Value, key: &str, max: usize) -> Option<&'a str> {
    row.get(key).and_then(serde_json::Value::as_str).filter(|value| !value.is_empty() && value.encode_utf16().count() <= max && !value.chars().any(char::is_control))
}

fn safe_ms(row: &serde_json::Value, key: &str) -> Option<i64> {
    row.get(key).and_then(serde_json::Value::as_i64).filter(|value| value.unsigned_abs() <= 9_007_199_254_740_991)
}

/// 📥️ Reads one receipt: every field bounds-checked, the token exactly a delegation capability, the principal derived.
pub fn parse_agent_delegation_receipt(source: &str) -> Result<AgentDelegationReceipt, &'static str> {
    let malformed = "directory.delegations.malformed-receipt";
    let value: serde_json::Value = serde_json::from_str(source).map_err(|_| malformed)?;
    if value.get("schema").and_then(serde_json::Value::as_str) != Some(contract().schemas["receipt"].as_str()) {
        return Err(malformed);
    }
    let bounds = &contract().bounds;
    let delegation_id = bounded_text(&value, "delegationId", bounds.identifier_max).ok_or(malformed)?;
    let principal = bounded_text(&value, "agentPrincipalId", bounds.identifier_max + 8).ok_or(malformed)?;
    let agent_label = bounded_text(&value, "agentLabel", bounds.label_max_bytes).ok_or(malformed)?;
    let space_id = bounded_text(&value, "spaceId", bounds.identifier_max).ok_or(malformed)?;
    let expires_at_ms = safe_ms(&value, "expiresAtMs").ok_or(malformed)?;
    let audience = value.get("audience").and_then(serde_json::Value::as_str).and_then(AgentAudience::parse).ok_or(malformed)?;
    let token = value.get("token").and_then(serde_json::Value::as_str).filter(|token| is_agent_delegation_token(token)).ok_or(malformed)?;
    if principal != agent_principal_id(delegation_id) {
        return Err(malformed);
    }
    Ok(AgentDelegationReceipt { delegation_id: delegation_id.to_string(), agent_label: agent_label.to_string(), space_id: space_id.to_string(), audience, expires_at_ms, token: token.to_string() })
}

/// 📥️ Reads the listing: a malformed row is dropped, never the listing.
pub fn parse_agent_delegation_list(source: &str) -> Vec<AgentDelegationSummary> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(source) else { return Vec::new() };
    let Some(rows) = value.get("delegations").and_then(serde_json::Value::as_array) else { return Vec::new() };
    let bounds = &contract().bounds;
    rows.iter()
        .take(bounds.page_max)
        .filter_map(|row| {
            Some(AgentDelegationSummary {
                delegation_id: bounded_text(row, "delegationId", bounds.identifier_max)?.to_string(),
                agent_label: bounded_text(row, "agentLabel", bounds.label_max_bytes)?.to_string(),
                space_id: bounded_text(row, "spaceId", bounds.identifier_max)?.to_string(),
                created_at_ms: safe_ms(row, "createdAtMs")?,
                expires_at_ms: safe_ms(row, "expiresAtMs")?,
                audience: row.get("audience").and_then(serde_json::Value::as_str).and_then(AgentAudience::parse)?,
                revoked: row.get("revoked").and_then(serde_json::Value::as_bool) == Some(true),
                last_used_at_ms: safe_ms(row, "lastUsedAtMs"),
            })
        })
        .collect()
}
//#endregion 🏗️Requests

//#region 🔖️Credential
/// 📄️ One downloadable credential file: the name and its exact bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentCredentialFile {
    pub file_name: String,
    pub contents: String,
}

fn label_slug(label: &str, empty: &str) -> String {
    let mut slug = String::new();
    for character in label.to_lowercase().chars() {
        if character.is_ascii_lowercase() || character.is_ascii_digit() {
            slug.push(character);
        } else if !slug.ends_with('-') && !slug.is_empty() {
            slug.push('-');
        }
    }
    let slug: String = slug.trim_matches('-').chars().take(48).collect();
    if slug.is_empty() { empty.to_string() } else { slug }
}

/// 📄️ A file name safe on every filesystem that still says which agent it belongs to.
pub fn agent_credential_file_name(receipt: &AgentDelegationReceipt) -> String {
    format!("semio-agent-{}.json", label_slug(&receipt.agent_label, "delegation"))
}

/// 📄️ The file `semio-os-mcp --credential-file` decodes, in the schema's key order; the only place the token leaves the receipt.
pub fn agent_credential_file(receipt: &AgentDelegationReceipt, hub_origin: &str) -> Result<AgentCredentialFile, &'static str> {
    if hub_origin.is_empty() || hub_origin.chars().any(char::is_control) {
        return Err("directory.delegations.invalid-origin");
    }
    if !is_agent_delegation_token(&receipt.token) {
        return Err("directory.delegations.invalid-token");
    }
    let contents = format!(
        "{{\n  \"schema\": {},\n  \"hubOrigin\": {},\n  \"spaceId\": {},\n  \"audience\": {},\n  \"token\": {}\n}}\n",
        json_string(&contract().schemas["credential"]),
        json_string(hub_origin),
        json_string(&receipt.space_id),
        json_string(receipt.audience.as_str()),
        json_string(&receipt.token)
    );
    Ok(AgentCredentialFile { file_name: agent_credential_file_name(receipt), contents })
}

/// 🖥️ The exact command line that makes the saved file an agent principal in this space.
pub fn agent_credential_command(receipt: &AgentDelegationReceipt, hub_origin: &str) -> String {
    format!("semio-os-mcp stdio --hub {hub_origin} --space {} --credential-file <path>", receipt.space_id)
}
//#endregion 🔖️Credential

//#region 🔌️McpClient
/// 🛰️ The loopback route a development host serves to install one credential file (and the browser door posts to).
pub fn agent_credential_install_endpoint() -> &'static str {
    &contract().paths["install"]
}

/// 🏷️ The installed credential's file name, derived from the delegation id so two delegations never share a file.
pub fn agent_credential_install_file_name(delegation_id: &str) -> Result<String, &'static str> {
    if delegation_id.is_empty() || delegation_id.len() > 128 || !delegation_id.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-')) {
        return Err("directory.delegations.invalid-delegation");
    }
    Ok(format!("semio-agent-{delegation_id}.json"))
}

/// 📥️ The install request body for one freshly minted delegation: its id and the exact credential file bytes.
pub fn agent_credential_install_request(receipt: &AgentDelegationReceipt, hub_origin: &str) -> Result<String, &'static str> {
    agent_credential_install_file_name(&receipt.delegation_id)?;
    let file = agent_credential_file(receipt, hub_origin)?;
    Ok(format!("{{\"schema\":{},\"delegationId\":{},\"contents\":{}}}", json_string(&contract().schemas["installRequest"]), json_string(&receipt.delegation_id), json_string(&file.contents)))
}

/// 🧾️ What a host answers after installing: the owner-only credential path and how it starts the MCP gateway.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentCredentialInstallReceipt {
    pub credential_path: String,
    pub launcher_command: String,
    pub launcher_args: Vec<String>,
}

/// 🧾️ Parses a host's install receipt, refusing anything that is not an absolute credential path and a non-empty launcher.
pub fn parse_agent_credential_install_receipt(body: &str) -> Result<AgentCredentialInstallReceipt, &'static str> {
    const INVALID: &str = "directory.delegations.invalid-install-receipt";
    let value: serde_json::Value = serde_json::from_str(body).map_err(|_| INVALID)?;
    let path = value.get("credentialPath").and_then(serde_json::Value::as_str).ok_or(INVALID)?;
    let absolute = path.starts_with('/') || (path.len() > 2 && path.as_bytes()[0].is_ascii_alphabetic() && path.as_bytes()[1] == b':' && matches!(path.as_bytes()[2], b'/' | b'\\'));
    let command = value.pointer("/launcher/command").and_then(serde_json::Value::as_str).filter(|command| !command.is_empty()).ok_or(INVALID)?;
    let args = value.pointer("/launcher/args").and_then(serde_json::Value::as_array).ok_or(INVALID)?.iter().map(|arg| arg.as_str().map(str::to_string)).collect::<Option<Vec<_>>>().ok_or(INVALID)?;
    if value.get("schema").and_then(serde_json::Value::as_str) != Some(contract().schemas["installReceipt"].as_str()) || !absolute {
        return Err(INVALID);
    }
    Ok(AgentCredentialInstallReceipt { credential_path: path.to_string(), launcher_command: command.to_string(), launcher_args: args })
}

/// 🏷️ The server name a client lists this agent under: `semio-` plus the label's slug.
pub fn agent_mcp_server_name(receipt: &AgentDelegationReceipt) -> String {
    format!("semio-{}", label_slug(&receipt.agent_label, "agent"))
}

/// 🎟️ The MCP scopes a client for one audience is granted.
pub fn agent_mcp_scopes(audience: AgentAudience) -> &'static str {
    match audience {
        AgentAudience::Read => "workspace.read",
        AgentAudience::Edit => "workspace.read,artifact.write,inference.execute",
    }
}

/// 📄️ The complete MCP client configuration text one delegation produces (`JSON.stringify(config, null, 2)` plus a newline):
/// the host's launcher bound to the hub and space, authenticated by the installed file, never carrying the token.
pub fn agent_mcp_client_config(receipt: &AgentDelegationReceipt, hub_origin: &str, installed: &AgentCredentialInstallReceipt) -> Result<String, &'static str> {
    if hub_origin.is_empty() || hub_origin.chars().any(char::is_control) {
        return Err("directory.delegations.invalid-origin");
    }
    let mut args = installed.launcher_args.clone();
    args.extend(["--hub", hub_origin, "--space", receipt.space_id.as_str(), "--credential-file", installed.credential_path.as_str(), "--scopes", agent_mcp_scopes(receipt.audience)].map(str::to_string));
    let args = args.iter().map(|arg| format!("        {}", json_string(arg))).collect::<Vec<_>>().join(",\n");
    Ok(format!(
        "{{\n  \"mcpServers\": {{\n    {}: {{\n      \"type\": \"stdio\",\n      \"command\": {},\n      \"args\": [\n{args}\n      ]\n    }}\n  }}\n}}\n",
        json_string(&agent_mcp_server_name(receipt)),
        json_string(&installed.launcher_command)
    ))
}
//#endregion 🔌️McpClient

//#region 🚫️Errors
/// 🚫️ Closed delegation failure classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentErrorCode {
    MalformedRequest,
    InvalidDelegation,
    DelegationRevoked,
    DelegationExpired,
    Forbidden,
    RateLimited,
    DirectoryUnavailable,
    Unreachable,
    Cancelled,
}

impl AgentErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MalformedRequest => "malformed-request",
            Self::InvalidDelegation => "invalid-delegation",
            Self::DelegationRevoked => "delegation-revoked",
            Self::DelegationExpired => "delegation-expired",
            Self::Forbidden => "forbidden",
            Self::RateLimited => "rate-limited",
            Self::DirectoryUnavailable => "directory-unavailable",
            Self::Unreachable => "unreachable",
            Self::Cancelled => "cancelled",
        }
    }

    fn label_key(self) -> &'static str {
        match self {
            Self::Forbidden | Self::InvalidDelegation => "errorForbidden",
            Self::RateLimited => "errorRateLimited",
            Self::Unreachable | Self::DirectoryUnavailable => "errorUnreachable",
            _ => "errorRefused",
        }
    }
}

/// 🌐️ One response status to its closed code.
pub fn agent_error_from_status(status: u16) -> AgentErrorCode {
    match status {
        400 => AgentErrorCode::MalformedRequest,
        401 => AgentErrorCode::InvalidDelegation,
        403 => AgentErrorCode::Forbidden,
        429 => AgentErrorCode::RateLimited,
        503 => AgentErrorCode::DirectoryUnavailable,
        _ => AgentErrorCode::Unreachable,
    }
}

/// 🚫️ The hub's own `agent-error/v1` body when it sent one, else the status.
pub fn agent_error_from_response(status: u16, body: &str) -> AgentErrorCode {
    let named = serde_json::from_str::<serde_json::Value>(body).ok().filter(|value| value.get("schema").and_then(serde_json::Value::as_str) == Some("semio.hub.auth.agent-error/v1")).and_then(|value| value.get("error").and_then(serde_json::Value::as_str).map(str::to_string));
    [AgentErrorCode::MalformedRequest, AgentErrorCode::InvalidDelegation, AgentErrorCode::DelegationRevoked, AgentErrorCode::DelegationExpired, AgentErrorCode::Forbidden, AgentErrorCode::RateLimited, AgentErrorCode::DirectoryUnavailable]
        .into_iter()
        .find(|code| Some(code.as_str()) == named.as_deref())
        .unwrap_or_else(|| agent_error_from_status(status))
}
//#endregion 🚫️Errors

//#region 🏛️State
/// 🔄️ The pane's own load/mutate phase.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentPhase {
    #[default]
    Idle,
    Loading,
    Ready,
    Submitting,
    Failed,
}

impl AgentPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Loading => "loading",
            Self::Ready => "ready",
            Self::Submitting => "submitting",
            Self::Failed => "failed",
        }
    }
}

/// 💾️ Whether the credential file was saved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentSave {
    Pending,
    Saved,
    Failed,
}

/// 🔌️ The MCP client block's phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentMcpPhase {
    Idle,
    Installing,
    Ready,
    Copied,
    Unavailable,
    Failed,
}

impl AgentMcpPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Installing => "installing",
            Self::Ready => "ready",
            Self::Copied => "copied",
            Self::Unavailable => "unavailable",
            Self::Failed => "failed",
        }
    }
}

/// 🎁️ The one-time disclosure: held in memory only while it is shown; dismissing destroys the only copy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentCredential {
    pub receipt: AgentDelegationReceipt,
    pub file: AgentCredentialFile,
    pub command: String,
    pub save: AgentSave,
    pub mcp: AgentMcpPhase,
    pub mcp_config: Option<String>,
}

/// 🤖️ Everything the delegations pane paints, in one owned value.
#[derive(Clone, Debug, PartialEq)]
pub struct HubAgentState {
    pub phase: AgentPhase,
    pub error: Option<AgentErrorCode>,
    pub rows: Vec<AgentDelegationRow>,
    pub credential: Option<AgentCredential>,
    pub name_draft: String,
    pub audience: AgentAudience,
    pub ttl_secs: u64,
    pub revoke_confirm: Option<String>,
    pub space_id: Option<String>,
}

impl Default for HubAgentState {
    fn default() -> Self {
        Self {
            phase: AgentPhase::Idle,
            error: None,
            rows: Vec::new(),
            credential: None,
            name_draft: String::new(),
            audience: AgentAudience::parse(&contract().default_audience).unwrap_or(AgentAudience::Edit),
            ttl_secs: contract().ttl_choices_secs[contract().default_ttl_index],
            revoke_confirm: None,
            space_id: None,
        }
    }
}

impl HubAgentState {
    /// 🔄️ Starts a listing for `space_id`; rows of another space are dropped, never shown under the wrong space.
    pub fn begin_load(&mut self, space_id: &str) {
        if self.space_id.as_deref() != Some(space_id) {
            self.rows.clear();
            self.revoke_confirm = None;
        }
        self.space_id = Some(space_id.to_string());
        self.phase = AgentPhase::Loading;
        self.error = None;
    }

    /// 📥️ A listing landed.
    pub fn loaded(&mut self, summaries: &[AgentDelegationSummary], now_ms: i64) {
        self.rows = agent_delegation_rows(summaries, now_ms);
        self.phase = AgentPhase::Ready;
    }

    /// 🚫️ A call failed: the phase is `Failed` and the closed cause is named.
    pub fn failed(&mut self, error: AgentErrorCode) {
        self.phase = AgentPhase::Failed;
        self.error = Some(error);
    }

    /// 🎁️ A delegation was minted: the one-time credential is held, the name draft cleared.
    pub fn minted(&mut self, receipt: AgentDelegationReceipt, hub_origin: &str) -> Result<(), &'static str> {
        let file = agent_credential_file(&receipt, hub_origin)?;
        self.credential = Some(AgentCredential { command: agent_credential_command(&receipt, hub_origin), receipt, file, save: AgentSave::Pending, mcp: AgentMcpPhase::Idle, mcp_config: None });
        self.name_draft.clear();
        self.phase = AgentPhase::Ready;
        self.error = None;
        Ok(())
    }

    /// 💾️ The credential file the download verb hands to the host's save door: its name and exact text.
    pub fn download_file(&self) -> Option<(String, String)> {
        self.credential.as_ref().map(|credential| (credential.file.file_name.clone(), credential.file.contents.clone()))
    }

    /// 💾️ The save door answered for `delegation_id`; an answer for a credential no longer shown is ignored.
    pub fn download_settled(&mut self, delegation_id: &str, saved: bool) {
        if let Some(credential) = self.credential.as_mut().filter(|credential| credential.receipt.delegation_id == delegation_id) {
            credential.save = if saved { AgentSave::Saved } else { AgentSave::Failed };
        }
    }

    /// 🔌️ Starts the MCP install: the request body for the host and the delegation it belongs to; the block shows `installing`.
    pub fn begin_install(&mut self, hub_origin: &str) -> Option<(String, String)> {
        let credential = self.credential.as_mut().filter(|credential| credential.mcp == AgentMcpPhase::Idle)?;
        match agent_credential_install_request(&credential.receipt, hub_origin) {
            Ok(body) => {
                credential.mcp = AgentMcpPhase::Installing;
                Some((credential.receipt.delegation_id.clone(), body))
            }
            Err(_) => {
                credential.mcp = AgentMcpPhase::Failed;
                None
            }
        }
    }

    /// 🔌️ The host answered the install for `delegation_id` (`status` `None` = it never answered): 404/405 means the host serves
    /// no install (`unavailable`, not a failure), another non-2xx or an unreadable receipt `failed`, else the config is shown.
    pub fn install_settled(&mut self, delegation_id: &str, hub_origin: &str, status: Option<u16>, body: &str) {
        let Some(credential) = self.credential.as_mut().filter(|credential| credential.receipt.delegation_id == delegation_id) else { return };
        let ready = status.filter(|status| (200..300).contains(status)).and_then(|_| parse_agent_credential_install_receipt(body).ok()).and_then(|installed| agent_mcp_client_config(&credential.receipt, hub_origin, &installed).ok());
        match (status, ready) {
            (_, Some(config)) => {
                credential.mcp = AgentMcpPhase::Ready;
                credential.mcp_config = Some(config);
            }
            (Some(404 | 405), None) => credential.mcp = AgentMcpPhase::Unavailable,
            _ => credential.mcp = AgentMcpPhase::Failed,
        }
    }

    /// 📋️ The MCP configuration text the copy control writes, once an install produced one.
    pub fn mcp_config(&self) -> Option<&str> {
        self.credential.as_ref().and_then(|credential| credential.mcp_config.as_deref())
    }

    /// 📋️ The clipboard door took the configuration of `delegation_id`.
    pub fn mcp_copied(&mut self, delegation_id: &str) {
        if let Some(credential) = self.credential.as_mut().filter(|credential| credential.receipt.delegation_id == delegation_id && credential.mcp_config.is_some()) {
            credential.mcp = AgentMcpPhase::Copied;
        }
    }

    /// 🗑️ Dismissing destroys the only copy of the credential.
    pub fn dismiss_credential(&mut self) {
        self.credential = None;
    }

    /// 🗑️ Opens the inline withdrawal confirmation of one revocable delegation.
    pub fn ask_revoke(&mut self, delegation_id: &str) {
        if self.rows.iter().any(|row| row.summary.delegation_id == delegation_id && agent_delegation_revocable(row)) {
            self.revoke_confirm = Some(delegation_id.to_string());
        }
    }

    /// 🗑️ Closes the confirmation without withdrawing.
    pub fn cancel_revoke(&mut self) {
        self.revoke_confirm = None;
    }

    /// 🗑️ Confirms: yields the id to withdraw and closes the confirmation.
    pub fn confirm_revoke(&mut self) -> Option<String> {
        self.revoke_confirm.take()
    }

    /// ✅️ Whether the create form may submit: a non-empty, control-free name.
    pub fn can_create(&self) -> bool {
        !self.name_draft.trim().is_empty() && !self.name_draft.chars().any(char::is_control)
    }
}
//#endregion 🏛️State

//#region 🌲️Tree
/// 🧭️ What the pane needs to know about the surrounding workspace.
#[derive(Clone, Copy, Debug)]
pub struct AgentSurfaceFacts<'a> {
    pub signed_in: bool,
    pub space_id: Option<&'a str>,
    pub can_delegate: bool,
}

/// 🧮️ How many retained nodes a subtree costs — the panel document is bounded.
pub fn node_count(node: &UiNode) -> usize {
    match node {
        UiNode::Stack(stack) => 1 + stack.children.iter().map(node_count).sum::<usize>(),
        _ => 1,
    }
}

fn credential_block(credential: &AgentCredential, locale: Locale) -> UiNode {
    let principal = agent_principal_id(&credential.receipt.delegation_id);
    let (download_id, download_icon) = control("download");
    let (dismiss_id, dismiss_icon) = control("dismiss");
    let (install_id, install_icon) = control("mcpInstall");
    let mut children = vec![
        tagged_row(agent_label(locale, "ready"), &[("data-semio-hub-agent-credential", credential.receipt.delegation_id.as_str())]),
        tagged_row(&agent_label_with(locale, "principal", &[("principal", principal.as_str())]), &[("data-semio-hub-agent-principal", principal.as_str())]),
        button(download_id, download_icon, agent_label(locale, "download"), descriptor(action::AGENT_DOWNLOAD, None), true),
        button(dismiss_id, dismiss_icon, agent_label(locale, "dismiss"), descriptor(action::AGENT_DISMISS, None), true),
        text_row(agent_label(locale, "permission")),
        text_row(agent_label(locale, "command")),
        tagged_row(&credential.command, &[("data-semio-hub-agent-command", "true")]),
        tagged_row(agent_label(locale, "mcpTitle"), &[("data-semio-hub-agent-mcp", credential.mcp.as_str())]),
    ];
    if matches!(credential.mcp, AgentMcpPhase::Idle | AgentMcpPhase::Installing) {
        children.push(button(install_id, install_icon, agent_label(locale, "mcpInstall"), descriptor(action::AGENT_INSTALL_MCP, None), credential.mcp == AgentMcpPhase::Idle));
    }
    match credential.mcp {
        AgentMcpPhase::Installing => children.push(text_row(agent_label(locale, "mcpInstalling"))),
        AgentMcpPhase::Copied => children.push(text_row(agent_label(locale, "mcpCopied"))),
        AgentMcpPhase::Unavailable => children.push(text_row(agent_label(locale, "mcpUnavailable"))),
        AgentMcpPhase::Failed => children.push(text_row(agent_label(locale, "mcpFailed"))),
        AgentMcpPhase::Idle | AgentMcpPhase::Ready => {}
    }
    if let Some(config) = &credential.mcp_config {
        let (copy_id, copy_icon) = control("mcpCopy");
        children.push(text_row(agent_label(locale, "mcpReady")));
        children.push(tagged_row(config, &[("data-semio-hub-agent-mcp-config", "true")]));
        children.push(button(copy_id, copy_icon, agent_label(locale, "mcpCopy"), descriptor(action::AGENT_COPY_MCP, None), true));
        children.push(text_row(agent_label(locale, "mcpRevocable")));
    }
    match credential.save {
        AgentSave::Saved => children.push(text_row(agent_label(locale, "downloaded"))),
        AgentSave::Failed => {
            children.push(tagged_row(agent_label(locale, "downloadFailed"), &[("data-semio-hub-agent-save", "failed")]));
            children.push(tagged_row(&credential.file.contents, &[("data-semio-hub-agent-credential-file", credential.file.file_name.as_str())]));
        }
        AgentSave::Pending => {}
    }
    stack("os.hub.agent.credential", children)
}

fn create_form(state: &HubAgentState, busy: bool, locale: Locale) -> UiNode {
    let (create_id, create_icon) = control("create");
    let fields = &contract().fields;
    let mut children = vec![
        text_row(agent_label(locale, "createTitle")),
        input(&fields["name"], &state.name_draft, agent_label(locale, "name"), action::AGENT_SET_NAME, Some(action::AGENT_CREATE)),
        text_row(agent_label(locale, "audience")),
    ];
    let audiences: Vec<(&str, &str)> = AgentAudience::ALL.iter().map(|audience| (audience.as_str(), agent_label(locale, if *audience == AgentAudience::Edit { "audienceEdit" } else { "audienceRead" }))).collect();
    children.push(select(&fields["audience"], state.audience.as_str(), &audiences, action::AGENT_SET_AUDIENCE, !busy));
    children.push(text_row(agent_label(locale, "expiry")));
    let ttls: Vec<(String, &str)> = agent_ttl_choices_secs().iter().zip(["expiryDay", "expiryWeek", "expiryMonth"]).map(|(ttl, key)| (ttl.to_string(), agent_label(locale, key))).collect();
    let ttls: Vec<(&str, &str)> = ttls.iter().map(|(ttl, label)| (ttl.as_str(), *label)).collect();
    children.push(select(&fields["expiry"], &state.ttl_secs.to_string(), &ttls, action::AGENT_SET_EXPIRY, !busy));
    children.push(button(create_id, create_icon, agent_label(locale, "create"), descriptor(action::AGENT_CREATE, None), !busy && state.can_create()));
    stack("os.hub.agent.form", children)
}

fn delegation_row(row: &AgentDelegationRow, state: &HubAgentState, busy: bool, locale: Locale) -> Vec<UiNode> {
    let summary = &row.summary;
    let state_label = agent_label(locale, match row.state {
        AgentDelegationState::Live => "stateLive",
        AgentDelegationState::Expired => "stateExpired",
        AgentDelegationState::Revoked => "stateRevoked",
    });
    let audience_label = agent_label(locale, if summary.audience == AgentAudience::Edit { "audienceEdit" } else { "audienceRead" });
    let usage = summary.last_used_at_ms.map_or_else(|| agent_label(locale, "lastUsedNever").to_string(), |ms| agent_label_with(locale, "lastUsed", &[("when", &agent_delegation_instant(Some(ms)))]));
    let created = agent_label_with(locale, "created", &[("when", &agent_delegation_instant(Some(summary.created_at_ms)))]);
    let expires = agent_label_with(locale, "expires", &[("when", &agent_delegation_instant(Some(summary.expires_at_ms)))]);
    let principal = agent_principal_id(&summary.delegation_id);
    let mut nodes = vec![
        tagged_row(&format!("{} — {state_label} · {audience_label} · {usage}", summary.agent_label), &[("data-delegation-id", summary.delegation_id.as_str()), ("data-delegation-state", row.state.as_str()), ("data-delegation-audience", summary.audience.as_str())]),
        tagged_row(&format!("{principal} · {created} · {expires}"), &[("data-delegation-principal", principal.as_str())]),
    ];
    let (revoke_id, revoke_icon) = control("revoke");
    let (confirm_id, confirm_icon) = control("revokeConfirm");
    let (cancel_id, cancel_icon) = control("revokeCancel");
    if state.revoke_confirm.as_deref() == Some(summary.delegation_id.as_str()) {
        let title = agent_label_with(locale, "revokeConfirmTitle", &[("name", &summary.agent_label)]);
        nodes.push(tagged_row(&title, &[("data-semio-hub-agent-revoke-confirm", summary.delegation_id.as_str())]));
        nodes.push(text_row(agent_label(locale, "revokeConfirmBody")));
        nodes.push(button(&format!("{confirm_id}.{}", summary.delegation_id), confirm_icon, agent_label(locale, "revokeConfirm"), descriptor(action::AGENT_REVOKE_CONFIRM, None), !busy));
        nodes.push(button(&format!("{cancel_id}.{}", summary.delegation_id), cancel_icon, agent_label(locale, "revokeCancel"), descriptor(action::AGENT_REVOKE_CANCEL, None), true));
    } else if agent_delegation_revocable(row) {
        nodes.push(button(&format!("{revoke_id}.{}", summary.delegation_id), revoke_icon, &agent_label_with(locale, "revoke", &[("name", &summary.agent_label)]), descriptor(action::AGENT_REVOKE_ASK, one_arg("delegationId", &summary.delegation_id)), !busy));
    }
    nodes
}

/// 🤖️ The delegations pane as one retained section: a live status line, the one-time credential block, the create form where
/// the caller may author, and the rows (revoked ones included). `max_nodes` is the node budget the panel document leaves it —
/// rows past it are summarised by a count line, never silently dropped.
pub fn build_agent_delegations_ui(state: &HubAgentState, facts: AgentSurfaceFacts<'_>, locale: Locale, max_nodes: usize) -> UiNode {
    let busy = state.phase == AgentPhase::Submitting;
    let (refresh_id, refresh_icon) = control("refresh");
    let mut children = vec![
        tagged_row(agent_label(locale, "title"), &[("data-semio-hub-agent-phase", state.phase.as_str()), ("data-semio-hub-agent-space", facts.space_id.unwrap_or(""))]),
        button(refresh_id, refresh_icon, agent_label(locale, "refresh"), descriptor(action::AGENT_REFRESH, None), !busy && facts.space_id.is_some()),
        text_row(agent_label(locale, "description")),
    ];
    let status = match state.phase {
        AgentPhase::Loading => "loading",
        AgentPhase::Submitting => "submitting",
        AgentPhase::Failed => "failed",
        _ => "",
    };
    if !status.is_empty() {
        children.push(tagged_row(agent_label(locale, status), &[("data-semio-hub-agent-status", state.phase.as_str())]));
    }
    if let Some(error) = state.error {
        children.push(tagged_row(agent_label(locale, error.label_key()), &[("data-semio-hub-agent-error", error.as_str())]));
    }
    let active = facts.signed_in && facts.space_id.is_some();
    if !facts.signed_in {
        children.push(text_row(agent_label(locale, "signedOut")));
    } else if facts.space_id.is_none() {
        children.push(text_row(agent_label(locale, "noSpace")));
    }
    if let Some(credential) = &state.credential {
        children.push(credential_block(credential, locale));
    }
    if active {
        children.push(if facts.can_delegate { create_form(state, busy, locale) } else { text_row(agent_label(locale, "notAuthor")) });
        if state.rows.is_empty() {
            children.push(text_row(agent_label(locale, "empty")));
        } else {
            let mut used = 1 + children.iter().map(node_count).sum::<usize>() + 2;
            let mut shown = 0;
            let mut rows = Vec::new();
            for row in &state.rows {
                let nodes = delegation_row(row, state, busy, locale);
                let cost = nodes.iter().map(node_count).sum::<usize>();
                if used + cost > max_nodes {
                    break;
                }
                used += cost;
                shown += 1;
                rows.extend(nodes);
            }
            let hidden = state.rows.len() - shown;
            if hidden > 0 {
                rows.push(text_row(&format!("+{hidden}")));
            }
            children.push(stack(&contract().fields["list"], rows));
        }
    }
    stack("os.hub.agent", children)
}
//#endregion 🌲️Tree

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
