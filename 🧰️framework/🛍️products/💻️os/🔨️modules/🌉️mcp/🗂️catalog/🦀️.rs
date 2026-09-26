//! 🗂️ `CapabilityDefinition` + the catalog compiler — packet `P2-catalog`, `📓️design-decisions.md`
//! D5 (these types live HERE, not in `🛂️manifest`) and D3 (the `<plugin_id>.<app_id>.<action_id>`
//! id grammar is mandatory: a bare action id is never a capability id, since 14 action ids collide
//! across plugins). `compile()` walks real `semio_framework::manifest` types
//! (`PackageDescriptor`/`PluginManifest`/`AppDefinition`) — this facet is the ONLY place in the
//! gateway crate that depends on `semio-framework`; every other facet stays framework-free
//! (`📓️terra-P1a-report.md` §5, D8) so the peer ticket's mid-flight plugin-host rewrite can never
//! break this crate's own build.

use crate::schema::{artifact_create_template_input_schema, capability_action_input_schema, capability_generic_input_schema, capability_generic_output_schema, ui_dialog_open_input_schema};
use semio_framework::manifest;
use semio_framework::manifest::kernel;
use semio_framework::{Locale, Terminology};
use semio_framework_os_kernel::{DslValue, FromValue, ToValue, ValueError};
use std::collections::BTreeMap;

//#region 🔖️CapabilityRef
/// 🪪️ A capability's full identity string — `<plugin_id>.<app_id>.<action_id>`, `framework.*`,
/// `os.*`, `ui.*`, or a bare gateway verb (`context.resolve`, `capabilities.search`, …). Never a
/// bare action id (D3).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct CapabilityRef(pub String);

impl CapabilityRef {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CapabilityRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
//#endregion 🔖️CapabilityRef

//#region 🔖️CapabilityOwner
/// 📍️ Who declared a capability — `app_id`/`window_kind_id`/`mode_id` are `None` for a plugin-scope
/// (not app-scope) command, matching `PluginManifest.commands` (a real, populated field with no
/// owning app) — a deliberate widening of `📋️master.md` §3.1's literal `Plugin{plugin_id, app_id,
/// window_kind_id, mode_id}` shape, documented in `📓️terra-P2-report.md`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CapabilityOwner {
    Os,
    Framework,
    Shell,
    Plugin {
        plugin_id: String,
        /// 🏷️ The declaring plugin's own display name (`PluginManifest.label`, e.g. "Draw") — indexed
        /// by `🔎️search` so a query can find a verb by the product a human would name, not only by
        /// the lowercase id embedded in the capability ref.
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        app_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        window_kind_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mode_id: Option<String>,
    },
    Extension {
        extension_id: String,
    },
    Gateway,
}

/// 🌉️ Hand-written, not derived: `#[value(...)]` has no way to express `skip_serializing_if` on an
/// enum VARIANT's own named field (the derive's internally-tagged codegen never reads that
/// attribute there — struct-field position only) — a derive here would silently start writing
/// `appId`/`windowKindId`/`modeId` as `null` instead of omitting them, changing the wire shape.
/// Mirrors `#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]`
/// exactly, field-for-field with the omit-if-`None` behavior serde's own `skip_serializing_if`
/// gives it (and the matching implicit "missing `Option` field decodes to `None`" serde gives every
/// `Option<T>` field, `#[serde(default)]` or not).
impl ToValue for CapabilityOwner {
    fn to_value(&self) -> DslValue {
        match self {
            CapabilityOwner::Os => DslValue::object([("kind".to_string(), DslValue::String("os".to_string()))]),
            CapabilityOwner::Framework => DslValue::object([("kind".to_string(), DslValue::String("framework".to_string()))]),
            CapabilityOwner::Shell => DslValue::object([("kind".to_string(), DslValue::String("shell".to_string()))]),
            CapabilityOwner::Plugin { plugin_id, label, app_id, window_kind_id, mode_id } => {
                let mut entries = vec![("kind".to_string(), DslValue::String("plugin".to_string())), ("pluginId".to_string(), plugin_id.to_value())];
                if let Some(label) = label {
                    entries.push(("label".to_string(), label.to_value()));
                }
                if let Some(app_id) = app_id {
                    entries.push(("appId".to_string(), app_id.to_value()));
                }
                if let Some(window_kind_id) = window_kind_id {
                    entries.push(("windowKindId".to_string(), window_kind_id.to_value()));
                }
                if let Some(mode_id) = mode_id {
                    entries.push(("modeId".to_string(), mode_id.to_value()));
                }
                DslValue::Object(entries)
            }
            CapabilityOwner::Extension { extension_id } => DslValue::object([("kind".to_string(), DslValue::String("extension".to_string())), ("extensionId".to_string(), extension_id.to_value())]),
            CapabilityOwner::Gateway => DslValue::object([("kind".to_string(), DslValue::String("gateway".to_string()))]),
        }
    }
}

impl FromValue for CapabilityOwner {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let tag = entries.iter().find(|(k, _)| k == "kind").map(|(_, v)| v.clone()).ok_or_else(|| ValueError::new("missing field `kind`"))?;
        let tag = match tag {
            DslValue::String(s) => s,
            other => return Err(ValueError::new(format!("expected a string tag, found {other:?}"))),
        };
        let field = |name: &str| entries.iter().find(|(k, _)| k == name).map(|(_, v)| v.clone());
        Ok(match tag.as_str() {
            "os" => CapabilityOwner::Os,
            "framework" => CapabilityOwner::Framework,
            "shell" => CapabilityOwner::Shell,
            "plugin" => CapabilityOwner::Plugin {
                plugin_id: match field("pluginId") {
                    Some(v) => String::from_value(v).map_err(|error| error.under("pluginId"))?,
                    None => return Err(ValueError::new("missing field `pluginId`")),
                },
                label: match field("label") {
                    Some(v) => Option::<String>::from_value(v).map_err(|error| error.under("label"))?,
                    None => None,
                },
                app_id: match field("appId") {
                    Some(v) => Option::<String>::from_value(v).map_err(|error| error.under("appId"))?,
                    None => None,
                },
                window_kind_id: match field("windowKindId") {
                    Some(v) => Option::<String>::from_value(v).map_err(|error| error.under("windowKindId"))?,
                    None => None,
                },
                mode_id: match field("modeId") {
                    Some(v) => Option::<String>::from_value(v).map_err(|error| error.under("modeId"))?,
                    None => None,
                },
            },
            "extension" => CapabilityOwner::Extension {
                extension_id: match field("extensionId") {
                    Some(v) => String::from_value(v).map_err(|error| error.under("extensionId"))?,
                    None => return Err(ValueError::new("missing field `extensionId`")),
                },
            },
            "gateway" => CapabilityOwner::Gateway,
            other => return Err(ValueError::new(format!("unknown `kind` variant `{other}`"))),
        })
    }
}

impl CapabilityOwner {
    /// 🔑️ A stable, hashable grouping key for the "no duplicate (owner, title)" conformance rule —
    /// collapses the `Plugin` variant's optional fields into one string rather than requiring
    /// `CapabilityOwner` itself to be `Hash`/`Ord` (it embeds `Option<String>`s that would make that
    /// derive noisy for no benefit outside this one use).
    pub fn dedup_key(&self) -> String {
        match self {
            CapabilityOwner::Os => "os".to_string(),
            CapabilityOwner::Framework => "framework".to_string(),
            CapabilityOwner::Shell => "shell".to_string(),
            CapabilityOwner::Gateway => "gateway".to_string(),
            CapabilityOwner::Extension { extension_id } => format!("extension:{extension_id}"),
            CapabilityOwner::Plugin { plugin_id, app_id, window_kind_id, mode_id, .. } => {
                format!("plugin:{plugin_id}:{}:{}:{}", app_id.as_deref().unwrap_or(""), window_kind_id.as_deref().unwrap_or(""), mode_id.as_deref().unwrap_or(""))
            }
        }
    }
}
//#endregion 🔖️CapabilityOwner

//#region 🔖️CapabilityKind
/// 🏷️ What kind of operation a capability is — the six `manifest::ActionKind` variants plus the
/// three gateway-only kinds `📋️master.md` §3.1 names (`Query`/`Job`/`Ui`) and `Meta` (the gateway's
/// own discovery/context verbs, which are neither an artifact operation nor a UI command).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum CapabilityKind {
    Mutation,
    View,
    History,
    Clipboard,
    Shell,
    Interaction,
    Query,
    Job,
    Ui,
    Meta,
}

impl From<manifest::ActionKind> for CapabilityKind {
    fn from(kind: manifest::ActionKind) -> Self {
        match kind {
            manifest::ActionKind::Mutation => CapabilityKind::Mutation,
            manifest::ActionKind::View => CapabilityKind::View,
            manifest::ActionKind::History => CapabilityKind::History,
            manifest::ActionKind::Clipboard => CapabilityKind::Clipboard,
            manifest::ActionKind::Shell => CapabilityKind::Shell,
            manifest::ActionKind::Interaction => CapabilityKind::Interaction,
        }
    }
}
//#endregion 🔖️CapabilityKind

//#region 🔖️CapabilityAudience
/// 🎯️ WHO a compiled capability is addressed to — the gateway's own projection of
/// `manifest::CapabilityAudience`, mirroring how [`CapabilityKind`] projects `manifest::ActionKind`
/// (D5: the gateway's wire vocabulary lives HERE, the declaration vocabulary lives in `🛂️manifest`).
/// [`compile`] publishes `Agent` only: an agent choosing a tool must never have to tell
/// `patchLayer` apart from `canvasPointerMove` by reading their ids.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum CapabilityAudience {
    /// 🤖️ An intent-level verb — published to agents and to the UI.
    Agent,
    /// 🖱️ A raw pointer/keyboard/engagement input event — the UI dispatches it, agents never see it.
    Input,
    /// 🪟️ Window/view/session chrome — the UI dispatches it, agents never see it.
    Chrome,
}

impl From<manifest::CapabilityAudience> for CapabilityAudience {
    fn from(audience: manifest::CapabilityAudience) -> Self {
        match audience {
            manifest::CapabilityAudience::Agent => CapabilityAudience::Agent,
            manifest::CapabilityAudience::Input => CapabilityAudience::Input,
            manifest::CapabilityAudience::Chrome => CapabilityAudience::Chrome,
        }
    }
}

/// 🎯️ The audience set [`compile`] publishes by default — agent-meaningful verbs only.
pub const AGENT_AUDIENCES: &[CapabilityAudience] = &[CapabilityAudience::Agent];

/// 🎯️ Every audience, for the callers that want the complete compiled surface (a shell's own
/// command registry, `🧪️conformance`'s duplicate/label checks) rather than the agent projection.
pub const ALL_AUDIENCES: &[CapabilityAudience] = &[CapabilityAudience::Agent, CapabilityAudience::Input, CapabilityAudience::Chrome];
//#endregion 🔖️CapabilityAudience

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
/// 📝️ One argument's search/display-facing summary — deliberately not the full `manifest::ActionArgDef`
/// (whose `schema`/`presentation` already live in `input_schema` below; repeating them here would be
/// duplicate state).
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityArgSummary {
    pub id: String,
    pub label: String,
    pub required: bool,
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
/// `🔖️ActionArgs`); the envelope itself is `crate::schema`'s `CapabilityActionInput` export.
fn action_input_schema(capability_id: &str, args: &[manifest::ActionArgDef]) -> serde_json::Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();
    for arg in args {
        properties.insert(arg.id.clone(), dsl_to_json_value(arg.json_schema()).expect("DslValue to JSON schema conversion is infallible"));
        if arg.required {
            required.push(arg.id.clone());
        }
    }
    capability_action_input_schema(capability_id, properties, required)
}
//#endregion 🔖️SchemaBuilders

//#region 🔖️FrameworkDedup
/// 🕹️ The 5 framework-injected action ids that carry `ActionKind::View`/`🐚️Shell` (not
/// `History`/`Clipboard`/`Interaction`, which are already unambiguous by kind) — see
/// `🛂️manifest/🦀️.rs` `history_action_definitions`/`clipboard_action_definitions`/
/// `set_active_utility_action_definition`/etc.'s own doc comments for the full auto-injection story.
const FRAMEWORK_VIEW_SHELL_ACTION_IDS: [&str; 5] = ["setActiveUtility", "setActiveTool", "startIntroduction", "setHistoryCommandFilter", "noteShellCommand"];

/// 🕹️ Whether `action` is one of the 21 framework-auto-injected ids (`📓️luna-actions-audit.md` §1) —
/// these are walked ONCE per distinct id across every app (see `framework_capabilities`) rather than
/// once per plugin/app declaration, so they compile into `framework.*`, never `<plugin>.<app>.*`.
fn is_framework_injected_action(action: &manifest::ActionDefinition) -> bool {
    matches!(action.kind, manifest::ActionKind::History | manifest::ActionKind::Clipboard | manifest::ActionKind::Interaction) || FRAMEWORK_VIEW_SHELL_ACTION_IDS.contains(&action.id.as_str())
}

/// 🎬️ One entry per distinct verb an app declares across all of its window kinds, in declaration
/// order, each paired with the FIRST window kind that declares it. An app's window kinds are views
/// onto one action registry — every plugin in the installed registry repeats its whole roster in
/// every window kind (`note` 54 verbs × 2 kinds, `architect` 39 × 5, `norm` 240 × 15) — so an action
/// id is a verb of the APP, not of a window, and collapsing them here is what makes
/// `<plugin>.<app>.<verb>` a unique capability id at all. The retained window kind is deliberately a
/// concrete declaring kind rather than a `"*"` marker: it is the `window_kind_id` a dispatch has to
/// address (`ActionAddress.window_kind_id`, checked against the guest's own
/// `registry.window_action`), which `"*"` could not satisfy. Two window kinds declaring the same id
/// with DIFFERENT definitions stay a real collision, reported as
/// [`CatalogError::DuplicateCapabilityId`].
///
/// 🗒️ App-SCOPE actions (`AppDefinition.actions`) are the other half of the surface, and walking
/// only `window_kinds` silently dropped every one of them: `🗒️note` declares all 48 of its verbs
/// here, so `capabilities_search "delete the selected blocks"` could not find a single note
/// capability while `🖍️draw` — which declares on its canvas window kind — worked. An app-scope
/// action belongs to no particular window, so it addresses `"*"`, the same marker
/// `framework_capabilities` already uses for the framework-injected verbs. A window kind that
/// also declares the id wins (it carries a dispatchable concrete `window_kind_id`), which is why
/// this loop runs second and skips what `seen` already holds instead of reporting a collision.
fn app_action_verbs(app: &manifest::AppDefinition) -> Result<Vec<(&manifest::ActionDefinition, &str)>, CatalogError> {
    let mut verbs: Vec<(&manifest::ActionDefinition, &str)> = Vec::new();
    let mut seen: BTreeMap<&str, &manifest::ActionDefinition> = BTreeMap::new();
    for window_kind in app.window_kinds.iter() {
        for action in &window_kind.actions {
            match seen.get(action.id.as_str()) {
                Some(declared) if *declared == action => continue,
                Some(_) => return Err(CatalogError::DuplicateCapabilityId(format!("{}.{}", app.id, action.id))),
                None => {
                    seen.insert(action.id.as_str(), action);
                    verbs.push((action, window_kind.id.as_str()));
                }
            }
        }
    }
    for action in &app.actions {
        if seen.contains_key(action.id.as_str()) {
            continue;
        }
        seen.insert(action.id.as_str(), action);
        verbs.push((action, "*"));
    }
    Ok(verbs)
}

/// 🧰️ The framework-injected actions one app receives — the framework's own definitions, never the
/// copies a descriptor serialized, so a gateway built from this tree publishes this tree's text.
fn framework_action_definitions(app: &manifest::AppDefinition) -> Vec<manifest::ActionDefinition> {
    let mut actions = manifest::history_action_definitions();
    actions.extend(manifest::clipboard_action_definitions());
    actions.extend(manifest::interaction_action_definitions(app));
    actions.extend(manifest::tool_run_action_definitions(app));
    actions.push(manifest::set_history_command_filter_action_definition());
    actions.push(manifest::note_shell_command_action_definition());
    if !app.utilities.is_empty() {
        actions.push(manifest::set_active_utility_action_definition());
    }
    if !app.tools.is_empty() {
        actions.push(manifest::set_active_tool_action_definition());
    }
    if app.introduction.is_some() {
        actions.push(manifest::start_introduction_action_definition());
    }
    actions
}

/// 🕹️ Every framework-injected action reachable from `apps`, deduped by id (first occurrence wins —
/// every app resolves the identical `ActionDefinition` for a given framework id, since none of these
/// take app-specific data into their manifest shape) — compiled into `framework.<action.id>`
/// capabilities owned by `CapabilityOwner::Framework`.
fn framework_capabilities(apps: &[&manifest::AppDefinition], locale: Locale, terminology: Terminology) -> BTreeMap<String, CapabilityDefinition> {
    let mut map = BTreeMap::new();
    for app in apps {
        for action in framework_action_definitions(app) {
            let id = format!("framework.{}", action.id);
            map.entry(id.clone()).or_insert_with(|| {
                capability_from_action(
                    &id,
                    CapabilityOwner::Framework,
                    None,
                    &action,
                    CapabilitySource::Action { plugin_id: "framework".into(), app_id: "framework".into(), window_kind_id: "*".into(), action_id: action.id.clone() },
                    locale,
                    terminology,
                )
            });
        }
    }
    map
}
//#endregion 🔖️FrameworkDedup

//#region 🔖️CapabilityBuilders
/// 🏭️ Shared action→capability projection used by both the per-plugin walk and the framework dedup
/// pass — `owner`/`artifact_kind`/`source` differ by caller, everything else (`title`/`description`/
/// `input_schema`/`effects`/`policy`/`execution`/`presentation`/`examples`) is derived from `action`
/// identically either way.
fn capability_from_action(id: &str, owner: CapabilityOwner, artifact_kind: Option<String>, action: &manifest::ActionDefinition, source: CapabilitySource, locale: Locale, terminology: Terminology) -> CapabilityDefinition {
    let title = action.label.resolve(terminology, locale).to_string();
    let description = action.semantics.description.as_ref().map(|label| label.resolve(terminology, locale).to_string()).unwrap_or_default();
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner,
        kind: action.kind.into(),
        audience: manifest::resolve_audience(action).into(),
        title,
        description,
        artifact_kind,
        use_when: action.semantics.use_when.clone(),
        input_schema: action_input_schema(id, &action.args),
        output_schema: capability_generic_output_schema(id),
        effects: action.semantics.effects.clone(),
        policy: action.semantics.policy.clone(),
        execution: action.semantics.execution.clone(),
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation {
            icon_id: Some(action.icon_id.as_str().to_string()),
            category: action.category.clone(),
            keys: action.keys.clone(),
            in_palette: action.in_palette,
            args: action.args.iter().map(|arg| CapabilityArgSummary { id: arg.id.clone(), label: arg.label.resolve(terminology, locale).to_string(), required: arg.required }).collect(),
        },
        examples: action.semantics.examples.iter().map(|request| CapabilityExample { request: request.clone(), input: serde_json::Value::Null }).collect(),
        source,
    }
}

/// 🏭️ `CommandDefinition`→`CapabilityDefinition` projection — mirrors `capability_from_action`, one
/// tier up the owner hierarchy (`app_id`/`mode_id` are `None` for a plugin-scope command).
#[allow(clippy::too_many_arguments)]
fn capability_from_command(id: &str, owner: CapabilityOwner, artifact_kind: Option<String>, command: &manifest::CommandDefinition, source: CapabilitySource, locale: Locale, terminology: Terminology) -> CapabilityDefinition {
    let title = command.label.resolve(terminology, locale).to_string();
    let description = command.semantics.description.as_ref().map(|label| label.resolve(terminology, locale).to_string()).unwrap_or_default();
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner,
        kind: command.kind.into(),
        audience: manifest::resolve_command_audience(command).into(),
        title,
        description,
        artifact_kind,
        use_when: command.semantics.use_when.clone(),
        input_schema: action_input_schema(id, &command.args),
        output_schema: capability_generic_output_schema(id),
        effects: command.semantics.effects.clone(),
        policy: command.semantics.policy.clone(),
        execution: command.semantics.execution.clone(),
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation {
            icon_id: Some(command.icon_id.as_str().to_string()),
            category: Some(command.category.clone()),
            keys: command.keybindings.first().map(|kb| kb.chord.clone()),
            in_palette: command.in_palette,
            args: command.args.iter().map(|arg| CapabilityArgSummary { id: arg.id.clone(), label: arg.label.resolve(terminology, locale).to_string(), required: arg.required }).collect(),
        },
        examples: command.semantics.examples.iter().map(|request| CapabilityExample { request: request.clone(), input: serde_json::Value::Null }).collect(),
        source,
    }
}

/// 🧩️ One typed contribution row, reduced to the three things a capability needs: a stable local
/// id, a human title, and the artifact kind it acts on. Implemented once per `ContributionSet`
/// category so {@link capability_from_contribution} stays a single generic function — the four
/// categories carry genuinely different shapes (an inference names schemas, an io entry names a
/// dialect pair), and flattening them back into one untyped row would discard exactly the fields
/// that make a contributed capability findable.
trait ContributionRow {
    /// 🪪️ Stable id segment, unique within `(plugin, category)`.
    fn row_id(&self) -> String;
    /// 🏷️ Human title for search and display.
    fn row_title(&self) -> String;
    /// 💬️ One sentence naming what this contribution does.
    fn row_description(&self) -> String;
    /// 🗿️ Artifact kind this row acts on, when the shape names one.
    fn row_artifact_kind(&self) -> Option<String> {
        None
    }
}

impl ContributionRow for manifest::ContributedInferenceMetadata {
    fn row_id(&self) -> String {
        self.inference_schema.clone()
    }

    fn row_title(&self) -> String {
        humanize(&self.inference_schema)
    }

    fn row_description(&self) -> String {
        format!("Infers {} on {} artifacts (schema {} v{}).", self.inference_schema, self.artifact_kind, self.artifact_schema, self.inference_schema_version)
    }

    fn row_artifact_kind(&self) -> Option<String> {
        Some(self.artifact_kind.clone())
    }
}

impl ContributionRow for manifest::ContributedMutationMetadata {
    fn row_id(&self) -> String {
        self.mutation_id.clone()
    }

    fn row_title(&self) -> String {
        format!("{} {}", humanize(&self.semantics.verb), humanize(&self.semantics.entity))
    }

    fn row_description(&self) -> String {
        format!("Contributed mutation: {} a {} ({}) on {}.", self.semantics.verb, self.semantics.entity, self.semantics.kind, self.semantics.record)
    }
}

impl ContributionRow for manifest::IoEntryDescriptor {
    fn row_id(&self) -> String {
        format!("{}:{}:{}", self.owner.to_coordinate(), direction_word(&self.direction), self.counterpart.to_coordinate())
    }

    fn row_title(&self) -> String {
        match self.direction {
            manifest::IoEntryDirection::Import => format!("Import {} as {}", self.counterpart.to_coordinate(), self.owner.to_coordinate()),
            manifest::IoEntryDirection::Export => format!("Export {} as {}", self.owner.to_coordinate(), self.counterpart.to_coordinate()),
        }
    }

    fn row_description(&self) -> String {
        match self.direction {
            manifest::IoEntryDirection::Import => format!("Reads {} into the {} artifact kind.", self.counterpart.to_coordinate(), self.owner.to_coordinate()),
            manifest::IoEntryDirection::Export => format!("Writes the {} artifact kind out as {}.", self.owner.to_coordinate(), self.counterpart.to_coordinate()),
        }
    }

    fn row_artifact_kind(&self) -> Option<String> {
        Some(self.owner.artifact_kind.clone())
    }
}

/// 🧭️ `Import`/`Export` as the lowercase word used inside a contributed io capability id.
fn direction_word(direction: &manifest::IoEntryDirection) -> &'static str {
    match direction {
        manifest::IoEntryDirection::Import => "import",
        manifest::IoEntryDirection::Export => "export",
    }
}

impl ContributionRow for manifest::ComposerEntryDescriptor {
    fn row_id(&self) -> String {
        self.writes.to_coordinate()
    }

    fn row_title(&self) -> String {
        format!("Compose {}", self.writes.to_coordinate())
    }

    fn row_description(&self) -> String {
        if self.reads.is_empty() {
            format!("Composes {}.", self.writes.to_coordinate())
        } else {
            format!("Composes {} from {}.", self.writes.to_coordinate(), self.reads.iter().map(|dialect| dialect.to_coordinate()).collect::<Vec<_>>().join(", "))
        }
    }

    fn row_artifact_kind(&self) -> Option<String> {
        Some(self.writes.artifact_kind.clone())
    }
}

/// 🏭️ One typed contribution row → a `Query`/`Job` capability. Generic over {@link ContributionRow}
/// so every `ContributionSet` category is projected by the same rules while keeping its own
/// identity and wording.
fn capability_from_contribution<Row: ContributionRow>(plugin_id: &str, plugin_label: &str, category: &str, entry: &Row, kind: CapabilityKind) -> CapabilityDefinition {
    let row_id = entry.row_id();
    let id = format!("{plugin_id}.{category}.{row_id}");
    CapabilityDefinition {
        id: CapabilityRef(id.clone()),
        version: 1,
        owner: CapabilityOwner::Plugin { plugin_id: plugin_id.to_string(), label: Some(plugin_label.to_string()), app_id: None, window_kind_id: None, mode_id: None },
        kind,
        audience: CapabilityAudience::Agent,
        title: entry.row_title(),
        description: entry.row_description(),
        artifact_kind: entry.row_artifact_kind(),
        use_when: Vec::new(),
        input_schema: capability_generic_input_schema(&id),
        output_schema: capability_generic_output_schema(&id),
        effects: manifest::CapabilityEffects::default(),
        policy: manifest::CapabilityPolicy::default(),
        execution: manifest::CapabilityExecution { class: manifest::ExecutionClass::Job, interactive_job: manifest::InteractiveJobClassification::Migrated, ..Default::default() },
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation { icon_id: None, category: Some(category.to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Descriptor { category: category.to_string(), id: row_id },
    }
}

/// 🏭️ `CommandDefinition`→capability for one real `💻️os/🎮️commands/*` module — `owner: Os`, `source:
/// OsCommand`. The `id` is already fully qualified (`os.<slug>`, see every `pub const ID` in
/// `💻️os/🎮️commands/*/🦀️.rs`) — used as-is, never re-prefixed.
fn capability_from_os_command(command: &manifest::CommandDefinition, locale: Locale, terminology: Terminology) -> CapabilityDefinition {
    capability_from_command(&command.id.clone(), CapabilityOwner::Os, None, command, CapabilitySource::OsCommand { id: command.id.clone() }, locale, terminology)
}

/// 🔤️ `"open-artifact"`/`"myThing"` → `"Open Artifact"`/`"My Thing"` — a last-resort title for
/// untyped `DescriptorEntry` rows that have no `LocalizedLabel` to resolve.
fn humanize(id: &str) -> String {
    let spaced: String = id.chars().flat_map(|character| if character.is_uppercase() { vec![' ', character] } else { vec![character] }).collect();
    spaced
        .replace(['-', '_'], " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
//#endregion 🔖️CapabilityBuilders

//#region 🔖️UiDialogAndArtifactCreate
/// 🗨️ `📋️master.md` §3.2: "`dialogs` → `ui.dialog.open`" — every app's declared `DialogDefinition`s
/// fold into ONE gateway-owned `ui.dialog.open` capability (never one capability per dialog), whose
/// `dialogId` argument enumerates every dialog id collected across every walked app.
fn ui_dialog_open_capability(dialog_ids: &[String]) -> CapabilityDefinition {
    let input_schema = ui_dialog_open_input_schema(dialog_ids);
    CapabilityDefinition {
        id: CapabilityRef("ui.dialog.open".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Ui,
        audience: CapabilityAudience::Agent,
        title: "Open Dialog".to_string(),
        description: "Opens one of the workspace's declared modal form dialogs by id.".to_string(),
        artifact_kind: None,
        use_when: vec!["open a dialog".to_string(), "show a form".to_string()],
        input_schema,
        output_schema: capability_generic_output_schema("ui.dialog.open"),
        effects: manifest::CapabilityEffects { reads: vec![manifest::ResourceSelector::new("ui:window")], ..Default::default() },
        policy: manifest::CapabilityPolicy { scopes: vec![kernel::CapabilityId("ui.dialog".into())], ..Default::default() },
        execution: manifest::CapabilityExecution::default(),
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation { icon_id: None, category: Some("ui".into()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

/// 📖️ `📋️master.md` §3.2: "`examples` → the `artifact.create` template enum" — every plugin's
/// declared `ExampleDefinition`s fold into ONE gateway-owned `artifact.create` capability, whose
/// `template` argument enumerates every `<plugin_id>:<example_id>` collected across every descriptor.
fn artifact_create_capability(template_ids: &[String]) -> CapabilityDefinition {
    let input_schema = artifact_create_template_input_schema(template_ids);
    CapabilityDefinition {
        id: CapabilityRef("artifact.create".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Job,
        audience: CapabilityAudience::Agent,
        title: "Create Artifact".to_string(),
        description: "Creates a new artifact, optionally seeded from a declared playground example/template.".to_string(),
        artifact_kind: None,
        use_when: vec!["create a new artifact".to_string(), "start a new document".to_string()],
        input_schema,
        output_schema: capability_generic_output_schema("artifact.create"),
        effects: manifest::CapabilityEffects { writes: vec![manifest::ResourceSelector::new("artifact:{self}")], reversible: false, ..Default::default() },
        policy: manifest::CapabilityPolicy { scopes: vec![kernel::CapabilityId("artifacts.write".into())], ..Default::default() },
        execution: manifest::CapabilityExecution { class: manifest::ExecutionClass::Job, interactive_job: manifest::InteractiveJobClassification::Migrated, ..Default::default() },
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation { icon_id: None, category: Some("artifact".into()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}
//#endregion 🔖️UiDialogAndArtifactCreate

//#region 🔖️CatalogSource
/// 📥️ Everything `compile()` walks — `📋️master.md` §3.2's `CatalogSource{descriptors, os_commands,
/// shell, gateway}` verbatim. `shell`/`gateway` are pre-built `CapabilityDefinition`s (P9's
/// `os_shell::shell_capabilities()` and this crate's own core-tool set respectively) — everything
/// else is compiled FROM the manifest source types.
#[derive(Clone, Debug, Default)]
pub struct CatalogSource {
    pub descriptors: Vec<manifest::PackageDescriptor>,
    pub os_commands: Vec<manifest::CommandDefinition>,
    pub shell: Vec<CapabilityDefinition>,
    pub gateway: Vec<CapabilityDefinition>,
}
//#endregion 🔖️CatalogSource

//#region 🔖️CatalogError
#[derive(Debug, Clone, PartialEq)]
pub enum CatalogError {
    DuplicateCapabilityId(String),
}

impl std::fmt::Display for CatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatalogError::DuplicateCapabilityId(id) => write!(f, "duplicate capability id: {id}"),
        }
    }
}

impl std::error::Error for CatalogError {}
//#endregion 🔖️CatalogError

//#region 🔖️Catalog
/// 📚️ The compiled, sorted, content-addressed capability catalog — `hash` changes if and only if
/// `entries` changes (blake3 over the canonical JSON serialization of the sorted entry vector, so
/// compiling the SAME `CatalogSource` twice is byte-identical, proven in `🧪️Tests` below).
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
pub struct Catalog {
    pub hash: String,
    pub entries: Vec<CapabilityDefinition>,
}

impl Catalog {
    /// 🔎️ Binary-search lookup by id — valid because `entries` is always sorted by `id.0`.
    pub fn get(&self, id: &str) -> Option<&CapabilityDefinition> {
        self.entries.binary_search_by(|entry| entry.id.as_str().cmp(id)).ok().map(|index| &self.entries[index])
    }

    /// 🔧️ Every capability whose `exposure` is `Direct` — the seam `root::component`'s tool
    /// registration walks to publish `tools/list` entries beyond the hand-registered core three.
    pub fn direct_exposures(&self) -> impl Iterator<Item = &CapabilityDefinition> {
        self.entries.iter().filter(|entry| matches!(entry.exposure, ToolExposure::Direct { .. }))
    }
}

fn compute_catalog_entries_hash(entries: &[CapabilityDefinition]) -> String {
    let bytes = serde_json::to_vec(entries).expect("capability catalog entries always serialize");
    framework_hash::hash_bytes(&bytes)
}
//#endregion 🔖️Catalog

//#region 🔖️Compile
fn insert_capability(entries: &mut BTreeMap<String, CapabilityDefinition>, capability: CapabilityDefinition) -> Result<(), CatalogError> {
    let id = capability.id.0.clone();
    if entries.contains_key(&id) {
        return Err(CatalogError::DuplicateCapabilityId(id));
    }
    entries.insert(id, capability);
    Ok(())
}

/// 🏭️ Compiles a `CatalogSource` into a sorted, hashed `Catalog` — `📋️master.md` §3.2. Walks, per
/// descriptor: every app's window-kind actions (skipping the 21 framework-injected ids, folded
/// separately into `framework.*` — see `framework_capabilities`), app-scope commands, mode-scope
/// commands, plugin-scope commands, and the four contribution categories (inference/mutation/io/
/// composer → `Query`/`Job`); then folds every collected `DialogDefinition`/`ExampleDefinition` id
/// into the two gateway-owned `ui.dialog.open`/`artifact.create` capabilities; then walks
/// `source.os_commands` and appends `source.shell`/`source.gateway` verbatim.
pub fn compile(source: &CatalogSource, locale: Locale, terminology: Terminology) -> Result<Catalog, CatalogError> {
    compile_with_audiences(source, locale, terminology, AGENT_AUDIENCES)
}

/// 🏭️ [`compile`] with an explicit audience projection — see [`CapabilityAudience`]. Every
/// declaration is still walked and still fights for its id (a duplicate stays
/// [`CatalogError::DuplicateCapabilityId`] even when the colliding pair is filtered out), so
/// narrowing the published set can never hide a real collision; only the FINAL entry vector, and
/// therefore `hash`, is filtered.
///
/// 🌱️ `artifact.create` has exactly ONE definition in a compiled catalog: `🗿️artifact`'s own
/// invocable `artifact_create` tool when the source carries it (the live gateway always does),
/// otherwise this module's catalog-only projection. The declared templates are an ARGUMENT of
/// that verb, never a second capability wearing its id — which is what made a live catalog with
/// both sources refuse to compile at all (`duplicate capability id: artifact.create`).
pub fn compile_with_audiences(source: &CatalogSource, locale: Locale, terminology: Terminology, audiences: &[CapabilityAudience]) -> Result<Catalog, CatalogError> {
    let mut entries: BTreeMap<String, CapabilityDefinition> = BTreeMap::new();
    let mut all_apps: Vec<&manifest::AppDefinition> = Vec::new();
    let mut dialog_ids: Vec<String> = Vec::new();
    let mut template_ids: Vec<String> = Vec::new();

    for descriptor in &source.descriptors {
        let plugin_id = descriptor.manifest.plugin_id.clone();
        let plugin_label = descriptor.manifest.label.clone();

        for app in descriptor.manifest.apps.iter() {
            let app_id = app.id.clone();
            let artifact_kind = app.dialect.artifact_kind.clone();

            for (action, window_kind_id) in app_action_verbs(app)? {
                if is_framework_injected_action(action) {
                    continue;
                }
                let id = format!("{plugin_id}.{app_id}.{}", action.id);
                let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), label: Some(plugin_label.clone()), app_id: Some(app_id.clone()), window_kind_id: Some(window_kind_id.to_string()), mode_id: None };
                let source_ref = CapabilitySource::Action { plugin_id: plugin_id.clone(), app_id: app_id.clone(), window_kind_id: window_kind_id.to_string(), action_id: action.id.clone() };
                insert_capability(&mut entries, capability_from_action(&id, owner, Some(artifact_kind.clone()), action, source_ref, locale, terminology))?;
            }

            for command in &app.commands {
                let id = format!("{plugin_id}.{app_id}.cmd.{}", command.id);
                let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), label: Some(plugin_label.clone()), app_id: Some(app_id.clone()), window_kind_id: None, mode_id: None };
                let source_ref = CapabilitySource::Command { plugin_id: Some(plugin_id.clone()), app_id: Some(app_id.clone()), mode_id: None, command_id: command.id.clone() };
                insert_capability(&mut entries, capability_from_command(&id, owner, Some(artifact_kind.clone()), command, source_ref, locale, terminology))?;
            }

            for mode in app.modes.iter() {
                for command in &mode.commands {
                    let id = format!("{plugin_id}.{app_id}.mode.{}.{}", mode.id, command.id);
                    let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), label: Some(plugin_label.clone()), app_id: Some(app_id.clone()), window_kind_id: None, mode_id: Some(mode.id.clone()) };
                    let source_ref = CapabilitySource::Command { plugin_id: Some(plugin_id.clone()), app_id: Some(app_id.clone()), mode_id: Some(mode.id.clone()), command_id: command.id.clone() };
                    insert_capability(&mut entries, capability_from_command(&id, owner, Some(artifact_kind.clone()), command, source_ref, locale, terminology))?;
                }
            }

            for dialog in &app.dialogs {
                dialog_ids.push(dialog.id.clone());
            }

            all_apps.push(app);
        }

        for command in &descriptor.manifest.commands {
            let id = format!("{plugin_id}.cmd.{}", command.id);
            let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), label: Some(plugin_label.clone()), app_id: None, window_kind_id: None, mode_id: None };
            let source_ref = CapabilitySource::Command { plugin_id: Some(plugin_id.clone()), app_id: None, mode_id: None, command_id: command.id.clone() };
            insert_capability(&mut entries, capability_from_command(&id, owner, None, command, source_ref, locale, terminology))?;
        }

        for example in &descriptor.manifest.examples {
            template_ids.push(format!("{plugin_id}:{}", example.id));
        }

        for entry in &descriptor.contributions.inference_services {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, &plugin_label, "infer", entry, CapabilityKind::Query))?;
        }
        for entry in &descriptor.contributions.mutation_services {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, &plugin_label, "mutate", entry, CapabilityKind::Job))?;
        }
        for entry in &descriptor.contributions.io_entries {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, &plugin_label, "io", entry, CapabilityKind::Job))?;
        }
        for entry in &descriptor.contributions.composer_entries {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, &plugin_label, "compose", entry, CapabilityKind::Job))?;
        }
    }

    for capability in framework_capabilities(&all_apps, locale, terminology).into_values() {
        insert_capability(&mut entries, capability)?;
    }

    if !dialog_ids.is_empty() {
        dialog_ids.sort();
        dialog_ids.dedup();
        insert_capability(&mut entries, ui_dialog_open_capability(&dialog_ids))?;
    }
    for command in &source.os_commands {
        insert_capability(&mut entries, capability_from_os_command(command, locale, terminology))?;
    }

    for capability in &source.shell {
        insert_capability(&mut entries, capability.clone())?;
    }
    for capability in &source.gateway {
        insert_capability(&mut entries, capability.clone())?;
    }

    if !template_ids.is_empty() {
        template_ids.sort();
        template_ids.dedup();
        match entries.get_mut("artifact.create") {
            Some(capability) => capability.input_schema = crate::schema::with_artifact_create_templates(capability.input_schema.clone(), &template_ids),
            None => insert_capability(&mut entries, artifact_create_capability(&template_ids))?,
        }
    }

    let sorted: Vec<CapabilityDefinition> = entries.into_values().filter(|capability| audiences.contains(&capability.audience)).collect();
    let hash = compute_catalog_entries_hash(&sorted);
    Ok(Catalog { hash, entries: sorted })
}
//#endregion 🔖️Compile

//#region 🔖️Audit
/// 🖱️ Verb-id word runs that NAME a raw live-surface gesture route. A route whose id reads like one
/// of these is dispatched by a cursor, a key or an engagement draft — never by an agent choosing a
/// tool — but `manifest::derive_audience` cannot see that: a pointer handler that commits a real
/// operation is `ActionKind::Mutation`, structurally identical to a panel-dispatched `patchLayer`.
/// So the lexicon is not a classifier — [`audit_source`] uses it only to ask the question the
/// derivation cannot: "this id names an event; did anyone actually declare its audience?".
pub const GESTURE_ROUTE_WORDS: &[&str] = &[
    "pointerdown",
    "pointerup",
    "pointermove",
    "pointercancel",
    "pointerenter",
    "pointerleave",
    "mousedown",
    "mouseup",
    "mousemove",
    "doubleclick",
    "dblclick",
    "dragstart",
    "dragmove",
    "dragend",
    "dragover",
    "dragenter",
    "dragleave",
    "drop",
    "wheel",
    "keydown",
    "keyup",
    "keypress",
    "escape",
    "hover",
    "touchstart",
    "touchmove",
    "touchend",
    "gesture",
    "engagementinput",
    "engagementsubmit",
    "engagementcancel",
    "engagementabort",
    "engagementcommit",
    "commitdraft",
    "canceldraft",
    "updatedraft",
    "applyevents",
];

/// ⚠️ Verb-id word runs that NAME the delete/clear/reset class — the verbs a human wants to be asked
/// about before an agent commits them. Same contract as [`GESTURE_ROUTE_WORDS`]: the lexicon asks the
/// question, the declaration (`ActionDefinition::destructive` / `AppBuilder::action_destructive`)
/// answers it. Asked of document mutations and of shell verbs, whose side effects (a deleted space, a
/// removed member) live outside the document history entirely.
pub const DESTRUCTIVE_VERB_WORDS: &[&str] = &["delete", "remove", "clear", "discard", "purge", "wipe", "erase", "truncate", "reset", "replace", "overwrite"];

/// 📄️ Verb-id word runs that NAME a replacement of the whole document by supplied or example content.
/// Only a `Mutation` replaces the document — a shell verb of the same name navigates elsewhere — so
/// this lexicon is asked of mutations alone.
pub const DOCUMENT_REPLACE_WORDS: &[&str] = &["setactiveexample", "setfixturejson", "setspecjson", "setsnapshot", "commitdocument", "loaddocument", "setdocument"];

/// 💾️ Verb-id word runs that NAME a write to a path the user owns — an export, a download, a save.
/// Such a verb creates or overwrites a file outside the artifact's own history, so no undo reaches it;
/// published to an agent it must ask first. Asked of shell verbs and of palette view verbs (a
/// `View`-kind `exportDocument` renders and hands the file to the host just the same).
pub const USER_PATH_WRITE_WORDS: &[&str] = &["export", "download", "save"];

/// ✂️ Splits a verb id into its lowercase words at camel-case and separator boundaries —
/// `canvasPointerDown` → `["canvas", "pointer", "down"]`.
fn verb_id_words(id: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    for character in id.chars() {
        if character == '.' || character == '_' || character == '-' || character == ':' {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            continue;
        }
        if character.is_ascii_uppercase() && !current.is_empty() {
            words.push(std::mem::take(&mut current));
        }
        current.push(character.to_ascii_lowercase());
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

/// 🔍️ The first lexicon entry that equals a CONTIGUOUS run of `id`'s words — so `dropOnPool` matches
/// `drop` while `addDropdown` (one word `dropdown`) matches nothing.
fn matching_lexicon_word(id: &str, lexicon: &[&'static str]) -> Option<&'static str> {
    let words = verb_id_words(id);
    for start in 0..words.len() {
        let mut run = String::new();
        for word in &words[start..] {
            run.push_str(word);
            if let Some(entry) = lexicon.iter().find(|candidate| **candidate == run) {
                return Some(entry);
            }
        }
    }
    None
}

/// 🚨️ One thing an agent-published capability gets wrong about itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CatalogAuditFinding {
    /// 🖱️ Published to agents, named like a live-surface gesture route, and nobody declared an
    /// audience — so `derive_audience` guessed `Agent` and the guess is unreviewed. Fix at the
    /// declaration: `ActionDefinition::input_event()` or `AppBuilder::action_audience(id, Input)`,
    /// or declare `Agent` explicitly if the name lies and it really is an intent verb.
    UndeclaredGestureRoute { capability_id: String, matched: &'static str },
    /// ⚠️ A `Mutation` or `Shell` verb published to agents whose id names the delete/clear/replace class but whose
    /// `effects.destructive` is false — so `ApprovalMode::WhenDestructive` never fires and an agent
    /// can discard the user's content without anyone being asked. Fix at the declaration:
    /// `ActionDefinition::destructive()` or `AppBuilder::action_destructive(id)`.
    UnmarkedDestructiveVerb { capability_id: String, matched: &'static str },
    /// 💾️ A shell or view verb published to agents whose id names an export/download/save but whose
    /// `effects.destructive` is false — so an agent writes to the user's disk without anyone being
    /// asked. Fix at the declaration: `AppBuilder::action_destructive(id)`.
    UnmarkedUserPathWrite { capability_id: String, matched: &'static str },
}

impl CatalogAuditFinding {
    pub fn capability_id(&self) -> &str {
        match self {
            CatalogAuditFinding::UndeclaredGestureRoute { capability_id, .. } | CatalogAuditFinding::UnmarkedDestructiveVerb { capability_id, .. } | CatalogAuditFinding::UnmarkedUserPathWrite { capability_id, .. } => capability_id,
        }
    }

    pub fn message(&self) -> String {
        match self {
            CatalogAuditFinding::UndeclaredGestureRoute { capability_id, matched } => {
                format!("{capability_id} reads as a `{matched}` gesture route but declares no audience — derive_audience published it to agents unreviewed")
            }
            CatalogAuditFinding::UnmarkedDestructiveVerb { capability_id, matched } => {
                format!("{capability_id} is a `{matched}`-class verb published to agents with effects.destructive = false — WhenDestructive never fires")
            }
            CatalogAuditFinding::UnmarkedUserPathWrite { capability_id, matched } => {
                format!("{capability_id} is a `{matched}` write to a user path published to agents with effects.destructive = false — no human is asked before the file is written")
            }
        }
    }
}

/// 🧭️ Audits one action/command declaration in place — shared by both walks below.
fn audit_declaration(capability_id: String, verb_id: &str, audience: manifest::CapabilityAudience, declared: bool, kind: manifest::ActionKind, destructive: bool, findings: &mut Vec<CatalogAuditFinding>) {
    if audience != manifest::CapabilityAudience::Agent {
        return;
    }
    if !declared {
        if let Some(matched) = matching_lexicon_word(verb_id, GESTURE_ROUTE_WORDS) {
            findings.push(CatalogAuditFinding::UndeclaredGestureRoute { capability_id: capability_id.clone(), matched });
        }
    }
    if destructive {
        return;
    }
    let replaces_document = if kind == manifest::ActionKind::Mutation { matching_lexicon_word(verb_id, DOCUMENT_REPLACE_WORDS) } else { None };
    let discards = if matches!(kind, manifest::ActionKind::Mutation | manifest::ActionKind::Shell) { matching_lexicon_word(verb_id, DESTRUCTIVE_VERB_WORDS) } else { None };
    if let Some(matched) = replaces_document.or(discards) {
        findings.push(CatalogAuditFinding::UnmarkedDestructiveVerb { capability_id, matched });
        return;
    }
    if matches!(kind, manifest::ActionKind::Shell | manifest::ActionKind::View) {
        if let Some(matched) = matching_lexicon_word(verb_id, USER_PATH_WRITE_WORDS) {
            findings.push(CatalogAuditFinding::UnmarkedUserPathWrite { capability_id, matched });
        }
    }
}

/// 🚨️ Every [`CatalogAuditFinding`] in a `CatalogSource`, sorted by capability id. Runs over the
/// SOURCE rather than a compiled `Catalog` because the one fact it needs — whether an audience was
/// DECLARED or merely derived — exists only in `ActionSemantics.audience: Option<…>`; `compile`
/// resolves that Option away. Empty means: every gesture-named route published to an agent was
/// looked at by a human, and every delete/clear/replace verb an agent can reach asks first.
pub fn audit_source(source: &CatalogSource) -> Vec<CatalogAuditFinding> {
    let mut findings: Vec<CatalogAuditFinding> = Vec::new();
    for descriptor in &source.descriptors {
        let plugin_id = descriptor.manifest.plugin_id.as_str();
        for app in descriptor.manifest.apps.iter() {
            let Ok(verbs) = app_action_verbs(app) else { continue };
            for (action, _) in verbs {
                if is_framework_injected_action(action) {
                    continue;
                }
                audit_declaration(
                    format!("{plugin_id}.{}.{}", app.id, action.id),
                    &action.id,
                    manifest::resolve_audience(action),
                    action.semantics.audience.is_some(),
                    action.kind,
                    action.semantics.effects.destructive,
                    &mut findings,
                );
            }
            for command in app.commands.iter().chain(app.modes.iter().flat_map(|mode| mode.commands.iter())) {
                audit_declaration(
                    format!("{plugin_id}.{}.cmd.{}", app.id, command.id),
                    &command.id,
                    manifest::resolve_command_audience(command),
                    command.semantics.audience.is_some(),
                    command.kind,
                    command.semantics.effects.destructive,
                    &mut findings,
                );
            }
        }
        for command in &descriptor.manifest.commands {
            audit_declaration(
                format!("{plugin_id}.cmd.{}", command.id),
                &command.id,
                manifest::resolve_command_audience(command),
                command.semantics.audience.is_some(),
                command.kind,
                command.semantics.effects.destructive,
                &mut findings,
            );
        }
    }
    findings.sort_by(|left, right| left.capability_id().cmp(right.capability_id()));
    findings
}
//#endregion 🔖️Audit

//#region 🔖️DescriptionLaw
/// 📜️ The manifest's schema document — `CapabilityDescription` is declared there, beside the
/// `ActionSemantics.description` field it constrains.
const MANIFEST_SCHEMA: &str = include_str!("../../../../../🔨️modules/🛂️manifest/🧬️schema/🔣️.json");

/// 💬️ One way an agent-published verb fails to explain itself — the `CapabilityDescription` contract
/// (`🛂️manifest/🧬️schema/🔣️.json`), in the order [`description_problems`] reports them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DescriptionProblem {
    /// 🕳️ No `semantics.description` at all.
    Missing,
    /// 📐️ A cell violates `CapabilityDescriptionSentence` (empty, too short or long, not one sentence line).
    Schema,
    /// 🌐️ A German cell is the English cell verbatim.
    Untranslated,
    /// 🔁️ A native cell only repeats the verb's own title in that locale.
    RepeatsTitle,
    /// 👯️ Another verb of the same app carries the same English description.
    SharedWithinApp,
}

impl DescriptionProblem {
    /// 🔤️ The fixture/wire spelling shared with the TypeScript twin.
    pub fn as_str(self) -> &'static str {
        match self {
            DescriptionProblem::Missing => "missing",
            DescriptionProblem::Schema => "schema",
            DescriptionProblem::Untranslated => "untranslated",
            DescriptionProblem::RepeatsTitle => "repeatsTitle",
            DescriptionProblem::SharedWithinApp => "sharedWithinApp",
        }
    }
}

/// 🗣️ One agent-published verb of one app as the description law reads it: the id, the native title
/// per locale, and the declared description in the `LocalizedLabel` wire shape a descriptor carries
/// (`{terminology: {locale: text}}`), `None` when undeclared.
#[derive(Clone, Debug, PartialEq)]
pub struct DescribedVerb {
    pub id: String,
    pub title_en: String,
    pub title_de: String,
    pub description: Option<serde_json::Value>,
}

impl DescribedVerb {
    fn from_declaration(id: String, label: &semio_framework_ui::wgpu::LocalizedLabel, description: Option<&semio_framework_ui::wgpu::LocalizedLabel>) -> Self {
        Self {
            id,
            title_en: label.resolve(Terminology::Native, Locale::En).to_string(),
            title_de: label.resolve(Terminology::Native, Locale::De).to_string(),
            description: description.map(|description| serde_json::to_value(description).expect("a LocalizedLabel always serializes")),
        }
    }
}

/// 🚨️ One `(capability, problem)` row of the description census. Its message line is
/// `<capability id> [<problem>] <reason>` — the `capability-audit-check` gate parses the first two
/// fields to hold the Rust census against the AJV one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescriptionFinding {
    pub capability_id: String,
    pub problem: DescriptionProblem,
}

impl DescriptionFinding {
    pub fn message(&self) -> String {
        let reason = match self.problem {
            DescriptionProblem::Missing => "declares no description — an agent sees a bare title",
            DescriptionProblem::Schema => "has a description cell that is not one en/de sentence line of 24–480 characters",
            DescriptionProblem::Untranslated => "has a German description cell identical to the English one",
            DescriptionProblem::RepeatsTitle => "has a description that only repeats its title",
            DescriptionProblem::SharedWithinApp => "shares its English description with another verb of the same app",
        };
        format!("{} [{}] {reason}", self.capability_id, self.problem.as_str())
    }
}

/// 📐️ `CapabilityDescription` compiled from the manifest schema document by the repo's owned
/// validator — the same `$defs` the TypeScript twin hands to AJV. Only the `CapabilityDescription*`
/// family is taken: the owned validator checks every def it is given, and sibling defs reference
/// other documents this law never needs.
fn description_validator() -> semio_framework_schema::OwnedJsonSchemaValidator {
    let manifest: serde_json::Value = serde_json::from_str(MANIFEST_SCHEMA).expect("the manifest schema is JSON");
    let family: serde_json::Map<String, serde_json::Value> = manifest["$defs"].as_object().expect("the manifest schema declares $defs").iter().filter(|(id, _)| id.starts_with("CapabilityDescription")).map(|(id, def)| (id.clone(), def.clone())).collect();
    let root = serde_json::json!({ "$schema": manifest["$schema"], "$id": manifest["$id"], "$defs": family, "$ref": "#/$defs/CapabilityDescription" });
    crate::schema::compile_validator(&root).expect("CapabilityDescription compiles")
}

fn description_cell<'value>(description: &'value serde_json::Value, terminology: &str, locale: &str) -> &'value str {
    description.get(terminology).and_then(|row| row.get(locale)).and_then(serde_json::Value::as_str).unwrap_or_default().trim()
}

fn repeats_title(cell: &str, title: &str) -> bool {
    !cell.is_empty() && cell.trim_end_matches(['.', '!', '?']).trim().to_lowercase() == title.trim().to_lowercase()
}

/// ⚖️ The `CapabilityDescription` contract over one app's agent-published verbs: per verb, in
/// declaration order, every [`DescriptionProblem`] it has in enum order (`Missing` alone when there
/// is nothing to judge). Replayed by `🧫️fixtures/💬️capability-description.json` against the
/// TypeScript twin `capabilityDescriptionProblems` (AJV), which must report the identical list.
pub fn description_problems(verbs: &[DescribedVerb]) -> Vec<(String, DescriptionProblem)> {
    let validator = description_validator();
    let mut english: BTreeMap<&str, usize> = BTreeMap::new();
    for description in verbs.iter().filter_map(|verb| verb.description.as_ref()) {
        let cell = description_cell(description, "native", "en");
        if !cell.is_empty() {
            *english.entry(cell).or_insert(0) += 1;
        }
    }
    let mut problems = Vec::new();
    for verb in verbs {
        let Some(description) = &verb.description else {
            problems.push((verb.id.clone(), DescriptionProblem::Missing));
            continue;
        };
        if crate::schema::validate(&validator, description).is_err() {
            problems.push((verb.id.clone(), DescriptionProblem::Schema));
        }
        if ["native", "reuse"].iter().any(|terminology| {
            let en = description_cell(description, terminology, "en");
            !en.is_empty() && en == description_cell(description, terminology, "de")
        }) {
            problems.push((verb.id.clone(), DescriptionProblem::Untranslated));
        }
        if repeats_title(description_cell(description, "native", "en"), &verb.title_en) || repeats_title(description_cell(description, "native", "de"), &verb.title_de) {
            problems.push((verb.id.clone(), DescriptionProblem::RepeatsTitle));
        }
        if english.get(description_cell(description, "native", "en")).is_some_and(|count| *count > 1) {
            problems.push((verb.id.clone(), DescriptionProblem::SharedWithinApp));
        }
    }
    problems
}

/// 🧾️ The description census over a `CatalogSource`: [`description_problems`] once per app of every
/// descriptor (its agent-published window-kind, app-scope, app and mode verbs, framework-injected
/// ids excluded), once per plugin-scope command set, and once over the framework-injected verbs every
/// app receives (the framework's own definitions, deduped by id). Empty means every verb an agent
/// can be offered explains itself in English and German. Sorted by capability id.
pub fn description_findings(source: &CatalogSource) -> Vec<DescriptionFinding> {
    let mut findings = Vec::new();
    let mut collect = |prefix: &str, verbs: Vec<DescribedVerb>| {
        for (verb, problem) in description_problems(&verbs) {
            findings.push(DescriptionFinding { capability_id: format!("{prefix}{verb}"), problem });
        }
    };
    let mut framework: BTreeMap<String, DescribedVerb> = BTreeMap::new();
    for descriptor in &source.descriptors {
        let plugin_id = descriptor.manifest.plugin_id.as_str();
        for app in descriptor.manifest.apps.iter() {
            let mut verbs = Vec::new();
            if let Ok(declared) = app_action_verbs(app) {
                for (action, _) in declared {
                    if !is_framework_injected_action(action) && manifest::resolve_audience(action) == manifest::CapabilityAudience::Agent {
                        verbs.push(DescribedVerb::from_declaration(action.id.clone(), &action.label, action.semantics.description.as_ref()));
                    }
                }
            }
            for command in &app.commands {
                if manifest::resolve_command_audience(command) == manifest::CapabilityAudience::Agent {
                    verbs.push(DescribedVerb::from_declaration(format!("cmd.{}", command.id), &command.label, command.semantics.description.as_ref()));
                }
            }
            for mode in app.modes.iter() {
                for command in &mode.commands {
                    if manifest::resolve_command_audience(command) == manifest::CapabilityAudience::Agent {
                        verbs.push(DescribedVerb::from_declaration(format!("mode.{}.{}", mode.id, command.id), &command.label, command.semantics.description.as_ref()));
                    }
                }
            }
            collect(&format!("{plugin_id}.{}.", app.id), verbs);
            for action in framework_action_definitions(app) {
                if manifest::resolve_audience(&action) == manifest::CapabilityAudience::Agent {
                    framework.entry(action.id.clone()).or_insert_with(|| DescribedVerb::from_declaration(action.id.clone(), &action.label, action.semantics.description.as_ref()));
                }
            }
        }
        let commands = descriptor.manifest.commands.iter().filter(|command| manifest::resolve_command_audience(command) == manifest::CapabilityAudience::Agent);
        collect(&format!("{plugin_id}.cmd."), commands.map(|command| DescribedVerb::from_declaration(command.id.clone(), &command.label, command.semantics.description.as_ref())).collect());
    }
    collect("framework.", framework.into_values().collect());
    findings.sort_by(|left, right| left.capability_id.cmp(&right.capability_id).then(left.problem.cmp(&right.problem)));
    findings
}
//#endregion 🔖️DescriptionLaw

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;
//#endregion 🧪️Tests
