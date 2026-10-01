// #region 🛂️Manifest
//! 🧩️ App manifest (`AppDefinition`/`ModeDefinition`/`WindowKindDefinition`/`PluginManifest`/`ViewModel`)
//! and kernel types shared by plugins and renderers; the declarative `UiNode` component model itself
//! lives in `ui_wgpu`'s `component` region.

// 🧭️ `dsl` already re-exports BOTH the `ToValue`/`FromValue` traits and their derive macros
// (`💻️os/📦️packages/🦀️rust/🦀️.rs:347`), so importing them again from `semio_framework_value_derive`
// is a same-namespace redefinition (E0252), not a second namespace.
use dsl::{DslValue, FromValue, ToValue, ValueError};
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
use ui_wgpu::wgpu::{ActionDescriptor, Locale, LocalizedLabel, NamedLayout, SurfaceKind, Terminology, WindowLayout, WindowOptions};
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

//#region 🔖️Manifest
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Keybinding {
    pub keys: String,
    pub action: ActionDescriptor,
}

/// ⌨️ Operating system selector for a platform-specific keybinding.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum Platform {
    MacOs,
    Windows,
    Linux,
}

/// ⌨️ One command chord, optionally restricted to a host platform.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
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

/// 🗂️ Classifies a declared action by how it interacts with VCS history.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ActionKind {
    /// Mutates the document — dispatched as VCS mutations with a true inverse, recorded in history.
    Mutation,
    /// Ephemeral view state (camera, selection, hover, active utility) — recorded in the session
    /// command log, never as a VCS edit.
    View,
    /// Framework-provided undo/redo/checkpoint/alternative — auto-injected, never app-declared.
    History,
    /// Framework-provided copy/cut/paste — auto-injected, never app-declared (mirrors `History`).
    Clipboard,
    /// Shell-only effect (navigate, export, spawn) — recorded in the session command log via
    /// dispatch or the `noteShellCommand` mechanism, no document mutation.
    Shell,
    /// Framework-provided hover/selection — auto-injected, never app-declared.
    Interaction,
}

//#region 🔖️ArgSchema
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema, D6: the
// stored, engine-neutral shape of one action argument's VALUE. `ActionArgDef.schema` (below, in
// `🔖️ActionArgs`) is now the ONLY persisted truth; `ActionArgControl` (the renderer's widget
// vocabulary, unchanged) is DERIVED fresh on every read by `ActionArgDef::control()` — never stored
// twice. `ArgFormat`'s `ArtifactKind`/`SurfaceApp` variants are this region's one addition beyond
// `📋️master.md` §3.1's literal format table: the pre-existing host-resolved
// `ActionArgControl::ArtifactKind`/`SurfaceApp` controls (see `🔖️HostResolvedArgs`) need SOME
// `ArgSchema` origin now that `control` is derived, not stored, and they are structurally exactly
// this — a `String` value whose valid set the host resolves from `roles` right before render.
/// 🧬️ Semantic refinement of a `String`-typed `ArgSchema` leaf — what KIND of string this is,
/// beyond "text". Orthogonal to `ArgPresentation` (which is about the WIDGET, not the value's
/// semantics): a `Color` format could still render as free text in a minimal shell.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ArgFormat {
    ArtifactRef,
    WindowId,
    // 🐛️ `entity_kind`, not `kind`: a struct-variant field literally named `kind` collides with this
    // enum's own internal tag key (`#[serde(tag = "kind")]`) and serde's derive hard-errors on it.
    EntityId {
        entity_kind: String,
    },
    IconId,
    Color,
    Uri,
    Json,
    Locale,
    Terminology,
    /// 🗂️ Host-resolved artifact-kind choice — see `ActionArgControl::ArtifactKind` and
    /// `ActionArgDef::artifact_kind`.
    ArtifactKind {
        roles: Vec<AppRole>,
    },
    /// 🎭️ Host-resolved `(pluginId, appId, role)` choice — see `ActionArgControl::SurfaceApp` and
    /// `ActionArgDef::surface_app`.
    SurfaceApp {
        roles: Vec<AppRole>,
        dialect_arg: String,
    },
    /// 🔐️ The document's own revision token, as its rendered bindings carry it — see `ActionArgDef::document_revision`.
    DocumentRevision,
    /// 🔐️ One addressed target's own revision token (a page item, a cell, an entry), as its rendered binding carries it —
    /// see `ActionArgDef::target_revision`.
    TargetRevision,
}

/// 🌳️ The stored, engine-neutral shape of one action argument's value — see this region's
/// header comment for the D6 stored/derived split.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ArgSchema {
    String {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        options: Vec<ActionArgOption>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        min_len: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        max_len: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pattern: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        format: Option<ArgFormat>,
    },
    /// 🔢️ `min`/`max` are hard bounds (`*_exclusive` makes one strict); every other field is a UI fact: `step` never
    /// constrains validity, `soft_min`/`soft_max` narrow a slider's travel inside the hard bounds, `snaps` and
    /// `snap_source` pull a dragged value onto points, `display_factor` converts the stored `unit` into `display_unit`.
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        #[value(default, skip_serializing_if = "std::ops::Not::not")]
        min_exclusive: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        #[value(default, skip_serializing_if = "std::ops::Not::not")]
        max_exclusive: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        #[serde(default)]
        #[value(default)]
        integer: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        snaps: Vec<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        snap_source: Option<SnapSource>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        soft_min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        soft_max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        precision: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_factor: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        scale: Option<NumberScale>,
    },
    Boolean,
    /// 🧭️ A fixed-length numeric tuple (`dims` components, e.g. a 3d position) edited component by component: every
    /// component shares the inclusive hard bounds `min`/`max` and the number facets (`step`, `snaps`, `snap_source` — the
    /// grid a 3d offset snaps to — `precision`, `display_unit`, `display_factor`). Under [`ArgPresentation::Color`] a
    /// vector of 3 or 4 components within `0..=1` is an sRGB colour with straight alpha.
    Vector {
        dims: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        snaps: Vec<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        snap_source: Option<SnapSource>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        precision: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_factor: Option<f64>,
    },
    /// 🎯️ The id (or, when `many`, the ids) of document entities of one of `kinds`, picked from the selection of
    /// `domain` at `granularity` when both are named — the selection picker behind "use selection".
    Reference {
        kinds: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        domain: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        granularity: Option<String>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        #[value(default, skip_serializing_if = "std::ops::Not::not")]
        many: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        min_items: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        max_items: Option<u32>,
        #[serde(default, skip_serializing_if = "ReferenceIdType::is_string")]
        #[value(default, skip_serializing_if = "ReferenceIdType::is_string")]
        id_type: ReferenceIdType,
    },
    Array {
        items: Box<ArgSchema>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        min_items: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        max_items: Option<u32>,
    },
    Object {
        fields: Vec<ActionArgDef>,
    },
    Any,
}

impl ArgSchema {
    /// 🎯️ Every entity kind this value names through [`ArgFormat::EntityId`], at any depth.
    pub fn entity_kinds(&self) -> Vec<&str> {
        match self {
            ArgSchema::String { format: Some(ArgFormat::EntityId { entity_kind }), .. } => vec![entity_kind.as_str()],
            ArgSchema::Array { items, .. } => items.entity_kinds(),
            ArgSchema::Object { fields } => fields.iter().flat_map(|field| field.schema.entity_kinds()).collect(),
            _ => Vec::new(),
        }
    }
}

/// 🪪️ The entity kind an [`ArgFormat::EntityId`] value names for one granularity of one declared interaction:
/// `<interaction id>/<granularity id>` — the same pair a selection target of that interaction carries.
pub fn interaction_entity_kind(interaction_id: &str, granularity_id: &str) -> String {
    format!("{interaction_id}/{granularity_id}")
}

/// 🖼️ How to WIDGET-render an argument beyond what its `ArgSchema` alone implies — consumed by
/// `ActionArgDef::control()` (e.g. a bounded `Number` still renders `Slider` without this, but a
/// single-bound one needs it to opt in).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ArgPresentation {
    Slider,
    Stepper,
    Dial,
    Segmented,
    IconSelect { classifier_kind: String },
    Multiline,
    Hidden,
    /// 🎨️ An sRGB colour: a [`ArgSchema::Vector`] of 3 (RGB) or 4 (RGBA, straight alpha) components within `0..=1`.
    /// The one canonical colour mapping — a hex string stays `text`.
    Color,
}

/// 🧲️ Where a number's snap points come from beyond its static `snaps`: every multiple of its `step`, a window
/// config value named `key` (the grid spacing a tool snaps to), or the document value at JSON `pointer`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SnapSource {
    Step,
    Config { key: String },
    Snapshot { pointer: String },
}

/// 📈️ How a slider or dial maps its travel onto the value range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum NumberScale {
    Linear,
    Log,
}

/// 🔢️ The JSON type of a reference's ids: text ids (the default, left off the wire) or integer ids. A selection id is
/// always text, so an integer reference stages the integer its selected text spells ([`ReferenceIdType::id_value`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ReferenceIdType {
    #[default]
    String,
    Integer,
}

impl ReferenceIdType {
    /// 🔤️ Whether the ids are text — the default.
    pub fn is_string(&self) -> bool {
        *self == Self::String
    }

    /// 🎯️ The payload value of one selected id: the text itself, or the integer it spells — `None` for empty text or,
    /// for an integer reference, text that is not a decimal integer within ±(2^53 − 1). TS twin: `referenceIdValue`.
    pub fn id_value(self, id: &str) -> Option<DslValue> {
        if id.is_empty() {
            return None;
        }
        match self {
            Self::String => Some(DslValue::String(id.to_string())),
            Self::Integer => {
                let digits = id.strip_prefix('-').unwrap_or(id);
                if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
                    return None;
                }
                let value = id.parse::<i64>().ok().filter(|value| value.unsigned_abs() <= REFERENCE_ID_INTEGER_MAX)?;
                Some(if value < 0 { DslValue::int(value) } else { DslValue::uint(value as u64) })
            }
        }
    }
}

/// 🔢️ The largest integer id magnitude a reference admits — the exact-integer range every host's numbers share.
pub const REFERENCE_ID_INTEGER_MAX: u64 = (1 << 53) - 1;

/// 🏷️ The text of one reference id value — a non-empty string id as it is, an integer id in decimal (the spelling a
/// selection carries); `None` for anything else. TS twin: `referenceIdText`.
pub fn reference_id_text(value: &DslValue) -> Option<String> {
    match value {
        DslValue::String(id) if !id.is_empty() => Some(id.clone()),
        DslValue::Number(number) => match (number.as_i64(), number.as_u64()) {
            (Some(value), _) if value.unsigned_abs() <= REFERENCE_ID_INTEGER_MAX => Some(value.to_string()),
            (None, Some(value)) if value <= REFERENCE_ID_INTEGER_MAX => Some(value.to_string()),
            _ => None,
        },
        _ => None,
    }
}
//#endregion 🔖️ArgSchema

//#region 🔖️ActionArgs
/// 🔘️ One selectable option of a `Select` argument control — the persisted `value` and its
/// human `label`.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ActionArgOption {
    pub value: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel`. Not yet owned-schema-mirrored
    /// (follow-up: `LocalizedLabel` itself has no `TS` impl).
    pub label: LocalizedLabel,
}

impl ActionArgOption {
    pub fn new(value: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self { value: value.into(), label: label.into() }
    }
}

/// 🎚️ Declarative input control for one action argument — a lean manifest-altitude enum,
/// deliberately NOT `ui_wgpu::wgpu::UiControlNode` (whose variants embed live values and immediate-dispatch
/// wiring). Renderers map each variant onto a staged form field. Tagged with `kind` to mirror the
/// sibling `UtilityNode`/`UiControlNode` declarative-tree convention.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActionArgControl {
    Text {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        placeholder: Option<String>,
    },
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        precision: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_factor: Option<f64>,
    },
    /// 🪜️ A number field with increment/decrement buttons of `step` (an integer steps by one unless it declares otherwise).
    Stepper {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        precision: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_factor: Option<f64>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        snaps: Vec<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        snap_source: Option<SnapSource>,
    },
    /// 🎚️ `min`/`max` are the travel range: the soft range when one is declared, else the hard bounds.
    Slider {
        min: f64,
        max: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        precision: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_factor: Option<f64>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        snaps: Vec<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        snap_source: Option<SnapSource>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        scale: Option<NumberScale>,
    },
    /// 🧭️ A rotary knob (an angle): `min`/`max` are its travel range exactly like [`Self::Slider`]'s.
    Dial {
        min: f64,
        max: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        precision: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_factor: Option<f64>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        snaps: Vec<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        snap_source: Option<SnapSource>,
    },
    Toggle,
    Select {
        options: Vec<ActionArgOption>,
    },
    /// 🔘️ The options as one row of mutually exclusive buttons — a [`Self::Select`] whose choices stay visible.
    Segmented {
        options: Vec<ActionArgOption>,
    },
    /// 🧭️ One number field per component, sharing the component facets of [`ArgSchema::Vector`].
    Vector {
        dims: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        snaps: Vec<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        snap_source: Option<SnapSource>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        precision: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        display_factor: Option<f64>,
    },
    /// 🎨️ A colour picker over an sRGB [`ArgSchema::Vector`] with components in `0..=1`: red, green, blue and, when
    /// `alpha`, a straight (non-premultiplied) alpha as the fourth component.
    Color {
        alpha: bool,
    },
    /// 🎯️ A selection picker: chips of the picked ids, remove, and "use selection" from `domain` at `granularity`.
    Reference {
        kinds: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        domain: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        granularity: Option<String>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        #[value(default, skip_serializing_if = "std::ops::Not::not")]
        many: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        min_items: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        max_items: Option<u32>,
        #[serde(default, skip_serializing_if = "ReferenceIdType::is_string")]
        #[value(default, skip_serializing_if = "ReferenceIdType::is_string")]
        id_type: ReferenceIdType,
    },
    IconSelect {
        classifier_kind: String,
    },
    /// 🗂️ Host-resolved: the plugin declares intent (which `AppRole`s qualify), the host resolves it
    /// into a plain `Select { options }` from its live plugin catalogue right before the dialog
    /// renders — see `artifact_kind_choices` and region `🔖️HostResolvedArgs` below. Mirrors the
    /// `IconSelect { classifier_kind }` precedent above (host-resolved, plugin declares only intent).
    ArtifactKind {
        roles: Vec<AppRole>,
    },
    /// 🎭️ Host-resolved: lists `(pluginId, appId, role)` for the dialect coordinate found in the
    /// dialog's seed argument named `dialect_arg` — see `artifact_kind_choices`'s sibling resolver
    /// and region `🔖️HostResolvedArgs` below.
    SurfaceApp {
        roles: Vec<AppRole>,
        dialect_arg: String,
    },
}

/// 📝️ Declares one input of an action or a mutation: its `id` (the JSON key sent in `ActionDescriptor.args`; for a
/// mutation input the single-segment RFC 6901 pointer of its key relative to its parent value, e.g. `/dx`), human
/// `label`, stored value `schema` (see `🔖️ArgSchema` — D6: this is the sole persisted truth, `control()` below is
/// derived from it), an optional widget `presentation` hint, whether it is `required` and `nullable`, an optional `default` value,
/// an optional localized `description`, and the `group`/`order` a form sorts it by. An empty
/// `ActionDefinition.args` (the common case) means a no-argument action. Mutation inputs come from
/// [`mutation_input_defs`].
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ActionArgDef {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub schema: ArgSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub presentation: Option<ArgPresentation>,
    #[serde(default)]
    #[value(default)]
    pub required: bool,
    /// 🫥️ Whether the value admits `null` (`type: [T, "null"]`, or a `null` branch beside one value branch): a "clear"
    /// that a form offers beside the value, distinct from leaving an optional input out.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    #[value(default, skip_serializing_if = "std::ops::Not::not")]
    pub nullable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub default: Option<DslValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedLabel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

impl ArgSchema {
    /// 🔢️ A plain number with hard bounds and a UI `step`, carrying no display facts.
    pub fn number(min: Option<f64>, max: Option<f64>, step: Option<f64>, integer: bool) -> Self {
        ArgSchema::Number { min, min_exclusive: false, max, max_exclusive: false, step, integer, unit: None, snaps: Vec::new(), snap_source: None, soft_min: None, soft_max: None, precision: None, display_unit: None, display_factor: None, scale: None }
    }
}

impl ActionArgDef {
    fn with_schema(id: impl Into<String>, label: impl Into<LocalizedLabel>, schema: ArgSchema) -> Self {
        Self { id: id.into(), label: label.into(), schema, presentation: None, required: false, nullable: false, default: None, description: None, group: None, order: None }
    }

    fn plain_string(format: Option<ArgFormat>) -> ArgSchema {
        ArgSchema::String { options: Vec::new(), min_len: None, max_len: None, pattern: None, format }
    }

    /// 🔤️ A free-text argument.
    pub fn text(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, Self::plain_string(None))
    }

    /// 🔢️ A numeric argument (a plain number field by default).
    pub fn number(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, ArgSchema::number(None, None, None, false))
    }

    /// 🔢️ A non-negative 32-bit ordinal (a stepper).
    pub fn index(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, ArgSchema::number(Some(0.0), Some(u32::MAX as f64), Some(1.0), true))
    }

    /// 🎚️ A bounded slider argument.
    pub fn slider(id: impl Into<String>, label: impl Into<LocalizedLabel>, min: f64, max: f64) -> Self {
        let mut def = Self::with_schema(id, label, ArgSchema::number(Some(min), Some(max), None, false));
        def.presentation = Some(ArgPresentation::Slider);
        def
    }

    /// 🔘️ A boolean toggle argument.
    pub fn toggle(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, ArgSchema::Boolean)
    }

    /// 🔽️ A single-choice select argument.
    pub fn select(id: impl Into<String>, label: impl Into<LocalizedLabel>, options: Vec<ActionArgOption>) -> Self {
        Self::with_schema(id, label, ArgSchema::String { options, min_len: None, max_len: None, pattern: None, format: None })
    }

    /// 🧭️ A `dims`-component numeric vector argument (a 3d point is `dims == 3`).
    pub fn vector(id: impl Into<String>, label: impl Into<LocalizedLabel>, dims: u32) -> Self {
        Self::with_schema(id, label, ArgSchema::Vector { dims, min: None, max: None, unit: None, step: None, snaps: Vec::new(), snap_source: None, precision: None, display_unit: None, display_factor: None })
    }

    /// 📜️ A list-of-strings argument — the shape every multi-entity verb takes (`ids`,
    /// `layerIds`), previously unexpressible, so those verbs published an empty input schema.
    pub fn text_list(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, ArgSchema::Array { items: Box::new(Self::plain_string(None)), min_items: None, max_items: None })
    }

    /// 🎯️ The ids of the entities a verb acts on, of one granularity of one declared interaction — the selection
    /// that verb otherwise reads, stated as an argument: a human may leave it empty and act on what they selected, an
    /// agent (which has no selection) names the entities. Each id is tagged `x-semio-format: entityId` with its
    /// [`interaction_entity_kind`], which the app's definition build checks against the interactions it declares.
    pub fn entity_ids(id: impl Into<String>, label: impl Into<LocalizedLabel>, interaction_id: &str, granularity_id: &str) -> Self {
        let entity_kind = interaction_entity_kind(interaction_id, granularity_id);
        Self::with_schema(id, label, ArgSchema::Array { items: Box::new(Self::plain_string(Some(ArgFormat::EntityId { entity_kind }))), min_items: None, max_items: None })
    }

    /// 🧱️ A record argument — the `#[dsl(block)]` payload shape a typed command decodes with
    /// `dsl::from_dsl_value` (`setFrame.frame`, `setSource.source`), previously unexpressible, so
    /// those verbs published an empty input schema and no agent could ever call them.
    pub fn object(id: impl Into<String>, label: impl Into<LocalizedLabel>, fields: Vec<ActionArgDef>) -> Self {
        Self::with_schema(id, label, ArgSchema::Object { fields })
    }

    /// 🧬️ An unconstrained typed value argument.
    pub fn any(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, ArgSchema::Any)
    }

    /// 🧬️ A JSON-text argument — a `String` wire field that actually carries a JSON document
    /// (`patchLayer.value`, `setFixtureJson.json`), tagged `x-semio-format: json` so a client knows
    /// to send JSON text rather than a bare word.
    pub fn json_text(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, Self::plain_string(Some(ArgFormat::Json)))
    }

    /// 🗂️ A host-resolved artifact-kind choice — see `ActionArgControl::ArtifactKind`.
    pub fn artifact_kind(id: impl Into<String>, label: impl Into<LocalizedLabel>, roles: Vec<AppRole>) -> Self {
        Self::with_schema(id, label, Self::plain_string(Some(ArgFormat::ArtifactKind { roles })))
    }

    /// 🎭️ A host-resolved `(pluginId, appId, role)` choice — see `ActionArgControl::SurfaceApp`.
    pub fn surface_app(id: impl Into<String>, label: impl Into<LocalizedLabel>, roles: Vec<AppRole>, dialect_arg: impl Into<String>) -> Self {
        Self::with_schema(id, label, Self::plain_string(Some(ArgFormat::SurfaceApp { roles, dialect_arg: dialect_arg.into() })))
    }

    /// 🔐️ The document revision a rendered binding carries — hidden and optional: the shell lanes pass the binding's
    /// token (and refuse its absence in the app's own parser); the agent lane omits it and is admitted against the document's
    /// revision at admission, behind the MCP `expectedRevision` guard (ticket 26/09/23, G12 session 14c).
    pub fn document_revision(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        let mut def = Self::with_schema(id, label, Self::plain_string(Some(ArgFormat::DocumentRevision)));
        def.presentation = Some(ArgPresentation::Hidden);
        def
    }

    /// 🔐️ One addressed target's revision a rendered binding carries — hidden and optional like
    /// [`Self::document_revision`]; the agent lane fills it from the app's own `agent_target_revision`.
    pub fn target_revision(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        let mut def = Self::with_schema(id, label, Self::plain_string(Some(ArgFormat::TargetRevision)));
        def.presentation = Some(ArgPresentation::Hidden);
        def
    }

    /// ❗️ Marks the argument as required — execution is blocked until it has an effective value.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// 🔤️ Constrains text length; zero explicitly admits an empty required value.
    pub fn min_length(mut self, minimum: u32) -> Self {
        if let ArgSchema::String { min_len, .. } = &mut self.schema {
            *min_len = Some(minimum);
        }
        self
    }

    /// 🎁️ Sets the default effective value used when nothing is staged.
    pub fn default_value(mut self, value: &impl ToValue) -> Self {
        self.default = Some(value.to_value());
        self
    }

    /// 💬️ Attaches a description shown alongside the field.
    pub fn describe(mut self, description: impl Into<LocalizedLabel>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 🔑️ The JSON key this input writes: `id` itself for an action argument, the decoded segment of a
    /// mutation input's single-segment pointer (`/new~1name` → `new/name`).
    pub fn key(&self) -> String {
        match self.id.strip_prefix('/') {
            Some(segment) if !segment.contains('/') => segment.replace("~1", "/").replace("~0", "~"),
            _ => self.id.clone(),
        }
    }

    /// 🎛️ Derives this argument's renderer-facing `ActionArgControl` from its stored `schema` +
    /// `presentation` — D6 (ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet
    /// P3-manifest-schema): `schema` is the ONLY persisted truth, this is computed fresh on every
    /// call, never cached/stored. Order matters: a non-empty `options` list always wins Select (or
    /// Segmented) over any format; a number's presentation wins, else an integer steps, else a
    /// fully-bounded number slides, else it is a plain number field. TypeScript twin: `argControl`.
    pub fn control(&self) -> ActionArgControl {
        match &self.schema {
            ArgSchema::String { options, format, .. } => {
                if !options.is_empty() {
                    return match self.presentation {
                        Some(ArgPresentation::Segmented) => ActionArgControl::Segmented { options: options.clone() },
                        _ => ActionArgControl::Select { options: options.clone() },
                    };
                }
                match format {
                    Some(ArgFormat::IconId) => ActionArgControl::IconSelect { classifier_kind: "icon".to_string() },
                    Some(ArgFormat::ArtifactKind { roles }) => ActionArgControl::ArtifactKind { roles: roles.clone() },
                    Some(ArgFormat::SurfaceApp { roles, dialect_arg }) => ActionArgControl::SurfaceApp { roles: roles.clone(), dialect_arg: dialect_arg.clone() },
                    _ => ActionArgControl::Text { placeholder: None },
                }
            }
            ArgSchema::Number { min, max, step, integer, unit, snaps, snap_source, soft_min, soft_max, precision, display_unit, display_factor, scale, .. } => {
                let travel_min = soft_min.or(*min).unwrap_or(0.0);
                let travel_max = soft_max.or(*max).unwrap_or(0.0);
                let (unit, display_unit, snaps, snap_source) = (unit.clone(), display_unit.clone(), snaps.clone(), snap_source.clone());
                let (step, precision, display_factor) = (*step, *precision, *display_factor);
                match self.presentation {
                    Some(ArgPresentation::Slider) => ActionArgControl::Slider { min: travel_min, max: travel_max, step, unit, precision, display_unit, display_factor, snaps, snap_source, scale: *scale },
                    Some(ArgPresentation::Dial) => ActionArgControl::Dial { min: travel_min, max: travel_max, step, unit, precision, display_unit, display_factor, snaps, snap_source },
                    Some(ArgPresentation::Stepper) => ActionArgControl::Stepper { min: *min, max: *max, step, unit, precision, display_unit, display_factor, snaps, snap_source },
                    _ if *integer => ActionArgControl::Stepper { min: *min, max: *max, step, unit, precision, display_unit, display_factor, snaps, snap_source },
                    _ if min.is_some() && max.is_some() => ActionArgControl::Slider { min: travel_min, max: travel_max, step, unit, precision, display_unit, display_factor, snaps, snap_source, scale: *scale },
                    _ => ActionArgControl::Number { min: *min, max: *max, step, unit, precision, display_unit, display_factor },
                }
            }
            ArgSchema::Boolean => ActionArgControl::Toggle,
            ArgSchema::Vector { dims, .. } if self.presentation == Some(ArgPresentation::Color) => ActionArgControl::Color { alpha: *dims == 4 },
            ArgSchema::Vector { dims, min, max, unit, step, snaps, snap_source, precision, display_unit, display_factor } => ActionArgControl::Vector {
                dims: *dims,
                min: *min,
                max: *max,
                unit: unit.clone(),
                step: *step,
                snaps: snaps.clone(),
                snap_source: snap_source.clone(),
                precision: *precision,
                display_unit: display_unit.clone(),
                display_factor: *display_factor,
            },
            ArgSchema::Reference { kinds, domain, granularity, many, min_items, max_items, id_type } => {
                ActionArgControl::Reference { kinds: kinds.clone(), domain: domain.clone(), granularity: granularity.clone(), many: *many, min_items: *min_items, max_items: *max_items, id_type: *id_type }
            }
            ArgSchema::Array { .. } | ArgSchema::Object { .. } | ArgSchema::Any => ActionArgControl::Text { placeholder: None },
        }
    }

    /// 📐️ JSON Schema (2020-12 leaf, no `$schema`/`$id` — the catalog compiler wraps those at
    /// the whole-action envelope, `📋️master.md` §3.2) for this one argument's value, folding in
    /// `default` and the `description` resolved to one locale × terminology.
    pub fn json_schema(&self, terminology: Terminology, locale: Locale) -> DslValue {
        let value = arg_schema_json_schema(&self.schema, terminology, locale);
        let value = if self.nullable { DslValue::object([("anyOf".to_string(), DslValue::Array(vec![value, DslValue::object([("type".to_string(), DslValue::String("null".to_string()))])]))]) } else { value };
        let DslValue::Object(mut entries) = value else {
            unreachable!("arg_schema_json_schema always returns an object");
        };
        if let Some(description) = &self.description {
            entries.push(("description".to_string(), DslValue::String(description.resolve(terminology, locale).to_string())));
        }
        if let Some(default) = &self.default {
            entries.push(("default".to_string(), default.clone()));
        }
        DslValue::Object(entries)
    }
}

/// 🧬️ Tags a leaf/nested `ArgSchema` JSON Schema object with its `ArgFormat` — `x-semio-format`
/// (the vendor extension every format carries) plus, for the two host-resolved refinements, the
/// `roles`/`dialect_arg` a host needs to resolve them (`x-semio-roles`/`x-semio-dialect-arg`) — and
/// the standard `format: "uri"` keyword where JSON Schema already defines one.
fn apply_arg_format(entries: &mut Vec<(String, DslValue)>, format: &ArgFormat) {
    let tag = match format {
        ArgFormat::ArtifactRef => "artifactRef",
        ArgFormat::WindowId => "windowId",
        ArgFormat::EntityId { entity_kind } => {
            entries.push(("x-semio-entity-kind".to_string(), DslValue::String(entity_kind.clone())));
            "entityId"
        }
        ArgFormat::IconId => "iconId",
        ArgFormat::Color => "color",
        ArgFormat::Uri => {
            entries.push(("format".to_string(), DslValue::String("uri".to_string())));
            "uri"
        }
        ArgFormat::Json => "json",
        ArgFormat::Locale => "locale",
        ArgFormat::Terminology => "terminology",
        ArgFormat::DocumentRevision => "documentRevision",
        ArgFormat::TargetRevision => "targetRevision",
        ArgFormat::ArtifactKind { roles } => {
            entries.push(("x-semio-roles".to_string(), DslValue::Array(roles.iter().map(ToValue::to_value).collect())));
            "artifactKind"
        }
        ArgFormat::SurfaceApp { roles, dialect_arg } => {
            entries.push(("x-semio-roles".to_string(), DslValue::Array(roles.iter().map(ToValue::to_value).collect())));
            entries.push(("x-semio-dialect-arg".to_string(), DslValue::String(dialect_arg.clone())));
            "surfaceApp"
        }
    };
    entries.push(("x-semio-format".to_string(), DslValue::String(tag.to_string())));
}

/// 📐️ JSON Schema 2020-12 for one `ArgSchema` node (recursive over `Array`/`Object`) — carries
/// `Number.unit`/`Vector.unit` as `x-semio-unit`, `String.format` via `apply_arg_format`, a
/// `Reference` as `x-semio-format: reference` with its `x-semio-ref`. A number's `step` is a UI fact, never
/// `multipleOf`. No `additionalProperties`/`$schema`/`$id` at this altitude; the catalog compiler owns the envelope.
fn arg_schema_json_schema(schema: &ArgSchema, terminology: Terminology, locale: Locale) -> DslValue {
    match schema {
        ArgSchema::String { options, min_len, max_len, pattern, format } => {
            let mut entries = vec![("type".to_string(), DslValue::String("string".to_string()))];
            if !options.is_empty() {
                entries.push(("enum".to_string(), DslValue::Array(options.iter().map(|option| DslValue::String(option.value.clone())).collect())));
            }
            if let Some(min_len) = min_len {
                entries.push(("minLength".to_string(), min_len.to_value()));
            }
            if let Some(max_len) = max_len {
                entries.push(("maxLength".to_string(), max_len.to_value()));
            }
            if let Some(pattern) = pattern {
                entries.push(("pattern".to_string(), DslValue::String(pattern.clone())));
            }
            if let Some(format) = format {
                apply_arg_format(&mut entries, format);
            }
            DslValue::Object(entries)
        }
        ArgSchema::Number { min, min_exclusive, max, max_exclusive, integer, unit, .. } => {
            let mut entries = vec![("type".to_string(), DslValue::String(if *integer { "integer" } else { "number" }.to_string()))];
            if let Some(min) = min {
                entries.push((if *min_exclusive { "exclusiveMinimum" } else { "minimum" }.to_string(), min.to_value()));
            }
            if let Some(max) = max {
                entries.push((if *max_exclusive { "exclusiveMaximum" } else { "maximum" }.to_string(), max.to_value()));
            }
            if let Some(unit) = unit {
                entries.push(("x-semio-unit".to_string(), DslValue::String(unit.clone())));
            }
            DslValue::Object(entries)
        }
        ArgSchema::Boolean => DslValue::object([("type".to_string(), DslValue::String("boolean".to_string()))]),
        ArgSchema::Vector { dims, min, max, unit, .. } => {
            let mut component = vec![("type".to_string(), DslValue::String("number".to_string()))];
            component.extend(min.iter().map(|min| ("minimum".to_string(), DslValue::json_number(*min))));
            component.extend(max.iter().map(|max| ("maximum".to_string(), DslValue::json_number(*max))));
            let mut entries = vec![
                ("type".to_string(), DslValue::String("array".to_string())),
                ("items".to_string(), DslValue::Object(component)),
                ("minItems".to_string(), DslValue::uint(u64::from(*dims))),
                ("maxItems".to_string(), DslValue::uint(u64::from(*dims))),
            ];
            if let Some(unit) = unit {
                entries.push(("x-semio-unit".to_string(), DslValue::String(unit.clone())));
            }
            DslValue::Object(entries)
        }
        ArgSchema::Reference { kinds, domain, granularity, many, min_items, max_items, id_type } => {
            let mut reference = vec![("kind".to_string(), DslValue::Array(kinds.iter().map(|kind| DslValue::String(kind.clone())).collect()))];
            reference.extend(domain.iter().map(|domain| ("domain".to_string(), DslValue::String(domain.clone()))));
            reference.extend(granularity.iter().map(|granularity| ("granularity".to_string(), DslValue::String(granularity.clone()))));
            let id = DslValue::object([
                ("type".to_string(), DslValue::String(if id_type.is_string() { "string" } else { "integer" }.to_string())),
                ("x-semio-format".to_string(), DslValue::String("reference".to_string())),
                ("x-semio-ref".to_string(), DslValue::Object(reference)),
            ]);
            if !*many {
                return id;
            }
            let mut entries = vec![("type".to_string(), DslValue::String("array".to_string())), ("items".to_string(), id)];
            entries.extend(min_items.iter().map(|min_items| ("minItems".to_string(), min_items.to_value())));
            entries.extend(max_items.iter().map(|max_items| ("maxItems".to_string(), max_items.to_value())));
            DslValue::Object(entries)
        }
        ArgSchema::Array { items, min_items, max_items } => {
            let mut entries = vec![("type".to_string(), DslValue::String("array".to_string())), ("items".to_string(), arg_schema_json_schema(items, terminology, locale))];
            if let Some(min_items) = min_items {
                entries.push(("minItems".to_string(), min_items.to_value()));
            }
            if let Some(max_items) = max_items {
                entries.push(("maxItems".to_string(), max_items.to_value()));
            }
            DslValue::Object(entries)
        }
        ArgSchema::Object { fields } => {
            let mut properties = Vec::new();
            let mut required = Vec::new();
            for field in fields {
                properties.push((field.key(), field.json_schema(terminology, locale)));
                if field.required {
                    required.push(DslValue::String(field.key()));
                }
            }
            let mut entries = vec![
                ("type".to_string(), DslValue::String("object".to_string())),
                ("properties".to_string(), DslValue::Object(properties)),
                ("additionalProperties".to_string(), DslValue::Bool(false)),
            ];
            if !required.is_empty() {
                entries.push(("required".to_string(), DslValue::Array(required)));
            }
            DslValue::Object(entries)
        }
        ArgSchema::Any => DslValue::Object(vec![]),
    }
}
//#endregion 🔖️ActionArgs

//#region 🔖️MutationInputs
/// 🧭️ Resolves a cross-document `$ref`: the schema document whose `$id` is `id` (the reference without its fragment).
pub trait InputSchemaResolver {
    fn resolve(&self, id: &str) -> Option<DslValue>;
}

impl<F: Fn(&str) -> Option<DslValue>> InputSchemaResolver for F {
    fn resolve(&self, id: &str) -> Option<DslValue> {
        self(id)
    }
}

/// 🚫️ The class of a [`InputSchemaError`], shared verbatim with the TypeScript twin and the lint `schema-mutation-input-ui`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub enum InputSchemaErrorCode {
    Malformed,
    RefUnresolved,
    UiInvalid,
    WidgetIncompatible,
    LabelMissing,
    OptionLabelMissing,
    LocaleMissing,
}

/// 🚫️ Why a mutation payload schema yields no input descriptors: the `code`, the RFC 6901 `pointer` of the input
/// in the payload (`""` for the payload itself, `-` for "every array item") and a human `detail`.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct InputSchemaError {
    pub code: InputSchemaErrorCode,
    pub pointer: String,
    pub detail: String,
}

impl std::fmt::Display for InputSchemaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?} at {:?}: {}", self.code, self.pointer, self.detail)
    }
}

impl std::error::Error for InputSchemaError {}

/// 📚️ The framework input-label glossary (`🔣️input-labels.json`, schema `InputLabelGlossary`): a label in every
/// locale for the most frequent mutation input names, the fallback when an input carries no `x-semio-ui.label`.
const INPUT_LABEL_GLOSSARY_JSON: &str = include_str!("🔣️input-labels.json");

/// 📚️ The parsed [`INPUT_LABEL_GLOSSARY_JSON`], field name → label.
pub fn input_label_glossary() -> &'static BTreeMap<String, LocalizedLabel> {
    static GLOSSARY: std::sync::OnceLock<BTreeMap<String, LocalizedLabel>> = std::sync::OnceLock::new();
    GLOSSARY.get_or_init(|| {
        let document = dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::parse(INPUT_LABEL_GLOSSARY_JSON).expect("the input label glossary is JSON"));
        let labels = document.get("labels").and_then(DslValue::as_object).expect("the input label glossary carries labels");
        labels.iter().map(|(name, label)| (name.clone(), input_localized_text(label).expect("every glossary label names every locale"))).collect()
    })
}

/// 🧬️ Reads a mutation leaf's payload JSON Schema (`MutationLeaf::PAYLOAD_SCHEMA`) into one [`ActionArgDef`] per
/// input, in `properties` order: `$ref`s resolve against the document itself (`#/$defs/…`) or, for another `$id`,
/// through `resolver`; hard bounds come from the standard keywords, UI facts from `x-semio-ui` (an outer annotation
/// overrides the one on the `$ref` target key by key); a `const` or `role: discriminator` input is skipped; objects
/// recurse into their fields. Without `x-semio-ui` the widget is inferred exactly as [`ActionArgDef::control`]
/// derives it, a string or string array named `<kind>Id(s)` becomes a [`ArgSchema::Reference`] of that kind, and
/// the label comes from [`input_label_glossary`] — an input with no label in every locale is an error, never a
/// fallback. TypeScript twin: `mutationInputDefs` (`🛂️manifest/🟦️.ts`); fixture `🧫️fixtures/🧫️mutation-inputs`.
pub fn mutation_input_defs(schema_json: &str, resolver: &dyn InputSchemaResolver) -> Result<Vec<ActionArgDef>, InputSchemaError> {
    InputSchemaReader::parse(schema_json, resolver, false)?.inputs()
}

/// 🧺️ Every finding of a mutation leaf payload schema at once: the collecting twin of the fail-fast
/// [`mutation_input_defs`] a runtime uses. Each refused input (at every nested pointer) is recorded and reading goes on
/// past it — a label, description, group, order, role or widget fault keeps the input, a value fault drops its schema —
/// so `findings` names every pointer an author must fix (first occurrence of each distinct finding, in reading order;
/// its first entry is the error the fail-fast reader returns) and `inputs` holds what reads despite them. TypeScript
/// twin: `mutationInputAudit`.
#[derive(Clone, Debug, PartialEq)]
pub struct InputSchemaAudit {
    pub inputs: Vec<ActionArgDef>,
    pub findings: Vec<InputSchemaError>,
}

/// 🧺️ Reads `schema_json` in collecting mode — see [`InputSchemaAudit`].
pub fn mutation_input_audit(schema_json: &str, resolver: &dyn InputSchemaResolver) -> InputSchemaAudit {
    let reader = match InputSchemaReader::parse(schema_json, resolver, true) {
        Ok(reader) => reader,
        Err(error) => return InputSchemaAudit { inputs: Vec::new(), findings: vec![error] },
    };
    let read = reader.inputs();
    let mut findings = reader.findings.map(std::cell::RefCell::into_inner).unwrap_or_default();
    let inputs = match read {
        Ok(inputs) => inputs,
        Err(error) => {
            findings.push(error);
            Vec::new()
        }
    };
    let mut distinct: Vec<InputSchemaError> = Vec::with_capacity(findings.len());
    for finding in findings {
        if !distinct.contains(&finding) {
            distinct.push(finding);
        }
    }
    InputSchemaAudit { inputs, findings: distinct }
}

/// 🧭️ The [`InputSchemaResolver`] over every JSON Schema document registered in the OS-wide `schema://` export registry
/// (artifact facets and named scope exports): the document whose `$id` is `id`, or `None` when no plugin published it.
pub fn registered_input_schema_document(id: &str) -> Option<DslValue> {
    let texts: Vec<&'static str> = semio_framework_schema::with_schema_export_registry(|registry| {
        registry.entries().filter(|entry| entry.format == semio_framework_schema::SchemaFormat::JsonSchema).filter_map(|entry| registry.resolve(entry.scope, entry.export, entry.format).ok()).filter(|text| text.contains(id)).collect()
    });
    texts.into_iter().find_map(|text| {
        let document = dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::parse(text).ok()?);
        (document.get("$id").and_then(DslValue::as_str) == Some(id)).then_some(document)
    })
}

/// 🧾️ The instance a leaf payload schema validates for a candidate `payload` (a `Mutation::payload_value`): the payload
/// with every root `const` property the schema declares (the aggregate discriminator `payload_value` strips, e.g.
/// `"mutation": "moveNode"`) spliced in where absent — hand the result to `OwnedJsonSchemaValidator`. TypeScript twin:
/// `mutationInputInstance`.
pub fn mutation_input_instance(schema_json: &str, resolver: &dyn InputSchemaResolver, payload: &DslValue) -> Result<DslValue, InputSchemaError> {
    let reader = InputSchemaReader::parse(schema_json, resolver, false)?;
    let mut root = reader.resolve(None, reader.root.clone(), "")?;
    let mut entries = payload.as_object().map(<[(String, DslValue)]>::to_vec).ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, "", "a mutation payload is an object"))?;
    if root.node.get("properties").is_none() && root.node.get("allOf").is_none() && input_union(&root.node).is_some() {
        let (key, variants) = reader.variants(&root, "")?;
        let chosen = payload.get(&key).and_then(DslValue::as_str).ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, "", format!("a union payload names its variant in {key}")))?;
        root = variants.into_iter().find(|variant| variant.value == chosen).map(|variant| variant.member).ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, "", format!("{chosen} is no variant of this payload union")))?;
    }
    let mut members = Vec::new();
    reader.members(&root, "", 0, &mut members)?;
    for (document, node) in &members {
        for (key, property) in node.get("properties").and_then(DslValue::as_object).unwrap_or_default() {
            if entries.iter().any(|(name, _)| name == key) {
                continue;
            }
            if let Some(value) = reader.resolve(document.clone(), property.clone(), &input_pointer("", key))?.node.get("const") {
                entries.push((key.clone(), value.clone()));
            }
        }
    }
    Ok(DslValue::Object(entries))
}

/// 🔀️ The `oneOf`/`anyOf` branches of a schema node, if it is a union.
fn input_union(node: &DslValue) -> Option<&[DslValue]> {
    node.get("oneOf").or_else(|| node.get("anyOf")).and_then(DslValue::as_array)
}

/// 🔀️ One variant of a discriminated payload union: the `const` its discriminator pins, the resolved member, and that
/// member's resolved discriminator property.
struct InputVariant {
    value: String,
    member: ResolvedInput,
    discriminator: ResolvedInput,
}

const INPUT_UI_KEYS: [&str; 18] = ["widget", "role", "label", "description", "step", "precision", "softMin", "softMax", "snaps", "snapSource", "unit", "displayUnit", "displayFactor", "scale", "group", "order", "options", "ref"];
const INPUT_UI_NUMBER_KEYS: [&str; 9] = ["step", "precision", "softMin", "softMax", "snaps", "snapSource", "displayUnit", "displayFactor", "scale"];

/// 🧭️ The number facets a vector shares across its components (every [`INPUT_UI_NUMBER_KEYS`] but the slider travel keys).
const INPUT_UI_VECTOR_KEYS: [&str; 6] = ["step", "precision", "snaps", "snapSource", "displayUnit", "displayFactor"];
const INPUT_REF_DEPTH: usize = 32;

fn input_error(code: InputSchemaErrorCode, pointer: &str, detail: impl Into<String>) -> InputSchemaError {
    InputSchemaError { code, pointer: pointer.to_string(), detail: detail.into() }
}

fn input_pointer(parent: &str, key: &str) -> String {
    format!("{parent}/{}", key.replace('~', "~0").replace('/', "~1"))
}

/// 🌐️ A `{<locale>: text}` map naming every locale, or a `{<terminology>: {<locale>: text}}` matrix naming every cell.
fn input_localized_text(value: &DslValue) -> Result<LocalizedLabel, InputSchemaErrorCode> {
    let entries = value.as_object().ok_or(InputSchemaErrorCode::UiInvalid)?;
    let cell = |map: &DslValue, locale: Locale| map.get(locale.as_str()).and_then(DslValue::as_str).filter(|text| !text.is_empty()).map(str::to_string);
    if entries.iter().all(|(key, _)| Locale::parse(key).is_some()) {
        if Locale::ALL.iter().any(|locale| cell(value, *locale).is_none()) {
            return Err(InputSchemaErrorCode::LocaleMissing);
        }
        return Ok(LocalizedLabel::from_fn(|_, locale| cell(value, locale).unwrap_or_default()));
    }
    if entries.iter().all(|(key, map)| Terminology::parse(key).is_some() && map.as_object().is_some_and(|cells| cells.iter().all(|(locale, _)| Locale::parse(locale).is_some()))) {
        let complete = Terminology::ALL.iter().all(|terminology| value.get(terminology.as_str()).is_some_and(|map| Locale::ALL.iter().all(|locale| cell(map, *locale).is_some())));
        if !complete {
            return Err(InputSchemaErrorCode::LocaleMissing);
        }
        return Ok(LocalizedLabel::from_fn(|terminology, locale| value.get(terminology.as_str()).and_then(|map| cell(map, locale)).unwrap_or_default()));
    }
    Err(InputSchemaErrorCode::UiInvalid)
}

/// 🔤️ The single non-null JSON type a schema node declares (`"object"` for bare `properties`, `"array"` for bare
/// `items`, `"string"` for an all-string `enum`), or `None` when it names several or none.
fn input_type(node: &DslValue) -> Option<&'static str> {
    const TYPES: [&str; 6] = ["string", "integer", "number", "boolean", "object", "array"];
    let named: Vec<&str> = match node.get("type") {
        Some(DslValue::String(name)) => vec![name.as_str()],
        Some(DslValue::Array(names)) => names.iter().filter_map(DslValue::as_str).filter(|name| *name != "null").collect(),
        _ if node.get("properties").is_some() => vec!["object"],
        _ if node.get("items").is_some() => vec!["array"],
        _ if node.get("enum").and_then(DslValue::as_array).is_some_and(|values| !values.is_empty() && values.iter().all(|value| value.as_str().is_some())) => vec!["string"],
        _ => Vec::new(),
    };
    match named.as_slice() {
        [name] => TYPES.iter().find(|candidate| *candidate == name).copied(),
        _ => None,
    }
}

/// 🎯️ `<kind>Id` / `<kind>_id` (a string) or `<kind>Ids` / `<kind>_ids` (an array) names a reference to `kind`;
/// a leading `new` of a replacement value (`newZoneId`) is not part of the kind.
fn input_inferred_reference_kind(key: &str, many: bool) -> Option<String> {
    let stem = if many { key.strip_suffix("Ids").or_else(|| key.strip_suffix("_ids")) } else { key.strip_suffix("Id").or_else(|| key.strip_suffix("_id")) }?;
    let stem = match stem.strip_prefix("new") {
        Some(rest) if rest.starts_with(|first: char| first.is_ascii_uppercase()) => rest,
        Some("") => return None,
        _ => stem,
    };
    let mut characters = stem.chars();
    let first = characters.next().filter(char::is_ascii_alphabetic)?;
    if !characters.clone().all(|character| character.is_ascii_alphanumeric() || character == '_') {
        return None;
    }
    Some(first.to_ascii_lowercase().to_string() + characters.as_str())
}

/// 🧬️ A schema node with every `$ref` hop and nullable union resolved, the document it lives in (`None` = the leaf
/// itself), and the merged `x-semio-ui` annotation (outer keys win).
struct ResolvedInput {
    document: Option<String>,
    node: DslValue,
    ui: Vec<(String, DslValue)>,
    refs: Vec<String>,
    nullable: bool,
}

impl ResolvedInput {
    fn ui(&self, key: &str) -> Option<&DslValue> {
        self.ui.iter().find(|(name, _)| name == key).map(|(_, value)| value)
    }

    fn ui_str(&self, key: &str) -> Option<&str> {
        self.ui(key).and_then(DslValue::as_str)
    }
}

struct InputSchemaReader<'r> {
    root: DslValue,
    resolver: &'r dyn InputSchemaResolver,
    documents: std::cell::RefCell<BTreeMap<String, DslValue>>,
    active: std::cell::RefCell<Vec<String>>,
    findings: Option<std::cell::RefCell<Vec<InputSchemaError>>>,
}

impl<'r> InputSchemaReader<'r> {
    fn parse(schema_json: &str, resolver: &'r dyn InputSchemaResolver, collect: bool) -> Result<Self, InputSchemaError> {
        let root = dsl::os_pack::json::parse(schema_json).map_err(|error| input_error(InputSchemaErrorCode::Malformed, "", error.to_string()))?;
        Ok(Self {
            root: dsl::os_pack::json::to_dsl_value(&root),
            resolver,
            documents: std::cell::RefCell::new(BTreeMap::new()),
            active: std::cell::RefCell::new(vec!["#".to_string()]),
            findings: collect.then(|| std::cell::RefCell::new(Vec::new())),
        })
    }

    /// 🧺️ In collecting mode records `result`'s error and goes on with `fallback`; the fail-fast mode passes it through.
    fn recover<T>(&self, result: Result<T, InputSchemaError>, fallback: impl FnOnce() -> T) -> Result<T, InputSchemaError> {
        match (result, &self.findings) {
            (Err(error), Some(findings)) => {
                findings.borrow_mut().push(error);
                Ok(fallback())
            }
            (result, _) => result,
        }
    }

    /// 🧬️ Every input of the payload: a discriminated union's selector and variant fields, else the object's fields.
    fn inputs(&self) -> Result<Vec<ActionArgDef>, InputSchemaError> {
        let payload = self.resolve(None, self.root.clone(), "")?;
        if payload.node.get("properties").is_none() && payload.node.get("allOf").is_none() {
            if input_union(&payload.node).is_some() {
                return self.variant_inputs(&payload, "");
            }
            return match input_type(&payload.node) {
                Some("object") | None => Ok(Vec::new()),
                _ => Err(input_error(InputSchemaErrorCode::Malformed, "", "a mutation payload schema describes an object")),
            };
        }
        self.fields(&payload, "")
    }

    /// ♾️ Expands `node` unless a `$ref` it passed through is already being expanded on this path: a recursive schema then
    /// yields a structured value (`ArgSchema::Any`), edited as a whole and validated by the leaf schema, never an endless tree.
    fn guarded(&self, node: &ResolvedInput, expand: impl FnOnce() -> Result<ArgSchema, InputSchemaError>) -> Result<ArgSchema, InputSchemaError> {
        if node.refs.iter().any(|key| self.active.borrow().contains(key)) {
            return Ok(ArgSchema::Any);
        }
        let mark = self.active.borrow().len();
        self.active.borrow_mut().extend(node.refs.iter().cloned());
        let expanded = expand();
        self.active.borrow_mut().truncate(mark);
        expanded
    }

    fn target(&self, document: &Option<String>, reference: &str, pointer: &str) -> Result<(Option<String>, DslValue), InputSchemaError> {
        let unresolved = |detail: String| input_error(InputSchemaErrorCode::RefUnresolved, pointer, detail);
        let (id, fragment) = reference.split_once('#').unwrap_or((reference, ""));
        let document = if id.is_empty() { document.clone() } else { Some(id.to_string()) };
        let fragment_path: Vec<String> = fragment.split('/').skip(1).map(|segment| segment.replace("~1", "/").replace("~0", "~")).collect();
        let walk = |start: &DslValue| -> Option<DslValue> {
            let mut current = start;
            for segment in &fragment_path {
                current = match current {
                    DslValue::Object(_) => current.get(segment)?,
                    DslValue::Array(items) => items.get(segment.parse::<usize>().ok()?)?,
                    _ => return None,
                };
            }
            Some(current.clone())
        };
        let found = match &document {
            None => walk(&self.root),
            Some(id) => {
                if !self.documents.borrow().contains_key(id) {
                    let fetched = self.resolver.resolve(id).ok_or_else(|| unresolved(format!("no schema document has $id {id}")))?;
                    self.documents.borrow_mut().insert(id.clone(), fetched);
                }
                walk(&self.documents.borrow()[id])
            }
        };
        found.map(|node| (document, node)).ok_or_else(|| unresolved(format!("{reference} lands on nothing")))
    }

    fn resolve(&self, document: Option<String>, node: DslValue, pointer: &str) -> Result<ResolvedInput, InputSchemaError> {
        let mut resolved = ResolvedInput { document, node, ui: Vec::new(), refs: Vec::new(), nullable: false };
        for _ in 0..INPUT_REF_DEPTH {
            if let Some(annotation) = resolved.node.get("x-semio-ui") {
                let entries = self.recover(annotation.as_object().ok_or_else(|| input_error(InputSchemaErrorCode::UiInvalid, pointer, "x-semio-ui is an object")), Default::default)?;
                for (key, value) in entries {
                    if !INPUT_UI_KEYS.contains(&key.as_str()) {
                        self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("x-semio-ui carries the undeclared key {key}"))), || ())?;
                        continue;
                    }
                    if !resolved.ui.iter().any(|(name, _)| name == key) {
                        resolved.ui.push((key.clone(), value.clone()));
                    }
                }
            }
            if let Some(reference) = resolved.node.get("$ref").and_then(DslValue::as_str).map(str::to_string) {
                let (document, node) = self.target(&resolved.document, &reference, pointer)?;
                resolved.refs.push(format!("{}#{}", document.as_deref().unwrap_or_default(), reference.split_once('#').map_or("", |(_, fragment)| fragment)));
                resolved.document = document;
                resolved.node = node;
                continue;
            }
            let union = resolved.node.get("oneOf").or_else(|| resolved.node.get("anyOf")).and_then(DslValue::as_array).map(<[DslValue]>::to_vec);
            if let Some(branches) = union {
                let null = |branch: &DslValue| branch.get("type").and_then(DslValue::as_str) == Some("null") && branch.as_object().is_some_and(|entries| entries.len() == 1);
                let concrete: Vec<&DslValue> = branches.iter().filter(|branch| !null(branch)).collect();
                if concrete.len() == 1 && concrete.len() < branches.len() {
                    resolved.node = concrete[0].clone();
                    resolved.nullable = true;
                    continue;
                }
            }
            resolved.nullable |= resolved.node.get("type").and_then(DslValue::as_array).is_some_and(|types| types.iter().any(|name| name.as_str() == Some("null")));
            return Ok(resolved);
        }
        Err(input_error(InputSchemaErrorCode::Malformed, pointer, "a $ref chain exceeds 32 hops"))
    }

    /// 🧩️ The object itself and, depth first, every `allOf` member it composes (each resolved in its own document).
    fn members(&self, object: &ResolvedInput, pointer: &str, depth: usize, members: &mut Vec<(Option<String>, DslValue)>) -> Result<(), InputSchemaError> {
        if depth == INPUT_REF_DEPTH {
            return Err(input_error(InputSchemaErrorCode::Malformed, pointer, "an allOf composition exceeds 32 levels"));
        }
        members.push((object.document.clone(), object.node.clone()));
        for member in object.node.get("allOf").and_then(DslValue::as_array).unwrap_or_default() {
            let resolved = self.resolve(object.document.clone(), member.clone(), pointer)?;
            self.members(&resolved, pointer, depth + 1, members)?;
        }
        Ok(())
    }

    /// 🔀️ A discriminated union's variants: the first property of the first member that every member pins to a
    /// distinct string `const`, and per member that value, the member and its discriminator property.
    fn variants(&self, union: &ResolvedInput, pointer: &str) -> Result<(String, Vec<InputVariant>), InputSchemaError> {
        let mut members = Vec::new();
        for branch in input_union(&union.node).unwrap_or_default() {
            let member = self.resolve(union.document.clone(), branch.clone(), pointer)?;
            let mut composed = Vec::new();
            self.members(&member, pointer, 0, &mut composed)?;
            let mut pins: Vec<(String, Option<String>, ResolvedInput)> = Vec::new();
            for (document, node) in &composed {
                for (key, property) in node.get("properties").and_then(DslValue::as_object).unwrap_or_default() {
                    if pins.iter().any(|(name, _, _)| name == key) {
                        continue;
                    }
                    let resolved = self.resolve(document.clone(), property.clone(), &input_pointer(pointer, key))?;
                    let value = resolved.node.get("const").and_then(DslValue::as_str).map(str::to_string);
                    pins.push((key.clone(), value, resolved));
                }
            }
            members.push((member, pins));
        }
        let pinned = |key: &str| -> Option<Vec<String>> {
            let values: Vec<String> = members.iter().map(|(_, pins)| pins.iter().find(|(name, _, _)| name == key).and_then(|(_, value, _)| value.clone())).collect::<Option<_>>()?;
            values.iter().enumerate().all(|(index, value)| !values[..index].contains(value)).then_some(values)
        };
        let key = members.first().and_then(|(_, pins)| pins.iter().map(|(name, _, _)| name.clone()).find(|name| pinned(name).is_some())).ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, pointer, "a payload union names no property every variant pins to a distinct string const"))?;
        let variants = members
            .into_iter()
            .filter_map(|(member, pins)| pins.into_iter().find(|(name, _, _)| *name == key).and_then(|(_, value, discriminator)| Some(InputVariant { value: value?, member, discriminator })))
            .collect();
        Ok((key, variants))
    }

    /// 🔀️ A discriminated union's inputs: one required variant selector (segmented up to four variants, else a select;
    /// each option labelled by its member's `x-semio-ui.label` or the glossary) followed by every member's fields, each
    /// grouped under its variant's value so an editor shows only the active variant's fields.
    fn variant_inputs(&self, union: &ResolvedInput, pointer: &str) -> Result<Vec<ActionArgDef>, InputSchemaError> {
        let (key, variants) = self.variants(union, pointer)?;
        let selector = input_pointer(pointer, &key);
        let discriminator = &variants.first().ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, pointer, "a payload union has no variant"))?.discriminator;
        let label = match discriminator.ui("label") {
            Some(label) => input_localized_text(label).map_err(|code| input_error(code, &selector, "label names every locale")),
            None => input_label_glossary().get(&key).cloned().ok_or_else(|| input_error(InputSchemaErrorCode::LabelMissing, &selector, format!("{key} has no x-semio-ui.label and no glossary label"))),
        };
        let label = self.recover(label, LocalizedLabel::default)?;
        let description = discriminator.ui("description").map(|description| input_localized_text(description).map_err(|code| input_error(code, &selector, "description names every locale"))).transpose();
        let description = self.recover(description, || None)?;
        let options = variants
            .iter()
            .map(|variant| {
                let label = match variant.member.ui("label") {
                    Some(label) => input_localized_text(label).map_err(|code| input_error(code, &selector, format!("variant {} names every locale", variant.value))),
                    None => input_label_glossary().get(&variant.value).cloned().ok_or_else(|| input_error(InputSchemaErrorCode::OptionLabelMissing, &selector, format!("variant {} has no x-semio-ui.label and no glossary label", variant.value))),
                };
                Ok(ActionArgOption { value: variant.value.clone(), label: self.recover(label, LocalizedLabel::default)? })
            })
            .collect::<Result<Vec<_>, InputSchemaError>>()?;
        let presentation = (options.len() <= 4).then_some(ArgPresentation::Segmented);
        let mut inputs = vec![ActionArgDef { id: input_pointer("", &key), label, schema: ArgSchema::String { options, min_len: None, max_len: None, pattern: None, format: None }, presentation, required: true, nullable: false, default: None, description, group: None, order: None }];
        for variant in &variants {
            for mut field in self.fields(&variant.member, pointer)? {
                field.group = Some(variant.value.clone());
                inputs.push(field);
            }
        }
        Ok(inputs)
    }

    fn fields(&self, object: &ResolvedInput, pointer: &str) -> Result<Vec<ActionArgDef>, InputSchemaError> {
        let mut members = Vec::new();
        self.members(object, pointer, 0, &mut members)?;
        let required: Vec<&str> = members.iter().flat_map(|(_, node)| node.get("required").and_then(DslValue::as_array).unwrap_or_default().iter().filter_map(DslValue::as_str)).collect();
        let mut fields = Vec::new();
        let mut seen = Vec::new();
        for (document, node) in &members {
            for (key, property) in node.get("properties").and_then(DslValue::as_object).unwrap_or_default() {
                if seen.contains(&key.as_str()) {
                    continue;
                }
                seen.push(key.as_str());
                let child_pointer = input_pointer(pointer, key);
                let Some(resolved) = self.recover(self.resolve(document.clone(), property.clone(), &child_pointer).map(Some), || None)? else { continue };
                if let Some(field) = self.recover(self.input(key, &resolved, required.contains(&key.as_str()), &child_pointer), || None)? {
                    fields.push(field);
                }
            }
        }
        Ok(fields)
    }

    fn input(&self, key: &str, input: &ResolvedInput, required: bool, pointer: &str) -> Result<Option<ActionArgDef>, InputSchemaError> {
        let role = match input.ui("role") {
            None => None,
            Some(DslValue::String(role)) if ["value", "target", "discriminator"].contains(&role.as_str()) => Some(role.as_str()),
            Some(_) => self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "role is value, target or discriminator")), || None)?,
        };
        if role == Some("discriminator") || input.node.get("const").is_some() {
            return Ok(None);
        }
        let widget = match input.ui("widget") {
            None => None,
            Some(DslValue::String(widget)) if ["slider", "stepper", "dial", "toggle", "select", "segmented", "text", "multiline", "vector", "color", "reference", "hidden"].contains(&widget.as_str()) => Some(widget.as_str()),
            Some(_) => self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "widget is not a declared widget")), || None)?,
        };
        let label = match input.ui("label") {
            Some(label) => input_localized_text(label).map_err(|code| input_error(code, pointer, "label names every locale")),
            None => input_label_glossary().get(key).cloned().ok_or_else(|| input_error(InputSchemaErrorCode::LabelMissing, pointer, format!("{key} has no x-semio-ui.label and no glossary label"))),
        };
        let label = self.recover(label, LocalizedLabel::default)?;
        let description = input.ui("description").map(|description| input_localized_text(description).map_err(|code| input_error(code, pointer, "description names every locale"))).transpose();
        let description = self.recover(description, || None)?;
        let group = match input.ui("group") {
            None => None,
            Some(DslValue::String(group)) if !group.is_empty() => Some(group.clone()),
            Some(_) => self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "group is a non-empty string")), || None)?,
        };
        let order = match input.ui("order") {
            None => None,
            Some(order) => self.recover(order.as_i64().map(Some).ok_or_else(|| input_error(InputSchemaErrorCode::UiInvalid, pointer, "order is an integer")), || None)?,
        };
        let Some(schema) = self.recover(self.guarded(input, || self.schema(key, input, role, widget, pointer)).map(Some), || None)? else {
            return Ok(Some(ActionArgDef { id: input_pointer("", key), label, schema: ArgSchema::Any, presentation: None, required, nullable: input.nullable, default: None, description, group, order }));
        };
        let compatible = match (widget, &schema) {
            (Some("slider" | "stepper" | "dial"), ArgSchema::Number { .. }) | (Some("toggle"), ArgSchema::Boolean) | (Some("vector"), ArgSchema::Vector { .. }) | (Some("reference"), ArgSchema::Reference { .. }) | (Some("hidden") | None, _) => true,
            (Some("color"), ArgSchema::Vector { dims, min, max, .. }) => matches!(dims, 3 | 4) && *min == Some(0.0) && *max == Some(1.0),
            (Some("select" | "segmented"), ArgSchema::String { options, .. }) => !options.is_empty(),
            (Some("text" | "multiline"), ArgSchema::String { options, .. }) => options.is_empty(),
            _ => false,
        };
        if !compatible {
            self.recover(Err(input_error(InputSchemaErrorCode::WidgetIncompatible, pointer, format!("widget {} cannot edit this value", widget.unwrap_or_default()))), || ())?;
        }
        let presentation = match widget {
            Some("slider") => Some(ArgPresentation::Slider),
            Some("stepper") => Some(ArgPresentation::Stepper),
            Some("dial") => Some(ArgPresentation::Dial),
            Some("segmented") => Some(ArgPresentation::Segmented),
            Some("multiline") => Some(ArgPresentation::Multiline),
            Some("hidden") => Some(ArgPresentation::Hidden),
            Some("color") => Some(ArgPresentation::Color),
            _ => None,
        };
        let default = input.node.get("default").cloned();
        Ok(Some(ActionArgDef { id: input_pointer("", key), label, schema, presentation, required, nullable: input.nullable, default, description, group, order }))
    }

    fn schema(&self, key: &str, input: &ResolvedInput, role: Option<&str>, widget: Option<&str>, pointer: &str) -> Result<ArgSchema, InputSchemaError> {
        let kind = input_type(&input.node);
        if !matches!(kind, Some("integer" | "number")) {
            if let Some((name, _)) = input.ui.iter().find(|(name, _)| INPUT_UI_NUMBER_KEYS.contains(&name.as_str()) && !(kind == Some("array") && INPUT_UI_VECTOR_KEYS.contains(&name.as_str()))) {
                self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{name} only applies to a number"))), || ())?;
            }
        }
        if input.ui("unit").is_some() && !matches!(kind, Some("integer" | "number" | "array")) {
            self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "unit only applies to a number or a vector")), || ())?;
        }
        if widget == Some("hidden") && matches!(kind, Some("object" | "array")) {
            return Ok(ArgSchema::Any);
        }
        let many = kind == Some("array");
        let items = match (many, input.node.get("items")) {
            (true, Some(items)) => self.recover(self.resolve(input.document.clone(), items.clone(), &format!("{pointer}/-")).map(Some), || None)?,
            _ => None,
        };
        let item_kind = items.as_ref().and_then(|items| input_type(&items.node));
        let id_kind = if many { item_kind } else { kind };
        let reference_shaped = id_kind == Some("string");
        let explicit_reference = input.ui("ref").is_some() || widget == Some("reference") || role == Some("target");
        if explicit_reference && !matches!(id_kind, Some("string" | "integer")) {
            return Err(input_error(InputSchemaErrorCode::WidgetIncompatible, pointer, "a reference is a string or integer id or an array of them"));
        }
        let inferred_kind = (role.is_none() && widget.is_none() && reference_shaped && input.node.get("enum").is_none()).then(|| input_inferred_reference_kind(key, many)).flatten();
        if explicit_reference || inferred_kind.is_some() {
            if let Some((name, _)) = input.ui.iter().find(|(name, _)| name == "unit" || INPUT_UI_NUMBER_KEYS.contains(&name.as_str())) {
                self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{name} does not apply to a reference"))), || ())?;
            }
            let id_type = if id_kind == Some("integer") { ReferenceIdType::Integer } else { ReferenceIdType::String };
            return self.reference(key, input, many, inferred_kind, id_type, pointer);
        }
        if input.ui("options").is_some() && input.node.get("enum").is_none() {
            self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "options only label enum values")), || ())?;
        }
        match kind {
            Some("string") => Ok(ArgSchema::String {
                options: input_options(input, pointer)?,
                min_len: input.node.get("minLength").and_then(DslValue::as_u64).map(|length| length as u32),
                max_len: input.node.get("maxLength").and_then(DslValue::as_u64).map(|length| length as u32),
                pattern: input.node.get("pattern").and_then(DslValue::as_str).map(str::to_string),
                format: (input.node.get("format").and_then(DslValue::as_str) == Some("uri")).then_some(ArgFormat::Uri),
            }),
            Some(number @ ("integer" | "number")) => input_number(input, number == "integer", pointer),
            Some("boolean") => Ok(ArgSchema::Boolean),
            Some("object") if input.node.get("properties").is_some() || input.node.get("allOf").is_some() => Ok(ArgSchema::Object { fields: self.fields(input, pointer)? }),
            Some("array") => {
                let bound = |name: &str| input.node.get(name).and_then(DslValue::as_u64).map(|count| count as u32);
                let (min_items, max_items) = (bound("minItems"), bound("maxItems"));
                let numeric = matches!(item_kind, Some("integer" | "number"));
                let fixed = min_items.filter(|count| Some(*count) == max_items && (2..=4).contains(count));
                if matches!(widget, Some("vector" | "color")) || (numeric && fixed.is_some()) {
                    let (Some(dims), Some(items)) = (fixed.filter(|_| numeric), items.as_ref()) else {
                        return Err(input_error(InputSchemaErrorCode::WidgetIncompatible, pointer, "a vector is an array of 2 to 4 numbers of fixed length"));
                    };
                    let component = ResolvedInput { document: items.document.clone(), node: items.node.clone(), ui: input.ui.clone(), refs: Vec::new(), nullable: items.nullable };
                    let ArgSchema::Number { min, min_exclusive, max, max_exclusive, snaps, snap_source, precision, display_unit, display_factor, .. } = input_number(&component, item_kind == Some("integer"), pointer)? else {
                        unreachable!("input_number reads a number");
                    };
                    return Ok(ArgSchema::Vector {
                        dims,
                        min: min.filter(|_| !min_exclusive),
                        max: max.filter(|_| !max_exclusive),
                        unit: input_string(input, "unit", pointer)?,
                        step: input_positive(input, "step", pointer)?,
                        snaps,
                        snap_source,
                        precision,
                        display_unit,
                        display_factor,
                    });
                }
                if let Some((name, _)) = input.ui.iter().find(|(name, _)| INPUT_UI_VECTOR_KEYS.contains(&name.as_str())).filter(|_| !numeric) {
                    self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{name} only applies to a number or a vector"))), || ())?;
                }
                let items = match &items {
                    Some(items) if numeric => {
                        let mut ui = items.ui.clone();
                        ui.extend(input.ui.iter().filter(|(name, _)| (name == "unit" || INPUT_UI_VECTOR_KEYS.contains(&name.as_str())) && !items.ui.iter().any(|(own, _)| own == name)).cloned());
                        self.item_schema(&ResolvedInput { document: items.document.clone(), node: items.node.clone(), ui, refs: items.refs.clone(), nullable: items.nullable }, &format!("{pointer}/-"))?
                    }
                    Some(items) => self.item_schema(items, &format!("{pointer}/-"))?,
                    None => ArgSchema::Any,
                };
                Ok(ArgSchema::Array { items: Box::new(items), min_items, max_items })
            }
            _ => Ok(ArgSchema::Any),
        }
    }

    fn item_schema(&self, items: &ResolvedInput, pointer: &str) -> Result<ArgSchema, InputSchemaError> {
        self.guarded(items, || self.item_schema_expanded(items, pointer))
    }

    fn item_schema_expanded(&self, items: &ResolvedInput, pointer: &str) -> Result<ArgSchema, InputSchemaError> {
        if items.node.get("const").is_some() {
            return Ok(ArgSchema::Any);
        }
        match input_type(&items.node) {
            Some("string") => Ok(ArgSchema::String { options: input_options(items, pointer)?, min_len: None, max_len: None, pattern: None, format: None }),
            Some(number @ ("integer" | "number")) => input_number(items, number == "integer", pointer),
            Some("boolean") => Ok(ArgSchema::Boolean),
            Some("object") if items.node.get("properties").is_some() || items.node.get("allOf").is_some() => Ok(ArgSchema::Object { fields: self.fields(items, pointer)? }),
            Some("array") => {
                let inner = match items.node.get("items") {
                    Some(inner) => self.item_schema(&self.resolve(items.document.clone(), inner.clone(), &format!("{pointer}/-"))?, &format!("{pointer}/-"))?,
                    None => ArgSchema::Any,
                };
                let bound = |name: &str| items.node.get(name).and_then(DslValue::as_u64).map(|count| count as u32);
                Ok(ArgSchema::Array { items: Box::new(inner), min_items: bound("minItems"), max_items: bound("maxItems") })
            }
            _ => Ok(ArgSchema::Any),
        }
    }

    fn reference(&self, key: &str, input: &ResolvedInput, many: bool, inferred_kind: Option<String>, id_type: ReferenceIdType, pointer: &str) -> Result<ArgSchema, InputSchemaError> {
        let invalid = |detail: &str| input_error(InputSchemaErrorCode::UiInvalid, pointer, detail);
        let reference = input.ui("ref");
        if reference.is_some_and(|reference| reference.as_object().is_none_or(|entries| entries.iter().any(|(name, _)| !["kind", "domain", "granularity"].contains(&name.as_str())))) {
            return Err(invalid("ref is {kind, domain?, granularity?}"));
        }
        let kinds = match reference.and_then(|reference| reference.get("kind")) {
            Some(DslValue::String(kind)) if !kind.is_empty() => vec![kind.clone()],
            Some(DslValue::Array(kinds)) if !kinds.is_empty() && kinds.iter().all(|kind| kind.as_str().is_some_and(|kind| !kind.is_empty())) => kinds.iter().filter_map(DslValue::as_str).map(str::to_string).collect(),
            Some(_) => return Err(invalid("ref.kind is a non-empty string or a non-empty array of them")),
            None => vec![inferred_kind.or_else(|| input_inferred_reference_kind(key, many)).ok_or_else(|| invalid("a target names its ref.kind"))?],
        };
        let text = |name: &str| -> Result<Option<String>, InputSchemaError> {
            match reference.and_then(|reference| reference.get(name)) {
                None => Ok(None),
                Some(DslValue::String(value)) if !value.is_empty() => Ok(Some(value.clone())),
                Some(_) => Err(invalid("ref.domain and ref.granularity are non-empty strings")),
            }
        };
        let bound = |name: &str| input.node.get(name).and_then(DslValue::as_u64).map(|count| count as u32);
        Ok(ArgSchema::Reference { kinds, domain: text("domain")?, granularity: text("granularity")?, many, min_items: many.then(|| bound("minItems")).flatten(), max_items: many.then(|| bound("maxItems")).flatten(), id_type })
    }
}

fn input_string(input: &ResolvedInput, key: &str, pointer: &str) -> Result<Option<String>, InputSchemaError> {
    match input.ui(key) {
        None => Ok(None),
        Some(DslValue::String(value)) if !value.is_empty() => Ok(Some(value.clone())),
        Some(_) => Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{key} is a non-empty string"))),
    }
}

fn input_finite(input: &ResolvedInput, key: &str, pointer: &str) -> Result<Option<f64>, InputSchemaError> {
    match input.ui(key) {
        None => Ok(None),
        Some(value) => value.as_f64().filter(|value| value.is_finite()).map(Some).ok_or_else(|| input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{key} is a finite number"))),
    }
}

fn input_positive(input: &ResolvedInput, key: &str, pointer: &str) -> Result<Option<f64>, InputSchemaError> {
    match input_finite(input, key, pointer)? {
        Some(value) if value <= 0.0 => Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{key} is positive"))),
        value => Ok(value),
    }
}

fn input_options(input: &ResolvedInput, pointer: &str) -> Result<Vec<ActionArgOption>, InputSchemaError> {
    let Some(values) = input.node.get("enum").and_then(DslValue::as_array) else { return Ok(Vec::new()) };
    let labels = input.ui("options");
    if labels.is_some_and(|labels| labels.as_object().is_none_or(|entries| entries.iter().any(|(value, _)| !values.iter().any(|candidate| candidate.as_str() == Some(value.as_str()))))) {
        return Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "options labels only declared enum values"));
    }
    values
        .iter()
        .map(|value| {
            let value = value.as_str().ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, pointer, "a labelled enum lists strings"))?;
            let label = match labels.and_then(|labels| labels.get(value)) {
                Some(label) => input_localized_text(label).map_err(|code| input_error(code, pointer, format!("option {value} names every locale")))?,
                None => input_label_glossary().get(value).cloned().ok_or_else(|| input_error(InputSchemaErrorCode::OptionLabelMissing, pointer, format!("option {value} has no label")))?,
            };
            Ok(ActionArgOption { value: value.to_string(), label })
        })
        .collect()
}

fn input_number(input: &ResolvedInput, integer: bool, pointer: &str) -> Result<ArgSchema, InputSchemaError> {
    let invalid = |detail: String| input_error(InputSchemaErrorCode::UiInvalid, pointer, detail);
    let keyword = |name: &str| input.node.get(name).and_then(DslValue::as_f64);
    let bound = |inclusive: Option<f64>, exclusive: Option<f64>, lower: bool| -> (Option<f64>, bool) {
        match (inclusive, exclusive) {
            (inclusive, None) => (inclusive, false),
            (Some(inclusive), Some(exclusive)) if (lower && inclusive > exclusive) || (!lower && inclusive < exclusive) => (Some(inclusive), false),
            (_, Some(exclusive)) if integer => (Some(if lower { exclusive.floor() + 1.0 } else { exclusive.ceil() - 1.0 }), false),
            (_, Some(exclusive)) => (Some(exclusive), true),
        }
    };
    let (min, min_exclusive) = bound(keyword("minimum"), keyword("exclusiveMinimum"), true);
    let (max, max_exclusive) = bound(keyword("maximum"), keyword("exclusiveMaximum"), false);
    let inside = |value: f64| min.is_none_or(|min| if min_exclusive { value > min } else { value >= min }) && max.is_none_or(|max| if max_exclusive { value < max } else { value <= max });
    let snaps = match input.ui("snaps") {
        None => Vec::new(),
        Some(DslValue::Array(values)) => values.iter().map(|value| value.as_f64().filter(|value| value.is_finite())).collect::<Option<Vec<f64>>>().ok_or_else(|| invalid("snaps are finite numbers".to_string()))?,
        Some(_) => return Err(invalid("snaps is an array of numbers".to_string())),
    };
    if let Some(snap) = snaps.iter().find(|snap| !inside(**snap)) {
        return Err(invalid(format!("snap {snap} lies outside the hard bounds")));
    }
    let snap_source = match input.ui("snapSource") {
        None => None,
        Some(source) => Some(match source.as_object() {
            Some([(name, DslValue::Bool(true))]) if name == "step" => SnapSource::Step,
            Some([(name, DslValue::String(key))]) if name == "config" && !key.is_empty() => SnapSource::Config { key: key.clone() },
            Some([(name, DslValue::String(target))]) if name == "snapshot" && (target.is_empty() || target.starts_with('/')) => SnapSource::Snapshot { pointer: target.clone() },
            _ => return Err(invalid("snapSource is {step: true}, {config: key} or {snapshot: pointer}".to_string())),
        }),
    };
    let soft_min = input_finite(input, "softMin", pointer)?;
    let soft_max = input_finite(input, "softMax", pointer)?;
    if soft_min.is_some_and(|soft| !inside(soft)) || soft_max.is_some_and(|soft| !inside(soft)) || soft_min.zip(soft_max).is_some_and(|(low, high)| low >= high) {
        return Err(invalid("softMin < softMax lie inside the hard bounds".to_string()));
    }
    let scale = match input.ui_str("scale") {
        None if input.ui("scale").is_none() => None,
        Some("linear") => Some(NumberScale::Linear),
        Some("log") => Some(NumberScale::Log),
        _ => return Err(invalid("scale is linear or log".to_string())),
    };
    if scale == Some(NumberScale::Log) && !soft_min.or(min).is_some_and(|low| low > 0.0) {
        return Err(invalid("a log scale starts at a positive softMin or minimum".to_string()));
    }
    let precision = match input.ui("precision") {
        None => None,
        Some(precision) => Some(precision.as_u64().filter(|digits| *digits <= 15).ok_or_else(|| invalid("precision is an integer 0..=15".to_string()))? as u32),
    };
    let display_factor = input_finite(input, "displayFactor", pointer)?;
    if display_factor == Some(0.0) {
        return Err(invalid("displayFactor is non-zero".to_string()));
    }
    Ok(ArgSchema::Number {
        min,
        min_exclusive,
        max,
        max_exclusive,
        step: input_positive(input, "step", pointer)?.or(integer.then_some(1.0)),
        integer,
        unit: input_string(input, "unit", pointer)?,
        snaps,
        snap_source,
        soft_min,
        soft_max,
        precision,
        display_unit: input_string(input, "displayUnit", pointer)?,
        display_factor,
        scale,
    })
}

#[cfg(test)]
#[path = "🧪️tests/🧪️mutation-inputs/🦀️.rs"]
mod mutation_inputs_tests;
//#endregion 🔖️MutationInputs

/// 🎯️ Neutral icon for an action or command whose owner does not declare presentation metadata.
pub fn default_action_icon_id(kind: ActionKind) -> IconName {
    match kind {
        ActionKind::View => "eye".into(),
        ActionKind::Shell => "code".into(),
        ActionKind::Mutation => "sparkles".into(),
        ActionKind::History => "clock".into(),
        ActionKind::Clipboard => "clipboard".into(),
        ActionKind::Interaction => "mouse-pointer".into(),
    }
}

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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
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

/// 🖐️ The verbs the framework's World3d gumball dispatches ITSELF around one drag —
/// `transformBegin` when a handle is grabbed and `transformEnd` when it is released
/// (`World3dHost`'s `handleGumballDragStart`/`handleGumballDragEnd`). They bracket the pose deltas
/// (`translateSelection`/`rotateSelection`/`scaleSelection`, each a self-contained verb that commits on
/// its own outside a bracket) and mean nothing outside a live pointer gesture.
pub const GUMBALL_GESTURE_BRACKET_ACTION_IDS: [&str; 2] = ["transformBegin", "transformEnd"];

/// 🎛️ The gumball's handle toggle — which move/rotate/scale handles one window shows.
pub const GUMBALL_CHROME_ACTION_IDS: [&str; 1] = ["setTransformGumballFlag"];

/// 🧭️ The one classification rule for the framework-owned gumball verbs, stated once for every
/// app that uses the gumball (lowpoly, puzzle, fem, block, …): the drag brackets are
/// [`CapabilityAudience::Input`], the handle toggle is [`CapabilityAudience::Chrome`]. `None` for every
/// other id. An app declaring one of these ids must resolve to exactly this audience — enforced when its
/// definition is built, held against `🧫️fixtures/🖐️gumball-verb-audience.json`.
pub fn framework_fixed_audience(action_id: &str) -> Option<CapabilityAudience> {
    if GUMBALL_GESTURE_BRACKET_ACTION_IDS.contains(&action_id) {
        Some(CapabilityAudience::Input)
    } else if GUMBALL_CHROME_ACTION_IDS.contains(&action_id) {
        Some(CapabilityAudience::Chrome)
    } else {
        None
    }
}

/// ⚖️ The audience a framework-owned verb requires when `action` resolves to a different one.
pub fn framework_fixed_audience_violation(action: &ActionDefinition) -> Option<CapabilityAudience> {
    framework_fixed_audience(&action.id).filter(|expected| *expected != resolve_audience(action))
}

/// 🧵️ Phase-8 migration disposition for every action and command declaration. The
/// default is deliberately not executable: manifests decoded without an explicit disposition remain
/// visible to audit tooling but are rejected by [`validate_interactive_job_classification`] before a
/// release catalog can be activated.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
            .describe(LocalizedLabel::native("Branches the artifact's history into a new named alternative that can be edited without disturbing the current one.", "Verzweigt die Historie des Artefakts in eine neue benannte Alternative, die unabhängig bearbeitet werden kann."))
            .use_when(["try a variant", "branch this design"]),
        ActionDefinition::resumable_framework(SWITCH_ALTERNATIVE_ACTION_ID, LocalizedLabel::native("Switch Alternative", "Alternative wechseln"), ActionKind::History, "git-branch")
            .describe(LocalizedLabel::native(
                "Moves the artifact head onto another existing alternative: the artifact shows that alternative's history, including the history edits kept as that alternative, and leaves those of the alternative it left.",
                "Setzt den Kopf des Artefakts auf eine andere vorhandene Alternative: Das Artefakt zeigt deren Verlauf einschließlich der als diese Alternative behaltenen Verlaufsbearbeitungen und lässt die der verlassenen Alternative weg.",
            ))
            .use_when(["switch to the other variant", "go back to the original history", "compare the edited history with the original"])
            .with_args([ActionArgDef::text(SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID, LocalizedLabel::native("Alternative", "Alternative"))
                .describe(LocalizedLabel::native("Id of an existing alternative of this artifact, as the history lists it.", "Kennung einer vorhandenen Alternative dieses Artefakts, wie der Verlauf sie auflistet."))
                .required()]),
        ActionDefinition::resumable_framework("checkoutCheckpoint", LocalizedLabel::native("Checkout Checkpoint", "Checkpoint auschecken"), ActionKind::History, "git-branch")
            .describe(LocalizedLabel::native("Restores the artifact to a previously committed checkpoint by id.", "Stellt das Artefakt auf einen zuvor festgeschriebenen Checkpoint zurück."))
            .use_when(["go back to that checkpoint"]),
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(REVERT_TO_COMMAND_ACTION_ID, LocalizedLabel::native("Revert to Command", "Auf Befehl zurücksetzen"), ActionKind::History, "clock") }
            .describe(LocalizedLabel::native("Rewinds the artifact to the state it had just after one numbered entry of the session command log.", "Setzt das Artefakt auf den Zustand direkt nach einem nummerierten Eintrag des Sitzungsprotokolls zurück."))
            .with_args([ActionArgDef::number("entrySeq", LocalizedLabel::native("Entry", "Eintrag")).required()]),
    ]
}

//#region 🔖️HistoryEdit
/// ✏️ Opens (or retargets) the history-edit session on one applied mutation: `mutationId`.
pub const HISTORY_EDIT_BEGIN_ACTION_ID: &str = "historyEditBegin";
/// 🎚️ Sets one input of the edited mutation's draft: `path` (RFC 6901 pointer into the payload) and `value`.
pub const HISTORY_EDIT_INPUT_ACTION_ID: &str = "historyEditInput";
/// 🎯️ Sets the reference input at `path` from the current selection of its declared domain.
pub const HISTORY_EDIT_USE_SELECTION_ACTION_ID: &str = "historyEditUseSelection";
/// 🚫️ Drafts the edited mutation as withdrawn: it folds as a no-op.
pub const HISTORY_EDIT_WITHDRAW_ACTION_ID: &str = "historyEditWithdraw";
/// ✅️ Accepts the draft and replays everything downstream.
pub const HISTORY_EDIT_ACCEPT_ACTION_ID: &str = "historyEditAccept";
/// ↩️ Drops the draft and returns to the previous stage.
pub const HISTORY_EDIT_DISCARD_ACTION_ID: &str = "historyEditDiscard";
/// 🏁️ Opens the finalize prompt once the replayed history is clean.
pub const HISTORY_EDIT_FINALIZE_ACTION_ID: &str = "historyEditFinalize";
/// 🌿️ Commits the accepted drafts: as a new alternative named `name`, or with `choice: overwrite` over every alternative.
pub const HISTORY_EDIT_COMMIT_ACTION_ID: &str = "historyEditCommit";
/// ⬅️ Leaves the finalize prompt and returns to reviewing.
pub const HISTORY_EDIT_BACK_ACTION_ID: &str = "historyEditBack";
/// 🚪️ Leaves history editing and discards every draft.
pub const HISTORY_EDIT_EXIT_ACTION_ID: &str = "historyEditExit";
/// ⏹️ Cancels the running downstream replay.
pub const HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID: &str = "historyEditCancelReplay";
/// 🔁️ Replays the accepted drafts again after a cancelled or faulted replay.
pub const HISTORY_EDIT_RERUN_ACTION_ID: &str = "historyEditRerun";
/// 🗂️ Every reserved history-edit verb, in lifecycle order. They are host-driven: applied to the session at the head of
/// the dispatch, never queued behind guest work, never recorded as history rows.
pub const HISTORY_EDIT_ACTION_IDS: [&str; 12] = [
    HISTORY_EDIT_BEGIN_ACTION_ID,
    HISTORY_EDIT_INPUT_ACTION_ID,
    HISTORY_EDIT_USE_SELECTION_ACTION_ID,
    HISTORY_EDIT_WITHDRAW_ACTION_ID,
    HISTORY_EDIT_ACCEPT_ACTION_ID,
    HISTORY_EDIT_DISCARD_ACTION_ID,
    HISTORY_EDIT_FINALIZE_ACTION_ID,
    HISTORY_EDIT_COMMIT_ACTION_ID,
    HISTORY_EDIT_BACK_ACTION_ID,
    HISTORY_EDIT_EXIT_ACTION_ID,
    HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID,
    HISTORY_EDIT_RERUN_ACTION_ID,
];
/// 🪪️ `historyEditBegin`'s mutation id argument (`<editId>#<opIndex>`).
pub const HISTORY_EDIT_ARG_MUTATION_ID: &str = "mutationId";
/// 🧩️ `historyEditBegin`'s member store argument (`<slot>/<childId>`): the composed member store that holds the mutation
/// (design §12); absent for the document's own store.
pub const HISTORY_EDIT_ARG_STORE: &str = "store";
/// 🧭️ The RFC 6901 pointer of one input inside the edited mutation's payload.
pub const HISTORY_EDIT_ARG_PATH: &str = "path";
/// 🎚️ `historyEditInput`'s new input value.
pub const HISTORY_EDIT_ARG_VALUE: &str = "value";
/// 🧿️ The session generation a verb addresses; absent addresses the live session.
pub const HISTORY_EDIT_ARG_GENERATION: &str = "generation";
/// 🏷️ `historyEditCommit`'s new alternative name.
pub const HISTORY_EDIT_ARG_NAME: &str = "name";
/// ✍️ The `historyEditCommit` choice (the finalize dialog's [`DIALOG_CHOICE_ARG`]) that overwrites every alternative.
pub const HISTORY_EDIT_CHOICE_OVERWRITE: &str = "overwrite";
/// 🗳️ The framework-injected finalize dialog of a history edit.
pub const HISTORY_EDIT_FINALIZE_DIALOG_ID: &str = "finalizeHistoryEdit";

/// ✏️ The twelve reserved history-edit verbs, part of [`history_action_definitions`]: never in the palette, no chords
/// (the hosts bind remappable `ui.timeTravel.*` chords), rejected on a viewer, agent-addressable like undo.
pub fn history_edit_action_definitions() -> Vec<ActionDefinition> {
    let generation = || history_edit_hidden_arg(ActionArgDef::with_schema(HISTORY_EDIT_ARG_GENERATION, LocalizedLabel::native("Generation", "Generation"), ArgSchema::number(Some(0.0), None, Some(1.0), true)));
    let path = || ActionArgDef::text(HISTORY_EDIT_ARG_PATH, LocalizedLabel::native("Input", "Eingabe")).describe(LocalizedLabel::native("RFC 6901 pointer of the input inside the mutation payload, such as /dx.", "RFC-6901-Zeiger der Eingabe in den Nutzdaten der Mutation, etwa /dx.")).required();
    let verb = |id: &str, en: &str, de: &str, icon: &str| ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(id, LocalizedLabel::native(en, de), ActionKind::History, icon) };
    vec![
        verb(HISTORY_EDIT_BEGIN_ACTION_ID, "Edit Mutation", "Mutation bearbeiten", "pencil")
            .describe(LocalizedLabel::native(
                "Opens history editing on one applied mutation: the artifact shows the state right before it with its inputs editable, and nothing downstream is applied until the draft is accepted.",
                "Öffnet die Verlaufsbearbeitung für eine angewendete Mutation: Das Artefakt zeigt den Zustand direkt davor mit bearbeitbaren Eingaben, und nichts Späteres wird angewendet, bis der Entwurf übernommen ist.",
            ))
            .use_when(["edit an earlier step", "change a past operation", "fix a mutation in the history"])
            .with_args([
                history_edit_hidden_arg(ActionArgDef::text(HISTORY_EDIT_ARG_MUTATION_ID, LocalizedLabel::native("Mutation", "Mutation")).required()),
                history_edit_hidden_arg(ActionArgDef::text(HISTORY_EDIT_ARG_STORE, LocalizedLabel::native("Member store", "Mitgliedsspeicher"))),
            ]),
        verb(HISTORY_EDIT_INPUT_ACTION_ID, "Set Mutation Input", "Mutationseingabe setzen", "sliders-horizontal")
            .describe(LocalizedLabel::native(
                "Sets one input of the mutation being edited; the value is validated against the mutation's input schema and previewed immediately.",
                "Setzt eine Eingabe der bearbeiteten Mutation; der Wert wird gegen das Eingabeschema der Mutation geprüft und sofort in der Vorschau gezeigt.",
            ))
            .use_when(["change this value in the edited step", "set the input of the mutation"])
            .with_args([path(), ActionArgDef::any(HISTORY_EDIT_ARG_VALUE, LocalizedLabel::native("Value", "Wert")).required(), generation()]),
        verb(HISTORY_EDIT_USE_SELECTION_ACTION_ID, "Use Selection", "Auswahl verwenden", "mouse-pointer")
            .describe(LocalizedLabel::native(
                "Sets a reference input of the mutation being edited to the entities currently selected in its domain.",
                "Setzt eine Referenzeingabe der bearbeiteten Mutation auf die aktuell ausgewählten Elemente ihres Bereichs.",
            ))
            .use_when(["use the current selection as targets"])
            .with_args([path(), generation()]),
        verb(HISTORY_EDIT_WITHDRAW_ACTION_ID, "Withdraw Mutation", "Mutation zurückziehen", "eye-off")
            .describe(LocalizedLabel::native(
                "Drafts the mutation being edited as withdrawn, so it no longer changes the artifact; use it for a step whose inputs cannot fix its error.",
                "Entwirft die bearbeitete Mutation als zurückgezogen, sodass sie das Artefakt nicht mehr verändert; für einen Schritt, dessen Fehler sich über die Eingaben nicht beheben lässt.",
            ))
            .use_when(["drop this step", "skip this operation"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_ACCEPT_ACTION_ID, "Accept Draft", "Entwurf übernehmen", "check")
            .describe(LocalizedLabel::native(
                "Accepts the draft of the mutation being edited and replays every later mutation, reporting success, warnings and errors per mutation.",
                "Übernimmt den Entwurf der bearbeiteten Mutation und wendet alle späteren Mutationen neu an; Erfolg, Warnungen und Fehler werden je Mutation gemeldet.",
            ))
            .use_when(["apply the edit", "accept the change to the history"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_DISCARD_ACTION_ID, "Discard Draft", "Entwurf verwerfen", "x")
            .describe(LocalizedLabel::native("Drops the draft of the mutation being edited; accepted drafts stay.", "Verwirft den Entwurf der bearbeiteten Mutation; übernommene Entwürfe bleiben erhalten."))
            .use_when(["discard this draft"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_FINALIZE_ACTION_ID, "Finalize History Edit", "Verlaufsbearbeitung abschließen", "list-checks")
            .describe(LocalizedLabel::native(
                "Asks how to keep the edited history once no replayed mutation has an error: as a new alternative or by overwriting the existing history.",
                "Fragt, wie der bearbeitete Verlauf behalten wird, sobald keine neu angewendete Mutation einen Fehler hat: als neue Alternative oder durch Überschreiben des bestehenden Verlaufs.",
            ))
            .use_when(["finish editing the history", "keep the edited history"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_COMMIT_ACTION_ID, "Commit History Edit", "Verlaufsbearbeitung festschreiben", "git-commit")
            .destructive()
            .describe(LocalizedLabel::native(
                "Commits the accepted drafts: as a new alternative named by name, keeping the original history, or with choice overwrite into every alternative that contains the edited mutations.",
                "Schreibt die übernommenen Entwürfe fest: als neue Alternative mit dem Namen name, wobei der ursprüngliche Verlauf bleibt, oder mit choice overwrite in jede Alternative, die die bearbeiteten Mutationen enthält.",
            ))
            .use_when(["save the edited history as a new alternative", "overwrite the history with the edit"])
            .with_args([
                ActionArgDef::select(DIALOG_CHOICE_ARG, LocalizedLabel::native("Choice", "Auswahl"), vec![ActionArgOption::new(HISTORY_EDIT_CHOICE_OVERWRITE, LocalizedLabel::native("Overwrite", "Überschreiben"))]),
                ActionArgDef::text(HISTORY_EDIT_ARG_NAME, LocalizedLabel::native("Alternative name", "Name der Alternative")),
                generation(),
            ]),
        verb(HISTORY_EDIT_BACK_ACTION_ID, "Back to Review", "Zurück zur Prüfung", "arrow-left")
            .describe(LocalizedLabel::native("Closes the finalize prompt and returns to reviewing the edited history.", "Schließt die Abschlussabfrage und kehrt zur Prüfung des bearbeiteten Verlaufs zurück."))
            .use_when(["go back to reviewing"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_EXIT_ACTION_ID, "Exit History Editing", "Verlaufsbearbeitung beenden", "rotate-ccw")
            .describe(LocalizedLabel::native("Leaves history editing and discards every draft; the artifact is unchanged.", "Beendet die Verlaufsbearbeitung und verwirft alle Entwürfe; das Artefakt bleibt unverändert."))
            .use_when(["stop editing the history", "cancel the history edit"]),
        verb(HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, "Cancel Replay", "Neuanwendung abbrechen", "square")
            .describe(LocalizedLabel::native(
                "Cancels the running replay of later mutations; the drafts stay and the replay can be started again by accepting.",
                "Bricht die laufende Neuanwendung späterer Mutationen ab; die Entwürfe bleiben, und die Neuanwendung kann durch Übernehmen erneut gestartet werden.",
            ))
            .use_when(["stop the replay"])
            .with_args([generation()]),
        verb(HISTORY_EDIT_RERUN_ACTION_ID, "Replay Again", "Erneut anwenden", "skip-forward")
            .describe(LocalizedLabel::native(
                "Replays every later mutation again with the accepted drafts after a cancelled or failed replay, so the history can be reviewed and finalized.",
                "Wendet nach einer abgebrochenen oder fehlgeschlagenen Neuanwendung alle späteren Mutationen mit den übernommenen Entwürfen erneut an, damit der Verlauf geprüft und abgeschlossen werden kann.",
            ))
            .use_when(["replay the edited history again", "retry the replay"])
            .with_args([generation()]),
    ]
}

/// 🫥️ A history-edit argument the chrome fills from the session, never shown in a staged form.
fn history_edit_hidden_arg(arg: ActionArgDef) -> ActionArgDef {
    ActionArgDef { presentation: Some(ArgPresentation::Hidden), ..arg }
}

/// 🗳️ The framework-injected finalize prompt of a history edit (every non-viewer app declares it): the submit keeps
/// the edit as a new alternative named by the staged `name` (the opener seeds a localized default), the destructive
/// choice `overwrite` replaces the inputs in every alternative, and dismissing it goes back to reviewing.
pub fn history_edit_finalize_dialog() -> DialogDefinition {
    DialogDefinition::new(HISTORY_EDIT_FINALIZE_DIALOG_ID, LocalizedLabel::native("Finish editing history", "Verlaufsbearbeitung abschließen"), ActionRef::new(HISTORY_EDIT_COMMIT_ACTION_ID))
        .body(LocalizedLabel::native("Keep the edit as a new alternative or overwrite the existing history.", "Die Bearbeitung als neue Alternative behalten oder den bestehenden Verlauf überschreiben."))
        .args(vec![ActionArgDef::text(HISTORY_EDIT_ARG_NAME, LocalizedLabel::native("Alternative name", "Name der Alternative")).required()])
        .submit_label(LocalizedLabel::native("New alternative", "Neue Alternative"))
        .cancel_label(LocalizedLabel::native("Back", "Zurück"))
        .on_cancel(ActionRef::new(HISTORY_EDIT_BACK_ACTION_ID))
        .choice(
            DialogChoice::new(HISTORY_EDIT_CHOICE_OVERWRITE, LocalizedLabel::native("Overwrite", "Überschreiben"), ActionRef::new(HISTORY_EDIT_COMMIT_ACTION_ID))
                .description(LocalizedLabel::native("Replaces the edited mutations in every alternative that contains them.", "Ersetzt die bearbeiteten Mutationen in jeder Alternative, die sie enthält."))
                .tone(semio_framework_ui_contract::Tone::Danger)
                .destructive(),
        )
}

#[cfg(test)]
#[path = "🧪️tests/🧪️history-edit-actions/🦀️.rs"]
mod history_edit_actions_tests;
//#endregion 🔖️HistoryEdit

/// 🎚️ The framework-owned action id apps dispatch to change the history panel's operations
/// filter — auto-injected unconditionally (mirrors `RECORD_TUTORIAL_ACTION_ID`).
pub const SET_HISTORY_COMMAND_FILTER_ACTION_ID: &str = "setHistoryCommandFilter";

/// 🎚️ The framework-injected `setHistoryCommandFilter` View action (never in the palette):
/// switches the history panel's tri-state operations filter. Ephemeral UI state, never an
/// operation — `ActionKind::View`. Arg id is `"value"` (not `"filter"`) — a top-level `UiNode::Select`
/// always dispatches its picked option merged into `args` under the `"value"` key (both renderers'
/// `Select` interpreters hardcode that key; see `with_item_value_arg` in ui_wgpu).
pub fn set_history_command_filter_action_definition() -> ActionDefinition {
    let options = vec![
        ActionArgOption::new("all", LocalizedLabel::native("All", "Alle")),
        ActionArgOption::new("withoutOperations", LocalizedLabel::native("Without Operations", "Ohne Operationen")),
        ActionArgOption::new("onlyOperations", LocalizedLabel::native("Only Operations", "Nur Operationen")),
    ];
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(SET_HISTORY_COMMAND_FILTER_ACTION_ID, LocalizedLabel::native("Set History Filter", "Verlaufsfilter festlegen"), ActionKind::View, "list") }
    .describe(LocalizedLabel::native("Filters the history panel to all entries, only document operations, or everything except document operations; the document is not changed.", "Filtert den Verlaufsbereich auf alle Einträge, nur Dokumentoperationen oder alles außer Dokumentoperationen; das Dokument wird nicht verändert."))
    .with_args([ActionArgDef::select(
        "value",
        LocalizedLabel::native("Filter", "Filter"),
        options,
    )
    .default_value(&"all")])
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

//#region 🔖️ToolRun
pub use semio_framework_tool_run::{
    JobKindId, ToolRunAction, ToolRunCounterDefinition, ToolRunDefinition, ToolRunDefinitionError, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunStageDefinition, ToolRunTraceCursor, ToolRunTraceKind, ToolRunVerdict, TOOL_RUN_ABORT_ACTION_ID,
    tool_run_panel_new_runs, TOOL_RUN_ACTION_IDS, TOOL_RUN_DISMISS_ACTION_ID, TOOL_RUN_DISMISS_CHORD, TOOL_RUN_FINALIZE_ACTION_ID, TOOL_RUN_PAUSE_ACTION_ID, TOOL_RUN_RESUME_ACTION_ID, TOOL_RUN_START_ACTION_ID, TOOL_RUN_STEP_ACTION_ID,
};

/// ⏯️ Whether any tool or utility of `app` declares a `ToolRunDefinition`.
pub fn app_declares_tool_run(app: &AppDefinition) -> bool {
    app.tools.iter().any(|tool| tool.run.is_some()) || app.utilities.iter().any(|utility| utility.run.is_some())
}

/// 🕹️ The seven framework-reserved tool run actions (§2.5 of the tool run contract), auto-injected into any
/// `AppDefinition` whose tools or utilities declare `run` — mirrors `interaction_action_definitions`. Ids,
/// chords, argument names and EN/DE labels come from `semio_framework_tool_run::ToolRunAction`. They are
/// `History` actions like `undo`: host-routed, never queued behind the run, never in the palette or the
/// Actions panel. `toolRunDismiss` carries no app-wide `keys` because `escape` dismisses only with the run
/// panel focused ([`TOOL_RUN_DISMISS_CHORD`]); pause and resume share one toggle chord.
pub fn tool_run_action_definitions(app: &AppDefinition) -> Vec<ActionDefinition> {
    if !app_declares_tool_run(app) {
        return Vec::new();
    }
    ToolRunAction::ALL
        .into_iter()
        .map(|action| {
            let icon = match action {
                ToolRunAction::Start | ToolRunAction::Resume => "play",
                ToolRunAction::Pause => "pause",
                ToolRunAction::Step => "skip-forward",
                ToolRunAction::Abort => "square",
                ToolRunAction::Finalize => "check",
                ToolRunAction::Dismiss => "x",
            };
            let keys = (action != ToolRunAction::Dismiss).then(|| action.chord().to_string());
            ActionDefinition { keys, in_palette: false, ..ActionDefinition::resumable_framework(action.id(), action.label().localized(), ActionKind::History, icon) }
                .with_args(action.args().iter().map(|arg| tool_run_action_arg(arg.name, arg.required)))
                .describe(tool_run_action_description(action))
        })
        .collect()
}

/// 💬️ What each reserved tool run action does to the run and to the document — a run holds its
/// result as a provisional preview, and only `toolRunFinalize` publishes it as one undoable edit.
fn tool_run_action_description(action: ToolRunAction) -> LocalizedLabel {
    match action {
        ToolRunAction::Start => LocalizedLabel::native(
            "Starts a run of the given tool (such as a solver) in a window; the run shows its result as a provisional preview and changes nothing until it is finalized.",
            "Startet einen Lauf des angegebenen Werkzeugs (etwa eines Lösers) in einem Fenster; der Lauf zeigt sein Ergebnis als vorläufige Vorschau und ändert nichts, bis er abgeschlossen wird.",
        ),
        ToolRunAction::Pause => LocalizedLabel::native(
            "Pauses a running tool run at its current step so its progress can be inspected, stepped or resumed.",
            "Hält einen laufenden Werkzeuglauf beim aktuellen Schritt an, damit sein Fortschritt geprüft, schrittweise fortgeführt oder fortgesetzt werden kann.",
        ),
        ToolRunAction::Resume => LocalizedLabel::native("Continues a paused tool run from the step where it stopped.", "Setzt einen pausierten Werkzeuglauf an dem Schritt fort, an dem er angehalten wurde."),
        ToolRunAction::Step => LocalizedLabel::native("Advances a paused tool run by exactly one step and pauses it again.", "Führt einen pausierten Werkzeuglauf genau einen Schritt weiter und hält ihn wieder an."),
        ToolRunAction::Abort => LocalizedLabel::native(
            "Stops a tool run and discards its provisional result; the document stays exactly as it was before the run started.",
            "Bricht einen Werkzeuglauf ab und verwirft sein vorläufiges Ergebnis; das Dokument bleibt genau so, wie es vor dem Start war.",
        ),
        ToolRunAction::Finalize => LocalizedLabel::native(
            "Commits the provisional result of a running, paused or complete tool run into the document as one undoable edit.",
            "Übernimmt das vorläufige Ergebnis eines laufenden, pausierten oder fertigen Werkzeuglaufs als eine rückgängig machbare Änderung in das Dokument.",
        ),
        ToolRunAction::Dismiss => LocalizedLabel::native(
            "Closes the panel of a finalized, aborted or failed tool run; the document is not touched.",
            "Schließt das Panel eines abgeschlossenen, abgebrochenen oder fehlgeschlagenen Werkzeuglaufs; das Dokument bleibt unberührt.",
        ),
    }
}

fn tool_run_action_arg(name: &'static str, required: bool) -> ActionArgDef {
    let (label, schema) = match name {
        semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID => (LocalizedLabel::native("Tool", "Werkzeug"), ActionArgDef::plain_string(None)),
        semio_framework_tool_run::TOOL_RUN_ARG_WINDOW_ID => (LocalizedLabel::native("Window", "Fenster"), ActionArgDef::plain_string(None)),
        semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID => (LocalizedLabel::native("Run", "Lauf"), ArgSchema::number(Some(0.0), None, Some(1.0), true)),
        _ => (LocalizedLabel::native("Generation", "Generation"), ArgSchema::number(Some(0.0), None, Some(1.0), true)),
    };
    ActionArgDef { presentation: Some(ArgPresentation::Hidden), required, ..ActionArgDef::with_schema(name, label, schema) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️tool-run-actions/🦀️.rs"]
mod tool_run_actions_tests;
//#endregion 🔖️ToolRun

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

/// 🎓️ The framework-owned action id apps dispatch to (re)start an app's introduction —
/// auto-injected as a shell-intercepted View action into any
/// `AppDefinition` that declares one (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).
pub const START_INTRODUCTION_ACTION_ID: &str = "startIntroduction";

/// 🎓️ The framework-injected `startIntroduction` View action: fully shell-intercepted (never
/// forwarded to the program), it resets playback to the first step of `AppDefinition.introduction`.
/// Unlike ordinary app actions this stays out of the action palette because the shell exposes the
/// dedicated `Introduce App` command.
pub fn start_introduction_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(START_INTRODUCTION_ACTION_ID, LocalizedLabel::native("Introduce App", "App vorstellen"), ActionKind::View, "graduation-cap") }
}

/// 📤️ The framework-owned action id of Export Document: injected into every app window and fully
/// shell-intercepted (never forwarded to the program) — the shell writes the document's canonical archive (root
/// envelope, its op log, every owned member) as one `.semio-archive` file, for every artifact kind alike.
pub const EXPORT_ARTIFACT_DOCUMENT_ACTION_ID: &str = "exportArtifactDocument";

/// 📥️ The framework-owned action id of Import Document, injected beside [`EXPORT_ARTIFACT_DOCUMENT_ACTION_ID`]
/// and shell-intercepted: the shell opens a picked archive as a NEW document of the same program (a cancellable,
/// progress-reporting load of its op log), never overwriting the focused one.
pub const IMPORT_ARTIFACT_DOCUMENT_ACTION_ID: &str = "importArtifactDocument";

/// 🗃️ The framework-injected Export/Import Document pair: `Shell` verbs in the palette and the `transfer` ribbon
/// category, shell chrome ([`CapabilityAudience::Chrome`]) — agents export through the MCP's own artifact export.
pub fn document_transfer_action_definitions() -> [ActionDefinition; 2] {
    [
        ActionDefinition::resumable_framework(EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, LocalizedLabel::native("Export Document", "Dokument exportieren"), ActionKind::Shell, "export").with_category("transfer").audience(CapabilityAudience::Chrome),
        ActionDefinition::resumable_framework(IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, LocalizedLabel::native("Import Document…", "Dokument importieren…"), ActionKind::Shell, "import").with_category("transfer").audience(CapabilityAudience::Chrome),
    ]
}

/// 📇️ A relative action id used by declarations nested beneath an owning window kind.
/// Distinct from `ActionAddress`, which qualifies a dispatched invocation down to a window instance.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ActionInvocation {
    pub address: ActionAddress,
    pub arguments: BTreeMap<String, DslValue>,
}

//#region 🔖️Utilities
/// 🧰️ Declares one interactive utility (a live-preview pointer mode) an app exposes. Distinct from
/// an `ActionDefinition`: exactly one utility is active per window kind at a time, and activation is
/// host-owned session view state (`ViewModel.active_utility_id`), never a document field or VCS operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct UtilityRef(String);

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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CommandOwnerAddress {
    Os,
    Plugin { plugin_id: String },
    App { plugin_id: String, app_id: String },
    Mode { plugin_id: String, app_id: String, mode_id: String },
}

/// 📍️ Fully qualified address of one command.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CommandAddress {
    pub owner: CommandOwnerAddress,
    pub command_id: String,
}

/// 📨️ One addressed command invocation with named DSL-value arguments — see
/// `ActionInvocation`'s docstring for why `arguments` is keyed to `DslValue`, not
/// `serde_json::Value`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CommandInvocation {
    pub address: CommandAddress,
    pub arguments: BTreeMap<String, DslValue>,
}

/// 💻️ Operating-system command catalog shared by every renderer.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
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

//#region 🔖️Introduction
/// 🎓️ A first-run walkthrough an app declares to introduce its UI, utilities, and actions to a
/// first-time user. Rendered as an ordered sequence of `IntroductionStepDefinition`s over a full-screen
/// glass veil; the shell owns playback (start/advance/skip) as ephemeral chrome state, never the
/// document.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IntroductionDefinition {
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    pub steps: Vec<IntroductionStepDefinition>,
}

/// 🪜️ One step of an `IntroductionDefinition`: an info box pointing at `introduce`, with `show`
/// raising extra elements above the glass veil and `interactions` completing the step.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IntroductionStepDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    pub body: LocalizedLabel,
    /// 🎯️ The single element id raised above the glass, pulsing `data-introduced`, that the info box
    /// anchors to. `None` = a screen-style step: full veil, centered info box.
    #[serde(default)]
    #[value(default)]
    pub introduce: Option<String>,
    /// 🕳️ Additional element ids raised above the glass — interactive, no pulse — e.g. every 3D window
    /// that accepts a catalogue drop while `introduce` teaches the drag source.
    #[serde(default)]
    #[value(default)]
    pub show: Vec<String>,
    #[serde(default)]
    #[value(default)]
    pub placement: IntroductionPlacement,
    /// ✅️ Interactions completing this step; empty means purely informational (Next-button-only).
    #[serde(default)]
    #[value(default)]
    pub interactions: Vec<IntroductionInteraction>,
    /// 🔢️ Whether `interactions` must complete in declaration order — out-of-order completions are
    /// ignored. Unordered: the first incomplete matching interaction completes.
    #[serde(default)]
    #[value(default)]
    pub ordered: bool,
    /// 🏛️ Institution/partner logos shown in the info box below the body — e.g. funding acknowledgements.
    #[serde(default)]
    #[value(default)]
    pub logos: Vec<IntroductionLogo>,
    /// 🎬️ Ghost-cursor demonstrations played in order, one after another, then looping back to the first —
    /// e.g. a viewport step showing zoom, then pan, then orbit. When the step also declares `interactions`,
    /// `demonstrations[i]` previews `interactions[i]` and completed interactions are omitted from replay.
    /// Empty means no demonstration.
    #[serde(default)]
    #[value(default)]
    pub demonstrations: Vec<IntroductionDemonstration>,
}

// 🚫️async: E1 — pure builder methods (self-mutation only, zero suspension points), reverted
// per R9: multiple test consumers are language-barred from async (plain `#[test] fn`, sync
// `catch_unwind` closures, `Vec<IntroductionStepDefinition>` literals).
impl IntroductionStepDefinition {
    pub fn new(id: impl Into<String>, title: impl Into<LocalizedLabel>, body: impl Into<LocalizedLabel>) -> Self {
        Self { id: id.into(), title: title.into(), body: body.into(), introduce: None, show: Vec::new(), placement: IntroductionPlacement::default(), interactions: Vec::new(), ordered: false, logos: Vec::new(), demonstrations: Vec::new() }
    }

    /// 🎯️ Sets the single element id raised above the glass and anchoring the info box.
    pub fn introduce(mut self, element_id: impl Into<String>) -> Self {
        self.introduce = Some(element_id.into());
        self
    }

    /// 🕳️ Additional element ids raised above the glass alongside `introduce` (no pulse).
    pub fn show(mut self, element_ids: Vec<String>) -> Self {
        self.show = element_ids;
        self
    }

    /// 📍️ Overrides where the info box is placed relative to `introduce`.
    pub fn placement(mut self, placement: IntroductionPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// ✅️ Makes the step complete when the user performs all `interactions` (any order) instead of
    /// pressing Next.
    pub fn interact(mut self, interactions: Vec<IntroductionInteraction>) -> Self {
        self.interactions = interactions;
        self
    }

    /// 🔢️ Like `interact`, but `interactions` must complete in declaration order.
    pub fn interact_ordered(mut self, interactions: Vec<IntroductionInteraction>) -> Self {
        self.interactions = interactions;
        self.ordered = true;
        self
    }

    /// 🏛️ Attaches institution/partner logos to the step's info box.
    pub fn logos(mut self, logos: Vec<IntroductionLogo>) -> Self {
        self.logos = logos;
        self
    }

    /// 🎬️ Attaches ghost-cursor demonstrations played in order, then looping back to the first.
    pub fn demonstrate(mut self, demonstrations: Vec<IntroductionDemonstration>) -> Self {
        self.demonstrations = demonstrations;
        self
    }
}

/// 🏛️ One institution/partner logo shown in an `IntroductionStepDefinition`'s info box — a plain
/// URL pair (no DOM/CSS types), optionally linking out when clicked.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IntroductionLogo {
    pub src: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dark_src: Option<String>,
    pub alt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

/// 📍️ Where the info box is placed relative to its anchor.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IntroductionPlacement {
    #[default]
    Auto,
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

/// 👉️ What one `IntroductionInteraction` requires: `Action`/`Utility`/`Tool`/`Panel`/`Expand`
/// complete as soon as the user activates that utility/tool, opens that panel tab, or expands that tree
/// section — teaching by doing. `Pan`/`Zoom`/`Orbit` complete on that camera-navigation gesture over the
/// 3D window named by the payload (a window-kind id) — classified from camera-state deltas by the shell
/// that renders the window, so only shells that render a 3D world (the React shell) can complete them.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", tag = "kind", content = "id")]
#[value(rename_all = "camelCase", tag = "kind", content = "id")]
pub enum IntroductionInteractionKind {
    /// 📇️ References an action owned by the active window kind.
    Action(ActionRef),
    /// 🧰️ References `AppDefinition.utilities`.
    Utility(UtilityRef),
    /// 🛠️ References `AppDefinition.tools` (mode-level tools such as fill).
    Tool(ToolRef),
    /// 📑️ Shell panel tab id (e.g. `framework.panel.catalogue`) — completes when that panel opens.
    Panel(String),
    /// 🌲️ Tree section/item id (e.g. `puzzle3d-play-kinds.objects`) — completes when the user expands it.
    Expand(String),
    /// 🖐️ Completes when the user pans the named 3D window.
    Pan(String),
    /// 🔍️ Completes when the user zooms (scroll or dolly) the named 3D window.
    Zoom(String),
    /// 🌐️ Completes when the user orbits the named 3D window.
    Orbit(String),
}

/// ✅️ One thing the user must do to complete an interaction-gated `IntroductionStepDefinition` —
/// rendered as a checklist row in the info box and celebrated individually on completion.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IntroductionInteraction {
    pub on: IntroductionInteractionKind,
    /// 🏷️ Short checklist label shown in the step's info box.
    pub label: String,
    /// 🎉️ Element id stamped `data-celebrated` on completion; `None` falls back to the step's `introduce`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub celebrate: Option<String>,
}

impl IntroductionInteraction {
    async fn new(on: IntroductionInteractionKind, label: impl Into<String>) -> Self {
        Self { on, label: label.into(), celebrate: None }
    }

    /// 📇️ An interaction completing when the user activates action `id`.
    pub async fn action(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Action(ActionRef::new(id.into())), label).await
    }

    /// 🧰️ An interaction completing when the user activates utility `id`.
    pub async fn utility(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Utility(UtilityRef::new(id.into())), label).await
    }

    /// 🛠️ An interaction completing when the user activates tool `id`.
    pub async fn tool(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Tool(ToolRef::new(id.into()).await), label).await
    }

    /// 📑️ An interaction completing when panel tab `id` opens.
    pub async fn panel(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Panel(id.into()), label).await
    }

    /// 🌲️ An interaction completing when tree section/item `id` expands.
    pub async fn expand(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Expand(id.into()), label).await
    }

    /// 🖐️ An interaction completing when the user pans 3D window `window_kind_id`.
    pub async fn pan(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Pan(window_kind_id.into()), label).await
    }

    /// 🔍️ An interaction completing when the user zooms 3D window `window_kind_id`.
    pub async fn zoom(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Zoom(window_kind_id.into()), label).await
    }

    /// 🌐️ An interaction completing when the user orbits 3D window `window_kind_id`.
    pub async fn orbit(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Orbit(window_kind_id.into()), label).await
    }

    /// 🎉️ Overrides which element id is stamped `data-celebrated` on completion.
    pub async fn celebrate(mut self, element_id: impl Into<String>) -> Self {
        self.celebrate = Some(element_id.into());
        self
    }
}

/// 📌️ Where a demonstration gesture points, resolvable to a viewport pixel at play time. One
/// point type covers click targets and drag endpoints across every addressing scheme the shell needs:
/// element-relative, absolute/normalized screen space, absolute/normalized window(pane)-local space, and
/// a 3D scene world position projected through that window's live camera.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
// 🐢️ `rename_all_fields` is required alongside `rename_all` — the latter only renames the *variant* tag
// values; without the former, a future multi-word field inside a variant would silently serialize
// snake_case and desync from the generated TS type (see `UiDirtyScope`'s comment for the full story).
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum IntroductionPoint {
    /// 🎯️ Center (or `offset`, normalized 0–1 within the element's rect) of the element `id` resolves to.
    Element {
        id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        offset: Option<[f64; 2]>,
    },
    /// 🖥️ Absolute viewport pixel.
    Screen { x: f64, y: f64 },
    /// 🖥️ Normalized 0–1 of the viewport.
    ScreenNormalized { x: f64, y: f64 },
    /// 🪟️ Pixel local to window/pane element `id`'s rect (top-left origin).
    Window { id: String, x: f64, y: f64 },
    /// 🪟️ Normalized 0–1 within window/pane element `id`'s rect.
    WindowNormalized { id: String, x: f64, y: f64 },
    /// 🧊️ 3D world-space position in the scene shown by window `id`, projected through its live camera.
    Scene { id: String, position: [f64; 3] },
    /// 🗺️ 2D world-space coordinates (camera x/y/zoom) on the infinite-canvas surface shown by window
    /// `id` — the 2D sibling of `Scene`. On a 3D window this resolves via the ground plane (z = 0).
    Canvas { id: String, x: f64, y: f64 },
    /// 🏷️ A live entity addressed semantically in the shell's established pick-target grammar (see
    /// `CanvasPickTarget`): `domain` is the surface's target domain ("vortex", "object", "attraction",
    /// "node", "edge", "handle", "position", "route", "block", "layer", …), `entity` its id verbatim
    /// (compound forms like `"objectId:vortexId"` or `"widgetId:port"` included; `"*"` = any — the
    /// surface picks a representative, nearest the viewport center). `offset` is normalized 0–1 within
    /// the entity's bounds, default center.
    Entity {
        id: String,
        domain: String,
        entity: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        offset: Option<[f64; 2]>,
    },
    /// 🪡️ A parametric point along an entity's curve geometry (an attraction segment, graph edge, ink
    /// stroke, or canvas path layer) — `t` in 0–1 by arc length.
    Curve { id: String, domain: String, entity: String, t: f64 },
    /// 🎚️ A value mapped through an entity's live value domain (e.g. a graph slider's min..max onto its
    /// track), resolved to the corresponding point along the entity's geometry.
    Domain { id: String, domain: String, entity: String, value: f64 },
}

impl IntroductionPoint {
    /// 🗺️ 2D world-space coordinates on the infinite-canvas surface shown by window `window_id`.
    pub async fn canvas(window_id: impl Into<String>, x: f64, y: f64) -> Self {
        Self::Canvas { id: window_id.into(), x, y }
    }

    /// 🏷️ A specific entity by domain + id, centered (no `offset`).
    pub async fn entity(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>) -> Self {
        Self::Entity { id: window_id.into(), domain: domain.into(), entity: entity.into(), offset: None }
    }

    /// 🏷️ Any entity in `domain` — the surface picks a representative, nearest the viewport center.
    pub async fn any_entity(window_id: impl Into<String>, domain: impl Into<String>) -> Self {
        Self::entity(window_id, domain, "*").await
    }

    /// 🪡️ A parametric point at `t` (0–1 by arc length) along an entity's curve geometry.
    pub async fn curve(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>, t: f64) -> Self {
        Self::Curve { id: window_id.into(), domain: domain.into(), entity: entity.into(), t }
    }

    /// 🎚️ A value mapped through an entity's live value domain (e.g. a slider's min..max).
    pub async fn domain_value(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>, value: f64) -> Self {
        Self::Domain { id: window_id.into(), domain: domain.into(), entity: entity.into(), value }
    }
}

/// 🖱️ Which mouse button a drag-like demonstration presses.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IntroductionPointerButton {
    #[default]
    Left,
    Middle,
    Right,
}

/// ⌨️ Keyboard modifier held during a drag-like demonstration.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IntroductionKeyModifier {
    Alt,
    Shift,
    Control,
    Meta,
}

// 🚫️async: E4 fn-pointer slot (serde default)
fn introduction_pointer_button_left() -> IntroductionPointerButton {
    IntroductionPointerButton::Left
}

// 🚫️async: E4 fn-pointer slot (serde default)
fn introduction_pointer_button_right() -> IntroductionPointerButton {
    IntroductionPointerButton::Right
}

// 🚫️async: E4 fn-pointer slot (serde default)
fn introduction_orbit_default_modifiers() -> Vec<IntroductionKeyModifier> {
    vec![IntroductionKeyModifier::Alt]
}

/// 👆️ A gesture a demonstration plays: the ghost cursor travels to (or between) `IntroductionPoint`s
/// and performs the visual press/release affordance for the gesture kind.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
// 🐢️ `rename_all_fields` required alongside `rename_all` so `Scroll`'s `delta_y` field actually
// serializes/types as `deltaY` — see `IntroductionPoint`'s comment / `UiDirtyScope`'s for the full story.
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum IntroductionGesture {
    LeftClick {
        at: IntroductionPoint,
    },
    RightClick {
        at: IntroductionPoint,
    },
    DoubleClick {
        at: IntroductionPoint,
    },
    Drag {
        from: IntroductionPoint,
        to: IntroductionPoint,
        #[serde(default = "introduction_pointer_button_left")]
        #[value(default = "introduction_pointer_button_left")]
        button: IntroductionPointerButton,
        #[serde(default)]
        #[value(default)]
        modifiers: Vec<IntroductionKeyModifier>,
    },
    Scroll {
        at: IntroductionPoint,
        delta_y: f64,
    },
    /// 🌐️ A curved (not straight-line) drag around a pivot — camera orbit, distinct from `Drag`'s
    /// straight-line pan/reposition motion.
    Orbit {
        from: IntroductionPoint,
        to: IntroductionPoint,
        #[serde(default = "introduction_pointer_button_right")]
        #[value(default = "introduction_pointer_button_right")]
        button: IntroductionPointerButton,
        #[serde(default = "introduction_orbit_default_modifiers")]
        #[value(default = "introduction_orbit_default_modifiers")]
        modifiers: Vec<IntroductionKeyModifier>,
    },
}

/// 🖱️ Ghost-cursor glyph, mirroring `🎨️ui.css`'s `--cursor-*` custom cursors.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IntroductionCursor {
    #[default]
    Default,
    Pointer,
    Grab,
    Grabbing,
    Crosshair,
    Move,
}

/// 🎬️ A looping ghost-cursor demonstration attached to an interaction-gated
/// `IntroductionStepDefinition`. Plays only while the user's own pointer is idle — any real pointer
/// movement mutes it and restores the real cursor instantly; going idle again while the step is still
/// active replays it from the beginning. `cursor` overrides the glyph shown over the target; omitted, it
/// derives from `gesture` (clicks → pointer, drag → grab/grabbing).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IntroductionDemonstration {
    pub gesture: IntroductionGesture,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<IntroductionCursor>,
}

impl IntroductionDemonstration {
    /// 👆️ A left-click demonstration at `at`.
    pub async fn left_click(at: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::LeftClick { at }, cursor: None }
    }

    /// 👆️ A right-click demonstration at `at`.
    pub async fn right_click(at: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::RightClick { at }, cursor: None }
    }

    /// ✋️ A click-and-drag demonstration from `from` to `to`.
    pub async fn drag(from: IntroductionPoint, to: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::Drag { from, to, button: IntroductionPointerButton::Left, modifiers: vec![] }, cursor: None }
    }

    /// 🖲️ A scroll-wheel demonstration at `at`; `delta_y` sign conveys direction.
    pub async fn scroll(at: IntroductionPoint, delta_y: f64) -> Self {
        Self { gesture: IntroductionGesture::Scroll { at, delta_y }, cursor: None }
    }

    /// 🌐️ A camera-orbit demonstration curving from `from` to `to`.
    pub async fn orbit(from: IntroductionPoint, to: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::Orbit { from, to, button: IntroductionPointerButton::Right, modifiers: vec![IntroductionKeyModifier::Alt] }, cursor: None }
    }
}
//#endregion 🔖️Introduction

//#region 🔖️Tutorial
/// 🎬️ A recorded, timed, replayable walkthrough — the timeline sibling of the step-gated
/// `IntroductionDefinition`. Where an introduction gates progression on the user performing an
/// interaction, a tutorial plays a multi-track recording (narration, video overlay, UI state, document
/// edits, camera, ghost-cursor gestures) against a sandboxed copy of the document while the user watches,
/// scrubs, or deviates and converges back. A *recording* IS a `TutorialDefinition` — the recorder simply
/// produces a densely-sampled one; nothing distinguishes a hand-authored tutorial from a captured one.
/// Distinct from the docs-tooltip `tutorial` link field in `ui/js/react`'s `UiLabelLeaf` (a URL into the
/// manual) — this is the interactive playback mechanism.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedLabel>,
    /// ⏱️ Total timeline length in milliseconds; every track entry's `at` (+ duration) must fit within.
    pub duration_ms: u64,
    /// 📖️ Scrub-bar markers, sorted ascending by `at`.
    #[serde(default)]
    #[value(default)]
    pub chapters: Vec<TutorialChapter>,
    /// 🎬️ Starting conditions the player restores into its sandbox before t=0.
    pub base: TutorialBase,
    pub tracks: TutorialTracks,
    /// 🧾️ Recorder provenance (ISO 8601 timestamp); `None` means hand-authored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub recorded_at: Option<String>,
}

impl TutorialDefinition {
    /// 📂️ Deserializes a `TutorialDefinition` from its JSON wire format — the constructor apps use
    /// to load a hand-authored or recorded tutorial (e.g. via `include_str!`) into `.tutorial(...)`.
    // 🚧️ BLOCKED: `TutorialDefinition` itself is serde-only (see its docstring) — this stays on
    // `serde_json` until that lands.
    pub async fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// 📖️ One scrub-bar marker in a `TutorialDefinition`'s timeline.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialChapter {
    pub id: String,
    pub at: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<LocalizedLabel>,
}

/// 🎬️ What must be true at t=0: the document the tutorial sandboxes and the initial UI/camera
/// state. The player snapshots the user's live document, loads this in its place, and restores the
/// snapshot on exit — a tutorial can never touch real work.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialBase {
    /// 📂️ Full document DSL text (`ArtifactTextFiles.dsl`) to sandbox-load; `None` falls back to `example_id`, and both
    /// `None` falls back to the app's default/empty document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document_dsl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub example_id: Option<String>,
    pub ui: TutorialUiSnapshot,
    /// 🎥️ Initial camera per window instance (every entry's `at` is `0`).
    #[serde(default)]
    #[value(default)]
    pub cameras: Vec<TutorialCameraKeyframe>,
}

/// 🎞️ The seven parallel tracks of a `TutorialDefinition`'s timeline; every entry's `at` is a
/// millisecond offset from tutorial start, and each `Vec` is sorted ascending by `at`
/// (`validate_tutorial` enforces this).
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialTracks {
    #[serde(default)]
    #[value(default)]
    pub narration: Vec<TutorialNarrationCue>,
    #[serde(default)]
    #[value(default)]
    pub video: Vec<TutorialVideoCue>,
    /// 🏷️ Annotational only — drives affordance pulses and scrub-bar tick marks; playback never
    /// re-dispatches these into a plugin (see `TutorialEventKind`).
    #[serde(default)]
    #[value(default)]
    pub events: Vec<TutorialEvent>,
    #[serde(default)]
    #[value(default)]
    pub ui: Vec<TutorialUiKeyframe>,
    /// 🖋️ The sole source of document mutation during playback — see `TutorialDocumentEventKind`.
    #[serde(default)]
    #[value(default)]
    pub document: Vec<TutorialDocumentEvent>,
    #[serde(default)]
    #[value(default)]
    pub camera: Vec<TutorialCameraKeyframe>,
    #[serde(default)]
    #[value(default)]
    pub gestures: Vec<TutorialGestureCue>,
}

/// 📦️ Where a tutorial media asset's bytes live. `Blob` is wire-identical to `store::BlobRef`
/// (content-addressed Blake3 hash + size + media type) — `framework/core` does not depend on
/// `semio-vcs`, so the shape is mirrored rather than reused; conversion between the two is
/// field-for-field.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialAssetSrc {
    /// 🌐️ Static asset route (a brand's `assetsDir` or the shared `ui/asset` mount).
    Url { url: String },
    /// 🗄️ Content-addressed blob in the studio's `BlobStore`.
    Blob { hash: String, size: u64, media_type: String },
    /// 🧵️ Inline data URL — the recorder's default before a save destination is chosen.
    DataUrl { data: String },
}

// 🚫️async: E4 fn-pointer slot (serde default)
fn tutorial_narration_default_rate() -> f64 {
    1.0
}

// 🚫️async: E4 fn-pointer slot (serde skip_serializing_if)
fn tutorial_rate_is_default(rate: &f64) -> bool {
    (*rate - 1.0).abs() < f64::EPSILON
}

/// 🎙️ One voiceover cue: `text` is both the TTS script and the caption fallback; `audio`
/// overrides TTS with a recorded take. The timeline is always the master clock — a still-speaking TTS
/// utterance is cancelled at the next cue's `at`; audio assets are seeked and rate-matched to the
/// playhead instead of played independently.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialNarrationCue {
    pub id: String,
    pub at: u64,
    /// ⏱️ Audio duration when `audio` is set (recorder-measured); a rough TTS estimate otherwise — used
    /// for scrub-bar layout only, never to gate playback.
    pub duration_ms: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub text: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<TutorialAssetSrc>,
    /// 🗣️ Web Speech API voice-name hint; ignored once `audio` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    /// 🎚️ TTS/audio rate multiplier layered under the player's own playback-rate control.
    #[serde(default = "tutorial_narration_default_rate", skip_serializing_if = "tutorial_rate_is_default")]
    #[value(default = "tutorial_narration_default_rate", skip_serializing_if = "tutorial_rate_is_default")]
    pub rate: f64,
    /// 💬️ Timed caption sub-segments (offsets relative to this cue's `at`); empty means `text` is shown
    /// whole for the cue's `duration_ms`.
    #[serde(default)]
    #[value(default)]
    pub captions: Vec<TutorialCaption>,
}

/// 💬️ One timed caption sub-segment of a `TutorialNarrationCue`.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialCaption {
    pub at: u64,
    pub duration_ms: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub text: LocalizedLabel,
}

/// 🖼️ Normalized 0–1 viewport rect for a `TutorialVideoCue` overlay.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialOverlayRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for TutorialOverlayRect {
    /// 📌️ Bottom-right picture-in-picture, ~16:9.
    fn default() -> Self {
        Self { x: 0.72, y: 0.70, width: 0.24, height: 0.24 }
    }
}

/// 📹️ A timed video overlay — e.g. a presenter webcam picture-in-picture, or an authored clip.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialVideoCue {
    pub at: u64,
    pub duration_ms: u64,
    pub src: TutorialAssetSrc,
    #[serde(default)]
    #[value(default)]
    pub rect: TutorialOverlayRect,
    /// 🔇️ True when narration carries the audio (a webcam take recorded muted).
    #[serde(default)]
    #[value(default)]
    pub muted: bool,
    /// ⏩️ Seek offset into the source at cue start.
    #[serde(default)]
    #[value(default)]
    pub source_offset_ms: u64,
}

/// 🏷️ One recorded action/command/keypress, annotational only — see `TutorialTracks::events`.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialEvent {
    pub at: u64,
    pub kind: TutorialEventKind,
}

/// 🏷️ What one `TutorialEvent` annotates.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialEventKind {
    /// 📇️ A relative dispatch to an action owned by the active window kind, with its effective args.
    Action {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    /// 🎛️ A `CommandDefinition` dispatch.
    Command {
        command: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    /// ⌨️ A keybinding press, display-only over the action it triggered.
    Key { keys: String },
}

/// 🧮️ One UI-state track entry: either a full restore-point snapshot (a valid seek anchor) or a
/// sparse list of changes since the previous sample.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialUiKeyframe {
    pub at: u64,
    pub sample: TutorialUiSample,
}

/// 🧮️ See `TutorialUiKeyframe`.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialUiSample {
    Snapshot { state: Box<TutorialUiSnapshot> },
    Delta { changes: Vec<TutorialUiChange> },
}

/// 🧮️ Renderer-neutral restore point for chrome/UI state — a superset of `ViewModel` plus the
/// dock/panel/dialog state neither shell serializes today. Deliberately NOT a serialization of either
/// shell's internal store: each shell implements its own `captureUiSnapshot`/`applyUiSnapshot` against
/// this shape. Locale/terminology are excluded on purpose — a tutorial plays in the viewer's own locale.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialUiSnapshot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub active_mode_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub focused_window_id: Option<String>,
    /// 🧰️ Mirrors `ViewModel.active_utility_by_window_id`.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub active_utility_by_window_id: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub active_tool_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<WindowLayout>,
    /// 📑️ Active tab id per concrete panel anchor; anchors absent from the map are collapsed/closed.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub active_panel_tab_by_group: std::collections::HashMap<String, String>,
    /// 🗂️ Opaque program vocabulary, verbatim `ViewModel.panel_json`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub panel_json: Option<String>,
    /// 🕹️ Per-domain selection state, keyed by `InteractionDefinition.id` — the framework-owned
    /// replacement for the deleted opaque `selection_json`; see `TutorialUiChange::Selection`.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub interaction_selection: std::collections::HashMap<String, DomainSelection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub open_dialog_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub expanded_tree_ids: Vec<String>,
    #[serde(default)]
    #[value(default)]
    pub command_panel_open: bool,
}

/// 🩹️ One typed, sparse UI-state change — the alphabet `compose_tutorial_ui` replays over a prior
/// `TutorialUiSnapshot` to reconstruct state at any timeline offset without shipping a full snapshot at
/// every sample.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialUiChange {
    ActiveMode {
        id: String,
    },
    FocusedWindow {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
    },
    /// 🧰️ `utility_id: None` deactivates — mirrors `SetActiveUtility` semantics.
    ActiveUtility {
        window_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        utility_id: Option<String>,
    },
    ActiveTool {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
    },
    Layout {
        layout: WindowLayout,
    },
    /// 📑️ `group` carries the concrete anchor id; `tab_id: None` closes that anchor.
    PanelTab {
        group: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        tab_id: Option<String>,
    },
    PanelState {
        panel_json: String,
    },
    /// 🕹️ Drives one interaction domain's selection during replay — carries the resolved
    /// `DomainSelection` directly rather than re-dispatching `interactionSelect` (a raw pointer/keyboard
    /// event would be non-deterministic on replay). `ids: []` clears the domain's selection.
    Selection {
        domain_id: String,
        granularity: String,
        #[serde(default)]
        #[value(default)]
        ids: Vec<String>,
    },
    Dialog {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    TreeExpansion {
        id: String,
        expanded: bool,
    },
    CommandPanel {
        open: bool,
    },
}

/// 🖋️ One document-track entry — mirrors `store::ArtifactCommand` with `Mutation =
/// serde_json::Value` (opaque per-app mutation JSON, already the wire shape of every `KernelMutation`
/// diff). This is the SOLE source of document mutation during playback: recorded `TutorialEvent`s are
/// annotational only, never re-dispatched, because re-dispatching a plugin action is non-deterministic
/// (fresh ids/timestamps) and would double-apply against this track.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialDocumentEvent {
    pub at: u64,
    pub kind: TutorialDocumentEventKind,
}

/// 🖋️ See `TutorialDocumentEvent`. `Edit` carries both `forwards` and `backwards` operations
/// verbatim from the vcs edit that produced it — the source of exact bidirectional scrubbing.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialDocumentEventKind {
    Edit {
        forwards: Vec<DslValue>,
        backwards: Vec<DslValue>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        coalesce_key: Option<String>,
    },
    Undo,
    Redo,
    Checkpoint {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
    CheckoutCheckpoint {
        checkpoint_id: String,
    },
    SwitchAlternative {
        alternative_id: String,
    },
    /// 📂️ Wholesale document replacement (e.g. a mid-tutorial example switch) — full
    /// `ArtifactEnvelope` JSON in both directions.
    Load {
        document_dsl: String,
        previous_dsl: String,
    },
}

// 🚫️async: E4 fn-pointer slot (serde default)
fn tutorial_camera_up_z() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

/// 🎥️ One camera track keyframe for a specific window instance.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialCameraKeyframe {
    pub at: u64,
    /// 🪟️ Window *instance* id (matches `ViewWindowInstance.id`).
    pub window_id: String,
    pub camera: TutorialCameraState,
    /// 🪄️ Easing INTO this keyframe from the previous one on the same window.
    #[serde(default)]
    #[value(default)]
    pub easing: TutorialEasing,
}

/// 🎥️ A camera pose — `Orbit` mirrors `World3dScene.camera_json`/`OrbitController`, `Canvas`
/// mirrors `Canvas2dScene`'s `cameraX`/`cameraY`/`zoom`.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialCameraState {
    Orbit {
        position: [f64; 3],
        target: [f64; 3],
        #[serde(default = "tutorial_camera_up_z")]
        #[value(default = "tutorial_camera_up_z")]
        up: [f64; 3],
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        fov: Option<f64>,
    },
    Canvas {
        x: f64,
        y: f64,
        zoom: f64,
    },
}

/// 🪄️ Interpolation curve into a `TutorialCameraKeyframe` from its predecessor on the same window.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum TutorialEasing {
    Linear,
    #[default]
    EaseInOut,
    /// 📌️ No interpolation — hold the previous pose until this keyframe, then snap.
    Hold,
}

/// 👻️ One ghost-cursor gesture cue, reusing the introduction demonstration vocabulary verbatim —
/// both shells already resolve/render `IntroductionGesture`/`IntroductionPoint`/`IntroductionCursor`.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialGestureCue {
    pub at: u64,
    pub duration_ms: u64,
    pub gesture: IntroductionGesture,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<IntroductionCursor>,
}

/// 🎬️ The framework-owned action id apps dispatch to (re)start a tutorial — auto-injected as a
/// fully shell-intercepted View action into any `AppDefinition` that declares one (mirrors
/// `START_INTRODUCTION_ACTION_ID`). Distinct from an introduction: a tutorial takes a required
/// `tutorialId` argument since an app may declare more than one.
pub const START_TUTORIAL_ACTION_ID: &str = "startTutorial";

/// 🎬️ The framework-injected `startTutorial` View action: fully shell-intercepted, it sandboxes
/// the live document, loads the selected tutorial's `base`, and starts playback from t=0.
pub fn start_tutorial_action_definition(tutorials: &[TutorialDefinition]) -> ActionDefinition {
    let mut options = Vec::with_capacity(tutorials.len());
    for t in tutorials {
        options.push(ActionArgOption::new(t.id.clone(), t.title.clone()));
    }
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(START_TUTORIAL_ACTION_ID, LocalizedLabel::native("Play Tutorial", "Tutorial abspielen"), ActionKind::View, "eye") }.with_args([ActionArgDef::select(
        "tutorialId",
        LocalizedLabel::native("Tutorial", "Tutorial"),
        options,
    )
    .required()])
}

/// ⏺️ The framework-owned action id that opens the tutorial recorder chrome — auto-injected into
/// EVERY `AppDefinition` (recording needs no app-side declaration at all).
pub const RECORD_TUTORIAL_ACTION_ID: &str = "recordTutorial";

/// ⏺️ The framework-injected `recordTutorial` View action: fully shell-intercepted, arms the
/// recorder against the live document (never a sandboxed copy — a recording IS the user's work).
pub fn record_tutorial_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(RECORD_TUTORIAL_ACTION_ID, LocalizedLabel::native("Record Tutorial", "Tutorial aufzeichnen"), ActionKind::View, "eye") }
        .describe(LocalizedLabel::native("Opens the tutorial recorder on the open document so the next interactions are recorded as a tutorial.", "Öffnet den Tutorial-Rekorder für das geöffnete Dokument, damit die nächsten Interaktionen als Tutorial aufgezeichnet werden."))
}

/// ⏱️ Real-time (not timeline-time, not rate-scaled) duration of the camera glide the player performs
/// when the user presses Play after deviating from an active tutorial's recorded state.
pub const TUTORIAL_CONVERGE_MS: u64 = 600;

//#region 🔖️TutorialEngine
/// ✅️ Structural validation shared by the plugin builder and both recorders before save: every
/// track sorted ascending by `at`, every entry within `[0, durationMs]`, chapter/narration-cue ids
/// unique, `base.cameras` all at `at == 0`. Does NOT check that referenced action/command/element ids
/// exist — the plugin builder's validation (which has the full `AppDefinition` in scope) does that.
pub fn validate_tutorial(def: &TutorialDefinition) -> Result<(), String> {
    fn sorted_by_at<T>(label: &str, items: &[T], at: impl Fn(&T) -> u64, duration_ms: u64) -> Result<(), String> {
        let mut last: Option<u64> = None;
        for item in items {
            let at = at(item);
            if at > duration_ms {
                return Err(format!("tutorial track `{label}` has an entry at {at}ms beyond durationMs {duration_ms}"));
            }
            if let Some(last) = last {
                if at < last {
                    return Err(format!("tutorial track `{label}` is not sorted ascending by `at` ({last}ms then {at}ms)"));
                }
            }
            last = Some(at);
        }
        Ok(())
    }

    sorted_by_at("chapters", &def.chapters, |c| c.at, def.duration_ms)?;
    sorted_by_at("narration", &def.tracks.narration, |c| c.at, def.duration_ms)?;
    sorted_by_at("video", &def.tracks.video, |c| c.at, def.duration_ms)?;
    sorted_by_at("events", &def.tracks.events, |e| e.at, def.duration_ms)?;
    sorted_by_at("ui", &def.tracks.ui, |k| k.at, def.duration_ms)?;
    sorted_by_at("document", &def.tracks.document, |e| e.at, def.duration_ms)?;
    sorted_by_at("camera", &def.tracks.camera, |k| k.at, def.duration_ms)?;
    sorted_by_at("gestures", &def.tracks.gestures, |c| c.at, def.duration_ms)?;

    let mut chapter_ids = std::collections::HashSet::new();
    for chapter in &def.chapters {
        if !chapter_ids.insert(chapter.id.as_str()) {
            return Err(format!("duplicate tutorial chapter id `{}`", chapter.id));
        }
    }
    let mut cue_ids = std::collections::HashSet::new();
    for cue in &def.tracks.narration {
        if !cue_ids.insert(cue.id.as_str()) {
            return Err(format!("duplicate tutorial narration cue id `{}`", cue.id));
        }
    }
    for camera in &def.base.cameras {
        if camera.at != 0 {
            return Err(format!("tutorial base camera keyframe for window `{}` must have at == 0", camera.window_id));
        }
    }
    Ok(())
}

fn tutorial_ease_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

fn tutorial_lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// 🎥️ Interpolates between two camera keyframes at timeline offset `at_ms` (clamped into
/// `[prev.at, next.at]`). Position/target/up/fov lerp componentwise; `Canvas.zoom` interpolates in log
/// space so zooming reads as constant visual speed. `next.easing` governs the curve; `Hold` snaps to
/// `prev` until `next.at`, then jumps. Mismatched camera kinds between the two keyframes (`Orbit` vs
/// `Canvas` on the same window) never interpolate — the result snaps to whichever side `t` is closer to.
pub fn interpolate_tutorial_camera(prev: &TutorialCameraKeyframe, next: &TutorialCameraKeyframe, at_ms: f64) -> TutorialCameraState {
    let span = (next.at as f64 - prev.at as f64).max(1.0);
    let raw = ((at_ms - prev.at as f64) / span).clamp(0.0, 1.0);
    let t = match next.easing {
        TutorialEasing::Linear => raw,
        TutorialEasing::EaseInOut => tutorial_ease_in_out(raw),
        TutorialEasing::Hold => {
            if raw >= 1.0 {
                1.0
            } else {
                0.0
            }
        }
    };
    match (&prev.camera, &next.camera) {
        (TutorialCameraState::Orbit { position: p0, target: t0, up: u0, fov: f0 }, TutorialCameraState::Orbit { position: p1, target: t1, up: u1, fov: f1 }) => TutorialCameraState::Orbit {
            position: tutorial_lerp3(*p0, *p1, t),
            target: tutorial_lerp3(*t0, *t1, t),
            up: tutorial_lerp3(*u0, *u1, t),
            fov: match (f0, f1) {
                (Some(a), Some(b)) => Some(a + (b - a) * t),
                (Some(a), None) => Some(*a),
                (None, Some(b)) => Some(*b),
                (None, None) => None,
            },
        },
        (TutorialCameraState::Canvas { x: x0, y: y0, zoom: z0 }, TutorialCameraState::Canvas { x: x1, y: y1, zoom: z1 }) => TutorialCameraState::Canvas { x: x0 + (x1 - x0) * t, y: y0 + (y1 - y0) * t, zoom: (z0.ln() + (z1.ln() - z0.ln()) * t).exp() },
        _ => {
            if t < 0.5 {
                prev.camera.clone()
            } else {
                next.camera.clone()
            }
        }
    }
}

/// 🎥️ Finds the camera pose for `window_id` at `at_ms`: exact if `at_ms` lands on or before the
/// first keyframe (falling back to `base.cameras`), interpolated between the bracketing pair otherwise,
/// held at the last pose past the final keyframe. `None` when the window has no camera keyframes at all.
pub fn tutorial_camera_at(def: &TutorialDefinition, window_id: &str, at_ms: f64) -> Option<TutorialCameraState> {
    let keyframes: Vec<&TutorialCameraKeyframe> = def.base.cameras.iter().chain(def.tracks.camera.iter()).filter(|k| k.window_id == window_id).collect();
    let first = keyframes.first()?;
    if at_ms <= first.at as f64 {
        return Some(first.camera.clone());
    }
    for pair in keyframes.windows(2) {
        let (prev, next) = (pair[0], pair[1]);
        if at_ms <= next.at as f64 {
            return Some(interpolate_tutorial_camera(prev, next, at_ms));
        }
    }
    Some(keyframes.last().unwrap().camera.clone())
}

/// 🩹️ Applies one `TutorialUiChange` onto a `TutorialUiSnapshot` in place — the pure core both
/// `compose_tutorial_ui` and each shell's live director share.
pub fn apply_tutorial_ui_change(state: &mut TutorialUiSnapshot, change: &TutorialUiChange) {
    match change {
        TutorialUiChange::ActiveMode { id } => state.active_mode_id = Some(id.clone()),
        TutorialUiChange::FocusedWindow { id } => state.focused_window_id = id.clone(),
        TutorialUiChange::ActiveUtility { window_id, utility_id } => match utility_id {
            Some(id) => {
                state.active_utility_by_window_id.insert(window_id.clone(), id.clone());
            }
            None => {
                state.active_utility_by_window_id.remove(window_id);
            }
        },
        TutorialUiChange::ActiveTool { id } => state.active_tool_id = id.clone(),
        TutorialUiChange::Layout { layout } => state.layout = Some(layout.clone()),
        TutorialUiChange::PanelTab { group, tab_id } => match tab_id {
            Some(id) => {
                state.active_panel_tab_by_group.insert(group.clone(), id.clone());
            }
            None => {
                state.active_panel_tab_by_group.remove(group);
            }
        },
        TutorialUiChange::PanelState { panel_json } => state.panel_json = Some(panel_json.clone()),
        TutorialUiChange::Selection { domain_id, granularity, ids } => {
            if ids.is_empty() {
                state.interaction_selection.remove(domain_id);
            } else {
                state.interaction_selection.insert(domain_id.clone(), DomainSelection { granularity: granularity.clone(), ids: ids.clone(), anchor_id: None });
            }
        }
        TutorialUiChange::Dialog { id, .. } => state.open_dialog_id = id.clone(),
        TutorialUiChange::TreeExpansion { id, expanded } => {
            if *expanded {
                if !state.expanded_tree_ids.iter().any(|existing| existing == id) {
                    state.expanded_tree_ids.push(id.clone());
                }
            } else {
                state.expanded_tree_ids.retain(|existing| existing != id);
            }
        }
        TutorialUiChange::CommandPanel { open } => state.command_panel_open = *open,
    }
}

/// 🧮️ Reconstructs the full `TutorialUiSnapshot` at `at_ms`: starts from `base.ui`, then the
/// latest `Snapshot` sample with `at <= at_ms` (if any, replacing the base), then replays every `Delta`
/// sample after that snapshot up to and including `at_ms`, in order. This is the one place seeking (and
/// the deviation-then-play converge step) source their target UI state.
pub fn compose_tutorial_ui(def: &TutorialDefinition, at_ms: f64) -> TutorialUiSnapshot {
    let mut state = def.base.ui.clone();
    let mut deltas: Vec<&TutorialUiChange> = Vec::new();
    for keyframe in &def.tracks.ui {
        if keyframe.at as f64 > at_ms {
            break;
        }
        match &keyframe.sample {
            TutorialUiSample::Snapshot { state: snapshot } => {
                state = snapshot.as_ref().clone();
                deltas.clear();
            }
            TutorialUiSample::Delta { changes } => {
                deltas.extend(changes.iter());
            }
        }
    }
    for change in deltas {
        apply_tutorial_ui_change(&mut state, change);
    }
    state
}

/// ✂️ Everything a live director's tick from `from_ms` to `to_ms` must apply: annotational
/// events, document edits, and UI deltas within the half-open interval on the crossing direction (empty
/// when `from_ms == to_ms`). Backward direction (scrubbing left) reverses entry order so callers apply
/// each `TutorialDocumentEventKind::Edit`'s `backwards` ops from most-recent to least-recent. Plain Rust
/// struct (not owned schema mirrored) — the TS port lives in `framework/renderer/react/index.tsx` and is pinned
/// to this one via shared golden fixtures, not a wasm call per frame.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TutorialSlice {
    pub forward: bool,
    pub events: Vec<TutorialEvent>,
    pub document: Vec<TutorialDocumentEvent>,
    pub ui_changes: Vec<TutorialUiChange>,
}

/// ✂️ Computes the `TutorialSlice` for advancing the playhead from `from_ms` to `to_ms` (`to_ms`
/// may be less than `from_ms` when scrubbing backward).
///
/// 🐢️ A `TutorialUiSample::Snapshot` crossed mid-slice is intentionally NOT flattened into deltas here:
/// recomposing state across a snapshot boundary is exactly what `compose_tutorial_ui` already does
/// correctly and cheaply. This function is for the live per-tick advance, which never spans a snapshot
/// in practice (ticks run far more often than the multi-second snapshot cadence); any caller that jumps
/// across a snapshot boundary (a seek/scrub) should call `compose_tutorial_ui` wholesale instead of
/// accumulating through this slice.
pub fn tutorial_slice(def: &TutorialDefinition, from_ms: f64, to_ms: f64) -> TutorialSlice {
    let forward = to_ms >= from_ms;
    let (lo, hi) = if forward { (from_ms, to_ms) } else { (to_ms, from_ms) };
    let in_range = |at: u64| (at as f64) > lo && (at as f64) <= hi;

    let mut events: Vec<TutorialEvent> = def.tracks.events.iter().filter(|e| in_range(e.at)).cloned().collect();
    let mut document: Vec<TutorialDocumentEvent> = def.tracks.document.iter().filter(|e| in_range(e.at)).cloned().collect();
    let mut ui_changes: Vec<TutorialUiChange> = Vec::new();
    for keyframe in def.tracks.ui.iter().filter(|k| in_range(k.at)) {
        if let TutorialUiSample::Delta { changes } = &keyframe.sample {
            ui_changes.extend(changes.iter().cloned());
        }
    }
    if !forward {
        events.reverse();
        document.reverse();
        ui_changes.reverse();
    }
    TutorialSlice { forward, events, document, ui_changes }
}
//#endregion 🔖️TutorialEngine
//#endregion 🔖️Tutorial

//#region 🔖️Dialog
/// 🗨️ A declared modal form dialog: a glass veil covers the screen and an info box (styled
/// identically to the introduction walkthrough box, see `ui_react`'s `GLASS_OVERLAY_BOX_CLASS`)
/// presents `args` as a staged form. Submit dispatches `submit_action` with the merged effective
/// args; empty `args` degenerates to a message/confirm dialog. Opened only via
/// `Effect::OpenDialog`; the shell owns open/close as ephemeral chrome state, never the document.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DialogDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub body: Option<LocalizedLabel>,
    pub args: Vec<ActionArgDef>,
    /// 📇️ References an action owned by the active window kind, dispatched with merged args.
    pub submit_action: ActionRef,
    pub submit_label: LocalizedLabel,
    /// 📇️ Optional active-window action reference dispatched on any dismissal (Escape, veil
    /// click, or the Cancel button).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub cancel_action: Option<ActionRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub cancel_label: Option<LocalizedLabel>,
    /// 🔀️ Further decisions offered beside the submit, in focus order before it. Each is gated on and
    /// dispatches only the args it [`requires`](DialogChoice::requires) (plus the seed context), with
    /// [`DIALOG_CHOICE_ARG`] naming the choice, so one action can serve several choices
    /// (`historyEditCommit{choice}`); the submit alone is gated on the dialog's required args.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<DialogChoice>,
}

/// 🔀️ The arg key a [`DialogChoice`] dispatch carries its own `id` under; reserved in a dialog with choices.
pub const DIALOG_CHOICE_ARG: &str = "choice";

/// 🎛️ One decision button of a [`DialogDefinition`]: a localized `label`, an optional `description` a
/// reader hears with it (`aria-describedby`) and a sighted user reads beside it, the `action` it
/// dispatches, the staged args it `requires` (enabled once they resolve, and the only form args it
/// sends), its visual `tone`, and whether it is `destructive` (danger styling, and an agent lane must
/// ask before taking it).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DialogChoice {
    pub id: String,
    pub label: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedLabel>,
    pub action: ActionRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub requires: Vec<String>,
    #[serde(default, skip_serializing_if = "is_neutral_tone")]
    #[value(default, skip_serializing_if = "is_neutral_tone")]
    pub tone: semio_framework_ui_contract::Tone,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    #[value(default, skip_serializing_if = "std::ops::Not::not")]
    pub destructive: bool,
}

fn is_neutral_tone(tone: &semio_framework_ui_contract::Tone) -> bool {
    *tone == semio_framework_ui_contract::Tone::Neutral
}

impl DialogChoice {
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, action: ActionRef) -> Self {
        Self { id: id.into(), label: label.into(), description: None, action, requires: Vec::new(), tone: semio_framework_ui_contract::Tone::Neutral, destructive: false }
    }

    /// 🔗️ The staged args this choice consumes: it is enabled once they resolve and sends only them.
    pub fn requires(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.requires = args.into_iter().map(Into::into).collect();
        self
    }

    /// 🚧️ The args of [`requires`](Self::requires) still unresolved in `effective` (absent, blank, or off
    /// their current choice set) — the choice is enabled exactly when this is empty.
    pub fn unresolved_args(&self, defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
        let required: Vec<ActionArgDef> = defs.iter().filter(|def| self.requires.contains(&def.id)).map(|def| ActionArgDef { required: true, ..def.clone() }).collect();
        unresolved_action_args(&required, effective)
    }

    /// 📤️ The dispatch args: the seed context of `effective` (keys no arg of `defs` declares), the args this
    /// choice requires, and [`DIALOG_CHOICE_ARG`] naming it.
    pub fn dispatch_args(&self, defs: &[ActionArgDef], effective: &DslValue) -> DslValue {
        let mut entries = effective.as_object().map(<[_]>::to_vec).unwrap_or_default();
        entries.retain(|(key, _)| key != DIALOG_CHOICE_ARG && (self.requires.contains(key) || !defs.iter().any(|def| def.id == *key)));
        entries.push((DIALOG_CHOICE_ARG.to_string(), DslValue::String(self.id.clone())));
        DslValue::Object(entries)
    }

    /// 📝️ The consequence a reader hears with the button and a sighted user reads beside it.
    pub fn description(mut self, description: impl Into<LocalizedLabel>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 🎨️ The button's colour role.
    pub fn tone(mut self, tone: semio_framework_ui_contract::Tone) -> Self {
        self.tone = tone;
        self
    }

    /// ⚠️ Marks the choice destructive: danger styling, and an agent must ask before taking it.
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

// 🚫️async: E1 — pure builder methods (self-mutation only, zero suspension points), reverted
// per R9: `catch_unwind` sync-closure test consumers are language-barred from async.
impl DialogDefinition {
    pub fn new(id: impl Into<String>, title: impl Into<LocalizedLabel>, submit_action: ActionRef) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: None,
            args: Vec::new(),
            submit_action,
            // 🌐️ "OK" is identical in both locales — a real (not placeholder) translation choice.
            submit_label: LocalizedLabel::native("OK", "OK"),
            cancel_action: None,
            cancel_label: None,
            choices: Vec::new(),
        }
    }

    /// 🔀️ Appends one decision offered beside the submit.
    pub fn choice(mut self, choice: DialogChoice) -> Self {
        self.choices.push(choice);
        self
    }

    /// 🧐️ Choice ids are non-empty and unique, a choice requires only declared args, and no staged arg
    /// shadows [`DIALOG_CHOICE_ARG`].
    pub fn validate_choices(&self) -> Result<(), String> {
        let mut ids = std::collections::BTreeSet::new();
        for choice in &self.choices {
            if choice.id.trim().is_empty() {
                return Err("choice id must be non-empty".to_string());
            }
            if !ids.insert(choice.id.as_str()) {
                return Err(format!("duplicate choice id {}", choice.id));
            }
            if let Some(arg) = choice.requires.iter().find(|arg| !self.args.iter().any(|def| def.id == **arg)) {
                return Err(format!("choice {} requires undeclared arg {arg}", choice.id));
            }
        }
        if !self.choices.is_empty() && self.args.iter().any(|arg| arg.id == DIALOG_CHOICE_ARG) {
            return Err(format!("arg {DIALOG_CHOICE_ARG} is reserved in a dialog with choices"));
        }
        Ok(())
    }

    /// 📝️ Attaches explanatory body text shown below the title.
    pub fn body(mut self, body: impl Into<LocalizedLabel>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// 🧾️ Attaches the staged-form field declarations.
    pub fn args(mut self, args: Vec<ActionArgDef>) -> Self {
        self.args = args;
        self
    }

    /// ✅️ Overrides the submit button label (default "OK").
    pub fn submit_label(mut self, label: impl Into<LocalizedLabel>) -> Self {
        self.submit_label = label.into();
        self
    }

    /// ❌️ Overrides the cancel button label (default "Cancel", applied by the renderer).
    pub fn cancel_label(mut self, label: impl Into<LocalizedLabel>) -> Self {
        self.cancel_label = Some(label.into());
        self
    }

    /// 🚪️ Declares an action dispatched on any dismissal (Escape, veil click, Cancel button).
    pub fn on_cancel(mut self, action: ActionRef) -> Self {
        self.cancel_action = Some(action);
        self
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧪️dialog-choices/🦀️.rs"]
mod dialog_choices_tests;
//#endregion 🔖️Dialog

// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ModeDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    /// 🛠️ Tools available while this mode is active — references `AppDefinition.tools` ids.
    #[serde(default)]
    #[value(default)]
    pub tools: Vec<ToolRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub layout_id: Option<String>,
    /// 🎛️ Commands owned by this mode and active only while it is active.
    #[serde(default)]
    #[value(default)]
    pub commands: Vec<CommandDefinition>,
}

/// 🚫️ A non-empty, order-preserving list — construction-time enforcement replaces what used to be a
/// runtime `assert!` deep inside `AppBuilder::build_definition`. The first entry is the implicit
/// fallback default when nothing else specifies one.
#[derive(Clone, Debug, PartialEq)]
pub struct NonEmptyVec<T> {
    first: T,
    rest: Vec<T>,
}

// 🚫️async: E1 transitive block — every method here is a pure accessor/iterator with no I/O, and
// `iter`/`iter_mut` must feed directly into std `Iterator` combinators (`find`/`filter`/`map`, an
// external trait) at call sites, so the whole inherent impl stays sync rather than forcing an
// `.await` between every accessor and the combinator chain that consumes it (R9).
impl<T> NonEmptyVec<T> {
    pub fn one(first: T) -> Self {
        Self { first, rest: Vec::new() }
    }

    pub fn new(first: T, rest: Vec<T>) -> Self {
        Self { first, rest }
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn len(&self) -> usize {
        1 + self.rest.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.first).chain(self.rest.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        std::iter::once(&mut self.first).chain(self.rest.iter_mut())
    }

    pub fn first_mut(&mut self) -> &mut T {
        &mut self.first
    }

    pub fn push(&mut self, value: T) {
        self.rest.push(value);
    }
}

impl<T> std::ops::Index<usize> for NonEmptyVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index == 0 {
            &self.first
        } else {
            &self.rest[index - 1]
        }
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Chain<std::iter::Once<&'a T>, std::slice::Iter<'a, T>>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(&self.first).chain(self.rest.iter())
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = String;
    // 🚫️async: E1 impl of external `TryFrom`; pure list-shape check, no I/O.
    fn try_from(mut values: Vec<T>) -> Result<Self, Self::Error> {
        if values.is_empty() {
            return Err("expected a non-empty list, got zero entries".to_string());
        }
        let first = values.remove(0);
        Ok(Self { first, rest: values })
    }
}

impl<T: Clone> From<NonEmptyVec<T>> for Vec<T> {
    // 🚫️async: E1 impl of external `From`; pure collection reshape, no I/O.
    fn from(value: NonEmptyVec<T>) -> Self {
        std::iter::once(value.first).chain(value.rest).collect()
    }
}

/// 🌉️ Hand-written, not derived: `#[value(…)]` has no `into`/`try_from` equivalent — this mirrors
/// the retired `#[serde(try_from = "Vec<T>", into = "Vec<T>")]` bridge, wire-shaped as a plain
/// array exactly like `Vec<T>` (see `ToValue`/`FromValue for Vec<T>` in `🌱️value/🔁️codec/🦀️.rs`).
impl<T: ToValue> ToValue for NonEmptyVec<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }
}
impl<T: FromValue> FromValue for NonEmptyVec<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Array(items) = value else {
            return Err(ValueError::new(format!("expected an array, found {value:?}")));
        };
        let values = items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect::<Result<Vec<T>, ValueError>>()?;
        NonEmptyVec::try_from(values).map_err(ValueError::new)
    }
}

// 🚧️ BLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `AppDefinition`
// (`modes: Modes`/`window_kinds: WindowKinds`, both `NonEmptyVec<_>`) is itself serde-only — blocked
// on `ui_wgpu`/`interaction`/`🎠️kernel` field types elsewhere in this file — so `NonEmptyVec` must
// keep pace with it. Hand-written (not `#[serde(try_from/into = "Vec<T>")]`, since the derive is
// gone): mirrors the retired attribute via the existing `TryFrom<Vec<T>>`/`From<NonEmptyVec<T>> for
// Vec<T>` bridge above.
impl<T: Clone + Serialize> Serialize for NonEmptyVec<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let values: Vec<T> = self.clone().into();
        values.serialize(serializer)
    }
}
impl<'de, T: serde::de::DeserializeOwned> Deserialize<'de> for NonEmptyVec<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let values = Vec::<T>::deserialize(deserializer)?;
        NonEmptyVec::try_from(values).map_err(serde::de::Error::custom)
    }
}

/// 🚫️ Every app has at least one mode — `playbook/module/procedural` and any other single-purpose app
/// must declare an explicit mode (e.g. `"default"`) instead of the zero-mode state the type system
/// now makes unrepresentable.
pub type Modes = NonEmptyVec<ModeDefinition>;

/// 🚫️ Every app has at least one window kind — mirrors `Modes`, formerly a runtime `assert!` in
/// `AppBuilder::build_definition`.
pub type WindowKinds = NonEmptyVec<WindowKindDefinition>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WindowKindDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub body_key: String,
    pub surface_kind: SurfaceKind,
    pub icon_id: IconName,
    /// 🎛️ Always-present chrome facets (was: separately-optional `measures`/`engagement`).
    #[serde(default)]
    #[value(default)]
    pub options: WindowOptions,
    /// 📇️ Actions owned by this window kind. Mandatory, may be empty, never absent.
    #[serde(default)]
    #[value(default)]
    pub actions: Vec<ActionDefinition>,
    /// 🧰️ Utilities this window kind accepts — references `AppDefinition.utilities` ids. Empty = no utilities.
    #[serde(default)]
    #[value(default)]
    pub utilities: Vec<UtilityRef>,
    /// 🕹️ Interaction domains this window kind accepts — references `AppDefinition.interactions` ids.
    /// Empty = no interactions.
    #[serde(default)]
    #[value(default)]
    pub interactions: Vec<InteractionRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub params_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub artifact_snapshot_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub input_event_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub capabilities: Vec<kernel::CapabilityRequirement>,
}

// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum PanelGroup {
    Workbench,
    Details,
    Display,
    Settings,
}

impl PanelGroup {
    /// 🧭️ The dock anchor this group defaults to. Groups only ever map to the four corner anchors —
    /// the four edge-middle anchors (`top-middle`/`right-middle`/`bottom-middle`/`left-middle`) start
    /// empty and are user-populated via drag-and-drop or a dock skeleton override, never via a `PanelGroup`.
    pub fn anchor(&self) -> &'static str {
        match self {
            PanelGroup::Workbench => "top-left",
            PanelGroup::Details => "top-right",
            PanelGroup::Display => "bottom-left",
            PanelGroup::Settings => "bottom-right",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PanelGroup::Workbench => "workbench",
            PanelGroup::Details => "details",
            PanelGroup::Display => "display",
            PanelGroup::Settings => "settings",
        }
    }
}

/// 🌳️ Closes the informal `FRAMEWORK_CATEGORY_*`/`*_TAB_ID` string-constant convention that used to
/// live in the renderer: every panel tab is either a framework-predefined kind (compile-time
/// exhaustive) or an app-declared custom tab (open id, still required to be unique/non-empty,
/// validated at construction by `AppBuilder`).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", tag = "kind", content = "id")]
#[value(rename_all = "camelCase", tag = "kind", content = "id")]
pub enum PanelTabKind {
    WorkbenchCategory,
    DisplayCategory,
    DetailsCategory,
    SettingsCategory,
    DisplayWindows,
    DisplayLayout,
    SettingsGeneral,
    SettingsTheme,
    /// 🎯️ ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET C1/C5: the "Default apps" settings
    /// sub-tab — table of dialect × {viewer, editor} with selects, writes only through the `os.*`
    /// default-app commands (see 🔖️Surface below).
    SettingsDefaultApps,
    /// 🧩️ App-declared tab — id is app-namespaced (e.g. `"puzzle.catalogue"`).
    App(String),
}

impl PanelTabKind {
    /// 🔤️ Flat string key for code that needs one, e.g. React `key=` props.
    pub fn id_str(&self) -> &str {
        match self {
            PanelTabKind::WorkbenchCategory => "framework.category.workbench",
            PanelTabKind::DisplayCategory => "framework.category.display",
            PanelTabKind::DetailsCategory => "framework.category.details",
            PanelTabKind::SettingsCategory => "framework.category.settings",
            PanelTabKind::DisplayWindows => "framework.display.windows",
            PanelTabKind::DisplayLayout => "framework.display.layout",
            PanelTabKind::SettingsGeneral => "framework.settings.general",
            PanelTabKind::SettingsTheme => "framework.settings.theme",
            PanelTabKind::SettingsDefaultApps => "framework.settings.default-apps",
            PanelTabKind::App(id) => id.as_str(),
        }
    }
}

/// 🌳️ A leaf carries `body_key` (its rendered panel); a branch carries `children` (the tab row shown below it). Exactly one of the two is set; `group` is only meaningful on root (non-nested) entries.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct PanelTabDefinition {
    pub kind: PanelTabKind,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub group: PanelGroup,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub body_key: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub children: Vec<PanelTabDefinition>,
}

impl PanelTabDefinition {
    pub fn id(&self) -> &str {
        self.kind.id_str()
    }
}

//#region 🔖️Surface
/// 👁️✏️ Whether a surface may change the artifact it is bound to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AppRole {
    Viewer,
    Editor,
}

impl AppRole {
    /// 🔤️ Wire spelling — exactly `"viewer"`/`"editor"`, shared by serde, TS, JSON schema and the
    /// `SEMIO_APP_ROLE`/`VITE_SEMIO_APP_ROLE` env values.
    pub fn as_str(&self) -> &'static str {
        match self {
            AppRole::Viewer => "viewer",
            AppRole::Editor => "editor",
        }
    }
}

impl std::str::FromStr for AppRole {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "viewer" => Ok(AppRole::Viewer),
            "editor" => Ok(AppRole::Editor),
            other => Err(format!("unknown app role {other:?}, expected \"viewer\" or \"editor\"")),
        }
    }
}

/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]`'s enum path only supports
/// internally-tagged representations, but `AppRole` is a plain unit-only "string enum" — serde's
/// own default (untagged bare-string) representation. Delegates to the existing `as_str`/`FromStr`
/// so the wire spelling never drifts from the serde/TS/JSON-schema one.
impl ToValue for AppRole {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.as_str().to_string())
    }
}
impl FromValue for AppRole {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => s.parse::<AppRole>().map_err(ValueError::new),
            other => Err(ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

// 🚧️ Needed in serde form too: referenced by a `🚧️ BLOCKED` serde-only manifest type — hand-written
// (not derived, same reasoning as the `ToValue`/`FromValue` pair above) via the same `as_str`/
// `FromStr` delegation.
impl Serialize for AppRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AppRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<AppRole>().map_err(serde::de::Error::custom)
    }
}

/// 🎯️ A surface addressed across plugin boundaries.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AppRef {
    pub plugin_id: String,
    pub app_id: String,
}

/// 🪪️ The one canonical spelling of a surface id: `<artifact_kind>@<standard>/<subset>#<role>`.
pub fn surface_app_id(dialect: &ArtifactDialect, role: AppRole) -> String {
    format!("{}#{}", dialect.to_coordinate(), role.as_str())
}

/// 🪪️ Inverse of `surface_app_id`; rejects anything not matching the grammar.
pub fn parse_surface_app_id(id: &str) -> Result<(ArtifactDialect, AppRole), String> {
    let (coordinate, role_str) = id.rsplit_once('#').ok_or_else(|| format!("surface id {id:?} missing '#'"))?;
    let dialect = ArtifactDialect::parse_coordinate(coordinate)?;
    let role: AppRole = role_str.parse().map_err(|err| format!("surface id {id:?}: {err}"))?;
    Ok((dialect, role))
}
//#endregion 🔖️Surface

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppDefinition {
    pub id: String,
    /// 👁️✏️ Whether this surface may mutate the artifact it is bound to — see `AppRole`.
    pub role: AppRole,
    /// 🎯️ The dialect coordinate (artifact kind, standard, subset) this surface is bound to — see
    /// `ArtifactDialect`. Together with `role` this derives the canonical `id` via `surface_app_id`.
    pub dialect: ArtifactDialect,
    /// 🗣️ The app's own display name (e.g. "Puzzle 3D") — manifest-level, locale×terminology-checked,
    /// see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub breadcrumb: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<IconName>,
    pub controller_id: String,
    /// 🚧️ `Modes` is `NonEmptyVec<ModeDefinition>`, whose `serde(try_from/into = "Vec<T>")` wire
    /// format is a flat array — not the `{ first, rest }` shape owned schema exporter would infer from the struct
    /// fields, so the wire-accurate array shape is supplied directly instead of deriving `TS` on
    /// `NonEmptyVec` itself.
    pub modes: Modes,
    pub default_mode_id: String,
    /// 🚧️ See `modes` above — `WindowKinds` is `NonEmptyVec<WindowKindDefinition>`.
    pub window_kinds: WindowKinds,
    /// 🕹️ The app-wide action roster: every action dispatchable in ANY window of this app that no
    /// window kind claims as its own, including the framework-injected History/Clipboard/tutorial
    /// constants. A window kind's dispatchable set is `WindowKindDefinition.actions` followed by
    /// this roster minus the ids some window claims — [`resolve_window_actions`], whose TypeScript
    /// twin is `resolveWindowActions` (`🎯️action-bus/🟦️.ts`). Kept here rather than copied into
    /// every window kind because a copy made the package descriptor grow as
    /// `apps × window kinds × actions`: 21 distinct rows were stored 675 times in one plugin, and
    /// the duplicates were 31.9 % of every shipped descriptor's bytes (ticket
    /// 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, slice DS1).
    #[serde(default)]
    #[value(default)]
    pub actions: Vec<ActionDefinition>,
    pub panel_tabs: Vec<PanelTabDefinition>,
    pub keybindings: Vec<Keybinding>,
    /// 🧰️ The interactive utilities this app exposes (referenced by `WindowKindDefinition.utilities`).
    #[serde(default)]
    #[value(default)]
    pub utilities: Vec<UtilityDefinition>,
    /// 🛠️ The mode-level tools this app exposes (referenced by `ModeDefinition.tools`).
    #[serde(default)]
    #[value(default)]
    pub tools: Vec<ToolDefinition>,
    /// 🎛️ Commands owned by this app and active whenever it is focused.
    #[serde(default)]
    #[value(default)]
    pub commands: Vec<CommandDefinition>,
    /// 🕹️ The interaction domains (hover + selection) this app exposes (referenced by
    /// `WindowKindDefinition.interactions`) — see `crate::InteractionDefinition`.
    #[serde(default)]
    #[value(default)]
    pub interactions: Vec<InteractionDefinition>,
    #[serde(default)]
    #[value(default)]
    pub named_layouts: Vec<NamedLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub default_layout: Option<WindowLayout>,
    /// 🗣️ Terminology ids this app declares beyond the implicit "native" default.
    #[serde(default)]
    #[value(default)]
    pub terminologies: Vec<String>,
    /// 🗺️ Terminology id -> full replacement breadcrumb (product + app segments), e.g. "reuse" ->
    /// ["Entwerfen mit Bestand", "Aggregator"]; ids absent here keep the canonical breadcrumb under that terminology.
    #[serde(default)]
    #[value(default)]
    pub terminology_breadcrumbs: std::collections::HashMap<String, Vec<String>>,
    /// 🎓️ This app's first-run walkthrough, if it declares one — see `IntroductionDefinition`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub introduction: Option<IntroductionDefinition>,
    /// 🎬️ Recorded, timed walkthroughs this app declares — see `TutorialDefinition`. A brand's own
    /// `tutorials` (if any) are shown alongside these, never replacing them (unlike `introduction`).
    #[serde(default)]
    #[value(default)]
    pub tutorials: Vec<TutorialDefinition>,
    /// 🗨️ The modal form dialogs this app can open via `Effect::OpenDialog`.
    #[serde(default)]
    #[value(default)]
    pub dialogs: Vec<DialogDefinition>,
    /// 🔌️ This app's workflow input ports — see `crate::MediaPortSpec`.
    #[serde(default)]
    #[value(default)]
    pub media_inputs: Vec<MediaPortSpec>,
    /// 🔌️ This app's workflow output ports — see `crate::MediaPortSpec`.
    #[serde(default)]
    #[value(default)]
    pub media_outputs: Vec<MediaPortSpec>,
    /// 🗂️ OS resource kinds this app produces/consumes — see `crate::ArtifactKindSpec`. Drives
    /// `framework/product/os/core`'s artifact catalog registry instead of a hardcoded per-app match.
    #[serde(default)]
    #[value(default)]
    pub artifact_kinds: Vec<ArtifactKindSpec>,
    /// 🧮️ This app's typed configuration record — see `crate::ConfigSpec`. Empty until per-app waves
    /// populate it.
    #[serde(default)]
    #[value(default)]
    pub config: ConfigSpec,
    /// 🎛️ This app's typed binary command grammar — see `crate::CommandGrammar`. Empty until per-app
    /// waves populate it.
    #[serde(default)]
    #[value(default)]
    pub command_grammar: CommandGrammar,
    /// 🔌️ This app's typed media I/O surface — see `crate::AppIo`. Not yet populated; `media_inputs`/
    /// `media_outputs`/`artifact_kinds` above remain the live source of truth until later waves migrate
    /// onto this.
    #[serde(default)]
    #[value(default)]
    pub io: AppIo,
}

/// 🧭️ Resolves the dock layout a mode should present.
pub fn resolve_layout_for_mode(app: &AppDefinition, mode_id: &str) -> Option<WindowLayout> {
    let mode = app.modes.iter().find(|mode| mode.id == mode_id)?;
    if let Some(layout_id) = &mode.layout_id {
        if let Some(named) = app.named_layouts.iter().find(|entry| entry.id == *layout_id) {
            return Some(named.layout.clone());
        }
    }
    app.default_layout.clone()
}

//#region 🔖️action-args
/// 🧮️ Computes the effective argument map for an action: for each declared arg, the staged value
/// if present, else its declared `default`, else omitted. Renderers stage edits locally and pass them
/// here; the contract enforcer ({@link VcsArtifactApp}) materializes defaults before dispatch so plugins
/// never re-implement default-filling.
///
/// 🌱️ `seed` carries a dialog's pre-seeded context args (e.g. a row-scoped `spaceId` that is never a
/// declared, editable form field, per `Effect::OpenDialog { args }`) through untouched: any `seed`
/// key that is not a declared arg id survives into the result unmodified, and a `seed` value for a
/// declared id that hasn't been staged yet acts as that field's initial value. A dialog with zero
/// declared `defs` (a plain confirm/cancel, e.g. `deleteSpace`) passes `seed`+`staged` through
/// wholesale — TS twin: {@link effectiveActionArgs} (`🧩️action-argument-resolution/🟦️.ts`).
pub fn effective_action_args(defs: &[ActionArgDef], staged: &DslValue, seed: Option<&DslValue>) -> DslValue {
    let seed_pairs: Vec<(String, DslValue)> = seed.and_then(DslValue::as_object).map(<[_]>::to_vec).unwrap_or_default();
    if defs.is_empty() {
        let mut effective = seed_pairs;
        if let Some(staged_pairs) = staged.as_object() {
            for (key, value) in staged_pairs {
                if let Some(existing) = effective.iter_mut().find(|(k, _)| k == key) {
                    existing.1 = value.clone();
                } else {
                    effective.push((key.clone(), value.clone()));
                }
            }
        }
        return DslValue::Object(effective);
    }
    let mut effective = seed_pairs;
    for def in defs {
        if let Some(value) = staged.get(&def.id) {
            if let Some(existing) = effective.iter_mut().find(|(k, _)| *k == def.id) {
                existing.1 = value.clone();
            } else {
                effective.push((def.id.clone(), value.clone()));
            }
        } else if effective.iter().any(|(k, _)| *k == def.id) {
            // 🌱️ seeded value already present for this declared field — keep it as the pre-fill.
        } else if let Some(default) = &def.default {
            effective.push((def.id.clone(), default.clone()));
        }
    }
    DslValue::Object(effective)
}

/// ❗️ Returns the ids of required args that are still unset in `effective`. "Unset" means absent,
/// `Null`, or an empty string (covers a blank Text/Select/IconSelect/ArtifactKind/SurfaceApp — the
/// latter two resolve to a `String` effective value exactly like `Select`, contract §C8.1); `false`,
/// `0`, and `[]` are valid values for Toggle/Number/Slider/Vec3 and never count as unset.
/// `Any` requires presence, allowing null and empty strings; a string with `min_len: Some(0)`
/// explicitly permits an empty value, such as document text or a root JSON pointer.
pub fn missing_required_args(defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
    defs.iter()
        .filter(|def| def.required)
        .filter(|def| match effective.get(&def.id) {
            None => true,
            Some(DslValue::Null) => !matches!(def.schema, ArgSchema::Any),
            Some(DslValue::String(text)) => text.is_empty() && !matches!(def.schema, ArgSchema::Any | ArgSchema::String { min_len: Some(0), .. }),
            Some(_) => false,
        })
        .map(|def| def.id.clone())
        .collect()
}

/// 🔽️ Closed choices must belong to their current host-resolved option set.
pub fn action_arg_requires_choice(def: &ActionArgDef) -> bool {
    matches!(&def.schema, ArgSchema::String { options, format, .. } if !options.is_empty() || matches!(format, Some(ArgFormat::ArtifactKind { .. } | ArgFormat::SurfaceApp { .. })))
}

/// 🚫️ Returns supplied choices absent from the current catalog, including an unavailable catalog.
pub fn invalid_action_choice_args(defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
    defs.iter().filter(|def| {
        let Some(value) = effective.get(&def.id) else { return false; };
        if matches!(value, DslValue::Null) || value.as_str() == Some("") || !action_arg_requires_choice(def) { return false; }
        matches!(&def.schema, ArgSchema::String { options, .. } if !options.iter().any(|option| value.as_str() == Some(option.value.as_str())))
    }).map(|def| def.id.clone()).collect()
}

/// 🛑️ Gates staged submission on required presence and exact current choice membership.
pub fn unresolved_action_args(defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
    let unresolved: std::collections::HashSet<String> = missing_required_args(defs, effective).into_iter().chain(invalid_action_choice_args(defs, effective)).collect();
    defs.iter().filter(|def| unresolved.contains(&def.id)).map(|def| def.id.clone()).collect()
}

/// 🚦️ Whether an action is eligible to appear in a window's Actions panel — excludes the six
/// framework History actions (rendered by the History rail) and the injected `setActiveUtility`/
/// `setActiveTool` (internal View actions wired to the utility bar/tool panel, never the panel).
// 🚫️async: E1 transitive — the only consumer is `Iterator::filter` (external trait), which takes a
// sync closure; pure comparison, no I/O (R9).
fn action_is_panel_eligible(action: &ActionDefinition) -> bool {
    action.kind != ActionKind::History && action.id != SET_ACTIVE_UTILITY_ACTION_ID && action.id != SET_ACTIVE_TOOL_ACTION_ID
}

/// 🕹️ THE window action predicate — every [`ActionDefinition`] dispatchable in `window`, in
/// the order a shell must offer them: the window kind's OWN roster first, then every
/// [`AppDefinition::actions`] row that no window kind of this app claims for itself. An id declared
/// by ANY window kind is that window's authored declaration and is never re-offered from the app
/// roster. The plugin builder used to bake this union into every window kind by cloning, which made
/// a package descriptor grow as `apps × window kinds × actions` — see `AppDefinition::actions`.
/// TypeScript twin: `resolveWindowActions` (`🎯️action-bus/🟦️.ts`).
pub fn window_kind_actions<'a>(app: &'a AppDefinition, window: &'a WindowKindDefinition) -> Vec<&'a ActionDefinition> {
    let claimed: std::collections::BTreeSet<&str> = app.window_kinds.iter().flat_map(|kind| kind.actions.iter().map(|action| action.id.as_str())).collect();
    let mut seen: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut resolved: Vec<&'a ActionDefinition> = Vec::with_capacity(window.actions.len() + app.actions.len());
    for action in window.actions.iter().chain(app.actions.iter().filter(|action| !claimed.contains(action.id.as_str()))) {
        if seen.insert(action.id.as_str()) {
            resolved.push(action);
        }
    }
    resolved
}

/// 📇️ Resolves the actions a window kind presents in its panel — [`window_kind_actions`]
/// minus the framework-only rail actions (the six History verbs and the injected
/// `setActiveUtility`/`setActiveTool`), preserving declaration order.
pub fn resolve_window_actions<'a>(app: &'a AppDefinition, window_kind: &'a WindowKindDefinition) -> Vec<&'a ActionDefinition> {
    window_kind_actions(app, window_kind).into_iter().filter(|action| action_is_panel_eligible(action)).collect()
}

/// 🛠️ Resolves the tools the active mode presents, in declared order — references into
/// `AppDefinition.tools` via `ModeDefinition.tools`. Unlike `resolve_window_actions`, unresolvable or
/// unreferenced tools have no orphan fallback: tools are opt-in per mode, not automatically shown
/// everywhere. Unresolvable refs are skipped (the builder validates them at construction time).
pub fn resolve_mode_tools<'a>(app: &'a AppDefinition, mode_id: &str) -> Vec<&'a ToolDefinition> {
    let Some(mode) = app.modes.iter().find(|mode| mode.id == mode_id) else {
        return Vec::new();
    };
    let mut resolved: Vec<&'a ToolDefinition> = Vec::new();
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for tool_ref in &mode.tools {
        if let Some(tool) = app.tools.iter().find(|tool| tool.id == tool_ref.as_str()) {
            if seen.insert(tool.id.as_str()) {
                resolved.push(tool);
            }
        }
    }
    resolved
}
//#endregion 🔖️action-args

/// 🪜️ Formats a canonical app breadcrumb for chrome.
pub fn app_breadcrumb(breadcrumb: &[String]) -> String {
    breadcrumb.join(" · ")
}

/// 🗺️ Resolves the breadcrumb effective under the active terminology; unknown/native ids fall back to the canonical breadcrumb.
pub fn resolve_app_breadcrumb<'a>(app: &'a AppDefinition, terminology: &str) -> &'a [String] {
    app.terminology_breadcrumbs.get(terminology).map_or(&app.breadcrumb, Vec::as_slice)
}

/// 🗂️ Formats a window tab within its canonical app breadcrumb, resolved under the active terminology
/// and `locale` (needed to resolve the now-`LocalizedLabel` `app.label` for the dedup comparison below).
pub fn app_window_label(app: &AppDefinition, terminology: &str, locale: Locale, window_label: &str) -> String {
    let mut breadcrumb = resolve_app_breadcrumb(app, terminology).to_vec();
    let normalized_window = window_label.trim().to_lowercase();
    let normalized_app = app.label.resolve(Terminology::parse(terminology).unwrap_or_default(), locale).trim().to_lowercase();
    if !normalized_window.is_empty() && normalized_window != normalized_app && breadcrumb.last().is_none_or(|segment| segment.to_lowercase() != normalized_window) {
        breadcrumb.push(normalized_window);
    }
    app_breadcrumb(&breadcrumb)
}

// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ExampleDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    pub artifact_json: String,
    /// 🎯️ The DIALECT this example is authored for, never one app of it. An example is a document of
    /// an artifact's subset, and every surface bound to that subset — the editor and the viewer
    /// alike — opens the same eight fixtures; `SubsetDeclaration.examples` already models them that
    /// way. Stamped at registration from the registering app's own `AppDefinition.dialect`, and
    /// resolved by [`examples_for_app`] (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub dialect: ArtifactDialect,
}

/// 🏷️ One asset-name component: everything outside `[0-9A-Za-z._-]` folds to `-` so a dialect
/// coordinate (`s.puzzle.5d@1/*`) and an example id yield a single stable path segment.
pub fn asset_name_segment(raw: &str) -> String {
    raw.chars().map(|character| if character.is_ascii_alphanumeric() || character == '.' || character == '_' || character == '-' { character } else { '-' }).collect()
}

/// 📦️ The directory-and-stem prefix every externalized example body of `example_id` travels under.
/// The suffix after it names the BYTES that were declared: `.json` for a body the descriptor
/// emitter moved out of the manifest, and the authored fixture's own extension for a body the
/// plugin never materialised at all (see `ExampleSource::deferred`). Consumers match the prefix,
/// never one fixed extension, because those two declarations hash different bytes on purpose.
pub fn example_body_asset_prefix(dialect: &ArtifactDialect, example_id: &str) -> String {
    format!(
        "📚️examples/{}.{}.{}/{}",
        asset_name_segment(&dialect.artifact_kind),
        asset_name_segment(&dialect.standard),
        asset_name_segment(&dialect.subset),
        asset_name_segment(example_id)
    )
}

/// 📦️ The full `AssetDeclaration.name` for one example body — [`example_body_asset_prefix`] plus the
/// suffix of the bytes actually declared (`.json`, `.dsl.semio`, …).
pub fn example_body_asset_name(dialect: &ArtifactDialect, example_id: &str, suffix: &str) -> String {
    format!("{}{}", example_body_asset_prefix(dialect, example_id), suffix)
}

/// 📚️ THE example-picker predicate — every example authored for `dialect`, in manifest order,
/// deduplicated by id. The react shell's navbar select (`ShellHost`'s `exampleOptions`) answers it,
/// and so does the TypeScript twin `examplesForDialect` (`🛂️manifest/🟦️.ts`); both are pinned against the shared
/// `🧫️fixtures/📚️example-picker.json` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn examples_for_dialect<'a>(examples: &'a [ExampleDefinition], dialect: &ArtifactDialect) -> Vec<&'a ExampleDefinition> {
    let mut seen = std::collections::BTreeSet::new();
    examples.iter().filter(|example| &example.dialect == dialect).filter(|example| seen.insert(example.id.clone())).collect()
}

/// 📚️ The examples one surface may offer — [`examples_for_dialect`] against that surface's own
/// dialect. Role plays no part: an editor and its viewer are two surfaces of ONE dialect and offer
/// exactly the same picker.
pub fn examples_for_app<'a>(examples: &'a [ExampleDefinition], app: &AppDefinition) -> Vec<&'a ExampleDefinition> {
    examples_for_dialect(examples, &app.dialect)
}

/// 🧩️ One host-aggregated plugin contribution entry — the element shape of the paged
/// `setContributions` command payload, never a view-state field.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ProgramContributionEntry {
    pub plugin_id: String,
    #[value(default)]
    pub topic_contribution: Option<TopicContribution>,
}

/// 📕️ Parses one assembled `setContributions` payload into typed entries — the ONE route
/// contributions reach a guest by.
pub fn parse_contributions(json: &str) -> Vec<ProgramContributionEntry> {
    dsl::os_pack::json::from_json_str(json).unwrap_or_default()
}

//#region 🔖️TopicContribution
/// 🗂️ Open contribution shape: a plugin declares a `topic` string instead of a hardcoded enum variant,
/// so the generic framework never has to know plugin-specific names. `topic` reuses the same
/// dot-namespaced vocabulary as a crate's existing `contributes`/`consumes` metadata (e.g.
/// `"flow.extension"`, `"playbook.blockKind"`, `"cad.computer"`) — each producer/consumer picks its
/// own topic string; this type does not enumerate them. See `component.ts`'s `TopicContribution` for
/// the mirror.
#[derive(Clone, Debug, PartialEq)]
pub struct TopicContribution {
    pub topic: String,
    pub payload: DslValue,
}

impl TopicContribution {
    pub fn new(topic: impl Into<String>, payload: DslValue) -> Self {
        Self { topic: topic.into(), payload }
    }

    /// 📕️ Decodes `payload` into a caller-chosen typed shape.
    pub fn decode<T: FromValue>(&self) -> Result<T, ValueError> {
        T::from_value(self.payload.clone())
    }
}

/// 🌉️ Hand-written, not derived (avoids a new `semio-framework-value-derive` dependency edge on
/// this file's two host crates — this shape is a two-field plain record, trivial either way):
/// `topic`/`payload` mirror the `#[serde(rename_all = "camelCase")]` wire shape exactly.
impl ToValue for TopicContribution {
    fn to_value(&self) -> DslValue {
        DslValue::object([("topic".to_string(), ToValue::to_value(&self.topic)), ("payload".to_string(), self.payload.clone())])
    }
}
impl FromValue for TopicContribution {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(fields) = value else {
            return Err(ValueError::new(format!("expected an object for TopicContribution, found {value:?}")));
        };
        let mut topic = None;
        let mut payload = None;
        for (key, entry) in fields {
            match key.as_str() {
                "topic" => topic = Some(String::from_value(entry).map_err(|e| e.under("topic"))?),
                "payload" => payload = Some(entry),
                _ => {}
            }
        }
        Ok(TopicContribution {
            topic: topic.ok_or_else(|| ValueError::new("TopicContribution missing topic"))?,
            payload: payload.ok_or_else(|| ValueError::new("TopicContribution missing payload"))?,
        })
    }
}

// 🚧️ Needed in serde form too: referenced by a `🚧️ BLOCKED` serde-only manifest type — hand-written
// (not derived, same reasoning as the `ToValue`/`FromValue` pair above), same `topic`/`payload`
// camelCase wire shape.
impl Serialize for TopicContribution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("TopicContribution", 2)?;
        state.serialize_field("topic", &self.topic)?;
        state.serialize_field("payload", &self.payload)?;
        state.end()
    }
}
impl<'de> Deserialize<'de> for TopicContribution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Raw {
            topic: String,
            payload: DslValue,
        }
        let raw = Raw::deserialize(deserializer)?;
        Ok(TopicContribution { topic: raw.topic, payload: raw.payload })
    }
}
//#endregion 🔖️TopicContribution

//#region 🔖️PluginDependency
/// 🔢️ A frozen `major.minor.patch` version triple — no external semver crate (contract freeze
/// `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` §3). `Ord` is
/// derived field-in-order (major, then minor, then patch), which is exactly semver precedence.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

/// 🌉️ Hand-written, not derived: `#[value(…)]` has no `into`/`try_from` equivalent — this mirrors
/// the retired `#[serde(into = "String", try_from = "String")]` bridge via the existing
/// `Display`/`FromStr` impls below.
impl ToValue for Version {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.to_string())
    }
}
impl FromValue for Version {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => Version::parse(&s).map_err(|e| ValueError::new(e.to_string())),
            other => Err(ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🚧️ Failure parsing a `Version` (`major.minor.patch`, all-numeric segments) or a `VersionPin`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionParseError {
    Malformed(String),
    NonNumeric(String, String),
}

impl std::fmt::Display for VersionParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malformed(input) => write!(formatter, "expected `major.minor.patch`, got {input:?}"),
            Self::NonNumeric(input, segment) => write!(formatter, "non-numeric version segment {segment:?} in {input:?}"),
        }
    }
}

impl std::error::Error for VersionParseError {}

impl Version {
    // 🚫️async: E1 transitive — `TryFrom<String>`/`FromStr` (external) construct this synchronously;
    // pure field assembly, no I/O (R9).
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self { major, minor, patch }
    }

    /// 🔢️ Parses a strict `major.minor.patch` triple — no pre-release/build metadata, no leniency.
    // 🚫️async: E1 transitive — `FromStr::from_str`/`TryFrom<String>` (external) call this
    // synchronously; pure string parsing, no I/O (R9).
    pub fn parse(input: &str) -> Result<Self, VersionParseError> {
        let mut segments = input.split('.');
        let (Some(major), Some(minor), Some(patch), None) = (segments.next(), segments.next(), segments.next(), segments.next()) else {
            return Err(VersionParseError::Malformed(input.to_string()));
        };
        let segment = |raw: &str| raw.parse::<u64>().map_err(|_| VersionParseError::NonNumeric(input.to_string(), raw.to_string()));
        Ok(Self { major: segment(major)?, minor: segment(minor)?, patch: segment(patch)? })
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl std::str::FromStr for Version {
    type Err = VersionParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Version::parse(input)
    }
}

impl From<Version> for String {
    fn from(version: Version) -> Self {
        version.to_string()
    }
}

impl TryFrom<String> for Version {
    type Error = VersionParseError;
    fn try_from(raw: String) -> Result<Self, Self::Error> {
        Version::parse(&raw)
    }
}

/// 📌️ A dependency pin: the exact version (`=X.Y.Z`) of the plugin a manifest depends on. One tree is one catalog, and a
/// trusted catalog admits only exact pins inside its own closure (`trustedBootstrapDescriptorClaims`,
/// `🌎️hub/📦️packages/🦀️rust/📜️script.ts`), so a manifest carries no range grammar: a declaration pins the version its
/// own tree builds, through [`tree_pin!`](crate::tree_pin).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VersionPin(pub Version);

impl VersionPin {
    /// 🔢️ Parses the one wire form, `=X.Y.Z`; every range (`*`, `^`, `~`, `>=`, a bare triple) is refused.
    // 🚫️async: E1 transitive — `FromValue::from_value` (this file, below) calls this synchronously; pure string
    // parsing, no I/O (R9).
    pub fn parse(input: &str) -> Result<Self, VersionPinParseError> {
        let trimmed = input.trim();
        let Some(rest) = trimmed.strip_prefix('=') else { return Err(VersionPinParseError::NotExact(trimmed.to_string())) };
        Ok(Self(Version::parse(rest)?))
    }

    /// 📌️ Pins the version a crate of this tree is compiled at. [`tree_pin!`](crate::tree_pin) evaluates it in a
    /// `const` over the declaring crate's own `CARGO_PKG_VERSION`, so a version that is not a strict
    /// `major.minor.patch` triple is a compile error in that crate, never a guest trap.
    pub const fn of_tree(crate_version: &str) -> Self {
        let bytes = crate_version.as_bytes();
        let mut segments = [0u64; 3];
        let mut segment = 0;
        let mut digits = 0;
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'.' {
                if digits == 0 || segment == 2 {
                    panic!("the compiled crate version is not a strict major.minor.patch triple");
                }
                segment += 1;
                digits = 0;
            } else if byte.is_ascii_digit() {
                segments[segment] = segments[segment] * 10 + (byte - b'0') as u64;
                digits += 1;
            } else {
                panic!("the compiled crate version is not a strict major.minor.patch triple");
            }
            index += 1;
        }
        if segment != 2 || digits == 0 {
            panic!("the compiled crate version is not a strict major.minor.patch triple");
        }
        Self(Version { major: segments[0], minor: segments[1], patch: segments[2] })
    }

    /// ✅️ Whether `version` is exactly the pinned version.
    // 🚫️async: E1 transitive — pure comparison consumed by `matches_raw`, itself required sync (R9).
    pub fn matches(&self, version: &Version) -> bool {
        &self.0 == version
    }

    /// ✅️ Convenience for the dependency graph: parses `raw` and compares; an unparsable target version never matches.
    // 🚫️async: E1 transitive — dependency-graph validation calls this synchronously via `!`; pure parse-and-compare,
    // no I/O (R9).
    pub fn matches_raw(&self, raw: &str) -> bool {
        Version::parse(raw).is_ok_and(|version| self.matches(&version))
    }
}

impl std::fmt::Display for VersionPin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "={}", self.0)
    }
}

/// 🚧️ Failure parsing a `VersionPin`: anything but `=X.Y.Z` (a range such as `*`, `^`, `~` or `>=` included).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionPinParseError {
    NotExact(String),
    Version(VersionParseError),
}

impl std::fmt::Display for VersionPinParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotExact(input) => write!(formatter, "a dependency pins an exact version `=X.Y.Z`, got {input:?}"),
            Self::Version(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for VersionPinParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Version(error) => Some(error),
            Self::NotExact(_) => None,
        }
    }
}

impl From<VersionParseError> for VersionPinParseError {
    fn from(error: VersionParseError) -> Self {
        Self::Version(error)
    }
}

impl ToValue for VersionPin {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.to_string())
    }
}
impl FromValue for VersionPin {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => VersionPin::parse(&s).map_err(|e| ValueError::new(e.to_string())),
            other => Err(ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only
// manifest type above/below (`PluginDependency.version`) — hand-written, same reasoning as the
// `ToValue`/`FromValue` pair above.
impl Serialize for VersionPin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for VersionPin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        VersionPin::parse(&raw).map_err(serde::de::Error::custom)
    }
}

/// 📌️ The exact pin of the tree the invoking crate is compiled from: `semio_framework::tree_pin!()` expands in the
/// declaring crate, so `CARGO_PKG_VERSION` is that crate's own workspace version, checked at compile time.
#[macro_export]
macro_rules! tree_pin {
    () => {{
        const TREE_PIN: $crate::VersionPin = $crate::VersionPin::of_tree(env!("CARGO_PKG_VERSION"));
        TREE_PIN
    }};
}

/// 🔗️ One direct plugin dependency: the depended-on plugin id plus the exact version it pins — see
/// `resolve_load_order`/`validate_dependency_graph`.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct PluginDependency {
    pub plugin_id: String,
    pub version: VersionPin,
}

impl PluginDependency {
    pub fn new(plugin_id: impl Into<String>, version: VersionPin) -> Self {
        Self { plugin_id: plugin_id.into(), version }
    }
}
//#endregion 🔖️PluginDependency

//#region 🔖️ArtifactContribution
/// 🗂️ The `verb`/`entity`/`kind`/`record` semantic identity of one contributed mutation, carried as
/// owned strings on the wire (the native `SemanticDescriptor` this mirrors lives in the os-kernel
/// protocol crate, which `semio-framework` must not require plugin manifests to link against).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContributedMutationSemantics {
    pub verb: String,
    pub entity: String,
    pub kind: String,
    pub record: String,
}

/// 🗂️ One mutation a plugin contributes onto an artifact kind it depends on — the manifest-declared
/// counterpart of a `contributor.list-artifact-mutations` roster entry (contract freeze §3/§6).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContributedMutationMetadata {
    /// 🪪️ `"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"` (contract freeze §3).
    pub mutation_id: String,
    pub semantics: ContributedMutationSemantics,
    pub schema_version: u32,
    pub algorithm_version: u32,
}

/// 💡️ One inference a plugin contributes onto an artifact kind it depends on — mirrors the native
/// `ArtifactInferenceServiceMetadata` fields (owned strings instead of `&'static str`, since this
/// travels over the wire in a manifest), plus `contributor`/`depends_on` for the contribution's own
/// identity and ordering (contract freeze §4: `owner == contributor`, `artifact_kind == target`).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContributedInferenceMetadata {
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub contributor: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    /// 📜️ The PUBLISHED request/response contract of this inference — what a generic client has to
    /// send and what it gets back. Absent for an inference whose owner has not declared one: a
    /// gateway that cannot read a contract refuses the call by name instead of dispatching a body
    /// the guest will reject after minutes of guest time (`📓️pz2-puzzle-describe-under-budget.md`
    /// §5.2 measured exactly that: `inference_run` on `s.wfc.bitmap.solve` never answered inside
    /// 240 s because nothing in the tree said the payload needed a `snapshot`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<InferencePayloadContract>,
}

/// 📜️ One inference's published payload contract — the schema-first answer to "what do I send?".
/// `input_schema`/`output_schema` are JSON Schema 2020-12 documents as TEXT, authored beside the
/// inference and carried verbatim, so the gateway validates against the plugin's own declaration
/// instead of a host-side guess. `progress_unit` names what the bounded job counts, so a client can
/// label a progress bar without knowing the algorithm.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InferencePayloadContract {
    pub payload_schema_id: String,
    pub input_schema: String,
    pub output_schema: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub progress_unit: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub artifact_binding: Option<InferenceArtifactBinding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<InferenceCommitBinding>,
}

/// 📌️ How one inference's result becomes a document edit: the artifact action that commits it. The
/// action receives the result's fields it declares as arguments, and runs through the ordinary edit
/// path — its own policy, undo and ledger — so a committed result is an edit like any other.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InferenceCommitBinding {
    pub action: String,
}

/// 🔗️ How one inference's canonical request is BOUND to an artifact document. An inference over an
/// artifact cannot be expressed by a client at all — nobody can type 4 096 bitmap cells into a tool
/// call — so the plugin declares the payload field its own document goes into and the host fills it
/// from the artifact the caller named. Declared, never invented: the host writes exactly the field
/// this row names, in exactly the encoding it names, and refuses anything else by name.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InferenceArtifactBinding {
    pub field: String,
    pub encoding: String,
    pub required: bool,
}

/// 📦️ The ONE encoding an artifact binding may name today: the artifact's canonical `pack`/`spr`
/// pair, each base64, as `{ "pack": "…", "spr": "…" }` under the declared field. A binding naming
/// anything else is refused at the gateway rather than silently filled with a shape the guest
/// cannot read.
pub const INFERENCE_ARTIFACT_PACK_BASE64: &str = "artifact-pack-base64";

/// 🗂️ Everything one plugin contributes onto one artifact kind it depends on — see the registration
/// gates in contract freeze §4 (accepted only when `artifact_kind`'s owner is a direct
/// `PluginManifest.dependencies` entry).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ArtifactContributionDescriptor {
    pub artifact_kind: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mutations: Vec<ContributedMutationMetadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub inferences: Vec<ContributedInferenceMetadata>,
}
//#endregion 🔖️ArtifactContribution

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginManifest {
    pub plugin_id: String,
    pub label: String,
    pub version: String,
    pub apps: Vec<AppDefinition>,
    pub examples: Vec<ExampleDefinition>,
    #[serde(default)]
    #[value(default)]
    pub capabilities: Vec<kernel::CapabilityRequirement>,
    /// 🗂️ Open plugin contributions — see `TopicContribution`.
    #[serde(default)]
    #[value(default)]
    pub topic_contributions: Vec<TopicContribution>,
    /// 🎛️ Plugin-scope commands this program exposes — apply whenever any of its apps is focused.
    #[serde(default)]
    #[value(default)]
    pub commands: Vec<CommandDefinition>,
    /// 🗂️ Plugin-level artifact kinds (library plugins with zero apps declare kinds here).
    #[serde(default)]
    #[value(default)]
    pub artifact_kinds: Vec<ArtifactKindSpec>,
    /// 🏠️ Artifact kinds another package owns whose documents this plugin's apps open — one row per kind and document
    /// schema its owner's codecs decode (`PluginBuilder::host_artifact`, or a surface of another plugin's artifact the plugin
    /// registers). Distinct from `artifact_kinds`: the codec rows stay
    /// the owner's, and a trusted catalog binds these kinds to the owner's codecs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub hosted_artifact_kinds: Vec<HostedArtifactKind>,
    /// 🔗️ Direct plugin dependencies this plugin requires to load — see `PluginDependency`/
    /// `resolve_load_order`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<PluginDependency>,
    /// 🗂️ Artifact-kind contributions (mutations/inferences) this plugin contributes onto artifact
    /// kinds it depends on — see `ArtifactContributionDescriptor`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub contributions: Vec<ArtifactContributionDescriptor>,
}

//#region 🔖️HostResolvedArgs
/// 🗂️ One artifact-kind choice offered by an `ActionArgControl::ArtifactKind` dialog field —
/// resolved by the host from its live plugin catalogue (`artifact_kind_choices`) into a plain
/// `Select { options }` right before the dialog renders. Round-trips through `ActionArgOption.value`
/// as JSON via `encode_artifact_kind_choice`/`decode_artifact_kind_choice` — the frozen wire shape
/// (contract §C8.1): `{"kindId":"s.draw.draw","schema":"draw.document","dialect":{"artifactKind":
/// "s.draw.draw","standard":"1","subset":"*"},"label":{"en":"Draw","de":"Zeichnung"}}`. TS twin:
/// `ArtifactKindChoice` (`🟦️.ts`) — both codecs must agree byte-for-byte over the pinned
/// fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct ArtifactKindChoice {
    pub kind_id: String,
    pub schema: String,
    pub dialect: ArtifactDialect,
    pub label: LocalizedLabel,
}

/// 🎭️ One `(pluginId, appId, role)` choice offered by an `ActionArgControl::SurfaceApp` dialog
/// field — resolved by the host against the dialect coordinate found in the dialog's seed argument
/// named `dialect_arg`. Round-trips through `ActionArgOption.value` as JSON via
/// `encode_surface_app_choice`/`decode_surface_app_choice`: `{"pluginId":"draw","appId":"s.draw.draw
/// @1/*#editor","role":"editor"}`. TS twin: `SurfaceAppChoice` (`🟦️.ts`).
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceAppChoice {
    pub app: AppRef,
    pub role: AppRole,
}

/// 🧵️ Encodes an `ArtifactKindChoice` into the frozen `ActionArgOption.value` JSON shape — `label`
/// resolves under `Terminology::Native`, the only terminology this wire shape carries (a dialog
/// re-resolves display strings client-side under the active terminology from `kind_id`/`schema`
/// alone if it ever needs to, but the frozen shape itself is native-only, matching `IconSelect`'s own
/// `classifier_kind`-not-label precedent for host-resolved controls).
pub async fn encode_artifact_kind_choice(choice: &ArtifactKindChoice) -> String {
    dsl::os_pack::json::to_string(&dsl::os_pack::json::object([
        ("kindId".to_string(), choice.kind_id.as_str().into()),
        ("schema".to_string(), choice.schema.as_str().into()),
        ("dialect".to_string(), dsl::os_pack::json::from_dsl_value(&choice.dialect.to_value())),
        (
            "label".to_string(),
            dsl::os_pack::json::object([
                ("en".to_string(), choice.label.resolve(Terminology::Native, Locale::En).to_string().into()),
                ("de".to_string(), choice.label.resolve(Terminology::Native, Locale::De).to_string().into()),
            ]),
        ),
    ]))
}

/// 🧵️ Inverse of `encode_artifact_kind_choice`.
pub async fn decode_artifact_kind_choice(value: &str) -> Result<ArtifactKindChoice, String> {
    let json = dsl::os_pack::json::parse(value).map_err(|error| format!("malformed artifact kind choice JSON: {error}"))?;
    let kind_id = json.get("kindId").and_then(dsl::os_pack::json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field kindId".to_string())?.to_string();
    let schema = json.get("schema").and_then(dsl::os_pack::json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field schema".to_string())?.to_string();
    let dialect: ArtifactDialect = json
        .get("dialect")
        .cloned()
        .ok_or_else(|| "artifact kind choice missing field dialect".to_string())
        .and_then(|value| ArtifactDialect::from_value(dsl::os_pack::json::to_dsl_value(&value)).map_err(|error| format!("artifact kind choice has a malformed dialect: {error}")))?;
    let en = json.pointer("/label/en").and_then(dsl::os_pack::json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field label.en".to_string())?;
    let de = json.pointer("/label/de").and_then(dsl::os_pack::json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field label.de".to_string())?;
    Ok(ArtifactKindChoice { kind_id, schema, dialect, label: LocalizedLabel::native(en, de) })
}

/// 🧵️ Encodes a `SurfaceAppChoice` into its frozen `ActionArgOption.value` JSON shape.
pub async fn encode_surface_app_choice(choice: &SurfaceAppChoice) -> String {
    dsl::os_pack::json::to_string(&dsl::os_pack::json::object([
        ("pluginId".to_string(), choice.app.plugin_id.as_str().into()),
        ("appId".to_string(), choice.app.app_id.as_str().into()),
        ("role".to_string(), choice.role.as_str().into()),
    ]))
}

/// 🧵️ Inverse of `encode_surface_app_choice`.
pub async fn decode_surface_app_choice(value: &str) -> Result<SurfaceAppChoice, String> {
    let json = dsl::os_pack::json::parse(value).map_err(|error| format!("malformed surface app choice JSON: {error}"))?;
    let plugin_id = json.get("pluginId").and_then(dsl::os_pack::json::Value::as_str).ok_or_else(|| "surface app choice missing string field pluginId".to_string())?.to_string();
    let app_id = json.get("appId").and_then(dsl::os_pack::json::Value::as_str).ok_or_else(|| "surface app choice missing string field appId".to_string())?.to_string();
    let role_str = json.get("role").and_then(dsl::os_pack::json::Value::as_str).ok_or_else(|| "surface app choice missing string field role".to_string())?;
    let role: AppRole = role_str.parse()?;
    Ok(SurfaceAppChoice { app: AppRef { plugin_id, app_id }, role })
}

/// 🗂️ Every artifact-kind choice for the given `roles` — Rust twin of TS `artifactKindChoices` (law in
/// `🧪️tests/🧪️hostresolvedargs/🟦️.ts`): every app across `manifests` whose `role` is in `roles`, whose
/// `io.artifact_schema` is non-empty and whose package declares a kind of that schema (on any of its apps
/// — a viewer shares its editor's kind — or on the manifest) contributes one choice per dialect coordinate,
/// labelled with that kind's own label, never its app's; an undeclared schema is not a creatable kind.
/// Deduped by dialect coordinate (first manifest/app wins — callers pass owner manifests first so the
/// owner's label wins over a later contributor's), sorted by coordinate for determinism.
pub async fn artifact_kind_choices(manifests: &[PluginManifest], roles: &[AppRole]) -> Vec<ArtifactKindChoice> {
    let mut by_coordinate: BTreeMap<String, ArtifactKindChoice> = BTreeMap::new();
    for manifest in manifests {
        for app in &manifest.apps {
            if !roles.contains(&app.role) || app.io.artifact_schema.is_empty() {
                continue;
            }
            let Some(kind) = manifest.apps.iter().flat_map(|other| other.artifact_kinds.iter()).chain(manifest.artifact_kinds.iter()).find(|kind| kind.schema == app.io.artifact_schema) else {
                continue;
            };
            by_coordinate.entry(app.dialect.to_coordinate()).or_insert_with(|| ArtifactKindChoice { kind_id: app.dialect.artifact_kind.clone(), schema: app.io.artifact_schema.clone(), dialect: app.dialect.clone(), label: kind.label.clone() });
        }
    }
    by_coordinate.into_values().collect()
}
//#endregion 🔖️HostResolvedArgs

//#region 🔖️DependencyGraph
/// 🚧️ Typed dependency-graph validation failures — contract freeze §4/§5: missing dependency,
/// version mismatch, or a cycle (naming every plugin id on the cycle, in traversal order).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DependencyGraphError {
    MissingDependency { plugin_id: String, depends_on: String },
    VersionMismatch { plugin_id: String, depends_on: String, required: String, actual: String },
    Cycle { members: Vec<String> },
}

impl std::fmt::Display for DependencyGraphError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDependency { plugin_id, depends_on } => write!(formatter, "plugin `{plugin_id}` depends on unknown plugin `{depends_on}`"),
            Self::VersionMismatch { plugin_id, depends_on, required, actual } => {
                write!(formatter, "plugin `{plugin_id}` requires `{depends_on}` `{required}` but the loaded version is `{actual}`")
            }
            Self::Cycle { members } => write!(formatter, "dependency cycle among plugins: {}", members.join(" -> ")),
        }
    }
}

impl std::error::Error for DependencyGraphError {}

/// ✅️ Checks every declared dependency resolves to a loaded plugin at a satisfying version —
/// deterministic: manifests are checked in input order, each manifest's dependencies in declaration
/// order, so the first violation found is always the same for the same input.
fn validate_dependency_graph(manifests: &[PluginManifest]) -> Result<(), DependencyGraphError> {
    let by_id: BTreeMap<&str, &PluginManifest> = manifests.iter().map(|manifest| (manifest.plugin_id.as_str(), manifest)).collect();
    for manifest in manifests {
        for dependency in &manifest.dependencies {
            let Some(target) = by_id.get(dependency.plugin_id.as_str()) else {
                return Err(DependencyGraphError::MissingDependency { plugin_id: manifest.plugin_id.clone(), depends_on: dependency.plugin_id.clone() });
            };
            if !dependency.version.matches_raw(&target.version) {
                return Err(DependencyGraphError::VersionMismatch { plugin_id: manifest.plugin_id.clone(), depends_on: dependency.plugin_id.clone(), required: dependency.version.to_string(), actual: target.version.clone() });
            }
        }
    }
    Ok(())
}

/// 🧭️ Kahn toposort of the plugin dependency graph: a dependency always precedes its dependents,
/// and among several simultaneously-ready plugins the lexicographically smallest id is always
/// picked next, so the returned order is a pure, deterministic function of the input set. Runs
/// `validate_dependency_graph` first, so a missing dependency or version mismatch is reported
/// before any cycle would be detected.
pub fn resolve_load_order(manifests: &[PluginManifest]) -> Result<Vec<String>, DependencyGraphError> {
    validate_dependency_graph(manifests)?;

    let mut in_degree: BTreeMap<&str, usize> = manifests.iter().map(|manifest| (manifest.plugin_id.as_str(), 0)).collect();
    let mut dependents_of: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for manifest in manifests {
        for dependency in &manifest.dependencies {
            *in_degree.get_mut(manifest.plugin_id.as_str()).expect("validated above") += 1;
            dependents_of.entry(dependency.plugin_id.as_str()).or_default().push(manifest.plugin_id.as_str());
        }
    }

    let mut ready: std::collections::BTreeSet<&str> = in_degree.iter().filter(|(_, degree)| **degree == 0).map(|(id, _)| *id).collect();
    let mut order: Vec<String> = Vec::with_capacity(manifests.len());
    while let Some(next) = ready.iter().next().copied() {
        ready.remove(next);
        order.push(next.to_string());
        if let Some(dependents) = dependents_of.get(next) {
            let mut sorted_dependents = dependents.clone();
            sorted_dependents.sort_unstable();
            for dependent in sorted_dependents {
                let degree = in_degree.get_mut(dependent).expect("validated above");
                *degree -= 1;
                if *degree == 0 {
                    ready.insert(dependent);
                }
            }
        }
    }

    if order.len() != manifests.len() {
        let resolved: std::collections::BTreeSet<&str> = order.iter().map(String::as_str).collect();
        let leftover: std::collections::BTreeSet<String> = manifests.iter().map(|manifest| manifest.plugin_id.as_str()).filter(|id| !resolved.contains(id)).map(str::to_string).collect();
        return Err(DependencyGraphError::Cycle { members: find_cycle_members(manifests, &leftover) });
    }
    Ok(order)
}

/// 🔁️ Walks the leftover (never-ready) subgraph depth-first from its lexicographically smallest
/// node, following each plugin's first declared dependency that is also leftover, until a node
/// repeats — the repeated slice of the walked path is the named cycle.
fn find_cycle_members(manifests: &[PluginManifest], leftover: &std::collections::BTreeSet<String>) -> Vec<String> {
    let by_id: BTreeMap<&str, &PluginManifest> = manifests.iter().map(|manifest| (manifest.plugin_id.as_str(), manifest)).collect();
    let mut visited: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for start in leftover {
        if visited.contains(start) {
            continue;
        }
        let mut path: Vec<String> = Vec::new();
        let mut on_path: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut node = start.clone();
        loop {
            if on_path.contains(&node) {
                let cycle_start = path.iter().position(|id| id == &node).expect("on_path implies present in path");
                return path[cycle_start..].to_vec();
            }
            if visited.contains(&node) {
                break;
            }
            visited.insert(node.clone());
            on_path.insert(node.clone());
            path.push(node.clone());
            let next = by_id.get(node.as_str()).and_then(|manifest| manifest.dependencies.iter().map(|dependency| dependency.plugin_id.clone()).find(|id| leftover.contains(id)));
            match next {
                Some(next_node) => node = next_node,
                None => break,
            }
        }
    }
    leftover.iter().cloned().collect()
}

/// 🔎️ Every plugin (direct dependents only, not transitive) that declares `plugin_id` as a
/// dependency, sorted for determinism — used to refuse unload/hot-reload while dependents are
/// loaded (contract freeze §4).
pub fn dependents(manifests: &[PluginManifest], plugin_id: &str) -> Vec<String> {
    let mut result: Vec<String> = manifests.iter().filter(|manifest| manifest.dependencies.iter().any(|dependency| dependency.plugin_id == plugin_id)).map(|manifest| manifest.plugin_id.clone()).collect();
    result.sort_unstable();
    result
}
//#endregion 🔖️DependencyGraph

#[cfg(test)]
#[path = "🧪️tests/🔬️plugin-dependency/🦀️.rs"]
mod plugin_dependency_tests;

// 🚧️ UNBLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `locale`/
// `terminology` are `ui_wgpu::wgpu::{Locale, Terminology}`, which now have hand-written
// `ToValue`/`FromValue` in `🖱️ui/🎯️targets/🧊️wgpu/🌐️locale-terminology/🧾️value/🦀️.rs`
// (that crate's own generated Rust projection is do-not-edit, so the impls live in a sibling `#[path]`
// mount instead). Kept additive: `ViewModel` is consumed outside this pass by 🛍️products/💻️os
// (plugin/renderer modules) and ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue
// added alongside, not replacing, Serialize/Deserialize.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ViewModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub active_mode_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub active_window_kind_id: Option<String>,
    /// 🧰️ Per-call overlay: the host-owned active utility for the window targeted by this `render`/`handle_action`
    /// call (`window_id`). On batched `refresh-ui`, the plugin stamps this from
    /// `active_utility_by_window_id` per window entry — never from the focused window alone.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub active_utility_id: Option<String>,
    /// 🧰️ Host-owned active utility per window **instance** (never a document field, never a VCS operation). The shell
    /// sends the full map on every refresh so plugins can build per-pane scene state; tools stay mode-wide via
    /// `active_tool_id`.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub active_utility_by_window_id: std::collections::HashMap<String, String>,
    /// 🛠️ The host-owned active tool of the active mode (never a document field, never a VCS operation) —
    /// mutually exclusive with `active_utility_id`: activating one clears the other (see the React
    /// shell's `onAction` interceptors).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub active_tool_id: Option<String>,
    /// 📌️ Host-owned panel state, opaque to the guest and bounded independently of embedded input.
    ///
    /// Contributions are deliberately NOT a view-state field. They are installed into the guest by
    /// the paged `setContributions` command run the host publisher owns
    /// (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`), which the guest folds into its own registry
    /// ([`crate::parse_contributions`], `🌊️flow/📔️registry/🦀️.rs`). Riding the aggregated closure
    /// inside every refresh's view state put a 248 635-character payload against a 65 536-character
    /// schema bound — no guest ever read it back
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub panel_json: Option<String>,
    /// 🪪️ Current host session identity for this call. This is ephemeral OS/session context:
    /// plugins may use it to qualify authored work and presentation, but must not persist a copy in
    /// app or document configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub session_identity: Option<ViewSessionIdentity>,
    /// 🧩️ Parent-owned input for one embedded surface; never persisted in the contributor document.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub extension_input_json: Option<String>,
    /// 🗣️ Active UI locale; plugins resolve their own label set from this via `resolve_labels`/
    /// `app_labels!`. Non-optional — the shell always resolves one (see `initUiLocaleSync`/
    /// `detectShellLocale`) before the first `render`, so "nobody set the locale" is unrepresentable.
    pub locale: Locale,
    /// 🗣️ Active terminology id (`Native` default, or an app-declared alternative term set).
    pub terminology: Terminology,
    /// 🪟️ The window instance a `render`/`handle_action` call targets — programs key all per-window
    /// option state (grid, LOD, selection mode, …) off this, never off `active_window_kind_id`, so that
    /// two window instances of the same kind (e.g. split top/perspective panes) never share options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// 🎯️ The window instance the user is LOOKING at — the shell's own last-focused pane. Sent on
    /// every call and, unlike [`Self::window_id`], deliberately kept by [`Self::for_panel`] while it
    /// names a live [`Self::window_instances`] entry: an
    /// app-level panel is not rendered FOR a window, but a panel that authors per-window settings
    /// still has to address the pane the user last touched instead of the roster's first entry
    /// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §5.1 measured a Settings edit landing on the
    /// base window kind, a pane nobody is looking at).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub focused_window_id: Option<String>,
    /// 🪟️ The live set of open window instances (base + spawned/split), sent on every refresh/action so
    /// `window_engagements`/`window_measures` can return one entry per instance instead of per kind.
    #[serde(default)]
    #[value(default)]
    pub window_instances: Vec<ViewWindowInstance>,
    /// ⏯️ The tool run trace cursor each window instance's renderer echoes, keyed by window instance id —
    /// host-owned like [`Self::active_utility_by_window_id`]. The guest answers the trace pages after it
    /// inside that window's scene `toolRunTrace` lane (`📋️tool-run-contract.md` §3.2).
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub tool_run_trace_cursor_by_window_id: std::collections::HashMap<String, semio_framework_tool_run::ToolRunTraceCursor>,
    /// 🪟️ Every tree container the host holds state for, flattened over all panel bodies — the ONE
    /// source of truth for which containers are open and which rows are on screen. A guest reads them
    /// per body and materialises exactly the named windows; it keeps no expansion state of its own,
    /// which is what lets an open container survive a refresh.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub tree_windows: Vec<TreeWindowRequest>,
    /// 🪟️ Rows the tallest visible panel body fits, the shared first-paint budget a guest spends in
    /// document order over containers the host has not yet seen. Absent means "the host has not
    /// measured a viewport yet" — the guest falls back to its own default, never to unbounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub tree_viewport_rows: Option<u32>,
}

/// 🪟️ One tree container's host-known state: whether the user opened or closed it (`None` = the
/// author's own default still stands) and the row window on screen, overscan included. `body_key`
/// names the panel body the container lives in, `node_key` the container's **window path** within it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TreeWindowRequest {
    pub body_key: String,
    /// 🔑️ A **view-context identifier**: printable (no C0 control or DEL code point) and at most 256
    /// code points, the same law `parseResolvedPluginViewState`/`admitCrossingViewContext` applies to
    /// every other identifier a view context carries (`🛂️manifest/🟦️.ts`). A path that cannot satisfy
    /// it is one the host never sends, and the guest then renders that container as unrequested.
    ///
    /// 🔑️ The container's **window path**, not a bare node key: the node keys of its enclosing
    /// windowed containers, outermost first, then its own key, joined by
    /// `semio_framework_ui_contract::TREE_WINDOW_PATH_SEPARATOR` (U+001F). A top-level section's path
    /// IS its own key, so a flat request is unchanged; a nested one is
    /// `"fem3d-play-artifact.load-cases\u{1f}dead"`. Identity is the path rather than the key so that
    /// two containers reusing one entity id under different parents — a load case and a combination
    /// both called `uls`, one object id in four cad pane sections — keep independent open and window
    /// state, while their node keys stay the raw pick target ids the interaction domain registers.
    pub node_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
    pub offset: u32,
    pub rows: u32,
}

/// 🪟️ One live window instance, as seen by a plugin: `id` is the instance id (equal to `window_kind_id`
/// for a base, unsplit window), `window_kind_id` is the `AppDefinition.windowKinds` entry it renders.
// 🚧️ UNBLOCKED: was gated on `ViewModel` above, itself gated on `ui_wgpu::wgpu::{Locale,
// Terminology}` gaining `ToValue`/`FromValue` — both now converted (see `ViewModel`'s own comment).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ViewWindowInstance {
    pub id: String,
    pub window_kind_id: String,
}

/// 🪪️ The authenticated OS session identity projected into every guest call while available.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ViewSessionIdentity {
    pub user_id: String,
    pub display_name: String,
}

impl ViewModel {
    /// 📌️ Projects app-level panels without binding their controls to a window.
    ///
    /// 🎯️ [`Self::focused_window_id`] survives — that is the whole point of the field — but only while
    /// it names a pane the host actually carries. A shell publishes the new mode's roster before it
    /// refocuses (the wgpu shell boots straight into `generate` with the edit-mode `procedural-main`
    /// still focused), and a focused pane the roster does not list made
    /// `WindowConfigOwnerRegistry::capture` fault before the panel's body key was ever matched — so
    /// every app panel published NOTHING and reached the user as three
    /// `wgpu-ui.surface-not-published:framework.panel.*` fault cards, while the framework-rendered
    /// History panel (served before that capture) published fine
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A stale focus is a mode-switch race; a panel
    /// answers it by authoring against no pane, never by publishing nothing.
    pub fn for_panel(&self) -> Self {
        let focused_window_id = self.focused_window_id.clone().filter(|id| self.window_instances.iter().any(|window| &window.id == id));
        Self { window_id: None, active_window_kind_id: None, active_utility_id: None, focused_window_id, ..self.clone() }
    }

    /// 🎯️ Projects host-owned context onto one concrete window without borrowing the focused window's utility.
    pub fn for_window_instance(&self, window_id: &str) -> Option<Self> {
        let window = self.window_instances.iter().find(|window| window.id == window_id)?;
        Some(Self {
            window_id: Some(window.id.clone()),
            active_window_kind_id: Some(window.window_kind_id.clone()),
            active_utility_id: self.active_utility_by_window_id.get(window_id).cloned(),
            ..self.clone()
        })
    }

    /// 🛠️ Overlays the host-owned mode tool, then binds the view to a window or the panel.
    pub fn for_host_armed_action(&self, host_active_tool_id: Option<&str>, window_id: Option<&str>) -> Option<Self> {
        let armed = Self { active_tool_id: host_active_tool_id.map(str::to_string), ..self.clone() };
        match window_id {
            Some(id) => armed.for_window_instance(id),
            None => Some(armed.for_panel()),
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window-view-context/🦀️.rs"]
mod window_view_context_tests;

//#region 📏️ViewContextCapacity
/// 📏️ The surface view context's capacities, mirrored from the ONE language-neutral declaration
/// `🪟️view-context/🧬️schema/🔣️.json` — the same file `parseResolvedPluginViewState`
/// (`🛂️manifest/🟦️.ts`) and its Ajv oracle validate against. Pinned from Rust by
/// `🧪️tests/🔬️view-context-capacity/🦀️.rs`, so neither side can drift from the schema.
pub const VIEW_CONTEXT_IDENTIFIER_CHARS: usize = 256;
/// 📏️ `panelJson` capacity, in Unicode characters (schema `maxLength`).
pub const VIEW_CONTEXT_LONG_STRING_CHARS: usize = 65_536;
/// 📏️ `activeUtilityByWindowId` capacity (schema `maxProperties`).
pub const VIEW_CONTEXT_UTILITY_ENTRIES: usize = 64;
/// 📏️ `windowInstances` capacity (schema `maxItems`).
pub const VIEW_CONTEXT_WINDOW_INSTANCES: usize = 64;
/// 📏️ `toolRunTraceCursorByWindowId` capacity (schema `maxProperties`).
pub const VIEW_CONTEXT_TRACE_CURSOR_ENTRIES: usize = 64;
/// 📏️ `treeWindows` capacity (schema `maxItems`) — one document's worth of containers
/// (`ui_contract::UI_DOCUMENT_NODES`), the widest tree a single refresh can be about.
pub const VIEW_CONTEXT_TREE_WINDOWS: usize = 128;
/// 🔢️ Numeric fields of one `TreeWindowRequest`: `offset` and `rows`. `open` is a boolean and
/// `bodyKey`/`nodeKey` are `Identifier`-shaped.
pub const VIEW_CONTEXT_TREE_WINDOW_NUMERIC_FIELDS: usize = 2;
/// 🔤️ Fields one `TreeWindowRequest` encodes: `bodyKey`, `nodeKey`, `open`, `offset`, `rows`.
pub const VIEW_CONTEXT_TREE_WINDOW_FIELDS: usize = 5;
/// 🔢️ Numeric fields of one `ToolRunTraceCursor`: `run`, `generation`, `page`.
pub const VIEW_CONTEXT_TRACE_CURSOR_FIELDS: usize = 3;
/// 📐️ Widest decimal a numeric field reaches in text form: `run` up to 2^53-1, and every `u32`
/// field (`generation`, `page`, `offset`, `rows`, `treeViewportRows`) comfortably inside it.
const VIEW_CONTEXT_TRACE_CURSOR_DIGITS: usize = 16;
/// 🔢️ `Identifier`-typed scalar fields: `activeModeId`, `activeWindowKindId`, `activeUtilityId`,
/// `activeToolId`, `windowId`, `focusedWindowId`.
pub const VIEW_CONTEXT_IDENTIFIER_FIELDS: usize = 6;
/// 🪪️ Identifier-shaped fields nested in `sessionIdentity`: `userId` and `displayName`.
pub const VIEW_CONTEXT_SESSION_IDENTITY_FIELDS: usize = 2;
/// 🔢️ Bounded opaque inputs: panel state and one embedded surface's parameters.
pub const VIEW_CONTEXT_LONG_STRING_FIELDS: usize = 2;
/// 📐️ Worst-case UTF-8 expansion of one schema character — the schema bounds characters, the wire
/// carries bytes.
const VIEW_CONTEXT_BYTES_PER_CHAR: usize = 4;
/// 📐️ Per-encoded-value framing allowance (tag, length prefix, key) of the pack wire the shell
/// encodes a view context with.
const VIEW_CONTEXT_FRAMING_BYTES_PER_VALUE: usize = 64;
/// 🔢️ Encoded values a maximal view context carries: every scalar field, the two enums, every
/// collection and each of their entries' fields.
const VIEW_CONTEXT_ENCODED_VALUES: usize = VIEW_CONTEXT_IDENTIFIER_FIELDS
    + VIEW_CONTEXT_SESSION_IDENTITY_FIELDS
    + VIEW_CONTEXT_LONG_STRING_FIELDS
    + 2
    + 1
    + VIEW_CONTEXT_UTILITY_ENTRIES * 2
    + 1
    + VIEW_CONTEXT_WINDOW_INSTANCES * 2
    + 1
    + VIEW_CONTEXT_TRACE_CURSOR_ENTRIES * (1 + VIEW_CONTEXT_TRACE_CURSOR_FIELDS)
    + 1
    + VIEW_CONTEXT_TREE_WINDOWS * (1 + VIEW_CONTEXT_TREE_WINDOW_FIELDS)
    + 1;

/// 📏️ Largest wire-encoded surface view context the contract can produce — the admission bound
/// `plugin_mount_surface` holds `Event::SurfaceVisible`'s `view_state` to.
///
/// Derived from the schema above rather than borrowed: this used to reuse
/// `MAX_PUBLIC_ACTION_BODY_BYTES` (256 KiB), the DFF *public action* admission cap, which is
/// unrelated to a view context and smaller than the 64 Ki-character long string the schema
/// alone permits — so a schema-valid context was rejected as
/// `plugin.internal: surface context exceeds its wire bound` and every window body of the affected
/// app was replaced by that fault card (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const MAX_SURFACE_VIEW_CONTEXT_BYTES: usize = (VIEW_CONTEXT_IDENTIFIER_FIELDS * VIEW_CONTEXT_IDENTIFIER_CHARS
    + VIEW_CONTEXT_SESSION_IDENTITY_FIELDS * VIEW_CONTEXT_IDENTIFIER_CHARS
    + VIEW_CONTEXT_LONG_STRING_FIELDS * VIEW_CONTEXT_LONG_STRING_CHARS
    + VIEW_CONTEXT_UTILITY_ENTRIES * 2 * VIEW_CONTEXT_IDENTIFIER_CHARS
    + VIEW_CONTEXT_WINDOW_INSTANCES * 2 * VIEW_CONTEXT_IDENTIFIER_CHARS
    + VIEW_CONTEXT_TRACE_CURSOR_ENTRIES * (VIEW_CONTEXT_IDENTIFIER_CHARS + VIEW_CONTEXT_TRACE_CURSOR_FIELDS * VIEW_CONTEXT_TRACE_CURSOR_DIGITS)
    + VIEW_CONTEXT_TREE_WINDOWS * (2 * VIEW_CONTEXT_IDENTIFIER_CHARS + VIEW_CONTEXT_TREE_WINDOW_NUMERIC_FIELDS * VIEW_CONTEXT_TRACE_CURSOR_DIGITS + VIEW_CONTEXT_TRACE_CURSOR_DIGITS)
    + VIEW_CONTEXT_TRACE_CURSOR_DIGITS)
    * VIEW_CONTEXT_BYTES_PER_CHAR
    + VIEW_CONTEXT_ENCODED_VALUES * VIEW_CONTEXT_FRAMING_BYTES_PER_VALUE;

/// 📏️ Largest surface body key the same contract admits — a body key is a window-kind id, a
/// panel-tab leaf id or a reserved `UiRefreshSection` key, all `Identifier`-shaped.
pub const MAX_SURFACE_BODY_KEY_BYTES: usize = VIEW_CONTEXT_IDENTIFIER_CHARS * VIEW_CONTEXT_BYTES_PER_CHAR;

#[cfg(test)]
#[path = "🧪️tests/🔬️view-context-capacity/🦀️.rs"]
mod view_context_capacity_tests;

#[cfg(test)]
#[path = "🧪️tests/🔢️integer-carriers/🦀️.rs"]
mod view_context_integer_carrier_tests;
//#endregion 📏️ViewContextCapacity

//#region 📏️PublicInvocationCapacity
/// 📏️ Largest UTF-8 body one structurally addressed action/command JSON invocation may occupy on
/// the way into a plugin process — `🎛️public-invocation/🧬️schema/🔣️.json`'s `maxBodyBytes`.
pub const PUBLIC_INVOCATION_BODY_BYTES: usize = 262_144;
/// 📏️ Largest single JSON string one invocation may carry — `maxStringBytes`. Counted as escaped
/// bytes minus their leading backslashes, so `\"` costs one and `A` costs five.
///
/// This is the bound that actually sizes a host→guest push, and NO tool execution contract can
/// widen it: `validate_public_json_envelope` runs before the addressed tool's own
/// `max_raw_wire_bytes` is ever consulted. Any payload larger than this must be paged by its
/// producer (`🏛️ShellHost/🟦️.tsx`'s `publicInvocationStringPages`).
pub const PUBLIC_INVOCATION_STRING_BYTES: usize = 4_096;
/// 📏️ Deepest object/array nesting one invocation may reach — `maxDepth`.
pub const PUBLIC_INVOCATION_DEPTH: usize = 64;
/// 📐️ Wire bytes one counted string byte may occupy — `escapePairWireFactor`. A two-character
/// escape pair (`\"`, `\\`, `\n`) counts ONE against [`PUBLIC_INVOCATION_STRING_BYTES`] and occupies
/// TWO on the wire; every longer escape (`\u00XX`, six bytes) counts five, a smaller ratio. So a
/// page filled to the string bound occupies at most twice its counted extent, which is what a tool
/// carrying such a page must declare as its `max_raw_wire_bytes`.
pub const PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR: usize = 2;

/// 📐️ What ONE character of a raw string costs against [`PUBLIC_INVOCATION_STRING_BYTES`] once the
/// JSON encoder has written it — the exact accounting `validate_public_json_envelope` performs,
/// which skips a leading `\` and counts every byte after it. Non-ASCII is charged at its `\uXXXX`
/// escape (five per UTF-16 unit), never at its shorter raw UTF-8 form, so a page cut with this
/// function is admitted whether or not the encoder escapes above U+007F.
pub fn public_invocation_char_cost(character: char) -> usize {
    match character {
        '"' | '\\' | '\n' | '\r' | '\t' | '\u{8}' | '\u{c}' => 1,
        character if (character as u32) < 0x20 => 5,
        character if character.is_ascii() => 1,
        character => character.len_utf16() * 5,
    }
}

/// 📄️ Cuts one oversized string into the page run a public invocation can actually carry — each
/// page filled to, and never past, [`PUBLIC_INVOCATION_STRING_BYTES`] as
/// [`public_invocation_char_cost`] measures it, split only on character boundaries.
///
/// The twin of `publicInvocationStringPages` in `🛠️ShellHelpers/🟦️.tsx`; the two must cut the same
/// payload identically, which `🔬️engine-contract/🟦️.ts` and this module's own capacity law pin.
/// An empty input yields one empty page, so a producer always sends at least one addressed page.
pub fn public_invocation_string_pages(text: &str) -> Vec<String> {
    let mut pages = Vec::new();
    let mut page = String::new();
    let mut cost = 0usize;
    for character in text.chars() {
        let next = public_invocation_char_cost(character);
        if cost + next > PUBLIC_INVOCATION_STRING_BYTES {
            pages.push(std::mem::take(&mut page));
            cost = 0;
        }
        page.push(character);
        cost += next;
    }
    pages.push(page);
    pages
}

#[cfg(test)]
#[path = "🧪️tests/🔬️public-invocation-capacity/🦀️.rs"]
mod public_invocation_capacity_tests;
//#endregion 📏️PublicInvocationCapacity

//#region 🔖️UiRefreshSection
/// 🧩️ The four refresh sections that are NOT authored window/panel bodies. Each is its own retained
/// surface whose reserved body key names the object-safe accessor the plugin runtime calls in place of
/// `PluginApp::render` (`window_engagements`/`window_measures`/`tool_measures`/`app_catalogue`), so
/// measures, tool measures, engagements and the app-static catalogue publish, re-publish and page
/// through exactly the same `Event::SurfaceVisible` → mount → reconcile → `UiPatch` law as a window body.
///
/// 🛍️ `Catalogue` is app-STATIC: it carries the whole registered operator/palette catalogue exactly
/// once per app instance (hash-conditional, so an unchanged catalogue costs one hash compare), instead
/// of riding on every node-graph scene payload where it blew the fixed `UI_FIXED_BYTES` surface
/// admission as soon as real operator sets were installed (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// Mirrored in TypeScript by `UI_REFRESH_SECTIONS` in `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`.
/// Both sides are pinned against the one language-neutral declaration in
/// `🧪️tests/🔬️ui-refresh-section/🔣️.json`, so neither can drift.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiRefreshSection {
    Engagements,
    Measures,
    Tools,
    Catalogue,
}

/// 🔑️ Response field / host refresh-cache key of each [`UiRefreshSection`], in `UiRefreshSection::ALL` order.
pub const UI_REFRESH_SECTION_KEYS: [&str; 4] = ["engagements", "measures", "tools", "catalogue"];

/// 🪧️ Reserved body key — and retained surface key — of each [`UiRefreshSection`], in
/// `UiRefreshSection::ALL` order. Dotted and `framework.`-prefixed so it can never collide with an
/// app-authored window instance id or panel tab id, which are plain identifiers.
pub const UI_REFRESH_SECTION_BODY_KEYS: [&str; 4] = ["framework.section.engagements", "framework.section.measures", "framework.section.tools", "framework.section.catalogue"];

impl UiRefreshSection {
    pub const ALL: [Self; 4] = [Self::Engagements, Self::Measures, Self::Tools, Self::Catalogue];

    /// 🔑️ See [`UI_REFRESH_SECTION_KEYS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn key(self) -> &'static str {
        UI_REFRESH_SECTION_KEYS[self as usize]
    }

    /// 🪧️ See [`UI_REFRESH_SECTION_BODY_KEYS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn body_key(self) -> &'static str {
        UI_REFRESH_SECTION_BODY_KEYS[self as usize]
    }

    /// 🔎️ Resolves a mounted surface's body key back to the section it renders, `None` for every
    /// app-authored body.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|section| section.body_key() == body_key)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️ui-refresh-section/🦀️.rs"]
mod ui_refresh_section_tests;
//#endregion 🔖️UiRefreshSection

// 🎗️ `AppLabelsOverlay` (the stringly-typed, per-id runtime label-patch map) is deleted — manifest
// labels are now `LocalizedLabel` fields resolved directly via `.resolve(terminology, locale)`, so a
// separate locale-aware overlay merged in after the fact is no longer needed. Downstream callers
// (plugin crates' `ArtifactApp::app_labels()`, the OS renderer's overlay-merge call sites) are
// follow-up work owned by other agents — left broken intentionally, out of scope here.

//#region 🔖️Kernel
#[path = "../🎠️kernel/🦀️.rs"]
pub mod kernel;
//#endregion 🔖️Kernel

//#region 🔖️PackageDescriptor
/// 🎭️ Which actor-world role a package fills — `📓️design-abi.md` §3.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum PackageRole {
    Plugin,
    Extension,
}

/// 🚦 How an extension actor runs relative to its host plugin — `📓️design-abi.md` §5. Default
/// `Isolated`: a same-process sandboxed actor, no publisher trust assumed. `Linked` additionally
/// requires the same publisher as the host plugin (enforced at link time, feature-gated to avoid
/// the `semio-framework-os-flow` ↔ extension-crate cycle); `Exclusive` gets a dedicated actor
/// (e.g. flow/brep tessellation); `Cold` runs as a bounded job, not a resident actor.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ExecutionMode {
    Declarative,
    Linked,
    #[default]
    Isolated,
    Exclusive,
    Cold,
}

/// 🧩️ One extension point a host plugin publishes — replaces the Cargo `consumes` tag
/// (`📓️design-abi.md` §5). `allowed_modes` gates `Linked` (same publisher required);
/// `capability_allowance`/`quota_ceiling` bound what any extension attaching here can ever hold,
/// regardless of what it requests — "a host can never delegate more than it holds".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExtensionPointDeclaration {
    pub id: String,
    pub publisher_scope: String,
    pub allowed_modes: Vec<ExecutionMode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_allowance: Vec<kernel::CapabilityId>,
    #[serde(default)]
    #[value(default)]
    pub quota_ceiling: kernel::QuotaSchema,
    pub payload_schema: kernel::SchemaId,
    pub activation: kernel::ActivationEvent,
}

/// 📦️ One asset bundled with a package and preloaded into `kernel::Event::InstanceOpen.assets` —
/// `📓️design-abi.md` §2's `read-asset` replacement.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct AssetDeclaration {
    pub name: String,
    pub media_type: MediaType,
    pub size_bytes: u64,
    pub sha256: String,
}

/// #️⃣ Content hashes the registry's `check` gate verifies against the built wasm —
/// `📓️design-abi.md` §3.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct PackageHashes {
    pub wasm_sha256: String,
    pub core_wasm_sha256: String,
    pub descriptor_sha256: String,
}

/// 🗂️ One free-form descriptor-only contribution row, keyed by `id` with an opaque JSON
/// `payload` — the residual placeholder shape for the two `ContributionSet` categories (`menus`,
/// `themes`) that still have no real declared-contribution precedent anywhere in the codebase
/// (E1-describe surveyed every `[package.metadata.semio]` `contributes`/`consumes` tag and every
/// manifest-adjacent type: no plugin declares a menu or theme as its own manifest concept today —
/// context menus are derived at runtime from `ActionSemantics`/category metadata, and there is no
/// declared theme/palette contribution anywhere under `🖱️ui/🎨️styling`). Additive: nothing
/// constructs one yet, and a future typed model can replace either category without a wire break.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DescriptorEntry {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<DslValue>,
}

/// 🗂️ One file-format kind an app declares it can import and/or export — the typed shape for
/// `ContributionSet.file_types` (`📓️design-abi.md` §3), grounded in `AppIo.export_formats`/
/// `import_formats` (currently flat `Vec<String>` scaffolding on that type) paired with the app's
/// own `artifact_media_type`, flattened to one row per format kind across every app the package
/// declares.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct FileTypeContribution {
    pub format_kind: String,
    pub media_type: MediaType,
    pub imports: bool,
    pub exports: bool,
}

/// 🚪️ Which side of an `IoEntryDescriptor` route this row is — owned mirror of `io::IoDirection`
/// (`🚪️io/🦀️.rs`), the same "owned wire twin of a native type living in a sibling
/// framework module" idiom `ContributedMutationMetadata`/`ContributedInferenceMetadata` already
/// use for the os-kernel protocol crate.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IoEntryDirection {
    Import,
    Export,
}

/// 🚪️ One registered IO dialect route — the typed shape for `ContributionSet.io_entries`
/// (`📓️design-abi.md` §2/§3's absorbed `io-dialects` routing table), an owned mirror of
/// `io::IoKey`'s `(owner, counterpart, direction)` identity built from the already-in-scope
/// `ArtifactDialect` (`🚪️io/🧬️schema/🦀️component.rs`) instead of `IoKey`'s seven flat fields —
/// `IoKey` itself isn't `owned schema exporter`-derived and this crate must not add that derive to a module it
/// doesn't own.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IoEntryDescriptor {
    pub owner: ArtifactDialect,
    pub counterpart: ArtifactDialect,
    pub direction: IoEntryDirection,
}

/// 🎹️ One registered composer/serializer/deserializer route — the typed shape for
/// `ContributionSet.composer_entries`, an owned mirror of `io::ComposerEntry`'s `(writes, reads)`
/// identity (its third field, the `compose` fn pointer, is runtime-only and has no wire form —
/// a descriptor is build-time, non-executable data).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ComposerEntryDescriptor {
    pub writes: ArtifactDialect,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub reads: Vec<ArtifactDialect>,
}

/// 🗂️ Everything a package contributes, gathered for static (`describe()`-time) emission —
/// `📓️design-abi.md` §3. `commands`/`topic_contributions`/`artifact_contributions` reuse this
/// crate's existing typed models; `panels` reuses `PanelTabDefinition` (already the typed shape
/// `AppDefinition.panel_tabs` declares, flattened across every app); `inference_services`/
/// `mutation_services` reuse `ContributedInferenceMetadata`/`ContributedMutationMetadata` (a
/// package's OWN registered services on artifact kinds it owns, as opposed to
/// `artifact_contributions`' services contributed onto a DEPENDENCY's kind — same wire shape
/// either way, `contributor == owner` and `depends_on` empty for a self-owned row);
/// `file_types`/`io_entries`/`composer_entries` are new types grounded in `AppIo`/`io::IoKey`/
/// `io::ComposerEntry` — see each type's own doc. `menus`/`themes` stay `DescriptorEntry` — see
/// its doc for why.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContributionSet {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<CommandDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub menus: Vec<DescriptorEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub file_types: Vec<FileTypeContribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub panels: Vec<PanelTabDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub themes: Vec<DescriptorEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub topic_contributions: Vec<TopicContribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_contributions: Vec<ArtifactContributionDescriptor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub inference_services: Vec<ContributedInferenceMetadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mutation_services: Vec<ContributedMutationMetadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub io_entries: Vec<IoEntryDescriptor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub composer_entries: Vec<ComposerEntryDescriptor>,
}

/// 🚨️ The `plugin_id` a failed assembly mints instead of a real one. `plugin_manifest()`
/// (`🔌️plugin/🦀️.rs`) returns this stub whenever `PLUGIN_ASSEMBLY_ERROR` is set, carrying
/// the real error text in `label`. It looks like a descriptor, parses as JSON, and would feed the
/// generated registry catalog with fabricated contributions — so the emitter refuses to write one
/// (`🖨️describe/📦️packages/🦀️rust/🦀️.rs`). Lives here, beside [`PackageDescriptor`], because it
/// is the one crate BOTH the guest SDK that mints it and the host emitter that rejects it depend
/// on; a duplicated string literal in either would drift silently.
pub const ASSEMBLY_FAILED_PLUGIN_ID: &str = "assembly-failed";

/// 📡️ Exact guest-described application-channel ABI required by the compiled component.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExecutionProtocol {
    pub app_channel_version: u32,
}

/// 📦️ The static, build-time-emitted description of a plugin or extension package —
/// `📓️design-abi.md` §3's `describe()` output (`🛂️.descriptor.semio`/`🔣️.json`).
/// Nothing constructs or reads one yet in this packet: additive contract only (packet
/// A2-abi-sdk's builder wiring and E1-describe's emitter/registry `check` gate consume it next).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PackageDescriptor {
    pub descriptor_version: u32,
    pub package_id: String,
    pub role: PackageRole,
    pub manifest: PluginManifest,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub activation_events: Vec<kernel::ActivationEvent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_requests: Vec<kernel::CapabilityRequest>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extension_points: Vec<ExtensionPointDeclaration>,
    pub execution: ExecutionMode,
    pub execution_protocol: ExecutionProtocol,
    #[serde(default)]
    #[value(default)]
    pub quotas: kernel::QuotaSchema,
    #[serde(default)]
    #[value(default)]
    pub contributions: ContributionSet,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<AssetDeclaration>,
    pub hashes: PackageHashes,
}

#[cfg(test)]
#[path = "🧪️tests/🔬️package-descriptor-value-codec/🦀️.rs"]
mod package_descriptor_value_codec_tests;
//#endregion 🔖️PackageDescriptor

//#region 🔖️AgentContributions
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P8-agent-spi, `📋️master.md`
// §3.1: what a package OFFERS to agents — distinct from `capability_requests` on
// `PackageDescriptor` above (what a package NEEDS host permission for). Overloading one for the
// other is exactly the mistake this design forbids: `capability_requests: Vec<kernel::
// CapabilityRequest>` gates HOST PRIVILEGE (`artifacts.write`, `fs:*`, …) this package asks the
// broker for; `AgentContributions` below is a curated ADVERTISEMENT of which of this package's own
// already-declared capabilities (actions/commands, already fully described via `ActionSemantics`/
// `CapabilityPolicy` — P3, `🔖️ActionSemantics` above) an agent may discover and invoke at all, and
// which of those are further promoted to a first-class MCP tool (`📋️master.md` §3.1's `tools/list`
// "promoted" set).
//
// Attachment point, and why it is a LEASE rather than a field added directly here: the obvious
// home is a new `PackageDescriptor.agent: Option<AgentContributions>` field. `PackageDescriptor`'s
// only known construction sites (`describe_plugin()`/`describe_extension()`,
// `🔌️plugin/🛂️describe/🦀️.rs`) live in `semio-framework-plugin` — a crate this packet
// does not own and the peer ticket's W3 is about to freeze — and both build the value as a full
// explicit struct literal (verified by reading the file: no `..Default::default()` anywhere, and
// `PackageDescriptor` has no `Default` impl — `role: PackageRole` has no default variant — so a
// `Default` impl could not rescue an untouched call site either way). Adding the field HERE alone
// would therefore break `cargo check -p semio-framework-plugin --lib` the moment this region
// lands, in a live shared tree, before any reviewer applies a companion lease — exactly the
// destabilisation this packet must not cause. So the field itself, and its counterpart on the
// `PluginDescriptorExtras` side-channel (`🔌️plugin/🦀️.rs`, E2's own established pattern
// for precisely this "avoid cascading through construction sites I don't own" problem) and on
// `ExtensionManifest`, ship together as ONE atomic lease bundle — see `📓️terra-P8-report.md` §2
// for the full reasoning and `📓️lease-P8-agent-descriptor.md` for the exact diffs.
/// 🤖️ What a package OFFERS to agents — see the region header above for the critical
/// `capability_requests` vs `AgentContributions` distinction. `capabilities` are fully-qualified
/// capability ids (the same grammar `🌉️mcp/🗂️catalog` compiles — `<plugin_id>.<app_id>.
/// <action_id>` / `….cmd.<id>` / `….mode.<mode_id>.<id>`, `📋️master.md` §3.1); `promoted` is the
/// subset exposed as a first-class MCP tool (`tools/list`) rather than only reachable via
/// `capabilities.search`/`capabilities.describe`. Both empty by default — an absent
/// `AgentContributions` (the `Option` on `PackageDescriptor` stays `None`) means "not yet
/// agent-enabled", never "agent-enabled with zero capabilities" (an empty-but-`Some` value).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct AgentContributions {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub promoted: Vec<String>,
}

impl AgentContributions {
    /// 🧪️ `promoted ⊆ capabilities` — the one structural invariant every producer
    /// (`describe_plugin()`/`describe_extension()`) and consumer (`📇️registry:check`) must hold.
    /// Pure and dependency-free so both the Rust builder side and the registry's own TypeScript
    /// check (which has no way to call back into this crate) can each verify it independently.
    pub async fn promoted_is_subset_of_capabilities(&self) -> bool {
        self.promoted.iter().all(|id| self.capabilities.contains(id))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️agent-contributions/🦀️.rs"]
mod agent_contributions_tests;
//#endregion 🔖️AgentContributions

//#region 🔖️MediaVocabulary
// 🔀️ Relocated verbatim from 🔺️mesh/🦀️.rs (ticket 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT
// wave 4a) — manifest-vocabulary types, not codec material; mesh keeps MeshData/Primitives/
// generic obj-glb-stl codecs and the DWG bit-codec, but the legacy media-format enum itself was retired
// in ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6 — every
// format-kind field below is now a plain string kind id.
//#region ArtifactKind
/// 🧬️ Which geometry backend a resource kind's media exporters/importers target — the manifest-level
/// counterpart threaded onto `AppDefinition.artifact_kinds` (see `ArtifactKindSpec`). Canonical home for
/// what used to be duplicated verbatim in `framework/plugin/rs` and `framework/product/os/core/rs`; both
/// now re-export this definition instead of declaring their own.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum OsMediaCapability {
    MeshOnly,
    Brep,
}

/// 🗂️ An app-declared OS resource kind (e.g. a 3D mesh format, a raster format) — the manifest-level
/// counterpart to `AppBuilder::artifact_kind(...)` (`framework/plugin/rs`), letting `framework/product/os/core`
/// build its artifact catalog from `AppDefinition.artifact_kinds` at plugin registration time instead of
/// hardcoding a per-app match on kind-id strings. Carries the manifest-level media-kind fields
/// (`media_type`/`schema`/`export_formats`/`import_formats`) directly
/// so one spec carries both the OS-catalog presentation shape and the `MediaType` a wire actually negotiates
/// — see `crate::media_types_compatible`. `OsArtifactDescriptor` (`framework/product/os/core`) threads
/// `media_type` through so registry lookups return it alongside the rest of the descriptor.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ArtifactKindSpec {
    pub id: String,
    /// 🗣️ The kind's own localized name — what a picker shows for it, never its editor app's label.
    pub label: LocalizedLabel,
    pub source_format: String,
    pub component_kind: String,
    pub dimension: String,
    pub media_capability: OsMediaCapability,
    pub media_type: MediaType,
    pub schema: String,
    /// 🗄️ Export target format kind ids (string, the legacy format enum was retired — ticket 26/08/11/
    /// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
    pub export_formats: Vec<String>,
    pub import_formats: Vec<String>,
    /// 🗄️ Stdio export target kind ids (e.g. `stdio.json`) — additive peer of `export_formats`.
    pub export_stdio_kinds: Vec<String>,
    /// 🗄️ Stdio import source kind ids — additive peer of `import_formats`.
    pub import_stdio_kinds: Vec<String>,
}

/// 🏠️ One artifact kind a package hosts — the kind id, one document schema of its owner's codecs, and the OWNER: the plugin
/// that owns the kind (declares it and its codec), a declared dependency of the host (see `PluginManifest::hosted_artifact_kinds`).
/// The owner is explicit, never read off the kind id — an embedded surface's kind (`3d.cad`) names no plugin.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostedArtifactKind {
    pub id: String,
    pub schema: String,
    pub owner: String,
}
//#endregion ArtifactKind

//#region MediaType
/// 🧬️ Typed-media lattice: every port/wire in the workflow carries a `MediaType` (`class` × `form`) instead of the legacy string `artifact_kind`. `MediaType` is what a wire negotiates; a format kind id string is only how bytes are encoded once they actually cross a process boundary (see `MediaWireFormat`). Dependent tickets retire `OsMediaCapability` (see the `ArtifactKind` region above) onto `MediaForm::{Brep,Mesh}`, which already covers what `OsMediaCapability::{Brep,MeshOnly}` expresses.
// 🚧️ BLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `🎠️kernel/🦀️.rs`
// and `🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs` (both owned by other agents this pass) still
// embed `MediaType`/`MediaClass`/`MediaForm` by value inside plain `#[derive(Serialize,
// Deserialize)]` types — dropping serde here breaks `cargo check -p semio-framework` today. Both
// derive families stay load-bearing simultaneously until those crates migrate; revisit once they do.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum MediaClass {
    TwoD,
    ThreeD,
    Text,
    Data,
    Graph,
    Kit,
    Computation,
    Presentation,
}

/// 🧬️ The shape/representation a `MediaClass` payload takes, orthogonal to `class` — e.g. `ThreeD` × `Brep` vs `ThreeD` × `Mesh`. `Any` only ever appears on the accepting side of a port (see `media_types_compatible`).
// 🚧️ BLOCKED: see `MediaClass` above — same cross-crate serde dependency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum MediaForm {
    Any,
    Vector,
    Raster,
    Brep,
    Mesh,
    Document,
    Value,
    Dag,
    Trinity,
    Type,
    Design,
    Kit,
    Flow,
    Sequence,
    Procedure,
    Deck,
}

/// 🧬️ A port or wire's declared media type — the pair a producer offers or a consumer accepts.
// 🚧️ BLOCKED: see `MediaClass` above — same cross-crate serde dependency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct MediaType {
    pub class: MediaClass,
    pub form: MediaForm,
}

/// 🔌️ How a `MediaType` is actually encoded once it crosses a process boundary — binary payloads
/// carry a format kind id string (the legacy format enum was retired — ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6), structured payloads carry
/// a schema id instead (see `ArtifactKindSpec::schema`).
// 🚧️ BLOCKED: see `MediaClass` above — `🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs`'s
// `WorkflowMediaPort`/`MediaContract` (owned by another agent this pass) still embed this by value
// inside plain `#[derive(Serialize, Deserialize)]` types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", tag = "kind")]
pub enum MediaWireFormat {
    Binary { format_kind: String },
    Document { schema: String },
}

/// 🔀️ Which side of a wire a `MediaPortSpec` sits on.
// 🚧️ BLOCKED: see `MediaWireFormat` above — same cross-crate serde dependency (`WorkflowMediaPort`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum MediaPortDirection {
    In,
    Out,
}

/// 🔢️ Whether a `MediaPortSpec` accepts/produces exactly one media value or a stream/collection of them — e.g. a mesh-array input that fans in from several upstream producers.
// 🚧️ BLOCKED: see `MediaWireFormat` above — same cross-crate serde dependency (`WorkflowMediaPort`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum PortMultiplicity {
    One,
    Many,
}

/// 🔌️ A single port an app exposes on the workflow — `kind_id` optionally pins it to one `ArtifactKindSpec.id` when the port is more specific than its `media_type` alone conveys.
// 🚧️ BLOCKED: see `MediaWireFormat` above — `WorkflowMediaPort.spec` embeds this by value inside a
// plain `#[derive(Serialize, Deserialize)]` type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct MediaPortSpec {
    pub id: String,
    pub label: String,
    pub direction: MediaPortDirection,
    pub media_type: MediaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub kind_id: Option<String>,
    pub required: bool,
    pub multiplicity: PortMultiplicity,
}

/// ⚖️ Result of checking whether a producer's `MediaType` can feed a consumer's accepted `MediaType`: exact match, a known lossy-but-allowed conversion, or outright rejection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaCompat {
    Direct,
    Convert { from: MediaForm, to: MediaForm },
    Reject,
}

/// 🔀️ One-way `MediaForm` conversions the workflow is allowed to insert implicitly (e.g. a B-Rep producer feeding a mesh-only consumer). `media_types_compatible` looks up `(produced, accepted)` directly, so add the reverse pair too if a conversion should also hold the other way.
const MEDIA_FORM_CONVERSIONS: &[(MediaForm, MediaForm)] = &[(MediaForm::Brep, MediaForm::Mesh), (MediaForm::Vector, MediaForm::Raster), (MediaForm::Design, MediaForm::Kit), (MediaForm::Type, MediaForm::Kit)];

/// ⚖️ The single source of truth for wire compatibility: classes must match exactly, `MediaForm::Any` on the accepting side takes anything within the class, equal forms are always direct, and everything else falls through to the explicit `MEDIA_FORM_CONVERSIONS` table.
pub async fn media_types_compatible(produced: &MediaType, accepted: &MediaType) -> MediaCompat {
    if produced.class != accepted.class {
        return MediaCompat::Reject;
    }
    if matches!(accepted.form, MediaForm::Any) || produced.form == accepted.form {
        return MediaCompat::Direct;
    }
    for (from, to) in MEDIA_FORM_CONVERSIONS {
        if *from == produced.form && *to == accepted.form {
            return MediaCompat::Convert { from: *from, to: *to };
        }
    }
    MediaCompat::Reject
}
//#endregion MediaType

//#region 🔖️AppIo
/// 🧷️ The non-format fields of `ArtifactKindSpec` (see `ArtifactKind` region above) that describe how
/// a resource presents in the OS catalog — split out so `AppIo` can carry its own `export_formats`/
/// `import_formats` lists without duplicating `ArtifactKindSpec`'s full shape (which stays alive
/// unchanged for now; later waves retire it onto `AppIo`).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ArtifactPresentation {
    pub id: String,
    pub name: String,
    pub dimension: String,
    pub component_kind: String,
}

/// 🔌️ An app's full media I/O surface — the artifact schema/type every app carries implicitly (see
/// `artifact_in_port`/`artifact_out_port`) plus whatever additional workflow ports, catalog
/// export/import formats, and OS presentation it declares itself. Scaffolding for the typed manifest
/// surface (`AppDefinition.io`); apps don't populate this yet — later waves migrate `media_inputs`/
/// `media_outputs`/`artifact_kinds` onto it.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct AppIo {
    pub artifact_schema: String,
    pub artifact_media_type: MediaType,
    /// 🔌️ App-specific ports only — the implicit artifact ports are auto-injected by `all_ports`.
    pub ports: Vec<MediaPortSpec>,
    pub export_formats: Vec<String>,
    pub import_formats: Vec<String>,
    pub artifact: ArtifactPresentation,
}

impl AppIo {
    /// 🔌️ The implicit `"artifact:in"` port every app accepts, keyed by `self.artifact_media_type`.
    pub async fn artifact_in_port(&self) -> MediaPortSpec {
        MediaPortSpec { id: "artifact:in".into(), label: "Artifact".into(), direction: MediaPortDirection::In, media_type: self.artifact_media_type, kind_id: None, required: true, multiplicity: PortMultiplicity::One }
    }

    /// 🔌️ The implicit `"artifact:out"` port every app produces — see `artifact_in_port`.
    pub async fn artifact_out_port(&self) -> MediaPortSpec {
        MediaPortSpec { id: "artifact:out".into(), label: "Artifact".into(), direction: MediaPortDirection::Out, media_type: self.artifact_media_type, kind_id: None, required: true, multiplicity: PortMultiplicity::One }
    }

    /// 🔌️ The full port list, in stable order: the implicit artifact ports first, followed by every app-specific port declared in `self.ports`.
    pub async fn all_ports(&self) -> Vec<MediaPortSpec> {
        let mut ports = vec![self.artifact_in_port().await, self.artifact_out_port().await];
        ports.extend(self.ports.clone());
        ports
    }

    /// 🏗️ Builds an `AppIo` from just its implicit artifact surface, with no extra ports/formats declared yet — chain `.with_ports(...)` to add app-specific ports.
    pub async fn from_artifact(schema: impl Into<String>, media_type: MediaType, artifact: ArtifactPresentation) -> Self {
        Self { artifact_schema: schema.into(), artifact_media_type: media_type, ports: Vec::new(), export_formats: Vec::new(), import_formats: Vec::new(), artifact }
    }

    /// 🔌️ Attaches app-specific ports (beyond the implicit artifact ports) to this `AppIo`.
    pub async fn with_ports(mut self, ports: Vec<MediaPortSpec>) -> Self {
        self.ports = ports;
        self
    }
}

impl Default for AppIo {
    fn default() -> Self {
        Self {
            artifact_schema: String::new(),
            artifact_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            ports: Vec::new(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            artifact: ArtifactPresentation { id: String::new(), name: String::new(), dimension: String::new(), component_kind: String::new() },
        }
    }
}
//#endregion 🔖️AppIo

//#region 🔖️ConfigSpec
/// 🧮️ An app's full typed configuration record — the manifest-level declaration
/// `AppDefinition.config` carries: one [`ActionArgDef`] per field (its `id` is the config key), so a config field,
/// an action argument and a mutation input share one input vocabulary and one renderer.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ConfigSpec {
    #[serde(default)]
    #[value(default)]
    pub fields: Vec<ActionArgDef>,
}

impl ConfigSpec {
    pub async fn empty() -> Self {
        Self::default()
    }
}
//#endregion 🔖️ConfigSpec

//#region 🔖️CommandGrammar
/// 🎛️ One keyword-dispatched command variant (e.g. `move x=1 y=2`) and its field grammar — one
/// [`ActionArgDef`] per field, `required` where the grammar demands the field.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandVariantSpec {
    pub keyword: String,
    pub fields: Vec<ActionArgDef>,
}

/// 🎛️ An app's full typed binary command grammar — the manifest-level declaration
/// `AppDefinition.command_grammar` carries. Empty until per-app waves populate it.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandGrammar {
    #[serde(default)]
    #[value(default)]
    pub variants: Vec<CommandVariantSpec>,
}

impl CommandGrammar {
    pub async fn empty() -> Self {
        Self::default()
    }
}
//#endregion 🔖️CommandGrammar

//#region Media
/// 🎞️ The value that actually flows over a workflow wire, produced by `ArtifactApp::export_media` and consumed by `ArtifactApp::import_media`. Kept separate from the `MediaType` lattice above (which only negotiates *compatibility*, never carries a value) so headless runners and the UI share one payload shape.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Media {
    pub media_type: MediaType,
    pub payload: MediaPayload,
}

/// 📦️ Structured payloads stay inline as canonical JSON (small, diffable); binary payloads are content-addressed through `store::BlobStore` so a `Media` value never carries megabytes across a WIT boundary.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", tag = "kind")]
pub enum MediaPayload {
    Structured { schema: String, json: String },
    Binary { format_kind: String, blob_hash: String },
}

/// 🔑️ A cheap identity for one port's current output, independent of serializing the full payload — the unit the `SpaceRunner` compares to decide whether a downstream node actually needs to see a new value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MediaFingerprint(pub String);

/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]` doesn't support tuple structs —
/// wire-shaped as the bare inner string, matching serde's own default newtype-struct behavior.
impl ToValue for MediaFingerprint {
    fn to_value(&self) -> DslValue {
        self.0.to_value()
    }
}
impl FromValue for MediaFingerprint {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        String::from_value(value).map(MediaFingerprint)
    }
}

impl MediaFingerprint {
    /// 🔑️ Canonical fingerprint of a `Media` value: structured payloads hash their JSON text, binary payloads reuse their existing content hash directly (no re-hashing bytes already addressed by the blob store).
    pub fn of(media: &Media) -> Self {
        match &media.payload {
            MediaPayload::Structured { schema, json } => MediaFingerprint(semio_framework_hash::hash_parts(&[schema.as_str(), json.as_str()])),
            MediaPayload::Binary { blob_hash, .. } => MediaFingerprint(blob_hash.clone()),
        }
    }
}

/// 🚧️ Failure exporting, importing, or fingerprinting media on a declared port.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaError {
    UnknownPort(String),
    Incompatible { port: String, produced: MediaType, accepted: MediaType },
    Payload(String, String),
    NotImplemented,
}

impl std::fmt::Display for MediaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownPort(port) => write!(formatter, "unknown media port `{port}`"),
            Self::Incompatible { port, produced, accepted } => {
                write!(formatter, "port `{port}` produced {produced:?} but the wire accepts {accepted:?}")
            }
            Self::Payload(port, message) => write!(formatter, "media payload error on port `{port}`: {message}"),
            Self::NotImplemented => formatter.write_str("media ports are not implemented for this app"),
        }
    }
}

impl std::error::Error for MediaError {}

/// 🔀️ A registered one-way conversion the workflow may insert on a wire when `media_types_compatible` reports `MediaCompat::Convert`. Kept behind a trait (never a bare closure) so converters can be enumerated, tested, and swapped without touching the runner.
pub trait MediaConverter: Send + Sync {
    fn source_form(&self) -> impl std::future::Future<Output = MediaForm> + Send;
    fn to_form(&self) -> impl std::future::Future<Output = MediaForm> + Send;
    fn convert(&self, media: &Media) -> impl std::future::Future<Output = Result<Media, MediaError>> + Send;
}
//#endregion Media
//#endregion 🔖️MediaVocabulary

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
