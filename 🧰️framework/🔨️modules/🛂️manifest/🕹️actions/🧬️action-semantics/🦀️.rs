//! 🧬️ Action semantics: what an action or command means to an agent, the classification error and the catalog classification validator. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️ActionSemantics
/// 🎯️ What an `ActionDefinition`/`CommandDefinition` MEANS to an agent: effects, policy,
/// execution shape, and natural-language framing (`use_when`/`examples`) — everything the MCP
/// catalog compiler needs beyond the UI-shaped fields already on the definition itself. Defaulted
/// per-kind by `for_kind` at construction time; `#[serde(default)]` on the owning field additionally
/// tolerates old serialized manifests with no `semantics` key at all (deserializes to
/// `ActionSemantics::default()`, the type-level default below — NOT re-derived from `kind`, since
/// serde field defaults cannot see sibling fields).
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ActionSemantics {
    #[serde(default)]
    #[value(default)]
    pub effects: CapabilityEffects,
    #[serde(default)]
    #[value(default)]
    pub policy: CapabilityPolicy,
    #[serde(default)]
    #[value(default)]
    pub execution: CapabilityExecution,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedLabel>,
    /// 👥️ Declared audience — see [`CapabilityAudience`]. `None` means "derive it", which
    /// [`resolve_audience`] does from `kind`/`in_palette`; only a verb the derivation would get wrong
    /// (a pointer gesture that commits a real operation, a palette verb that is pure chrome) declares
    /// one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<CapabilityAudience>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub use_when: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<String>,
}

impl ActionSemantics {
    /// 🏭️ The `📋️master.md` §3.1 defaults table, keyed by `ActionKind`: `Mutation` writes its
    /// own artifact, is reversible, previews a `Diff`, undoes via `Inverse`, expects a revision, and
    /// needs `artifacts.write` gated `WhenDestructive`; `View`/`Interaction` read the config lane
    /// (`artifacts.read` + `shell.observe`); `History` needs `artifacts.write`; `Clipboard` needs
    /// `shell.clipboard`; `🐚️Shell` is not reversible and needs `shell.navigate`.
    pub fn for_kind(kind: ActionKind) -> Self {
        match kind {
            ActionKind::Mutation => Self {
                effects: CapabilityEffects { writes: vec![ResourceSelector::new("artifact:{self}")], reversible: true, ..Default::default() },
                policy: CapabilityPolicy { scopes: vec![CapabilityId("artifacts.write".into())], approval: ApprovalMode::WhenDestructive },
                execution: CapabilityExecution { preview: PreviewMode::Diff, undo: UndoMode::Inverse, expected_revision: true, ..Default::default() },
                ..Default::default()
            },
            ActionKind::View | ActionKind::Interaction => Self {
                effects: CapabilityEffects { reads: vec![ResourceSelector::new("config:{self}")], ..Default::default() },
                policy: CapabilityPolicy { scopes: vec![CapabilityId("artifacts.read".into()), CapabilityId("shell.observe".into())], ..Default::default() },
                ..Default::default()
            },
            ActionKind::History => Self { policy: CapabilityPolicy { scopes: vec![CapabilityId("artifacts.write".into())], ..Default::default() }, ..Default::default() },
            ActionKind::Clipboard => Self { policy: CapabilityPolicy { scopes: vec![CapabilityId("shell.clipboard".into())], ..Default::default() }, ..Default::default() },
            ActionKind::Shell => Self { effects: CapabilityEffects { reversible: false, ..Default::default() }, policy: CapabilityPolicy { scopes: vec![CapabilityId("shell.navigate".into())], ..Default::default() }, ..Default::default() },
        }
    }
}

/// 🛡️ One release-blocking Phase-8 classification failure, addressed by owner path and
/// declaration id so generated catalogs can report every omission in one pass.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InteractiveJobClassificationError {
    pub owner: String,
    pub id: String,
}

impl std::fmt::Display for InteractiveJobClassificationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "unclassified interactive command '{}:{}'", self.owner, self.id)
    }
}

impl std::error::Error for InteractiveJobClassificationError {}

/// ✅️ Rejects an action/command catalog containing an unclassified declaration. Deleted and
/// batch-only declarations are classified data, while UI dispatch separately rejects dispositions
/// that are not [`InteractiveJobClassification::Migrated`].
pub fn validate_interactive_job_classification<'a>(actions: impl IntoIterator<Item = (&'a str, &'a ActionDefinition)>, commands: impl IntoIterator<Item = (&'a str, &'a CommandDefinition)>) -> Result<(), Vec<InteractiveJobClassificationError>> {
    let mut errors = Vec::new();
    for (owner, action) in actions {
        if action.semantics.execution.interactive_job == InteractiveJobClassification::Unclassified {
            errors.push(InteractiveJobClassificationError { owner: owner.to_string(), id: action.id.clone() });
        }
    }
    for (owner, command) in commands {
        if command.semantics.execution.interactive_job == InteractiveJobClassification::Unclassified {
            errors.push(InteractiveJobClassificationError { owner: owner.to_string(), id: command.id.clone() });
        }
    }
    errors.sort_by(|left, right| (&left.owner, &left.id).cmp(&(&right.owner, &right.id)));
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
//#endregion 🔖️ActionSemantics
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
