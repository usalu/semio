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

/// @emoji ⌨️ Operating system selector for a platform-specific keybinding.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum Platform {
    MacOs,
    Windows,
    Linux,
}

/// @emoji ⌨️ One command chord, optionally restricted to a host platform.
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

/// @emoji 🗂️ Classifies a declared action by how it interacts with VCS history.
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
/// @emoji 🧬️ Semantic refinement of a `String`-typed `ArgSchema` leaf — what KIND of string this is,
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
}

/// @emoji 🌳️ The stored, engine-neutral shape of one action argument's value — see this region's
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
        #[serde(default)]
        #[value(default)]
        integer: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    Boolean,
    Vec3 {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
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

/// @emoji 🖼️ How to WIDGET-render an argument beyond what its `ArgSchema` alone implies — consumed by
/// `ActionArgDef::control()` (e.g. a bounded `Number` still renders `Slider` without this, but a
/// single-bound one needs it to opt in).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ArgPresentation {
    Slider,
    IconSelect { classifier_kind: String },
    Multiline,
    Hidden,
}
//#endregion 🔖️ArgSchema

//#region 🔖️ActionArgs
/// @emoji 🔘️ One selectable option of a `Select` argument control — the persisted `value` and its
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

/// @emoji 🎚️ Declarative input control for one action argument — a lean manifest-altitude enum,
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
    },
    Slider {
        min: f64,
        max: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    Toggle,
    Select {
        options: Vec<ActionArgOption>,
    },
    Vec3,
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

/// @emoji 📝️ Declares one argument of an action: its `id` (the JSON key sent in `ActionDescriptor.args`),
/// human `label`, stored value `schema` (see `🔖️ArgSchema` — D6: this is the sole persisted truth,
/// `control()` below is derived from it), an optional widget `presentation` hint, whether it is
/// `required`, an optional `default` value, and an optional `description`. An empty
/// `ActionDefinition.args` (the common case) means a no-argument action.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub default: Option<DslValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ActionArgDef {
    fn with_schema(id: impl Into<String>, label: impl Into<LocalizedLabel>, schema: ArgSchema) -> Self {
        Self { id: id.into(), label: label.into(), schema, presentation: None, required: false, default: None, description: None }
    }

    fn plain_string(format: Option<ArgFormat>) -> ArgSchema {
        ArgSchema::String { options: Vec::new(), min_len: None, max_len: None, pattern: None, format }
    }

    /// @emoji 🔤️ A free-text argument.
    pub fn text(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, Self::plain_string(None))
    }

    /// @emoji 🔢️ A numeric argument (unbounded stepper by default).
    pub fn number(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, ArgSchema::Number { min: None, max: None, step: None, integer: false, unit: None })
    }

    /// @emoji 🎚️ A bounded slider argument.
    pub fn slider(id: impl Into<String>, label: impl Into<LocalizedLabel>, min: f64, max: f64) -> Self {
        let mut def = Self::with_schema(id, label, ArgSchema::Number { min: Some(min), max: Some(max), step: None, integer: false, unit: None });
        def.presentation = Some(ArgPresentation::Slider);
        def
    }

    /// @emoji 🔘️ A boolean toggle argument.
    pub fn toggle(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, ArgSchema::Boolean)
    }

    /// @emoji 🔽️ A single-choice select argument.
    pub fn select(id: impl Into<String>, label: impl Into<LocalizedLabel>, options: Vec<ActionArgOption>) -> Self {
        Self::with_schema(id, label, ArgSchema::String { options, min_len: None, max_len: None, pattern: None, format: None })
    }

    /// @emoji 🧭️ A three-component vector argument.
    pub fn vec3(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        Self::with_schema(id, label, ArgSchema::Vec3 { unit: None })
    }

    /// @emoji 🗂️ A host-resolved artifact-kind choice — see `ActionArgControl::ArtifactKind`.
    pub fn artifact_kind(id: impl Into<String>, label: impl Into<LocalizedLabel>, roles: Vec<AppRole>) -> Self {
        Self::with_schema(id, label, Self::plain_string(Some(ArgFormat::ArtifactKind { roles })))
    }

    /// @emoji 🎭️ A host-resolved `(pluginId, appId, role)` choice — see `ActionArgControl::SurfaceApp`.
    pub fn surface_app(id: impl Into<String>, label: impl Into<LocalizedLabel>, roles: Vec<AppRole>, dialect_arg: impl Into<String>) -> Self {
        Self::with_schema(id, label, Self::plain_string(Some(ArgFormat::SurfaceApp { roles, dialect_arg: dialect_arg.into() })))
    }

    /// @emoji ❗️ Marks the argument as required — execution is blocked until it has an effective value.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// @emoji 🎁️ Sets the default effective value used when nothing is staged.
    pub fn default_value(mut self, value: &impl ToValue) -> Self {
        self.default = Some(value.to_value());
        self
    }

    /// @emoji 💬️ Attaches a description shown alongside the field.
    pub fn describe(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// @emoji 🎛️ Derives this argument's renderer-facing `ActionArgControl` from its stored `schema` +
    /// `presentation` — D6 (ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet
    /// P3-manifest-schema): `schema` is the ONLY persisted truth, this is computed fresh on every
    /// call, never cached/stored. Order matters: a non-empty `options` list always wins Select over
    /// any format; a `Slider` presentation or a fully-bounded `Number` wins Slider over plain Number.
    pub fn control(&self) -> ActionArgControl {
        match &self.schema {
            ArgSchema::String { options, format, .. } => {
                if !options.is_empty() {
                    return ActionArgControl::Select { options: options.clone() };
                }
                match format {
                    Some(ArgFormat::IconId) => ActionArgControl::IconSelect { classifier_kind: "icon".to_string() },
                    Some(ArgFormat::ArtifactKind { roles }) => ActionArgControl::ArtifactKind { roles: roles.clone() },
                    Some(ArgFormat::SurfaceApp { roles, dialect_arg }) => ActionArgControl::SurfaceApp { roles: roles.clone(), dialect_arg: dialect_arg.clone() },
                    _ => ActionArgControl::Text { placeholder: None },
                }
            }
            ArgSchema::Number { min, max, step, unit, .. } => {
                if matches!(self.presentation, Some(ArgPresentation::Slider)) || (min.is_some() && max.is_some()) {
                    ActionArgControl::Slider { min: min.unwrap_or(0.0), max: max.unwrap_or(0.0), step: *step, unit: unit.clone() }
                } else {
                    ActionArgControl::Number { min: *min, max: *max, step: *step }
                }
            }
            ArgSchema::Boolean => ActionArgControl::Toggle,
            ArgSchema::Vec3 { .. } => ActionArgControl::Vec3,
            ArgSchema::Array { .. } | ArgSchema::Object { .. } | ArgSchema::Any => ActionArgControl::Text { placeholder: None },
        }
    }

    /// @emoji 📐️ JSON Schema (2020-12 leaf, no `$schema`/`$id` — the catalog compiler wraps those at
    /// the whole-action envelope, `📋️master.md` §3.2) for this one argument's value, folding in
    /// `description`/`default`.
    pub fn json_schema(&self) -> DslValue {
        let DslValue::Object(mut entries) = arg_schema_json_schema(&self.schema) else {
            unreachable!("arg_schema_json_schema always returns an object");
        };
        if let Some(description) = &self.description {
            entries.push(("description".to_string(), DslValue::String(description.clone())));
        }
        if let Some(default) = &self.default {
            entries.push(("default".to_string(), default.clone()));
        }
        DslValue::Object(entries)
    }
}

/// @emoji 🧬️ Tags a leaf/nested `ArgSchema` JSON Schema object with its `ArgFormat` — `x-semio-format`
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

/// @emoji 📐️ JSON Schema 2020-12 for one `ArgSchema` node (recursive over `Array`/`Object`) — carries
/// `Number.unit`/`Vec3.unit` as `x-semio-unit`, `String.format` via `apply_arg_format`. No
/// `additionalProperties`/`$schema`/`$id` at this altitude; the catalog compiler owns the envelope.
fn arg_schema_json_schema(schema: &ArgSchema) -> DslValue {
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
        ArgSchema::Number { min, max, step, integer, unit } => {
            let mut entries = vec![("type".to_string(), DslValue::String(if *integer { "integer" } else { "number" }.to_string()))];
            if let Some(min) = min {
                entries.push(("minimum".to_string(), min.to_value()));
            }
            if let Some(max) = max {
                entries.push(("maximum".to_string(), max.to_value()));
            }
            if let Some(step) = step {
                entries.push(("multipleOf".to_string(), step.to_value()));
            }
            if let Some(unit) = unit {
                entries.push(("x-semio-unit".to_string(), DslValue::String(unit.clone())));
            }
            DslValue::Object(entries)
        }
        ArgSchema::Boolean => DslValue::object([("type".to_string(), DslValue::String("boolean".to_string()))]),
        ArgSchema::Vec3 { unit } => {
            let mut entries = vec![
                ("type".to_string(), DslValue::String("array".to_string())),
                ("items".to_string(), DslValue::object([("type".to_string(), DslValue::String("number".to_string()))])),
                ("minItems".to_string(), DslValue::uint(3)),
                ("maxItems".to_string(), DslValue::uint(3)),
            ];
            if let Some(unit) = unit {
                entries.push(("x-semio-unit".to_string(), DslValue::String(unit.clone())));
            }
            DslValue::Object(entries)
        }
        ArgSchema::Array { items, min_items, max_items } => {
            let mut entries = vec![("type".to_string(), DslValue::String("array".to_string())), ("items".to_string(), arg_schema_json_schema(items))];
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
                properties.push((field.id.clone(), field.json_schema()));
                if field.required {
                    required.push(DslValue::String(field.id.clone()));
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

/// @emoji 🎛️ Canonical catalog icon for a declared app mode id.
pub async fn catalog_mode_icon_id(id: &str) -> IconName {
    match id {
        "edit" | "main" => "pencil".into(),
        "paint" => "paintbrush".into(),
        "generate" => "sparkles".into(),
        "explore" => "focus".into(),
        "builder" => "component".into(),
        "curation" => "folder-open".into(),
        "blueprint" => "cad-shape".into(),
        "review" => "search".into(),
        "report" => "bar-chart-3".into(),
        "view" => "eye".into(),
        "capture" => "camera".into(),
        "model" => "box".into(),
        "analyze" => "search".into(),
        _ => "layers".into(),
    }
}

/// @emoji 🧪️ Canonical catalog icon for a playground example id (content-specific ids override at declaration).
pub async fn catalog_example_icon_id(id: &str) -> IconName {
    match id {
        "empty" | "default" => "file".into(),
        "demo" => "cylinder".into(),
        "semio" => "sparkles".into(),
        _ if id.contains("capsule") || id.contains("nakagin") => "building".into(),
        _ if id.contains("forest") || id.contains("concrete") => "list-tree".into(),
        _ if id.contains("hex") => "hexagon".into(),
        _ => "file-text".into(),
    }
}

/// @emoji 🎯️ Canonical catalog icon for a declared action id (view/shell/operation/history/clipboard).
pub fn catalog_action_icon_id(id: &str, kind: ActionKind) -> IconName {
    match id {
        "undo" => "undo-2".into(),
        "redo" => "redo-2".into(),
        "commitCheckpoint" => "git-commit".into(),
        "createAlternative" => "git-branch".into(),
        "switchAlternative" => "git-branch".into(),
        "checkoutCheckpoint" => "git-branch".into(),
        "revertToCommand" => "clock".into(),
        "copy" => "copy".into(),
        "cut" => "scissors".into(),
        "paste" => "clipboard".into(),
        "setHistoryCommandFilter" => "list".into(),
        "noteShellCommand" => "book-open".into(),
        "setActiveUtility" => "wrench".into(),
        "setActiveTool" => "hammer".into(),
        "startIntroduction" => "graduation-cap".into(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W1: the six framework-owned
        // Interaction actions (`interaction_action_definitions`) replace every per-app
        // `setSelection`/`documentSelect`/`selectNode`/`nodeGraphSelect`/`setNodeSelection`/
        // `setFeatureSelection`/`setReferenceSelection`/`setMediaNodeSelection`/
        // `setAppInstanceSelection`/`selectRegister`/`selectInstance`/`selectSameKind`/
        // `selectSameKindSelection`/`worldSelect`/`worldVortexSelect`/`deselect`/`worldHover`/
        // `setHover`/`nodeGraphHover`/`textHover`/`referenceHover` id that used to live here — those
        // arms are deleted, not merely renamed (per-app selection/hover commands dissolve in wave 4).
        "interactionSelect" => "mouse-pointer".into(),
        "interactionHover" => "eye".into(),
        "clearSelection" => "mouse-pointer-2".into(),
        "selectAll" => "maximize-2".into(),
        "setSelectionMode" => "sliders-horizontal".into(),
        "setInteractionGranularity" => "layers".into(),
        "setCamera" | "setCamera2d" | "setCamera3d" | "nodeGraphViewport" => "camera".into(),
        "setProjection" | "setProjectionParam" => "scan".into(),
        "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" | "graphPointerDown" | "worldPointerDown" => "mouse-pointer".into(),
        "worldPick" => "crosshair".into(),
        "engagementInput" | "engagementAbort" | "engagementControlSelect" | "editorEngagementInput" | "graphEngagementInput" | "resultsEngagementInput" | "workflowEngagementInput" | "compiledDagEngagementInput" => "hand".into(),
        "setLodMode" => "layers".into(),
        "toggleGrid" | "setGridSnapEnabled" | "setGridFactor" => "grid-3x3".into(),
        "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => "sun".into(),
        "run" | "stop" => "play".into(),
        "search" => "search".into(),
        "exportProgram" | "exportRegistersCsv" | "exportMedia" | "exportStudioPack" | "exportStudioDsl" | "exportVideoFromDeck" => "download".into(),
        "importMedia" | "importSpacePack" | "importFrames" | "importVideo" | "openSource" => "hard-drive".into(),
        "goHome" => "home".into(),
        "openSpace" | "openInstance" => "folder-open".into(),
        "navigateVirtualFileSystemNode" => "folder".into(),
        "setActiveExample" | "setActivePanelTab" => "panel-left".into(),
        "copyPrompt" => "copy".into(),
        "evaluate" => "hash".into(),
        "recomputeRewrite" | "reorganize" => "rotate-cw".into(),
        "textEdit" | "formatDocument" | "requestCompletions" => "typography".into(),
        "textSelect" => "text-cursor".into(),
        "paintStrokeBegin" | "paintStroke" | "paintAt" | "paintSample" => "paintbrush".into(),
        "transformBegin" => "move".into(),
        "incrementViaCommand" | "setLabelViaCommand" => "plus".into(),
        _ => match kind {
            ActionKind::View => "eye".into(),
            ActionKind::Shell => "code".into(),
            ActionKind::Mutation => "sparkles".into(),
            ActionKind::History => "clock".into(),
            ActionKind::Clipboard => "clipboard".into(),
            ActionKind::Interaction => "mouse-pointer".into(),
        },
    }
}

/// @emoji 🎛️ Canonical catalog icon for a footer command id.
pub fn catalog_command_icon_id(id: &str) -> IconName {
    match id {
        id if id.starts_with("os.set") => "settings".into(),
        "os.resetDock" => "panel-left".into(),
        "os.toggleCompact" => "minimize-2".into(),
        "app.export" | "incrementViaCommand" | "setLabelViaCommand" => "download".into(),
        "mode.focus" => "focus".into(),
        "animate.resetGrid" => "grid-3x3".into(),
        _ => "code".into(),
    }
}

//#region 🔖️ActionSemantics
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema, §3.1/D5:
// what an `ActionDefinition`/`CommandDefinition` MEANS to an agent — effects, policy, execution
// shape, and natural-language framing — kept deliberately separate from `CapabilityDefinition` (the
// gateway's compiled projection, per D5, which lives in the gateway crate, not here) and from
// `kernel::Broker`'s own `CapabilityId`/`BrokerCapabilityGrant` (the enforcement primitive
// `CapabilityPolicy.scopes` below references, never redefines).
/// @emoji 🎯️ A templated resource-selector string identifying what a capability reads/writes —
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

/// @emoji 🧮️ What one capability touches — read/write resource selectors plus the three coarse flags
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

/// @emoji 🚦️ When the gateway must pause for human approval before committing an invocation of this
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

/// @emoji 🛡️ The scope/approval gate a capability invocation must clear — `scopes` are
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

/// @emoji 👁️ Whether/how the gateway can show the effect of an invocation before committing it.
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

/// @emoji ↩️ How a committed invocation of this capability can be undone.
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

/// @emoji 🔁️ Whether replaying the same invocation twice is safe, and how the gateway makes it so.
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

/// @emoji ⏱️ How long-running/interactive an invocation of this capability is — the gateway's job
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

/// @emoji 🧵️ Phase-8 migration disposition for every action and command declaration. The
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

/// @emoji ⚙️ Preview/undo/idempotency/cancellation shape of one capability invocation.
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

/// @emoji 🎯️ What an `ActionDefinition`/`CommandDefinition` MEANS to an agent: effects, policy,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub use_when: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<String>,
}

impl ActionSemantics {
    /// @emoji 🏭️ The `📋️master.md` §3.1 defaults table, keyed by `ActionKind`: `Mutation` writes its
    /// own artifact, is reversible, previews a `Diff`, undoes via `Inverse`, expects a revision, and
    /// needs `documents.write` gated `WhenDestructive`; `View`/`Interaction` read the config lane
    /// (`documents.read` + `shell.observe`); `History` needs `documents.write`; `Clipboard` needs
    /// `shell.clipboard`; `🐚️Shell` is not reversible and needs `shell.navigate`.
    pub fn for_kind(kind: ActionKind) -> Self {
        match kind {
            ActionKind::Mutation => Self {
                effects: CapabilityEffects { writes: vec![ResourceSelector::new("artifact:{self}")], reversible: true, ..Default::default() },
                policy: CapabilityPolicy { scopes: vec![kernel::CapabilityId("documents.write".into())], approval: ApprovalMode::WhenDestructive },
                execution: CapabilityExecution { preview: PreviewMode::Diff, undo: UndoMode::Inverse, expected_revision: true, ..Default::default() },
                ..Default::default()
            },
            ActionKind::View | ActionKind::Interaction => Self {
                effects: CapabilityEffects { reads: vec![ResourceSelector::new("config:{self}")], ..Default::default() },
                policy: CapabilityPolicy { scopes: vec![kernel::CapabilityId("documents.read".into()), kernel::CapabilityId("shell.observe".into())], ..Default::default() },
                ..Default::default()
            },
            ActionKind::History => Self { policy: CapabilityPolicy { scopes: vec![kernel::CapabilityId("documents.write".into())], ..Default::default() }, ..Default::default() },
            ActionKind::Clipboard => Self { policy: CapabilityPolicy { scopes: vec![kernel::CapabilityId("shell.clipboard".into())], ..Default::default() }, ..Default::default() },
            ActionKind::Shell => Self { effects: CapabilityEffects { reversible: false, ..Default::default() }, policy: CapabilityPolicy { scopes: vec![kernel::CapabilityId("shell.navigate".into())], ..Default::default() }, ..Default::default() },
        }
    }
}

/// @emoji 🛡️ One release-blocking Phase-8 classification failure, addressed by owner path and
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

/// @emoji ✅️ Rejects an action/command catalog containing an unclassified declaration. Deleted and
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

/// @emoji 📇️ Declares one action an app can receive via `ActionDescriptor.action`.
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

    /// @emoji 🎯️ Declares an action whose icon is resolved from {@link catalog_action_icon_id}.
    pub fn new_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, kind: ActionKind) -> Self {
        let id = id.into();
        Self::new(id.clone(), label, kind, catalog_action_icon_id(&id, kind))
    }

    /// ⚡️ Builds a catalog row without granting UI execution authority. A factory registration or
    /// static bounded-first-step proof must classify the exact key separately.
    pub fn bounded_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, kind: ActionKind) -> Self {
        Self::new_catalog(id, label, kind)
    }

    /// 🧵️ Grants execution classification only to framework routes backed by the explicit
    /// route-specific resumable factories registered by `VcsArtifactApp`.
    pub fn resumable_framework_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, kind: ActionKind) -> Self {
        let mut definition = Self::new_catalog(id, label, kind);
        definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
        definition
    }

    /// @emoji 📝️ Attaches typed argument declarations to this action.
    pub fn with_args(mut self, args: impl IntoIterator<Item = ActionArgDef>) -> Self {
        self.args = args.into_iter().collect();
        self
    }

    /// @emoji 🎨️ Sets palette visibility for this action.
    pub fn with_in_palette(mut self, in_palette: bool) -> Self {
        self.in_palette = in_palette;
        self
    }

    /// @emoji 🎨️ Sets palette visibility for this action.
    pub fn in_palette(self, in_palette: bool) -> Self {
        self.with_in_palette(in_palette)
    }

    /// @emoji 🗂️ Sets this action's ribbon-parent-taxonomy category (a `ui_wgpu::wgpu::RIBBON_PARENT_CATEGORIES`
    /// id) — read back by `AppActionRegistry::category_of` and fed into `organize_context_menu`'s
    /// `category_of` lookup at the context-menu funnel, so an overflowing flat menu buckets this
    /// action's row into `menu.group.<category>` instead of `menu.group.actions`.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// @emoji 🗂️ Sets this action's ribbon-parent-taxonomy category — see `with_category`.
    pub fn category(self, category: impl Into<String>) -> Self {
        self.with_category(category)
    }

    /// @emoji 🎯️ Replaces this action's whole `ActionSemantics` wholesale.
    pub async fn semantics(mut self, semantics: ActionSemantics) -> Self {
        self.semantics = semantics;
        self
    }

    /// @emoji ⚠️ Marks this action destructive: sets `effects.destructive` and raises `policy.approval`
    /// to `WhenDestructive` (a no-op if it was already `Always`).
    pub async fn destructive(mut self) -> Self {
        self.semantics.effects.destructive = true;
        if self.semantics.policy.approval == ApprovalMode::Never {
            self.semantics.policy.approval = ApprovalMode::WhenDestructive;
        }
        self
    }

    /// @emoji 🗣️ Sets the natural-language phrases a capability search should match this action
    /// against (`ActionSemantics.use_when`).
    pub fn use_when(mut self, phrases: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.semantics.use_when = phrases.into_iter().map(Into::into).collect();
        self
    }

    /// @emoji 📖️ Appends one natural-language usage example (`ActionSemantics.examples`).
    pub async fn example(mut self, example: impl Into<String>) -> Self {
        self.semantics.examples.push(example.into());
        self
    }
}

/// @emoji ⏪️ The framework-owned action id apps dispatch to revert to a past command-log entry —
/// auto-injected as the 7th `history_action_definitions()` entry (never in the palette; needs a
/// concrete `entrySeq` from the history panel's "backwards" button).
pub const REVERT_TO_COMMAND_ACTION_ID: &str = "revertToCommand";

/// @emoji 🕹️ The seven framework-owned History actions, auto-injected into every `AppDefinition`.
pub fn history_action_definitions() -> Vec<ActionDefinition> {
    vec![
        ActionDefinition { keys: Some("mod+z".into()), ..ActionDefinition::resumable_framework_catalog("undo", LocalizedLabel::native("Undo", "Rückgängig"), ActionKind::History) },
        ActionDefinition { keys: Some("mod+shift+z".into()), ..ActionDefinition::resumable_framework_catalog("redo", LocalizedLabel::native("Redo", "Wiederholen"), ActionKind::History) },
        ActionDefinition::resumable_framework_catalog("commitCheckpoint", LocalizedLabel::native("Commit Checkpoint", "Checkpoint festschreiben"), ActionKind::History),
        ActionDefinition::resumable_framework_catalog("createAlternative", LocalizedLabel::native("Create Alternative", "Alternative erstellen"), ActionKind::History),
        ActionDefinition::resumable_framework_catalog("switchAlternative", LocalizedLabel::native("Switch Alternative", "Alternative wechseln"), ActionKind::History),
        ActionDefinition::resumable_framework_catalog("checkoutCheckpoint", LocalizedLabel::native("Checkout Checkpoint", "Checkpoint auschecken"), ActionKind::History),
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework_catalog(REVERT_TO_COMMAND_ACTION_ID, LocalizedLabel::native("Revert to Command", "Auf Befehl zurücksetzen"), ActionKind::History) }
            .with_args([ActionArgDef::number("entrySeq", LocalizedLabel::native("Entry", "Eintrag")).required()]),
    ]
}

/// @emoji 🎚️ The framework-owned action id apps dispatch to change the history panel's operations
/// filter — auto-injected unconditionally (mirrors `RECORD_TUTORIAL_ACTION_ID`).
pub const SET_HISTORY_COMMAND_FILTER_ACTION_ID: &str = "setHistoryCommandFilter";

/// @emoji 🎚️ The framework-injected `setHistoryCommandFilter` View action (never in the palette):
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
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework_catalog(SET_HISTORY_COMMAND_FILTER_ACTION_ID, LocalizedLabel::native("Set History Filter", "Verlaufsfilter festlegen"), ActionKind::View) }.with_args([ActionArgDef::select(
        "value",
        LocalizedLabel::native("Filter", "Filter"),
        options,
    )
    .default_value(&"all")])
}

/// @emoji 🗒️ The framework-owned action id apps dispatch to note a shell effect (navigate, export,
/// spawn, …) into the session command log without any document mutation — mirrors
/// `SET_HISTORY_COMMAND_FILTER_ACTION_ID`'s auto-injected-constant pattern.
pub const NOTE_SHELL_COMMAND_ACTION_ID: &str = "noteShellCommand";

/// @emoji 🗒️ The framework-injected `noteShellCommand` Shell action (never in the palette): records a
/// shell-kind effect that already happened into the session command log, for effects dispatched
/// outside the normal `ActionDescriptor` path. `commandId` and `label` are required; `detail` is an
/// optional free-text elaboration shown in the history panel.
pub fn note_shell_command_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework_catalog(NOTE_SHELL_COMMAND_ACTION_ID, LocalizedLabel::native("Note Shell Command", "Shell-Befehl vermerken"), ActionKind::Shell) }.with_args([
        ActionArgDef::text("commandId", LocalizedLabel::native("Command", "Befehl")).required(),
        ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")).required(),
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
        ActionDefinition { keys: Some("mod+c".into()), ..ActionDefinition::resumable_framework_catalog("copy", LocalizedLabel::native("Copy", "Kopieren"), ActionKind::Clipboard) },
        ActionDefinition { keys: Some("mod+x".into()), ..ActionDefinition::resumable_framework_catalog("cut", LocalizedLabel::native("Cut", "Ausschneiden"), ActionKind::Clipboard) },
        ActionDefinition { keys: Some("mod+v".into()), ..ActionDefinition::resumable_framework_catalog("paste", LocalizedLabel::native("Paste", "Einfügen"), ActionKind::Clipboard) }.with_args([
            ActionArgDef::select("anchor", LocalizedLabel::native("Anchoring", "Verankerung"), anchoring_options).default_value(&"original"),
            ActionArgDef::vec3("position", LocalizedLabel::native("Position", "Position")),
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
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework_catalog(INTERACTION_SELECT_ACTION_ID, LocalizedLabel::native("Select", "Auswählen"), ActionKind::Interaction) }.with_args([
            ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(),
            ActionArgDef::text("targets", LocalizedLabel::native("Targets", "Ziele")).required(),
            ActionArgDef::select("merge", LocalizedLabel::native("Merge", "Zusammenführen"), merge_options).required(),
            ActionArgDef::select("method", LocalizedLabel::native("Method", "Methode"), method_options).required(),
        ]),
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework_catalog(INTERACTION_HOVER_ACTION_ID, LocalizedLabel::native("Hover", "Hover"), ActionKind::Interaction) }.with_args([
            ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(),
            ActionArgDef::text("channel", LocalizedLabel::native("Channel", "Kanal")).required(),
            ActionArgDef::text("targets", LocalizedLabel::native("Targets", "Ziele")).required(),
        ]),
        ActionDefinition { keys: Some("escape".into()), ..ActionDefinition::resumable_framework_catalog(CLEAR_SELECTION_ACTION_ID, LocalizedLabel::native("Clear Selection", "Auswahl aufheben"), ActionKind::Interaction) },
        ActionDefinition { keys: Some("mod+a".into()), ..ActionDefinition::resumable_framework_catalog(SELECT_ALL_ACTION_ID, LocalizedLabel::native("Select All", "Alles auswählen"), ActionKind::Interaction) },
        ActionDefinition::resumable_framework_catalog(SET_SELECTION_MODE_ACTION_ID, LocalizedLabel::native("Set Selection Mode", "Auswahlmodus festlegen"), ActionKind::Interaction)
            .with_args([ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(), ActionArgDef::select("mode", LocalizedLabel::native("Mode", "Modus"), mode_options).required()]),
        ActionDefinition::resumable_framework_catalog(SET_INTERACTION_GRANULARITY_ACTION_ID, LocalizedLabel::native("Set Granularity", "Granularität festlegen"), ActionKind::Interaction)
            .with_args([ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(), ActionArgDef::text("granularityId", LocalizedLabel::native("Granularity", "Granularität")).required()]),
    ]
}
//#endregion 🔖️Interaction

/// @emoji 🧰️ The framework-owned action id apps dispatch to activate a utility — auto-injected as a View
/// action into any `AppDefinition` that declares utilities (mirrors `history_action_definitions`).
pub const SET_ACTIVE_UTILITY_ACTION_ID: &str = "setActiveUtility";

/// @emoji 🧰️ The framework-injected `setActiveUtility` View action (never in the palette): switches the
/// host-owned active utility of a window kind. `utilityId` is required; `windowKindId` is contextual (the
/// shell fills it from the focused window when absent).
pub fn set_active_utility_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework_catalog(SET_ACTIVE_UTILITY_ACTION_ID, LocalizedLabel::native("Set Active Utility", "Aktives Hilfsmittel festlegen"), ActionKind::View) }
        .with_args([ActionArgDef::text("utilityId", LocalizedLabel::native("Utility", "Hilfsmittel")).required(), ActionArgDef::text("windowKindId", LocalizedLabel::native("Window", "Fenster"))])
}

/// @emoji 🛠️ The framework-owned action id apps dispatch to activate a mode-level tool — auto-injected
/// as a View action into any `AppDefinition` that declares tools (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).
pub const SET_ACTIVE_TOOL_ACTION_ID: &str = "setActiveTool";

/// @emoji 🛠️ The framework-injected `setActiveTool` View action (never in the palette): switches the
/// host-owned active tool of the active mode. Unlike `setActiveUtility` this takes no `windowKindId` —
/// tools are windowless, scoped to the whole mode.
pub fn set_active_tool_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework_catalog(SET_ACTIVE_TOOL_ACTION_ID, LocalizedLabel::native("Set Active Tool", "Aktives Werkzeug festlegen"), ActionKind::View) }.with_args([ActionArgDef::text(
        "toolId",
        LocalizedLabel::native("Tool", "Werkzeug"),
    )
    .required()])
}

/// @emoji 🎓️ The framework-owned action id apps dispatch to (re)start an app's introduction —
/// auto-injected as a shell-intercepted View action into any
/// `AppDefinition` that declares one (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).
pub const START_INTRODUCTION_ACTION_ID: &str = "startIntroduction";

/// @emoji 🎓️ The framework-injected `startIntroduction` View action: fully shell-intercepted (never
/// forwarded to the program), it resets playback to the first step of `AppDefinition.introduction`.
/// Unlike ordinary app actions this stays out of the action palette because the shell exposes the
/// dedicated `Introduce App` command.
pub fn start_introduction_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework_catalog(START_INTRODUCTION_ACTION_ID, LocalizedLabel::native("Introduce App", "App vorstellen"), ActionKind::View) }
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

/// @emoji 📍️ Fully qualified address of an action owned by one concrete window instance.
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

/// @emoji 📨️ One addressed action invocation with named DSL-value arguments. Ticket
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
/// @emoji 🧰️ Declares one interactive utility (a live-preview pointer mode) an app exposes. Distinct from
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
}

impl UtilityDefinition {
    /// @emoji 🧰️ A utility with sensible defaults (no group/keys/cursor/category, gates actions while active).
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), group: None, keys: None, cursor: None, category: None, allows_actions_while_active: false }
    }
}

/// @emoji 🧰️ A validated reference into an app's `AppDefinition.utilities` registry — the utility mirror of
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
/// @emoji 🎛️ Declares one command: a categorized verb offered in the footer command panel.
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

    /// @emoji 🎛️ Declares a command whose icon is resolved from {@link catalog_command_icon_id}.
    pub fn new_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, kind: ActionKind) -> Self {
        let id = id.into();
        Self::new(id.clone(), label, category, catalog_command_icon_id(&id), kind)
    }

    /// ⚡️ Builds a catalog row without granting UI execution authority. A factory registration or
    /// static bounded-first-step proof must classify the exact key separately.
    pub fn bounded_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, kind: ActionKind) -> Self {
        Self::new_catalog(id, label, category, kind)
    }

    /// @emoji 📝️ Attaches typed argument declarations to this command.
    pub fn with_args(mut self, args: impl IntoIterator<Item = ActionArgDef>) -> Self {
        self.args = args.into_iter().collect();
        self
    }

    /// @emoji ⌨️ Attaches one platform-aware command keybinding.
    pub fn with_keybinding(mut self, keybinding: PlatformKeybinding) -> Self {
        self.keybindings.push(keybinding);
        self
    }

    /// @emoji 🎯️ Replaces this command's whole `ActionSemantics` wholesale.
    pub async fn semantics(mut self, semantics: ActionSemantics) -> Self {
        self.semantics = semantics;
        self
    }

    /// @emoji ⚠️ Marks this command destructive — see `ActionDefinition::destructive`.
    pub async fn destructive(mut self) -> Self {
        self.semantics.effects.destructive = true;
        if self.semantics.policy.approval == ApprovalMode::Never {
            self.semantics.policy.approval = ApprovalMode::WhenDestructive;
        }
        self
    }

    /// @emoji 🗣️ Sets `ActionSemantics.use_when` — see `ActionDefinition::use_when`.
    pub async fn use_when(mut self, phrases: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.semantics.use_when = phrases.into_iter().map(Into::into).collect();
        self
    }

    /// @emoji 📖️ Appends one `ActionSemantics.examples` entry — see `ActionDefinition::example`.
    pub async fn example(mut self, example: impl Into<String>) -> Self {
        self.semantics.examples.push(example.into());
        self
    }
}

/// @emoji 📍️ Hierarchical owner of a command definition.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CommandOwnerAddress {
    Os,
    Plugin { plugin_id: String },
    App { plugin_id: String, app_id: String },
    Mode { plugin_id: String, app_id: String, mode_id: String },
}

/// @emoji 📍️ Fully qualified address of one command.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CommandAddress {
    pub owner: CommandOwnerAddress,
    pub command_id: String,
}

/// @emoji 📨️ One addressed command invocation with named DSL-value arguments — see
/// `ActionInvocation`'s docstring for why `arguments` is keyed to `DslValue`, not
/// `serde_json::Value`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CommandInvocation {
    pub address: CommandAddress,
    pub arguments: BTreeMap<String, DslValue>,
}

/// @emoji 💻️ Operating-system command catalog shared by every renderer.
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
/// @emoji 🛠️ Declares one mode-level tool: an activatable, stateful capability of a whole app mode.
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
}

impl ToolDefinition {
    /// @emoji 🛠️ A tool with sensible defaults (no keybinding).
    pub async fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), keys: None }
    }
}

/// @emoji 🛠️ A validated reference into an app's `AppDefinition.tools` registry — the tool mirror of
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
/// @emoji 🆔️ Whether `id` matches the renderer-agnostic UI element id grammar: dot-separated segments,
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

/// @emoji 🆔️ Normalizes arbitrary input (a domain object's own id, a free-text label, an already
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

/// @emoji 🆔️ Derives a child element id by suffixing `parent` with one or more segments, each normalized
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

/// @emoji 🆔️ Element id of the app shell's navbar — singular, shell-owned chrome.
pub const UI_NAVBAR_ELEMENT_ID: &str = "ui.navbar";
/// @emoji 🆔️ Element id of the app shell's footer — singular, shell-owned chrome.
pub const UI_FOOTER_ELEMENT_ID: &str = "ui.footer";

/// @emoji 🆔️ Element id of a window kind's body — `framework.window.{camelCased kind id}`.
pub fn window_element_id(kind_id: &str) -> String {
    child_element_id("framework.window", &[kind_id])
}

/// @emoji 🆔️ Element id of a panel tab's uncollapsed panel body. `tab_id` is already a dotted
/// `PanelTabDefinition.id()` (e.g. `puzzle.catalogue`) — appended verbatim rather than through
/// `child_element_id`, which would collapse its dots into camelCase.
pub fn panel_tab_element_id(tab_id: &str) -> String {
    format!("framework.panelTab.{tab_id}")
}

/// @emoji 🆔️ Alias id of the first draggable tree row inside a panel tab (document order within that
/// uncollapsed panel) — stamped via `data-element-alias` since no single tree row has a stable semantic
/// id at authoring time. Used to teach catalogue drag-and-drop without hardcoding a kind id.
pub fn panel_tab_first_draggable_element_id(tab_id: &str) -> String {
    format!("framework.panelTab.{tab_id}.firstDraggable")
}
//#endregion 🆔️ElementId

//#region 🔖️Introduction
/// @emoji 🎓️ A first-run walkthrough an app declares to introduce its UI, utilities, and actions to a
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

/// @emoji 🪜️ One step of an `IntroductionDefinition`: an info box pointing at `introduce`, with `show`
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

    /// @emoji 🎯️ Sets the single element id raised above the glass and anchoring the info box.
    pub fn introduce(mut self, element_id: impl Into<String>) -> Self {
        self.introduce = Some(element_id.into());
        self
    }

    /// @emoji 🕳️ Additional element ids raised above the glass alongside `introduce` (no pulse).
    pub fn show(mut self, element_ids: Vec<String>) -> Self {
        self.show = element_ids;
        self
    }

    /// @emoji 📍️ Overrides where the info box is placed relative to `introduce`.
    pub fn placement(mut self, placement: IntroductionPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// @emoji ✅️ Makes the step complete when the user performs all `interactions` (any order) instead of
    /// pressing Next.
    pub fn interact(mut self, interactions: Vec<IntroductionInteraction>) -> Self {
        self.interactions = interactions;
        self
    }

    /// @emoji 🔢️ Like `interact`, but `interactions` must complete in declaration order.
    pub fn interact_ordered(mut self, interactions: Vec<IntroductionInteraction>) -> Self {
        self.interactions = interactions;
        self.ordered = true;
        self
    }

    /// @emoji 🏛️ Attaches institution/partner logos to the step's info box.
    pub fn logos(mut self, logos: Vec<IntroductionLogo>) -> Self {
        self.logos = logos;
        self
    }

    /// @emoji 🎬️ Attaches ghost-cursor demonstrations played in order, then looping back to the first.
    pub fn demonstrate(mut self, demonstrations: Vec<IntroductionDemonstration>) -> Self {
        self.demonstrations = demonstrations;
        self
    }
}

/// @emoji 🏛️ One institution/partner logo shown in an `IntroductionStepDefinition`'s info box — a plain
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

/// @emoji 📍️ Where the info box is placed relative to its anchor.
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

/// @emoji 👉️ What one `IntroductionInteraction` requires: `Action`/`Utility`/`Tool`/`Panel`/`Expand`
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

/// @emoji ✅️ One thing the user must do to complete an interaction-gated `IntroductionStepDefinition` —
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

    /// @emoji 📇️ An interaction completing when the user activates action `id`.
    pub async fn action(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Action(ActionRef::new(id.into())), label).await
    }

    /// @emoji 🧰️ An interaction completing when the user activates utility `id`.
    pub async fn utility(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Utility(UtilityRef::new(id.into())), label).await
    }

    /// @emoji 🛠️ An interaction completing when the user activates tool `id`.
    pub async fn tool(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Tool(ToolRef::new(id.into()).await), label).await
    }

    /// @emoji 📑️ An interaction completing when panel tab `id` opens.
    pub async fn panel(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Panel(id.into()), label).await
    }

    /// @emoji 🌲️ An interaction completing when tree section/item `id` expands.
    pub async fn expand(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Expand(id.into()), label).await
    }

    /// @emoji 🖐️ An interaction completing when the user pans 3D window `window_kind_id`.
    pub async fn pan(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Pan(window_kind_id.into()), label).await
    }

    /// @emoji 🔍️ An interaction completing when the user zooms 3D window `window_kind_id`.
    pub async fn zoom(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Zoom(window_kind_id.into()), label).await
    }

    /// @emoji 🌐️ An interaction completing when the user orbits 3D window `window_kind_id`.
    pub async fn orbit(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Orbit(window_kind_id.into()), label).await
    }

    /// @emoji 🎉️ Overrides which element id is stamped `data-celebrated` on completion.
    pub async fn celebrate(mut self, element_id: impl Into<String>) -> Self {
        self.celebrate = Some(element_id.into());
        self
    }
}

/// @emoji 📌️ Where a demonstration gesture points, resolvable to a viewport pixel at play time. One
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
    /// @emoji 🗺️ 2D world-space coordinates on the infinite-canvas surface shown by window `window_id`.
    pub async fn canvas(window_id: impl Into<String>, x: f64, y: f64) -> Self {
        Self::Canvas { id: window_id.into(), x, y }
    }

    /// @emoji 🏷️ A specific entity by domain + id, centered (no `offset`).
    pub async fn entity(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>) -> Self {
        Self::Entity { id: window_id.into(), domain: domain.into(), entity: entity.into(), offset: None }
    }

    /// @emoji 🏷️ Any entity in `domain` — the surface picks a representative, nearest the viewport center.
    pub async fn any_entity(window_id: impl Into<String>, domain: impl Into<String>) -> Self {
        Self::entity(window_id, domain, "*").await
    }

    /// @emoji 🪡️ A parametric point at `t` (0–1 by arc length) along an entity's curve geometry.
    pub async fn curve(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>, t: f64) -> Self {
        Self::Curve { id: window_id.into(), domain: domain.into(), entity: entity.into(), t }
    }

    /// @emoji 🎚️ A value mapped through an entity's live value domain (e.g. a slider's min..max).
    pub async fn domain_value(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>, value: f64) -> Self {
        Self::Domain { id: window_id.into(), domain: domain.into(), entity: entity.into(), value }
    }
}

/// @emoji 🖱️ Which mouse button a drag-like demonstration presses.
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

/// @emoji ⌨️ Keyboard modifier held during a drag-like demonstration.
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

/// @emoji 👆️ A gesture a demonstration plays: the ghost cursor travels to (or between) `IntroductionPoint`s
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

/// @emoji 🖱️ Ghost-cursor glyph, mirroring `🎨️ui.css`'s `--cursor-*` custom cursors.
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

/// @emoji 🎬️ A looping ghost-cursor demonstration attached to an interaction-gated
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
    /// @emoji 👆️ A left-click demonstration at `at`.
    pub async fn left_click(at: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::LeftClick { at }, cursor: None }
    }

    /// @emoji 👆️ A right-click demonstration at `at`.
    pub async fn right_click(at: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::RightClick { at }, cursor: None }
    }

    /// @emoji ✋️ A click-and-drag demonstration from `from` to `to`.
    pub async fn drag(from: IntroductionPoint, to: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::Drag { from, to, button: IntroductionPointerButton::Left, modifiers: vec![] }, cursor: None }
    }

    /// @emoji 🖲️ A scroll-wheel demonstration at `at`; `delta_y` sign conveys direction.
    pub async fn scroll(at: IntroductionPoint, delta_y: f64) -> Self {
        Self { gesture: IntroductionGesture::Scroll { at, delta_y }, cursor: None }
    }

    /// @emoji 🌐️ A camera-orbit demonstration curving from `from` to `to`.
    pub async fn orbit(from: IntroductionPoint, to: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::Orbit { from, to, button: IntroductionPointerButton::Right, modifiers: vec![IntroductionKeyModifier::Alt] }, cursor: None }
    }
}
//#endregion 🔖️Introduction

//#region 🔖️Tutorial
/// @emoji 🎬️ A recorded, timed, replayable walkthrough — the timeline sibling of the step-gated
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
    /// @emoji 📂️ Deserializes a `TutorialDefinition` from its JSON wire format — the constructor apps use
    /// to load a hand-authored or recorded tutorial (e.g. via `include_str!`) into `.tutorial(...)`.
    // 🚧️ BLOCKED: `TutorialDefinition` itself is serde-only (see its docstring) — this stays on
    // `serde_json` until that lands.
    pub async fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// @emoji 📖️ One scrub-bar marker in a `TutorialDefinition`'s timeline.
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

/// @emoji 🎬️ What must be true at t=0: the document the tutorial sandboxes and the initial UI/camera
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

/// @emoji 🎞️ The seven parallel tracks of a `TutorialDefinition`'s timeline; every entry's `at` is a
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
    /// 🖋️ The sole source of document mutation during playback — see `TutorialArtifactEventKind`.
    #[serde(default)]
    #[value(default)]
    pub document: Vec<TutorialArtifactEvent>,
    #[serde(default)]
    #[value(default)]
    pub camera: Vec<TutorialCameraKeyframe>,
    #[serde(default)]
    #[value(default)]
    pub gestures: Vec<TutorialGestureCue>,
}

/// @emoji 📦️ Where a tutorial media asset's bytes live. `Blob` is wire-identical to `store::BlobRef`
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

/// @emoji 🎙️ One voiceover cue: `text` is both the TTS script and the caption fallback; `audio`
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

/// @emoji 💬️ One timed caption sub-segment of a `TutorialNarrationCue`.
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

/// @emoji 🖼️ Normalized 0–1 viewport rect for a `TutorialVideoCue` overlay.
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

/// @emoji 📹️ A timed video overlay — e.g. a presenter webcam picture-in-picture, or an authored clip.
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

/// @emoji 🏷️ One recorded action/command/keypress, annotational only — see `TutorialTracks::events`.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialEvent {
    pub at: u64,
    pub kind: TutorialEventKind,
}

/// @emoji 🏷️ What one `TutorialEvent` annotates.
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

/// @emoji 🧮️ One UI-state track entry: either a full restore-point snapshot (a valid seek anchor) or a
/// sparse list of changes since the previous sample.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialUiKeyframe {
    pub at: u64,
    pub sample: TutorialUiSample,
}

/// @emoji 🧮️ See `TutorialUiKeyframe`.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialUiSample {
    Snapshot { state: Box<TutorialUiSnapshot> },
    Delta { changes: Vec<TutorialUiChange> },
}

/// @emoji 🧮️ Renderer-neutral restore point for chrome/UI state — a superset of `ViewModel` plus the
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
    /// 📑️ Active tab id per panel group; groups absent from the map are collapsed/closed.
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

/// @emoji 🩹️ One typed, sparse UI-state change — the alphabet `compose_tutorial_ui` replays over a prior
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
    /// 📑️ `tab_id: None` collapses/closes the group.
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

/// @emoji 🖋️ One document-track entry — mirrors `store::ArtifactCommand` with `Mutation =
/// serde_json::Value` (opaque per-app mutation JSON, already the wire shape of every `KernelMutation`
/// diff). This is the SOLE source of document mutation during playback: recorded `TutorialEvent`s are
/// annotational only, never re-dispatched, because re-dispatching a plugin action is non-deterministic
/// (fresh ids/timestamps) and would double-apply against this track.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialArtifactEvent {
    pub at: u64,
    pub kind: TutorialArtifactEventKind,
}

/// @emoji 🖋️ See `TutorialArtifactEvent`. `Edit` carries both `forwards` and `backwards` operations
/// verbatim from the vcs edit that produced it — the source of exact bidirectional scrubbing.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialArtifactEventKind {
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

/// @emoji 🎥️ One camera track keyframe for a specific window instance.
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

/// @emoji 🎥️ A camera pose — `Orbit` mirrors `World3dScene.camera_json`/`OrbitController`, `Canvas`
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

/// @emoji 🪄️ Interpolation curve into a `TutorialCameraKeyframe` from its predecessor on the same window.
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

/// @emoji 👻️ One ghost-cursor gesture cue, reusing the introduction demonstration vocabulary verbatim —
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

/// @emoji 🎬️ The framework-owned action id apps dispatch to (re)start a tutorial — auto-injected as a
/// fully shell-intercepted View action into any `AppDefinition` that declares one (mirrors
/// `START_INTRODUCTION_ACTION_ID`). Distinct from an introduction: a tutorial takes a required
/// `tutorialId` argument since an app may declare more than one.
pub const START_TUTORIAL_ACTION_ID: &str = "startTutorial";

/// @emoji 🎬️ The framework-injected `startTutorial` View action: fully shell-intercepted, it sandboxes
/// the live document, loads the selected tutorial's `base`, and starts playback from t=0.
pub fn start_tutorial_action_definition(tutorials: &[TutorialDefinition]) -> ActionDefinition {
    let mut options = Vec::with_capacity(tutorials.len());
    for t in tutorials {
        options.push(ActionArgOption::new(t.id.clone(), t.title.clone()));
    }
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework_catalog(START_TUTORIAL_ACTION_ID, LocalizedLabel::native("Play Tutorial", "Tutorial abspielen"), ActionKind::View) }.with_args([ActionArgDef::select(
        "tutorialId",
        LocalizedLabel::native("Tutorial", "Tutorial"),
        options,
    )
    .required()])
}

/// @emoji ⏺️ The framework-owned action id that opens the tutorial recorder chrome — auto-injected into
/// EVERY `AppDefinition` (recording needs no app-side declaration at all).
pub const RECORD_TUTORIAL_ACTION_ID: &str = "recordTutorial";

/// @emoji ⏺️ The framework-injected `recordTutorial` View action: fully shell-intercepted, arms the
/// recorder against the live document (never a sandboxed copy — a recording IS the user's work).
pub fn record_tutorial_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework_catalog(RECORD_TUTORIAL_ACTION_ID, LocalizedLabel::native("Record Tutorial", "Tutorial aufzeichnen"), ActionKind::View) }
}

/// ⏱️ Real-time (not timeline-time, not rate-scaled) duration of the camera glide the player performs
/// when the user presses Play after deviating from an active tutorial's recorded state.
pub const TUTORIAL_CONVERGE_MS: u64 = 600;

//#region 🔖️TutorialEngine
/// @emoji ✅️ Structural validation shared by the plugin builder and both recorders before save: every
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

/// @emoji 🎥️ Interpolates between two camera keyframes at timeline offset `at_ms` (clamped into
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

/// @emoji 🎥️ Finds the camera pose for `window_id` at `at_ms`: exact if `at_ms` lands on or before the
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

/// @emoji 🩹️ Applies one `TutorialUiChange` onto a `TutorialUiSnapshot` in place — the pure core both
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
            state.interaction_selection.insert(domain_id.clone(), DomainSelection { granularity: granularity.clone(), ids: ids.clone(), anchor_id: None });
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

/// @emoji 🧮️ Reconstructs the full `TutorialUiSnapshot` at `at_ms`: starts from `base.ui`, then the
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

/// @emoji ✂️ Everything a live director's tick from `from_ms` to `to_ms` must apply: annotational
/// events, document edits, and UI deltas within the half-open interval on the crossing direction (empty
/// when `from_ms == to_ms`). Backward direction (scrubbing left) reverses entry order so callers apply
/// each `TutorialArtifactEventKind::Edit`'s `backwards` ops from most-recent to least-recent. Plain Rust
/// struct (not owned schema mirrored) — the TS port lives in `framework/renderer/react/index.tsx` and is pinned
/// to this one via shared golden fixtures, not a wasm call per frame.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TutorialSlice {
    pub forward: bool,
    pub events: Vec<TutorialEvent>,
    pub document: Vec<TutorialArtifactEvent>,
    pub ui_changes: Vec<TutorialUiChange>,
}

/// @emoji ✂️ Computes the `TutorialSlice` for advancing the playhead from `from_ms` to `to_ms` (`to_ms`
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
    let mut document: Vec<TutorialArtifactEvent> = def.tracks.document.iter().filter(|e| in_range(e.at)).cloned().collect();
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
/// @emoji 🗨️ A declared modal form dialog: a glass veil covers the screen and an info box (styled
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
        }
    }

    /// @emoji 📝️ Attaches explanatory body text shown below the title.
    pub fn body(mut self, body: impl Into<LocalizedLabel>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// @emoji 🧾️ Attaches the staged-form field declarations.
    pub fn args(mut self, args: Vec<ActionArgDef>) -> Self {
        self.args = args;
        self
    }

    /// @emoji ✅️ Overrides the submit button label (default "OK").
    pub fn submit_label(mut self, label: impl Into<LocalizedLabel>) -> Self {
        self.submit_label = label.into();
        self
    }

    /// @emoji ❌️ Overrides the cancel button label (default "Cancel", applied by the renderer).
    pub fn cancel_label(mut self, label: impl Into<LocalizedLabel>) -> Self {
        self.cancel_label = Some(label.into());
        self
    }

    /// @emoji 🚪️ Declares an action dispatched on any dismissal (Escape, veil click, Cancel button).
    pub fn on_cancel(mut self, action: ActionRef) -> Self {
        self.cancel_action = Some(action);
        self
    }
}
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
/// @emoji 🧮️ Computes the effective argument map for an action: for each declared arg, the staged value
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

/// @emoji ❗️ Returns the ids of required args that are still unset in `effective`. "Unset" means absent,
/// `Null`, or an empty string (covers a blank Text/Select/IconSelect/ArtifactKind/SurfaceApp — the
/// latter two resolve to a `String` effective value exactly like `Select`, contract §C8.1); `false`,
/// `0`, and `[]` are valid values for Toggle/Number/Slider/Vec3 and never count as unset.
pub fn missing_required_args(defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
    defs.iter()
        .filter(|def| def.required)
        .filter(|def| match effective.get(&def.id) {
            None | Some(DslValue::Null) => true,
            Some(DslValue::String(text)) => text.is_empty(),
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

/// @emoji 🚦️ Whether an action is eligible to appear in a window's Actions panel — excludes the six
/// framework History actions (rendered by the History rail) and the injected `setActiveUtility`/
/// `setActiveTool` (internal View actions wired to the utility bar/tool panel, never the panel).
// 🚫️async: E1 transitive — the only consumer is `Iterator::filter` (external trait), which takes a
// sync closure; pure comparison, no I/O (R9).
fn action_is_panel_eligible(action: &ActionDefinition) -> bool {
    action.kind != ActionKind::History && action.id != SET_ACTIVE_UTILITY_ACTION_ID && action.id != SET_ACTIVE_TOOL_ACTION_ID
}

/// @emoji 📇️ Resolves the actions a window kind presents in its panel from its authoritative
/// owned definitions, preserving declaration order and excluding framework-only rail actions.
pub fn resolve_window_actions<'a>(_app: &'a AppDefinition, window_kind: &'a WindowKindDefinition) -> Vec<&'a ActionDefinition> {
    window_kind.actions.iter().filter(|action| action_is_panel_eligible(action)).collect()
}

/// @emoji 🛠️ Resolves the tools the active mode presents, in declared order — references into
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
    pub app_id: String,
}

/// 🧩️ One host-aggregated plugin contribution entry (`contributionsJson` wire shape).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ProgramContributionEntry {
    pub plugin_id: String,
    #[value(default)]
    pub topic_contribution: Option<TopicContribution>,
}

/// 📕️ Parses host-pushed `contributionsJson` into typed entries.
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

/// 🚧️ Failure parsing a `Version` (`major.minor.patch`, all-numeric segments) or a `VersionReq`.
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

/// 🔢️ A dependency version requirement — the frozen grammar `=X.Y.Z` / `^X.Y.Z` / `~X.Y.Z` /
/// `>=X.Y.Z` / `*` (contract freeze §3). `^`/`~` follow standard semver caret/tilde precedence:
/// caret allows any change that does not bump the leftmost nonzero component, tilde allows only
/// patch-level movement within the same `major.minor`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VersionReq {
    Any,
    Exact(Version),
    Caret(Version),
    Tilde(Version),
    AtLeast(Version),
}

impl VersionReq {
    /// 🔢️ Parses one of the five frozen grammar forms.
    // 🚫️async: E1 transitive — `FromValue::from_value` (this file, below) calls this synchronously;
    // pure string parsing, no I/O (R9).
    pub fn parse(input: &str) -> Result<Self, VersionReqParseError> {
        let trimmed = input.trim();
        if trimmed == "*" {
            return Ok(VersionReq::Any);
        }
        if let Some(rest) = trimmed.strip_prefix(">=") {
            return Ok(VersionReq::AtLeast(Version::parse(rest)?));
        }
        if let Some(rest) = trimmed.strip_prefix('^') {
            return Ok(VersionReq::Caret(Version::parse(rest)?));
        }
        if let Some(rest) = trimmed.strip_prefix('~') {
            return Ok(VersionReq::Tilde(Version::parse(rest)?));
        }
        if let Some(rest) = trimmed.strip_prefix('=') {
            return Ok(VersionReq::Exact(Version::parse(rest)?));
        }
        Err(VersionReqParseError::UnknownOperator(trimmed.to_string()))
    }

    /// ✅️ Whether `version` satisfies this requirement.
    // 🚫️async: E1 transitive — pure comparison consumed by `matches_raw`, itself required sync (R9).
    pub fn matches(&self, version: &Version) -> bool {
        match self {
            VersionReq::Any => true,
            VersionReq::Exact(required) => version == required,
            VersionReq::AtLeast(required) => version >= required,
            VersionReq::Caret(required) => {
                if required.major != 0 {
                    version.major == required.major && version >= required
                } else if required.minor != 0 {
                    version.major == 0 && version.minor == required.minor && version >= required
                } else {
                    version.major == 0 && version.minor == 0 && version.patch == required.patch
                }
            }
            VersionReq::Tilde(required) => version.major == required.major && version.minor == required.minor && version.patch >= required.patch,
        }
    }

    /// ✅️ Convenience for the dependency graph: parses `raw` and matches, treating an unparsable
    /// target version as non-matching (except `*`, which never needs to parse its target).
    // 🚫️async: E1 transitive — dependency-graph validation calls this synchronously via `!`; pure
    // parse-and-compare, no I/O (R9).
    pub fn matches_raw(&self, raw: &str) -> bool {
        match self {
            VersionReq::Any => true,
            _ => Version::parse(raw).is_ok_and(|version| self.matches(&version)),
        }
    }
}

impl std::fmt::Display for VersionReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionReq::Any => write!(f, "*"),
            VersionReq::Exact(v) => write!(f, "={v}"),
            VersionReq::Caret(v) => write!(f, "^{v}"),
            VersionReq::Tilde(v) => write!(f, "~{v}"),
            VersionReq::AtLeast(v) => write!(f, ">={v}"),
        }
    }
}

/// 🚧️ Failure parsing a `VersionReq` string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionReqParseError {
    UnknownOperator(String),
    Version(VersionParseError),
}

impl std::fmt::Display for VersionReqParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownOperator(input) => write!(formatter, "unknown version requirement operator in {input:?} (expected one of `=`,`^`,`~`,`>=`,`*`)"),
            Self::Version(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for VersionReqParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Version(error) => Some(error),
            Self::UnknownOperator(_) => None,
        }
    }
}

impl From<VersionParseError> for VersionReqParseError {
    fn from(error: VersionParseError) -> Self {
        Self::Version(error)
    }
}

impl ToValue for VersionReq {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.to_string())
    }
}
impl FromValue for VersionReq {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => VersionReq::parse(&s).map_err(|e| ValueError::new(e.to_string())),
            other => Err(ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only
// manifest type above/below (`PluginDependency.version`) — hand-written, same reasoning as the
// `ToValue`/`FromValue` pair above.
impl Serialize for VersionReq {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for VersionReq {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        VersionReq::parse(&raw).map_err(serde::de::Error::custom)
    }
}

/// 🔗️ One direct plugin dependency: the depended-on plugin id plus the version requirement it must
/// satisfy — see `resolve_load_order`/`validate_dependency_graph`.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct PluginDependency {
    pub plugin_id: String,
    pub version: VersionReq,
}

impl PluginDependency {
    pub fn new(plugin_id: impl Into<String>, version: VersionReq) -> Self {
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
    pub document_schema: String,
    pub document_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub contributor: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
}

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
/// @emoji 🗂️ One artifact-kind choice offered by an `ActionArgControl::ArtifactKind` dialog field —
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

/// @emoji 🎭️ One `(pluginId, appId, role)` choice offered by an `ActionArgControl::SurfaceApp` dialog
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

/// 🗂️ Every artifact-kind choice for the given `roles`: every app across `manifests` whose `role` is
/// in `roles` and whose `io.document_schema` is non-empty contributes one choice per dialect
/// coordinate. Deduped by dialect coordinate (first manifest/app wins — callers pass owner manifests
/// first so the owner's label wins over a later contributor's), sorted by coordinate for determinism
/// — the pure resolver behind `ActionArgControl::ArtifactKind`.
pub async fn artifact_kind_choices(manifests: &[PluginManifest], roles: &[AppRole]) -> Vec<ArtifactKindChoice> {
    let mut by_coordinate: BTreeMap<String, ArtifactKindChoice> = BTreeMap::new();
    for manifest in manifests {
        for app in &manifest.apps {
            if !roles.contains(&app.role) || app.io.document_schema.is_empty() {
                continue;
            }
            by_coordinate.entry(app.dialect.to_coordinate()).or_insert_with(|| ArtifactKindChoice { kind_id: app.dialect.artifact_kind.clone(), schema: app.io.document_schema.clone(), dialect: app.dialect.clone(), label: app.label.clone() });
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
async fn validate_dependency_graph(manifests: &[PluginManifest]) -> Result<(), DependencyGraphError> {
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
pub async fn resolve_load_order(manifests: &[PluginManifest]) -> Result<Vec<String>, DependencyGraphError> {
    validate_dependency_graph(manifests).await?;

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
        return Err(DependencyGraphError::Cycle { members: find_cycle_members(manifests, &leftover).await });
    }
    Ok(order)
}

/// 🔁️ Walks the leftover (never-ready) subgraph depth-first from its lexicographically smallest
/// node, following each plugin's first declared dependency that is also leftover, until a node
/// repeats — the repeated slice of the walked path is the named cycle.
async fn find_cycle_members(manifests: &[PluginManifest], leftover: &std::collections::BTreeSet<String>) -> Vec<String> {
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
pub async fn dependents(manifests: &[PluginManifest], plugin_id: &str) -> Vec<String> {
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
// `ToValue`/`FromValue` in `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️locale_terminology_value.rs`
// (that crate's own `🤖️generated.rs` is do-not-edit, so the impls live in a sibling `#[path]`
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub panel_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub contributions_json: Option<String>,
    /// 🗣️ Active UI locale; plugins resolve their own label set from this via `resolve_labels`/
    /// `app_labels!`. Non-optional — the shell always resolves one (see `initUiLocaleSync`/
    /// `detectShellLocale`) before the first `render`, so "nobody set the locale" is unrepresentable.
    #[serde(default)]
    #[value(default)]
    pub locale: Locale,
    /// 🗣️ Active terminology id (`Native` default, or an app-declared alternative term set).
    #[serde(default)]
    #[value(default)]
    pub terminology: Terminology,
    /// 🪟️ The window instance a `render`/`handle_action` call targets — programs key all per-window
    /// option state (grid, LOD, selection mode, …) off this, never off `active_window_kind_id`, so that
    /// two window instances of the same kind (e.g. split top/perspective panes) never share options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// 🪟️ The live set of open window instances (base + spawned/split), sent on every refresh/action so
    /// `window_engagements`/`window_measures` can return one entry per instance instead of per kind.
    #[serde(default)]
    #[value(default)]
    pub window_instances: Vec<ViewWindowInstance>,
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
/// own `document_media_type`, flattened to one row per format kind across every app the package
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
// CapabilityRequest>` gates HOST PRIVILEGE (`documents.write`, `fs:*`, …) this package asks the
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
/// @emoji 🤖️ What a package OFFERS to agents — see the region header above for the critical
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
    /// @emoji 🧪️ `promoted ⊆ capabilities` — the one structural invariant every producer
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
    pub name: String,
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

/// 🔌️ An app's full media I/O surface — the document schema/type every app carries implicitly (see
/// `document_in_port`/`document_out_port`) plus whatever additional workflow ports, catalog
/// export/import formats, and OS presentation it declares itself. Scaffolding for the typed manifest
/// surface (`AppDefinition.io`); apps don't populate this yet — later waves migrate `media_inputs`/
/// `media_outputs`/`artifact_kinds` onto it.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct AppIo {
    pub document_schema: String,
    pub document_media_type: MediaType,
    /// 🔌️ App-specific ports only — the implicit document ports are auto-injected by `all_ports`.
    pub ports: Vec<MediaPortSpec>,
    pub export_formats: Vec<String>,
    pub import_formats: Vec<String>,
    pub artifact: ArtifactPresentation,
}

impl AppIo {
    /// 🔌️ The implicit `"document:in"` port every app accepts, keyed by `self.document_media_type`.
    pub async fn document_in_port(&self) -> MediaPortSpec {
        MediaPortSpec { id: "document:in".into(), label: "Document".into(), direction: MediaPortDirection::In, media_type: self.document_media_type, kind_id: None, required: true, multiplicity: PortMultiplicity::One }
    }

    /// 🔌️ The implicit `"document:out"` port every app produces — see `document_in_port`.
    pub async fn document_out_port(&self) -> MediaPortSpec {
        MediaPortSpec { id: "document:out".into(), label: "Document".into(), direction: MediaPortDirection::Out, media_type: self.document_media_type, kind_id: None, required: true, multiplicity: PortMultiplicity::One }
    }

    /// 🔌️ The full port list, in stable order: the implicit document ports first, followed by every app-specific port declared in `self.ports`.
    pub async fn all_ports(&self) -> Vec<MediaPortSpec> {
        let mut ports = vec![self.document_in_port().await, self.document_out_port().await];
        ports.extend(self.ports.clone());
        ports
    }

    /// 🏗️ Builds an `AppIo` from just its implicit document surface, with no extra ports/formats declared yet — chain `.with_ports(...)` to add app-specific ports.
    pub async fn from_document(schema: impl Into<String>, media_type: MediaType, artifact: ArtifactPresentation) -> Self {
        Self { document_schema: schema.into(), document_media_type: media_type, ports: Vec::new(), export_formats: Vec::new(), import_formats: Vec::new(), artifact }
    }

    /// 🔌️ Attaches app-specific ports (beyond the implicit document ports) to this `AppIo`.
    pub async fn with_ports(mut self, ports: Vec<MediaPortSpec>) -> Self {
        self.ports = ports;
        self
    }
}

impl Default for AppIo {
    fn default() -> Self {
        Self {
            document_schema: String::new(),
            document_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            ports: Vec::new(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            artifact: ArtifactPresentation { id: String::new(), name: String::new(), dimension: String::new(), component_kind: String::new() },
        }
    }
}
//#endregion 🔖️AppIo

//#region 🔖️ConfigSpec
/// 🧮️ How one config field's value is edited/validated, independent of what record it belongs to.
/// Deliberately hand-rolled rather than derived from `dsl_schema::Shape` (`dsl_schema`'s `Shape` isn't
/// `Serialize`/`Deserialize` — `Shape::Record`/`Statements`/`Table` carry `fn() -> RecordSpec` pointers
/// — and `semio-framework-core` doesn't depend on `dsl`/`dsl_schema` today, so wrapping it would add a
/// new cross-crate dependency purely to reach a shape that can't round-trip over the wire anyway).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", tag = "kind")]
pub enum ConfigFieldShape {
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
    },
    Toggle,
    Text,
    Select {
        options: Vec<String>,
    },
    Record {
        fields: Vec<ConfigFieldSpec>,
    },
}

/// 🧮️ One field of an app's declared configuration record — the whole-app-settings counterpart to
/// `ActionArgDef` (which scopes to a single action's arguments instead).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ConfigFieldSpec {
    pub key: String,
    pub label: String,
    pub shape: ConfigFieldShape,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub default: Option<DslValue>,
}

/// 🧮️ An app's full typed configuration record — the manifest-level declaration
/// `AppDefinition.config` carries. Empty until per-app waves populate it.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ConfigSpec {
    #[serde(default)]
    #[value(default)]
    pub fields: Vec<ConfigFieldSpec>,
}

impl ConfigSpec {
    pub async fn empty() -> Self {
        Self::default()
    }
}
//#endregion 🔖️ConfigSpec

//#region 🔖️CommandGrammar
/// 🎛️ One field of a binary command variant — reuses `ConfigFieldShape` for the value shape (see
/// `ConfigFieldShape`'s doc comment for why command grammar fields are hand-rolled rather than
/// derived from `dsl_schema`). No `List`/array shape exists yet — the manifest's existing field-typed
/// vocabulary (`ActionArgControl`: Text/Number/Slider/Toggle/Select/Vec3/IconSelect) has no array
/// control either, so `ConfigFieldShape` doesn't invent one ahead of a real need.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandFieldSpec {
    pub key: String,
    pub shape: ConfigFieldShape,
    pub optional: bool,
}

/// 🎛️ One keyword-dispatched command variant (e.g. `move x=1 y=2`) and its field grammar.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandVariantSpec {
    pub keyword: String,
    pub fields: Vec<CommandFieldSpec>,
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
//#endregion 🔖️Manifest

// #endregion 🛂️Manifest
