//! 🎛️ Action argument control: the argument option and the control that renders and constrains one action argument. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️ActionArgs
/// 🎬️ One selectable option of a `Select` argument control — the persisted `value` and its
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
    /// 🧳️ `min`/`max` are the travel range: the soft range when one is declared, else the hard bounds.
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
    /// 💠️ One number field per component, sharing the component facets of [`ArgSchema::Vector`].
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
//#endregion 🔖️ActionArgs
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
