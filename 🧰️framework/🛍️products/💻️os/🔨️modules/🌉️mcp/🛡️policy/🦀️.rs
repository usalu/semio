//! 🛡️ Policy engine — packet `P6-actions-policy`, `📋️master.md` §3.4. `AgentPrincipal` carries the
//! effective, ALREADY-EXPANDED `kernel::CapabilityId` scope set a session was granted (from the CLI
//! flags P1a's `StdioOptions`/`HttpOptions` already parse); [`PolicyEngine`] is the pure decision
//! layer `🎬️actions`' `ActionAdapter` calls for scope enforcement and the approval gate. Depends on
//! `semio_framework::manifest::{kernel, ApprovalMode}` only — the SAME one hop `🗂️catalog` already
//! takes (D8: no plugin/channel/actor dependency), and on `crate::handles` (P1b) for `HandleTable`/
//! `Attachment`/`HandleKind`/`SessionHandle`. Zero dependency on `crate::audit` — every decision this
//! facet makes is reported by ITS CALLER (`🎬️actions`, which owns the `AuditSink`), so this facet
//! stays testable without an audit sink at all.

use crate::catalog::CapabilityDefinition;
use crate::errors::{GatewayError, GatewayErrorCode};
use crate::handles::{Attachment, HandleKind, HandleTable, SessionHandle};
use semio_framework::manifest::{kernel, ApprovalMode};
use semio_framework_os_kernel::{DslValue, FromValue, ToValue, ValueError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

//#region 🔖️ScopeTable
/// 📇️ `📋️master.md` §3.4's scope table, verbatim — the left column is the CLI-facing MCP scope name
/// (`--scopes artifact.write,ui.control`), the right column the `kernel::CapabilityId` strings it
/// expands to. Parametrized families (`fs.read:<root>`, `fs.write:<root>`, `http:<origin>`,
/// `secrets:<name>`) expand to a `<prefix>*` wildcard grant here — [`AgentPrincipal::grants`] treats a
/// `<prefix>*` entry as "every concrete scope under this prefix", matching `🧪️conformance`'s own
/// `KNOWN_SCOPE_PREFIXES` four-family list.
pub const MCP_SCOPE_TABLE: &[(&str, &[&str])] = &[
    ("workspace.read", &["registry.query", "documents.read"]),
    ("artifact.read", &["documents.read"]),
    ("artifact.write", &["documents.write", "jobs.spawn"]),
    ("ui.observe", &["shell.observe"]),
    ("ui.control", &["shell.control", "ui.window", "ui.dialog", "shell.navigate"]),
    ("ui.raw-control", &["shell.raw"]),
    ("clipboard.read", &["shell.clipboard"]),
    ("clipboard.write", &["shell.clipboard"]),
    ("host.filesystem.read", &["fs.read:*"]),
    ("host.filesystem.write", &["fs.write:*"]),
    ("network.external", &["http:*"]),
    ("process.spawn", &["process.spawn"]),
    ("plugin.install", &["packages.install"]),
    ("extension.install", &["packages.install"]),
    ("secrets.use", &["secrets:*"]),
];

/// 🔤️ Every `kernel::CapabilityId` string one raw `--scopes` entry expands to — an entry that does
/// not name a row in [`MCP_SCOPE_TABLE`] passes through LITERALLY (a caller may also grant a bare
/// `CapabilityId` directly, e.g. `documents.write` or `fs.read:/tmp/workspace`, without going through
/// the MCP-scope alias layer at all).
pub fn expand_scope(raw: &str) -> Vec<String> {
    for (name, targets) in MCP_SCOPE_TABLE {
        if *name == raw {
            return targets.iter().map(|target| target.to_string()).collect();
        }
    }
    vec![raw.to_string()]
}

/// 🔒️ The four scope families a `<prefix>*` wildcard grant covers — `🧪️conformance`'s
/// `KNOWN_SCOPE_PREFIXES` verbatim.
const WILDCARD_SCOPE_PREFIXES: [&str; 4] = ["fs.read:", "fs.write:", "http:", "secrets:"];
//#endregion 🔖️ScopeTable

//#region 🔖️AgentPrincipal
/// 🪪️ `📋️master.md` §3.4: `AgentPrincipal{id, kind: Agent, scopes, delegated_by, hub_token}` — `kind`
/// is implicit (this type only ever represents an agent principal) and `hub_token` is deferred with
/// the rest of hub identity (D7); `label` is this packet's own addition, a human-readable name for
/// audit/approval UI (`AgentAuditEvent.principal` carries `id`, not `label`).
#[derive(Clone, Debug, PartialEq)]
pub struct AgentPrincipal {
    pub id: String,
    pub label: String,
    pub scopes: Vec<kernel::CapabilityId>,
    pub delegated_by: Option<String>,
}

impl AgentPrincipal {
    /// 🏭️ Expands every raw `--scopes` entry through [`expand_scope`], dedupes, and returns the
    /// principal — the one constructor `run_stdio`/`run_http`/tests use.
    pub fn from_scope_names(id: impl Into<String>, label: impl Into<String>, raw_scopes: &[String], delegated_by: Option<String>) -> Self {
        let mut scopes: Vec<kernel::CapabilityId> = Vec::new();
        for raw in raw_scopes {
            for expanded in expand_scope(raw) {
                let capability_id = kernel::CapabilityId(expanded);
                if !scopes.contains(&capability_id) {
                    scopes.push(capability_id);
                }
            }
        }
        Self { id: id.into(), label: label.into(), scopes, delegated_by }
    }

    /// ✅️ Whether this principal's effective scope set covers `required` — an exact match, or a
    /// `<prefix>*` wildcard grant covering one of the four parametrized families.
    pub fn grants(&self, required: &kernel::CapabilityId) -> bool {
        if self.scopes.contains(required) {
            return true;
        }
        for prefix in WILDCARD_SCOPE_PREFIXES {
            if required.0.starts_with(prefix) {
                let wildcard = kernel::CapabilityId(format!("{prefix}*"));
                if self.scopes.contains(&wildcard) {
                    return true;
                }
            }
        }
        false
    }
}
//#endregion 🔖️AgentPrincipal

//#region 🔖️AutoApprovePolicy
/// 🚦️ `--auto-approve never|readonly|all` — the headless fallback when no `elicitation`-capable
/// client is attached to resolve an `ApprovalRecord` interactively. `Never` (the `Default`) is the
/// safe default the brief's §3.2 requires: nothing bypasses the approval gate unless explicitly told
/// to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AutoApprovePolicy {
    #[default]
    Never,
    ReadonlyOnly,
    All,
}

impl AutoApprovePolicy {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "never" => Some(Self::Never),
            "readonly" => Some(Self::ReadonlyOnly),
            "all" => Some(Self::All),
            _ => None,
        }
    }

    fn allows(self, capability: &CapabilityDefinition) -> bool {
        match self {
            AutoApprovePolicy::All => true,
            AutoApprovePolicy::ReadonlyOnly => !capability.effects.destructive && capability.effects.writes.is_empty(),
            AutoApprovePolicy::Never => false,
        }
    }
}
//#endregion 🔖️AutoApprovePolicy

//#region 🔖️ApprovalRecord
/// 🧾️ The payload parked in a `HandleKind::Approval` handle — `decided: None` while pending,
/// `Some(true|false)` once a human (or `--auto-approve`) resolves it. `diff_summary` is the same
/// preview shape `PreparedActionReport.preview` carries, shown to whoever resolves the approval.
/// 🌉️ `serde_json::Value` ↔ `DslValue` bridge for `ApprovalRecord::diff_summary`, built on
/// `🌱️value/🦀️.rs`'s own infallible `From<&DslValue>`/`From<&serde_json::Value>` impls.
fn json_value_to_dsl(value: &serde_json::Value) -> DslValue {
    DslValue::from(value)
}

/// 🌉️ See [`json_value_to_dsl`] — the `FromValue` direction, infallible.
fn dsl_to_json_value(value: DslValue) -> Result<serde_json::Value, ValueError> {
    Ok(serde_json::Value::from(value))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ApprovalRecord {
    pub capability_id: String,
    pub principal_id: String,
    #[value(serialize_with = "json_value_to_dsl", deserialize_with = "dsl_to_json_value")]
    pub diff_summary: serde_json::Value,
    pub decided: Option<bool>,
}
//#endregion 🔖️ApprovalRecord

//#region 🔖️ApprovalGate
/// ⛩️ The result of gating one invocation against `capability.policy.approval` — `Required` carries
/// the freshly-minted `appr_` handle the caller (shell, or a resumed `action.invoke{approvalHandle}`)
/// must resolve before the invocation can proceed.
#[derive(Debug, Clone, PartialEq)]
pub enum ApprovalGate {
    Proceed,
    Required { approval_handle: String },
}
//#endregion 🔖️ApprovalGate

//#region 🔖️PolicyEngine
/// ⚖️ The pure decision layer — scope subset checks and the approval gate. Holds the SAME
/// `Arc<HandleTable>` `🎬️actions`' `ActionAdapter` holds (never a private copy), so an `appr_` handle
/// minted here is resolvable by `ActionAdapter::invoke`'s own handle lookups and vice versa.
pub struct PolicyEngine {
    handles: Arc<HandleTable>,
    auto_approve: AutoApprovePolicy,
}

impl PolicyEngine {
    pub fn new(handles: Arc<HandleTable>, auto_approve: AutoApprovePolicy) -> Self {
        Self { handles, auto_approve }
    }

    /// 🔐️ `capability.policy.scopes` must be a SUBSET of `principal`'s effective scopes — the first
    /// missing scope is reported (not the full missing set; one denial is enough to deny the whole
    /// call, and reporting the first keeps the message short).
    pub fn authorize_scopes(&self, principal: &AgentPrincipal, capability: &CapabilityDefinition) -> Result<(), GatewayError> {
        for scope in &capability.policy.scopes {
            if !principal.grants(scope) {
                return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, format!("principal {} lacks required scope {}", principal.id, scope.0)));
            }
        }
        Ok(())
    }

    /// 🚦️ `Never` → never; `WhenDestructive` → iff `effects.destructive`; `Always` → always — then
    /// `auto_approve` may still waive the requirement for the headless case.
    fn requires_approval(&self, capability: &CapabilityDefinition) -> bool {
        let required_by_mode = match capability.policy.approval {
            ApprovalMode::Never => false,
            ApprovalMode::WhenDestructive => capability.effects.destructive,
            ApprovalMode::Always => true,
        };
        required_by_mode && !self.auto_approve.allows(capability)
    }

    /// ⛩️ Checks whether `capability` needs approval; if it does and `existing_handle` names an
    /// already-decided (`decided: Some(true)`) approval for the SAME capability owned by `session`, the
    /// gate is consumed (revoked) and invocation proceeds; otherwise a fresh `appr_` handle is minted
    /// and `Required` is returned.
    pub fn gate_approval(&self, principal: &AgentPrincipal, capability: &CapabilityDefinition, diff_summary: serde_json::Value, existing_handle: Option<&str>, session: &SessionHandle, now_ms: u64) -> ApprovalGate {
        if !self.requires_approval(capability) {
            return ApprovalGate::Proceed;
        }
        if let Some(handle) = existing_handle {
            if let Ok(record) = self.handles.resolve(handle, session, now_ms) {
                if record.kind == HandleKind::Approval {
                    if let Ok(approval) = serde_json::from_value::<ApprovalRecord>(record.payload) {
                        if approval.decided == Some(true) && approval.capability_id == capability.id.to_string() {
                            self.handles.revoke(handle);
                            return ApprovalGate::Proceed;
                        }
                    }
                }
            }
        }
        let record = ApprovalRecord { capability_id: capability.id.to_string(), principal_id: principal.id.clone(), diff_summary, decided: None };
        let payload = serde_json::to_value(&record).unwrap_or(serde_json::Value::Null);
        let handle = self.handles.mint(HandleKind::Approval, session.clone(), Attachment::Capability { capability_id: capability.id.to_string() }, payload, now_ms);
        ApprovalGate::Required { approval_handle: handle }
    }

    /// ↩️ Resolves a pending `appr_` handle (human decision or `--auto-approve` shortcut) — revokes
    /// the pending record and mints a FRESH `appr_` handle already carrying the decision, returning its
    /// id (the value a subsequent `action.invoke{approvalHandle}` must supply). Minting fresh rather
    /// than mutating in place is forced by `HandleTable` exposing no update-payload primitive (P1b's
    /// public API, outside this packet's `path_scope` to extend).
    pub fn resolve_approval(&self, session: &SessionHandle, approval_handle: &str, approve: bool, now_ms: u64) -> Result<String, GatewayError> {
        let record = self.handles.resolve(approval_handle, session, now_ms)?;
        if record.kind != HandleKind::Approval {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "handle is not an approval handle"));
        }
        let mut approval: ApprovalRecord = serde_json::from_value(record.payload).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        approval.decided = Some(approve);
        self.handles.revoke(approval_handle);
        let payload = serde_json::to_value(&approval).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        Ok(self.handles.mint(HandleKind::Approval, session.clone(), Attachment::Capability { capability_id: approval.capability_id.clone() }, payload, now_ms))
    }
}
//#endregion 🔖️PolicyEngine

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use crate::catalog::{CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, ToolExposure};
    use semio_framework::manifest::{CapabilityEffects, CapabilityExecution, CapabilityPolicy};

    fn capability(id: &str, scopes: &[&str], approval: ApprovalMode, destructive: bool) -> CapabilityDefinition {
        CapabilityDefinition {
            id: CapabilityRef(id.to_string()),
            version: 1,
            owner: CapabilityOwner::Plugin { plugin_id: "cad".into(), app_id: Some("editor".into()), window_kind_id: Some("viewport".into()), mode_id: None },
            kind: CapabilityKind::Mutation,
            title: id.to_string(),
            description: String::new(),
            artifact_kind: None,
            use_when: Vec::new(),
            input_schema: serde_json::json!({"type": "object"}),
            output_schema: serde_json::json!({"type": "object"}),
            effects: CapabilityEffects { destructive, writes: vec![semio_framework::manifest::ResourceSelector::new("artifact:{self}")], ..Default::default() },
            policy: CapabilityPolicy { scopes: scopes.iter().map(|scope| kernel::CapabilityId(scope.to_string())).collect(), approval },
            execution: CapabilityExecution::default(),
            exposure: ToolExposure::CatalogOnly,
            presentation: CapabilityPresentation { icon_id: None, category: None, keys: None, in_palette: false, args: Vec::new() },
            examples: Vec::new(),
            source: CapabilitySource::Gateway,
        }
    }

    //#region 🔖️ScopeExpansion
    #[test]
    fn artifact_write_expands_to_documents_write_and_jobs_spawn() {
        let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["artifact.write".to_string()], None);
        assert!(principal.grants(&kernel::CapabilityId("documents.write".into())));
        assert!(principal.grants(&kernel::CapabilityId("jobs.spawn".into())));
        assert!(!principal.grants(&kernel::CapabilityId("shell.raw".into())));
    }

    #[test]
    fn ui_raw_control_expands_to_shell_raw() {
        let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["ui.raw-control".to_string()], None);
        assert!(principal.grants(&kernel::CapabilityId("shell.raw".into())));
    }

    #[test]
    fn an_unknown_alias_passes_through_as_a_literal_capability_id() {
        let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["documents.write".to_string()], None);
        assert!(principal.grants(&kernel::CapabilityId("documents.write".into())));
    }

    #[test]
    fn wildcard_family_grant_covers_any_concrete_member() {
        let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["host.filesystem.read".to_string()], None);
        assert!(principal.grants(&kernel::CapabilityId("fs.read:/tmp/workspace".into())));
        assert!(!principal.grants(&kernel::CapabilityId("fs.write:/tmp/workspace".into())));
    }
    //#endregion 🔖️ScopeExpansion

    //#region 🔖️ScopeEnforcement
    #[test]
    fn authorize_scopes_denies_when_a_required_scope_is_missing() {
        let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::Never);
        let principal = AgentPrincipal::from_scope_names("agent:local", "local", &[], None);
        let capability = capability("cad.editor.translateSelection", &["documents.write"], ApprovalMode::Never, false);
        let error = engine.authorize_scopes(&principal, &capability).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::PermissionDenied);
    }

    #[test]
    fn authorize_scopes_allows_when_every_scope_is_granted() {
        let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::Never);
        let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["artifact.write".to_string()], None);
        let capability = capability("cad.editor.translateSelection", &["documents.write"], ApprovalMode::Never, false);
        assert!(engine.authorize_scopes(&principal, &capability).is_ok());
    }
    //#endregion 🔖️ScopeEnforcement

    //#region 🔖️ApprovalGate
    #[test]
    fn never_approval_mode_proceeds_without_any_gate() {
        let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::Never);
        let principal = AgentPrincipal::from_scope_names("agent:local", "local", &[], None);
        let capability = capability("cad.editor.translateSelection", &[], ApprovalMode::Never, true);
        let session = SessionHandle::new("sess_1");
        assert_eq!(engine.gate_approval(&principal, &capability, serde_json::json!({}), None, &session, 0), ApprovalGate::Proceed);
    }

    #[test]
    fn a_destructive_capability_under_when_destructive_requires_approval_then_proceeds_once_resolved() {
        let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::Never);
        let principal = AgentPrincipal::from_scope_names("agent:local", "local", &[], None);
        let capability = capability("cad.editor.deleteSelection", &[], ApprovalMode::WhenDestructive, true);
        let session = SessionHandle::new("sess_1");

        let first = engine.gate_approval(&principal, &capability, serde_json::json!({"opsCount": 1}), None, &session, 0);
        let handle = match first {
            ApprovalGate::Required { approval_handle } => approval_handle,
            ApprovalGate::Proceed => panic!("a destructive WhenDestructive capability must require approval"),
        };

        // not yet decided: resubmitting the same (undecided) handle must still be Required.
        let still_pending = engine.gate_approval(&principal, &capability, serde_json::json!({}), Some(&handle), &session, 1);
        assert_ne!(still_pending, ApprovalGate::Proceed);

        let approved_handle = engine.resolve_approval(&session, &handle, true, 2).unwrap();
        let proceeds = engine.gate_approval(&principal, &capability, serde_json::json!({}), Some(&approved_handle), &session, 3);
        assert_eq!(proceeds, ApprovalGate::Proceed);
    }

    #[test]
    fn a_denied_approval_never_lets_the_gate_proceed() {
        let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::Never);
        let principal = AgentPrincipal::from_scope_names("agent:local", "local", &[], None);
        let capability = capability("cad.editor.deleteSelection", &[], ApprovalMode::Always, false);
        let session = SessionHandle::new("sess_1");
        let first = match engine.gate_approval(&principal, &capability, serde_json::json!({}), None, &session, 0) {
            ApprovalGate::Required { approval_handle } => approval_handle,
            ApprovalGate::Proceed => panic!("Always must require approval"),
        };
        let denied_handle = engine.resolve_approval(&session, &first, false, 1).unwrap();
        let gate = engine.gate_approval(&principal, &capability, serde_json::json!({}), Some(&denied_handle), &session, 2);
        assert_ne!(gate, ApprovalGate::Proceed);
    }

    #[test]
    fn auto_approve_all_waives_the_gate_entirely() {
        let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::All);
        let principal = AgentPrincipal::from_scope_names("agent:local", "local", &[], None);
        let capability = capability("cad.editor.deleteSelection", &[], ApprovalMode::Always, true);
        let session = SessionHandle::new("sess_1");
        assert_eq!(engine.gate_approval(&principal, &capability, serde_json::json!({}), None, &session, 0), ApprovalGate::Proceed);
    }

    #[test]
    fn auto_approve_parses_the_three_frozen_values_and_nothing_else() {
        assert_eq!(AutoApprovePolicy::parse("never"), Some(AutoApprovePolicy::Never));
        assert_eq!(AutoApprovePolicy::parse("readonly"), Some(AutoApprovePolicy::ReadonlyOnly));
        assert_eq!(AutoApprovePolicy::parse("all"), Some(AutoApprovePolicy::All));
        assert_eq!(AutoApprovePolicy::parse("sometimes"), None);
        assert_eq!(AutoApprovePolicy::default(), AutoApprovePolicy::Never);
    }
    //#endregion 🔖️ApprovalGate
}
//#endregion 🧪️Tests
