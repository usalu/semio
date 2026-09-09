
use super::*;
use semio_framework_plugin::PluginApp;
use testkit::{Block3dApp, new_app};

fn block_on_preview_law<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

async fn drive_preview_operation(app: &mut Block3dApp, stage: &str) -> Result<(u64, u64, u64), String> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let mut artifact = 0;
    let mut config = 0;
    let mut window_transient = 0;
    while app.has_pending_typed_operations() {
        if std::time::Instant::now() >= deadline {
            return Err(format!("block3d preview {stage} operation did not finish"));
        }
        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
        app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
        if let Some(page) = app.take_typed_operation_result_page(1) {
            let fault = (page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault)
                .then(|| format!("block3d preview {stage} publication fault: {}", String::from_utf8_lossy(page.bytes())));
            artifact += u64::from(page.lane == semio_framework_plugin::app::TypedOperationResultLane::Artifact);
            config += u64::from(page.lane == semio_framework_plugin::app::TypedOperationResultLane::Config);
            window_transient += u64::from(page.lane == semio_framework_plugin::app::TypedOperationResultLane::WindowTransient);
            app.acknowledge_typed_operation_result(page.token).map_err(|error| format!("{error:?}"))?;
            if let Some(fault) = fault {
                while app.has_pending_typed_operations() {
                    app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
                    app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
                    std::thread::yield_now();
                }
                return Err(fault);
            }
        }
        app.take_typed_operation_effect();
        app.take_typed_operation_event();
        app.take_typed_operation_ui_scope();
        std::thread::yield_now();
    }
    Ok((artifact, config, window_transient))
}

//#region 🔖️CommandSurface
fn every_command() -> Vec<Block3dCommand> {
    vec![
        Block3dCommand::PatchObjectKind(patch_object_kind::PatchObjectKind { field: "name".into(), value: "x".into() }),
        Block3dCommand::AddRepresentation(add_representation::AddRepresentation {}),
        Block3dCommand::RemoveRepresentation(remove_representation::RemoveRepresentation { id: "r0".into() }),
        Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {}),
        Block3dCommand::RemoveVortexKind(remove_vortex_kind::RemoveVortexKind { id: "v0".into() }),
        Block3dCommand::AddVortex(add_vortex::AddVortex {}),
        Block3dCommand::RemoveVortex(remove_vortex::RemoveVortex { id: "v0".into() }),
        Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: "capsule".into() }),
        Block3dCommand::Edit(edit::Edit { text: "{}".into() }),
        Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: Some("r0".into()) }),
        Block3dCommand::SetWindowRepresentations(set_window_representations::SetWindowRepresentations { window_id: "w0".into(), representation_ids: vec!["r0".into()] }),
        Block3dCommand::ToggleWindowRepresentation(toggle_window_representation::ToggleWindowRepresentation { window_id: "w0".into(), representation_id: "r0".into(), visible: true }),
        Block3dCommand::SetWindowArrangement(set_window_arrangement::SetWindowArrangement { window_id: "w0".into(), arrangement: "x".into() }),
        Block3dCommand::SetWindowSpacing(set_window_spacing::SetWindowSpacing { window_id: "w0".into(), spacing: 8.0 }),
        Block3dCommand::SetBrushVortexKind(set_brush_vortex_kind::SetBrushVortexKind { vortex_kind_id: Some("v0".into()) }),
        Block3dCommand::SetBrushRadius(set_brush_radius::SetBrushRadius { radius: 0.3 }),
        Block3dCommand::SetBrushFlip(set_brush_flip::SetBrushFlip { flip: true }),
        Block3dCommand::HoverSurface(hover_surface::HoverSurface { window_id: "w0".into(), object_id: "r0".into(), position: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] }),
        Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {}),
        Block3dCommand::PlaceVortex(place_vortex::PlaceVortex { window_id: "w0".into(), object_id: "r0".into(), position: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] }),
        Block3dCommand::SetCamera(set_camera::SetCamera { camera: BlockCamera3d::default() }),
        Block3dCommand::PatchRepresentation(patch_representation::PatchRepresentation { id: "r0".into(), field: "name".into(), value: "x".into() }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_cover_every_row() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(Block3dCommand::command_id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎮️command-roster/🔣️.json")).unwrap();
    let expected: Vec<&str> = fixture["toolIds"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect();
    assert_eq!(ids, expected, "the artifact command roster matches the language-neutral oracle");
}

#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🧷️ Pins the current binary ordinal and text of the divergent leave-surface command.
#[semio_framework_async_macros::async_test]
async fn leave_surface_text_and_binary_match_the_command_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎮️command-roster/🔣️.json")).unwrap();
    let hex = |command: &Block3dCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
    assert_eq!(protocol::OpText::print_op(&Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {})), fixture["leaveSurface"]["text"].as_str().unwrap());
    assert_eq!(hex(&Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {})), fixture["leaveSurface"]["binaryHex"].as_str().unwrap());
}

/// ⚖️ LAW: every one of the 22 declared `Block3dCommand` rows is retained-owned by
/// `Block3dRetainedCommandJobFactory`, classified `Migrated` in the manifest, and carries an exact,
/// nonempty publication-lane contract. `AppActionRegistry::tool_job_registration` enforces the same
/// set equality at app construction (`interactive-job.catalog-incomplete`), and
/// `validate_ui_dispatch_classification` rejects anything not `Migrated` at the very first gate of
/// `handle_action` — this test pins both so a future command row that forgets its retained-tool-id,
/// its classification, or its lane contract fails here instead of going silently dispatch-dead.
/// Mirrors block5d's own retained-route discipline and generation3d's
/// `retained_route_dispositions_are_exact_and_exhaustive`.
#[semio_framework_async_macros::async_test]
async fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::ToolExecutionShape;
    assert_eq!(BLOCK3D_RETAINED_TOOL_IDS.len(), 22);
    assert_eq!(<Block3dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 22);
    assert_eq!(BLOCK3D_PUBLICATION_CONTRACTS.len(), 22);
    assert_eq!(block3d_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
    let mut sorted_ids = BLOCK3D_RETAINED_TOOL_IDS.to_vec();
    sorted_ids.sort_unstable();
    sorted_ids.dedup();
    assert_eq!(sorted_ids.len(), BLOCK3D_RETAINED_TOOL_IDS.len(), "duplicate retained tool ids in {BLOCK3D_RETAINED_TOOL_IDS:?}");
    assert_eq!(Block3dRetainedCommandJobFactory::TOOL_IDS, BLOCK3D_RETAINED_TOOL_IDS);
    for command in every_command() {
        let tool_id = command.command_id();
        assert!(BLOCK3D_RETAINED_TOOL_IDS.contains(&tool_id), "command {tool_id} is not owned by Block3dRetainedCommandJobFactory");
        let contract = BLOCK3D_PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == tool_id).unwrap_or_else(|| panic!("tool {tool_id} declares a publication contract"));
        assert!(!contract.lanes.is_empty(), "tool {tool_id} declares a nonempty publication lane set");
    }
    // 🪟️ App-level actions are fanned onto every window kind by `try_build_definition`, so the
    // world window carries the complete classified action set (`AppDefinition` has no app-level
    // `actions` field of its own).
    let definition = create_block3d_app();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎮️command-roster/🔣️.json")).unwrap();
    assert_eq!(serde_json::to_value(&definition.breadcrumb).unwrap(), fixture["document"]);
    let world_window = definition.window_kinds.iter().find(|window| window.id == world::BLOCK3D_WINDOW_WORLD).expect("world window declared");
    for tool_id in BLOCK3D_RETAINED_TOOL_IDS {
        let action = world_window.actions.iter().find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} is declared by the manifest"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "action {tool_id} must be UI-dispatchable");
    }
}

/// ⚖️ LAW: the two lanes any block3d tool can publish into both have a real one-item preparation
/// factory — a Config-lane tool without `build_config_store_one_item_preparation_factory` is
/// rejected at dispatch with `interactive-job.publication-authority-missing`.
#[semio_framework_async_macros::async_test]
async fn both_declared_publication_lanes_have_a_preparation_factory() {
    assert!(<Block3dPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some());
    assert!(<Block3dPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some());
}

#[test]
fn brush_preview_publications_are_partitioned_by_trusted_window_context() {
    std::thread::Builder::new()
        .name("block3d-window-transient-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| block_on_preview_law(async {
    let mut app = new_app().await;
    let outcome: Result<(), String> = async {
        let document = app.snapshot().map_err(|error| format!("{error:?}"))?;
        let view_a = testkit::world_view_state("world-a");
        let view_b = testkit::world_view_state("world-b");
        testkit::dispatch_in_window(
            &mut app,
            Block3dCommand::HoverSurface(hover_surface::HoverSurface {
                window_id: "spoofed-window".into(),
                object_id: "object-a".into(),
                position: [1.0, 2.0, 3.0],
                normal: [0.0, 1.0, 0.0],
            }),
            "world-a",
        )
        .await;
        if drive_preview_operation(&mut app, "world-a hover").await? != (0, 0, 1)
            || app.window_transient_generation(&view_a).map_err(|error| format!("{error:?}"))? != Some(1)
            || app.window_transient_generation(&view_b).map_err(|error| format!("{error:?}"))? != Some(0)
        {
            return Err("world-a hover leaked into world-b".into());
        }
        if app.snapshot().map_err(|error| format!("{error:?}"))? != document {
            return Err("hover modified document content".into());
        }
        testkit::dispatch_in_window(
            &mut app,
            Block3dCommand::HoverSurface(hover_surface::HoverSurface {
                window_id: "world-a".into(),
                object_id: "object-b".into(),
                position: [-4.0, 5.0, 6.0],
                normal: [0.0, 0.0, -1.0],
            }),
            "world-b",
        )
        .await;
        if drive_preview_operation(&mut app, "world-b hover").await? != (0, 0, 1)
            || app.window_transient_generation(&view_a).map_err(|error| format!("{error:?}"))? != Some(1)
            || app.window_transient_generation(&view_b).map_err(|error| format!("{error:?}"))? != Some(1)
        {
            return Err("world-b hover did not remain independent".into());
        }
        let before_place = app.snapshot().map_err(|error| format!("{error:?}"))?.vortices.len();
        testkit::dispatch_in_window(&mut app, Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {}), "world-a").await;
        if drive_preview_operation(&mut app, "world-a leave").await? != (0, 0, 1)
            || app.window_transient_generation(&view_a).map_err(|error| format!("{error:?}"))? != Some(2)
            || app.window_transient_generation(&view_b).map_err(|error| format!("{error:?}"))? != Some(1)
        {
            return Err("leave did not clear only world-a".into());
        }
        testkit::dispatch_in_window(
            &mut app,
            Block3dCommand::PlaceVortex(place_vortex::PlaceVortex {
                window_id: "world-a".into(),
                object_id: "object-b".into(),
                position: [0.5, 0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            }),
            "world-b",
        )
        .await;
        let place_lanes = drive_preview_operation(&mut app, "world-b place").await?;
        let place_generation_a = app.window_transient_generation(&view_a).map_err(|error| format!("{error:?}"))?;
        let place_generation_b = app.window_transient_generation(&view_b).map_err(|error| format!("{error:?}"))?;
        if place_lanes != (2, 0, 1) || place_generation_a != Some(2) || place_generation_b != Some(2) {
            return Err(format!(
                "placement did not mutate the artifact and clear only trusted world-b: lanes={place_lanes:?} world-a={place_generation_a:?} world-b={place_generation_b:?}"
            ));
        }
        if app.snapshot().map_err(|error| format!("{error:?}"))?.vortices.len() != before_place + 1 {
            return Err("placement did not create one vortex".into());
        }
        Ok(())
    }
    .await;
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut app);
    outcome.expect("window-partitioned brush preview");
        }))
        .expect("spawn Block3d window-transient law")
        .join()
        .expect("Block3d window-transient law thread");
}

/// 🌉️ Every surface-declared action must bridge through `command_from_action` and round-trip
/// `command_id`.
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<Block3dPlayApp>>(testkit::block3d_app_manifest_for_testkit).await;
    assert!(<Block3dPlayApp as ArtifactEditor>::command_from_action("noSuchAction", None).is_err());
}
//#endregion 🔖️CommandSurface

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let definition = create_block3d_app();
    assert_eq!(definition.modes.len(), 1);
    assert_eq!(definition.window_kinds.len(), 1);
    for body_key in [document_panel::BLOCK3D_BODY_DOCUMENT, inspection_panel::BLOCK3D_BODY_INSPECTOR] {
        assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
    }
    assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain is declared
/// once, with both granularities, a `Topology` hierarchy, and scoped to the world window kind —
/// the framework auto-injects the six interaction actions for it (asserted separately below via
/// `assert_declared_actions_bridge_to_commands`'s injected-action allowance).
#[semio_framework_async_macros::async_test]
async fn declares_the_vortex_interaction_domain_scoped_to_the_world_window() {
    let definition = create_block3d_app();
    let interaction = definition.interactions.iter().find(|def| def.id == BLOCK3D_INTERACTION_VORTEX).expect("vortex domain declared");
    assert_eq!(interaction.granularities.iter().map(|granularity| granularity.id.as_str()).collect::<Vec<_>>(), vec![BLOCK3D_GRANULARITY_VORTEX, BLOCK3D_GRANULARITY_SURFACE]);
    assert!(matches!(interaction.hierarchy, HierarchyProvider::Topology));
    let world_window = definition.window_kinds.iter().find(|window| window.id == world::BLOCK3D_WINDOW_WORLD).expect("world window declared");
    assert!(world_window.interactions.contains(&InteractionRef::new(BLOCK3D_INTERACTION_VORTEX)));
}

/// 🕹️ `interaction_topology` returns one flat root per representation (`surface` granularity) and
/// per vortex template (`vortex` granularity) — enough structure for `validate_state` to prune a
/// stale selection the moment `removeRepresentation`/`removeVortex` deletes its target.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_covers_every_representation_and_vortex() {
    let mut app: Block3dApp = new_app().await;
    testkit::dispatch(&mut app, Block3dCommand::AddRepresentation(add_representation::AddRepresentation {})).await;
    testkit::dispatch(&mut app, Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {})).await;
    testkit::dispatch(&mut app, Block3dCommand::AddVortex(add_vortex::AddVortex {})).await;
    let snapshot = app.snapshot().expect("snapshot");
    let representation_id = snapshot.representations[0].id.clone();
    let vortex_id = snapshot.vortices[0].id.clone();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = Block3dConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let topology = <Block3dPlayApp as ArtifactEditor>::interaction_topology(&doc, &cfg);
    let domain = topology.domains.get(BLOCK3D_INTERACTION_VORTEX).expect("vortex domain topology present");
    assert!(domain.contains(&format!("surface:{representation_id}")).await);
    assert!(domain.contains(&format!("vortex:{vortex_id}")).await);
}

#[semio_framework_async_macros::async_test]
async fn block3d_io_declares_the_catalog_out_port() {
    let io = block3d_io();
    assert_eq!(io.document_schema, BLOCK_3D_SCHEMA);
    let ports = io.all_ports().await;
    assert!(ports.iter().any(|port| port.id == "document:in"));
    assert!(ports.iter().any(|port| port.id == "document:out"));
    let catalog = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
    assert_eq!(catalog.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(catalog.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(catalog.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
}

#[semio_framework_async_macros::async_test]
async fn renders_document_tree_and_inspector() {
    let mut app: Block3dApp = new_app().await;
    let json = testkit::render(&mut app, document_panel::BLOCK3D_BODY_DOCUMENT).await;
    assert!(json.contains("Representations"));
    let inspector = testkit::render(&mut app, inspection_panel::BLOCK3D_BODY_INSPECTOR).await;
    assert!(inspector.contains("\"type\":\"tree\""));
    assert!(inspector.contains("Name"));
    assert!(inspector.contains("Vortices"));
}
//#endregion 🔖️Manifest

//#region 🔖️Behavior
/// ⚖️ LAW: the editor boots non-empty. `world_meshes_json` drops any representation whose
/// `mesh_url` is `None`, so a boot document must both carry representations and name their meshes
/// for the `World3d` window to paint anything before the first user action. The mesh url is
/// asserted on the scene's own compute facet rather than on the rendered body: the semantic
/// surface contract pack-encodes the scene into `SurfaceProps.doc.bytes`, so no scene string
/// survives into the rendered tree's JSON any more.
#[semio_framework_async_macros::async_test]
async fn the_editor_boots_with_a_renderable_world() {
    let mut app: Block3dApp = new_app().await;
    let snapshot = app.snapshot().expect("snapshot");
    assert!(!snapshot.representations.is_empty(), "the boot document must carry at least one representation");
    assert!(snapshot.representations.iter().all(|representation| representation.mesh_url.is_some()), "every boot representation must name a mesh url");
    let visible: Vec<&crate::BlockRepresentation> = snapshot.representations.iter().collect();
    assert!(crate::editor::block3d::world::world_meshes_json(&snapshot, &visible).contains("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb"), "the world scene must reference the boot document's mesh");
    assert!(testkit::render(&mut app, world::BLOCK3D_BODY_WORLD).await.contains("\"type\":\"surface\""), "the world body must render a semantic scene surface");
}

#[semio_framework_async_macros::async_test]
async fn world_scene_projects_only_the_supplied_window_preview() {
    let mut app: Block3dApp = new_app().await;
    let snapshot = app.snapshot().expect("snapshot");
    let config = Block3dConfig::default();
    let view = crate::Block3dWindowView::for_window("world-a");
    let visible = crate::editor::block3d::world::visible_representations(&snapshot, &view);
    let preview = Block3dBrushPreview { position: [1.0, 2.0, 3.0], direction: [0.0, 1.0, 0.0] };
    let with_preview: serde_json::Value = serde_json::from_str(&crate::editor::block3d::world::world_vortices_json(&snapshot, &config, &visible, &view, Some(&preview))).expect("preview scene");
    let without_preview: serde_json::Value = serde_json::from_str(&crate::editor::block3d::world::world_vortices_json(&snapshot, &config, &visible, &view, None)).expect("plain scene");
    let find_preview = |value: &serde_json::Value| {
        value
            .as_array()
            .and_then(|records| records.iter().find(|record| record["fullId"] == "__brush_preview__"))
            .cloned()
    };
    let projected = find_preview(&with_preview).expect("supplied preview projected");
    assert_eq!(projected["position"], serde_json::json!([1.0, 2.0, 3.0]));
    assert_eq!(projected["direction"], serde_json::json!([0.0, 1.0, 0.0]));
    assert!(find_preview(&without_preview).is_none(), "a window without transient preview must render none");
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn add_representation_then_set_active_then_render_world_shows_mesh() {
    let mut app: Block3dApp = new_app().await;
    testkit::dispatch(&mut app, Block3dCommand::AddRepresentation(add_representation::AddRepresentation {})).await;
    let representation_id = app.snapshot().expect("snapshot").representations[0].id.clone();
    testkit::dispatch(&mut app, Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: Some(representation_id) })).await;
    let json = testkit::render(&mut app, world::BLOCK3D_BODY_WORLD).await;
    assert!(json.contains("\"type\":\"surface\""), "world body must render a scene surface");
    assert!(json.contains("world-3d"), "the scene surface must declare the world-3d surface kind");
}

/// 🚀️ Counted relative to the boot document (`initial_snapshot` now parses the
/// `hexagonal-cut-concrete-forest-left` fixture, which already ships vortex kinds and vortices)
/// rather than against a hard-coded 1/0.
#[semio_framework_async_macros::async_test]
async fn add_vortex_kind_then_add_vortex_then_remove_round_trips() {
    let mut app: Block3dApp = new_app().await;
    let before = app.snapshot().expect("snapshot").vortices.len();
    testkit::dispatch(&mut app, Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {})).await;
    testkit::dispatch(&mut app, Block3dCommand::AddVortex(add_vortex::AddVortex {})).await;
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(projection.vortices.len(), before + 1);
    let vortex_id = projection.vortices[before].id.clone();
    testkit::dispatch(&mut app, Block3dCommand::RemoveVortex(remove_vortex::RemoveVortex { id: vortex_id })).await;
    assert_eq!(app.snapshot().expect("snapshot").vortices.len(), before);
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_capsule_fixture() {
    let mut app: Block3dApp = new_app().await;
    testkit::dispatch(&mut app, Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK3D_EXAMPLE_CAPSULE.into() })).await;
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(projection.object_kind.id, "Capsule J");
    // 🥽️ One representation, not two: the former `"1:500"` row named `/mesh/capsule_J.1to500.glb`,
    // which no mesh delivery catalog ships, so `resolveMeshAsset` threw the instant the example
    // loaded. There is no 1:500 `.glb` anywhere in the repo (only a Rhino `.3dm` source), so the
    // row was removed from the fixture rather than repointed at an unrelated mesh.
    assert_eq!(projection.representations.len(), 1);
    assert_eq!(projection.representations[0].mesh_url.as_deref(), Some("/mesh/🧊️capsule_J.glb"));
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_through_the_wrapper() {
    let mut app: Block3dApp = new_app().await;
    let kinds = |app: &mut Block3dApp| crate::vortex_kinds_of(&app.snapshot().expect("snapshot")).len();
    let before = kinds(&mut app);
    testkit::dispatch(&mut app, Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {})).await;
    assert_eq!(kinds(&mut app), before + 1);
    app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).await.expect("undo");
    assert_eq!(kinds(&mut app), before);
    app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).await.expect("redo");
    assert_eq!(kinds(&mut app), before + 1);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `setSelection`/`selectVortex`/
/// `hoverVortex` are gone — the still-config-only `setActiveRepresentation` view action now
/// exercises the "view action never touches the document" contract this test used to cover.
#[semio_framework_async_macros::async_test]
async fn set_active_representation_writes_config_not_document() {
    let mut app: Block3dApp = new_app().await;
    let result = app
        .dispatch_typed(Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: Some("r0".into()) }), &semio_framework_plugin::testkit::meta("local"))
        .await
        .expect("set active representation");
    assert!(result.mutations.is_empty(), "setActiveRepresentation is config-only and must emit no document operations");
}

#[semio_framework_async_macros::async_test]
async fn export_media_catalog_out_wraps_the_puzzle3d_fragment() {
    let mut app: Block3dApp = new_app().await;
    testkit::dispatch(&mut app, Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK3D_EXAMPLE_CAPSULE.into() })).await;
    let media = semio_framework_plugin::resolve_ready(app.export_media("catalog:out")).expect("export catalog");
    assert_eq!(media.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "kit.catalog");
            let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
            assert_eq!(value["objectKinds"][0]["id"], "Capsule J");
        }
        other => panic!("expected Structured payload, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn place_vortex_on_surface_auto_creates_kind_and_vortex() {
    let mut app: Block3dApp = new_app().await;
    testkit::dispatch(&mut app, Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK3D_EXAMPLE_CAPSULE.into() })).await;
    testkit::dispatch(&mut app, Block3dCommand::PlaceVortex(place_vortex::PlaceVortex { window_id: BLOCK3D_DEFAULT_WINDOW_ID.into(), object_id: "r0".into(), position: [0.5, 0.0, 1.0], normal: [0.0, 1.0, 0.0] })).await;
    let projection = app.snapshot().expect("snapshot");
    assert!(!crate::vortex_kinds_of(&projection).is_empty());
    assert_eq!(projection.vortices.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn command_from_action_bridges_set_active_example() {
    assert!(
        matches!(<Block3dPlayApp as ArtifactEditor>::command_from_action("setActiveExample", Some(&dsl::json::to_dsl_value(&dsl::json!({ "exampleId": "capsule" })))), Ok(Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id })) if id == "capsule")
    );
}
//#endregion 🔖️Behavior

//#region 🔖️WindowMeasures
/// 🧬️ Kind-discipline wrapper: the real registry enforces View actions never emit document
/// operations. Exercising it here (rather than only the plain `new_app()`) is the reason
/// `testkit::app_with_registry` exists.
#[semio_framework_async_macros::async_test]
async fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
    let mut app = testkit::app_with_registry().await;
    let result = testkit::dispatch(&mut app, Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: Some("r0".into()) })).await;
    assert!(result.mutations.is_empty(), "setActiveRepresentation is a view action and must never reach document operations under kind discipline");
}

/// 🎚️ The world window collects its five option measures (representations/quick-pick/arrangement/
/// spacing/brush) fresh per frame — never frozen into the manifest.
#[semio_framework_async_macros::async_test]
async fn world_window_measures_collect_all_five_options() {
    let mut app: Block3dApp = new_app().await;
    testkit::dispatch(&mut app, Block3dCommand::AddRepresentation(add_representation::AddRepresentation {})).await;
    let measures = testkit::main_window_measures(&mut app).await;
    assert_eq!(measures.len(), 5, "world window must expose representations/quick-pick/arrangement/spacing/brush");
}
//#endregion 🔖️WindowMeasures
