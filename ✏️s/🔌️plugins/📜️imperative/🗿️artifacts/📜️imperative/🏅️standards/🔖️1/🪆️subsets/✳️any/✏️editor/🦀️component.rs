//! 🖥️ Imperative play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`. This file is a routing table: `handle` →
//! `ImperativeCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::editor::imperative::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::editor::imperative::presence::{ImperativePresence, ImperativePresenceMutation};
use crate::editor::imperative::modes::edit;
use crate::editor::imperative::modes::edit::windows::{main, script};
use crate::editor::imperative::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::imperative::terminology::imperative_labels;
use crate::artifacts::imperative::schema::default_snapshot;
use crate::editor::imperative::engine::imperative_io;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::{ImperativeSnapshot, Step, IMPERATIVE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    NoDraft, NoDraftMutation, DraftView, ActionArgDef, ActionArgOption, ActionDescriptor, ActionKind, ArtifactEditor, CommandDefinition, ConfigView, ArtifactView, Editor, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, UiNode,
    GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, MergeMode, SelectionMethod, SelectionMode, SelectionSpec,
    DomainTopology, InteractionTopology, TopologyNode,
};
use semio_framework_plugin::app::InteractionView;
// 🚧️ Dialect/StandardId/SubsetId are not yet in the crate-root re-export list (w0-f gap 1 closed
// ArtifactEditor/Editor/etc but left these three under `app::`, already reachable via
// `semio_framework::*` elsewhere) — see `crate::artifacts::imperative::IMPERATIVE_DIALECT`'s own
// definition for the qualified form this file only reads back through that constant.
use store::EngineHandles;
use serde_json::Value;
use store::ArtifactPack;

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_APP_ID: &str = "imperative-play";
pub use main::{IMPERATIVE_PLAY_BODY_MAIN, IMPERATIVE_PLAY_WINDOW_MAIN};
pub use script::IMPERATIVE_PLAY_BODY_SCRIPT;
pub use catalogue_panel::IMPERATIVE_PLAY_BODY_CATALOGUE;
pub use document_panel::IMPERATIVE_PLAY_BODY_DOCUMENT;
pub use inspection_panel::IMPERATIVE_PLAY_BODY_INSPECTOR;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory `📌️panels/*` builds its item
/// actions with.
pub async fn imperative_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: IMPERATIVE_PLAY_APP_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}
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
async fn imperative_steps_topology(document: &ImperativeSnapshot) -> DomainTopology {
    async fn visit(steps: &[Step], parent: Option<&str>, out: &mut Vec<TopologyNode>) {
        for step in steps {
            let id = document_panel::step_row_id(&step.id);
            out.push(TopologyNode { id: id.clone(), granularity: "step".into(), parent: parent.map(str::to_string) });
            for body in step.bodies.values() {
                visit(&body.steps, Some(id.as_str()), out);
            }
        }
    }
    let path = crate::artifacts::imperative::imperative_working_scene(document).path;
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
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — `setLocale`/`locale` is the row that
    /// proves they are different vocabularies. **Row order is the binary variant ordinal: appending is
    /// safe, reordering is a wire-format break.**
    pub enum ImperativeCommand for ImperativeSnapshot, ImperativeMutation, ImperativeConfig, ImperativeConfigMutation {
        "addStep" as "add-step" => add_step::AddStep,
        "addStepAt" as "add-step-at" => add_step_at::AddStepAt,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "removeStepAt" as "remove-step-at" => remove_step_at::RemoveStepAt,
        "moveStep" as "move-step" => move_step::MoveStep,
        "moveStepAt" as "move-step-at" => move_step_at::MoveStepAt,
        "setStepParams" as "set-step-params" => set_step_params::SetStepParams,
        "setStepParamsAt" as "set-step-params-at" => set_step_params_at::SetStepParamsAt,
        "run" as "run" => run::Run,
        "setLocale" as "locale" => set_locale::SetLocale,
        "setContributions" as "contributions" => set_contributions::SetContributions,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::imperative::commands::set_contributions;
use crate::editor::imperative::commands::{add_step, add_step_at, move_step, move_step_at, remove_step, remove_step_at, set_step_params, set_step_params_at};
use crate::editor::imperative::commands::{run, set_locale};
//#endregion 🔖️Commands

//#region 🔖️ImperativePlayApp
/// 🧪️ B1: unit struct — the former `ImperativePlayRuntime`/`self.runtime` field now lives in
/// `ImperativeConfig` (see `ArtifactEditor::Config`), written via `ImperativeConfigMutation`s.
#[derive(Default)]
pub struct ImperativePlayApp;

impl ArtifactEditor for ImperativePlayApp {
    type Snapshot = ImperativeSnapshot;
    type Mutation = ImperativeMutation;
    type Config = ImperativeConfig;
    type ConfigMutation = ImperativeConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = ImperativePresence;
    type PresenceMutation = ImperativePresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = ImperativeCommand;

    const DIALECT: semio_framework_plugin::app::Dialect = crate::artifacts::imperative::IMPERATIVE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = IMPERATIVE_DOCUMENT_SCHEMA;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::imperative::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> ImperativeSnapshot {
        default_snapshot()
    }

    async fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(imperative_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    async fn command_id(command: &ImperativeCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(command: &ImperativeCommand, doc: &ArtifactView<'_, ImperativeSnapshot>, cfg: &ConfigView<'_, ImperativeConfig>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ `steps` domain: `HierarchyProvider::Topology` from the document's own `Step::bodies` nesting —
    /// see `imperative_steps_topology`'s doc comment.
    async fn interaction_topology(doc: &ArtifactView<'_, ImperativeSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> InteractionTopology {
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(IMPERATIVE_INTERACTION_STEPS.to_string(), imperative_steps_topology(doc.snapshot));
        InteractionTopology { domains }
    }

    /// 🎞️ `"result:out"` exports the last `run` scope (a generic data value, the port recipe's
    /// `computation.imperative`-kinded output); `"document:out"` replicates `ArtifactEditor::export_media`'s
    /// default whole-document-pack behavior (unreachable once this override exists).
    async fn export_media(port: &str, doc: &ArtifactView<'_, ImperativeSnapshot>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let host = crate::editor::imperative::engine::ImperativeHost::from_snapshot(doc.snapshot.clone());
                let result = host.run();
                let json = serde_json::to_string(&result.scope).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.imperative".into(), json } })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, ImperativeSnapshot>, cfg: &ConfigView<'_, ImperativeConfig>) -> UiNode {
        imperative_engine::sync_imperative_module_contributions(&cfg.snapshot.contributions_json);
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = imperative_labels(config);
        match body_key {
            IMPERATIVE_PLAY_BODY_MAIN => main::render(document, &config.run_output_json, labels),
            IMPERATIVE_PLAY_BODY_SCRIPT => script::render(document),
            IMPERATIVE_PLAY_BODY_DOCUMENT => document_panel::render(document, labels),
            IMPERATIVE_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            IMPERATIVE_PLAY_BODY_INSPECTOR => inspection_panel::render(document, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️ImperativePlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub async fn create_imperative_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::imperative::IMPERATIVE_DIALECT)
            .command(CommandDefinition { in_palette: false, ..CommandDefinition::new_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) })
            .document(["semio", "imperative"])
            .artifact_kind(crate::artifacts::imperative::artifact_kind())
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
            .view_action("run", LocalizedLabel::native("Run", "Ausführen"))
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            // 📝️ Staged argument form for the panel-visible create action (the step kind is a choice).
            .action_args("addStep", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("state.set", LocalizedLabel::native("Set State", "Zustand setzen")),
                    ActionArgOption::new("log.print", LocalizedLabel::native("Print Log", "Log ausgeben")),
                    ActionArgOption::new("control.if", LocalizedLabel::native("If", "Wenn")),
                    ActionArgOption::new("control.while", LocalizedLabel::native("While", "Solange")),
                    ActionArgOption::new("math.add", LocalizedLabel::native("Add", "Addieren")),
                ]).default_value("log.print"),
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
            .window_kind_interactions(IMPERATIVE_PLAY_WINDOW_MAIN, vec![InteractionRef::new(IMPERATIVE_INTERACTION_STEPS)])
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS /
            // WORKFLOWS-END-TO-END-TYPED-PORTS) — `imperative_io()` is this port information's single
            // source of truth, reused here rather than duplicated.
            .io(imperative_io())
            // 🚧️ SDK GAP (w2-cad-report "SDK gaps found" #4, still open as of w0-f): `EditorBuilder`
            // has no `.example_source(...)`/`.workflow(...)` — `PluginBuilder::editor::<E>` only takes
            // the bare `AppDefinition`, so the demo-session example source and the `"imperative"`
            // workflow registration this app used to chain here are dropped, not ported. The
            // artifact-level `📚️examples/🎬️demo` facet (`crate::examples::art_imperative_demo`,
            // still mounted in `📦️glue.rs`) is the surviving example registration path.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type ImperativeApp = VcsArtifactApp<EditorApp<ImperativePlayApp>>;

    /// ✏️ `ImperativePlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<ImperativePlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<ImperativePlayApp>` builds it.
    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn imperative_app() -> ImperativeApp {
        new_app::<EditorApp<ImperativePlayApp>>()
    }

    /// 🧪️ Adapts `create_imperative_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `testkit::new_app_with_registry`/`assert_declared_actions_bridge_to_commands`
    /// still expect — framework testkit gap (w2-cad-report "SDK gaps found" #3), not modifiable here
    /// (`🧰️framework/**` is outside this packet's lease).
    pub async fn imperative_app_manifest_for_testkit() -> App {
        App { definition: create_imperative_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline and materializes
    /// declared action-arg defaults (e.g. `addStep`'s `kind`).
    pub async fn imperative_app_with_registry() -> ImperativeApp {
        new_app_with_registry::<EditorApp<ImperativePlayApp>>(imperative_app_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut ImperativeApp, command: ImperativeCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut ImperativeApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::imperative::testkit::{dispatch, imperative_app, imperative_app_with_registry, render};
    use semio_framework_plugin::testkit::{assert_undo_redo_round_trip, meta};
    use std::collections::BTreeMap;

    #[test]
    async fn app_definition_builds_without_panicking() {
        let app = create_imperative_app();
        assert_eq!(app.id, semio_framework::surface_app_id(&crate::artifacts::imperative::IMPERATIVE_DIALECT.into(), semio_framework::AppRole::Editor));
        assert!(app.keybindings.iter().any(|binding| binding.action.action == "undo"));
    }

    #[test]
    async fn imperative_io_is_declared_on_the_manifest() {
        let app = create_imperative_app();
        assert_eq!(app.io.artifact.id, "computation.imperative");
        assert_eq!(app.io.ports.len(), 1);
        assert_eq!(app.io.ports[0].id, "result:out");
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 11, "every ImperativeCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id, except for the one documented divergence (`setLocale` → `locale`, an
    /// undeclared host-pushed command). This is what a missing `#[dsl(keyword = ..)]` on a payload struct
    /// silently breaks (the record prints with no keyword at all and no longer parses).
    #[test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected = match id {
                "setLocale" => "locale".to_string(),
                "setContributions" => "contributions".to_string(),
                _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
            };
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// ⚖️ Rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact bytes
    /// captured from the pre-merge `semio-s-app-imperative-protocol` crate (ticket
    /// `🧪️wire-baseline-before.txt`). A regression here is a real format break, not a test-fixture
    /// mismatch.
    #[test]
    async fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(ImperativeCommand, &str, &str); 2] = [
            (ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: Some(1) }), "add-step add-step kind=log.print index=1", "010001096c6f672e7072696e7402000600010401"),
            (ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None }), "add-step add-step kind=log.print", "010001096c6f672e7072696e7401000600"),
        ];
        for (command, text, _hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<ImperativeCommand> {
        let mut params = BTreeMap::new();
        params.insert("message".to_string(), crate::artifacts::imperative::dsl::value_to_value_dsl(&neural_engine::Value::Atom(neural_engine::Atom::String("updated".into()))));
        vec![
            ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: Some(1) }),
            ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some("step-if".into()), slot: Some("then".into()) }),
            ImperativeCommand::RemoveStep(remove_step::RemoveStep { id: "step-1".into() }),
            ImperativeCommand::RemoveStepAt(remove_step_at::RemoveStepAt { id: "step-1".into(), owner: Some("step-if".into()), slot: Some("then".into()) }),
            ImperativeCommand::MoveStep(move_step::MoveStep { id: "step-1".into(), index: 2 }),
            ImperativeCommand::MoveStepAt(move_step_at::MoveStepAt { id: "step-1".into(), index: 2, owner: None, slot: None }),
            ImperativeCommand::SetStepParams(set_step_params::SetStepParams { id: "step-1".into(), params: params.clone() }),
            ImperativeCommand::SetStepParamsAt(set_step_params_at::SetStepParamsAt { id: "step-1".into(), owner: Some("step-if".into()), slot: Some("then".into()), params }),
            ImperativeCommand::Run(run::Run {}),
            ImperativeCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            ImperativeCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_imperative_app()).expect("app definition json");
        for id in [IMPERATIVE_PLAY_WINDOW_MAIN, script::IMPERATIVE_PLAY_WINDOW_SCRIPT] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::IMPERATIVE_PLAY_MODE_EDIT), "mode missing from the manifest");
        for body in [IMPERATIVE_PLAY_BODY_DOCUMENT, IMPERATIVE_PLAY_BODY_CATALOGUE, IMPERATIVE_PLAY_BODY_INSPECTOR] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("computation.imperative"), "artifact kind missing from the manifest");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️Interaction
    /// 🕹️ The `steps` domain is declared `HierarchyProvider::Topology`, transitive on both hover and
    /// selection, and scoped to the main window kind — the manifest side of ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    #[test]
    async fn steps_interaction_domain_is_declared_topology_and_transitive_on_the_main_window() {
        let definition = create_imperative_app();
        let steps = definition.interactions.iter().find(|interaction| interaction.id == IMPERATIVE_INTERACTION_STEPS).expect("steps interaction domain declared");
        assert!(matches!(steps.hierarchy, HierarchyProvider::Topology));
        assert!(steps.hover.transitive, "steps hover must be transitive so a control step's hover covers its nested body steps");
        assert!(steps.selection.transitive, "steps selection must be transitive so a control step's selection covers its nested body steps");
        let main_window = definition.window_kinds.iter().find(|window| window.id == IMPERATIVE_PLAY_WINDOW_MAIN).expect("main window kind declared");
        assert!(main_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == IMPERATIVE_INTERACTION_STEPS), "main window must reference the steps interaction domain");
    }

    /// 🌳️ `interaction_topology` walks a `control.if` step's `bodies["then"]` nesting into
    /// `TopologyNode.parent` links — the owner step has no parent, the nested step's parent is the
    /// owner's own row id.
    #[test]
    async fn interaction_topology_walks_nested_control_bodies_into_parent_links() {
        let mut app = imperative_app();
        dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "control.if".into(), index: None }));
        let owner_id = crate::artifacts::imperative::imperative_working_scene(&app.snapshot().expect("projection")).path.steps.last().expect("owner").id.clone();
        dispatch(&mut app, ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some(owner_id.clone()), slot: Some("then".into()) }));
        let document = app.snapshot().expect("projection");
        let config = ImperativeConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config };
        let topology = ImperativePlayApp::interaction_topology(&doc, &cfg);
        let steps = topology.domains.get(IMPERATIVE_INTERACTION_STEPS).expect("steps domain present in topology");
        let owner_row_id = document_panel::step_row_id(&owner_id);
        let owner_node = steps.ordered.iter().find(|node| node.id == owner_row_id).expect("owner node present");
        assert!(owner_node.parent.is_none(), "top-level owner step has no parent");
        let nested = steps.ordered.iter().find(|node| node.parent.as_deref() == Some(owner_row_id.as_str())).expect("nested step present under owner");
        assert_eq!(nested.granularity, "step");
    }

    /// 🌱️ A document with no steps has an empty `steps` topology — every stale `steps` selection id
    /// gets pruned.
    #[test]
    async fn interaction_topology_is_empty_for_a_document_with_no_steps() {
        let document = ImperativeSnapshot::default();
        let config = ImperativeConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config };
        let topology = ImperativePlayApp::interaction_topology(&doc, &cfg);
        assert!(topology.domains.get(IMPERATIVE_INTERACTION_STEPS).expect("steps domain present in topology").ordered.is_empty());
    }
    //#endregion 🔖️Interaction

    //#region 🔖️CrossCutting
    #[test]
    async fn add_step_materializes_kind_default_and_run_emits_no_artifact_mutations() {
        let mut app = imperative_app_with_registry();
        // AddStep fired with no explicit kind: the declared `kind` default ("log.print") must be
        // materialized by the registry's action-arg default resolution.
        app.dispatch_typed(ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None }), &meta("local")).expect("add step");
        let document = app.snapshot().expect("materialize projection");
        let path = crate::artifacts::imperative::imperative_working_scene(&document).path;
        assert_eq!(path.steps.last().unwrap().kind, "log.print");
        // `run` is a View-kind command: under registry enforcement it must not emit document operations.
        let result = app.dispatch_typed(ImperativeCommand::Run(run::Run {}), &meta("local")).expect("run");
        assert!(result.mutations.is_empty(), "run evaluates into config, never the document");
    }

    #[test]
    async fn default_snapshot_has_steps() {
        let app = imperative_app();
        let path = crate::artifacts::imperative::imperative_working_scene(&app.snapshot().expect("projection")).path;
        assert_eq!(path.steps.len(), 2);
    }

    #[test]
    async fn add_step_command_appends_step() {
        let mut app = imperative_app();
        dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None }));
        let path = crate::artifacts::imperative::imperative_working_scene(&app.snapshot().expect("projection")).path;
        assert!(path.steps.len() > 2);
    }

    #[test]
    async fn add_step_at_owner_slot_nests_into_control_body() {
        let mut app = imperative_app();
        dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "control.if".into(), index: None }));
        let owner_id = crate::artifacts::imperative::imperative_working_scene(&app.snapshot().expect("projection")).path.steps.last().expect("owner").id.clone();
        let root_len = crate::artifacts::imperative::imperative_working_scene(&app.snapshot().expect("projection")).path.steps.len();
        dispatch(&mut app, ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some(owner_id.clone()), slot: Some("then".into()) }));
        let document = app.snapshot().expect("projection");
        let path = crate::artifacts::imperative::imperative_working_scene(&document).path;
        let owner_step = path.steps.iter().find(|step| step.id == owner_id).expect("owner step");
        assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
        assert_eq!(path.steps.len(), root_len, "nested step lives in the slot, not the root path");
    }

    #[test]
    async fn add_step_at_falls_back_to_root_for_unknown_owner() {
        let mut app = imperative_app();
        dispatch(&mut app, ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some("missing-step".into()), slot: Some("then".into()) }));
        let document = app.snapshot().expect("projection");
        let path = crate::artifacts::imperative::imperative_working_scene(&document).path;
        let added_id = path.steps.last().expect("added").id.clone();
        assert!(path.steps.iter().any(|step| step.id == added_id));
    }

    #[test]
    async fn undo_after_add_step_restores_original_document_exactly() {
        let mut app = imperative_app();
        let base = default_snapshot();
        let mut path = crate::artifacts::imperative::imperative_working_scene(&base).path;
        path.steps.push(Step { id: "step-3".into(), kind: "log.print".into(), params: crate::artifacts::imperative::Dictionary::new(), bodies: BTreeMap::new() });
        let expected_after = crate::artifacts::imperative::imperative_snapshot_with_content(&base.schema, &path, &crate::artifacts::imperative::imperative_working_scene(&base).seed);
        assert_undo_redo_round_trip(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None }), |app| app.snapshot().expect("projection"), default_snapshot(), expected_after);
    }

    #[test]
    async fn remove_step_command_is_exact_inverse_of_add() {
        let mut app = imperative_app();
        let original = app.snapshot().expect("projection");
        dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None }));
        let added_id = crate::artifacts::imperative::imperative_working_scene(&app.snapshot().expect("projection")).path.steps.last().expect("added").id.clone();
        dispatch(&mut app, ImperativeCommand::RemoveStep(remove_step::RemoveStep { id: added_id }));
        assert_eq!(app.snapshot().expect("projection"), original);
    }

    /// 🧪️ The definitional regression proof: two independent instances start from the same document,
    /// apply DISJOINT edits (A appends a root step, B patches an existing step's params), and exchanging
    /// operations over a `MemoryBackbone` converges both sides onto an identical projection — impossible
    /// under whole-document `setDocument` snapshots, which would clobber one side's write.
    #[test]
    async fn two_instances_converge_disjoint_edits_via_backbone() {
        let mut params = BTreeMap::new();
        params.insert("key".to_string(), crate::artifacts::imperative::dsl::value_to_value_dsl(&neural_engine::Value::Atom(neural_engine::Atom::String("renamed".into()))));
        semio_framework_plugin::testkit::assert_two_instances_converge::<semio_framework_plugin::EditorApp<ImperativePlayApp>, _>(
            "mem://imperative-convergence",
            ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None }),
            ImperativeCommand::SetStepParams(set_step_params::SetStepParams { id: "step-1".into(), params }),
            |app| app.snapshot().expect("projection"),
        );
    }

    #[test]
    async fn ingest_operations_is_idempotent_for_imperative() {
        semio_framework_plugin::testkit::assert_ingest_idempotent::<semio_framework_plugin::EditorApp<ImperativePlayApp>, _>(ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None }), |app| {
            crate::artifacts::imperative::imperative_working_scene(&app.snapshot().expect("projection")).path.steps.len()
        });
    }

    #[test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = imperative_app();
        assert!(render(&mut app, "imperative.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
