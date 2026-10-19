//! 🗂️ Action kind and capability vocabulary: the history classification of an action, the resource selector, the capability effects, approval, policy, preview, undo, idempotency and execution class. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️ActionKind
/// 🗂️ Classifies a declared action by how it interacts with VCS history.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ActionKind {
    /// 🧬️ Mutates the document — dispatched as VCS mutations with a true inverse, recorded in history.
    Mutation,
    /// 📷️ Ephemeral window or session state — recorded in the session command log, never as a VCS edit.
    View,
    /// 📜️ Moves the applied stack, the head of the artifact or a reserved history session — injected by the framework or
    /// a [`ReservedContribution`], never app-declared.
    History,
    /// 📋️ Copies, cuts or pastes through the shell clipboard — framework-injected, never app-declared.
    Clipboard,
    /// 🐚️ Shell-only effect without a document mutation — recorded in the session command log via dispatch or the
    /// `noteShellCommand` mechanism.
    Shell,
    /// 🤝️ Raw pointer interaction of a declared interaction domain — framework-injected, never app-declared.
    Interaction,
}

semio_framework_value::artifact_retire_leaf!(ActionKind);
//#endregion 🔖️ActionKind

//#region 🔖️ActionSemantics
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema, §3.1/D5:
// what an `ActionDefinition`/`CommandDefinition` MEANS to an agent — effects, policy, execution
// shape, and natural-language framing — kept deliberately separate from `CapabilityDefinition` (the
// gateway's compiled projection, per D5, which lives in the gateway crate, not here) and from
// `Broker`'s own `CapabilityId`/`BrokerCapabilityGrant` (the enforcement primitive
// `CapabilityPolicy.scopes` below references, never redefines).
/// 🎯️ A templated resource-selector string identifying what a capability reads/writes —
/// documented vocabulary (`"artifact:{self}"`, `"artifact:{arg.<id>}"`, `"config:{self}"`,
/// `"ui:window"`, `"clipboard"`, `"fs:{arg.<id>}"`, `"net:{origin}"`), not a closed enum: a new
/// resource family never needs a manifest schema change.
// 🌱️ Already unblocked via the hand-written `ToValue`/`FromValue` impls below — NOT because
// `#[value(...)]` lacks tuple-struct support (that claim was stale/false: the sibling
// `#[serde(transparent)]` tuple structs `ActionRef`/`UtilityRef`/`ToolRef` above prove
// `#[value(transparent)]` works fine on tuple structs); `pub String` here is just hand-written
// instead. A `#[derive(ToValue, FromValue)]` here would conflict (E0119) with the impls below.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceSelector(pub String);

/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]` conflicts (E0119) with these
/// manual impls — wire-shaped as the bare inner string, matching serde's own default
/// newtype-struct behavior.
impl ToValue for ResourceSelector {
    fn to_value(&self) -> DslValue {
        self.0.to_value()
    }
}

impl FromValue for ResourceSelector {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        String::from_value(value).map(ResourceSelector)
    }
}

impl ResourceSelector {
    pub fn new(selector: impl Into<String>) -> Self {
        Self(selector.into())
    }
}

/// 🧮️ What one capability touches — read/write resource selectors plus the three coarse flags
/// the gateway's policy/preview machinery gates on.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityEffects {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub reads: Vec<ResourceSelector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub writes: Vec<ResourceSelector>,
    #[serde(default)]
    #[value(default)]
    pub external: bool,
    #[serde(default)]
    #[value(default)]
    pub destructive: bool,
    #[serde(default)]
    #[value(default)]
    pub reversible: bool,
}

/// 🚦️ When the gateway must pause for human approval before committing an invocation of this
/// capability.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ApprovalMode {
    #[default]
    Never,
    WhenDestructive,
    Always,
}

/// 🛡️ The scope/approval gate a capability invocation must clear — `scopes` are
/// `CapabilityId`s (the Broker's own enforcement primitive, see `🔖️Kernel` below), never a
/// parallel string vocabulary: `ExtensionPointDeclaration.capability_allowance` already establishes
/// that `CapabilityId` is reachable from this crate with no dependency cycle.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityPolicy {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<CapabilityId>,
    #[serde(default)]
    #[value(default)]
    pub approval: ApprovalMode,
}

/// 👁️ Whether/how the gateway can show the effect of an invocation before committing it.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum PreviewMode {
    #[default]
    None,
    DryRun,
    Diff,
}

/// ↩️ How a committed invocation of this capability can be undone.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum UndoMode {
    #[default]
    None,
    Inverse,
    /// 🔁️ Undone by invoking a DIFFERENT capability (id, not the gateway's `CapabilityDefinition` —
    /// that type lives in the gateway crate per D5) rather than a true inverse.
    Compensate {
        capability: String,
    },
}

/// 🔁️ Whether replaying the same invocation twice is safe, and how the gateway makes it so.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IdempotencyMode {
    Natural,
    Key,
    #[default]
    None,
}

/// ⏱️ How long-running/interactive an invocation of this capability is — the gateway's job
/// vs. interactive-call dispatch hint.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ExecutionClass {
    #[default]
    Interactive,
    Background,
    Job,
}
//#endregion 🔖️ActionSemantics
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
