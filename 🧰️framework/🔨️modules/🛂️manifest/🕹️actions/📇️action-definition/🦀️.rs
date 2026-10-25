//! 📇️ Action definition: the declared action with its builders and the revert and switch alternative action ids. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
/// 🎬️ Declares one action an app can receive via `ActionDescriptor.action`.
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

    /// 📚️ Declares an action with the neutral icon for its kind.
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

    /// 📎️ Attaches typed argument declarations to this action.
    pub fn with_args(mut self, args: impl IntoIterator<Item = ActionArgDef>) -> Self {
        self.args = args.into_iter().collect();
        self
    }

    /// 🎨️ Sets palette visibility for this action.
    pub fn with_in_palette(mut self, in_palette: bool) -> Self {
        self.in_palette = in_palette;
        self
    }

    /// 👁️ Sets palette visibility for this action.
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

    /// 🎀️ Sets this action's ribbon-parent-taxonomy category — see `with_category`.
    pub fn category(self, category: impl Into<String>) -> Self {
        self.with_category(category)
    }

    /// 🧬️ Replaces this action's whole `ActionSemantics` wholesale.
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

    /// 🔎️ Sets the natural-language phrases a capability search should match this action
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

    /// 👥️ Declares this action's [`CapabilityAudience`] explicitly, overriding
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

/// 🌿️ Moves the artifact head onto an existing alternative: `alternativeId`.
pub const SWITCH_ALTERNATIVE_ACTION_ID: &str = "switchAlternative";

/// 🪪️ `switchAlternative`'s alternative id argument.
pub const SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID: &str = "alternativeId";
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
