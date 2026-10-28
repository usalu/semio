//! 🎯️ Audience and execution of a capability: the capability audience with its derivation, the interactive job classification and the capability execution shape. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️ActionSemantics
/// 🎬️ WHO a declared action/command is addressed to. Orthogonal to [`ActionKind`] (which says
/// how a verb relates to VCS history) and to `in_palette` (which says whether a human sees it in the
/// command palette): an agent driving the artifact through the semio MCP needs to tell an
/// intent-level document verb (`addLayer`, `patchLayer`, `exportDocument`) apart from the raw
/// pointer/keyboard/engagement events a live surface feeds the same app (`canvasPointerMove`,
/// `engagementInput`, `canvasEscape`) and from window/view chrome that only means something inside a
/// running shell (`setCamera`, `setActiveUtility`). Publishing all three as one flat capability list
/// is what made `capabilities_search "draw rectangle"` unusable (ticket 26/09/18 slice M5a).
///
/// Left undeclared, [`resolve_audience`] derives it from facts the manifest already carries; a
/// declaration overrides the derivation and is the only way to mark a `Mutation`-kind gesture route
/// (a pointer handler that really does commit an operation) as [`CapabilityAudience::Input`].
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum CapabilityAudience {
    /// 🤖️ An intent-level verb: a human or an agent can pick it to accomplish a goal on the artifact.
    Agent,
    /// 🖱️ A raw pointer/keyboard/engagement input event — meaningful only to a live UI surface that
    /// owns a cursor, a gesture and a draft. Reachable to the shell, never published to agents.
    Input,
    /// 🪟️ Window/view/session chrome — camera poses, active utility, panel filters. Reachable to the
    /// shell, never published to agents.
    Chrome,
}

/// 🧭️ The derivation [`resolve_audience`] applies when an action/command declares no
/// [`CapabilityAudience`], stated once so the catalog compiler, the descriptor emitter and the shell
/// all agree: framework-injected hover/selection is `Input`; ephemeral view state that is not a
/// palette command is `Chrome`; everything else — every mutation, every history/clipboard/shell verb,
/// and every palette-visible view verb such as `exportDocument` — is `Agent`.
pub fn derive_audience(kind: ActionKind, in_palette: bool) -> CapabilityAudience {
    match kind {
        ActionKind::Interaction => CapabilityAudience::Input,
        ActionKind::View if !in_palette => CapabilityAudience::Chrome,
        _ => CapabilityAudience::Agent,
    }
}

/// 🔎️ This action's audience — its own declaration when it has one, otherwise
/// [`derive_audience`] over `kind`/`in_palette`.
pub fn resolve_audience(action: &ActionDefinition) -> CapabilityAudience {
    action.semantics.audience.unwrap_or_else(|| derive_audience(action.kind, action.in_palette))
}

/// 👥️ This command's audience — see [`resolve_audience`].
pub fn resolve_command_audience(command: &CommandDefinition) -> CapabilityAudience {
    command.semantics.audience.unwrap_or_else(|| derive_audience(command.kind, command.in_palette))
}

/// 🧵️ How a declared action or command may be dispatched as an interactive job. The default is deliberately not
/// executable: a declaration decoded without an explicit disposition stays visible to audit tooling but is rejected by
/// [`validate_interactive_job_classification`] before a release catalog can be activated.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum InteractiveJobClassification {
    #[default]
    Unclassified,
    Migrated,
    BatchOnlyPendingRewrite,
    ForbiddenFromUi,
    Deleted,
}

/// ⚙️ Preview/undo/idempotency/cancellation shape of one capability invocation.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityExecution {
    #[serde(default)]
    #[value(default)]
    pub preview: PreviewMode,
    #[serde(default)]
    #[value(default)]
    pub undo: UndoMode,
    #[serde(default)]
    #[value(default)]
    pub idempotency: IdempotencyMode,
    #[serde(default)]
    #[value(default)]
    pub expected_revision: bool,
    #[serde(default)]
    #[value(default)]
    pub cancellable: bool,
    #[serde(default)]
    #[value(default)]
    pub class: ExecutionClass,
    #[serde(default)]
    #[value(default)]
    pub interactive_job: InteractiveJobClassification,
}
//#endregion 🔖️ActionSemantics
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
