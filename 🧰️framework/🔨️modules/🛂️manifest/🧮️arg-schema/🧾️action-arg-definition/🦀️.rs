//! 🧾️ Action argument definition: the declared argument with its builders, validation and value coercion. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️ActionArgs
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
//#endregion 🔖️ActionArgs
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
