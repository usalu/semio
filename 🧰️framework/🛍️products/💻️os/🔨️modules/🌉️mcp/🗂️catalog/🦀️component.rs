//! 🗂️ `CapabilityDefinition` + the catalog compiler — packet `P2-catalog`, `📓️design-decisions.md`
//! D5 (these types live HERE, not in `🛂️manifest`) and D3 (the `<plugin_id>.<app_id>.<action_id>`
//! id grammar is mandatory: a bare action id is never a capability id, since 14 action ids collide
//! across plugins). `compile()` walks real `semio_framework::manifest` types
//! (`PackageDescriptor`/`PluginManifest`/`AppDefinition`) — this facet is the ONLY place in the
//! gateway crate that depends on `semio-framework`; every other facet stays framework-free
//! (`📓️terra-P1a-report.md` §5, D8) so the peer ticket's mid-flight plugin-host rewrite can never
//! break this crate's own build.

use semio_framework::manifest;
use semio_framework::manifest::kernel;
use semio_framework::{Locale, Terminology};
use std::collections::BTreeMap;

//#region 🔖️CapabilityRef
/// 🪪️ A capability's full identity string — `<plugin_id>.<app_id>.<action_id>`, `framework.*`,
/// `os.*`, `ui.*`, or a bare gateway verb (`context.resolve`, `capabilities.search`, …). Never a
/// bare action id (D3).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
#[serde(transparent)]
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
            CapabilityOwner::Plugin { plugin_id, app_id, window_kind_id, mode_id } => {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
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

//#region 🔖️ToolExposure
/// 🔌️ Whether a capability is invocable only through `action.invoke`/the deterministic catalog
/// (`CatalogOnly`, the common case) or ALSO published as its own named `tools/list` entry
/// (`Direct` — the small stable core set P2 registers, plus whatever a later packet promotes).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ToolExposure {
    CatalogOnly,
    Direct { tool_name: String },
}
//#endregion 🔖️ToolExposure

//#region 🔖️CapabilityPresentation
/// 📝️ One argument's search/display-facing summary — deliberately not the full `manifest::ActionArgDef`
/// (whose `schema`/`presentation` already live in `input_schema` below; repeating them here would be
/// duplicate state).
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityArgSummary {
    pub id: String,
    pub label: String,
    pub required: bool,
}

/// 🎨️ UI-shaped presentation hints carried through from the source `ActionDefinition`/
/// `CommandDefinition` — everything a renderer needs to draw a palette row without re-deriving it
/// from `manifest` types the gateway crate otherwise never exposes on the wire.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityExample {
    pub request: String,
    pub input: serde_json::Value,
}
//#endregion 🔖️CapabilityExample

//#region 🔖️CapabilitySource
/// 🧵️ Where a compiled capability came from — lets a debugging tool or `catalog lint` walk back to
/// the exact manifest declaration without re-deriving it from the id string.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityDefinition {
    pub id: CapabilityRef,
    pub version: u32,
    pub owner: CapabilityOwner,
    pub kind: CapabilityKind,
    pub title: String,
    pub description: String,
    pub artifact_kind: Option<String>,
    pub use_when: Vec<String>,
    pub input_schema: serde_json::Value,
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
/// 📐️ JSON Schema 2020-12 envelope wrapping one action/command's declared args as `properties` —
/// `📋️master.md` §3.2 step 1: `{type:"object", properties, required, additionalProperties:false,
/// $schema, $id: semio://capability/<id>/input}`. Leaf schemas come from `ActionArgDef::json_schema()`
/// (landed by P3, `🛂️manifest/🦀️component.rs` `🔖️ActionArgs`).
fn action_input_schema(capability_id: &str, args: &[manifest::ActionArgDef]) -> serde_json::Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();
    for arg in args {
        properties.insert(arg.id.clone(), arg.json_schema());
        if arg.required {
            required.push(serde_json::Value::String(arg.id.clone()));
        }
    }
    let mut schema = serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": format!("semio://capability/{capability_id}/input"),
        "type": "object",
        "properties": properties,
        "additionalProperties": false,
    });
    if !required.is_empty() {
        schema.as_object_mut().expect("object schema").insert("required".into(), serde_json::Value::Array(required));
    }
    schema
}

/// 📐️ A permissive JSON Schema 2020-12 output envelope — no `manifest::ActionDefinition` carries a
/// typed output shape yet (the bridge's `AppFrame::Emit`/`DispatchReport` payload is dynamic), so
/// every capability's `output_schema` is `{type:"object"}` tagged with its own `$id` until a later
/// packet (P6+) types individual results.
fn generic_output_schema(capability_id: &str) -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": format!("semio://capability/{capability_id}/output"),
        "type": "object",
    })
}
//#endregion 🔖️SchemaBuilders

//#region 🔖️FrameworkDedup
/// 🕹️ The 5 framework-injected action ids that carry `ActionKind::View`/`Shell` (not
/// `History`/`Clipboard`/`Interaction`, which are already unambiguous by kind) — see
/// `🛂️manifest/🦀️component.rs` `history_action_definitions`/`clipboard_action_definitions`/
/// `set_active_utility_action_definition`/etc.'s own doc comments for the full auto-injection story.
const FRAMEWORK_VIEW_SHELL_ACTION_IDS: [&str; 5] = ["setActiveUtility", "setActiveTool", "startIntroduction", "setHistoryCommandFilter", "noteShellCommand"];

/// 🕹️ Whether `action` is one of the 21 framework-auto-injected ids (`📓️luna-actions-audit.md` §1) —
/// these are walked ONCE per distinct id across every app (see `framework_capabilities`) rather than
/// once per plugin/app declaration, so they compile into `framework.*`, never `<plugin>.<app>.*`.
fn is_framework_injected_action(action: &manifest::ActionDefinition) -> bool {
    matches!(action.kind, manifest::ActionKind::History | manifest::ActionKind::Clipboard | manifest::ActionKind::Interaction) || FRAMEWORK_VIEW_SHELL_ACTION_IDS.contains(&action.id.as_str())
}

/// 🕹️ Every framework-injected action reachable from `apps`, deduped by id (first occurrence wins —
/// every app resolves the identical `ActionDefinition` for a given framework id, since none of these
/// take app-specific data into their manifest shape) — compiled into `framework.<action.id>`
/// capabilities owned by `CapabilityOwner::Framework`.
fn framework_capabilities(apps: &[&manifest::AppDefinition], locale: Locale, terminology: Terminology) -> BTreeMap<String, CapabilityDefinition> {
    let mut map = BTreeMap::new();
    for app in apps {
        let mut actions = manifest::history_action_definitions();
        actions.extend(manifest::clipboard_action_definitions());
        actions.extend(manifest::interaction_action_definitions(app));
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
        for action in actions {
            let id = format!("framework.{}", action.id);
            map.entry(id.clone()).or_insert_with(|| capability_from_action(&id, CapabilityOwner::Framework, None, &action, CapabilitySource::Action { plugin_id: "framework".into(), app_id: "framework".into(), window_kind_id: "*".into(), action_id: action.id.clone() }, locale, terminology));
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
        title,
        description,
        artifact_kind,
        use_when: action.semantics.use_when.clone(),
        input_schema: action_input_schema(id, &action.args),
        output_schema: generic_output_schema(id),
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
        title,
        description,
        artifact_kind,
        use_when: command.semantics.use_when.clone(),
        input_schema: action_input_schema(id, &command.args),
        output_schema: generic_output_schema(id),
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

/// 🏭️ One `DescriptorEntry` (an untyped contribution row — `manifest::DescriptorEntry` has no label
/// of its own, per its doc comment "don't have a typed manifest model of their own yet") →
/// `Query`/`Job` capability. `title` is the entry id, humanized; `description` names the source
/// category so a search hit is at least traceable back to its plugin contribution.
fn capability_from_contribution(plugin_id: &str, category: &str, entry: &manifest::DescriptorEntry, kind: CapabilityKind) -> CapabilityDefinition {
    let id = format!("{plugin_id}.{category}.{}", entry.id);
    CapabilityDefinition {
        id: CapabilityRef(id.clone()),
        version: 1,
        owner: CapabilityOwner::Plugin { plugin_id: plugin_id.to_string(), app_id: None, window_kind_id: None, mode_id: None },
        kind,
        title: humanize(&entry.id),
        description: format!("{category} contribution from {plugin_id}"),
        artifact_kind: None,
        use_when: Vec::new(),
        input_schema: serde_json::json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "$id": format!("semio://capability/{id}/input"), "type": "object" }),
        output_schema: generic_output_schema(&id),
        effects: manifest::CapabilityEffects::default(),
        policy: manifest::CapabilityPolicy::default(),
        execution: manifest::CapabilityExecution { class: manifest::ExecutionClass::Job, ..Default::default() },
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation { icon_id: None, category: Some(category.to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Descriptor { category: category.to_string(), id: entry.id.clone() },
    }
}

/// 🏭️ `CommandDefinition`→capability for one real `💻️os/🎮️commands/*` module — `owner: Os`, `source:
/// OsCommand`. The `id` is already fully qualified (`os.<slug>`, see every `pub const ID` in
/// `💻️os/🎮️commands/*/🦀️component.rs`) — used as-is, never re-prefixed.
fn capability_from_os_command(command: &manifest::CommandDefinition, locale: Locale, terminology: Terminology) -> CapabilityDefinition {
    capability_from_command(&command.id.clone(), CapabilityOwner::Os, None, command, CapabilitySource::OsCommand { id: command.id.clone() }, locale, terminology)
}

/// 🔤️ `"open-artifact"`/`"myThing"` → `"Open Artifact"`/`"My Thing"` — a last-resort title for
/// untyped `DescriptorEntry` rows that have no `LocalizedLabel` to resolve.
fn humanize(id: &str) -> String {
    let spaced: String = id.chars().flat_map(|character| if character.is_uppercase() { vec![' ', character] } else { vec![character] }).collect();
    spaced.replace(['-', '_'], " ").split_whitespace().map(|word| {
        let mut chars = word.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }).collect::<Vec<_>>().join(" ")
}
//#endregion 🔖️CapabilityBuilders

//#region 🔖️UiDialogAndArtifactCreate
/// 🗨️ `📋️master.md` §3.2: "`dialogs` → `ui.dialog.open`" — every app's declared `DialogDefinition`s
/// fold into ONE gateway-owned `ui.dialog.open` capability (never one capability per dialog), whose
/// `dialogId` argument enumerates every dialog id collected across every walked app.
fn ui_dialog_open_capability(dialog_ids: &[String]) -> CapabilityDefinition {
    let options: Vec<serde_json::Value> = dialog_ids.iter().map(|id| serde_json::Value::String(id.clone())).collect();
    let input_schema = serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/ui.dialog.open/input",
        "type": "object",
        "properties": { "dialogId": { "type": "string", "enum": options }, "args": { "type": "object" } },
        "required": ["dialogId"],
        "additionalProperties": false,
    });
    CapabilityDefinition {
        id: CapabilityRef("ui.dialog.open".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Ui,
        title: "Open Dialog".to_string(),
        description: "Opens one of the workspace's declared modal form dialogs by id.".to_string(),
        artifact_kind: None,
        use_when: vec!["open a dialog".to_string(), "show a form".to_string()],
        input_schema,
        output_schema: generic_output_schema("ui.dialog.open"),
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
    let options: Vec<serde_json::Value> = template_ids.iter().map(|id| serde_json::Value::String(id.clone())).collect();
    let input_schema = serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/artifact.create/input",
        "type": "object",
        "properties": { "kind": { "type": "string" }, "template": { "type": "string", "enum": options } },
        "required": ["kind"],
        "additionalProperties": false,
    });
    CapabilityDefinition {
        id: CapabilityRef("artifact.create".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Job,
        title: "Create Artifact".to_string(),
        description: "Creates a new artifact, optionally seeded from a declared playground example/template.".to_string(),
        artifact_kind: None,
        use_when: vec!["create a new artifact".to_string(), "start a new document".to_string()],
        input_schema,
        output_schema: generic_output_schema("artifact.create"),
        effects: manifest::CapabilityEffects { writes: vec![manifest::ResourceSelector::new("artifact:{self}")], reversible: false, ..Default::default() },
        policy: manifest::CapabilityPolicy { scopes: vec![kernel::CapabilityId("documents.write".into())], ..Default::default() },
        execution: manifest::CapabilityExecution { class: manifest::ExecutionClass::Job, ..Default::default() },
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
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
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
    blake3::hash(&bytes).to_hex().to_string()
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
    let mut entries: BTreeMap<String, CapabilityDefinition> = BTreeMap::new();
    let mut all_apps: Vec<&manifest::AppDefinition> = Vec::new();
    let mut dialog_ids: Vec<String> = Vec::new();
    let mut template_ids: Vec<String> = Vec::new();

    for descriptor in &source.descriptors {
        let plugin_id = descriptor.manifest.plugin_id.clone();

        for app in descriptor.manifest.apps.iter() {
            let app_id = app.id.clone();
            let artifact_kind = app.dialect.artifact_kind.clone();

            for window_kind in app.window_kinds.iter() {
                for action in &window_kind.actions {
                    if is_framework_injected_action(action) {
                        continue;
                    }
                    let id = format!("{plugin_id}.{app_id}.{}", action.id);
                    let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), app_id: Some(app_id.clone()), window_kind_id: Some(window_kind.id.clone()), mode_id: None };
                    let source_ref = CapabilitySource::Action { plugin_id: plugin_id.clone(), app_id: app_id.clone(), window_kind_id: window_kind.id.clone(), action_id: action.id.clone() };
                    insert_capability(&mut entries, capability_from_action(&id, owner, Some(artifact_kind.clone()), action, source_ref, locale, terminology))?;
                }
            }

            for command in &app.commands {
                let id = format!("{plugin_id}.{app_id}.cmd.{}", command.id);
                let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), app_id: Some(app_id.clone()), window_kind_id: None, mode_id: None };
                let source_ref = CapabilitySource::Command { plugin_id: Some(plugin_id.clone()), app_id: Some(app_id.clone()), mode_id: None, command_id: command.id.clone() };
                insert_capability(&mut entries, capability_from_command(&id, owner, Some(artifact_kind.clone()), command, source_ref, locale, terminology))?;
            }

            for mode in app.modes.iter() {
                for command in &mode.commands {
                    let id = format!("{plugin_id}.{app_id}.mode.{}.{}", mode.id, command.id);
                    let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), app_id: Some(app_id.clone()), window_kind_id: None, mode_id: Some(mode.id.clone()) };
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
            let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), app_id: None, window_kind_id: None, mode_id: None };
            let source_ref = CapabilitySource::Command { plugin_id: Some(plugin_id.clone()), app_id: None, mode_id: None, command_id: command.id.clone() };
            insert_capability(&mut entries, capability_from_command(&id, owner, None, command, source_ref, locale, terminology))?;
        }

        for example in &descriptor.manifest.examples {
            template_ids.push(format!("{plugin_id}:{}", example.id));
        }

        for entry in &descriptor.contributions.inference_services {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, "infer", entry, CapabilityKind::Query))?;
        }
        for entry in &descriptor.contributions.mutation_services {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, "mutate", entry, CapabilityKind::Job))?;
        }
        for entry in &descriptor.contributions.io_entries {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, "io", entry, CapabilityKind::Job))?;
        }
        for entry in &descriptor.contributions.composer_entries {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, "compose", entry, CapabilityKind::Job))?;
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
    if !template_ids.is_empty() {
        template_ids.sort();
        template_ids.dedup();
        insert_capability(&mut entries, artifact_create_capability(&template_ids))?;
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

    let sorted: Vec<CapabilityDefinition> = entries.into_values().collect();
    let hash = compute_catalog_entries_hash(&sorted);
    Ok(Catalog { hash, entries: sorted })
}
//#endregion 🔖️Compile

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use crate::fixtures;

    #[test]
    fn compiling_the_same_source_twice_is_byte_identical() {
        let source = fixtures::note_and_cad_source();
        let first = compile(&source, Locale::En, Terminology::Native).expect("compiles");
        let second = compile(&source, Locale::En, Terminology::Native).expect("compiles");
        assert_eq!(first.hash, second.hash);
        assert_eq!(first.entries, second.entries);
    }

    #[test]
    fn entries_are_sorted_by_id() {
        let source = fixtures::note_and_cad_source();
        let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
        let ids: Vec<&str> = catalog.entries.iter().map(|entry| entry.id.as_str()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted);
    }

    /// 🆔️ D3: two plugins declaring the SAME bare action id must compile into two distinct
    /// capability ids — a bare action id is never a capability id.
    #[test]
    fn two_plugins_declaring_the_same_action_id_compile_to_distinct_capability_ids() {
        let source = fixtures::colliding_action_id_source();
        let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles distinct ids without error");
        assert!(catalog.get("plugin-a.surface.deleteSelection").is_some());
        assert!(catalog.get("plugin-b.surface.deleteSelection").is_some());
        assert_ne!(catalog.get("plugin-a.surface.deleteSelection").unwrap().id, catalog.get("plugin-b.surface.deleteSelection").unwrap().id);
    }

    #[test]
    fn cad_translate_selection_compiles_with_the_dxyz_input_schema() {
        let source = fixtures::note_and_cad_source();
        let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
        let capability = catalog.get("cad.editor.translateSelection").expect("translateSelection present");
        assert_eq!(capability.kind, CapabilityKind::Mutation);
        let properties = capability.input_schema["properties"].as_object().expect("object schema");
        assert!(properties.contains_key("dx"));
        assert!(properties.contains_key("dy"));
        assert!(properties.contains_key("dz"));
        assert!(properties.contains_key("objectIds"));
    }

    #[test]
    fn framework_actions_dedupe_into_one_entry_per_id_across_both_apps() {
        let source = fixtures::note_and_cad_source();
        let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
        let framework_undo_count = catalog.entries.iter().filter(|entry| entry.id.as_str() == "framework.undo").count();
        assert_eq!(framework_undo_count, 1);
        assert!(matches!(catalog.get("framework.undo").unwrap().owner, CapabilityOwner::Framework));
    }

    #[test]
    fn duplicate_capability_id_is_rejected() {
        let mut source = fixtures::note_and_cad_source();
        let duplicate = source.gateway.first().cloned();
        if let Some(capability) = duplicate {
            source.gateway.push(capability);
        } else {
            source.gateway.push(ui_dialog_open_capability(&[]));
            source.gateway.push(ui_dialog_open_capability(&[]));
        }
        let result = compile(&source, Locale::En, Terminology::Native);
        assert!(matches!(result, Err(CatalogError::DuplicateCapabilityId(_))));
    }
}
//#endregion 🧪️Tests
