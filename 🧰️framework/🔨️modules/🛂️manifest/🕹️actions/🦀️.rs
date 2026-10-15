//! 🕹️ Action semantics and the action and command definitions the framework injects:
//! `ActionSemantics`, `ActionDefinition`, history filter, host event, clipboard, interaction, active utility and tool, document transfer.
//!
//! Split from the manifest root by section C of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/SEAM-MAP-PLUGIN-MANIFEST-HUB.md`.
use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️ActionSemantics
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema, §3.1/D5:
// what an `ActionDefinition`/`CommandDefinition` MEANS to an agent — effects, policy, execution
// shape, and natural-language framing — kept deliberately separate from `CapabilityDefinition` (the
// gateway's compiled projection, per D5, which lives in the gateway crate, not here) and from
// `kernel::Broker`'s own `CapabilityId`/`BrokerCapabilityGrant` (the enforcement primitive
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
/// `kernel::CapabilityId`s (the Broker's own enforcement primitive, see `🔖️Kernel` below), never a
/// parallel string vocabulary: `ExtensionPointDeclaration.capability_allowance` already establishes
/// that `kernel::CapabilityId` is reachable from this crate with no dependency cycle.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityPolicy {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<kernel::CapabilityId>,
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

/// 🎯️ WHO a declared action/command is addressed to. Orthogonal to [`ActionKind`] (which says
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
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

/// 🧭️ This action's audience — its own declaration when it has one, otherwise
/// [`derive_audience`] over `kind`/`in_palette`.
pub fn resolve_audience(action: &ActionDefinition) -> CapabilityAudience {
    action.semantics.audience.unwrap_or_else(|| derive_audience(action.kind, action.in_palette))
}

/// 🧭️ This command's audience — see [`resolve_audience`].
pub fn resolve_command_audience(command: &CommandDefinition) -> CapabilityAudience {
    command.semantics.audience.unwrap_or_else(|| derive_audience(command.kind, command.in_palette))
}

/// ⚙️ Preview/undo/idempotency/cancellation shape of one capability invocation.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
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
    /// 🎯️ Declared audience — see [`CapabilityAudience`]. `None` means "derive it", which
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
                policy: CapabilityPolicy { scopes: vec![kernel::CapabilityId("artifacts.write".into())], approval: ApprovalMode::WhenDestructive },
                execution: CapabilityExecution { preview: PreviewMode::Diff, undo: UndoMode::Inverse, expected_revision: true, ..Default::default() },
                ..Default::default()
            },
            ActionKind::View | ActionKind::Interaction => Self {
                effects: CapabilityEffects { reads: vec![ResourceSelector::new("config:{self}")], ..Default::default() },
                policy: CapabilityPolicy { scopes: vec![kernel::CapabilityId("artifacts.read".into()), kernel::CapabilityId("shell.observe".into())], ..Default::default() },
                ..Default::default()
            },
            ActionKind::History => Self { policy: CapabilityPolicy { scopes: vec![kernel::CapabilityId("artifacts.write".into())], ..Default::default() }, ..Default::default() },
            ActionKind::Clipboard => Self { policy: CapabilityPolicy { scopes: vec![kernel::CapabilityId("shell.clipboard".into())], ..Default::default() }, ..Default::default() },
            ActionKind::Shell => Self { effects: CapabilityEffects { reversible: false, ..Default::default() }, policy: CapabilityPolicy { scopes: vec![kernel::CapabilityId("shell.navigate".into())], ..Default::default() }, ..Default::default() },
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

/// 📇️ Declares one action an app can receive via `ActionDescriptor.action`.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ActionDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub kind: ActionKind,
    pub icon_id: IconName,
    /// 📝️ Typed argument declarations. Empty (the common case) = a no-argument action.
    pub args: Vec<ActionArgDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub keys: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub in_palette: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 🎯️ Effects/policy/execution/use_when — see `🔖️ActionSemantics`. Defaulted per-`kind` by
    /// `ActionSemantics::for_kind` in `new`/`new_catalog`; every struct-update-syntax call site
    /// (`ActionDefinition { .., ..Self::new_catalog(..) }`) inherits it unchanged from the base
    /// expression, so none of the ~126 declaration sites need touching.
    #[serde(default)]
    #[value(default)]
    pub semantics: ActionSemantics,
}

impl ActionDefinition {
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, kind: ActionKind, icon_id: impl Into<IconName>) -> Self {
        Self { id: id.into(), label: label.into(), kind, icon_id: icon_id.into(), args: Vec::new(), keys: None, in_palette: true, category: None, semantics: ActionSemantics::for_kind(kind) }
    }

    /// 🎯️ Declares an action with the neutral icon for its kind.
    pub fn new_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, kind: ActionKind) -> Self {
        Self::new(id, label, kind, default_action_icon_id(kind))
    }

    /// ⚡️ Builds a catalog row without granting UI execution authority. A factory registration or
    /// static bounded-first-step proof must classify the exact key separately.
    pub fn bounded_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, kind: ActionKind) -> Self {
        Self::new_catalog(id, label, kind)
    }

    /// 🧵️ Grants execution classification only to framework routes backed by the explicit
    /// route-specific resumable factories registered by `VcsArtifactApp`.
    pub fn resumable_framework(id: impl Into<String>, label: impl Into<LocalizedLabel>, kind: ActionKind, icon_id: impl Into<IconName>) -> Self {
        let mut definition = Self::new(id, label, kind, icon_id);
        definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
        definition
    }

    /// 📝️ Attaches typed argument declarations to this action.
    pub fn with_args(mut self, args: impl IntoIterator<Item = ActionArgDef>) -> Self {
        self.args = args.into_iter().collect();
        self
    }

    /// 🎨️ Sets palette visibility for this action.
    pub fn with_in_palette(mut self, in_palette: bool) -> Self {
        self.in_palette = in_palette;
        self
    }

    /// 🎨️ Sets palette visibility for this action.
    pub fn in_palette(self, in_palette: bool) -> Self {
        self.with_in_palette(in_palette)
    }

    /// 🗂️ Sets this action's ribbon-parent-taxonomy category (a `ui_wgpu::wgpu::RIBBON_PARENT_CATEGORIES`
    /// id) — read back by `AppActionRegistry::category_of` and fed into `organize_context_menu`'s
    /// `category_of` lookup at the context-menu funnel, so an overflowing flat menu buckets this
    /// action's row into `menu.group.<category>` instead of `menu.group.actions`.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// 🗂️ Sets this action's ribbon-parent-taxonomy category — see `with_category`.
    pub fn category(self, category: impl Into<String>) -> Self {
        self.with_category(category)
    }

    /// 🎯️ Replaces this action's whole `ActionSemantics` wholesale.
    pub async fn semantics(mut self, semantics: ActionSemantics) -> Self {
        self.semantics = semantics;
        self
    }

    /// ⚠️ Marks this action destructive: it discards user content no later verb reconstructs
    /// (delete, clear, replace-the-whole-document). Sets `effects.destructive` and raises
    /// `policy.approval` to `WhenDestructive` (a no-op if it was already `Always`), which is the one
    /// fact the MCP gateway's approval gate reads before committing an agent's invocation.
    pub fn destructive(mut self) -> Self {
        self.semantics.effects.destructive = true;
        if self.semantics.policy.approval == ApprovalMode::Never {
            self.semantics.policy.approval = ApprovalMode::WhenDestructive;
        }
        self
    }

    /// 🗣️ Sets the natural-language phrases a capability search should match this action
    /// against (`ActionSemantics.use_when`).
    pub fn use_when(mut self, phrases: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.semantics.use_when = phrases.into_iter().map(Into::into).collect();
        self
    }

    /// 💬️ Sets the localized one-or-two-sentence description an agent reads to decide whether
    /// this is the verb it wants (`ActionSemantics.description`) — `LocalizedLabel::native(en, de)`,
    /// English first and German second per `AGENTS.md`, never a default language.
    pub fn describe(mut self, description: impl Into<LocalizedLabel>) -> Self {
        self.semantics.description = Some(description.into());
        self
    }

    /// 🎯️ Declares this action's [`CapabilityAudience`] explicitly, overriding
    /// [`derive_audience`].
    pub fn audience(mut self, audience: CapabilityAudience) -> Self {
        self.semantics.audience = Some(audience);
        self
    }

    /// 🖱️ Marks this action a raw input event ([`CapabilityAudience::Input`]) and takes it out
    /// of the palette — the declaration a `Mutation`-kind pointer/gesture route needs, since
    /// [`derive_audience`] reads every mutation as agent-addressable.
    pub fn input_event(mut self) -> Self {
        self.in_palette = false;
        self.semantics.audience = Some(CapabilityAudience::Input);
        self
    }

    /// 🪟️ Marks this action window/view/session chrome ([`CapabilityAudience::Chrome`]) and
    /// takes it out of the palette.
    pub fn chrome(mut self) -> Self {
        self.in_palette = false;
        self.semantics.audience = Some(CapabilityAudience::Chrome);
        self
    }

    /// 📖️ Appends one natural-language usage example (`ActionSemantics.examples`).
    pub async fn example(mut self, example: impl Into<String>) -> Self {
        self.semantics.examples.push(example.into());
        self
    }
}

/// ⏪️ The framework-owned action id apps dispatch to revert to a past command-log entry —
/// auto-injected as the 7th `history_action_definitions()` entry (never in the palette; needs a
/// concrete `entrySeq` from the history panel's "backwards" button).
pub const REVERT_TO_COMMAND_ACTION_ID: &str = "revertToCommand";

/// 🕹️ The framework-owned History actions, auto-injected into every `AppDefinition`: the seven history-lane verbs
/// followed by the [`history_edit_action_definitions`] of non-destructive history editing.
pub fn history_action_definitions() -> Vec<ActionDefinition> {
    let mut actions = history_lane_action_definitions();
    actions.extend(history_edit_action_definitions());
    actions
}

/// 🌿️ Moves the artifact head onto an existing alternative: `alternativeId`.
pub const SWITCH_ALTERNATIVE_ACTION_ID: &str = "switchAlternative";
/// 🪪️ `switchAlternative`'s alternative id argument.
pub const SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID: &str = "alternativeId";

/// 🕹️ The seven history-lane verbs: they move the applied stack or the head of the artifact.
fn history_lane_action_definitions() -> Vec<ActionDefinition> {
    vec![
        ActionDefinition { keys: Some("mod+z".into()), ..ActionDefinition::resumable_framework("undo", LocalizedLabel::native("Undo", "Rückgängig"), ActionKind::History, "undo-2") }
            .describe(LocalizedLabel::native("Reverts the most recent edit on the open artifact and moves its head back one revision.", "Macht die letzte Änderung am geöffneten Artefakt rückgängig und setzt den Kopf eine Revision zurück."))
            .use_when(["undo the last change", "take that back", "revert my last edit"]),
        ActionDefinition { keys: Some("mod+shift+z".into()), ..ActionDefinition::resumable_framework("redo", LocalizedLabel::native("Redo", "Wiederholen"), ActionKind::History, "redo-2") }
            .describe(LocalizedLabel::native("Re-applies the edit that was last undone and moves the artifact head forward one revision.", "Wendet die zuletzt rückgängig gemachte Änderung erneut an und setzt den Kopf eine Revision vor."))
            .use_when(["redo", "re-apply what I undid"]),
        ActionDefinition::resumable_framework("commitCheckpoint", LocalizedLabel::native("Commit Checkpoint", "Checkpoint festschreiben"), ActionKind::History, "git-commit")
            .describe(LocalizedLabel::native("Writes a named checkpoint into the artifact's history so a later edit can be reverted back to exactly this state.", "Schreibt einen benannten Checkpoint in die Historie des Artefakts, auf den später zurückgesetzt werden kann."))
            .use_when(["save a checkpoint", "mark this state"]),
        ActionDefinition::resumable_framework("createAlternative", LocalizedLabel::native("Create Alternative", "Alternative erstellen"), ActionKind::History, "git-branch")
            .describe(LocalizedLabel::native("Branches the artifact's history into a new named alternative that can be edited without disturbing the current one. Only this viewer's head moves; every collaborator stays on theirs.", "Verzweigt die Historie des Artefakts in eine neue benannte Alternative, die bearbeitet werden kann, ohne die aktuelle zu stören. Nur der Kopf dieses Betrachters wechselt; jeder Mitarbeiter bleibt auf seinem."))
            .use_when(["try a variant", "branch this design"]),
        ActionDefinition::resumable_framework(SWITCH_ALTERNATIVE_ACTION_ID, LocalizedLabel::native("Switch Alternative", "Alternative wechseln"), ActionKind::History, "git-branch")
            .describe(LocalizedLabel::native(
                "Moves this viewer's head onto another existing alternative. Collaborators stay on theirs. This viewer sees that alternative's history, including the history edits kept as that alternative, and not those of the alternative they left.",
                "Setzt den Kopf dieses Betrachters auf eine andere vorhandene Alternative. Mitarbeiter bleiben auf ihrem. Dieser Betrachter sieht deren Verlauf einschließlich der als diese Alternative behaltenen Verlaufsbearbeitungen und nicht den der verlassenen Alternative.",
            ))
            .use_when(["switch to the other variant", "go back to the original history", "compare the edited history with the original"])
            .with_args([ActionArgDef::text(SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID, LocalizedLabel::native("Alternative", "Alternative"))
                .describe(LocalizedLabel::native("Id of an existing alternative of this artifact, as the history lists it.", "Kennung einer vorhandenen Alternative dieses Artefakts, wie der Verlauf sie auflistet."))
                .required()]),
        ActionDefinition::resumable_framework("checkoutCheckpoint", LocalizedLabel::native("Checkout Checkpoint", "Checkpoint auschecken"), ActionKind::History, "git-branch")
            .describe(LocalizedLabel::native("Restores this viewer's view of the artifact to a previously committed checkpoint by id. Collaborators stay where they are.", "Setzt die Ansicht dieses Betrachters auf einen zuvor festgeschriebenen Checkpoint zurück. Mitarbeiter bleiben, wo sie sind."))
            .use_when(["go back to that checkpoint"]),
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(REVERT_TO_COMMAND_ACTION_ID, LocalizedLabel::native("Revert to Command", "Auf Befehl zurücksetzen"), ActionKind::History, "clock") }
            .describe(LocalizedLabel::native("Rewinds the artifact to the state it had just after one numbered entry of the session command log.", "Setzt das Artefakt auf den Zustand direkt nach einem nummerierten Eintrag des Sitzungsprotokolls zurück."))
            .with_args([ActionArgDef::number("entrySeq", LocalizedLabel::native("Entry", "Eintrag")).required()]),
    ]
}

/// 🎚️ The framework-owned action id apps dispatch to change the history panel's mutations
/// filter — auto-injected unconditionally (mirrors `RECORD_TUTORIAL_ACTION_ID`).
pub const SET_HISTORY_COMMAND_FILTER_ACTION_ID: &str = "setHistoryCommandFilter";
/// 🎚️ The `setHistoryCommandFilter` option (and `HistoryPatch.commandFilter` value) that shows every row.
pub const HISTORY_COMMAND_FILTER_ALL: &str = "all";
/// 🎚️ The option that hides the rows of document mutations.
pub const HISTORY_COMMAND_FILTER_WITHOUT_MUTATIONS: &str = "withoutMutations";
/// 🎚️ The option that shows only the rows of document mutations.
pub const HISTORY_COMMAND_FILTER_ONLY_MUTATIONS: &str = "onlyMutations";

/// 🎚️ The framework-injected `setHistoryCommandFilter` View action (never in the palette):
/// switches the history panel's tri-state operations filter. Ephemeral UI state, never an
/// operation — `ActionKind::View`. Arg id is `"value"` (not `"filter"`) — a top-level `UiNode::Select`
/// always dispatches its picked option merged into `args` under the `"value"` key (both renderers'
/// `Select` interpreters hardcode that key; see `with_item_value_arg` in ui_wgpu).
pub fn set_history_command_filter_action_definition() -> ActionDefinition {
    let options = vec![
        ActionArgOption::new(HISTORY_COMMAND_FILTER_ALL, LocalizedLabel::native("All", "Alle")),
        ActionArgOption::new(HISTORY_COMMAND_FILTER_WITHOUT_MUTATIONS, LocalizedLabel::native("Without Mutations", "Ohne Mutationen")),
        ActionArgOption::new(HISTORY_COMMAND_FILTER_ONLY_MUTATIONS, LocalizedLabel::native("Only Mutations", "Nur Mutationen")),
    ];
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(SET_HISTORY_COMMAND_FILTER_ACTION_ID, LocalizedLabel::native("Set History Filter", "Verlaufsfilter festlegen"), ActionKind::View, "list") }
    .describe(LocalizedLabel::native("Filters the history panel to all entries, only document mutations, or everything except document mutations; the document is not changed.", "Filtert den Verlaufsbereich auf alle Einträge, nur Dokumentmutationen oder alles außer Dokumentmutationen; das Dokument wird nicht verändert."))
    .with_args([ActionArgDef::select(
        "value",
        LocalizedLabel::native("Filter", "Filter"),
        options,
    )
    .default_value(&HISTORY_COMMAND_FILTER_ALL)])
}

/// 🗒️ The framework-owned action id apps dispatch to note a shell effect (navigate, export,
/// spawn, …) into the session command log without any document mutation — mirrors
/// `SET_HISTORY_COMMAND_FILTER_ACTION_ID`'s auto-injected-constant pattern.
pub const NOTE_SHELL_COMMAND_ACTION_ID: &str = "noteShellCommand";

//#region 🔖️HostEvent
/// 📨️ The host-forwarded window fact `hostEvent{windowId, kind}`: the runtime answers it with the app's own typed
/// host-event command for that window (`ArtifactApp::host_event`), so an open gesture there ends without a trace.
pub const HOST_EVENT_ACTION_ID: &str = "hostEvent";
/// 🪟️ `hostEvent`'s window instance argument.
pub const HOST_EVENT_ARG_WINDOW_ID: &str = "windowId";
/// 🏷️ `hostEvent`'s event kind argument: [`HOST_EVENT_KIND_BLUR`], [`HOST_EVENT_KIND_CAPTURE_LOST`] or [`HOST_EVENT_KIND_RETIRING`].
pub const HOST_EVENT_ARG_KIND: &str = "kind";
/// 🌫️ The window lost keyboard focus.
pub const HOST_EVENT_KIND_BLUR: &str = "blur";
/// 🖐️ The window lost the pointer capture of a gesture (the release never reached it).
pub const HOST_EVENT_KIND_CAPTURE_LOST: &str = "captureLost";
/// 🚪️ The window is about to close.
pub const HOST_EVENT_KIND_RETIRING: &str = "retiring";

/// 📨️ The framework-injected `hostEvent` input verb (never in the palette, never an agent verb, never a history row):
/// both hosts forward a window's blur and pointer-capture loss through it, and may announce its closing.
pub fn host_event_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(HOST_EVENT_ACTION_ID, LocalizedLabel::native("Window Event", "Fensterereignis"), ActionKind::Interaction, "hand") }
        .describe(LocalizedLabel::native(
            "Tells the program that a window lost focus, lost the pointer capture of a gesture or is about to close, so an open gesture there ends without a trace.",
            "Teilt dem Programm mit, dass ein Fenster den Fokus oder die Zeigererfassung einer Geste verloren hat oder gleich geschlossen wird, damit eine offene Geste dort spurlos endet.",
        ))
        .with_args([
            ActionArgDef::text(HOST_EVENT_ARG_WINDOW_ID, LocalizedLabel::native("Window", "Fenster")).required(),
            ActionArgDef::select(
                HOST_EVENT_ARG_KIND,
                LocalizedLabel::native("Event", "Ereignis"),
                vec![
                    ActionArgOption::new(HOST_EVENT_KIND_BLUR, LocalizedLabel::native("Window lost focus", "Fenster hat den Fokus verloren")),
                    ActionArgOption::new(HOST_EVENT_KIND_CAPTURE_LOST, LocalizedLabel::native("Pointer capture lost", "Zeigererfassung verloren")),
                    ActionArgOption::new(HOST_EVENT_KIND_RETIRING, LocalizedLabel::native("Window closing", "Fenster wird geschlossen")),
                ],
            )
            .required(),
        ])
}
//#endregion 🔖️HostEvent

/// 🗒️ The framework-injected `noteShellCommand` Shell action (never in the palette): records a
/// shell-kind effect that already happened into the session command log, for effects dispatched
/// outside the normal `ActionDescriptor` path. `commandId` and `label` are required; `detail` is an
/// optional free-text elaboration shown in the history panel. A note is an UNDO TARGET only when the
/// shell also declares `inverseCommandId` (plus optional `inverseArgs`): chrome whose prior state the
/// shell never captured has no computable inverse, so it stays a log row the document undo ledger
/// steps straight over — see `VcsArtifactApp::dispatch_chrome_history_action`.
pub fn note_shell_command_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(NOTE_SHELL_COMMAND_ACTION_ID, LocalizedLabel::native("Note Shell Command", "Shell-Befehl vermerken"), ActionKind::Shell, "book-open") }
        .chrome()
        .describe(LocalizedLabel::native("Records a shell effect that already happened (navigation, export, spawn) in the session command log; the document is not changed.", "Vermerkt eine bereits erfolgte Shell-Wirkung (Navigation, Export, Start) im Sitzungsprotokoll; das Dokument wird nicht verändert."))
        .with_args([
        ActionArgDef::text("commandId", LocalizedLabel::native("Command", "Befehl")).required(),
        ActionArgDef::any("label", LocalizedLabel::native("Label", "Bezeichnung"))
            .describe(LocalizedLabel::native(
                "The row's label in every locale: an object {en, de} (or a full localized label); plain text is kept as locale-invariant data, such as a file name.",
                "Die Bezeichnung der Zeile in jeder Sprache: ein Objekt {en, de} (oder eine vollständige lokalisierte Bezeichnung); reiner Text bleibt sprachunabhängige Angabe, etwa ein Dateiname.",
            ))
            .required(),
        ActionArgDef::text("detail", LocalizedLabel::native("Detail", "Detail")),
    ])
}

//#region 🔖️Clipboard
/// 🕹️ The three framework-owned Clipboard actions, auto-injected into every `AppDefinition` —
/// mirrors `history_action_definitions`. `paste` carries a staged `anchoring` choice (defaulting to
/// `original`) plus an optional `position` override, both consumed as a `PastePlacement`.
pub fn clipboard_action_definitions() -> Vec<ActionDefinition> {
    let anchoring_options = vec![
        ActionArgOption::new("original", LocalizedLabel::native("Original", "Original")),
        ActionArgOption::new("middle", LocalizedLabel::native("Middle", "Mitte")),
        ActionArgOption::new("centroid", LocalizedLabel::native("Centroid", "Schwerpunkt")),
        ActionArgOption::new("bottomLeft", LocalizedLabel::native("Bottom Left", "Unten links")),
        ActionArgOption::new("bottomRight", LocalizedLabel::native("Bottom Right", "Unten rechts")),
        ActionArgOption::new("topLeft", LocalizedLabel::native("Top Left", "Oben links")),
        ActionArgOption::new("topRight", LocalizedLabel::native("Top Right", "Oben rechts")),
    ];
    vec![
        ActionDefinition { keys: Some("mod+c".into()), ..ActionDefinition::resumable_framework("copy", LocalizedLabel::native("Copy", "Kopieren"), ActionKind::Clipboard, "copy") }
            .describe(LocalizedLabel::native("Copies the current selection of the focused window onto the workspace clipboard.", "Kopiert die aktuelle Auswahl des fokussierten Fensters in die Zwischenablage.")),
        ActionDefinition { keys: Some("mod+x".into()), ..ActionDefinition::resumable_framework("cut", LocalizedLabel::native("Cut", "Ausschneiden"), ActionKind::Clipboard, "scissors") }
            .describe(LocalizedLabel::native("Removes the current selection from the artifact and puts it on the workspace clipboard.", "Entfernt die aktuelle Auswahl aus dem Artefakt und legt sie in der Zwischenablage ab.")),
        ActionDefinition { keys: Some("mod+v".into()), ..ActionDefinition::resumable_framework("paste", LocalizedLabel::native("Paste", "Einfügen"), ActionKind::Clipboard, "clipboard") }
            .describe(LocalizedLabel::native("Inserts the workspace clipboard's contents into the artifact at the chosen anchoring.", "Fügt den Inhalt der Zwischenablage an der gewählten Verankerung in das Artefakt ein."))
            .with_args([
            ActionArgDef::select("anchor", LocalizedLabel::native("Anchoring", "Verankerung"), anchoring_options).default_value(&"original"),
            ActionArgDef::vector("position", LocalizedLabel::native("Position", "Position"), 3),
        ]),
    ]
}
//#endregion 🔖️Clipboard

//#region 🔖️Interaction
/// 🕹️ The framework-owned action id a renderer dispatches to change a domain's selection (pick,
/// marquee gather, keyboard range/toggle) — never in the palette: renderers translate raw
/// pointer/keyboard input into this, the user never picks it from a menu.
pub const INTERACTION_SELECT_ACTION_ID: &str = "interactionSelect";

/// 🐁️ The framework-owned action id a renderer dispatches to change a domain's hover — never in the
/// palette (mirrors `INTERACTION_SELECT_ACTION_ID`).
pub const INTERACTION_HOVER_ACTION_ID: &str = "interactionHover";

/// 🧹️ The framework-owned action id apps dispatch to clear every declared domain's selection.
pub const CLEAR_SELECTION_ACTION_ID: &str = "clearSelection";

/// 🗂️ The framework-owned action id apps dispatch to select every target of the active domain at its
/// active granularity.
pub const SELECT_ALL_ACTION_ID: &str = "selectAll";

/// 🔀️ The framework-owned action id apps dispatch to switch a domain's active `SelectionMode`.
pub const SET_SELECTION_MODE_ACTION_ID: &str = "setSelectionMode";

/// 🪜️ The framework-owned action id apps dispatch to switch a domain's active granularity.
pub const SET_INTERACTION_GRANULARITY_ACTION_ID: &str = "setInteractionGranularity";

/// 🎮️ The six framework-owned interaction verbs as one closed type — the key an app's declared
/// interaction refresh scope is resolved by (`ArtifactApp::interaction_scope`), so "what does a hover
/// repaint" is a match on a verb rather than a string compare re-derived at every call site. The
/// framework owns the verbs; only the SCOPE each one dirties is app knowledge, because only the app
/// knows which of its window/panel bodies render a selection or a hover.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InteractionVerb {
    /// 🖱️ `interactionSelect` — a pick, marquee gather or keyboard range/toggle.
    Select,
    /// 🐁️ `interactionHover` — pointer motion, the highest-frequency verb in the shell.
    Hover,
    /// 🧹️ `clearSelection` — every declared domain at once.
    ClearSelection,
    /// 🗂️ `selectAll` — every target of every declared domain at its active granularity.
    SelectAll,
    /// 🔀️ `setSelectionMode` — one domain's active `SelectionMode`.
    SetSelectionMode,
    /// 🪜️ `setInteractionGranularity` — one domain's active granularity.
    SetGranularity,
}

impl InteractionVerb {
    /// 🎮️ The verb one dispatched action id stands for; `None` for every action that is not one of the
    /// six (the same membership test `INTERACTION_ACTION_IDS` makes, as a typed answer).
    pub fn of_action(action: &str) -> Option<Self> {
        match action {
            INTERACTION_SELECT_ACTION_ID => Some(Self::Select),
            INTERACTION_HOVER_ACTION_ID => Some(Self::Hover),
            CLEAR_SELECTION_ACTION_ID => Some(Self::ClearSelection),
            SELECT_ALL_ACTION_ID => Some(Self::SelectAll),
            SET_SELECTION_MODE_ACTION_ID => Some(Self::SetSelectionMode),
            SET_INTERACTION_GRANULARITY_ACTION_ID => Some(Self::SetGranularity),
            _ => None,
        }
    }

    /// 🎮️ The declared action id this verb is dispatched under — exact inverse of `of_action`.
    pub fn action_id(self) -> &'static str {
        match self {
            Self::Select => INTERACTION_SELECT_ACTION_ID,
            Self::Hover => INTERACTION_HOVER_ACTION_ID,
            Self::ClearSelection => CLEAR_SELECTION_ACTION_ID,
            Self::SelectAll => SELECT_ALL_ACTION_ID,
            Self::SetSelectionMode => SET_SELECTION_MODE_ACTION_ID,
            Self::SetGranularity => SET_INTERACTION_GRANULARITY_ACTION_ID,
        }
    }

    /// 🎮️ Every verb, in dispatch-id declaration order — what a law walks so a seventh verb cannot be
    /// added without an author deciding what it dirties.
    pub const ALL: [Self; 6] = [Self::Select, Self::Hover, Self::ClearSelection, Self::SelectAll, Self::SetSelectionMode, Self::SetGranularity];
}

//#region 🔖️InteractionRefreshScope
/// 🐢️ The refresh scope one reserved interaction verb owes, DERIVED from the app's own surface
/// declarations instead of hand-written once per app.
///
/// 🪟️ `window_bodies` is the body of every window kind that declares one of the domains the verb
/// touched ([`WindowKindDefinition::interactions`] — the same list that lets a renderer dispatch the
/// verb from that pane at all), so a window that never paints the domain is never asked to re-render
/// for it.
///
/// 🧯️ Why the panels are NOT narrowed the same way: a [`PanelTabDefinition`] declares no interaction
/// refs, so nothing in the manifest says which panel paints a selection — every declared panel body
/// therefore stays dirty for the three verbs that MOVE a selection, which is the widest honest
/// answer and can never under-refresh an inspector. The one lane narrowed without a declaration is
/// [`InteractionVerb::Hover`]: hover is the pointer-transient lane, owed to the surfaces the pointer
/// is over, and a panel row that chased the pointer would flicker rather than inform.
///
/// 🎛️ `measures` is the window's own Select chrome (active mode / active granularity, bound by the
/// measures rail): every verb but `Hover` can move persisted interaction state a rail control binds,
/// and `Hover` can move none. Utilities, tools, engagements, labels and the app-static catalogue are
/// moved by no interaction verb.
pub fn interaction_verb_surface_scope(verb: InteractionVerb, window_bodies: &[String], panel_bodies: &[String]) -> kernel::UiDirtyScope {
    let selection_moved = matches!(verb, InteractionVerb::Select | InteractionVerb::ClearSelection | InteractionVerb::SelectAll);
    kernel::UiDirtyScope::Partial {
        window_bodies: window_bodies.to_vec(),
        panel_bodies: if selection_moved { panel_bodies.to_vec() } else { Vec::new() },
        utilities: false,
        tools: false,
        engagements: false,
        measures: verb != InteractionVerb::Hover,
        labels: false,
    }
}

/// 🕹️ Every declared window body key indexed by the interaction domain that window kind declares —
/// the schema-first half of [`interaction_verb_surface_scope`], read once when an app's action
/// registry is built rather than per dispatched hover.
pub fn interaction_window_bodies_by_domain(app: &AppDefinition) -> BTreeMap<String, Vec<String>> {
    let mut index: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for window in app.window_kinds.iter() {
        for interaction in &window.interactions {
            let bodies = index.entry(interaction.as_str().to_string()).or_default();
            if !bodies.iter().any(|body| body == &window.body_key) {
                bodies.push(window.body_key.clone());
            }
        }
    }
    index
}

/// 🌳️ Every declared panel LEAF body key, in declaration order, branches walked — the panel half of
/// [`interaction_verb_surface_scope`].
pub fn panel_leaf_body_keys(app: &AppDefinition) -> Vec<String> {
    fn walk(tab: &PanelTabDefinition, out: &mut Vec<String>) {
        if let Some(body_key) = &tab.body_key {
            if !out.iter().any(|entry| entry == body_key) {
                out.push(body_key.clone());
            }
        }
        for child in &tab.children {
            walk(child, out);
        }
    }
    let mut out = Vec::new();
    for tab in &app.panel_tabs {
        walk(tab, &mut out);
    }
    out
}

/// 🐢️ The framework's own answer to "what does this verb repaint", for every app that implements no
/// `ArtifactApp::interaction_scope` of its own. `None` — and with it the framework's widest
/// [`kernel::UiDirtyScope::Full`] — is the answer whenever NO declared window paints a touched
/// domain, because then there is no declaration to derive from and narrowing would be a guess.
pub fn interaction_declared_refresh_scope(verb: InteractionVerb, domains: &[&str], windows_by_domain: &BTreeMap<String, Vec<String>>, panel_bodies: &[String]) -> Option<kernel::UiDirtyScope> {
    let mut window_bodies: Vec<String> = Vec::new();
    for domain in domains {
        for body in windows_by_domain.get(*domain).into_iter().flatten() {
            if !window_bodies.iter().any(|entry| entry == body) {
                window_bodies.push(body.clone());
            }
        }
    }
    (!window_bodies.is_empty()).then(|| interaction_verb_surface_scope(verb, &window_bodies, panel_bodies))
}
//#endregion 🔖️InteractionRefreshScope

/// 🕹️ The six framework-owned Interaction actions, auto-injected into any `AppDefinition` that
/// declares at least one `InteractionDefinition` — mirrors `history_action_definitions`/
/// `clipboard_action_definitions`, except conditional (like `set_active_utility_action_definition`)
/// rather than unconditional: returns `[]` when `app.interactions` is empty. `interactionSelect`/
/// `interactionHover` are the raw dispatch verbs renderers translate clicks/marquee/hover into
/// (never in the palette); `clearSelection`/`selectAll`/`setSelectionMode`/`setInteractionGranularity`
/// are user-facing and drive the per-domain Select controls.
pub fn interaction_action_definitions(app: &AppDefinition) -> Vec<ActionDefinition> {
    if app.interactions.is_empty() {
        return Vec::new();
    }
    let merge_options = vec![
        ActionArgOption::new("replace", LocalizedLabel::native("Replace", "Ersetzen")),
        ActionArgOption::new("additive", LocalizedLabel::native("Additive", "Additiv")),
        ActionArgOption::new("subtractive", LocalizedLabel::native("Subtractive", "Subtraktiv")),
        ActionArgOption::new("invertive", LocalizedLabel::native("Invertive", "Invertierend")),
        ActionArgOption::new("range", LocalizedLabel::native("Range", "Bereich")),
    ];
    let method_options =
        vec![ActionArgOption::new("pick", LocalizedLabel::native("Pick", "Auswahl")), ActionArgOption::new("rectangle", LocalizedLabel::native("Rectangle", "Rechteck")), ActionArgOption::new("lasso", LocalizedLabel::native("Lasso", "Lasso"))];
    let mode_options = vec![ActionArgOption::new("single", LocalizedLabel::native("Single", "Einzeln")), ActionArgOption::new("multiple", LocalizedLabel::native("Multiple", "Mehrfach"))];
    vec![
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(INTERACTION_SELECT_ACTION_ID, LocalizedLabel::native("Select", "Auswählen"), ActionKind::Interaction, "mouse-pointer") }.with_args([
            ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(),
            ActionArgDef::text("targets", LocalizedLabel::native("Targets", "Ziele")).required(),
            ActionArgDef::select("merge", LocalizedLabel::native("Merge", "Zusammenführen"), merge_options).required(),
            ActionArgDef::select("method", LocalizedLabel::native("Method", "Methode"), method_options).required(),
        ]),
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(INTERACTION_HOVER_ACTION_ID, LocalizedLabel::native("Hover", "Darüberfahren"), ActionKind::Interaction, "eye") }.with_args([
            ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(),
            ActionArgDef::text("channel", LocalizedLabel::native("Channel", "Kanal")).required(),
            ActionArgDef::text("targets", LocalizedLabel::native("Targets", "Ziele")).required(),
        ]),
        ActionDefinition { keys: Some("escape".into()), ..ActionDefinition::resumable_framework(CLEAR_SELECTION_ACTION_ID, LocalizedLabel::native("Clear Selection", "Auswahl aufheben"), ActionKind::Interaction, "mouse-pointer-2") },
        ActionDefinition { keys: Some("mod+a".into()), ..ActionDefinition::resumable_framework(SELECT_ALL_ACTION_ID, LocalizedLabel::native("Select All", "Alles auswählen"), ActionKind::Interaction, "maximize-2") },
        ActionDefinition::resumable_framework(SET_SELECTION_MODE_ACTION_ID, LocalizedLabel::native("Set Selection Mode", "Auswahlmodus festlegen"), ActionKind::Interaction, "sliders-horizontal")
            .with_args([ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(), ActionArgDef::select("mode", LocalizedLabel::native("Mode", "Modus"), mode_options).required()]),
        ActionDefinition::resumable_framework(SET_INTERACTION_GRANULARITY_ACTION_ID, LocalizedLabel::native("Set Granularity", "Granularität festlegen"), ActionKind::Interaction, "layers")
            .with_args([ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(), ActionArgDef::text("granularityId", LocalizedLabel::native("Granularity", "Granularität")).required()]),
    ]
}
//#endregion 🔖️Interaction

/// 🧰️ The framework-owned action id apps dispatch to activate a utility — auto-injected as a View
/// action into any `AppDefinition` that declares utilities (mirrors `history_action_definitions`).
pub const SET_ACTIVE_UTILITY_ACTION_ID: &str = "setActiveUtility";

/// 🧰️ The framework-injected `setActiveUtility` View action (never in the palette): switches the
/// host-owned active utility of a window kind. `utilityId` is required; `windowKindId` is contextual (the
/// shell fills it from the focused window when absent).
pub fn set_active_utility_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(SET_ACTIVE_UTILITY_ACTION_ID, LocalizedLabel::native("Set Active Utility", "Aktives Hilfsmittel festlegen"), ActionKind::View, "wrench") }
        .with_args([ActionArgDef::text("utilityId", LocalizedLabel::native("Utility", "Hilfsmittel")).required(), ActionArgDef::text("windowKindId", LocalizedLabel::native("Window", "Fenster"))])
}

/// 🛠️ The framework-owned action id apps dispatch to activate a mode-level tool — auto-injected
/// as a View action into any `AppDefinition` that declares tools (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).
pub const SET_ACTIVE_TOOL_ACTION_ID: &str = "setActiveTool";

/// 🛠️ The framework-injected `setActiveTool` View action (never in the palette): switches the
/// host-owned active tool of the active mode. Unlike `setActiveUtility` this takes no `windowKindId` —
/// tools are windowless, scoped to the whole mode.
pub fn set_active_tool_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(SET_ACTIVE_TOOL_ACTION_ID, LocalizedLabel::native("Set Active Tool", "Aktives Werkzeug festlegen"), ActionKind::View, "hammer") }.with_args([ActionArgDef::text(
        "toolId",
        LocalizedLabel::native("Tool", "Werkzeug"),
    )
    .required()])
}

/// 📤️ The framework-owned action id of Export Document: injected into every app window and fully
/// shell-intercepted (never forwarded to the program) — the shell writes the document's canonical archive (root
/// envelope, its op log, every owned member) as one `.semio-archive` file, for every artifact kind alike.
pub const EXPORT_ARTIFACT_DOCUMENT_ACTION_ID: &str = "exportArtifactDocument";

/// 📥️ The framework-owned action id of Import Document, injected beside [`EXPORT_ARTIFACT_DOCUMENT_ACTION_ID`]
/// and shell-intercepted: the shell opens a picked archive as a NEW document of the same program (a cancellable,
/// progress-reporting load of its op log), never overwriting the focused one.
pub const IMPORT_ARTIFACT_DOCUMENT_ACTION_ID: &str = "importArtifactDocument";

/// 💾️ The shell-intercepted action that writes the focused editor's declared natural file bytes.
pub const SAVE_ARTIFACT_FILE_ACTION_ID: &str = "saveArtifactFile";

/// 📂️ The shell-intercepted action that opens one declared natural file as a new document owner.
pub const OPEN_ARTIFACT_FILE_ACTION_ID: &str = "openArtifactFile";

/// 📄️ The ABI media port reserved for an editor's declared natural-file representation.
pub const NATURAL_FILE_PORT_ID: &str = "artifact:native";

/// 🗃️ The framework-injected Export/Import Document pair: `Shell` verbs in the palette and the `transfer` ribbon
/// category, shell chrome ([`CapabilityAudience::Chrome`]) — agents export through the MCP's own artifact export.
pub fn document_transfer_action_definitions() -> [ActionDefinition; 2] {
    [
        ActionDefinition::resumable_framework(EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, LocalizedLabel::native("Export Document", "Dokument exportieren"), ActionKind::Shell, "export").with_category("transfer").audience(CapabilityAudience::Chrome),
        ActionDefinition::resumable_framework(IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, LocalizedLabel::native("Import Document…", "Dokument importieren…"), ActionKind::Shell, "import").with_category("transfer").audience(CapabilityAudience::Chrome),
    ]
}
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
