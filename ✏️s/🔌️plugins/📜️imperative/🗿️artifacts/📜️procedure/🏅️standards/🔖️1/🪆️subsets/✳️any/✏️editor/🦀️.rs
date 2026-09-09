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
pub use document_panel::IMPERATIVE_PLAY_BODY_DOCUMENT;
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
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::procedure::commands::run;
use crate::editor::procedure::commands::set_contributions;
use crate::editor::procedure::commands::{add_step, add_step_at, move_step, move_step_at, remove_step, remove_step_at, set_step_params, set_step_params_at};
//#endregion 🔖️Commands

//#region 🔖️ImperativePlayApp
/// 🧪️ B1: unit struct — the former `ImperativePlayRuntime`/`self.runtime` field now lives in
/// `ImperativeConfig` (see `ArtifactEditor::Config`), written via `ImperativeConfigMutation`s.
#[derive(Default)]
pub struct ImperativePlayApp;

impl ArtifactEditor for ImperativePlayApp {
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
    const DOCUMENT_SCHEMA: &'static str = PROCEDURE_DOCUMENT_SCHEMA;

    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::procedure::config::schema::app_schema_descriptor())
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
    /// `computation.procedure`-kinded output); `"document:out"` replicates `ArtifactEditor::export_media`'s
    /// default whole-document-pack behavior (unreachable once this override exists).
    fn export_media(port: &str, doc: &ArtifactView<'_, ProcedureSnapshot>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let host = crate::editor::procedure::engine::ImperativeHost::from_snapshot(doc.snapshot.clone());
                let result = host.run();
                let json = dsl::os_pack::json::to_json_string(&result.scope);
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.procedure".into(), json } })
            }
            "document:out" => {
                let media_type = imperative_io().document_media_type;
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
            IMPERATIVE_PLAY_BODY_DOCUMENT => document_panel::render(document, labels),
            IMPERATIVE_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
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
            .mutation("removeStepAt", LocalizedLabel::native("Remove Step At", "Schritt bei Position entfernen"))
            .mutation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .mutation("moveStepAt", LocalizedLabel::native("Move Step At", "Schritt bei Position verschieben"))
            .mutation("setStepParams", LocalizedLabel::native("Set Step Params", "Schrittparameter festlegen"))
            .mutation("setStepParamsAt", LocalizedLabel::native("Set Step Params At", "Schrittparameter bei Position festlegen"))
            // 👁️ Ephemeral view state / runtime effect — `run` evaluates into config. Step selection/
            // hover are no longer declared here: framework-owned, injected via `.interaction(...)` below.
            .action_with(semio_framework_plugin::ActionDefinition::new("run", LocalizedLabel::native("Run", "Ausführen"), ActionKind::View, "play"))
            .action_interactive_job("setContributions", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addStep", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addStepAt", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("removeStep", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("removeStepAt", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("moveStep", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("moveStepAt", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setStepParams", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setStepParamsAt", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("run", InteractiveJobClassification::BatchOnlyPendingRewrite)
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
                granularities: vec![GranularityDefinition { id: "step".into(), label: LocalizedLabel::native("Step", "Schritt"), icon_id: "square".into() }],
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
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
