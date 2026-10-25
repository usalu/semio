//! 📐️ Argument format and schema: the argument formats, the argument schema with its accessors and the interaction entity kind. See [`super`].

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
    /// 📃️ One addressed target's own revision token (a page item, a cell, an entry), as its rendered binding carries it —
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
    /// 🌱️ Every entity kind this value names through [`ArgFormat::EntityId`], at any depth.
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
//#endregion 🔖️ArgSchema
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
