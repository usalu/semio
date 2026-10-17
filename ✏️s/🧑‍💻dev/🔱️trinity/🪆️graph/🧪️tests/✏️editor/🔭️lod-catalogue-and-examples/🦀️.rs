//! 🔭️ Level of detail, the catalogue, the inspection panel, example loading, deletion, export and ports of the editor laws. See [`super`].

use super::*;

#[semio_framework_async_macros::async_test]
async fn graph_scene_has_lod_json() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let node = app.render("trinity.jack.play.main", None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree");
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&json).expect("node-graph scene");
    assert!(scene.lod_json.as_deref().is_some_and(|lod| lod.contains("automatic")), "lodJson lane: {:?}", scene.lod_json);

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn set_lod_mode_reflects_in_window_measures() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let view = ViewModel { window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "jack-graph-main".into(), window_kind_id: "trinity-jack-graph".into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let addressed = view.for_window_instance("jack-graph-main").unwrap();
    app.dispatch_typed(TrinityJackCommand::SetLodMode { value: "compact".into() }, &semio_framework_plugin::ActionMeta { view_state: Some(addressed), ..meta("local") }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx).await.expect("lod");
    // 📬️ `settle` drains the completion outbox too — a page-only loop spins forever on it.
    let receipt = settle(&mut app,cx).await;
    assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault));
    let measures = app.window_measures(&view).await;
    assert!(measures["jack-graph-main"].iter().any(|measure| matches!(measure, WindowMeasure::Select { value, .. } if value == "compact")));

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn catalogue_tree_renders() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let node = app.render("trinity.jack.play.catalogue", None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    assert!(artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree").contains("trinity-jack-catalogue"));

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn inspection_panel_renders_the_selection_prompt() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let node_id = node_id_at(&app, 0).await;
    select_ast(&mut app, &[&node_id],cx).await;
    let node = app.render("trinity.jack.play.inspection", None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    // 🕹️ `render` has no `InteractionView` (see the panel's own doc comment) — it can no longer
    // build per-selection fields, so it always renders the static prompt.
    assert!(artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree").contains("trinity-inspector.empty"));

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn document_tree_de_locale_translates_labels() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let view = ViewModel { locale: semio_framework_ui_locale::Locale::De, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let node = app.render("trinity.jack.play.artifact", None, &view).await.expect("render");
    assert!(artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree").contains("Stücke"));

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_the_example_with_its_preset_query() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let result = app.dispatch_typed(TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() }, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx).await.expect("set active example");
    let receipt = settle(&mut app,cx).await;
    // 🩹 Pre-existing test/implementation mismatch (traced to commit `a445617c`, 2026-08-12
    // 15:50:51 +0200 — predates this migration, not introduced by it): `set_active_example`
    // routes the fixture swap through `Effect::LoadDocument` (whole-document replace is
    // banned from the `Mutation` enum outright), never through `artifact_mutations`, so
    // `InvocationResult.mutations` is always empty for this command — `requested_effects` is
    // the field that actually carries the swap.
    let loaded = result.requested_effects.iter().chain(receipt.effects.iter()).find_map(|effect| match effect {
        semio_framework_plugin::Effect::LoadDocument { pack, .. } => Some(<jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot as pack::snapshot::ArtifactPack>::decode_pack(pack).expect("the loaded example decodes")),
        _ => None,
    });
    let loaded = loaded.expect("setActiveExample must request the LoadDocument effect");
    assert_eq!(loaded.query, jack::editor::jack::commands::set_active_example::preset_query("branch-chain"), "the example document carries its own preset query");
    let bytes=SemioJackComposition::genesis_child_pack(&loaded,"content",&loaded.content.child_id).expect("selected original genesis").expect("actual authored child");
    let rich=<semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("actual child pack");
    assert_eq!((rich.nodes.len(),rich.edges.len()),(9,6));
    jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(rich).expect("actual child retirement");

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_selected_node() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let node_id = node_id_at(&app, 0).await;
    select_ast(&mut app, &[&node_id],cx).await;
    let result = app.dispatch_typed(TrinityJackCommand::DeleteSelection, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx).await.expect("delete");
    let receipt = settle(&mut app,cx).await;
    assert!(result.mutations.is_empty(), "deleteSelection writes no parent leaf: {:?}", result.mutations);
    assert!(receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Child), "deleteSelection must publish one content child edit: lanes {:?}", receipt.lanes);
    assert!(!live_scene(&app).await.nodes.iter().any(|node| node.id == node_id));

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_stays_within_row_budget_and_ends_with_delete_selection() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let node_id = node_id_at(&app, 0).await;
    let request = ContextMenuRequest {
        menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
        surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: "trinity.jack.play".into(),
            kind: "nodeGraph".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: node_id.clone(), label: None }],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: vec![node_id] }],
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    let menu = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("grouped disclosure menu should not be empty");
    let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("deleteSelection");
    let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
    assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must be last: {menu:?}");

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn export_media_graph_out_matches_document_pack() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    use semio_framework_plugin::PluginApp as _;
    let mut app = new_app().await;
    let document_out = app.export_media("artifact:out",cx).await.expect("document:out export").expect("actual document media");
    let graph_out = app.export_media("graph:out",cx).await.expect("graph:out export").expect("actual graph media");
    assert_eq!(document_out.payload, graph_out.payload);

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn jack_io_declares_graph_out_fan_out_port() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let io = create_trinity_jack_app().io;
    assert_eq!(io.artifact_schema, TRINITY_GRAPH_SCHEMA);
    assert_eq!(io.artifact.id, "graph.trinity");
    let graph_out = io.ports.iter().find(|port| port.id == "graph:out").expect("graph:out declared");
    assert_eq!(graph_out.kind_id.as_deref(), Some("graph.trinity"));
    assert_eq!(graph_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}
