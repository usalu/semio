// #region 🛂️Manifest
//! 🧩️ App manifest (`AppDefinition`/`ModeDefinition`/`WindowKindDefinition`/`PluginManifest`/`ViewModel`)
//! and kernel types shared by plugins and renderers; the declarative `UiNode` component model itself
//! lives in `ui_wgpu`'s `component` region.

// 🧭️ `dsl` already re-exports BOTH the `ToValue`/`FromValue` traits and their derive macros
// (`💻️os/📦️packages/🦀️rust/🦀️.rs:347`), so importing them again from `semio_framework_value_derive`
// is a same-namespace redefinition (E0252), not a second namespace.
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
// 🚧️ Still needed unconditionally: most `#[cfg(test)] mod …` blocks in this file oracle-test a type
// through real `serde_json`, AND a handful of production types (the `MediaVocabulary` family) stay
// `#[derive(Serialize, Deserialize)]` — see their own `🚧️ BLOCKED` docstrings — because a sibling
// `#[path]`-mounted module (`🎠️kernel`, `🛍️products/💻️os/🔨️modules/🔁️workflow`, both owned by
// other agents this pass) still embeds them by value inside plain serde-deriving types. `ViewModel`
// itself is now dual-derived (`ToValue`/`FromValue` added alongside serde, not replacing it) — kept
// additive for the same reason: `🎠️kernel`'s `ActionContext`/`CommandContext` still embed it inside
// serde-only structs. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ui_wgpu::wgpu::ActionDescriptor;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use ui_wgpu::wgpu::NamedLayout;
use ui_wgpu::wgpu::SurfaceKind;
use semio_framework_ui_locale::Terminology;
use ui_wgpu::wgpu::WindowLayout;
use ui_wgpu::wgpu::WindowOptions;
// 🔀️ ArtifactKindSpec/OsMediaCapability/MediaType/MediaClass/MediaForm/MediaWireFormat/MediaPortSpec/
// PortMultiplicity/MediaCompat/AppIo/ArtifactPresentation/ConfigSpec/CommandGrammar/Media/MediaPayload/
// MediaConverter now live locally (see 🔖️MediaVocabulary below) — relocated from 🔺️mesh, ticket
// 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT wave 4a. The legacy format enum itself was retired in
// ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6.
use crate::IconName;
// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W1: the wave-0 interaction
// definition family, re-exported at the crate root (see
// 🧰️framework/📦️packages/🦀️rust/🦀️.rs `pub use interaction::*;`) — referenced here exactly
// like `IconName` above, so `AppDefinition.interactions`/`WindowKindDefinition.interactions` see
// them the same way manifest consumers already see `ActionDefinition`/`ActionRef`.
use crate::{DomainSelection, InteractionDefinition, InteractionRef};
// 🎯️ ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET C1: `AppDefinition.dialect` is the
// owned wire form of a dialect coordinate (`ArtifactDialect`, not the compile-time `&'static str`
// `Dialect`) — see 🔖️Surface below.
use crate::ArtifactDialect;

//#region 🔖️Children
#[path = "🧮️arg-schema/🦀️.rs"]
mod arg_schema;
pub use arg_schema::*;

#[path = "🕹️actions/🦀️.rs"]
mod actions;
pub use actions::*;

#[path = "🧳️product-vocabulary/🦀️.rs"]
mod product_vocabulary;
pub use product_vocabulary::*;

#[path = "🧱️app-surface/🦀️.rs"]
mod app_surface;
pub use app_surface::*;

#[path = "🧾️plugin-manifest/🦀️.rs"]
mod plugin_manifest;
pub use plugin_manifest::*;

#[path = "📇️space-index/🦀️.rs"]
mod space_index;
pub use space_index::*;

#[path = "🖼️view-model/🦀️.rs"]
mod view_model;
pub use view_model::*;
//#endregion 🔖️Children

//#region 🔖️Manifest
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Keybinding {
    pub keys: String,
    pub action: ActionDescriptor,
}

/// ⌨️ Operating system selector for a platform-specific keybinding.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum Platform {
    MacOs,
    Windows,
    Linux,
}

/// ⌨️ One command chord, optionally restricted to a host platform.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct PlatformKeybinding {
    pub chord: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
}

impl PlatformKeybinding {
    pub fn new(chord: impl Into<String>) -> Self {
        Self { chord: chord.into(), platform: None }
    }

    pub fn for_platform(chord: impl Into<String>, platform: Platform) -> Self {
        Self { chord: chord.into(), platform: Some(platform) }
    }
}

/// 📇️ A relative action id used by declarations nested beneath an owning window kind.
/// Distinct from `ActionAddress`, which qualifies a dispatched invocation down to a window instance.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct ActionRef(String);

// 🚫️async: E1 — `new` is a pure single-field wrapper, zero suspension points, same rationale
// already applied to `as_str` below; reverted per R9 (sync closure / catch_unwind consumers).
impl ActionRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    // 🚫️async: E1 transitive — the only consumer is inside an `Iterator::find` (external trait)
    // closure, which must be sync; pure field access, no I/O (R9).
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ActionRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ActionRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// 📍️ Fully qualified address of an action owned by one concrete window instance.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ActionAddress {
    pub plugin_id: String,
    pub app_id: String,
    pub mode_id: String,
    pub window_kind_id: String,
    pub window_instance_id: String,
    pub action_id: String,
}

/// 📨️ One addressed action invocation with named DSL-value arguments. Ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: `arguments` used to be
/// keyed to `serde_json::Value` (not a `ToValue`/`FromValue` target by design); `DslValue` carries
/// the exact same schema-less-JSON shape without the serde dependency.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ActionInvocation {
    pub address: ActionAddress,
    pub arguments: BTreeMap<String, DslValue>,
}

//#region 🔖️Utilities
/// 🧰️ Declares one interactive utility (a live-preview pointer mode) an app exposes. Distinct from
/// an `ActionDefinition`: exactly one utility is active per window kind at a time, and activation is
/// host-owned session view state (`ViewModel.active_utility_id`), never a document field or VCS operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct UtilityDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    /// 🧺️ Visual ribbon collection this utility groups into; `None` = a flat top-level ribbon entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub keys: Option<String>,
    /// 🖱️ CSS/winit cursor name applied to the window body while this utility is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub category: Option<ui_wgpu::wgpu::UtilityCategory>,
    /// 🚦️ Whether window-scoped actions stay enabled while this utility is active. Defaults to `false`
    /// (matching today's whitelist-based gating where an active utility suppresses the action panel);
    /// set `true` for passive view utilities (e.g. cad `cad.play.view.*`) that should not gate actions.
    #[serde(default)]
    #[value(default)]
    pub allows_actions_while_active: bool,
    /// ⏯️ Declares that activating this utility runs a visible, abortable algorithm (tool run contract §2.4);
    /// injects [`tool_run_action_definitions`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub run: Option<ToolRunDefinition>,
}

impl UtilityDefinition {
    /// 🧰️ A utility with sensible defaults (no group/keys/cursor/category/run, gates actions while active).
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), group: None, keys: None, cursor: None, category: None, allows_actions_while_active: false, run: None }
    }
}

/// 🧰️ A validated reference into an app's `AppDefinition.utilities` registry — the utility mirror of
/// `ActionRef`, scoping utilities to window kinds/modes with a typed, resolvable id.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct UtilityRef(String);

#[path = "🪛️utilities/🌅️initial/🦀️.rs"]
mod initial_window_utility;
pub use initial_window_utility::{InitialWindowUtilityInput, InitialWindowUtilityResolution, resolve_initial_window_utility};

impl UtilityRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for UtilityRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for UtilityRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}
//#endregion 🔖️Utilities

//#region 🔖️Commands
/// 🎛️ Declares one command: a categorized verb offered in the footer command panel.
/// Its owner and availability are derived from the containing OS, plugin, app, or mode definition.
/// Handling a command may emit VCS-tracked operations exactly like an operation-kind action — see
/// `ArtifactApp::handle_command`/`ActionEmit`.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    /// 🗂️ Footer category tab this command groups under (an open id, e.g. "document", "appearance").
    pub category: String,
    pub icon_id: IconName,
    pub kind: ActionKind,
    /// 📝️ Reuses `ActionArgDef` — one staged-form contract shared by actions, dialogs, and commands.
    pub args: Vec<ActionArgDef>,
    #[serde(default)]
    #[value(default)]
    pub keybindings: Vec<PlatformKeybinding>,
    #[serde(default)]
    #[value(default)]
    pub in_palette: bool,
    /// 🎯️ See `ActionDefinition.semantics` — same D6/§3.1 field, same defaulting/inheritance story.
    #[serde(default)]
    #[value(default)]
    pub semantics: ActionSemantics,
}

impl CommandDefinition {
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, icon_id: impl Into<IconName>, kind: ActionKind) -> Self {
        Self { id: id.into(), label: label.into(), category: category.into(), icon_id: icon_id.into(), kind, args: Vec::new(), keybindings: Vec::new(), in_palette: true, semantics: ActionSemantics::for_kind(kind) }
    }

    /// 🎛️ Declares a command with the neutral icon for its action kind.
    pub fn new_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, kind: ActionKind) -> Self {
        Self::new(id, label, category, default_action_icon_id(kind), kind)
    }

    /// ⚡️ Builds a catalog row without granting UI execution authority. A factory registration or
    /// static bounded-first-step proof must classify the exact key separately.
    pub fn bounded_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, kind: ActionKind) -> Self {
        Self::new_catalog(id, label, category, kind)
    }

    /// 📝️ Attaches typed argument declarations to this command.
    pub fn with_args(mut self, args: impl IntoIterator<Item = ActionArgDef>) -> Self {
        self.args = args.into_iter().collect();
        self
    }

    /// ⌨️ Attaches one platform-aware command keybinding.
    pub fn with_keybinding(mut self, keybinding: PlatformKeybinding) -> Self {
        self.keybindings.push(keybinding);
        self
    }

    /// 🎯️ Replaces this command's whole `ActionSemantics` wholesale.
    pub async fn semantics(mut self, semantics: ActionSemantics) -> Self {
        self.semantics = semantics;
        self
    }

    /// ⚠️ Marks this command destructive — see `ActionDefinition::destructive`.
    pub fn destructive(mut self) -> Self {
        self.semantics.effects.destructive = true;
        if self.semantics.policy.approval == ApprovalMode::Never {
            self.semantics.policy.approval = ApprovalMode::WhenDestructive;
        }
        self
    }

    /// 🗣️ Sets `ActionSemantics.use_when` — see `ActionDefinition::use_when`.
    pub async fn use_when(mut self, phrases: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.semantics.use_when = phrases.into_iter().map(Into::into).collect();
        self
    }

    /// 💬️ Sets `ActionSemantics.description` — see `ActionDefinition::describe`.
    pub fn describe(mut self, description: impl Into<LocalizedLabel>) -> Self {
        self.semantics.description = Some(description.into());
        self
    }

    /// 🎯️ Declares this command's [`CapabilityAudience`] — see `ActionDefinition::audience`.
    pub fn audience(mut self, audience: CapabilityAudience) -> Self {
        self.semantics.audience = Some(audience);
        self
    }

    /// 📖️ Appends one `ActionSemantics.examples` entry — see `ActionDefinition::example`.
    pub async fn example(mut self, example: impl Into<String>) -> Self {
        self.semantics.examples.push(example.into());
        self
    }
}

/// 📍️ Hierarchical owner of a command definition.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CommandOwnerAddress {
    Os,
    Plugin { plugin_id: String },
    App { plugin_id: String, app_id: String },
    Mode { plugin_id: String, app_id: String, mode_id: String },
}

/// 📍️ Fully qualified address of one command.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct CommandAddress {
    pub owner: CommandOwnerAddress,
    pub command_id: String,
}

/// 📨️ One addressed command invocation with named DSL-value arguments — see
/// `ActionInvocation`'s docstring for why `arguments` is keyed to `DslValue`, not
/// `serde_json::Value`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct CommandInvocation {
    pub address: CommandAddress,
    pub arguments: BTreeMap<String, DslValue>,
}

/// 👓️ Plugin handlers borrow the retained semantic fields instead of materializing a second argument map.
pub struct CommandInvocationView<'a>{pub address:&'a CommandAddress,pub arguments:&'a DslValue}

/// 💻️ Operating-system command catalog shared by every renderer.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct OsDefinition {
    #[serde(default)]
    #[value(default)]
    pub commands: Vec<CommandDefinition>,
}
//#endregion 🔖️Commands

//#region 🔖️Tools
/// 🛠️ Declares one mode-level tool: an activatable, stateful capability of a whole app mode.
/// Distinct from `UtilityDefinition` (a per-window pointer mode — a utility is a tool for a specific
/// window) and `CommandDefinition` (a fire-once verb): exactly one tool is active per app at a time,
/// and activation is host-owned session view state (`ViewModel.active_tool_id`), never a document
/// field or VCS operation. A tool's live options are supplied dynamically via `ArtifactApp::tool_measures`,
/// keyed by tool id — not part of this static declaration.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ToolDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub keys: Option<String>,
    /// ⏯️ Declares that this tool runs a visible, abortable, finalizable algorithm (tool run contract §2.4);
    /// injects [`tool_run_action_definitions`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub run: Option<ToolRunDefinition>,
}

impl ToolDefinition {
    /// 🛠️ A tool with sensible defaults (no keybinding, no run).
    pub async fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), keys: None, run: None }
    }
}

/// 🛠️ A validated reference into an app's `AppDefinition.tools` registry — the tool mirror of
/// `UtilityRef`, scoping tools to modes with a typed, resolvable id.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct ToolRef(String);

impl ToolRef {
    pub async fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    // 🚫️async: E1 transitive — the only consumer is inside an `Iterator::find` (external trait)
    // closure, which must be sync; pure field access, no I/O (R9).
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ToolRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ToolRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}
//#endregion 🔖️Tools

//#region 🆔️ElementId
/// 🆔️ Whether `id` matches the renderer-agnostic UI element id grammar: dot-separated segments,
/// each starting with a lowercase letter and continuing with letters/digits only (camelCase, no
/// hyphens/underscores) — e.g. `framework.window.main.action.addLayer`. This id is the single
/// integration key across i18n, tooltips, hotkeys, command origin tracking, tutorials, E2E selectors,
/// and introduction anchors; each renderer maps it onto its own element (React → DOM `id` attribute,
/// wgpu → hit-target `control_id`), so no renderer-specific shape leaks into the grammar itself.
pub fn is_element_id(id: &str) -> bool {
    if id.is_empty() {
        return false;
    }
    id.split('.').all(|segment| {
        let mut chars = segment.chars();
        match chars.next() {
            Some(first) if first.is_ascii_lowercase() => chars.all(|c| c.is_ascii_alphanumeric()),
            _ => false,
        }
    })
}

/// 🆔️ Normalizes arbitrary input (a domain object's own id, a free-text label, an already
/// grammar-safe word) into a single camelCase element-id segment: splits on `-`/`_`/` `/`.`, lowercases
/// the very first character, capitalizes the first character after each separator, and drops any other
/// non-alphanumeric character. Idempotent on input that is already a valid segment. Used as the last
/// resort by `child_element_id` when a child id is derived from something not already grammar-safe (e.g.
/// a runtime label) — prefer a real semantic key first, then this, then a numeric index.
pub fn element_id_segment(raw: &str) -> String {
    let mut segment = String::new();
    let mut capitalize_next = false;
    for ch in raw.chars() {
        if ch == '-' || ch == '_' || ch == ' ' || ch == '.' {
            capitalize_next = true;
            continue;
        }
        if !ch.is_ascii_alphanumeric() {
            continue;
        }
        if segment.is_empty() {
            segment.push(ch.to_ascii_lowercase());
        } else if capitalize_next {
            segment.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            segment.push(ch);
        }
    }
    segment
}

/// 🆔️ Derives a child element id by suffixing `parent` with one or more segments, each normalized
/// through `element_id_segment` — the hierarchical mechanism every composite element uses to name its
/// parts instead of a context/registry: `child_element_id("ui.chat", &["send"])` → `"ui.chat.send"`.
pub fn child_element_id(parent: &str, segments: &[&str]) -> String {
    let mut id = parent.to_string();
    for segment in segments {
        id.push('.');
        id.push_str(&element_id_segment(segment));
    }
    id
}

/// 🆔️ Element id of the app shell's navbar — singular, shell-owned chrome.
pub const UI_NAVBAR_ELEMENT_ID: &str = "ui.navbar";
/// 🆔️ Element id of the app shell's footer — singular, shell-owned chrome.
pub const UI_FOOTER_ELEMENT_ID: &str = "ui.footer";

/// 🆔️ Element id of a window kind's body — `framework.window.{camelCased kind id}`.
pub fn window_element_id(kind_id: &str) -> String {
    child_element_id("framework.window", &[kind_id])
}

/// 🆔️ Element id of a panel tab's uncollapsed panel body. `tab_id` is already a dotted
/// `PanelTabDefinition.id()` (e.g. `puzzle.catalogue`) — appended verbatim rather than through
/// `child_element_id`, which would collapse its dots into camelCase.
pub fn panel_tab_element_id(tab_id: &str) -> String {
    format!("framework.panelTab.{tab_id}")
}

/// 🆔️ Alias id of the first draggable tree row inside a panel tab (document order within that
/// uncollapsed panel) — stamped via `data-element-alias` since no single tree row has a stable semantic
/// id at authoring time. Used to teach catalogue drag-and-drop without hardcoding a kind id.
pub fn panel_tab_first_draggable_element_id(tab_id: &str) -> String {
    format!("framework.panelTab.{tab_id}.firstDraggable")
}
//#endregion 🆔️ElementId

// 🎗️ `AppLabelsOverlay` (the stringly-typed, per-id runtime label-patch map) is deleted — manifest
// labels are now `LocalizedLabel` fields resolved directly via `.resolve(terminology, locale)`, so a
// separate locale-aware overlay merged in after the fact is no longer needed. Downstream callers
// (plugin crates' `ArtifactApp::app_labels()`, the OS renderer's overlay-merge call sites) are
// follow-up work owned by other agents — left broken intentionally, out of scope here.

//#region 🔖️Kernel
#[path = "../🎠️kernel/🦀️.rs"]
pub mod kernel;
//#endregion 🔖️Kernel

#[cfg(test)]
#[path = "🧪️tests/🔬️media-vocabulary/🦀️.rs"]
mod media_vocabulary_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️app-label/🦀️.rs"]
mod app_label_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️example-picker/🦀️.rs"]
mod example_picker_tests;

#[cfg(test)]
#[path = "🧪️tests/🪟️resolved-host-context/🦀️.rs"]
mod resolved_host_context_tests;

#[cfg(test)]
#[path = "🧪️tests/🖐️gumball-verb-audience/🦀️.rs"]
mod gumball_verb_audience_tests;
//#endregion 🔖️Manifest

// #endregion 🛂️Manifest

#[cfg(test)]
#[path="🔖️action-args/♻️retirement/🧪️tests/🦀️.rs"]
mod argument_retirement_tests;
