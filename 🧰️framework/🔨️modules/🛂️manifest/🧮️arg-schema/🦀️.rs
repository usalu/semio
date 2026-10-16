//! 🧮️ Action argument schema: formats, schemas, controls and argument definitions.

use semio_framework_pack_json::ArtifactCanonicalJsonText as Text;
use semio_framework_ui_contract::SliderAppearance;
use semio_framework_ui_contract::UiNumberBound;
use semio_framework_ui_contract::UiNumberLimits;
use semio_framework_ui_contract::UiNumberScale;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::Number;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::any::Any;
use std::error::Error;
use semio_framework_kernel_identity::ArtifactKind;

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
    pub fn with_schema(id: impl Into<String>, label: impl Into<LocalizedLabel>, schema: ArgSchema) -> Self {
        Self { id: id.into(), label: label.into(), schema, presentation: None, required: false, nullable: false, default: None, description: None, group: None, order: None }
    }

    pub fn plain_string(format: Option<ArgFormat>) -> ArgSchema {
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

//#endregion 🔖️ActionArgFacets
//#region 🔖️Surface
/// 👁️✏️ Whether a surface may change the artifact it is bound to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_value::RetireOwned)]
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
            DslValue::String(s) => s.parse::<AppRole>().map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error)),
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
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

//#endregion 🔖️Surface
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️ActionArgFacets
#[cfg(test)]
#[path = "../🧪️tests/🧪️number-facets/🦀️.rs"]
mod number_facets_tests;
//#endregion 🔖️ActionArgFacets
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
