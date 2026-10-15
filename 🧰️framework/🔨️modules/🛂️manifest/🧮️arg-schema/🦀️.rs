//! 🧮️ Argument schemas, action arguments, number facets and mutation-input resolution of the manifest:
//! `ArgSchema`, `ActionArgDef`, `mutation_input_defs` and the `InputSchemaReader` that reads a mutation input schema.
//!
//! Split from the manifest root by section C of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/SEAM-MAP-PLUGIN-MANIFEST-HUB.md`.
use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ArgSchema {
    String {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        options: Vec<ActionArgOption>,
        /// 🗝️ Where a choice's options come from when no `enum` lists them: the keys found in the edited document
        /// ([`OptionSource`]). A UI fact only — the payload schema stays open, so the fold decides an unknown key.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        option_source: Option<OptionSource>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SnapSource {
    Step,
    Config { key: String },
    Snapshot { pointer: String },
}

/// 🗝️ Where a string choice's options come from (`x-semio-ui.optionSource`): the KEYS of the object `pointer` leads to in
/// the document the editor previews — an RFC 6901 template whose `{field}` segments take the edited payload's top-level
/// member: an object's key, or, on an array, the record whose own `field` equals it (`/hostSnapshot/widgets/{id}/params`).
/// TypeScript twin: `OptionSource`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum OptionSource {
    Snapshot { pointer: String },
}

/// 📈️ How a slider or dial maps its travel onto the value range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum NumberScale {
    Linear,
    Log,
}

/// 🔢️ The JSON type of a reference's ids: text ids (the default, left off the wire) or integer ids. A selection id is
/// always text, so an integer reference stages the integer its selected text spells ([`ReferenceIdType::id_value`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActionArgControl {
    Text {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        placeholder: Option<String>,
    },
    /// 📜️ A multi-line text field (a plain string declared `presentation: Multiline`): Enter inserts a line,
    /// Ctrl/⌘+Enter commits, Escape reverts — the text-control keyboard law (`semio_framework_ui_contract::text_input_key`).
    Multiline,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
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
    pub(super) fn with_schema(id: impl Into<String>, label: impl Into<LocalizedLabel>, schema: ArgSchema) -> Self {
        Self { id: id.into(), label: label.into(), schema, presentation: None, required: false, nullable: false, default: None, description: None, group: None, order: None }
    }

    pub(super) fn plain_string(format: Option<ArgFormat>) -> ArgSchema {
        ArgSchema::String { options: Vec::new(), option_source: None, min_len: None, max_len: None, pattern: None, format }
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
        Self::with_schema(id, label, ArgSchema::String { options, option_source: None, min_len: None, max_len: None, pattern: None, format: None })
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
    /// `semio_framework_value::FromValue::from_value` (`setFrame.frame`, `setSource.source`), previously unexpressible, so
    /// those verbs published an empty input schema and no agent could ever call them.
    pub fn object(id: impl Into<String>, label: impl Into<LocalizedLabel>, fields: Vec<ActionArgDef>) -> Self {
        Self::with_schema(id, label, ArgSchema::Object { fields })
    }

    /// 🧬️ An unconstrained typed value argument.
    pub fn any(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, ArgSchema::Any)
    }

    /// 🧬️ A JSON-text argument — a `String` wire field that actually carries a JSON document
    /// (`patchLayer.value`, `loadDocumentJson.json`), tagged `x-semio-format: json` so a client knows
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
    /// Segmented) over any format; a plain string declared multiline is Multiline; a number's
    /// presentation wins, else an integer steps, else a fully-bounded number slides, else it is a
    /// plain number field. TypeScript twin: `argControl`.
    pub fn control(&self) -> ActionArgControl {
        match &self.schema {
            ArgSchema::String { options, option_source, format, .. } => {
                if !options.is_empty() || option_source.is_some() {
                    return match self.presentation {
                        Some(ArgPresentation::Segmented) => ActionArgControl::Segmented { options: options.clone() },
                        _ => ActionArgControl::Select { options: options.clone() },
                    };
                }
                match format {
                    Some(ArgFormat::IconId) => ActionArgControl::IconSelect { classifier_kind: "icon".to_string() },
                    Some(ArgFormat::ArtifactKind { roles }) => ActionArgControl::ArtifactKind { roles: roles.clone() },
                    Some(ArgFormat::SurfaceApp { roles, dialect_arg }) => ActionArgControl::SurfaceApp { roles: roles.clone(), dialect_arg: dialect_arg.clone() },
                    _ if self.presentation == Some(ArgPresentation::Multiline) => ActionArgControl::Multiline,
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
/// `multipleOf`, and so are the options an `optionSource` resolved from the document — never an `enum`, the fold decides an
/// unknown key. No `additionalProperties`/`$schema`/`$id` at this altitude; the catalog compiler owns the envelope.
fn arg_schema_json_schema(schema: &ArgSchema, terminology: Terminology, locale: Locale) -> DslValue {
    match schema {
        ArgSchema::String { options, option_source, min_len, max_len, pattern, format } => {
            let mut entries = vec![("type".to_string(), DslValue::String("string".to_string()))];
            if !options.is_empty() && option_source.is_none() {
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

//#region 🔖️ActionArgFacets
/// 🎛️ The UI-contract number facets of one input — the single mapping every renderer of an [`ActionArgDef`] shares (the
/// time-travel editor, both shells' staged dialog fields): its key range (a slider's or dial's travel; a stepper's, number
/// field's or vector axis' hard bounds, an excluded bound being none since no key lands on it), the step, the look (`Dial`
/// for a dial presentation) and axis, the stored and shown unit as symbols ([`action_arg_unit_symbol`]), the display factor
/// and precision, the declared detents the detent law and the limits admit, and the hard limits of the schema with their
/// refusals naming the bound in display units ("Must be greater than 0", "Darf höchstens 180 ° sein"). TypeScript twin:
/// `actionArgNumberFacets`; fixture `🧫️fixtures/🧫️number-facets`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionArgNumberFacets {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub appearance: semio_framework_ui_contract::SliderAppearance,
    pub scale: semio_framework_ui_contract::UiNumberScale,
    pub unit: Option<String>,
    pub display_unit: Option<String>,
    pub display_factor: Option<f64>,
    pub precision: Option<u16>,
    pub snaps: Vec<f64>,
    pub limits: semio_framework_ui_contract::UiNumberLimits,
}

impl ActionArgNumberFacets {
    /// 🏷️ The unit a value shows with — the display unit, else the stored one.
    pub fn shown_unit(&self) -> Option<&str> {
        self.display_unit.as_deref().or(self.unit.as_deref())
    }
}

/// 🔣️ The symbol a descriptor's unit name shows as (`deg` reads `°`, `percent` reads `%`); any other unit as written.
pub fn action_arg_unit_symbol(unit: &str) -> String {
    match unit {
        "deg" | "degree" | "degrees" => "°".to_string(),
        "percent" => "%".to_string(),
        other => other.to_string(),
    }
}

/// 🚧️ The refusal of a typed value crossing a hard bound, `{bound}` naming it — framework copy, EN and DE, no default.
fn action_arg_bound_refusal(below: bool, exclusive: bool, locale: Locale) -> &'static str {
    match (locale, below, exclusive) {
        (Locale::En, true, false) => "Must be at least {bound}",
        (Locale::En, true, true) => "Must be greater than {bound}",
        (Locale::En, false, false) => "Must be at most {bound}",
        (Locale::En, false, true) => "Must be less than {bound}",
        (Locale::De, true, false) => "Muss mindestens {bound} sein",
        (Locale::De, true, true) => "Muss größer als {bound} sein",
        (Locale::De, false, false) => "Darf höchstens {bound} sein",
        (Locale::De, false, true) => "Muss kleiner als {bound} sein",
    }
}

impl ActionArgDef {
    /// 🎛️ The [`ActionArgNumberFacets`] of the control [`Self::control`] derives, its refusals in `locale` — `None` unless
    /// the input is a number field, stepper, slider, dial or vector.
    pub fn number_facets(&self, locale: Locale) -> Option<ActionArgNumberFacets> {
        use semio_framework_ui_contract::{SliderAppearance, UiNumberBound, UiNumberLimits, UiNumberScale};
        let (hard_min, hard_max, min_exclusive, max_exclusive) = match &self.schema {
            ArgSchema::Number { min, max, min_exclusive, max_exclusive, .. } => (*min, *max, *min_exclusive, *max_exclusive),
            ArgSchema::Vector { min, max, .. } => (*min, *max, false, false),
            _ => return None,
        };
        let number = |min: Option<f64>, max: Option<f64>, step: Option<f64>, unit: Option<String>, precision: Option<u32>, display_unit: Option<String>, display_factor: Option<f64>, snaps: Vec<f64>| ActionArgNumberFacets {
            min,
            max,
            step: step.filter(|step| step.is_finite() && *step > 0.0),
            unit: unit.as_deref().map(action_arg_unit_symbol),
            display_unit: display_unit.as_deref().map(action_arg_unit_symbol),
            display_factor,
            precision: precision.map(|precision| u16::try_from(precision).unwrap_or(u16::MAX).min(semio_framework_ui_contract::UI_NUMBER_PRECISION_MAX)),
            snaps,
            ..ActionArgNumberFacets::default()
        };
        let key = |bound: Option<f64>, excluded: bool| bound.filter(|_| !excluded);
        let mut facets = match self.control() {
            ActionArgControl::Slider { min, max, step, unit, precision, display_unit, display_factor, snaps, scale, .. } => ActionArgNumberFacets { scale: if scale == Some(NumberScale::Log) { UiNumberScale::Log } else { UiNumberScale::Linear }, ..number(Some(min), Some(max), step, unit, precision, display_unit, display_factor, snaps) },
            ActionArgControl::Dial { min, max, step, unit, precision, display_unit, display_factor, snaps, .. } => ActionArgNumberFacets { appearance: SliderAppearance::Dial, ..number(Some(min), Some(max), step, unit, precision, display_unit, display_factor, snaps) },
            ActionArgControl::Stepper { min, max, step, unit, precision, display_unit, display_factor, snaps, .. } => number(key(min, min_exclusive), key(max, max_exclusive), step, unit, precision, display_unit, display_factor, snaps),
            ActionArgControl::Number { min, max, step, unit, precision, display_unit, display_factor } => number(key(min, min_exclusive), key(max, max_exclusive), step, unit, precision, display_unit, display_factor, Vec::new()),
            ActionArgControl::Vector { min, max, step, unit, precision, display_unit, display_factor, snaps, .. } => number(min, max, step, unit, precision, display_unit, display_factor, snaps),
            _ => return None,
        };
        let shown_unit = facets.shown_unit().map(ToOwned::to_owned);
        let bound = |value: f64, exclusive: bool, below: bool| {
            let text = semio_framework_ui_contract::ui_number_display_text(value, facets.display_factor, None);
            let text = shown_unit.as_ref().map_or_else(|| text.clone(), |unit| format!("{text} {unit}"));
            let refusal = action_arg_bound_refusal(below, exclusive, locale).replace("{bound}", &text);
            UiNumberBound { value, exclusive, refusal: Some(semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::clipped(&refusal))) }
        };
        let limits = UiNumberLimits { min: hard_min.map(|value| bound(value, min_exclusive, true)), max: hard_max.map(|value| bound(value, max_exclusive, false)) };
        let (low, high) = (facets.min.unwrap_or(f64::NEG_INFINITY), facets.max.unwrap_or(f64::INFINITY));
        let mut previous = f64::NEG_INFINITY;
        facets.snaps.retain(|&snap| {
            let kept = snap.is_finite() && snap > previous && (low..=high).contains(&snap) && limits.crossed(snap).is_none();
            if kept {
                previous = snap;
            }
            kept
        });
        facets.limits = limits;
        Some(facets)
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🧪️number-facets/🦀️.rs"]
mod number_facets_tests;
//#endregion 🔖️ActionArgFacets

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum InputSchemaErrorCode {
    Malformed,
    RefUnresolved,
    UiInvalid,
    WidgetIncompatible,
    LabelMissing,
    OptionLabelMissing,
    LocaleMissing,
    WordOnlyFloat,
}

/// 🚫️ Why a mutation payload schema yields no input descriptors: the `code`, the RFC 6901 `pointer` of the input
/// in the payload (`""` for the payload itself, `-` for "every array item") and a human `detail`.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
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
const INPUT_LABEL_GLOSSARY_JSON: &str = include_str!("../🔣️input-labels.json");

/// 📚️ The parsed [`INPUT_LABEL_GLOSSARY_JSON`], field name → label.
pub fn input_label_glossary() -> &'static BTreeMap<String, LocalizedLabel> {
    static GLOSSARY: std::sync::OnceLock<BTreeMap<String, LocalizedLabel>> = std::sync::OnceLock::new();
    GLOSSARY.get_or_init(|| {
        let document = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(INPUT_LABEL_GLOSSARY_JSON, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the input label glossary is JSON"));
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
/// (artifact facets and named scope exports) and every document a published mutation leaf references
/// (`semio_framework_schema_registry::registered_referenced_schema_documents`), and the framework's own shared documents
/// ([`FRAMEWORK_INPUT_SCHEMA_DOCUMENTS`], held without any registration): the document whose `$id` is `id`, or `None`
/// when nobody published it.
pub fn registered_input_schema_document(id: &str) -> Option<DslValue> {
    let mut texts: Vec<&'static str> = semio_framework_schema_registry::with_schema_export_registry(|registry| {
        registry.entries().filter(|entry| entry.format == semio_framework_schema_registry::SchemaFormat::JsonSchema).filter_map(|entry| registry.resolve(entry.scope, entry.export, entry.format).ok()).filter(|text| text.contains(id)).collect()
    });
    texts.extend(semio_framework_schema_registry::registered_referenced_schema_documents().into_iter().filter(|text| text.contains(id)));
    texts.extend(FRAMEWORK_INPUT_SCHEMA_DOCUMENTS.into_iter().filter(|text| text.contains(id)));
    texts.into_iter().find_map(|text| {
        let document = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?);
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

/// 🔢️ The framework value schema, whose `$defs/Binary64` (the exact binary64 word) and `$defs/Binary64Transport` (that word or a
/// plain number) define the numeric transport the reader edits as a number wherever a carrier references or restates it.
const INPUT_NUMERIC_TRANSPORT_SCHEMA_JSON: &str = include_str!("../../🌱️value/🧬️schema/🔣️.json");

/// 🪣️ The shared schema documents of the framework's own namespace an input schema of any plugin may `$ref`:
/// [`registered_input_schema_document`] always holds them, so publishing them is never a plugin's duty (design §23).
/// Today the framework value schema (`framework/value/schema.json`); the OS adds its own at every app construction.
pub const FRAMEWORK_INPUT_SCHEMA_DOCUMENTS: [&str; 1] = [INPUT_NUMERIC_TRANSPORT_SCHEMA_JSON];

/// 🏷️ Schema keys that annotate without constraining; a shape comparison ignores them (and every `x-` key).
const INPUT_SHAPE_ANNOTATIONS: [&str; 6] = ["title", "description", "$comment", "examples", "default", "format"];

/// 🧬️ The constraint shape of a schema node: annotations dropped, object keys sorted, `required` sorted — equal shapes
/// validate the same instances.
fn input_shape(node: &DslValue) -> DslValue {
    match node {
        DslValue::Object(entries) => {
            let mut kept: Vec<(String, DslValue)> = entries
                .iter()
                .filter(|(key, _)| !key.starts_with("x-") && !INPUT_SHAPE_ANNOTATIONS.contains(&key.as_str()))
                .map(|(key, value)| match (key.as_str(), value) {
                    ("required", DslValue::Array(names)) => {
                        let mut names = names.clone();
                        names.sort_by(|left, right| left.as_str().cmp(&right.as_str()));
                        (key.clone(), DslValue::Array(names))
                    }
                    _ => (key.clone(), input_shape(value)),
                })
                .collect();
            kept.sort_by(|left, right| left.0.cmp(&right.0));
            DslValue::Object(kept)
        }
        DslValue::Array(items) => DslValue::Array(items.iter().map(input_shape).collect()),
        other => other.clone(),
    }
}

/// 🔢️ The shapes of the framework's numeric transport: the binary64 word and the plain-number branch of the transport.
fn input_numeric_transport() -> &'static (DslValue, DslValue) {
    static SHAPES: std::sync::OnceLock<(DslValue, DslValue)> = std::sync::OnceLock::new();
    SHAPES.get_or_init(|| {
        let schema = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(INPUT_NUMERIC_TRANSPORT_SCHEMA_JSON, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the framework value schema is JSON"));
        let definition = |name: &str| schema.get("$defs").and_then(|definitions| definitions.get(name)).cloned().unwrap_or_else(|| panic!("the framework value schema defines {name}"));
        let number = input_union(&definition("Binary64Transport")).and_then(|branches| branches.iter().find(|branch| branch.get("$ref").is_none())).map(input_shape).expect("the framework numeric transport has a plain-number branch");
        (input_shape(&definition("Binary64")), number)
    })
}

/// 🔀️ One variant of a discriminated payload union: the `const` its discriminator pins, the resolved member, and that
/// member's resolved discriminator property.
struct InputVariant {
    value: String,
    member: ResolvedInput,
    discriminator: ResolvedInput,
}

const INPUT_UI_KEYS: [&str; 19] = ["widget", "role", "label", "description", "step", "precision", "softMin", "softMax", "snaps", "snapSource", "optionSource", "unit", "displayUnit", "displayFactor", "scale", "group", "order", "options", "ref"];
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
        let root = semio_framework_pack_json::parse(schema_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| input_error(InputSchemaErrorCode::Malformed, "", error.to_string()))?;
        Ok(Self {
            root: semio_framework_pack_json::to_dsl_value(&root),
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
            if self.numeric_transport(&resolved.document, &resolved.node, pointer) {
                if input_shape(&resolved.node) == input_numeric_transport().0 {
                    self.recover(Err(input_error(InputSchemaErrorCode::WordOnlyFloat, pointer, "a number input accepts only the exact binary64 word, never the plain number its payload and every draft carry; reference framework/value/schema.json#/$defs/Binary64Transport")), || ())?;
                }
                resolved.node = DslValue::Object(vec![("type".to_string(), DslValue::String("number".to_string()))]);
            }
            return Ok(resolved);
        }
        Err(input_error(InputSchemaErrorCode::Malformed, pointer, "a $ref chain exceeds 32 hops"))
    }

    /// 🔢️ Whether `node` (in `document`) is the framework's numeric transport ([`input_numeric_transport`]): the binary64 word
    /// itself (read as a number but refused as `WordOnlyFloat`: the payload and every draft carry the plain number), or a
    /// two-branch union of that word and a plain number (each branch restated or referenced) with nothing but annotations
    /// beside it — an input edited as a number, its facets from the property's `x-semio-ui`.
    fn numeric_transport(&self, document: &Option<String>, node: &DslValue, pointer: &str) -> bool {
        let (word, number) = input_numeric_transport();
        if input_shape(node) == *word {
            return true;
        }
        let Some(branches) = input_union(node).filter(|branches| branches.len() == 2) else { return false };
        if node.as_object().is_some_and(|entries| entries.iter().any(|(key, _)| !matches!(key.as_str(), "anyOf" | "oneOf") && !key.starts_with("x-") && !INPUT_SHAPE_ANNOTATIONS.contains(&key.as_str()))) {
            return false;
        }
        let shapes: Vec<DslValue> = branches
            .iter()
            .map(|branch| {
                let (mut owner, mut current) = (document.clone(), branch.clone());
                for _ in 0..INPUT_REF_DEPTH {
                    let Some(reference) = current.get("$ref").and_then(DslValue::as_str).map(str::to_string) else { break };
                    let Ok(target) = self.target(&owner, &reference, pointer) else { return DslValue::Null };
                    (owner, current) = target;
                }
                input_shape(&current)
            })
            .collect();
        shapes.contains(word) && shapes.contains(number)
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
        let mut inputs = vec![ActionArgDef { id: input_pointer("", &key), label, schema: ArgSchema::String { options, option_source: None, min_len: None, max_len: None, pattern: None, format: None }, presentation, required: true, nullable: false, default: None, description, group: None, order: None }];
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
            (Some("select" | "segmented"), ArgSchema::String { options, option_source, .. }) => !options.is_empty() || option_source.is_some(),
            (Some("text" | "multiline"), ArgSchema::String { options, option_source, .. }) => options.is_empty() && option_source.is_none(),
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
        if input.ui("optionSource").is_some() && (kind != Some("string") || input.node.get("enum").is_some()) {
            self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "optionSource only sources the options of a string without an enum")), || ())?;
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
                option_source: input_option_source(input, pointer)?,
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
            Some("string") => Ok(ArgSchema::String { options: input_options(items, pointer)?, option_source: input_option_source(items, pointer)?, min_len: None, max_len: None, pattern: None, format: None }),
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

/// 🗝️ A string input's `x-semio-ui.optionSource`: `{snapshot: "<pointer template>"}`, an RFC 6901 template (`/…`) whose
/// `{field}` segments name members of the edited payload.
fn input_option_source(input: &ResolvedInput, pointer: &str) -> Result<Option<OptionSource>, InputSchemaError> {
    match input.ui("optionSource").map(DslValue::as_object) {
        None => Ok(None),
        Some(Some([(name, DslValue::String(template))])) if name == "snapshot" && template.starts_with('/') => Ok(Some(OptionSource::Snapshot { pointer: template.clone() })),
        Some(_) => Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "optionSource is {snapshot: pointer template}")),
    }
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
#[path = "../🧪️tests/🧪️mutation-inputs/🦀️.rs"]
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
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
