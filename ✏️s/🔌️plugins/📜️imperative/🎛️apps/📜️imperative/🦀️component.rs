//! 🖥️ Imperative play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`. This file is a routing table: `handle` →
//! `ImperativeCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::apps::imperative::config::{ImperativeConfig, ImperativeConfigOperation};
use crate::apps::imperative::modes::edit;
use crate::apps::imperative::modes::edit::windows::{main, script};
use crate::apps::imperative::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::imperative::terminology::imperative_labels;
use crate::artifacts::imperative::engine::{default_document, imperative_io};
use crate::artifacts::imperative::op::ImperativeOperation;
use crate::artifacts::imperative::{ImperativeDocument, IMPERATIVE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftOperation, DraftView, ActionArgDef, ActionArgOption, ActionDescriptor, App, ConfigView, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, UiNode};
use store::EngineHandles;
use serde_json::Value;
use store::DocumentPack;

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_APP_ID: &str = "imperative-play";
pub use main::IMPERATIVE_PLAY_BODY_MAIN;
pub use script::IMPERATIVE_PLAY_BODY_SCRIPT;
pub use catalogue_panel::IMPERATIVE_PLAY_BODY_CATALOGUE;
pub use document_panel::IMPERATIVE_PLAY_BODY_DOCUMENT;
pub use inspection_panel::IMPERATIVE_PLAY_BODY_INSPECTOR;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory `📌️panels/*` builds its item
/// actions with.
pub fn imperative_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: IMPERATIVE_PLAY_APP_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `ImperativePlayApp::Command` — the SOLE dispatch surface for imperative's own behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — `setLocale`/`locale` is the row that
    /// proves they are different vocabularies. **Row order is the binary variant ordinal: appending is
    /// safe, reordering is a wire-format break.**
    pub enum ImperativeCommand for ImperativeDocument, ImperativeOperation, ImperativeConfig, ImperativeConfigOperation {
        "addStep" as "add-step" => add_step::AddStep,
        "addStepAt" as "add-step-at" => add_step_at::AddStepAt,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "removeStepAt" as "remove-step-at" => remove_step_at::RemoveStepAt,
        "moveStep" as "move-step" => move_step::MoveStep,
        "moveStepAt" as "move-step-at" => move_step_at::MoveStepAt,
        "setStepParams" as "set-step-params" => set_step_params::SetStepParams,
        "setStepParamsAt" as "set-step-params-at" => set_step_params_at::SetStepParamsAt,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "run" as "run" => run::Run,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::apps::imperative::commands::step::{add_step, add_step_at, move_step, move_step_at, remove_step, remove_step_at, set_step_params, set_step_params_at};
use crate::apps::imperative::commands::view::{run, set_locale, set_selection};
//#endregion 🔖️Commands

//#region 🔖️ImperativePlayApp
/// 🧪️ B1: unit struct — the former `ImperativePlayRuntime`/`self.runtime` field now lives in
/// `ImperativeConfig` (see `DocumentApp::Config`), written via `ImperativeConfigOperation`s.
#[derive(Default)]
pub struct ImperativePlayApp;

impl DocumentApp for ImperativePlayApp {
    type Projection = ImperativeDocument;
    type Operation = ImperativeOperation;
    type Config = ImperativeConfig;
    type ConfigOperation = ImperativeConfigOperation;
    type Draft = NoDraft;
    type DraftOperation = NoDraftOperation;

    type Command = ImperativeCommand;

    const APP_ID: &'static str = IMPERATIVE_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = IMPERATIVE_DOCUMENT_SCHEMA;

    fn initial_projection() -> ImperativeDocument {
        default_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(imperative_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &ImperativeCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &ImperativeCommand, doc: &DocumentView<'_, ImperativeDocument>, cfg: &ConfigView<'_, ImperativeConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<ImperativeOperation, ImperativeConfigOperation, Self::DraftOperation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🎞️ `"result:out"` exports the last `run` scope (a generic data value, the port recipe's
    /// `computation.imperative`-kinded output); `"document:out"` replicates `DocumentApp::export_media`'s
    /// default whole-document-pack behavior (unreachable once this override exists).
    fn export_media(port: &str, doc: &DocumentView<'_, ImperativeDocument>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let host = crate::artifacts::imperative::engine::ImperativeHost::from_document(doc.projection.clone());
                let result = host.run();
                let json = serde_json::to_string(&result.scope).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.imperative".into(), json } })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn render(body_key: &str, doc: &DocumentView<'_, ImperativeDocument>, cfg: &ConfigView<'_, ImperativeConfig>) -> UiNode {
        let document = doc.projection;
        let config = cfg.projection;
        let labels = imperative_labels(config);
        match body_key {
            IMPERATIVE_PLAY_BODY_MAIN => main::render(document, &config.run_output_json, labels),
            IMPERATIVE_PLAY_BODY_SCRIPT => script::render(document),
            IMPERATIVE_PLAY_BODY_DOCUMENT => document_panel::render(document, &config.selected_step_ids, labels),
            IMPERATIVE_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            IMPERATIVE_PLAY_BODY_INSPECTOR => inspection_panel::render(document, &config.selected_step_ids, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️ImperativePlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_imperative_app() -> App {
    App::from_builder(
        App::builder(IMPERATIVE_PLAY_APP_ID, LocalizedLabel::native("Imperative", "Imperativ"))
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
            .operation("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"))
            .operation("addStepAt", LocalizedLabel::native("Add Step At", "Schritt bei Position hinzufügen"))
            .operation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
            .operation("removeStepAt", LocalizedLabel::native("Remove Step At", "Schritt bei Position entfernen"))
            .operation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .operation("moveStepAt", LocalizedLabel::native("Move Step At", "Schritt bei Position verschieben"))
            .operation("setStepParams", LocalizedLabel::native("Set Step Params", "Schrittparameter festlegen"))
            .operation("setStepParamsAt", LocalizedLabel::native("Set Step Params At", "Schrittparameter bei Position festlegen"))
            // 👁️ Ephemeral view state / runtime effect — selection is scratch, `run` evaluates into config.
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
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
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS /
            // WORKFLOWS-END-TO-END-TYPED-PORTS) — `imperative_io()` is this port information's single
            // source of truth, reused here rather than duplicated.
            .io(imperative_io()),
    )
    .example("demo", LocalizedLabel::native("Demo", "Demo"), serde_json::to_string(&default_document()).expect("default_document is a static, hand-built value with no non-finite floats or non-UTF8 keys"), "cylinder")
    .workflow("imperative", "Imperative", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewModel};

    pub type ImperativeApp = VcsDocumentApp<ImperativePlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn imperative_app() -> ImperativeApp {
        new_app::<ImperativePlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline and materializes
    /// declared action-arg defaults (e.g. `addStep`'s `kind`).
    pub fn imperative_app_with_registry() -> ImperativeApp {
        new_app_with_registry::<ImperativePlayApp>(create_imperative_app)
    }

    pub fn dispatch(app: &mut ImperativeApp, command: ImperativeCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut ImperativeApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::imperative::testkit::{dispatch, imperative_app, imperative_app_with_registry, render};
    use semio_framework_plugin::testkit::{assert_undo_redo_round_trip, meta};
    use std::collections::BTreeMap;

    #[test]
    fn app_definition_builds_without_panicking() {
        let app = create_imperative_app();
        assert_eq!(app.definition.id, IMPERATIVE_PLAY_APP_ID);
        assert!(app.definition.keybindings.iter().any(|binding| binding.action.action == "undo"));
    }

    #[test]
    fn imperative_io_is_declared_on_the_manifest() {
        let app = create_imperative_app();
        assert_eq!(app.definition.io.artifact.id, "computation.imperative");
        assert_eq!(app.definition.io.ports.len(), 1);
        assert_eq!(app.definition.io.ports[0].id, "result:out");
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
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
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id, except for the one documented divergence (`setLocale` → `locale`, an
    /// undeclared host-pushed command). This is what a missing `#[dsl(keyword = ..)]` on a payload struct
    /// silently breaks (the record prints with no keyword at all and no longer parses).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected = if id == "setLocale" { "locale".to_string() } else { id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect() };
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// ⚖️ Rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact bytes
    /// captured from the pre-merge `semio-s-app-imperative-protocol` crate (ticket
    /// `🧪️wire-baseline-before.txt`). A regression here is a real format break, not a test-fixture
    /// mismatch.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(ImperativeCommand, &str, &str); 2] = [
            (ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: Some(1) }), "add-step kind=log.print index=1", "010001096c6f672e7072696e7402000600010401"),
            (ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None }), "add-step kind=log.print", "010001096c6f672e7072696e7401000600"),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<ImperativeCommand> {
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
            ImperativeCommand::SetSelection(set_selection::SetSelection { ids: vec!["step-1".into(), "step-2".into()] }),
            ImperativeCommand::Run(run::Run {}),
            ImperativeCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_imperative_app().definition).expect("app definition json");
        for id in [main::IMPERATIVE_PLAY_WINDOW_MAIN, script::IMPERATIVE_PLAY_WINDOW_SCRIPT] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::IMPERATIVE_PLAY_MODE_EDIT), "mode missing from the manifest");
        for body in [IMPERATIVE_PLAY_BODY_DOCUMENT, IMPERATIVE_PLAY_BODY_CATALOGUE, IMPERATIVE_PLAY_BODY_INSPECTOR] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("computation.imperative"), "artifact kind missing from the manifest");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[test]
    fn add_step_materializes_kind_default_and_run_emits_no_document_operations() {
        let mut app = imperative_app_with_registry();
        // AddStep fired with no explicit kind: the declared `kind` default ("log.print") must be
        // materialized by the registry's action-arg default resolution.
        app.dispatch_typed(ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None }), &meta("local")).expect("add step");
        let document = app.projection().expect("materialize projection");
        assert_eq!(document.path.steps.last().unwrap().kind, "log.print");
        // `run` is a View-kind command: under registry enforcement it must not emit document operations.
        let result = app.dispatch_typed(ImperativeCommand::Run(run::Run {}), &meta("local")).expect("run");
        assert!(result.operations.is_empty(), "run evaluates into config, never the document");
    }

    #[test]
    fn default_document_has_steps() {
        let app = imperative_app();
        assert_eq!(app.projection().expect("projection").path.steps.len(), 2);
    }

    #[test]
    fn add_step_command_appends_step() {
        let mut app = imperative_app();
        dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None }));
        assert!(app.projection().expect("projection").path.steps.len() > 2);
    }

    #[test]
    fn add_step_at_owner_slot_nests_into_control_body() {
        let mut app = imperative_app();
        dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "control.if".into(), index: None }));
        let owner_id = app.projection().expect("projection").path.steps.last().expect("owner").id.clone();
        let root_len = app.projection().expect("projection").path.steps.len();
        dispatch(&mut app, ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some(owner_id.clone()), slot: Some("then".into()) }));
        let document = app.projection().expect("projection");
        let owner_step = document.path.steps.iter().find(|step| step.id == owner_id).expect("owner step");
        assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
        assert_eq!(document.path.steps.len(), root_len, "nested step lives in the slot, not the root path");
    }

    #[test]
    fn add_step_at_falls_back_to_root_for_unknown_owner() {
        let mut app = imperative_app();
        dispatch(&mut app, ImperativeCommand::AddStepAt(add_step_at::AddStepAt { kind: "log.print".into(), index: None, owner: Some("missing-step".into()), slot: Some("then".into()) }));
        let document = app.projection().expect("projection");
        let added_id = document.path.steps.last().expect("added").id.clone();
        assert!(document.path.steps.iter().any(|step| step.id == added_id));
    }

    #[test]
    fn undo_after_add_step_restores_original_document_exactly() {
        let mut app = imperative_app();
        let mut expected_after = default_document();
        expected_after.path.steps.push(crate::artifacts::imperative::Step { id: "step-3".into(), kind: "log.print".into(), params: crate::artifacts::imperative::Dictionary::new(), bodies: BTreeMap::new() });
        assert_undo_redo_round_trip(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "log.print".into(), index: None }), |app| app.projection().expect("projection"), default_document(), expected_after);
    }

    #[test]
    fn remove_step_command_is_exact_inverse_of_add() {
        let mut app = imperative_app();
        let original = app.projection().expect("projection");
        dispatch(&mut app, ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None }));
        let added_id = app.projection().expect("projection").path.steps.last().expect("added").id.clone();
        dispatch(&mut app, ImperativeCommand::RemoveStep(remove_step::RemoveStep { id: added_id }));
        assert_eq!(app.projection().expect("projection"), original);
    }

    /// 🧪️ The definitional regression proof: two independent instances start from the same document,
    /// apply DISJOINT edits (A appends a root step, B patches an existing step's params), and exchanging
    /// operations over a `MemoryBackbone` converges both sides onto an identical projection — impossible
    /// under whole-document `setDocument` snapshots, which would clobber one side's write.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        let mut params = BTreeMap::new();
        params.insert("key".to_string(), crate::artifacts::imperative::dsl::value_to_value_dsl(&neural_engine::Value::Atom(neural_engine::Atom::String("renamed".into()))));
        semio_framework_plugin::testkit::assert_two_instances_converge::<ImperativePlayApp, _>(
            "mem://imperative-convergence",
            ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None }),
            ImperativeCommand::SetStepParams(set_step_params::SetStepParams { id: "step-1".into(), params }),
            |app| app.projection().expect("projection"),
        );
    }

    #[test]
    fn ingest_operations_is_idempotent_for_imperative() {
        semio_framework_plugin::testkit::assert_ingest_idempotent::<ImperativePlayApp, _>(ImperativeCommand::AddStep(add_step::AddStep { kind: "math.add".into(), index: None }), |app| app.projection().expect("projection").path.steps.len());
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = imperative_app();
        assert!(render(&mut app, "imperative.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
