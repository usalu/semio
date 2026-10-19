/// 🪟️ Window and menu laws: action bridging, viewport decoding, installs, history, labels, context menus and window measures.

#[semio_framework_async_macros::async_test]
async fn declared_actions_bridge_to_commands() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    semio_framework_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<Generation3dPlayApp>>(context::generation3d_app_manifest_for_tests).await;
}

/// 🎥️ LAW: `nodeGraphViewport` declares NO args, so the shell can stage nothing for it — an absent
/// `viewport` therefore decodes to the identity camera rather than refusing the bridge, while a
/// PRESENT but malformed one still faults. Both halves of `parse_flow_viewport` are stated here
/// because the generic bridge law only ever exercises the absent half
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn node_graph_viewport_decodes_an_absent_viewport_as_identity_and_refuses_a_malformed_one() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let decode = |args: semio_framework_value::DslValue| <Generation3dPlayApp as ArtifactEditor>::command_from_action("nodeGraphViewport", Some((args).clone()), None);
    let identity = semio_framework_os_kernel::Viewport2d::default();
    assert_eq!(decode(semio_framework_value::DslValue::Object(Vec::new())).expect("an argless invocation bridges"), Generation3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: identity }));
    let authored = semio_framework_os_kernel::Viewport2d { x: 12.0, y: -8.0, zoom: 2.0 };
    assert_eq!(
        decode(semio_framework_value::DslValue::object([("viewport".into(), semio_framework_value::ToValue::to_value(&authored))])).expect("an authored viewport bridges"),
        Generation3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: authored })
    );
    let refused = decode(semio_framework_value::DslValue::object([("viewport".into(), semio_framework_value::DslValue::object([("x".into(), semio_framework_value::DslValue::float(1.0)), ("y".into(), semio_framework_value::DslValue::float(2.0)), ("zoom".into(), semio_framework_value::DslValue::float(0.0))]))]));
    assert!(refused.is_err(), "a zoom the graph cannot express must fault instead of being replaced by the identity camera");
}

#[semio_framework_async_macros::async_test]
async fn registry_backed_editor_installs_every_declared_bounded_command_proof() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let _app = app_with_registry().await;
}

#[test]
fn the_manifest_stitches_every_taxonomy_node() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let json = serde_json::to_string(&create_generation3d_app()).expect("app definition json");
    for id in [
        flow_window::GENERATION_3D_PLAY_WINDOW_MAIN,
        edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW,
        generations::GENERATION_3D_PLAY_WINDOW_GENERATIONS,
        form::GENERATION_3D_PLAY_WINDOW_GENERATE_FORM,
        generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW,
    ] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for id in [edit::GENERATION_3D_PLAY_MODE_EDIT, generate::GENERATION_3D_PLAY_MODE_GENERATE] {
        assert!(json.contains(id), "mode {id} missing from the manifest");
    }
    assert!(json.contains("3d.generation"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn each_example_loads_distinct_fixture_and_preview_geometry() {
    use semio_s_artifact_procedural_3d_model::*;
    use semio_s_artifact_procedural_3d_model::widget_id;
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let examples = [
        PROCEDURAL_EXAMPLE_HEX_COLUMN,
        PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
        PROCEDURAL_EXAMPLE_SPHERE_TORUS,
        PROCEDURAL_EXAMPLE_BOX_FILLET,
        PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
        PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE,
        PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
        PROCEDURAL_EXAMPLE_BOX_SHELL,
    ];
    let mut signatures = std::collections::BTreeSet::new();
    for example_id in examples {
        let mut app = app().await;
        context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() })).await;
        let signature = format!("{:?}", context::snapshot(&app).host_snapshot.widgets.iter().map(|widget| widget_id(widget).to_string()).collect::<std::collections::BTreeSet<_>>());
        assert!(signatures.insert(signature.clone()), "duplicate fixture signature for {example_id}: {signature}");
    }
}

#[semio_framework_async_macros::async_test]
async fn refresh_pending_effects_starts_the_preview_eval_run() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() })).await;
    let (_, preview_view) = context::preview_views("procedural-preview-test", "procedural-preview-test-other");
    let effects = app.pending_effects(Some(&preview_view)).await;
    assert!(effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, args: Some(args), .. } if action == semio_framework_tool_run::TOOL_RUN_START_ACTION_ID && args.get("toolId").and_then(semio_framework_value::DslValue::as_str) == Some(semio_s_artifact_procedural_3d_preview_eval::PREVIEW_EVAL_TOOL_ID))), "a refresh over a never-evaluated preview starts the previewEval run: {effects:?}");
    let receipt = context::drive_preview_run(&mut app, &preview_view, &effects).await;
    assert_eq!(receipt.state.as_deref(), Some("finalized"), "the read-only run settles and finalizes itself: {receipt:?}");
}

/// ♻️ A replacing `toolRunStart` retires the replaced run's snapshots through the store, never by a plain drop. Measured live
/// 2026-09-27 (`s` serve, V1 matrix en, `s13-s17-logs/live/gen3d-trap-1.txt`): after an `addWidget` moved the head and the
/// store retired its own alias, the replaced previewEval run held the last `Arc<Generation3dSnapshot>` and its plain drop
/// aborted the guest with `ordered-map root must be explicitly retired before drop` (`FlowHostSnapshot.layout`). Several
/// edit → re-run rounds give the store's own retirement time to finish before each replacing start.
#[semio_framework_async_macros::async_test]
async fn a_replacing_preview_run_start_retires_the_previous_runs_last_snapshot_alias() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() })).await;
    let (_, preview_view) = context::preview_views("procedural-preview-test", "procedural-preview-test-other");
    for round in 0..4 {
        let effects = app.pending_effects(Some(&preview_view)).await;
        let receipt = context::drive_preview_run(&mut app, &preview_view, &effects).await;
        assert_eq!(receipt.state.as_deref(), Some("finalized"), "round {round}: the read-only run settles and finalizes itself: {receipt:?}");
        context::dispatch(&mut app, Generation3dCommand::AddWidget(add_widget::AddWidget { neuron_kind: None, format: None, action: None, kind: "inputNote".into(), x: None, y: None })).await;
        context::settle(&mut app).await;
    }
    let effects = app.pending_effects(Some(&preview_view)).await;
    let receipt = context::drive_preview_run(&mut app, &preview_view, &effects).await;
    assert_eq!(receipt.state.as_deref(), Some("finalized"), "the last replacing run finalizes with every earlier run's snapshots retired: {receipt:?}");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_flow_graph_edits() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app).host_snapshot.widgets.len();
    semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip(&mut app.0, Generation3dCommand::AddWidget(add_widget::AddWidget { neuron_kind: None, format: None, action: None, kind: "inputNote".into(), x: None, y: None }), |app| context::snapshot(&app).host_snapshot.widgets.len(), before, before + 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())
    .await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_widget_moves() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let widgets: Vec<String> = {
        let app = app().await;
        context::snapshot(&app).host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget).to_string()).collect()
    };
    assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
    let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
    semio_framework_plugin::artifact_app_laws::assert_two_registered_instances_converge::<EditorApp<Generation3dPlayApp>, (Option<f64>, Option<f64>), _, _>(
        "mem://generation3d-convergence",
        || async { context::generation3d_app_manifest_for_tests() },
        Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: node_move_operations_json(&w0, 111.0, 5.0) }),
        Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: node_move_operations_json(&w1, 222.0, 6.0) }),
        move |app| {
            let layout = &context::snapshot(&app).host_snapshot.layout;
            (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
        },
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn generation3d_labels_translate_catalogue_and_inspector_in_german() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::De, ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native) };
    let catalogue = context::render_with_view(&mut app, catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE, &view_state).await;
    assert!(catalogue.contains("\"Elemente\""));
    let inspector = context::render_with_view(&mut app, inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION, &view_state).await;
    assert!(inspector.contains("Elemente:"));
}

/// 🕹️ The runtime graph-selection route persists through an exactly owned interaction store.
#[semio_framework_async_macros::async_test]
async fn generation3d_interaction_selection_owns_its_persisted_history() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let node_id = context::snapshot(&app).host_snapshot.widgets.first().map(semio_s_artifact_procedural_3d_model::widget_id).expect("default fixture node").to_string();
    context::select_graph(&mut app, "node", &[node_id.as_str()]).await;
    assert_eq!(app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.as_slice()), Some([node_id].as_slice()));
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

/// 🗂️ With nothing selected the selection-conditioned rows stay folded away and the disclosure
/// budget holds.
#[semio_framework_async_macros::async_test]
async fn context_menu_grouped_disclosure_stays_within_budget() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let widgets: Vec<String> = context::snapshot(&app).host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget).to_string()).collect();
    assert!(!widgets.is_empty(), "default fixture needs at least one widget for the test");
    let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let menu = app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    assert!(!menu.is_empty(), "grouped disclosure menu should not be empty");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

/// 🕹️ The runtime funnels every right-click through `context_menu_with_request_context`, so a node
/// selected through the framework-owned `graph` domain — never through `request.surface`, which is
/// `None` here exactly as it is for a menu opened off a scene surface the app did not paint — must
/// unfold the transform trio, the removal group and the destructive delete row.
#[semio_framework_async_macros::async_test]
async fn context_menu_reads_the_framework_owned_graph_selection() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let node_id = "extrude".to_string();
    let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let ids_of = |menu: &[semio_framework_plugin::ContextMenuItemSpec]| -> Vec<String> {
        menu.iter().flat_map(|item| std::iter::once(item.id.clone()).chain(item.children.iter().flatten().map(|child| child.id.clone()))).collect()
    };
    let unselected = ids_of(&app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await);
    assert!(!unselected.iter().any(|id| id == "translateSelection"), "an empty selection must not offer a transform: {unselected:?}");
    assert!(!unselected.iter().any(|id| id == "removeWidget"), "an empty selection must not offer a removal target: {unselected:?}");
    context::select_graph(&mut app, "node", &[node_id.as_str()]).await;
    let selected = ids_of(&app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await);
    for id in ["translateSelection", "rotateSelection", "scaleSelection", "removeWidget", "removeGeneration"] {
        assert!(selected.iter().any(|candidate| candidate == id), "a live graph selection must offer {id}: {selected:?}");
    }
    assert!(selected.iter().any(|id| id.contains("delete")), "a live graph selection must offer the destructive delete row: {selected:?}");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

/// 🕸️ A preview instance id (`{widget}@{channel}#{index}`) and a synapse id land in the node and edge
/// domains respectively — the projection `context_menu_body` hands `selection_domains_from_surface`.
#[test]
fn graph_selection_splits_into_node_and_edge_domains() {
    let mut fixture = semio_framework_artifact_flow_flow::FlowHostSnapshot::default();
    fixture.widgets.push(semio_framework_artifact_flow_flow::Widget::InputNote { id: "note".into(), text: "n".into() });
    fixture.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "wire".into(), from: "note".into(), to: "note".into(), from_port: String::new(), to_port: String::new() });
    let marks = PreviewInteractionMarks { source_revision: None, components: Default::default(), hovered: Default::default(), selected: ["note@out#2".to_string(), "wire".to_string()].into_iter().collect() };
    assert_eq!(marks.graph_selection_domains(&fixture), (vec!["note".to_string()], vec!["wire".to_string()]));
}

/// 🕹️ `worldPointerDown`/`graphPointerDown` are gone: the world host reports a pick through the
/// framework-injected `interactionSelect` on the declared `graph` domain (`World3dHost/🟦️.tsx:4447`),
/// and nothing in the repo dispatches a graph pointer-down at all. A route no surface can reach is
/// dead API, not an observation hook.


/// 🗂️ Every `menu.group.<category>` row this app emits names a category the SHELL can label.
///
/// A group row travels with `label: None` by contract and the shell resolves it from the closed
/// `RIBBON_PARENT_CATEGORIES` taxonomy; a category outside that table falls back to the raw id, and a
/// user saw a context-menu row reading literally `menu.group.io` (measured on the React serve
/// 2026-09-14, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The menu is asked with AND without a
/// selection because the `targets` group only exists with one.
#[semio_framework_async_macros::async_test]
async fn context_menu_groups_are_taxonomy_categories() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let mut categories: Vec<String> = Vec::new();
    for selection in [Vec::new(), vec!["extrude"]] {
        if !selection.is_empty() {
            context::select_graph(&mut app, "node", &selection).await;
        }
        for item in app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await {
            let Some(category) = item.id.strip_prefix("menu.group.") else { continue };
            assert!(item.label.is_none(), "a taxonomy group row carries no label of its own: {item:?}");
            categories.push(category.to_string());
        }
    }
    assert!(!categories.is_empty(), "the menu is built out of taxonomy groups; none was emitted");
    for category in &categories {
        assert!(
            semio_framework_ui_declaration::layout::ribbon_parent_label(category, false).is_some() && semio_framework_ui_declaration::layout::ribbon_parent_label(category, true).is_some(),
            "`menu.group.{category}` is outside the closed ribbon-parent taxonomy, so the shell renders its raw id to the user"
        );
    }
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

/// 🌞️ Every window-chrome measure label this app paints is TRANSLATED, group headers and picker rows
/// included.
///
/// The Show picker, the LOD picker and the whole Sun group were hardcoded English strings sitting
/// outside `Generation3dLabels` entirely, so a German-locale user read `Sun / Enabled / Azimuth /
/// Elevation / Intensity` beside a correctly-translated palette entry for the identical verb
/// (`📓️window-coverage-audit-2026-09-14.md` §4). Comparing the two locales' label trees is what makes
/// the next added measure fail here rather than in front of a user.
#[semio_framework_async_macros::async_test]
async fn window_measure_labels_are_localized() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let view_for = |locale| semio_framework_plugin::ViewModel {
        locale,
        window_instances: vec![
            semio_framework_plugin::ViewWindowInstance { id: flow_window::GENERATION_3D_PLAY_WINDOW_MAIN.into(), window_kind_id: flow_window::GENERATION_3D_PLAY_WINDOW_MAIN.into() },
            semio_framework_plugin::ViewWindowInstance { id: edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), window_kind_id: edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() },
        ],
        ..semio_framework_plugin::ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native)
    };
    let labelled = |measures: &std::collections::HashMap<String, Vec<semio_framework_plugin::WindowMeasure>>| -> Vec<(String, String)> {
        fn walk(measure: &semio_framework_plugin::WindowMeasure, out: &mut Vec<(String, String)>) {
            match measure {
                semio_framework_plugin::WindowMeasure::Group { id, label, children, .. } => {
                    out.push((id.clone(), label.clone()));
                    for child in children {
                        walk(child, out);
                    }
                }
                semio_framework_plugin::WindowMeasure::Select { id, label, items, .. } => {
                    out.push((id.clone(), label.clone().unwrap_or_default()));
                    for item in items {
                        out.push((item.id.clone(), item.label.clone()));
                    }
                }
                semio_framework_plugin::WindowMeasure::Slider { id, label, .. } | semio_framework_plugin::WindowMeasure::Number { id, label, .. } => out.push((id.clone(), label.clone().unwrap_or_default())),
                semio_framework_plugin::WindowMeasure::Toggle { id, label, text, .. } => out.push((id.clone(), label.clone().or_else(|| text.clone()).unwrap_or_default())),
            }
        }
        let mut out = Vec::new();
        let mut kinds: Vec<&String> = measures.keys().collect();
        kinds.sort();
        for kind in kinds {
            for measure in &measures[kind] {
                walk(measure, &mut out);
            }
        }
        out
    };
    let english = labelled(&app.window_measures(&view_for(semio_framework_ui_locale::Locale::En)).await);
    let german = labelled(&app.window_measures(&view_for(semio_framework_ui_locale::Locale::De)).await);
    assert!(!english.is_empty(), "both preview and flow windows publish measures");
    assert_eq!(english.len(), german.len(), "the two locales must paint the same measure tree");
    let untranslated: Vec<&str> = english
        .iter()
        .zip(&german)
        .filter(|((id, en), (de_id, de))| id == de_id && !en.is_empty() && en == de)
        .map(|((id, _), _)| id.as_str())
        .collect();
    assert!(untranslated.is_empty(), "window-chrome measure labels a German user still reads in English: {untranslated:?}");
}

#[semio_framework_async_macros::async_test]
async fn sun_measures_are_exposed_on_preview_windows() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let view = semio_framework_plugin::ViewModel {
        window_instances: vec![
            semio_framework_plugin::ViewWindowInstance { id: edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), window_kind_id: edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() },
            semio_framework_plugin::ViewWindowInstance { id: generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into(), window_kind_id: generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
        ],
        ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let measures = app.window_measures(&view).await;
    assert!(measures.contains_key(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW));
    assert!(measures.contains_key(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW));
}
