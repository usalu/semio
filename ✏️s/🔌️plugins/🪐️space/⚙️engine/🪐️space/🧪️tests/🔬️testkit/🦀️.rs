
use super::*;
use semio_framework_os::{ArtifactPresentation, MediaClass, MediaForm, PortMultiplicity, apply_workflow_operation, register_app_io};
use semio_framework_os::{MediaPortDirection, MediaPortSpec, MediaType, WorkflowMediaPort, WorkflowNode};
use semio_framework_plugin::{App, AppIo, HistoryView, LocalizedLabel};

pub(crate) fn empty_history() -> HistoryView {
    HistoryView::empty()
}

pub type SpaceVcsApp = semio_framework_plugin::VcsArtifactApp<SpaceApp>;

/// 🕹️ Creates a fixture with the real manifest registry and graph interaction domain.
pub(crate) async fn app_with_registry() -> SpaceVcsApp {
    semio_framework_plugin::testkit::new_registered_app::<SpaceApp, _>(create_space_app()).await
}

pub(crate) async fn dispatch(app: &mut SpaceVcsApp, command: SpaceCommand) -> semio_framework_plugin::InvocationResult {
    app.dispatch_typed(command, &semio_framework_plugin::testkit::meta("local")).await.expect("dispatch")
}

/// 🕹️ Routes through `SpaceCommand::dispatch` (the `app_commands!`-generated, framework-fixed
/// 3-arg path — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), NOT
/// `SpaceApp::handle` (which now needs a real `InteractionView`, only obtainable through a full
/// `VcsArtifactApp` dispatch — see `testkit::app`/`dispatch` below for that path). The 7 commands
/// that read live selection (`deleteSelection`/`nodeGraphEdit`/`reorganizeWorkflow`/
/// `copyAppInstance`/`duplicateAppInstance`/`removeAppInstance`/`renameAppInstance`) fall back to
/// treating the selection as empty here — exactly the same degradation `SpaceApp::render`'s own
/// selection-dependent branches already carry — so this helper stays usable for every OTHER
/// command's non-selection-dependent behavior unchanged.
pub(crate) async fn studio_emit(projection: &WorkflowSnapshot, config: &SpaceConfig, command: &SpaceCommand) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let history = empty_history();
    let doc = ArtifactView::new(projection, &history);
    let cfg = ConfigView { snapshot: config, window: None };
    command.dispatch(&doc, &cfg)
}

/// 📽️ Folds studio document operations onto a projection the way the store would (minus history).
pub(crate) async fn apply_mutations(projection: &WorkflowSnapshot, operations: &[WorkflowMutation]) -> WorkflowSnapshot {
    operations.iter().fold(projection.clone(), |current, operation| apply_workflow_operation(&current, operation))
}

/// 📽️ Folds studio config operations onto a config snapshot the way the store would.
pub(crate) async fn apply_config(config: &SpaceConfig, operations: &[SpaceConfigMutation]) -> SpaceConfig {
    apply_config_mutations(config, operations).await
}

/// 🪪️ Canonical surface id for a synthetic test-registry app (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET §1) — every `App::builder(...)` id must parse
/// via `semio_framework::parse_surface_app_id`, so this mirrors `surface_app_id` over a throwaway
/// `s.<slug>@1/*` dialect. Shared by `seed_app` and every command test module that dispatches
/// `SpawnApp`/looks the registration back up, so both sides agree on the same string.
pub(crate) async fn test_surface_id(slug: &str) -> String {
    semio_framework::surface_app_id(&semio_framework::ArtifactDialect { artifact_kind: format!("s.{slug}"), standard: "1".into(), subset: "*".into() }, semio_framework::AppRole::Editor)
}

async fn seed_app(plugin_id: &str, app_id: &str, label: &str, document: &[&str], document_schema: &str, ports: Vec<MediaPortSpec>) {
    let surface_id = test_surface_id(app_id).await;
    let definition = App::builder(surface_id, LocalizedLabel::data(label))
        .await
        .document(document.iter().map(|segment| segment.to_string()))
        .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::native("Main", "Hauptansicht"), format!("{app_id}.main"), semio_framework_ui_contract::SurfaceKind::Canvas2d, "square-pen")
        .await
        .io(AppIo::from_document(document_schema, MediaType { class: MediaClass::Data, form: MediaForm::Value }, ArtifactPresentation { id: app_id.into(), name: label.into(), dimension: String::new(), component_kind: app_id.into() })
            .await
            .with_ports(ports)
            .await)
        .await
        .build_definition();
    register_app_io(plugin_id, &definition);
}

pub(crate) async fn seed_draw_plugin() {
    seed_app("draw", "draw", "Draw", &["semio", "draw"], "draw.document", Vec::new()).await;
}

pub(crate) async fn seed_multi_port_plugins() {
    let puzzle_ports = vec![
        MediaPortSpec {
            id: "in-a".into(),
            label: "In A".into(),
            direction: MediaPortDirection::In,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: Some("topology".into()),
            required: false,
            multiplicity: PortMultiplicity::One,
        },
        MediaPortSpec {
            id: "out-a".into(),
            label: "Out A".into(),
            direction: MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: Some("topology".into()),
            required: false,
            multiplicity: PortMultiplicity::One,
        },
        MediaPortSpec {
            id: "out-b".into(),
            label: "Out B".into(),
            direction: MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: Some("topology".into()),
            required: false,
            multiplicity: PortMultiplicity::One,
        },
    ];
    seed_app("puzzle.5d", "puzzle5d", "Puzzle 5D", &["semio", "puzzle", "5d"], "puzzle5d.document", puzzle_ports).await;

    let shooting_ports = vec![MediaPortSpec {
        id: "scene-in".into(),
        label: "Scene".into(),
        direction: MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        kind_id: Some("2d.shooting".into()),
        required: true,
        multiplicity: PortMultiplicity::One,
    }];
    seed_app("shooting", "shooting", "Shooting", &["semio", "shooting"], "shooting.document", shooting_ports).await;
}

pub(crate) async fn test_node(id: &str, inputs: Vec<WorkflowMediaPort>, outputs: Vec<WorkflowMediaPort>) -> WorkflowNode {
    WorkflowNode {
        id: id.into(),
        plugin_id: "test".into(),
        app_id: "test".into(),
        label: id.into(),
        yields: String::new(),
        artifact_ref: format!("artifacts/{id}"),
        config_ref: format!("config/{id}"),
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
        inputs,
        outputs,
    }
}

pub(crate) async fn test_port(node_id: &str, spec_id: &str, direction: MediaPortDirection, media_type: MediaType, kind_id: &str) -> WorkflowMediaPort {
    let dir_word = match direction {
        MediaPortDirection::In => "in",
        MediaPortDirection::Out => "out",
    };
    WorkflowMediaPort { id: format!("{node_id}:{spec_id}:{dir_word}"), spec: MediaPortSpec { id: spec_id.into(), label: spec_id.into(), direction, media_type, kind_id: Some(kind_id.into()), required: false, multiplicity: PortMultiplicity::One } }
}
