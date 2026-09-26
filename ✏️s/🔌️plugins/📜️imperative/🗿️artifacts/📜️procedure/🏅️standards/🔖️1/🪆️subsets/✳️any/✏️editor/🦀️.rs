//! 🖥️ Imperative play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`. This file is a routing table: `handle` →
//! `ImperativeCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::editor::procedure::engine::imperative_io;
use crate::editor::procedure::modes::edit;
use crate::editor::procedure::modes::edit::windows::{main, script};
use crate::editor::procedure::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::procedure::terminology::imperative_labels;
use crate::mutations::ProcedureMutation;
use crate::schema::default_snapshot;
use crate::{ProcedureSnapshot, Step, PROCEDURE_DOCUMENT_SCHEMA};
use semio_framework::InteractiveJobClassification;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionKind, ArtifactEditor, ArtifactView, CommandDefinition, ComponentTree, ConfigView, DomainTopology, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition,
    InteractionTopology, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode,
};
// 🚧️ Dialect/StandardId/SubsetId are not yet in the crate-root re-export list (w0-f gap 1 closed
// ArtifactEditor/Editor/etc but left these three under `app::`, already reachable via
// `semio_framework::*` elsewhere) — see `crate::PROCEDURE_DIALECT`'s own
// definition for the qualified form this file only reads back through that constant.
use store::{ArtifactPack, EngineHandles};

/// 🏷️ Admits semantic labels for the imperative editor.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("imperative.ui.capacity", "label admission failed"))
}

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_APP_ID: &str = "s.imperative.procedure@1/*#editor";
pub use catalogue_panel::IMPERATIVE_PLAY_BODY_CATALOGUE;
pub use document_panel::IMPERATIVE_PLAY_BODY_ARTIFACT;
pub use inspection_panel::IMPERATIVE_PLAY_BODY_INSPECTOR;
pub use main::{IMPERATIVE_PLAY_BODY_MAIN, IMPERATIVE_PLAY_WINDOW_MAIN};
pub use script::IMPERATIVE_PLAY_BODY_SCRIPT;

//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ "steps" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: `HierarchyProvider::Topology` over the document's own `Step::bodies` nesting
/// (`control.if`/`control.while` control-flow blocks), transitive (selecting/hovering a control step
/// covers the steps nested in its bodies).
pub const IMPERATIVE_INTERACTION_STEPS: &str = "steps";

/// 🕹️ The only granularity the "steps" domain declares — stamped on every pick row of the document
/// tree so the host synthesizes `interactionSelect` without a per-row argument map.
pub const IMPERATIVE_INTERACTION_GRANULARITY: &str = "step";

/// 🌳️ `steps` domain topology from the document's own `Step::bodies` nesting — row-id-prefixed ids
/// (matching the document panel tree's own item ids, see `document_panel::step_row_id`'s doc comment),
/// so `validate_state` prunes deleted steps and range/transitive selection walk the real control-flow
/// tree, including steps nested inside `control.if`/`control.while` bodies.
fn imperative_steps_topology(document: &ProcedureSnapshot) -> DomainTopology {
    fn visit(steps: &[Step], parent: Option<&str>, out: &mut Vec<TopologyNode>) {
        for step in steps {
            let id = document_panel::step_row_id(&step.id);
            out.push(TopologyNode { id: id.clone(), granularity: "step".into(), parent: parent.map(str::to_string) });
            for body in step.bodies.values() {
                visit(&body.steps, Some(id.as_str()), out);
            }
        }
    }
    let path = crate::procedure_working_scene(document).path;
    let mut ordered = Vec::new();
    visit(&path.steps, None, &mut ordered);
    DomainTopology { ordered }
}
//#endregion 🔖️Interaction

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `ImperativePlayApp::Command` — the SOLE dispatch surface for imperative's own behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// proves they are different vocabularies. **Row order is the binary variant ordinal: appending is
    /// safe, reordering is a wire-format break.**
    pub enum ImperativeCommand for ProcedureSnapshot, ProcedureMutation, ImperativeConfig, ImperativeConfigMutation {
        "addStep" as "add-step" => add_step::AddStep,
        "addStepAt" as "add-step-at" => add_step_at::AddStepAt,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "removeStepAt" as "remove-step-at" => remove_step_at::RemoveStepAt,
        "moveStep" as "move-step" => move_step::MoveStep,
        "moveStepAt" as "move-step-at" => move_step_at::MoveStepAt,
        "setStepParams" as "set-step-params" => set_step_params::SetStepParams,
        "setStepParamsAt" as "set-step-params-at" => set_step_params_at::SetStepParamsAt,
        "run" as "run" => run::Run,
        "setContributions" as "contributions" => set_contributions::SetContributions,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::procedure::commands::run;
use crate::editor::procedure::commands::set_active_example;
use crate::editor::procedure::commands::set_contributions;
use crate::editor::procedure::commands::{add_step, add_step_at, move_step, move_step_at, remove_step, remove_step_at, set_step_params, set_step_params_at};
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
/// 🧵️ Every verb the shell may dispatch is a retained tool. `validate_ui_dispatch_classification`
/// refuses anything not classified `Migrated`, and `Migrated` only survives the guest's
/// `interactive-job.catalog-incomplete` boot check when this list, the publication contracts, the
/// extent function and the `bounded_first_step_tool_proofs!` block below all name the same ids against
/// ONE registered factory type. All ten verbs were `BatchOnlyPendingRewrite`, so every row of this
/// app's Actions pane was refused at the dispatch gate (ticket 26/09/18 slice B2c).
const IMPERATIVE_RETAINED_TOOL_IDS: &[&str] =
    &["addStep", "addStepAt", "removeStep", "removeStepAt", "moveStep", "moveStepAt", "setStepParams", "setStepParamsAt", "run", "setContributions", "setActiveExample"];
const IMPERATIVE_RETAINED_PAYLOAD_SCHEMA: &str = "imperative.procedure.tool-command.v1";
const IMPERATIVE_RETAINED_RAW_BYTES: usize = 8_192;
const IMPERATIVE_RETAINED_WORK_ITEMS: usize = 64;
/// 🎒️ Real bound for one Artifact-lane edit: a single step insert/remove/reorder/params leaf.
const IMPERATIVE_STORE_MAXIMUM_BYTES: usize = 65_536;

/// 🚦️ Per-tool publication lanes, read straight off the command bodies: the eight structural step
/// verbs emit `artifact_mutations` only, while `run` and `setContributions` write the config store.
const IMPERATIVE_RETAINED_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStepAt", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeStepAt", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveStepAt", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStepParams", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStepParamsAt", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "run", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
];

fn imperative_retained_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(IMPERATIVE_RETAINED_RAW_BYTES, 64, IMPERATIVE_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

/// 📏️ Every imperative command is ONE bounded step: none of the ten walks a collection incrementally.
fn imperative_retained_extent(command: &ImperativeCommand, _snapshot: &ProcedureSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        ImperativeCommand::SetContributions(payload) => payload.json.len(),
        ImperativeCommand::SetActiveExample(payload) => payload.example_id.len(),
        _ => 0,
    };
    (bytes <= IMPERATIVE_RETAINED_RAW_BYTES && IMPERATIVE_RETAINED_TOOL_IDS.contains(&command.command_id())).then_some(1)
}

/// 🧮️ One shell-supplied argument as this app's own `ValueDsl` scalar — `SetStepParams::params` is a
/// `BTreeMap<String, ValueDsl>` whose fields are private to `crate::document_dsl`, so the conversion
/// goes through the engine `Value` the module already converts from.
fn imperative_value_dsl(value: &dsl::DslValue) -> crate::document_dsl::ValueDsl {
    let engine = match value {
        dsl::DslValue::Bool(flag) => neural_engine::Value::Atom(neural_engine::Atom::Boolean(*flag)),
        dsl::DslValue::Number(number) => match number.as_i64() {
            Some(integer) => neural_engine::Value::Atom(neural_engine::Atom::Integer(integer)),
            None => neural_engine::Value::Atom(neural_engine::Atom::Decimal(number.as_f64())),
        },
        dsl::DslValue::String(text) => neural_engine::Value::Atom(neural_engine::Atom::String(text.clone())),
        dsl::DslValue::Null => neural_engine::Value::Atom(neural_engine::Atom::Null),
        other => neural_engine::Value::Atom(neural_engine::Atom::String(dsl::json::to_json_string(other))),
    };
    crate::document_dsl::value_to_value_dsl(&engine)
}

/// 🌉️ Resolves the React/wgpu shells' `{action, args}` pair into the typed `ImperativeCommand` every
/// dispatch path already speaks. `ArtifactEditor::command_from_action`'s default refuses EVERY id
/// (`app.command.unsupported`), so without this bridge no Actions-pane row could reach
/// `ImperativeCommand::dispatch`.
fn imperative_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<ImperativeCommand, Fault> {
    let entries: &[(String, dsl::DslValue)] = match args {
        Some(dsl::DslValue::Object(object)) => object.as_slice(),
        _ => &[],
    };
    let lookup = |keys: &[&str]| keys.iter().find_map(|key| entries.iter().find(|(name, _)| name == key).map(|(_, value)| value));
    let text = |keys: &[&str], fallback: &str| match lookup(keys) {
        Some(dsl::DslValue::String(raw)) if !raw.is_empty() => raw.clone(),
        Some(dsl::DslValue::String(_)) | None => fallback.to_string(),
        Some(other) => dsl::json::to_json_string(other),
    };
    let optional_text = |keys: &[&str]| match lookup(keys) {
        Some(dsl::DslValue::String(raw)) if !raw.is_empty() => Some(raw.clone()),
        _ => None,
    };
    let index = |keys: &[&str]| match lookup(keys) {
        Some(dsl::DslValue::Number(value)) => value.as_f64().max(0.0) as usize,
        Some(dsl::DslValue::String(raw)) => raw.trim().parse::<usize>().unwrap_or_default(),
        _ => 0,
    };
    let optional_index = |keys: &[&str]| match lookup(keys) {
        Some(dsl::DslValue::Number(value)) => Some(value.as_f64().max(0.0) as usize),
        Some(dsl::DslValue::String(raw)) => raw.trim().parse::<usize>().ok(),
        _ => None,
    };
    let params = |keys: &[&str]| match lookup(keys) {
        Some(dsl::DslValue::Object(object)) => object.iter().map(|(name, value)| (name.clone(), imperative_value_dsl(value))).collect(),
        _ => std::collections::BTreeMap::new(),
    };
    match action {
        "addStep" => Ok(ImperativeCommand::AddStep(add_step::AddStep { kind: text(&["kind", "value"], "log.print"), index: optional_index(&["index"]) })),
        "addStepAt" => Ok(ImperativeCommand::AddStepAt(add_step_at::AddStepAt {
            kind: text(&["kind", "value"], "log.print"),
            index: optional_index(&["index"]),
            owner: optional_text(&["owner", "ownerId", "owner_id"]),
            slot: optional_text(&["slot"]),
        })),
        "removeStep" => Ok(ImperativeCommand::RemoveStep(remove_step::RemoveStep { id: text(&["id", "stepId", "step_id", "value"], "") })),
        "removeStepAt" => Ok(ImperativeCommand::RemoveStepAt(remove_step_at::RemoveStepAt {
            id: text(&["id", "stepId", "step_id", "value"], ""),
            owner: optional_text(&["owner", "ownerId", "owner_id"]),
            slot: optional_text(&["slot"]),
        })),
        "moveStep" => Ok(ImperativeCommand::MoveStep(move_step::MoveStep { id: text(&["id", "stepId", "step_id"], ""), index: index(&["index"]) })),
        "moveStepAt" => Ok(ImperativeCommand::MoveStepAt(move_step_at::MoveStepAt {
            id: text(&["id", "stepId", "step_id"], ""),
            index: index(&["index"]),
            owner: optional_text(&["owner", "ownerId", "owner_id"]),
            slot: optional_text(&["slot"]),
        })),
        "setStepParams" => Ok(ImperativeCommand::SetStepParams(set_step_params::SetStepParams { id: text(&["id", "stepId", "step_id"], ""), params: params(&["params"]) })),
        "setStepParamsAt" => Ok(ImperativeCommand::SetStepParamsAt(set_step_params_at::SetStepParamsAt {
            id: text(&["id", "stepId", "step_id"], ""),
            owner: optional_text(&["owner", "ownerId", "owner_id"]),
            slot: optional_text(&["slot"]),
            params: params(&["params"]),
        })),
        "run" => Ok(ImperativeCommand::Run(run::Run {})),
        "setContributions" => Ok(ImperativeCommand::SetContributions(set_contributions::SetContributions { json: text(&["json", "value"], "{}") })),
        "setActiveExample" => Ok(ImperativeCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text(&["exampleId", "example_id", "id", "value"], crate::examples::demo::ID) })),
        other => Err(Fault::new(
            semio_framework_plugin::FaultOrigin::App,
            semio_framework_plugin::FaultCode::new("imperative.unhandled-action"),
            format!("action '{other}' is not one of this app's declared verbs"),
        )),
    }
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn imperative_retained_reduce(
    command: &ImperativeCommand,
    snapshot: &ProcedureSnapshot,
    config: &ImperativeConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<ImperativePlayApp>>>,
    operation: &semio_framework_plugin::AppOperationContext,
) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None })
}

/// 🏭️ The ONE registered factory serving every retained id — `bounded_first_step_tool_proofs!` binds
/// all of them to a single `factory_type`, and the guest's catalog authority rejects a boot where a
/// registered concrete factory for any id is a different type.
struct ImperativeRetainedCommandJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl ImperativeRetainedCommandJobFactory {
    fn new(controller: &str) -> Self {
        Self { keys: IMPERATIVE_RETAINED_TOOL_IDS.iter().map(|tool| semio_framework::ToolFactoryKey::new(controller, *tool)).collect() }
    }
}

impl semio_framework::ToolJobFactory for ImperativeRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<ImperativePlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<ImperativePlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        IMPERATIVE_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        imperative_retained_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > IMPERATIVE_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((semio_framework::ToolJobFactoryError::new("imperative retained command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for ImperativeRetainedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<ImperativePlayApp>;
    const TOOL_IDS: &'static [&'static str] = IMPERATIVE_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PROCEDURE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = IMPERATIVE_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 🔖️ImperativePlayApp
/// 🧪️ B1: unit struct — the former `ImperativePlayRuntime`/`self.runtime` field now lives in
/// `ImperativeConfig` (see `ArtifactEditor::Config`), written via `ImperativeConfigMutation`s.
#[derive(Default)]
pub struct ImperativePlayApp;

/// 🧬️ The whole-document replacement `setActiveExample` emits. `store::empty_document_spr` (never a
/// minted `create_document_envelope`) is what keeps the guest off the `terminal shell reached Drop
/// before its app-owned bounded retirement authority detached` trap on this path; the framework
/// re-stamps the log with the live mount's identity before hydration sees it
/// (`store::stamp_document_spr_identity`), so no app ever states its own mount.
pub fn reset_procedure_document_effect(document: &ProcedureSnapshot) -> semio_framework_plugin::Effect {
    let pack = <ProcedureSnapshot as ArtifactPack>::encode_pack(document);
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("procedure", PROCEDURE_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}

impl ArtifactEditor for ImperativePlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    /// 🧩️ The roster both composed `s.stdio.semio` children (`flow`, `text`) open through. A
    /// `NoMembers` editor cannot materialise the children `genesis_child_pack` derives, so every
    /// whole-document load fails its archive closure leg before any of them is opened.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = ProcedureSnapshot;
    type Mutation = ProcedureMutation;
    type Config = ImperativeConfig;
    type ConfigMutation = ImperativeConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = ImperativeCommand;

    const DIALECT: semio_framework_plugin::app::Dialect = crate::PROCEDURE_DIALECT;
    /// 🧬️ The crate's one loaded-parent child projection (`crate::procedure_child_restore_projection`).
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
        crate::procedure_child_restore_projection(snapshot)
    }
    const DOCUMENT_SCHEMA: &'static str = PROCEDURE_DOCUMENT_SCHEMA;

    /// 📬️ The ARTIFACT lane's publication authority — a retained tool whose contract states `Artifact`
    /// has nowhere to stage its edit without one, and dies after reaching the typed operation.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("imperative-artifact-retained", IMPERATIVE_STORE_MAXIMUM_BYTES))
    }

    /// 📬️ The CONFIG lane's twin, for `run` and `setContributions`.
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Config, Self::ConfigMutation>("imperative-config-retained", IMPERATIVE_STORE_MAXIMUM_BYTES))
    }

    /// ♻️ The exact store owners and disposers every lane of this editor retires through. Declaring
    /// none answers the first real publication `returned snapshot read requires its exact
    /// owned-snapshot retirement factory`.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    /// 👤️ `EditorApp<E>` forwards these two straight to the editor (only the VIEWER adapter falls
    /// back to the `NoPresence` owners), so an editor that declares neither leaves
    /// `PresenceStore::local_retirement_factory` empty — and every read fails closed with `presence
    /// local read requires a live exact local retirement owner`, every close with `presence close
    /// requires its installed local-root retirement factory`. The step dispatch path reads local
    /// presence, so this editor needs the owners even though its presence is `NoPresence`
    /// (`🕸️dag` and `🪐️space`'s Home declare the same pair).
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<ImperativePlayApp>,
        owner_file: "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.imperative.procedure@1/*#editor",
        artifact_schema: "procedure.document/v1",
        factory: "ImperativeRetainedCommandJobFactory",
        factory_type: ImperativeRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 64, 16_384, 7_500),
        tools: ["addStep", "addStepAt", "removeStep", "removeStepAt", "moveStep", "moveStepAt", "setStepParams", "setStepParamsAt", "run", "setContributions", "setActiveExample"]
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(ImperativeRetainedCommandJobFactory::new(&controller))
    }

    /// 🌉️ The `{action, args}` bridge every shell dispatch arrives as.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        imperative_command_from_action(action, args)
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !IMPERATIVE_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || imperative_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::from("imperative-retained-command-tool-mismatch-or-capacity"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> =
            Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, imperative_retained_reduce, imperative_retained_extent));
        let operation = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            ImperativeCommand::command_id,
            IMPERATIVE_RETAINED_RAW_BYTES,
            IMPERATIVE_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::procedure::config::schema::app_schema_descriptor())
    }

    /// 🏗️ Admits the whole-document replacement `reset_procedure_document_effect` emits for every
    /// example switch. The trait default refuses the envelope, so the host answers every
    /// `setActiveExample` with `artifact-store.persisted-initializer-refused` at the archive-load
    /// boundary.
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, PROCEDURE_DOCUMENT_SCHEMA, operation, generation))
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
        crate::genesis_procedure_child_pack(snapshot, slot, child_id)
    }

    fn initial_snapshot() -> ProcedureSnapshot {
        default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(imperative_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &ImperativeCommand) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &ImperativeCommand,
        doc: &ArtifactView<'_, ProcedureSnapshot>,
        cfg: &ConfigView<'_, ImperativeConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ `steps` domain: `HierarchyProvider::Topology` from the document's own `Step::bodies` nesting —
    /// see `imperative_steps_topology`'s doc comment.
    fn interaction_topology(doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> InteractionTopology {
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(IMPERATIVE_INTERACTION_STEPS.to_string(), imperative_steps_topology(doc.snapshot));
        InteractionTopology { domains }
    }

    /// 🎞️ `"result:out"` exports the last `run` scope (a generic data value, the port recipe's
    /// `computation.procedure`-kinded output); `"artifact:out"` replicates `ArtifactEditor::export_media`'s
    /// default whole-document-pack behavior (unreachable once this override exists).
    fn export_media(port: &str, doc: &ArtifactView<'_, ProcedureSnapshot>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let host = crate::editor::procedure::engine::ImperativeHost::from_snapshot(doc.snapshot.clone());
                let result = host.run();
                let json = dsl::os_pack::json::to_json_string(&result.scope);
                // 🧊️ Same cold boundary as the `run` command: the result owns dictionaries.
                neural_engine::ColdRetire::retire_cold(result);
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.procedure".into(), json } })
            }
            "artifact:out" => {
                let media_type = imperative_io().artifact_media_type;
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, ProcedureSnapshot>, cfg: &ConfigView<'_, ImperativeConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<ComponentTree> {
        imperative_engine::sync_imperative_module_contributions(&cfg.snapshot.contributions_json);
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = imperative_labels(view_state);
        (match body_key {
            IMPERATIVE_PLAY_BODY_MAIN => main::render(document, &config.run_output_json, labels),
            IMPERATIVE_PLAY_BODY_SCRIPT => script::render(document),
            IMPERATIVE_PLAY_BODY_ARTIFACT => document_panel::render(document, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, IMPERATIVE_PLAY_BODY_ARTIFACT)),
            IMPERATIVE_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, IMPERATIVE_PLAY_BODY_CATALOGUE)),
            IMPERATIVE_PLAY_BODY_INSPECTOR => inspection_panel::render(document, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("imperative.ui.capacity", "diagnostic admission failed")),
        })
        .map(semio_framework_plugin::built_to_component_tree)
    }
}
//#endregion 🔖️ImperativePlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_imperative_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::PROCEDURE_DIALECT)
            .command(CommandDefinition {
                args: vec![ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))],
                in_palette: false,
                ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View)
            })
            .document(["semio", "imperative"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("imperative")
            .mode_def(edit::definition())
            .default_mode_id(edit::IMPERATIVE_PLAY_MODE_EDIT)
            .window_kind_def(main::definition())
            .window_kind_def(script::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // 🔧️ Document-mutating step edits — dispatched as VCS operations with a true inverse.
            // The `*At` variants address a nested body via owner/slot fields (drag-and-drop into blocks).
            .mutation("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"))
            .mutation("addStepAt", LocalizedLabel::native("Add Step At", "Schritt bei Position hinzufügen"))
            .mutation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
            .action_destructive("removeStep")
            .mutation("removeStepAt", LocalizedLabel::native("Remove Step At", "Schritt bei Position entfernen"))
            .action_destructive("removeStepAt")
            .mutation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .mutation("moveStepAt", LocalizedLabel::native("Move Step At", "Schritt bei Position verschieben"))
            .mutation("setStepParams", LocalizedLabel::native("Set Step Params", "Schrittparameter festlegen"))
            .mutation("setStepParamsAt", LocalizedLabel::native("Set Step Params At", "Schrittparameter bei Position festlegen"))
            // 👁️ Ephemeral view state / runtime effect — `run` evaluates into config. Step selection/
            // hover are no longer declared here: framework-owned, injected via `.interaction(...)` below.
            .action_with(semio_framework_plugin::ActionDefinition::new("run", LocalizedLabel::native("Run", "Ausführen"), ActionKind::View, "play"))
            // 🧬️ The example picker's verb. The subset registers `crate::examples::demo`, so the shell
            // dispatches this at boot and on every navbar pick; with no declaration at all every one of
            // those was dropped `undeclared-action` before it reached the app.
            .action_with(
                semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Beispiel setzen"), ActionKind::View, "file")
                    .with_args(vec![ActionArgDef::text("exampleId", LocalizedLabel::native("Example", "Beispiel")).default_value(&crate::examples::demo::ID)]),
            )
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setContributions", InteractiveJobClassification::Migrated)
            .action_interactive_job("addStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("addStepAt", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeStepAt", InteractiveJobClassification::Migrated)
            .action_interactive_job("moveStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("moveStepAt", InteractiveJobClassification::Migrated)
            .action_interactive_job("setStepParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setStepParamsAt", InteractiveJobClassification::Migrated)
            .action_interactive_job("run", InteractiveJobClassification::Migrated)
            // 📝️ Staged argument form for the panel-visible create action (the step kind is a choice).
            .action_args("addStep", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("state.set", LocalizedLabel::native("Set State", "Zustand setzen")),
                    ActionArgOption::new("log.print", LocalizedLabel::native("Print Log", "Log ausgeben")),
                    ActionArgOption::new("control.if", LocalizedLabel::native("If", "Wenn")),
                    ActionArgOption::new("control.while", LocalizedLabel::native("While", "Solange")),
                    ActionArgOption::new("math.add", LocalizedLabel::native("Add", "Addieren")),
                ]).default_value(&"log.print"),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `steps` interaction
            // domain — one granularity ("step"), `HierarchyProvider::Topology` from the document's own
            // `Step::bodies` nesting (`imperative_steps_topology`/`ImperativePlayApp::interaction_topology`),
            // both hover and selection transitive (selecting/hovering a control step covers the steps
            // nested in its bodies). Multi-select via Pick (document panel tree rows only — no canvas).
            .interaction(InteractionDefinition {
                id: IMPERATIVE_INTERACTION_STEPS.into(),
                label: LocalizedLabel::native("Steps", "Schritte"),
                granularities: vec![GranularityDefinition { id: IMPERATIVE_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Step", "Schritt"), icon_id: "square".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                    transitive: true,
                    broadcast: true,
                },
            })
            .window_kind_interactions(IMPERATIVE_PLAY_WINDOW_MAIN, vec![IMPERATIVE_INTERACTION_STEPS.into()])
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS /
            // WORKFLOWS-END-TO-END-TYPED-PORTS) — `imperative_io()` is this port information's single
            // source of truth, reused here rather than duplicated.
            .io(imperative_io())
            // 🚧️ SDK GAP (w2-cad-report "SDK gaps found" #4, still open as of w0-f): `EditorBuilder`
            // has no `.example_source(...)`/`.workflow(...)` — `PluginBuilder::editor::<E>` only takes
            // the bare `AppDefinition`, so the demo-session example source and the `"imperative"`
            // workflow registration this app used to chain here are dropped, not ported. The
            // artifact-level `📚️examples/🎬️demo` facet (`crate::examples::demo`,
            // still mounted in `🦀️.rs`) is the surviving example registration path.
            .action_describe("addStep", LocalizedLabel::native("Appends a new step of the given kind (for example a control, math, text or effect step) to the end of the procedure.", "Hängt einen neuen Schritt der angegebenen Art (etwa einen Steuerungs-, Mathe-, Text- oder Effektschritt) an das Ende der Prozedur an."))
            .action_describe("addStepAt", LocalizedLabel::native("Appends a new step of the given kind inside a slot of a control step (such as the then or else branch of an if), or to the procedure's end without owner and slot.", "Hängt einen neuen Schritt der angegebenen Art in einen Slot eines Steuerungsschritts (etwa den Dann- oder Sonst-Zweig eines Wenn) an, ohne Besitzer und Slot an das Ende der Prozedur."))
            .action_describe("removeStep", LocalizedLabel::native("Removes one step by id from the top level of the procedure, including every step nested inside it.", "Entfernt einen Schritt anhand seiner Id aus der obersten Ebene der Prozedur, samt aller darin verschachtelten Schritte."))
            .action_describe("removeStepAt", LocalizedLabel::native("Removes one step by id from a control step's slot, or from the top level, including every step nested inside it.", "Entfernt einen Schritt anhand seiner Id aus dem Slot eines Steuerungsschritts oder der obersten Ebene, samt aller darin verschachtelten Schritte."))
            .action_describe("moveStep", LocalizedLabel::native("Moves one top-level step to a new position (index) in the procedure.", "Verschiebt einen Schritt der obersten Ebene an eine neue Position (Index) in der Prozedur."))
            .action_describe("moveStepAt", LocalizedLabel::native("Moves one step to a new position (index) within its control step's slot or the top level.", "Verschiebt einen Schritt an eine neue Position (Index) innerhalb des Slots seines Steuerungsschritts oder der obersten Ebene."))
            .action_describe("setStepParams", LocalizedLabel::native("Replaces the parameters of one top-level step with the given name-to-value map.", "Ersetzt die Parameter eines Schritts der obersten Ebene durch die angegebene Zuordnung von Namen zu Werten."))
            .action_describe("setStepParamsAt", LocalizedLabel::native("Replaces the parameters of one step inside a control step's slot, or at the top level, with the given name-to-value map.", "Ersetzt die Parameter eines Schritts im Slot eines Steuerungsschritts oder der obersten Ebene durch die angegebene Zuordnung von Namen zu Werten."))
            .action_describe("run", LocalizedLabel::native("Runs the procedure from its first step and shows the resulting variable scope in the output view; the procedure itself is not changed.", "Führt die Prozedur ab dem ersten Schritt aus und zeigt den entstandenen Variablenbereich in der Ausgabeansicht; die Prozedur selbst ändert sich nicht."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole procedure with the bundled demo program, or with an empty procedure for any other example id.", "Ersetzt die gesamte Prozedur durch das mitgelieferte Demoprogramm, bei jeder anderen Beispiel-Id durch eine leere Prozedur."))
            .action_destructive("setActiveExample")
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts
#[path = "👥️presence/🧬️schema/🦀️.rs"]
pub mod schema;
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
