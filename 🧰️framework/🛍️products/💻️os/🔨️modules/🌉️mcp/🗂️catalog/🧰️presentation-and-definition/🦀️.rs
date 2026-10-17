//! 🧰️ Presentation and definition of the catalog: the tool exposure, the argument summary and presentation, the example, the source, the capability definition and the action input schema builder. See [`super`].

use super::*;

//#region 🔖️ToolExposure
/// 🔌️ Whether a capability is invocable only through `action.invoke`/the deterministic catalog
/// (`CatalogOnly`, the common case) or ALSO published as its own named `tools/list` entry
/// (`Direct` — the small stable core set P2 registers, plus whatever a later packet promotes).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ToolExposure {
    CatalogOnly,
    Direct { tool_name: String },
}
//#endregion 🔖️ToolExposure

//#region 🔖️CapabilityPresentation
/// 📝️ One argument's search/display-facing summary — on the wire deliberately not the full `manifest::ActionArgDef`
/// (whose `schema`/`presentation` already live in `input_schema` below; repeating them there would be
/// duplicate state).
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityArgSummary {
    pub id: String,
    pub label: String,
    pub required: bool,
    /// 🧮️ The declared argument itself, never on the wire: `ActionAdapter::prepare` runs an agent's input through the
    /// shells' one effective-args rule (`manifest::effective_action_args`) with it, so an omitted optional argument
    /// runs with its declared `default`. `None` only for a summary decoded from a value.
    #[serde(skip)]
    #[value(skip)]
    pub definition: Option<manifest::ActionArgDef>,
}

/// 🎨️ UI-shaped presentation hints carried through from the source `ActionDefinition`/
/// `CommandDefinition` — everything a renderer needs to draw a palette row without re-deriving it
/// from `manifest` types the gateway crate otherwise never exposes on the wire.
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityPresentation {
    pub icon_id: Option<String>,
    pub category: Option<String>,
    pub keys: Option<String>,
    pub in_palette: bool,
    pub args: Vec<CapabilityArgSummary>,
}
//#endregion 🔖️CapabilityPresentation

//#region 🔖️CapabilityExample
/// 📖️ One natural-language usage example paired with the concrete input it would dispatch —
/// `input: Null` means only the phrase is known yet (the common case pre-enrichment; P13/P14 fill
/// in real inputs per `📋️master.md` §4.2's DAG).
/// 🌉️ `serde_json::Value` ↔ `DslValue` bridge for `CapabilityExample::input`, built on
/// `🌱️value/🦀️.rs`'s own infallible `From<&DslValue>`/`From<&serde_json::Value>` impls.
fn json_value_to_dsl(value: &serde_json::Value) -> DslValue {
    DslValue::from(value)
}

/// 🌉️ See [`json_value_to_dsl`] — the `FromValue` direction, infallible.
fn dsl_to_json_value(value: DslValue) -> Result<serde_json::Value, ValueError> {
    Ok(serde_json::Value::from(value))
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityExample {
    pub request: String,
    #[value(serialize_with = "json_value_to_dsl", deserialize_with = "dsl_to_json_value")]
    pub input: serde_json::Value,
}
//#endregion 🔖️CapabilityExample

//#region 🔖️CapabilitySource
/// 🧵️ Where a compiled capability came from — lets a debugging tool or `catalog lint` walk back to
/// the exact manifest declaration without re-deriving it from the id string.
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CapabilitySource {
    Action { plugin_id: String, app_id: String, window_kind_id: String, action_id: String },
    Command { plugin_id: Option<String>, app_id: Option<String>, mode_id: Option<String>, command_id: String },
    ShellCommand { variant: String },
    OsCommand { id: String },
    Descriptor { category: String, id: String },
    Gateway,
}
//#endregion 🔖️CapabilitySource

//#region 🔖️CapabilityDefinition
/// 🎯️ The gateway's compiled, tool-and-search-ready projection of one invocable operation — see this
/// module's header doc (D5). `title`/`description` are already resolved to the `locale`×`terminology`
/// `compile()` was called with (never both languages at once — `conformance::check_bilingual_labels`
/// compiles twice, once per locale, to verify both resolve non-empty).
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityDefinition {
    pub id: CapabilityRef,
    pub version: u32,
    pub owner: CapabilityOwner,
    pub kind: CapabilityKind,
    /// 🎯️ Who this capability is addressed to — see [`CapabilityAudience`].
    pub audience: CapabilityAudience,
    pub title: String,
    pub description: String,
    pub artifact_kind: Option<String>,
    pub use_when: Vec<String>,
    #[value(serialize_with = "json_value_to_dsl", deserialize_with = "dsl_to_json_value")]
    pub input_schema: serde_json::Value,
    #[value(serialize_with = "json_value_to_dsl", deserialize_with = "dsl_to_json_value")]
    pub output_schema: serde_json::Value,
    pub effects: manifest::CapabilityEffects,
    pub policy: manifest::CapabilityPolicy,
    pub execution: manifest::CapabilityExecution,
    pub exposure: ToolExposure,
    pub presentation: CapabilityPresentation,
    pub examples: Vec<CapabilityExample>,
    pub source: CapabilitySource,
}
//#endregion 🔖️CapabilityDefinition

//#region 🔖️SchemaBuilders
/// 📐️ `📋️master.md` §3.2 step 1's input envelope, folding one action/command's declared args in as
/// `properties`. Leaf schemas come from `ActionArgDef::json_schema()` (P3, `🛂️manifest/🦀️.rs`
/// `🔖️ActionArgs`), their descriptions resolved to the catalog's `locale`×`terminology`; the envelope itself is
/// `crate::schema`'s `CapabilityActionInput` export.
pub(super) fn action_input_schema(capability_id: &str, args: &[manifest::ActionArgDef], locale: Locale, terminology: Terminology) -> serde_json::Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();
    for arg in args {
        properties.insert(arg.id.clone(), dsl_to_json_value(arg.json_schema(terminology, locale)).expect("DslValue to JSON schema conversion is infallible"));
        if arg.required {
            required.push(arg.id.clone());
        }
    }
    capability_action_input_schema(capability_id, properties, required)
}
//#endregion 🔖️SchemaBuilders
