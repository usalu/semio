pub(crate) mod context {
    
    use super::super::*;
    use semio_framework_plugin::{ActionMeta, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, artifact_app_laws};
    
    /// ✏️ `Puzzle5dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<Puzzle5dPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<Puzzle5dPlayApp>` builds it.
    pub type Puzzle5dApp = VcsArtifactApp<EditorApp<Puzzle5dPlayApp>>;
    
    pub fn meta(actor: &str) -> ActionMeta {
        semio_framework_plugin::artifact_app_laws::meta(actor)
    }
    
    /// ✏️ Adapts `create_puzzle5d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `context::new_app_with_registry` still expects — framework test context gap, not
    /// modifiable here (`🧰️framework/**` is outside this packet's lease).
    pub fn puzzle5d_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_puzzle5d_app(), examples: Vec::new() }
    }
    
    /// 🧰️ The registry-backed app every test drives: the tool proof catalog joins the migrated declarations to live
    /// factories only with a manifest, so kind discipline and the utility contract are enforced exactly as in
    /// production.
    pub fn app() -> Puzzle5dTestApp {
        app_with_registry()
    }

    /// 🧹️ A registered app that closes itself to terminal-empty ownership when the test drops it, so a test that
    /// never reaches `close_app` still retires its stores exactly.
    pub struct Puzzle5dTestApp(Puzzle5dApp);

    impl std::ops::Deref for Puzzle5dTestApp {
        type Target = Puzzle5dApp;

        fn deref(&self) -> &Puzzle5dApp {
            &self.0
        }
    }

    impl std::ops::DerefMut for Puzzle5dTestApp {
        fn deref_mut(&mut self) -> &mut Puzzle5dApp {
            &mut self.0
        }
    }

    impl Drop for Puzzle5dTestApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                close_app(&mut self.0);
            }
        }
    }

    /// 🧰️ A registry-backed app so kind discipline (View actions must emit no operations) and the
    /// utility contract are enforced exactly as in production.
    pub fn app_with_registry() -> Puzzle5dTestApp {
        let mut app = semio_framework::io::resolve_ready(semio_framework_plugin::artifact_app_laws::new_app_with_registry::<EditorApp<Puzzle5dPlayApp>>(puzzle5d_app_manifest_for_tests));
        semio_framework::io::resolve_ready(app.bind_instance_id(1));
        Puzzle5dTestApp(app)
    }
    
    fn action_window_kind(action: &str) -> &'static str {
        if matches!(action, "setCamera2d" | "setLodMode" | "setGridSnapEnabled" | "setGridFactor" | "setSuggestionOffset" | "setFillCount" | "canvasPointerDown") {
            board2d::WINDOW_KIND_ID
        } else {
            world3d::WINDOW_KIND_ID
        }
    }
    
    pub fn window_view(kind: &str, id: &str) -> ViewModel {
        let mut window_instances = vec![
            ViewWindowInstance { id: board2d::WINDOW_KIND_ID.into(), window_kind_id: board2d::WINDOW_KIND_ID.into() },
            ViewWindowInstance { id: world3d::WINDOW_KIND_ID.into(), window_kind_id: world3d::WINDOW_KIND_ID.into() },
        ];
        if !window_instances.iter().any(|window| window.id == id) {
            window_instances.push(ViewWindowInstance { id: id.into(), window_kind_id: kind.into() });
        }
        ViewModel { window_instances, ..Default::default() }.for_window_instance(id).expect("puzzle5d test window roster")
    }
    
    fn action_meta(action: &str, args: Option<&Value>, window_id: Option<&str>) -> ActionMeta {
        let requested_window = args.and_then(|value| value.get("windowId").or_else(|| value.get("window"))).and_then(Value::as_str);
        let kind = requested_window.filter(|window| PUZZLE5D_PLAY_WINDOWS.contains(window)).unwrap_or_else(|| action_window_kind(action));
        let id = window_id.or(requested_window).unwrap_or(kind);
        ActionMeta { view_state: Some(window_view(kind, id)), ..meta("local") }
    }
    
    fn settle(app: &mut Puzzle5dApp, result: Result<InvocationResult, Fault>) -> Result<InvocationResult, Fault> {
        let mut result = result?;
        for _ in 0..1_048_576 {
            if !app.has_pending_typed_operations() {
                return Ok(result);
            }
            PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)?;
            semio_framework::io::resolve_ready(app.advance_typed_operation_publication())?;
            if let Some(page) = app.take_typed_operation_result_page(1) {
                if page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
                    return Err(Fault::from(String::from_utf8_lossy(page.bytes()).into_owned()));
                }
                app.acknowledge_typed_operation_result(page.token)?;
            }
            result.requested_effects.extend(app.take_typed_operation_effect());
            result.events.extend(app.take_typed_operation_event());
            let _ = semio_framework::io::resolve_ready(app.take_typed_operation_completion())?;
            if let Some(scope) = app.take_typed_operation_ui_scope() {
                result.ui_scope = scope;
            }
        }
        Err(Fault::from("puzzle5d test operation did not settle"))
    }
    
    /// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
    /// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
    /// `Self::Command` channel). Reconstructs the `Puzzle5dCommand` from the same
    /// `(action, args, window_id)` triple every pre-migration test already passed.
    pub fn dispatch(app: &mut Puzzle5dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
        let action_meta = action_meta(action, args, window_id);
        // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…/the six interaction verbs) stay on
        // `handle_action` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM added
        // interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
        // setInteractionGranularity to this reserved set.
        if matches!(
            action,
            "undo"
                | "redo"
                | "checkpoint"
                | "alternative"
                | "revertToCommand"
                | "historyFilter"
                | "noteShellCommand"
                | "copy"
                | "cut"
                | "paste"
                | "setActiveUtility"
                | "interactionSelect"
                | "interactionHover"
                | "clearSelection"
                | "selectAll"
                | "setSelectionMode"
                | "setInteractionGranularity"
        ) {
            let dsl_args = args.map(dsl::os_pack::json::to_dsl_value);
            let result = semio_framework::io::resolve_ready(app.handle_action(action, dsl_args.as_ref(), &action_meta)).and_then(|admitted| semio_framework::io::resolve_ready(semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted)));
            return settle(app, result);
        }
        let result = semio_framework::io::resolve_ready(app.dispatch_typed(Puzzle5dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &action_meta));
        settle(app, result)
    }
    
    /// 🎛️ Dispatches an app command from `window_id` with `utility_id` armed in that window, as a host whose
    /// window instance view state carries the armed utility does.
    pub fn dispatch_armed(app: &mut Puzzle5dApp, action: &str, args: Option<&Value>, window_id: &str, utility_id: &str) -> Result<InvocationResult, Fault> {
        let mut view = window_view(window_id, window_id);
        view.active_utility_by_window_id.insert(window_id.to_string(), utility_id.to_string());
        let action_meta = ActionMeta { view_state: Some(view), ..meta("local") };
        let result = semio_framework::io::resolve_ready(app.dispatch_typed(Puzzle5dCommand::from_action(action, args.cloned(), Some(window_id.to_string())), &action_meta));
        settle(app, result)
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
    /// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
    /// deleted `setSelection` action.
    pub fn select_id(app: &mut Puzzle5dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
        dispatch(app, "interactionSelect", Some(&dsl::json!({ "domainId": PUZZLE5D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None)
    }
    
    /// 🖼️ The rendered body, as a JSON string — every panel/window assertion greps this value.
    pub fn render_body(app: &mut Puzzle5dApp, body_key: &str) -> String {
        let tree = semio_framework::io::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render");
        let mut scene_json = None;
        let mut stack = vec![&tree.root];
        while let Some(node) = stack.pop() {
            if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
                let scene = match surface.doc_schema.as_str() {
                    schema if schema == <semio_framework_ui_scene::Board2dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA => {
                        serde_json::to_value(semio_framework_ui_scene::decode::<semio_framework_ui_scene::Board2dScene>(surface).expect("decode board scene"))
                    }
                    schema if schema == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA => {
                        serde_json::to_value(semio_framework_plugin::artifact_app_laws::built_surface_scene::<semio_framework_ui_scene::World3dScene>(node).expect("assemble world scene"))
                    }
                    _ => continue,
                }
                .expect("serialize scene");
                scene_json = Some(serde_json::json!({ "schema": surface.doc_schema, "scene": scene }).to_string());
                break;
            }
            stack.extend(node.children.iter());
        }
        let projected = artifact_app_laws::project_and_retire_fixture_tree(tree).expect("retire rendered node");
        scene_json.unwrap_or(projected)
    }
    
    pub fn render_window(app: &mut Puzzle5dApp, body_key: &str, window_id: &str) -> String {
        render_body(app, &format!("{body_key}:{window_id}"))
    }
    
    pub fn close_app(app: &mut Puzzle5dApp) {
        for _ in 0..1_048_576 {
            if app.close_terminal_is_empty() {
                return;
            }
            if PluginApp::close_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Puzzle 5D registered app close") == semio_framework_plugin::PluginCloseStep::Complete {
                break;
            }
        }
        assert!(app.close_terminal_is_empty(), "Puzzle 5D registered app close did not reach terminal-empty ownership");
    }
    
    //#region 🎚️Measures
    /// 🎚️ Every measure id in a window's chrome, groups included — the shared reader both window
    /// nodes' option laws assert their group/row ids against.
    pub fn measure_ids(measures: &[WindowMeasure]) -> Vec<String> {
        let mut ids = Vec::new();
        let mut stack: Vec<&WindowMeasure> = measures.iter().collect();
        while let Some(measure) = stack.pop() {
            match measure {
                WindowMeasure::Group { id, children, .. } => {
                    ids.push(id.clone());
                    stack.extend(children.iter());
                }
                WindowMeasure::Toggle { id, .. } | WindowMeasure::Slider { id, .. } | WindowMeasure::Select { id, .. } => ids.push(id.clone()),
                _ => {}
            }
        }
        ids
    }

    /// 🎚️ The pressed state one toggle row currently renders, at any depth.
    pub fn toggle_pressed(measures: &[WindowMeasure], wanted: &str) -> Option<bool> {
        let mut stack: Vec<&WindowMeasure> = measures.iter().collect();
        while let Some(measure) = stack.pop() {
            match measure {
                WindowMeasure::Group { children, .. } => stack.extend(children.iter()),
                WindowMeasure::Toggle { id, pressed, .. } if id == wanted => return Some(*pressed),
                _ => {}
            }
        }
        None
    }

    /// 🎚️ The value one slider row currently renders, at any depth.
    pub fn slider_value(measures: &[WindowMeasure], wanted: &str) -> Option<f64> {
        let mut stack: Vec<&WindowMeasure> = measures.iter().collect();
        while let Some(measure) = stack.pop() {
            match measure {
                WindowMeasure::Group { children, .. } => stack.extend(children.iter()),
                WindowMeasure::Slider { id, value, .. } if id == wanted => return Some(*value),
                _ => {}
            }
        }
        None
    }

    /// 🎚️ The value one select row currently renders, at any depth.
    pub fn select_value(measures: &[WindowMeasure], wanted: &str) -> Option<String> {
        let mut stack: Vec<&WindowMeasure> = measures.iter().collect();
        while let Some(measure) = stack.pop() {
            match measure {
                WindowMeasure::Group { children, .. } => stack.extend(children.iter()),
                WindowMeasure::Select { id, value, .. } if id == wanted => return Some(value.clone()),
                _ => {}
            }
        }
        None
    }

    /// 🎚️ The live chrome of ONE window instance, exactly as `ArtifactApp::window_measures` builds it
    /// — the read half of every "dispatch → config changes → measure reflects it" law.
    pub fn window_measures_of(app: &mut Puzzle5dApp, window_kind: &str) -> Vec<WindowMeasure> {
        let view = window_view(window_kind, window_kind);
        semio_framework::io::resolve_ready(PluginApp::window_measures(app, &view)).remove(window_kind).unwrap_or_default()
    }
    //#endregion 🎚️Measures

    pub fn projection_of(app: &Puzzle5dApp) -> Value {
        parse(&app.snapshot().expect("projection").0.to_string()).expect("snapshot JSON")
    }
    
    pub fn part_count(app: &Puzzle5dApp) -> usize {
        projection_of(app).get("parts").and_then(|value| value.as_array()).map_or(0, Vec::len)
    }
    
    pub fn first_part_id(app: &Puzzle5dApp) -> String {
        projection_of(app).get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).and_then(|part| part.get("id")).and_then(Value::as_str).expect("first part id").to_string()
    }
    
    /// 🎯️ Top-level utility tag of a `WindowMeasure::Group` by id, or `None` when the group is absent.
    pub fn measure_group_tag(measures: &[WindowMeasure], group_id: &str) -> Option<Option<String>> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Group { id, active_utility_id, .. } if id == group_id => Some(active_utility_id.clone()),
            _ => None,
        })
    }
    
    /// 🔢️ Finds one `WindowMeasure::Number` leaf anywhere in a measure tree, descending into groups.
    pub fn find_measure_number<'a>(measures: &'a [WindowMeasure], number_id: &str) -> Option<&'a WindowMeasure> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Number { id, .. } if id == number_id => Some(measure),
            WindowMeasure::Group { children, .. } => find_measure_number(children, number_id),
            _ => None,
        })
    }
}

use context::*;
use semio_framework::SET_ACTIVE_UTILITY_ACTION_ID;
use super::*;

use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, PluginApp, UiMenuRef};

#[test]
fn retained_publication_contracts_are_an_exact_nonempty_tool_bijection() {
    let exact = |contracts: &[ArtifactToolPublicationContract]| {
        let ids = contracts.iter().map(|contract| contract.tool_id).collect::<std::collections::BTreeSet<_>>();
        ids == PUZZLE5D_RETAINED_TOOL_IDS.iter().copied().collect()
            && ids.len() == contracts.len()
            && contracts.iter().all(|contract| !contract.lanes.is_empty() && (!contract.lanes.contains(&ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1))
    };
    let contracts = <Puzzle5dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert!(exact(contracts));
    assert!(!exact(&contracts[..contracts.len() - 1]));
    let mut duplicate = contracts.to_vec();
    let copied = duplicate[1];
    duplicate[0] = copied;
    assert!(!exact(&duplicate));
}

#[test]
fn retained_import_media_has_no_live_synchronous_fallback() {
    let source = include_str!("../../🦀️.rs");
    let production = source.split_once("//#region 🧪️UnitTests").map(|(production, _)| production).expect("production prefix");
    let fallback = production.split_once("fn import_media(_port:").and_then(|(_, suffix)| suffix.split_once("fn render(").map(|(fallback, _)| fallback)).expect("closed synchronous import callback");
    assert!(fallback.contains("Err(MediaError::NotImplemented)"));
    assert!(!fallback.contains("serde_json::from_str"));
    assert!(!fallback.contains("artifact_mutations"));
    let hostile = fallback.replace("Err(MediaError::NotImplemented)", "serde_json::from_str(\"{}\").map(|_| Emit::default()).map_err(|_| MediaError::NotImplemented)");
    assert!(hostile.contains("serde_json::from_str"));
}

fn complex_retained_route_is_cursorized(source: &str) -> bool {
    source.contains("\"applyBoardEvents\" => Box::new(Puzzle5dBoardEventsWork::default())")
        && source.contains("struct Puzzle5dBoardEventsWork")
        && source.contains("self.scan_one(source)?")
        && source.contains("Puzzle5dBoardEventsStage::FindMovePart")
        && source.contains("Puzzle5dBoardEventsStage::ScanEdge")
        && source.contains("Puzzle5dBoardEventsStage::ScanDeleteEdges")
        && source.contains("Puzzle5dBoardEventsStage::Brush")
        && source.contains("Puzzle5dBoardEventsStage::CloseBrush")
        && !source.contains("\"applyBoardEvents\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork")
}

#[test]
fn apply_board_events_hostile_static_law_rejects_the_old_one_grant_reducer() {
    let source = include_str!("../../🦀️.rs");
    assert!(complex_retained_route_is_cursorized(source));
    let direct = source
        .replace("\"applyBoardEvents\" => Box::new(Puzzle5dBoardEventsWork::default())", "\"applyBoardEvents\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))");
    assert!(!complex_retained_route_is_cursorized(&direct), "hostile old-reducer replacement must fail closed");
    for marker in ["self.scan_one(source)?", "Puzzle5dBoardEventsStage::FindMovePart", "Puzzle5dBoardEventsStage::ScanEdge", "Puzzle5dBoardEventsStage::ScanDeleteEdges", "Puzzle5dBoardEventsStage::CloseBrush"] {
        assert!(!complex_retained_route_is_cursorized(&source.replace(marker, "cursor-removed")), "missing cursor marker was falsely accepted: {marker}");
    }
}

fn focus_selection_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""focusSelection" => Box::new(Puzzle5dFocusSelectionWork::default())"#)
        && source.contains("Puzzle5dFocusSelectionStage::Selection")
        && source.contains("Puzzle5dFocusSelectionStage::Parts")
        && source.contains("Puzzle5dFocusSelectionStage::Publish")
        && source.contains("self.part_cursor += 1")
        && !source.contains(r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn focus_selection_hostile_static_law_rejects_whole_selection_reducers() {
    let source = include_str!("../../🦀️.rs");
    assert!(focus_selection_route_is_cursorized(source));
    let direct = source
        .replace(r#""focusSelection" => Box::new(Puzzle5dFocusSelectionWork::default())"#, r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#);
    assert!(!focus_selection_route_is_cursorized(&direct));
    for marker in ["Puzzle5dFocusSelectionStage::Selection", "Puzzle5dFocusSelectionStage::Parts", "Puzzle5dFocusSelectionStage::Publish", "self.part_cursor += 1"] {
        assert!(!focus_selection_route_is_cursorized(&source.replace(marker, "cursor-removed")), "missing focus cursor marker was falsely accepted: {marker}");
    }
}

fn window_owner_routes_are_exact(source: &str) -> bool {
    [
        "struct Puzzle5dWindowCommandWork",
        "window if PUZZLE5D_WINDOW_TOOL_IDS.contains(&window) => Box::new(Puzzle5dWindowCommandWork::new(window))",
        "fn bind_window_owners",
        "config_from_snapshot(self.window_config.as_ref())",
        "transient_from_snapshot(self.window_transient.as_ref())",
        "window_config_mutations",
        "window_transient",
        "addressed_config(view, window_after)",
        "addressed_transient(view, transient_after)",
    ]
    .into_iter()
    .all(|marker| source.contains(marker))
        && !source.contains("Puzzle5dConfigMutation::SetCamera2d")
        && !source.contains("Puzzle5dConfigMutation::SetBrushCandidateIndex")
        && !source.contains("Puzzle5dConfigMutation::SetEngagementInput")
        && !source.contains("Puzzle5dConfigMutation::SetGridFactor")
        && !source.contains("Puzzle5dConfigMutation::SetSun")
        && !source.contains("Puzzle5dEngagementAbortWork")
        && !source.contains("Puzzle5dEngagementSubmitWork")
        && !source.contains("Puzzle5dPrecomputeCommandWork")
        && !source.contains(r#""cycleBrushCandidate" | "registerBrushMesh" | "setFillCount" =>"#)
}

#[test]
fn window_owner_hostile_static_law_rejects_missing_owner_boundaries_and_app_config_leaks() {
    let source = include_str!("../../🦀️.rs");
    assert!(window_owner_routes_are_exact(source));
    for marker in [
        "struct Puzzle5dWindowCommandWork",
        "fn bind_window_owners",
        "config_from_snapshot(self.window_config.as_ref())",
        "transient_from_snapshot(self.window_transient.as_ref())",
        "addressed_config(view, window_after)",
        "addressed_transient(view, transient_after)",
    ] {
        assert!(!window_owner_routes_are_exact(&source.replacen(marker, "route-removed", 1)), "missing exact window-owner boundary was falsely accepted: {marker}");
    }
    let leaked = source.replace("Puzzle5dConfigMutation::Snapshot { config: shared_after }", "Puzzle5dConfigMutation::SetCamera2d");
    assert!(!window_owner_routes_are_exact(&leaked), "hostile app-config window leak must fail closed");
}

fn add_part_kind_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""addBrushPart" | "addPartKind" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id))"#)
        && source.contains("Puzzle5dAddBrushPartStage::Catalog")
        && source.contains("Puzzle5dAddBrushPartStage::Grips")
        && source.contains("Puzzle5dAddBrushPartStage::Target")
        && source.contains("Puzzle5dAddBrushPartStage::Create")
        && source.contains("Puzzle5dAddBrushPartStage::Connect")
        && !source.contains(r#""addPartKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn add_part_kind_hostile_static_law_rejects_old_brush_reducer_and_missing_cursors() {
    let source = include_str!("../../🦀️.rs");
    assert!(add_part_kind_route_is_cursorized(source));
    let direct = source.replace(
        r#""addBrushPart" | "addPartKind" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id))"#,
        r#""addBrushPart" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id)),
            "addPartKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
    );
    assert!(!add_part_kind_route_is_cursorized(&direct));
}

fn kind_weight_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""setPartKindWeight" | "setGripKindWeight" => Box::new(Puzzle5dKindWeightWork::new(tool_id))"#)
        && source.contains("Puzzle5dKindWeightStage::Catalog")
        && source.contains("Puzzle5dKindWeightStage::InferParts")
        && source.contains("Puzzle5dKindWeightStage::InferGrips")
        && source.contains("Puzzle5dKindWeightStage::Validate")
        && source.contains("Puzzle5dKindWeightStage::SumOthers")
        && source.contains("Puzzle5dKindWeightStage::Build")
        && source.contains("Puzzle5dConfigMutation::SetObjectKindWeights")
        && source.contains("Puzzle5dConfigMutation::SetVortexKindWeights")
        && !source.contains(r#""setPartKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn kind_weight_hostile_static_law_rejects_whole_normalizer_and_missing_cursors() {
    let source = include_str!("../../🦀️.rs");
    assert!(kind_weight_route_is_cursorized(source));
    let direct = source.replace(
        r#""setPartKindWeight" | "setGripKindWeight" => Box::new(Puzzle5dKindWeightWork::new(tool_id))"#,
        r#""setPartKindWeight" | "setGripKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
    );
    assert!(!kind_weight_route_is_cursorized(&direct));
    assert!(!source.contains("puzzle5d_normalize_kind_weight_group(self.weights"));
}

fn engagement_submit_route_is_cursorized(source: &str) -> bool {
    source.contains(r#"const PUZZLE5D_WINDOW_TOOL_IDS: &[&str] = &["#) && PUZZLE5D_WINDOW_TOOL_IDS.iter().all(|tool| source.contains(&format!("    \"{tool}\",\n")) || source.contains(&format!("    \"{tool}\",\r\n")))
        && source.contains("window if PUZZLE5D_WINDOW_TOOL_IDS.contains(&window) => Box::new(Puzzle5dWindowCommandWork::new(window))")
        && source.contains("transient_from_snapshot(self.window_transient.as_ref())")
        && source.contains("addressed_transient(view, transient_after)")
        && source.contains("EphemeralEmit { window_transient, ..Default::default() }")
        && !source.contains("Puzzle5dConfigMutation::SetEngagementInput")
        && !source.contains(r#""engagementSubmit" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn engagement_submit_hostile_static_law_rejects_old_reducer_and_missing_transfers() {
    let source = include_str!("../../🦀️.rs");
    assert!(engagement_submit_route_is_cursorized(source));
    for marker in [
        "transient_from_snapshot(self.window_transient.as_ref())",
        "addressed_transient(view, transient_after)",
        "EphemeralEmit { window_transient, ..Default::default() }",
    ] {
        assert!(!engagement_submit_route_is_cursorized(&source.replacen(marker, "route-removed", 1)), "missing engagement submit marker was falsely accepted: {marker}");
    }
    let leaked = source.replace("Puzzle5dConfigMutation::Snapshot { config: shared_after }", "Puzzle5dConfigMutation::SetEngagementInput");
    assert!(!engagement_submit_route_is_cursorized(&leaked));
}

fn world_relocate_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""worldRelocate" => Box::new(Puzzle5dWorldRelocateWork::default())"#)
        && source.contains("struct Puzzle5dWorldRelocateWork")
        && source.contains("Puzzle5dWorldRelocateStage::SourcePart")
        && source.contains("Puzzle5dWorldRelocateStage::ExistingFasteners")
        && source.contains("Puzzle5dWorldRelocateStage::CandidatePart")
        && source.contains("Puzzle5dWorldRelocateStage::CandidateGrip")
        && source.contains("Puzzle5dWorldRelocateStage::PublishFastener")
        && source.contains("PUZZLE5D_RELOCATE_GRIPS_PER_PART")
        && !source.contains(r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn world_relocate_hostile_static_law_rejects_whole_proximity_scans() {
    let source = include_str!("../../🦀️.rs");
    assert!(world_relocate_route_is_cursorized(source));
    let direct =
        source.replace(r#""worldRelocate" => Box::new(Puzzle5dWorldRelocateWork::default())"#, r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#);
    assert!(!world_relocate_route_is_cursorized(&direct), "hostile old-reducer replacement must fail closed");
    for marker in ["Puzzle5dWorldRelocateStage::ExistingFasteners", "Puzzle5dWorldRelocateStage::CandidatePart", "Puzzle5dWorldRelocateStage::CandidateGrip", "Puzzle5dWorldRelocateStage::PublishFastener", "PUZZLE5D_RELOCATE_GRIPS_PER_PART"] {
        assert!(!world_relocate_route_is_cursorized(&source.replace(marker, "cursor-removed")), "missing world-relocate marker was falsely accepted: {marker}");
    }
}

//#region 🔖️Rendering
#[semio_framework_async_macros::async_test]
async fn renders_paired_board_and_world_scenes() {
    let mut app = app();
    assert!(render_body(&mut app, board2d::BODY_KEY).contains("board-2d"));
    assert!(render_body(&mut app, world3d::BODY_KEY).contains("world-3d"));
}

#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_the_concrete_forest_document() {
    let app = app();
    assert_eq!(projection_of(&app).get("schema").and_then(|value| value.as_str()), Some(PUZZLE5D_SCHEMA));
    assert!(part_count(&app) > 0, "the concrete-forest default document ships with parts");
}

#[semio_framework_async_macros::async_test]
async fn document_panel_renders() {
    let mut app = app();
    assert!(!render_body(&mut app, artifact_panel::BODY_KEY).is_empty());
}
//#endregion 🔖️Rendering

//#region 🔖️ContextMenu
/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the selection context menu stays a shallow,
/// disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall of rows,
/// and the known destructive `deleteSelection` action stays the trailing group's last item.
#[semio_framework_async_macros::async_test]
async fn context_menu_is_grouped_and_keeps_delete_selection_last() {
    let mut app = app_with_registry();
    let part_id = first_part_id(&app);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &part_id).expect("select part");
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget { surface_id: world3d::WINDOW_KIND_ID.into(), kind: "world3d".into(), hits: vec![], selection: vec![ContextMenuSelectionGroup { domain: "part".into(), ids: vec![part_id] }], text: None }),
        window_instance_id: None,
        point: None,
    };
    let menu = semio_framework::io::resolve_ready(app.context_menu(&request, &Default::default()));
    assert!(menu.len() <= 9, "top-level context menu should stay progressively disclosed: {menu:?}");
    let last = menu.last().expect("selection context menu should not be empty");
    let last_is_destructive_leaf = last.action.as_deref() == Some("deleteSelection") && last.destructive == Some(true);
    let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.action.as_deref() == Some("deleteSelection") && child.destructive == Some(true));
    assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must stay last: {menu:?}");
}
//#endregion 🔖️ContextMenu

//#region 🔖️Pack
/// 📦️ `Puzzle5dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
/// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
/// `serde_json::Value` bridge impls), reusing the default concrete-forest fixture.
#[semio_framework_async_macros::async_test]
async fn puzzle5d_play_projection_pack_round_trips() {
    let app = app();
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
}
//#endregion 🔖️Pack

//#region 🔖️Operations
#[semio_framework_async_macros::async_test]
async fn set_active_example_swaps_the_document_and_undo_restores_it() {
    let mut app = app();
    let loaded = part_count(&app);
    assert!(loaded > 0);
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": "" })), None).expect("empty");
    assert_eq!(part_count(&app), 0, "empty example clears the parts");
    semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
    assert_eq!(part_count(&app), loaded, "undo restores the concrete-forest parts");
    semio_framework::io::resolve_ready(app.handle_action("redo", None, &meta("local"))).expect("redo");
    assert_eq!(part_count(&app), 0);
}

#[semio_framework_async_macros::async_test]
async fn patch_fastener_updates_transform_offsets_and_undoes() {
    let mut app = app();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin (has fasteners)");
    let projection = projection_of(&app);
    let fastener_id = projection["fasteners"][0]["id"].as_str().expect("seeded fastener").to_string();
    dispatch(&mut app, "patchFastener", Some(&dsl::json!({ "fastenerId": fastener_id, "field": "gap", "value": 2.5 })), None).expect("patch gap");
    let after = projection_of(&app);
    let fastener = after["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
    assert_eq!(fastener["gap"], 2.5);
    assert_eq!(fastener["shift"], 0.0);
    dispatch(&mut app, "patchFastener", Some(&dsl::json!({ "fastenerId": fastener_id, "field": "rotation", "value": 30.0 })), None).expect("patch rotation");
    let after2 = projection_of(&app);
    let fastener2 = after2["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
    assert_eq!(fastener2["gap"], 2.5, "earlier gap edit must survive a later rotation edit");
    assert_eq!(fastener2["rotation"], 30.0);
    semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
    let undone = projection_of(&app);
    let fastener3 = undone["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
    assert_eq!(fastener3["rotation"], 0.0, "undo restores the pre-rotation-edit value");
    assert_eq!(fastener3["gap"], 2.5, "undo of rotation edit must not also revert the earlier gap edit");
}
//#endregion 🔖️Operations

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `Puzzle5dMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s. Deliberately
/// dispatches through a standalone typed `Puzzle5dStore` — NOT through `Puzzle5dPlayApp`/
/// `Puzzle5dPlaySnapshot` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app's real
/// `ArtifactApp` still uses) — since `Puzzle5dMutation`'s canonical `Mutation<Puzzle5dSnapshot>`
/// impl (not its `Mutation<Value>` bridge impl) is what the CW7 law is about.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::standards::v1::subsets::any::schema::mutations::binary::{close_puzzle5d_store, puzzle5d_store};
    use crate::{PUZZLE_5D_SCHEMA, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::create_document_envelope;

    let mut store = puzzle5d_store(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", Puzzle5dSnapshot::default(), None)).await.expect("store");
    let part = Puzzle5dPart { id: "p1".into(), part_kind: None, anchor: Default::default(), part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() };
    semio_framework::io::resolve_ready(store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_part(part, None)], description: None })).expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle5dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework::io::resolve_ready(semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle5dSnapshot, Puzzle5dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())));
    close_puzzle5d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
//#endregion 🔖️CommandEnvelopeTests

//#region 🔖️Clipboard
#[semio_framework_async_macros::async_test]
async fn copy_emits_clipboard_fragment_for_the_closed_selection() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let first_part_id = first_part_id(&app);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
    let result = app.handle_action("copy", None, &meta("local")).await.expect("copy");
    assert!(result.mutations.is_empty(), "copy must not record an undo entry");
    assert_eq!(result.requested_effects.len(), 1);
    let Effect::ClipboardWrite { fragment } = &result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
    assert_eq!(fragment.source_app, PUZZLE5D_PLAY_APP_ID);
    let fragment_value: serde_json::Value = serde_json::from_str(&fragment.dsl_text).expect("fragment dsl_text is JSON");
    assert_eq!(fragment_value["parts"].as_array().expect("parts").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn copy_with_no_selection_is_a_benign_no_operation() {
    let mut app = app();
    let result = app.handle_action("copy", None, &meta("local")).await.expect("copy");
    assert!(result.mutations.is_empty());
    assert!(result.requested_effects.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn cut_removes_selected_part_and_undo_restores_it() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let before_count = part_count(&app);
    let first_part_id = first_part_id(&app);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
    let result = app.handle_action("cut", None, &meta("local")).await.expect("cut");
    assert_eq!(result.requested_effects.len(), 1, "cut must also copy to the clipboard");
    assert_eq!(part_count(&app), before_count - 1);
    let after = projection_of(&app);
    assert!(!after["parts"].as_array().unwrap().iter().any(|part| part["id"] == first_part_id));
    semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
    assert_eq!(part_count(&app), before_count, "one undo restores the cut part as a single edit");
}

#[semio_framework_async_macros::async_test]
async fn paste_materializes_fragment_parts_at_original_anchor_with_fresh_ids() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let projection = projection_of(&app);
    let first_part_id = first_part_id(&app);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
    let copy_result = app.handle_action("copy", None, &meta("local")).await.expect("copy");
    let Effect::ClipboardWrite { fragment } = &copy_result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
    let before_count = part_count(&app);
    let before_ids: HashSet<String> = projection["parts"].as_array().unwrap().iter().map(|part| part["id"].as_str().unwrap_or_default().to_string()).collect();
    let paste_args: dsl::DslValue = serde_json::json!({ "fragment": fragment, "anchor": "original", "position": [10.0, 0.0, 0.0] }).into();
    app.handle_action("paste", Some(&paste_args), &meta("local")).await.expect("paste");
    assert_eq!(part_count(&app), before_count + 1);
    let after = projection_of(&app);
    let pasted_parts: Vec<&Value> = after["parts"].as_array().unwrap().iter().filter(|part| !before_ids.contains(part["id"].as_str().unwrap_or_default())).collect();
    assert_eq!(pasted_parts.len(), 1);
    // "original" anchor uses the raw position override verbatim as the 2D delta.
    let original_x = projection["parts"][0]["2d"]["x"].as_f64().unwrap_or(0.0);
    assert_eq!(pasted_parts[0]["2d"]["x"].as_f64().unwrap(), original_x + 10.0);
    semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
    assert_eq!(part_count(&app), before_count, "one undo removes the whole pasted fragment");
}

#[semio_framework_async_macros::async_test]
async fn paste_with_no_fragment_arg_is_a_benign_no_operation() {
    let mut app = app();
    let before_count = part_count(&app);
    let result = app.handle_action("paste", None, &meta("local")).await.expect("paste");
    assert!(result.mutations.is_empty());
    assert_eq!(part_count(&app), before_count);
}
//#endregion 🔖️Clipboard

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn app_definition_has_the_paired_windows() {
    let definition = create_puzzle5d_app();
    let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
    assert!(ids.contains(&board2d::WINDOW_KIND_ID) && ids.contains(&world3d::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn window_kind_actions_scope_transform_to_3d_only() {
    let definition = create_puzzle5d_app();
    let resolve = |window_id: &str| -> Vec<String> {
        let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
        semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
    };
    let board = resolve(board2d::WINDOW_KIND_ID);
    let world = resolve(world3d::WINDOW_KIND_ID);
    for transform_operation in ["translateSelection", "rotateSelection", "scaleSelection", "worldRelocate", "setCamera3d"] {
        assert!(world.contains(&transform_operation.to_string()), "3D must expose {transform_operation}");
        assert!(!board.contains(&transform_operation.to_string()), "2D must NOT expose {transform_operation}");
    }
    assert!(board.contains(&"applyBoardEvents".to_string()), "2D must expose applyBoardEvents");
    assert!(!world.contains(&"applyBoardEvents".to_string()), "3D must NOT expose applyBoardEvents");
    for shared in ["addBrushPart", "deleteSelection"] {
        assert!(board.contains(&shared.to_string()) && world.contains(&shared.to_string()), "{shared} stays on both windows");
    }
}

/// 📑️ The three declared panel tabs must survive the `panel_tab_def` stitch. Asserts PRESENCE
/// only — the framework injects tabs of its own, so a total count would be brittle.
#[semio_framework_async_macros::async_test]
async fn app_definition_declares_its_three_panel_tabs() {
    let definition = create_puzzle5d_app();
    let body_keys: Vec<&str> = definition.panel_tabs.iter().filter_map(|tab| tab.body_key.as_deref()).collect();
    for body_key in [artifact_panel::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY] {
        assert!(body_keys.contains(&body_key), "panel tab {body_key} must be declared, got {body_keys:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn window_engagements_cover_both_windows() {
    let mut app = app();
    let engagements = semio_framework::io::resolve_ready(app.window_engagements(&Default::default()));
    assert!(engagements.contains_key(board2d::WINDOW_KIND_ID));
    assert!(engagements.contains_key(world3d::WINDOW_KIND_ID));
}

/// 🎯️ Every action id `dispatch_puzzle5d_action` matches on must have a `Puzzle5dCommand` variant
/// under the SAME literal — the two lists are the whole app's dispatch contract and drift between
/// them is silent. (Deliberately NOT the framework's own
/// `assert_declared_actions_bridge_to_commands`, which probes `command_from_action`, the
/// string-dispatch path this app does not implement — its commands carry an opaque `args: Value`,
/// see the `🔖️Puzzle5dCommand` macro.)
#[semio_framework_async_macros::async_test]
async fn every_dispatched_action_bridges_to_a_command() {
    for action in [
        "exportFixture",
        "importFixture",
        "openImportFixture",
        "openAddPartDialog",
        "setActiveExample",
        "selectSameKindSelection",
        "addNode",
        "addPartKind",
        "deleteSelection",
        "duplicateSelection",
        "setSelectionFlag",
        "patchPart",
        "patchGrip",
        "patchFastener",
        "setCamera",
        "setCamera2d",
        "setCamera3d",
        "focusSelection",
        "toggleSun",
        "setSunAzimuth",
        "setSunElevation",
        "setSunIntensity",
        "setLodMode",
        "setGridSnapEnabled",
        "setGridFactor",
        "addBrushPart",
        "cycleBrushCandidate",
        "registerBrushMesh",
        "setBrushPlacementContactTolerance",
        "setPartKindWeight",
        "setGripKindWeight",
        "engagementControlSelect",
        "setSuggestionOffset",
        "setFillCount",
        "engagementInput",
        "engagementSubmit",
        "engagementAbort",
        "translateSelection",
        "rotateSelection",
        "scaleSelection",
        "worldRelocate",
        "applyBoardEvents",
        "worldPointerDown",
        "canvasPointerDown",
    ] {
        assert_eq!(Puzzle5dCommand::from_action(action, None, None).action_id(), action, "dispatched action {action} must have a Puzzle5dCommand variant");
    }
}
//#endregion 🔖️Manifest

//#region 🧰️ Window Actions & Utilities contract
#[semio_framework_async_macros::async_test]
async fn add_part_kind_materializes_the_declared_kind_default() {
    // 📝️ P1 arg form: addPartKind with no args materializes the declared `partKind` default and adds a part.
    // 🗨️ The default is the FIRST LIVE catalog kind (`puzzle5d_default_part_kind`), never the literal
    // "Part" this select used to hardcode — a literal no catalog declares could not add a real kind.
    let expected_default = puzzle5d_default_part_kind(&puzzle5d_part_kind_options());
    assert!(!expected_default.is_empty() && expected_default != "Part", "the declared default is a real catalog kind, got {expected_default:?}");
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": "" })), None).expect("empty");
    let before = part_count(&app);
    let result = dispatch(&mut app, "addPartKind", None, None).expect("addPartKind");
    assert!(!result.mutations.is_empty(), "addPartKind is a Mutation that emits mutations");
    assert_eq!(part_count(&app), before + 1, "the materialized default kind adds exactly one part");
    let projection = projection_of(&app);
    let kind = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.last()).and_then(|part| part.get("partKind")).and_then(Value::as_str);
    assert_eq!(kind, Some(expected_default.as_str()), "the declared partKind default was materialized host-side");
}

#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_ops_and_no_history_entry() {
    // 🧰️ Switching utilities is the framework View action: no document operations, no undo entry, no re-emitted effect.
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&dsl::json!({ "utilityId": "brush" })), None).expect("switch utility");
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
    assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
}

#[semio_framework_async_macros::async_test]
async fn exact_window_cameras_isolate_render_and_reload_without_document_or_app_config_changes() {
    let mut app = Box::new(app_with_registry());
    let mut reopened = Box::new(app_with_registry());
    let window_a = "puzzle5d-board-a";
    let window_b = "puzzle5d-board-b";
    let view_a = window_view(board2d::WINDOW_KIND_ID, window_a);
    let view_b = window_view(board2d::WINDOW_KIND_ID, window_b);
    let before = projection_of(&app);
    let app_config_before = app.config_pack().await.expect("app config before window publications");
    let result_a = dispatch(&mut app, "setCamera2d", Some(&dsl::json!({ "camera": { "x": 12.5, "y": -6.5, "zoom": 3.5 } })), Some(window_a)).expect("setCamera2d a");
    let result_b = dispatch(&mut app, "setCamera2d", Some(&dsl::json!({ "camera": { "x": -42.5, "y": 7.5, "zoom": 1.5 } })), Some(window_b)).expect("setCamera2d b");
    assert!(result_a.mutations.is_empty() && result_b.mutations.is_empty());
    assert_eq!(projection_of(&app), before);
    let app_config_after = app.config_pack().await.expect("app config after window publications");
    assert_eq!((app_config_after.pack, app_config_after.spr), (app_config_before.pack, app_config_before.spr));
    let board_a = render_window(&mut app, board2d::BODY_KEY, window_a);
    let board_b = render_window(&mut app, board2d::BODY_KEY, window_b);
    assert!(board_a.contains("12.5") && board_a.contains("-6.5"));
    assert!(board_b.contains("-42.5") && board_b.contains("7.5"));
    assert_eq!(app.window_config_generation(&view_a).await.expect("window a generation"), Some(1));
    assert_eq!(app.window_config_generation(&view_b).await.expect("window b generation"), Some(1));
    let packs = app.window_config_packs().await.expect("two exact window packs");
    assert_eq!(packs.len(), 2);
    for pack in packs {
        reopened.load_window_config_pack(pack).await.expect("reload exact window pack");
    }
    assert_eq!(render_window(&mut reopened, board2d::BODY_KEY, window_a), board_a);
    assert_eq!(render_window(&mut reopened, board2d::BODY_KEY, window_b), board_b);
    close_app(&mut reopened);
    close_app(&mut app);
    eprintln!("[DEBUG] two Puzzle 5D board windows published and rendered independent cameras, preserved document and app config, reloaded both exact persisted partitions, and closed their registered apps");
}

#[semio_framework_async_macros::async_test]
async fn exact_window_transient_isolated_abort_and_reload_reset_through_registered_app() {
    let mut app = Box::new(app_with_registry());
    let mut reopened = Box::new(app_with_registry());
    let window_a = "puzzle5d-transient-a";
    let window_b = "puzzle5d-transient-b";
    let view_a = window_view(board2d::WINDOW_KIND_ID, window_a);
    let view_b = window_view(board2d::WINDOW_KIND_ID, window_b);
    dispatch(&mut app, "engagementInput", Some(&dsl::json!({ "window": board2d::WINDOW_KIND_ID, "value": "fill" })), Some(window_a)).expect("window a engagement input");
    let transient_a = app.window_transient_snapshot(&view_a).expect("window a transient").expect("window a owner");
    let transient_b = app.window_transient_snapshot(&view_b).expect("window b transient").expect("window b owner");
    assert_eq!(transient_a.get::<window_ownership::Puzzle5dBoardWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some("fill"));
    assert_eq!(transient_b.get::<window_ownership::Puzzle5dBoardWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    dispatch(&mut app, "engagementAbort", Some(&dsl::json!({ "window": board2d::WINDOW_KIND_ID })), Some(window_a)).expect("abort window a engagement");
    let aborted = app.window_transient_snapshot(&view_a).expect("aborted transient").expect("aborted owner");
    assert_eq!(aborted.get::<window_ownership::Puzzle5dBoardWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    dispatch(&mut app, "engagementInput", Some(&dsl::json!({ "window": board2d::WINDOW_KIND_ID, "value": "brush" })), Some(window_a)).expect("window a second engagement input");
    let submitted = dispatch(&mut app, "engagementSubmit", Some(&dsl::json!({ "window": board2d::WINDOW_KIND_ID, "value": "brush" })), Some(window_a)).expect("submit window a engagement");
    assert!(submitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { window_id, .. } if window_id == window_a)));
    dispatch(&mut app, "engagementInput", Some(&dsl::json!({ "window": board2d::WINDOW_KIND_ID, "value": "draft" })), Some(window_a)).expect("window a third engagement input");
    let reset = reopened.window_transient_snapshot(&view_a).expect("reopened transient").expect("reopened owner");
    assert_eq!(reset.get::<window_ownership::Puzzle5dBoardWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    assert_eq!(app.window_transient_generation(&view_a).expect("window a transient generation"), Some(5));
    assert_eq!(app.window_transient_generation(&view_b).expect("window b transient generation"), Some(0));
    close_app(&mut reopened);
    close_app(&mut app);
    eprintln!("[DEBUG] Puzzle 5D transient engagement stayed exact-window isolated, abort cleared only its owner, reload reset ephemeral state, and both registered apps reached close");
}

#[semio_framework_async_macros::async_test]
async fn engagements_expose_no_utility_switch_options_for_either_window() {
    // 🧰️ select/brush/fill switching lives only on the framework utility bar; neither the 2D nor the 3D
    // engagement HUD may duplicate it as options.
    let mut app = app();
    let engagements = semio_framework::io::resolve_ready(app.window_engagements(&Default::default()));
    for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
        assert!(engagements.get(window).expect("engagement").options.is_none(), "the {window} engagement must not re-expose utility switching as options");
    }
}

/// 🎯️ D-3 follow-up, re-based on the fill TOOL: the brush placement picker is a `"brush"`-tagged
/// `WindowMeasure::Group` in each window's `window_measures` (surfaced by `partition_window_measures` only for
/// its active utility) and the fill count is a MODE-LEVEL tool measure (`tool_measures`), never a
/// `WindowEngagementControl` on the HUD and never a window rail group — for both the 2D and 3D windows.
#[semio_framework_async_macros::async_test]
async fn fill_is_tool_options_and_brush_is_utility_options_never_engagement_controls() {
    let labels = puzzle5d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
    // 🪣️ Fill TOOL: the count entry and the distribution trees live in the tool options rail, NOT in either
    // window's measures and NOT on the engagement HUD.
    let fill_runtime = Puzzle5dRuntime { fill_count: 3, ..Default::default() };
    let fill_scene = Puzzle5dScene { document: default_document(), runtime: fill_runtime, active_utility: fill_tool::TOOL_ID.into(), interaction: Default::default() };
    let tool_measures = fill_tool::measures(&fill_scene, labels, None);
    assert!(matches!(find_measure_number(&tool_measures, "puzzle5d-fill-count"), Some(WindowMeasure::Number { value, .. }) if *value == 3.0), "the fill tool options carry the count entry");
    for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
        let measures = if window == board2d::WINDOW_KIND_ID { board2d::window_measures(&fill_scene, labels) } else { world3d::window_measures(&fill_scene, labels) };
        assert!(find_measure_number(&measures, "puzzle5d-fill-count").is_none(), "{window} must not duplicate the fill count as a window measure");
        assert_eq!(measure_group_tag(&measures, "puzzle5d-play-utility-options-fill"), None, "{window} must no longer carry a fill Utility Options rail group");
        let fill_hud = edit::puzzle5d_engagement(&fill_scene, window, labels, None);
        assert!(fill_hud.control.is_none() && fill_hud.controls.is_none(), "{window} fill engagement HUD must no longer carry the relocated control");
    }
    // 🖌️ Brush utility: with no candidates to place, the "brush"-tagged group still surfaces (matching the
    // old gate), and the engagement HUD is likewise bare.
    let brush_scene = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: "brush".into(), interaction: Default::default() };
    for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
        let measures = if window == board2d::WINDOW_KIND_ID { board2d::window_measures(&brush_scene, labels) } else { world3d::window_measures(&brush_scene, labels) };
        assert_eq!(measure_group_tag(&measures, "puzzle5d-play-utility-options-brush"), Some(Some("brush".into())), "{window} brush Utility Options surfaces even without candidates");
        let brush_hud = edit::puzzle5d_engagement(&brush_scene, window, labels, None);
        assert!(brush_hud.control.is_none() && brush_hud.controls.is_none(), "{window} brush engagement HUD must no longer carry the relocated control");
    }
}

/// ♾️ The fill count is an unbounded `Number` defaulting to 100 — the deleted `PUZZLE5D_FILL_COUNT_MAX`
/// pin must not come back anywhere on the path from runtime default to rendered tool measure.
#[semio_framework_async_macros::async_test]
async fn fill_count_entry_is_unbounded_and_defaults_to_one_hundred() {
    let labels = puzzle5d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
    assert_eq!(PUZZLE5D_DEFAULT_FILL_COUNT, 100);
    assert_eq!(Puzzle5dRuntime::default().fill_count, 100);
    assert_eq!(Puzzle5dConfig::default().fill_count, 100);

    let default_scene = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: fill_tool::TOOL_ID.into(), interaction: Default::default() };
    let measures = fill_tool::measures(&default_scene, labels, None);
    assert!(matches!(find_measure_number(&measures, "puzzle5d-fill-count"), Some(WindowMeasure::Number { value, .. }) if *value == 100.0));

    let large_scene = Puzzle5dScene { runtime: Puzzle5dRuntime { fill_count: 5_000, ..Default::default() }, ..default_scene };
    let measures = fill_tool::measures(&large_scene, labels, None);
    let Some(WindowMeasure::Number { value, min, max, step, .. }) = find_measure_number(&measures, "puzzle5d-fill-count") else { panic!("fill count entry") };
    assert_eq!(*value, 5_000.0);
    assert_eq!(*min, Some(0.0));
    assert_eq!(*max, None, "the fill count must carry no ceiling");
    assert_eq!(*step, Some(1.0));
}

/// 🛠️ LAW: fill is a first-class framework TOOL in the manifest — one `.tool(...)` registration carrying its
/// `ToolRunDefinition`, listed in the edit mode's tools, and NO fill utility rail left behind in either window.
#[semio_framework_async_macros::async_test]
async fn fill_is_registered_as_a_mode_tool_and_no_longer_as_a_utility() {
    let definition = create_puzzle5d_app();
    let tool = definition.tools.iter().find(|tool| tool.id == fill_tool::TOOL_ID).expect("the manifest registers a fill tool");
    assert!(tool.run.is_some(), "the fill tool carries its tool run declaration");
    assert!(!definition.utilities.iter().any(|utility| utility.id == fill_tool::TOOL_ID), "fill is no longer a utility");
    let mode = definition.modes.iter().find(|mode| mode.id == edit::PUZZLE5D_PLAY_MODE_EDIT).expect("the edit mode");
    assert!(mode.tools.iter().any(|tool| tool.as_str() == fill_tool::TOOL_ID), "the edit mode lists the fill tool");
    for window in definition.window_kinds.iter() {
        assert!(!window.utilities.iter().any(|utility| utility.as_str() == fill_tool::TOOL_ID), "{} must not bind fill as a utility", window.id);
    }
}

/// ⌨️ LAW: the engagement placeholder advertises EXACTLY the verbs `engagement_submit` parses — an advertised
/// verb no arm implements is a dead promise (the defect 3d still carries with `pick`/`rectangle`/`lasso`).
#[semio_framework_async_macros::async_test]
async fn engagement_placeholder_advertises_exactly_the_parsed_verbs() {
    let labels = puzzle5d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
    let scene = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: "select".into(), interaction: Default::default() };
    for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
        let hud = edit::puzzle5d_engagement(&scene, window, labels, None);
        let placeholder = hud.input.as_ref().expect("engagement input").placeholder.clone().expect("placeholder");
        assert_eq!(placeholder, crate::editor::puzzle5d::commands::engagement_submit::PUZZLE5D_ENGAGEMENT_VERBS.join(", "), "{window} placeholder is derived from the parsed verb list");
        assert!(hud.input.as_ref().and_then(|input| input.on_repeat_last.clone()).is_some(), "{window} offers repeat-last");
    }
}

#[semio_framework_async_macros::async_test]
async fn engagement_submit_switches_utility_via_host_effect_for_both_windows() {
    // 🧰️ Reconciled dual entry point: the engagement token drives the same host-owned utility switch, once per window.
    let mut app = app();
    let result = dispatch(&mut app, "engagementSubmit", Some(&dsl::json!({ "window": world3d::WINDOW_KIND_ID, "value": "brush" })), None).expect("submit");
    let windows: Vec<&str> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::SetActiveUtility { window_id, utility_id } if utility_id == "brush" => Some(window_id.as_str()),
            _ => None,
        })
        .collect();
    assert!(windows.contains(&board2d::WINDOW_KIND_ID) && windows.contains(&world3d::WINDOW_KIND_ID), "brush switch is pushed to both windows, got {windows:?}");
}

#[semio_framework_async_macros::async_test]
async fn gumball_translate_drag_coalesces_into_one_edit() {
    // 🌀️ Coalescing regression: three translate ticks with the same key are ONE undoable edit.
    let mut app = app();
    let part_id = first_part_id(&app);
    let origin_x = |app: &Puzzle5dApp| -> f64 {
        projection_of(app)
            .get("parts")
            .and_then(Value::as_array)
            .and_then(|parts| parts.iter().find(|part| part.get("id").and_then(Value::as_str) == Some(part_id.as_str())).cloned())
            .and_then(|part| part.pointer("/3d/origin/0").and_then(Value::as_f64))
            .unwrap_or(0.0)
    };
    let start = origin_x(&app);
    for dx in [1.0, 2.0, 3.0] {
        dispatch(&mut app, "translateSelection", Some(&dsl::json!({ "ids": [part_id], "dx": dx, "dy": 0.0, "dz": 0.0 })), None).expect("drag tick");
    }
    assert!((origin_x(&app) - start - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
    semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
    assert!((origin_x(&app) - start).abs() < 1e-9, "one undo restores the whole coalesced gumball drag");
}
//#endregion 🧰️ Window Actions & Utilities contract

//#region 🔖️KitInPort
#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_dispatches_the_exact_factory_and_applies_canonical_output() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let fragment = serde_json::json!({
        "schema": "manifest",
        "objectKinds": [{
            "id": "retained-capsule",
            "name": "retained-capsule",
            "label": "Retained Capsule",
            "meshUrl": "/mesh/retained-capsule.glb",
            "vortices": [{ "id": "v0", "vortexKind": "retained-door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
        }],
        "vortexKinds": [{ "id": "retained-door", "name": "retained-door", "label": "Retained Door", "color": "#ff0000", "defaultCableKind": "" }],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [{ "source": "retained-door", "target": "retained-door", "bidirectional": true }],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };
    let result = app.import_media("kit:in", media, &meta("local")).await.expect("retained kit:in dispatch");
    assert!(!result.mutations.is_empty(), "exact retained import must publish document mutations");
    let after = projection_of(&app);
    assert_ne!(after, before, "exact retained import must apply its completion output");
    let snapshot: Puzzle5dSnapshot = dsl::json::from_json_str(&after.to_string()).expect("retained projection deserializes");
    let catalogs = crate::kind_catalogs_of(&snapshot.kind_catalogs, &snapshot.kind_catalogs_extra).expect("retained catalog replacement applied");
    let part = catalogs.parts.iter().find(|part| part.id == "retained-capsule").expect("retained part catalog row");
    assert_eq!(part.grips.first().and_then(|grip| grip.grip_kind.as_deref()), Some("retained-door"));
    assert!(catalogs.grips.iter().any(|grip| grip.id == "retained-door"));
}

#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_enforces_exact_media_max_plus_one_before_decode() {
    let prefix = r#"{"objectKinds":[],"vortexKinds":[{"id":"grip","name":"Grip","label":""#;
    let suffix = r##"","color":"#fff","defaultCableKind":""}],"kindCompatibility":[]}"##;
    let label = "x".repeat(PUZZLE5D_IMPORT_MEDIA_BYTES.checked_sub(prefix.len() + suffix.len()).expect("retained max fixture shell"));
    let maximum = format!("{prefix}{label}{suffix}");
    assert_eq!(maximum.len(), PUZZLE5D_IMPORT_MEDIA_BYTES);
    let mut app = app_with_registry();
    let exact = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: maximum.clone() } };
    app.import_media("kit:in", exact, &meta("local")).await.expect("exact media maximum retained dispatch");
    let after_exact = projection_of(&app);
    let plus_one = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: format!("{maximum} ") } };
    let error = app.import_media("kit:in", plus_one, &meta("local")).await.expect_err("media maximum plus one must fail before Serde");
    assert!(error.message.contains("predecode cap"));
    assert_eq!(projection_of(&app), after_exact, "rejected plus-one media must not mutate the document");
}

/// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like
/// block3d's `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
/// `Puzzle5dPlayApp::import_media` must normalize `objectKinds` into the typed
/// `kindCatalogs.parts` (with each per-object `vortices[]` entry becoming a grip template) and
/// `vortexKinds` into `kindCatalogs.grips`, and land both after applying the returned operations.
#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_upserts_part_and_grip_kinds_into_kind_catalogs() {
    let mut app = app_with_registry();
    let fragment = serde_json::json!({
        "schema": "manifest",
        "objectKinds": [{
            "id": "capsule",
            "name": "capsule",
            "label": "Capsule",
            "meshUrl": "/mesh/capsule.glb",
            "vortices": [{ "id": "v0", "vortexKind": "door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
        }],
        "vortexKinds": [{ "id": "door", "name": "door", "label": "Door", "color": "#ff0000", "defaultCableKind": "" }],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [{ "source": "door", "target": "door", "bidirectional": true }],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    let result = app.import_media("kit:in", media, &meta("local")).await.expect("retained kit:in import succeeds");
    assert!(!result.mutations.is_empty(), "importing a non-empty fragment must emit real operations");
    let next_projection = projection_of(&app);

    // 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W4d: `next_projection`'s raw
    // `kindCatalogs` key is now the composed `{childId,target}` handle, not the embedded
    // `{parts:[...],...}` shape a JSON pointer could probe directly — reassemble the full
    // `Puzzle5dKindCatalogs` through the typed snapshot + `kind_catalogs_of` accessor instead
    // (same pattern `sourcing`'s `stock_of` established for its own composed catalog field).
    let next_snapshot: Puzzle5dSnapshot = dsl::json::from_json_str(&next_projection.to_string()).expect("next_projection deserializes as Puzzle5dSnapshot");
    let catalogs = crate::kind_catalogs_of(&next_snapshot.kind_catalogs, &next_snapshot.kind_catalogs_extra).expect("parts catalog present");
    let capsule = catalogs.parts.iter().find(|entry| entry.id == "capsule").expect("the imported part kind must appear in kindCatalogs.parts");
    assert_eq!(capsule.representations.first().map(|representation| representation.url.as_str()), Some("/mesh/capsule.glb"));
    assert_eq!(capsule.grips.first().and_then(|grip| grip.grip_kind.as_deref()), Some("door"), "the per-part grip template keeps its gripKind after normalization");
    assert_eq!(capsule.grips.first().map(|grip| grip.point), Some([0.0, 0.0, 0.0]));
    assert_eq!(capsule.grips.first().map(|grip| grip.direction), Some([0.0, 1.0, 0.0]));
    assert_eq!(capsule.grips.first().and_then(|grip| grip.radius), Some(0.3));

    let door = catalogs.grips.iter().find(|entry| entry.id == "door").expect("the imported grip kind must appear in kindCatalogs.grips");
    assert_eq!(door.default_rope_kind.as_str(), "", "defaultCableKind maps onto defaultRopeKind (a naming judgment call — see import_media's doc comment)");

    let compatibility = next_projection.pointer("/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
    assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
}

/// 🔁️ Re-importing the SAME fragment (simulating a second producer edge, or a redelivered
/// message on a `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_is_idempotent_on_repeated_delivery() {
    let mut app = app_with_registry();
    let fragment = serde_json::json!({
        "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
        "vortexKinds": [],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    for _ in 0..2 {
        app.import_media("kit:in", media.clone(), &meta("local")).await.expect("retained kit:in import succeeds");
    }

    let current = projection_of(&app);
    let current_snapshot: Puzzle5dSnapshot = dsl::json::from_json_str(&current.to_string()).expect("current deserializes as Puzzle5dSnapshot");
    let catalogs = crate::kind_catalogs_of(&current_snapshot.kind_catalogs, &current_snapshot.kind_catalogs_extra).expect("parts catalog present");
    assert_eq!(catalogs.parts.iter().filter(|entry| entry.id == "capsule").count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
}

#[semio_framework_async_macros::async_test]
async fn kit_in_port_is_declared_on_the_app_io() {
    let io = Puzzle5dPlayApp::io().expect("puzzle5d declares an AppIo");
    let kit_in = io.ports.iter().find(|port| port.id == "kit:in").expect("kit:in port declared");
    assert_eq!(kit_in.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(kit_in.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
    assert!(matches!(kit_in.multiplicity, PortMultiplicity::Many));
    let design_out = io.ports.iter().find(|port| port.id == "design:out").expect("design:out port declared");
    assert_eq!(design_out.kind_id.as_deref(), Some("5d.puzzle"));
    assert_eq!(design_out.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Design });
    assert!(matches!(design_out.multiplicity, PortMultiplicity::Many));
}
//#endregion 🔖️KitInPort

//#region 🔖️ClipboardMaximum
const CLIPBOARD_MAXIMUM_FIXTURE: &str = include_str!("../../../🧫️fixtures/📋️clipboard-maximum/🔣️.json");

/// 📋️ LAW (language-neutral fixture): on the largest shipped example with every part selected, the copy fragment
/// carries every part, the cut removes every copied part and the paste of that fragment creates every part again.
/// Each verb runs as one step of its framework-reserved job (cost in `📓️wave-W2-B.md` §5); `import-media` is not
/// implemented by puzzle 5d (only the retained `kit:in` import is).
#[test]
fn clipboard_verbs_cover_every_part_of_the_largest_example() {
    let fixture: serde_json::Value = serde_json::from_str(CLIPBOARD_MAXIMUM_FIXTURE).expect("clipboard fixture parses");
    assert_eq!(fixture["example"], PUZZLE5D_EXAMPLE_CAPSULE_DREAM);
    let offset = fixture["pasteOffset"].as_array().expect("offset").iter().map(|axis| axis.as_f64().expect("axis")).collect::<Vec<_>>();
    let document = capsule_dream_example_document();
    assert_eq!(document.parts.len() as u64, fixture["parts"].as_u64().expect("parts"));
    let snapshot = Puzzle5dPlaySnapshot(serde_json::to_value(&document).expect("document serializes"));
    let history = semio_framework_plugin::HistoryView::empty();
    let view = ArtifactView::new(&snapshot, &history);
    let part_ids: Vec<String> = document.parts.iter().map(|part| part.id.clone()).collect();
    let placement = PastePlacement { anchor: PasteAnchor::Original, position: Some([offset[0], offset[1], offset[2]]) };
    let fragment = puzzle5d_copy_fragment(&snapshot, &part_ids, &[]).expect("copy every part");
    let cut_operations = puzzle5d_cut_operations(&snapshot, &part_ids, &[]);
    let paste_operations = <Puzzle5dPlayApp as ArtifactEditor>::paste_operations(&view, &fragment, &placement).expect("paste every part");
    assert_eq!(fragment.label, format!("{} part(s)", document.parts.len()));
    assert_eq!(cut_operations.iter().filter(|operation| matches!(operation, Puzzle5dMutation::DeletePart(_))).count(), document.parts.len(), "the cut removes every copied part");
    assert_eq!(paste_operations.iter().filter(|operation| matches!(operation, Puzzle5dMutation::CreatePart(_))).count(), document.parts.len(), "the paste creates every fragment part");
}
//#endregion 🔖️ClipboardMaximum

//#region 🎚️WindowAndConfigLaneVerbs
/// 🪟️ The exact board pane partition `window_id` currently holds, projected typed out of the concrete
/// window registry — the same authority the retained route publishes into.
async fn board_window_config(app: &mut Puzzle5dApp, window_id: &str) -> window_ownership::Puzzle5dBoardWindowConfig {
    let view = window_view(board2d::WINDOW_KIND_ID, window_id);
    semio_framework_plugin::artifact_app_laws::capture_fixture_window_config::<window_ownership::Puzzle5dBoardWindowConfigOwner, _, _>(app, &view)
        .await
        .expect("board window config capture")
        .expect("the addressed board pane owns a partition")
}

/// 🌍️ The exact world pane partition `window_id` currently holds.
async fn world_window_config(app: &mut Puzzle5dApp, window_id: &str) -> window_ownership::Puzzle5dWorldWindowConfig {
    let view = window_view(world3d::WINDOW_KIND_ID, window_id);
    semio_framework_plugin::artifact_app_laws::capture_fixture_window_config::<window_ownership::Puzzle5dWorldWindowConfigOwner, _, _>(app, &view)
        .await
        .expect("world window config capture")
        .expect("the addressed world pane owns a partition")
}

/// 📜️ The declared lanes of one migrated tool id.
fn declared_lanes(tool_id: &str) -> &'static [ArtifactToolPublicationLane] {
    <Puzzle5dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .find(|contract| contract.tool_id == tool_id)
        .unwrap_or_else(|| panic!("{tool_id} declares a publication contract"))
        .lanes
}

/// 🧵️ LAW: every verb this slice migrated is a live retained tool — registered in the id list, in the
/// bounded first-step proofs, in the publication contracts, and classified `Migrated`. A
/// `BatchOnlyPendingRewrite` row here is the repo's known hard-dead runtime pattern: the click reaches
/// nothing and no fault surfaces.
#[test]
fn window_and_config_lane_verbs_are_registered_migrated_retained_tools() {
    let source = include_str!("../../🦀️.rs");
    let production = source.split_once("//#region 🧪️UnitTests").map(|(production, _)| production).expect("production prefix");
    for tool_id in MIGRATED_WINDOW_LANE_VERBS.iter().chain(MIGRATED_CONFIG_LANE_VERBS) {
        assert!(PUZZLE5D_RETAINED_TOOL_IDS.contains(tool_id), "{tool_id} must be a retained tool id");
        assert!(production.contains(&format!(r#".action_interactive_job("{tool_id}", InteractiveJobClassification::Migrated)"#)), "{tool_id} must be classified Migrated");
        assert!(!production.contains(&format!(r#".action_interactive_job("{tool_id}", InteractiveJobClassification::BatchOnlyPendingRewrite)"#)), "{tool_id} must carry no BatchOnlyPendingRewrite row");
        assert!(!declared_lanes(tool_id).is_empty(), "{tool_id} must declare its publication lanes");
    }
    for tool_id in MIGRATED_WINDOW_LANE_VERBS {
        assert_eq!(declared_lanes(tool_id), &[ArtifactToolPublicationLane::WindowConfig], "{tool_id} publishes the addressed pane's window config alone");
    }
    for tool_id in MIGRATED_CONFIG_LANE_VERBS {
        assert_eq!(declared_lanes(tool_id), &[ArtifactToolPublicationLane::Config], "{tool_id} publishes the shared app config alone");
    }
    // 🗡️ The pre-5A1 vocabulary must be gone everywhere, aliases included.
    assert!(!production.contains("setObjectKindWeight") && !production.contains("setVortexKindWeight"), "the object/vortex weight verb ids are renamed to the 5d part/grip vocabulary");
}

/// 🪟️ The window-config-lane verbs this slice migrated.
const MIGRATED_WINDOW_LANE_VERBS: &[&str] =
    &["setCamera", "setCamera2d", "setCamera3d", "setGridFactor", "setGridSnapEnabled", "setLodMode", "setSuggestionOffset", "toggleSun", "setSunAzimuth", "setSunElevation", "setSunIntensity"];
/// 🎚️ The shared-app-config-lane verbs this slice migrated.
const MIGRATED_CONFIG_LANE_VERBS: &[&str] = &["setBrushPlacementContactTolerance", "setPartKindWeight", "setGripKindWeight"];

/// 📜️ LAW: the wire band the retained factory admits is the one its bounded first-step proofs declare —
/// a proof advertising more than the factory admits is a contract the runtime refuses, and the shared
/// 8 KiB puzzle default is too narrow for a Nakagin-sized document's command wire.
#[test]
fn retained_factory_contract_matches_its_declared_proof_band() {
    let contract = ToolJobFactory::execution_contract(&Puzzle5dRetainedCommandJobFactory::new("s.puzzle.puzzle5d@1/*#editor"));
    assert_eq!(contract.max_raw_wire_bytes, PUZZLE5D_RETAINED_RAW_BYTES);
    assert_eq!(PUZZLE5D_RETAINED_RAW_BYTES, 262_144);
    assert_eq!(PUZZLE5D_RETAINED_DECODED_ITEMS, 16_384);
    assert!(PUZZLE5D_RETAINED_RAW_BYTES > crate::retained_command::PUZZLE_COMMAND_RAW_BYTES, "the retained route must admit more than the shared puzzle default it replaced");
}

/// 🎥️ LAW (camera family): `setCamera` is scoped by the ADDRESSED WINDOW, not by the payload shape —
/// the board pane writes the flat camera, the world pane the volume camera — and each pane publishes
/// only its own `WindowConfig`, leaving the document and the shared app config untouched.
#[semio_framework_async_macros::async_test]
async fn camera_verbs_publish_only_the_addressed_window_config() {
    let mut app = Box::new(app_with_registry());
    let board = "puzzle5d-camera-board";
    let world = "puzzle5d-camera-world";
    let document_before = projection_of(&app);
    let config_before = app.config_pack().await.expect("app config before camera publications");

    let flat = dispatch(&mut app, "setCamera", Some(&dsl::json!({ "windowId": board2d::WINDOW_KIND_ID, "camera": { "x": 4.5, "y": -2.5, "zoom": 3.0 } })), Some(board)).expect("setCamera in the board pane");
    assert!(flat.mutations.is_empty(), "a camera pose is never a document edit");
    assert_eq!(board_window_config(&mut app, board).await.camera2d, Puzzle5dCamera2d { x: 4.5, y: -2.5, zoom: 3.0 }, "the board pane's setCamera wrote its own flat camera");

    let volume = dispatch(&mut app, "setCamera", Some(&dsl::json!({ "windowId": world3d::WINDOW_KIND_ID, "camera": { "position": [1.0, 2.0, 3.0], "target": [4.0, 5.0, 6.0], "zoom": 2.0 } })), Some(world)).expect("setCamera in the world pane");
    assert!(volume.mutations.is_empty());
    let volume_camera = world_window_config(&mut app, world).await.camera3d;
    assert_eq!((volume_camera.position, volume_camera.target, volume_camera.zoom), ([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], 2.0), "the world pane's setCamera wrote its own volume camera");

    dispatch(&mut app, "setCamera2d", Some(&dsl::json!({ "windowId": board2d::WINDOW_KIND_ID, "camera": { "x": -8.0, "y": 9.0, "zoom": 0.5 } })), Some(board)).expect("setCamera2d");
    assert_eq!(board_window_config(&mut app, board).await.camera2d, Puzzle5dCamera2d { x: -8.0, y: 9.0, zoom: 0.5 });
    dispatch(&mut app, "setCamera3d", Some(&dsl::json!({ "windowId": world3d::WINDOW_KIND_ID, "camera": { "position": [7.0, 7.0, 7.0], "target": [0.0, 0.0, 0.0], "zoom": 1.25 } })), Some(world)).expect("setCamera3d");
    assert_eq!(world_window_config(&mut app, world).await.camera3d.position, [7.0, 7.0, 7.0]);

    assert_eq!(projection_of(&app), document_before, "no camera verb touches the document lane");
    let config_after = app.config_pack().await.expect("app config after camera publications");
    assert_eq!((config_after.pack, config_after.spr), (config_before.pack, config_before.spr), "no camera verb touches the shared app config lane");
    close_app(&mut app);
}

/// 🧲️ LAW (grid family): the snap toggle reads its `pressed` state and flips without one — reading an
/// absent argument as `false` turned every press into an unconditional "off" — and the factor takes an
/// absolute `value` or a `delta` nudge, clamped into the declared band the window schema states as
/// `exclusiveMinimum: 0`.
#[semio_framework_async_macros::async_test]
async fn grid_verbs_publish_the_clamped_board_window_config() {
    let mut app = Box::new(app_with_registry());
    let board = "puzzle5d-grid-board";
    assert!(window_ownership::Puzzle5dBoardWindowConfig::default().grid_snap_enabled, "snap starts on");

    dispatch(&mut app, "setGridSnapEnabled", Some(&dsl::json!({ "pressed": false })), Some(board)).expect("press the snap toggle off");
    assert!(!board_window_config(&mut app, board).await.grid_snap_enabled, "the pressed state is what persists");
    dispatch(&mut app, "setGridSnapEnabled", None, Some(board)).expect("argument-less snap toggle");
    assert!(board_window_config(&mut app, board).await.grid_snap_enabled, "an argument-less invocation flips the current state");

    dispatch(&mut app, "setGridFactor", Some(&dsl::json!({ "value": 4.0 })), Some(board)).expect("absolute grid factor");
    assert_eq!(board_window_config(&mut app, board).await.grid_factor, 4.0);
    dispatch(&mut app, "setGridFactor", Some(&dsl::json!({ "delta": -1.0 })), Some(board)).expect("grid factor stepper nudge");
    assert_eq!(board_window_config(&mut app, board).await.grid_factor, 3.0, "a delta nudges the current factor");
    dispatch(&mut app, "setGridFactor", Some(&dsl::json!({ "value": 0.0 })), Some(board)).expect("grid factor below the band");
    assert_eq!(board_window_config(&mut app, board).await.grid_factor, PUZZLE5D_GRID_FACTOR_MIN, "the schema's exclusive minimum is what the clamp protects");
    dispatch(&mut app, "setGridFactor", Some(&dsl::json!({ "value": 1_000.0 })), Some(board)).expect("grid factor above the band");
    assert_eq!(board_window_config(&mut app, board).await.grid_factor, PUZZLE5D_GRID_FACTOR_MAX);
    close_app(&mut app);
}

/// 🔭️🧭️ LAW (LOD + suggestion offset): both persist into the addressed board pane's own partition, and
/// the offset clamps into the band its slider declares whether it arrives absolute or as a nudge.
#[semio_framework_async_macros::async_test]
async fn lod_and_suggestion_offset_publish_the_board_window_config() {
    let mut app = Box::new(app_with_registry());
    let board = "puzzle5d-lod-board";
    dispatch(&mut app, "setLodMode", Some(&dsl::json!({ "value": "manual" })), Some(board)).expect("setLodMode");
    assert_eq!(board_window_config(&mut app, board).await.lod_mode, "manual");
    dispatch(&mut app, "setLodMode", Some(&dsl::json!({ "value": PUZZLE5D_LOD_MODE_AUTOMATIC })), Some(board)).expect("setLodMode back to automatic");
    assert_eq!(board_window_config(&mut app, board).await.lod_mode, PUZZLE5D_LOD_MODE_AUTOMATIC);

    dispatch(&mut app, "setSuggestionOffset", Some(&dsl::json!({ "value": PUZZLE5D_SUGGESTION_OFFSET_MAX + 1.0 })), Some(board)).expect("setSuggestionOffset above the band");
    assert_eq!(board_window_config(&mut app, board).await.suggestion_offset, PUZZLE5D_SUGGESTION_OFFSET_MAX);
    dispatch(&mut app, "setSuggestionOffset", Some(&dsl::json!({ "delta": -1.0 })), Some(board)).expect("setSuggestionOffset stepper nudge");
    assert_eq!(board_window_config(&mut app, board).await.suggestion_offset, PUZZLE5D_SUGGESTION_OFFSET_MAX - 1.0, "the settings stepper's delta nudges the current offset");
    dispatch(&mut app, "setSuggestionOffset", Some(&dsl::json!({ "value": PUZZLE5D_SUGGESTION_OFFSET_MIN - 1.0 })), Some(board)).expect("setSuggestionOffset below the band");
    assert_eq!(board_window_config(&mut app, board).await.suggestion_offset, PUZZLE5D_SUGGESTION_OFFSET_MIN);
    close_app(&mut app);
}

/// ☀️ LAW (sun family): the four sun verbs publish the world pane's own partition and nothing else.
#[semio_framework_async_macros::async_test]
async fn sun_verbs_publish_the_world_window_config() {
    let mut app = Box::new(app_with_registry());
    let world = "puzzle5d-sun-world";
    let document_before = projection_of(&app);
    let enabled_before = window_ownership::Puzzle5dWorldWindowConfig::default().sun.enabled;
    dispatch(&mut app, "toggleSun", None, Some(world)).expect("toggleSun");
    assert_eq!(world_window_config(&mut app, world).await.sun.enabled, !enabled_before, "toggling the sun flips the world pane's own environment");
    dispatch(&mut app, "setSunAzimuth", Some(&dsl::json!({ "value": 123.0 })), Some(world)).expect("setSunAzimuth");
    dispatch(&mut app, "setSunElevation", Some(&dsl::json!({ "value": 41.0 })), Some(world)).expect("setSunElevation");
    dispatch(&mut app, "setSunIntensity", Some(&dsl::json!({ "value": 0.25 })), Some(world)).expect("setSunIntensity");
    let sun = world_window_config(&mut app, world).await.sun;
    assert_eq!((sun.azimuth, sun.elevation, sun.intensity), (123.0, 41.0, 0.25));
    assert_eq!(projection_of(&app), document_before, "no sun verb touches the document lane");
    close_app(&mut app);
}

/// ⚖️🚧️ LAW (Config-lane family): the contact tolerance and both renamed kind-weight verbs are shared
/// app configuration, not window state — dispatched through the real retained factory they publish the
/// `Config` lane, leave the addressed pane's window partition at generation zero and never touch the
/// document.
#[semio_framework_async_macros::async_test]
async fn config_lane_verbs_publish_the_shared_app_config_only() {
    let mut app = Box::new(app_with_registry());
    let pane = "puzzle5d-config-pane";
    let view = window_view(world3d::WINDOW_KIND_ID, pane);
    let document_before = projection_of(&app);
    let mut config_before = app.config_pack().await.expect("app config before");

    let part_kind = projection_of(&app).get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).and_then(|part| part.get("partKind")).and_then(Value::as_str).expect("the boot document carries a part kind").to_string();
    let grip_kind = projection_of(&app)
        .get("parts")
        .and_then(Value::as_array)
        .and_then(|parts| parts.iter().find_map(|part| part.get("grips").and_then(Value::as_array).and_then(|grips| grips.first()).and_then(|grip| grip.get("gripKind")).and_then(Value::as_str)))
        .expect("the boot document carries a grip kind")
        .to_string();

    for (tool_id, args) in [
        ("setBrushPlacementContactTolerance", dsl::json!({ "value": 0.02 })),
        ("setPartKindWeight", dsl::json!({ "kindId": part_kind.as_str(), "value": 0.75 })),
        ("setGripKindWeight", dsl::json!({ "kindId": grip_kind.as_str(), "value": 0.25 })),
    ] {
        let result = dispatch(&mut app, tool_id, Some(&args), Some(pane)).expect("Config-lane dispatch through the retained factory");
        assert!(result.mutations.is_empty(), "{tool_id} is never a document edit");
        let after = app.config_pack().await.expect("app config after a Config-lane publication");
        assert_ne!((&after.pack, &after.spr), (&config_before.pack, &config_before.spr), "{tool_id} must be a real shared-app-config publication");
        assert_eq!(app.window_config_generation(&view).await.expect("window config generation").unwrap_or(0), 0, "{tool_id} must publish no window partition");
        config_before = after;
    }

    assert_eq!(projection_of(&app), document_before, "no Config-lane verb touches the document lane");
    close_app(&mut app);
}

/// ⚖️ LAW: the group `setPartKindWeight`/`setGripKindWeight` re-normalize always sums to 1 and
/// redistributes the remainder across the siblings in proportion to what they already held — the pure
/// core both verbs share, so the retained route's law above only has to prove the lane.
#[test]
fn kind_weight_group_normalization_keeps_the_group_summing_to_one() {
    let ids = ["a".to_string(), "b".to_string(), "c".to_string()];
    let weights: HashMap<String, f64> = ids.iter().map(|id| (id.clone(), 1.0 / 3.0)).collect();
    let normalized = puzzle5d_normalize_kind_weight_group(&weights, &ids, "a", 0.5);
    assert!((normalized["a"] - 0.5).abs() < 1e-9, "the changed kind keeps exactly what was asked for");
    assert!((normalized.values().sum::<f64>() - 1.0).abs() < 1e-9, "the group sums to 1");
    assert!((normalized["b"] - normalized["c"]).abs() < 1e-9, "equal siblings stay equal");
    let skewed: HashMap<String, f64> = [("a", 0.2), ("b", 0.6), ("c", 0.2)].into_iter().map(|(id, weight)| (id.to_string(), weight)).collect();
    let normalized = puzzle5d_normalize_kind_weight_group(&skewed, &ids, "a", 0.5);
    assert!((normalized.values().sum::<f64>() - 1.0).abs() < 1e-9);
    assert!(normalized["b"] > normalized["c"], "the remainder is redistributed in proportion to what the siblings held");
    let single = [("only".to_string(), 1.0)].into_iter().collect::<HashMap<String, f64>>();
    let normalized = puzzle5d_normalize_kind_weight_group(&single, &["only".to_string()], "only", 0.0);
    assert!((normalized["only"] - 1.0).abs() < 1e-9, "a one-kind group is always the whole distribution");
}
//#endregion 🎚️WindowAndConfigLaneVerbs

//#region ☑️WindowOptionGroups
/// 🌐️ LAW (grid group): `setGridVisible` toggles from `pressed` and flips without one, and
/// `setGridSpacing` clamps into the band its slider declares — each in the ADDRESSED pane's own
/// partition, never the document and never the shared app config.
#[semio_framework_async_macros::async_test]
async fn grid_visibility_and_spacing_publish_the_addressed_window_config() {
    let mut app = Box::new(app_with_registry());
    let board = "puzzle5d-gridvis-board";
    let world = "puzzle5d-gridvis-world";
    let document_before = projection_of(&app);
    let config_before = app.config_pack().await.expect("app config before the grid group");

    assert!(window_ownership::Puzzle5dBoardWindowConfig::default().grid_visible, "the grid starts visible");
    dispatch(&mut app, "setGridVisible", Some(&dsl::json!({ "windowId": board2d::WINDOW_KIND_ID, "pressed": false })), Some(board)).expect("press the board grid off");
    assert!(!board_window_config(&mut app, board).await.grid_visible, "the pressed state is what persists");
    dispatch(&mut app, "setGridVisible", Some(&dsl::json!({ "windowId": board2d::WINDOW_KIND_ID })), Some(board)).expect("argument-less grid toggle");
    assert!(board_window_config(&mut app, board).await.grid_visible, "an invocation without `pressed` flips the current state");

    dispatch(&mut app, "setGridVisible", Some(&dsl::json!({ "pressed": false })), Some(world)).expect("press the world grid off");
    assert!(!world_window_config(&mut app, world).await.grid_visible, "the world pane owns its own grid visibility");
    assert!(board_window_config(&mut app, board).await.grid_visible, "one pane's grid toggle never reaches the other");

    dispatch(&mut app, "setGridSpacing", Some(&dsl::json!({ "value": 4.0 })), Some(world)).expect("absolute grid spacing");
    assert_eq!(world_window_config(&mut app, world).await.grid_spacing, 4.0);
    dispatch(&mut app, "setGridSpacing", Some(&dsl::json!({ "value": 0.0 })), Some(world)).expect("grid spacing below the band");
    assert_eq!(world_window_config(&mut app, world).await.grid_spacing, PUZZLE5D_GRID_SPACING_MIN);
    dispatch(&mut app, "setGridSpacing", Some(&dsl::json!({ "value": 10_000.0 })), Some(world)).expect("grid spacing above the band");
    assert_eq!(world_window_config(&mut app, world).await.grid_spacing, PUZZLE5D_GRID_SPACING_MAX);

    assert_eq!(projection_of(&app), document_before, "no grid-group verb touches the document lane");
    let config_after = app.config_pack().await.expect("app config after the grid group");
    assert_eq!((config_after.pack, config_after.spr), (config_before.pack, config_before.spr), "no grid-group verb touches the shared app config lane");
    close_app(&mut app);
}

/// 🔭️ LAW (LOD trio): the two toggles read `pressed` and flip without it, and the manual slider clamps
/// into the band `PUZZLE5D_LOD_SLIDER_MIN..=MAX` states — all three in the world pane's partition.
#[semio_framework_async_macros::async_test]
async fn lod_trio_publishes_the_world_window_config() {
    let mut app = Box::new(app_with_registry());
    let world = "puzzle5d-lodtrio-world";
    assert!(window_ownership::Puzzle5dWorldWindowConfig::default().lod_automatic, "automatic LOD starts on");

    dispatch(&mut app, "setLodAutomatic", Some(&dsl::json!({ "pressed": false })), Some(world)).expect("press automatic LOD off");
    assert!(!world_window_config(&mut app, world).await.lod_automatic);
    dispatch(&mut app, "setLodAutomatic", None, Some(world)).expect("argument-less automatic LOD toggle");
    assert!(world_window_config(&mut app, world).await.lod_automatic);

    dispatch(&mut app, "setLodDepthVariable", Some(&dsl::json!({ "pressed": true })), Some(world)).expect("press depth-variable LOD on");
    assert!(world_window_config(&mut app, world).await.lod_depth_variable);

    dispatch(&mut app, "setLodManual", Some(&dsl::json!({ "value": 250.0 })), Some(world)).expect("absolute manual LOD");
    assert_eq!(world_window_config(&mut app, world).await.lod_manual, 250.0);
    dispatch(&mut app, "setLodManual", Some(&dsl::json!({ "value": -4.0 })), Some(world)).expect("manual LOD below the band");
    assert_eq!(world_window_config(&mut app, world).await.lod_manual, PUZZLE5D_LOD_SLIDER_MIN);
    dispatch(&mut app, "setLodManual", Some(&dsl::json!({ "value": 9_000.0 })), Some(world)).expect("manual LOD above the band");
    assert_eq!(world_window_config(&mut app, world).await.lod_manual, PUZZLE5D_LOD_SLIDER_MAX);
    close_app(&mut app);
}

/// 🎯️ LAW (selection group): `setSelectableKind` toggles exactly the named kind of the ADDRESSED pane
/// and refuses a kind this artifact does not carry.
#[semio_framework_async_macros::async_test]
async fn selectable_kinds_publish_the_addressed_window_config() {
    let mut app = Box::new(app_with_registry());
    let board = "puzzle5d-selectable-board";
    let world = "puzzle5d-selectable-world";

    dispatch(&mut app, "setSelectableKind", Some(&dsl::json!({ "windowId": board2d::WINDOW_KIND_ID, "kind": "grips", "pressed": false })), Some(board)).expect("press board grips off");
    let kinds = board_window_config(&mut app, board).await.selectable_kinds;
    assert_eq!((kinds.parts, kinds.grips, kinds.fasteners), (true, false, true), "only the named kind changed");
    let untouched = world_window_config(&mut app, world).await.selectable_kinds;
    assert!(untouched.grips, "the pick filter is per pane");

    dispatch(&mut app, "setSelectableKind", Some(&dsl::json!({ "kind": "fasteners" })), Some(world)).expect("argument-less fastener toggle");
    assert!(!world_window_config(&mut app, world).await.selectable_kinds.fasteners, "an absent `pressed` flips the current state");
    dispatch(&mut app, "setSelectableKind", Some(&dsl::json!({ "kind": "vortices", "pressed": false })), Some(world)).expect("an unknown kind is a silent no-op, never a fault");
    let after = world_window_config(&mut app, world).await.selectable_kinds;
    assert_eq!((after.parts, after.grips), (true, true), "a kind this artifact does not carry changes nothing");
    close_app(&mut app);
}

/// 🤏️🧭️ LAW (grip group): show/direction take only the values their select declares; anything else is a
/// no-op, and both persist in the world pane's partition.
#[semio_framework_async_macros::async_test]
async fn grip_show_and_direction_publish_the_world_window_config() {
    let mut app = Box::new(app_with_registry());
    let world = "puzzle5d-grip-world";
    assert_eq!(window_ownership::Puzzle5dWorldWindowConfig::default().grip_show, PUZZLE5D_GRIP_SHOW_SELECTED);

    dispatch(&mut app, "setGripShow", Some(&dsl::json!({ "value": PUZZLE5D_GRIP_SHOW_ALWAYS })), Some(world)).expect("setGripShow");
    assert_eq!(world_window_config(&mut app, world).await.grip_show, PUZZLE5D_GRIP_SHOW_ALWAYS);
    dispatch(&mut app, "setGripShow", Some(&dsl::json!({ "value": "vielleicht" })), Some(world)).expect("an undeclared mode is a no-op, never a fault");
    assert_eq!(world_window_config(&mut app, world).await.grip_show, PUZZLE5D_GRIP_SHOW_ALWAYS);

    dispatch(&mut app, "setGripDirection", Some(&dsl::json!({ "value": PUZZLE5D_GRIP_DIRECTION_INWARDS })), Some(world)).expect("setGripDirection");
    assert_eq!(world_window_config(&mut app, world).await.grip_direction, PUZZLE5D_GRIP_DIRECTION_INWARDS);
    dispatch(&mut app, "setGripDirection", Some(&dsl::json!({ "value": "seitwärts" })), Some(world)).expect("an undeclared direction is a no-op");
    assert_eq!(world_window_config(&mut app, world).await.grip_direction, PUZZLE5D_GRIP_DIRECTION_INWARDS);
    close_app(&mut app);
}

/// 🎥️ LAW (projection): `setProjection` re-derives the camera pose around the unchanged target when the
/// change moves it, while `setProjectionParam` tunes a parameter in place and leaves the pose alone.
#[semio_framework_async_macros::async_test]
async fn projection_verbs_publish_the_world_window_config_and_repose_only_when_they_must() {
    let mut app = Box::new(app_with_registry());
    let world = "puzzle5d-projection-world";
    let before = world_window_config(&mut app, world).await.camera3d;

    dispatch(&mut app, "setProjection", Some(&dsl::json!({ "field": "orthographicView", "value": "front" })), Some(world)).expect("setProjection");
    let reposed = world_window_config(&mut app, world).await.camera3d;
    assert_eq!(reposed.projection.kind, "orthographic");
    assert_eq!(reposed.projection.orthographic_view, "front");
    assert_ne!(reposed.position, before.position, "a projection change that moves the pose re-derives position");
    assert_eq!(reposed.target, before.target, "the target it orbits is unchanged");
    assert!(reposed.up.is_some(), "the re-derived pose states its up vector");

    dispatch(&mut app, "setProjectionParam", Some(&dsl::json!({ "param": "fov", "value": 35.0 })), Some(world)).expect("setProjectionParam");
    let tuned = world_window_config(&mut app, world).await.camera3d;
    assert_eq!(tuned.projection.fov, 35.0);
    assert_eq!(tuned.position, reposed.position, "a pure parameter tweak keeps the pose");
    close_app(&mut app);
}

/// 🎛️ LAW (transform gumball flags): each flag toggles on its own and persists in the addressed pane's
/// partition — with both off the world pane refuses to draw a gumball nobody could grab.
#[semio_framework_async_macros::async_test]
async fn transform_gumball_flags_publish_the_window_config() {
    let mut app = Box::new(app_with_registry());
    let world = "puzzle5d-gumball-world";
    let defaults = window_ownership::Puzzle5dWorldWindowConfig::default();
    assert!(defaults.transform_move && defaults.transform_rotate, "both handle families start on");

    dispatch(&mut app, "setTransformGumballFlag", Some(&dsl::json!({ "flag": "move", "pressed": false })), Some(world)).expect("press move off");
    let after_move = world_window_config(&mut app, world).await;
    assert!(!after_move.transform_move && after_move.transform_rotate, "only the named flag changed");
    dispatch(&mut app, "setTransformGumballFlag", Some(&dsl::json!({ "flag": "rotate" })), Some(world)).expect("argument-less rotate toggle");
    let after_rotate = world_window_config(&mut app, world).await;
    assert!(!after_rotate.transform_move && !after_rotate.transform_rotate);
    dispatch(&mut app, "setTransformGumballFlag", Some(&dsl::json!({ "flag": "scale", "pressed": false })), Some(world)).expect("an unknown flag is a no-op, never a fault");

    let runtime = Puzzle5dRuntime { transform_move: false, transform_rotate: false, ..Puzzle5dRuntime::default() };
    let marked = Puzzle5dInteractionSnapshot { granularity: PUZZLE5D_GRANULARITY_PART.into(), selected: vec!["teil-ä".into()], hovered: Vec::new() };
    assert!(!puzzle5d_gumball_active(&runtime, "move", &marked), "an all-off gumball never renders");
    close_app(&mut app);
}

/// 🎚️ LAW (measure round-trip): a dispatched option change is what the pane's own chrome renders on the
/// next frame — the "dispatch → config changes → measure reflects it" loop closed end to end.
#[semio_framework_async_macros::async_test]
async fn dispatched_window_options_are_what_the_next_measure_frame_renders() {
    let mut app = Box::new(app_with_registry());
    dispatch(&mut app, "setGridVisible", Some(&dsl::json!({ "pressed": false })), Some(world3d::WINDOW_KIND_ID)).expect("setGridVisible");
    dispatch(&mut app, "setGridSpacing", Some(&dsl::json!({ "value": 6.0 })), Some(world3d::WINDOW_KIND_ID)).expect("setGridSpacing");
    dispatch(&mut app, "setLodManual", Some(&dsl::json!({ "value": 320.0 })), Some(world3d::WINDOW_KIND_ID)).expect("setLodManual");
    dispatch(&mut app, "setGripShow", Some(&dsl::json!({ "value": PUZZLE5D_GRIP_SHOW_ALWAYS })), Some(world3d::WINDOW_KIND_ID)).expect("setGripShow");
    dispatch(&mut app, "setSelectableKind", Some(&dsl::json!({ "kind": "grips", "pressed": false })), Some(world3d::WINDOW_KIND_ID)).expect("setSelectableKind");
    let measures = window_measures_of(&mut app, world3d::WINDOW_KIND_ID);
    assert_eq!(toggle_pressed(&measures, "puzzle5d-play-world-grid-visible"), Some(false));
    assert_eq!(slider_value(&measures, "puzzle5d-play-world-grid-spacing"), Some(6.0));
    assert_eq!(slider_value(&measures, "puzzle5d-play-world-lod-value"), Some(320.0));
    assert_eq!(select_value(&measures, "puzzle5d-play-world-grip-show").as_deref(), Some(PUZZLE5D_GRIP_SHOW_ALWAYS));
    assert_eq!(toggle_pressed(&measures, "puzzle5d-play-world-select-grips"), Some(false));

    dispatch(&mut app, "setGridVisible", Some(&dsl::json!({ "pressed": false })), Some(board2d::WINDOW_KIND_ID)).expect("board setGridVisible");
    let board_measures = window_measures_of(&mut app, board2d::WINDOW_KIND_ID);
    assert_eq!(toggle_pressed(&board_measures, "puzzle5d-play-board-grid-visible"), Some(false));
    assert_eq!(toggle_pressed(&board_measures, "puzzle5d-play-board-select-parts"), Some(true));
    close_app(&mut app);
}

/// 🕹️ LAW (one shared interaction domain): the SAME `Puzzle5dInteractionSnapshot` feeds both panes, so a
/// part selected or hovered from either pane is painted by the board scene AND by the world scene.
#[test]
fn one_interaction_snapshot_paints_both_panes() {
    let projection = parse(
        r#"{"schema":"puzzle.5d","parts":[{"id":"teil-ä","partKind":"Part","2d":{"x":1.0,"y":2.0},"3d":{"origin":[0.0,0.0,0.0]},"grips":[{"id":"g1","gripKind":"griff-ü","2d":{},"3d":{"position":[1.0,0.0,0.0]}}]}],"fasteners":[]}"#,
    )
    .expect("projection");
    let part_id = "teil-ä";
    let interaction = Puzzle5dInteractionSnapshot { granularity: PUZZLE5D_GRANULARITY_PART.into(), selected: vec![part_id.into()], hovered: vec![part_id.into()] };
    let envelope = scene_from_projection_with_interaction(&projection, Puzzle5dRuntime::default(), "select", interaction);
    assert_eq!(envelope.document.parts.len(), 1, "the fixture projection materialized its one part");

    let board = board2d::puzzle5d_board_scene(&envelope);
    assert!(board.selection_json.contains(part_id), "the board pane paints the shared selection");
    assert_eq!(board.hovered_id.as_deref(), Some(part_id), "the board pane paints the shared hover");

    let world_instances: Value = parse(&world3d::world_instances_json(&envelope.document, &envelope.interaction, None)).expect("instancesJson");
    let marked = world_instances.as_array().and_then(|rows| rows.iter().find(|row| row.get("id").and_then(Value::as_str) == Some(part_id))).expect("the world pane carries the same part");
    assert_eq!(marked.get("selected").and_then(Value::as_bool), Some(true), "the world pane paints the shared selection");
    assert_eq!(marked.get("hovered").and_then(Value::as_bool), Some(true), "the world pane paints the shared hover");
}
//#endregion ☑️WindowOptionGroups

//#region 🧩️ArtifactLaneVerbs
/// 🧩️ Every Artifact-lane / interaction verb slice 5A2 migrated, with the lanes its retained route may
/// publish to. An undeclared lane is a runtime fault and a `BatchOnlyPendingRewrite` row is hard-dead, so
/// this table is the whole slice's registration contract in one place.
const MIGRATED_ARTIFACT_LANE_VERBS: &[(&str, &[ArtifactToolPublicationLane])] = &[
    ("addBrushPart", &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction]),
    ("addNode", &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction]),
    ("addPartKind", &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction]),
    ("createFastener", &[ArtifactToolPublicationLane::Artifact]),
    ("deleteFastener", &[ArtifactToolPublicationLane::Artifact]),
    ("editFastener", &[ArtifactToolPublicationLane::Artifact]),
    ("focusSelection", &[ArtifactToolPublicationLane::WindowConfig]),
    ("patchFastener", &[ArtifactToolPublicationLane::Artifact]),
    ("patchGrip", &[ArtifactToolPublicationLane::Artifact]),
    ("patchPart", &[ArtifactToolPublicationLane::Artifact]),
    ("proximityConnect", &[ArtifactToolPublicationLane::Artifact]),
    ("registerBrushMesh", &[ArtifactToolPublicationLane::HostOnly]),
    ("retargetFastener", &[ArtifactToolPublicationLane::Artifact]),
    ("rotateSelection", &[ArtifactToolPublicationLane::Artifact]),
    ("scaleSelection", &[ArtifactToolPublicationLane::Artifact]),
    ("selectSameKindSelection", &[ArtifactToolPublicationLane::Interaction]),
    ("translateSelection", &[ArtifactToolPublicationLane::Artifact]),
    ("worldRelocate", &[ArtifactToolPublicationLane::Artifact]),
];

/// 🧵️ LAW: every Artifact-lane verb is a live retained tool — in the id list, in the bounded first-step
/// proofs, in the publication contracts with EXACTLY the lanes above, and classified `Migrated`.
#[test]
fn artifact_lane_verbs_are_registered_migrated_retained_tools() {
    let source = include_str!("../../🦀️.rs");
    let production = source.split_once("//#region 🧪️UnitTests").map(|(production, _)| production).expect("production prefix");
    let proofs = production.split_once("semio_framework_plugin::bounded_first_step_tool_proofs!").and_then(|(_, rest)| rest.split_once("fn register_tool_job_factories")).map(|(proofs, _)| proofs).expect("proof block");
    for (tool_id, lanes) in MIGRATED_ARTIFACT_LANE_VERBS {
        assert!(PUZZLE5D_RETAINED_TOOL_IDS.contains(tool_id), "{tool_id} must be a retained tool id");
        assert!(proofs.contains(&format!("\"{tool_id}\"")), "{tool_id} must carry a bounded first-step proof");
        assert!(production.contains(&format!(".action_interactive_job(\"{tool_id}\", InteractiveJobClassification::Migrated)")), "{tool_id} must be classified Migrated");
        assert_eq!(declared_lanes(tool_id), *lanes, "{tool_id} declares exactly the lanes its retained route publishes to");
    }
    for tool_id in ["applyBoardEvents", "setActiveExample"] {
        assert!(PUZZLE5D_RETAINED_TOOL_IDS.contains(&tool_id));
        assert!(declared_lanes(tool_id).contains(&ArtifactToolPublicationLane::Artifact), "{tool_id} edits the document");
        assert!(declared_lanes(tool_id).contains(&ArtifactToolPublicationLane::Interaction), "{tool_id} moves the selection it invalidated");
    }
    assert!(!production.contains("BatchOnlyPendingRewrite"), "no puzzle 5d verb may stay on the hard-dead classification");
}

/// 🗡️ LAW: the duplicate verb ids are GONE, not aliased — one real verb each for focus, select-same-kind
/// and brush placement, so no context-menu or keybinding row can address a dead twin.
#[test]
fn duplicate_verb_ids_are_removed_rather_than_aliased() {
    let source = include_str!("../../🦀️.rs");
    let production = source.split_once("//#region 🧪️UnitTests").map(|(production, _)| production).expect("production prefix");
    for removed in ["zoomToSelection", "\"selectSameKind\""] {
        assert!(!production.contains(removed), "{removed} must not survive anywhere in the production source");
    }
    assert!(production.contains(".action(\"focusSelection\")"), "the context menu addresses the one real focus verb");
    assert!(Puzzle5dCommand::try_from_action("zoomToSelection", None, None).is_none(), "the removed focus alias has no command variant");
    assert!(Puzzle5dCommand::try_from_action("addBrushObject", None, None).is_none(), "the removed brush alias has no command variant");
    assert!(Puzzle5dCommand::try_from_action("selectSameKind", None, None).is_none(), "the removed selection alias has no command variant");
}

/// 🌱️ A document built by this app's OWN live verbs: an empty example, then `count` catalogue adds. It
/// deliberately depends on no shipped example file — `document_from_json` silently answers
/// `empty_document()` for a fixture it cannot deserialize, which is exactly how `🌙️capsule-dream` and
/// `🏗️nakagin-capsule-tower` currently load (see `clipboard_verbs_cover_every_part_of_the_largest_example`,
/// red before this slice) — so a law that read those files would assert `0 == 0` and prove nothing.
fn seeded_parts(app: &mut Puzzle5dApp, count: usize) -> Vec<String> {
    dispatch(app, "setActiveExample", Some(&dsl::json!({ "exampleId": "" })), None).expect("empty document");
    assert_eq!(part_count(app), 0, "the empty example really empties the document");
    for index in 0..count {
        dispatch(app, "addPartKind", Some(&dsl::json!({ "partKind": "Part", "x": 120.0 + index as f64 * 60.0, "y": 120.0 })), None).unwrap_or_else(|error| panic!("addPartKind {index}: {error:?}"));
    }
    assert_eq!(part_count(app), count, "every catalogue add really wrote the document");
    projection_of(app).get("parts").and_then(Value::as_array).map(|parts| parts.iter().filter_map(|part| part.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default()
}

/// 🛍️ LAW: the example switcher really switches, and it can NEVER produce the framework's opaque
/// "exceeds fixed semantic work capacity" fault: `extent` answers `Some(_)` for every shipped example id,
/// refusing an oversized switch with the localized `example_too_large` notice on its first step instead.
#[semio_framework_async_macros::async_test]
async fn set_active_example_switches_the_document_and_never_faults_on_capacity() {
    let mut app = Box::new(app_with_registry());
    for (example_id, document) in [("", empty_document()), ("concrete-forest", concrete_forest_example_document()), ("nakagin", nakagin_example_document()), ("capsule-dream", capsule_dream_example_document())] {
        dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": example_id })), None).unwrap_or_else(|error| panic!("setActiveExample {example_id} must reach the document, not fault: {error:?}"));
        assert_eq!(part_count(&app), document.parts.len(), "setActiveExample {example_id} really replaced the document");
    }
    let snapshot = Puzzle5dPlaySnapshot(serde_json::to_value(concrete_forest_example_document()).expect("document serializes"));
    let interaction = protocol::InteractionState::default();
    let work = Puzzle5dSetActiveExampleWork::default();
    for example_id in ["", "concrete-forest", "nakagin", "capsule-dream"] {
        let command = Puzzle5dCommand::from_action("setActiveExample", Some(dsl::json!({ "exampleId": example_id })), None);
        let extent = crate::retained_command::PuzzleCommandWork::extent(&work, &command, &snapshot, &interaction);
        assert!(extent.is_some(), "setActiveExample {example_id} must declare an extent — `None` is the opaque capacity fault this slice replaced with a notice");
        assert!(extent.is_some_and(|extent| extent <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS), "setActiveExample {example_id} must stay inside the fixed work ceiling");
    }
    let source = include_str!("../../🦀️.rs");
    assert!(source.contains("labels.example_too_large.as_str()"), "the oversized switch completes with a localized notice");
    close_app(&mut app);
}

/// 🎯️ LAW: `focusSelection` publishes the ADDRESSED pane's own camera — the world pane's orbit target and
/// the board pane's flat centre — through that pane's `WindowConfig`, and never edits the document.
#[semio_framework_async_macros::async_test]
async fn focus_selection_publishes_the_addressed_pane_camera() {
    let mut app = Box::new(app_with_registry());
    let parts = seeded_parts(&mut app, 2);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &parts[1]).expect("select one part");
    let world = "puzzle5d-focus-world";
    let before = world_window_config(&mut app, world).await.camera3d;
    let focused = dispatch(&mut app, "focusSelection", Some(&dsl::json!({ "windowId": world3d::WINDOW_KIND_ID })), Some(world)).expect("focusSelection in the world pane");
    assert!(focused.mutations.is_empty(), "a camera focus is never a document edit");
    let after = world_window_config(&mut app, world).await.camera3d;
    assert_ne!(after.target, before.target, "the world pane's orbit target moved onto the selection");

    let board = "puzzle5d-focus-board";
    let board_before = board_window_config(&mut app, board).await.camera2d;
    dispatch(&mut app, "focusSelection", Some(&dsl::json!({ "windowId": board2d::WINDOW_KIND_ID })), Some(board)).expect("focusSelection in the board pane");
    let board_after = board_window_config(&mut app, board).await.camera2d;
    assert!(board_after != board_before, "the board pane's flat camera moved onto the selection");
    close_app(&mut app);
}

/// 🚀️ LAW (dual pose): a world translate moves BOTH poses of every selected part — the volume origin by
/// the world delta and the flat pin by the same delta through the one board↔world scale — so the board
/// never falls behind the world. A LOCKED part refuses the gesture with a notice and no edit.
#[semio_framework_async_macros::async_test]
async fn translate_selection_moves_both_poses_and_refuses_a_locked_part() {
    let mut app = Box::new(app_with_registry());
    let parts = seeded_parts(&mut app, 1);
    let part_id = parts[0].clone();
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &part_id).expect("select the part");
    let read = |app: &Puzzle5dApp, id: &str| -> (f64, f64, [f64; 3]) {
        let projection = projection_of(app);
        let row = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.iter().find(|part| part.get("id").and_then(Value::as_str) == Some(id))).expect("part row").clone();
        let flat = row.get("2d").expect("flat pose").clone();
        let origin = row.get("3d").and_then(|part| part.get("origin")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
        (flat.get("x").and_then(Value::as_f64).unwrap_or_default(), flat.get("y").and_then(Value::as_f64).unwrap_or_default(), origin)
    };
    let (flat_x, flat_y, origin) = read(&app, &part_id);
    dispatch(&mut app, "translateSelection", Some(&dsl::json!({ "dx": 0.5, "dy": -0.25, "dz": 0.0 })), Some(world3d::WINDOW_KIND_ID)).expect("translateSelection");
    let (next_x, next_y, next_origin) = read(&app, &part_id);
    assert!((next_origin[0] - (origin[0] + 0.5)).abs() < 1e-9 && (next_origin[1] - (origin[1] - 0.25)).abs() < 1e-9, "the volume origin moved by the world delta");
    assert!((next_x - (flat_x + 0.5 / PUZZLE5D_FLAT_TO_WORLD)).abs() < 1e-9, "the flat pose tracked the world delta on x");
    assert!((next_y - (flat_y + 0.25 / PUZZLE5D_FLAT_TO_WORLD)).abs() < 1e-9, "the flat pose tracked the world delta on y");

    dispatch(&mut app, "setSelectionFlag", Some(&dsl::json!({ "flag": "locked", "value": true })), None).expect("lock the selection");
    let locked = projection_of(&app);
    let refusal = dispatch(&mut app, "translateSelection", Some(&dsl::json!({ "dx": 5.0 })), Some(world3d::WINDOW_KIND_ID)).expect("a locked transform refuses, it never faults");
    assert!(refusal.mutations.is_empty(), "a locked part publishes no edit");
    assert_eq!(projection_of(&app), locked, "a locked part did not move");
    assert!(refusal.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "a locked refusal is visible: {:?}", refusal.requested_effects);
    close_app(&mut app);
}

/// 🎲️ LAW (board fold): one `applyBoardEvents` batch creates an edge, moves a node and then deletes that
/// node — and the delete takes the part AND every fastener incident on it in the SAME edit.
#[semio_framework_async_macros::async_test]
async fn board_node_delete_removes_the_part_and_its_fasteners() {
    let mut app = Box::new(app_with_registry());
    let parts = seeded_parts(&mut app, 2);
    let (victim, peer) = (parts[0].clone(), parts[1].clone());
    let create = dsl::json!({ "windowId": board2d::WINDOW_KIND_ID, "eventsJson": format!("[{{\"name\":\"edgeCreate\",\"payload\":{{\"id\":\"kante-ä\",\"source\":\"{victim}:v0\",\"target\":\"{peer}:v0\"}}}}]") });
    dispatch(&mut app, "applyBoardEvents", Some(&create), Some(board2d::WINDOW_KIND_ID)).expect("applyBoardEvents edgeCreate");
    assert_eq!(projection_of(&app).get("fasteners").and_then(Value::as_array).map_or(0, Vec::len), 1, "the board edge really became a fastener");

    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &victim).expect("select the victim");
    let delete = dsl::json!({ "windowId": board2d::WINDOW_KIND_ID, "eventsJson": format!("[{{\"name\":\"nodeDelete\",\"payload\":{{\"id\":\"{victim}\"}}}}]") });
    dispatch(&mut app, "applyBoardEvents", Some(&delete), Some(board2d::WINDOW_KIND_ID)).expect("applyBoardEvents nodeDelete");
    assert_eq!(part_count(&app), 1, "the node is gone from the document");
    assert_eq!(projection_of(&app).get("fasteners").and_then(Value::as_array).map_or(0, Vec::len), 0, "the fastener incident on the node went with it");
    assert!(
        app.interaction_state().await.selection.get(PUZZLE5D_INTERACTION_DOMAIN).is_none_or(|selection| !selection.ids.contains(&victim)),
        "the deleted node is no longer selected"
    );
    close_app(&mut app);
}

/// 🛍️ LAW: a catalogue add really adds and re-selects — the placed part is what every follow-up verb then
/// addresses, instead of whatever was selected before.
#[semio_framework_async_macros::async_test]
async fn add_part_kind_creates_a_part_and_reselects_it() {
    let mut app = Box::new(app_with_registry());
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": "" })), None).expect("empty document");
    let before = part_count(&app);
    dispatch(&mut app, "addPartKind", Some(&dsl::json!({ "partKind": "Part" })), None).expect("addPartKind");
    assert_eq!(part_count(&app), before + 1, "the catalogue add really wrote the document");
    let created = projection_of(&app).get("parts").and_then(Value::as_array).and_then(|parts| parts.last()).and_then(|part| part.get("id")).and_then(Value::as_str).expect("created id").to_string();
    let selection = app.interaction_state().await.selection.get(PUZZLE5D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE5D_GRANULARITY_PART, "the placed part is selected at part granularity");
    assert_eq!(selection.ids, vec![created], "the placed part is what the next gesture addresses");
    close_app(&mut app);
}

/// 🧬️ LAW: `selectSameKindSelection` widens the live selection to every part of the clicked part's kind
/// through ONE framework interaction write, and edits no document.
#[semio_framework_async_macros::async_test]
async fn select_same_kind_widens_the_live_selection() {
    let mut app = Box::new(app_with_registry());
    let parts = seeded_parts(&mut app, 3);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &parts[0]).expect("select one part");
    let widened = dispatch(&mut app, "selectSameKindSelection", None, None).expect("selectSameKindSelection");
    assert!(widened.mutations.is_empty(), "widening the selection is never a document edit");
    let selection = app.interaction_state().await.selection.get(PUZZLE5D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.ids.len(), parts.len(), "every part of that kind is selected now");
    close_app(&mut app);
}

/// 🔗️ LAW (fastener CRUD): edit and delete both reach the document through the real retained factory, and
/// each one changes it.
#[semio_framework_async_macros::async_test]
async fn fastener_crud_reaches_the_document() {
    let mut app = Box::new(app_with_registry());
    let parts = seeded_parts(&mut app, 2);
    let create = dsl::json!({ "windowId": board2d::WINDOW_KIND_ID, "eventsJson": format!("[{{\"name\":\"edgeCreate\",\"payload\":{{\"id\":\"kante-ß\",\"source\":\"{}:v0\",\"target\":\"{}:v0\"}}}}]", parts[0], parts[1]) });
    dispatch(&mut app, "applyBoardEvents", Some(&create), Some(board2d::WINDOW_KIND_ID)).expect("applyBoardEvents edgeCreate");

    dispatch(&mut app, "editFastener", Some(&dsl::json!({ "id": "kante-ß", "gap": 0.75 })), None).expect("editFastener");
    let gap = projection_of(&app)
        .get("fasteners")
        .and_then(Value::as_array)
        .and_then(|rows| rows.iter().find(|row| row.get("id").and_then(Value::as_str) == Some("kante-ß")))
        .and_then(|row| row.get("gap"))
        .and_then(Value::as_f64);
    assert_eq!(gap, Some(0.75), "editFastener really wrote the fastener geometry");

    dispatch(&mut app, "deleteFastener", Some(&dsl::json!({ "id": "kante-ß" })), None).expect("deleteFastener");
    assert_eq!(projection_of(&app).get("fasteners").and_then(Value::as_array).map_or(0, Vec::len), 0, "deleteFastener really removed it");
    close_app(&mut app);
}

/// 🩹️ LAW (inspector write-back): `patchPart` writes the addressed field, and a patch that addresses
/// nothing completes with ONE notice instead of the silent no-op the pre-migration arm fell through with.
#[semio_framework_async_macros::async_test]
async fn patch_part_writes_the_document_and_notices_an_inapplicable_edit() {
    let mut app = Box::new(app_with_registry());
    let parts = seeded_parts(&mut app, 1);
    let part_id = parts[0].clone();
    dispatch(&mut app, "patchPart", Some(&dsl::json!({ "partId": part_id, "field": "x", "value": 42.0 })), None).expect("patchPart x");
    let flat_x = projection_of(&app)
        .get("parts")
        .and_then(Value::as_array)
        .and_then(|parts| parts.iter().find(|part| part.get("id").and_then(Value::as_str) == Some(part_id.as_str())))
        .and_then(|part| part.get("2d"))
        .and_then(|flat| flat.get("x"))
        .and_then(Value::as_f64);
    assert_eq!(flat_x, Some(42.0), "patchPart really wrote the flat pose");

    let before = projection_of(&app);
    let refusal = dispatch(&mut app, "patchPart", Some(&dsl::json!({ "partId": "no-such-part", "field": "x", "value": 1.0 })), None).expect("an unaddressed patch refuses, it never faults");
    assert!(refusal.mutations.is_empty());
    assert_eq!(projection_of(&app), before, "an unaddressed patch leaves the document alone");
    assert!(refusal.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "an inapplicable patch is visible: {:?}", refusal.requested_effects);
    close_app(&mut app);
}
//#endregion 🧩️ArtifactLaneVerbs

//#region 📤️📥️DocumentIO
/// 📚️ LAW (diagnosis-first): the three SHIPPED examples really carry puzzle 5d content. Every other
/// law in this region, and most of the clipboard, dialog and context-menu laws, read a shipped
/// document — so when the examples are empty they all fail at once and none of them names WHY.
///
/// 🧊️ `Puzzle5dDocument` takes `#[serde(default)]` on `parts`/`fasteners` and ignores unknown members,
/// so a document of the WRONG artifact shape (puzzle 3d's `objects`/`attractions`) deserializes
/// silently into an empty 5d document instead of failing. This law is the one place that refuses it.
#[test]
fn every_shipped_example_document_really_carries_its_content() {
    for (id, document, parts, fasteners, catalogs) in [
        (PUZZLE5D_EXAMPLE_CONCRETE_FOREST, concrete_forest_example_document(), 1, 0, false),
        (PUZZLE5D_EXAMPLE_NAKAGIN, nakagin_example_document(), 180, 179, true),
        (PUZZLE5D_EXAMPLE_CAPSULE_DREAM, capsule_dream_example_document(), 2880, 2864, true),
    ] {
        assert_eq!(document.schema, PUZZLE5D_SCHEMA, "{id} declares the puzzle 5d schema");
        assert!(document.label.as_deref().is_some_and(|label| !label.is_empty()), "{id} carries a label (the export filename is its slug)");
        assert_eq!(document.parts.len(), parts, "{id} carries its own part census — a wrong count means the example asset is not this artifact's document");
        assert_eq!(document.fasteners.len(), fasteners, "{id} carries its own fastener census");
        assert_eq!(document.kind_catalogs.is_some(), catalogs, "{id} authors kindCatalogs exactly as its shipped asset does");
    }
}

/// 📤️📥️ LAW: exporting a document and importing those exact bytes back yields a BYTE-IDENTICAL
/// document, for every shipped example — including the one above the transport budget, because the
/// projection and the paged reassembly are the same law the transport only carries. The reassembly
/// runs through the paged cursor (`puzzle5d_import_root_value`), never one contiguous string, so this
/// also proves the cursor reads every member of the largest example correctly.
#[test]
fn export_import_round_trips_every_shipped_example_byte_for_byte() {
    for document in [concrete_forest_example_document(), nakagin_example_document(), capsule_dream_example_document()] {
        // 🚦️ Non-vacuity: an EMPTY document round-trips trivially, so this law would pass while proving
        // nothing the moment the shipped examples regress (see the diagnosis law above).
        assert!(!document.parts.is_empty(), "a round trip over an empty document proves nothing");
        let exported = export_fixture::puzzle5d_export_json(&document);
        let pages = import_fixture::puzzle5d_import_chunks(&exported);
        assert!(pages.iter().all(|page| page.len() <= import_fixture::PUZZLE5D_IMPORT_CHUNK_BYTES), "every chunk fits the wire page");
        assert_eq!(pages.iter().map(String::len).sum::<usize>(), exported.len(), "chunking loses no byte");
        let root = import_fixture::puzzle5d_import_root_value(&pages).expect("paged root parse");
        let reimported: Puzzle5dDocument = serde_json::from_value(root).expect("the reassembled root is a puzzle 5d document");
        assert_eq!(export_fixture::puzzle5d_export_json(&reimported), exported, "{} did not round-trip byte-for-byte", document.label.clone().unwrap_or_default());
    }
}

/// 📤️ LAW: the export lane is chosen by payload size, not by guesswork — inline under one guest wire
/// page, segmented above it, and a NOTICE above what one segmented download may carry. The three
/// shipped examples are deliberately one case each.
#[test]
fn export_publication_picks_its_lane_by_payload_size() {
    let inline_budget = export_fixture::puzzle5d_export_inline_budget_bytes();
    let segmented_budget = export_fixture::puzzle5d_export_segmented_budget_bytes().expect("segmented budget");
    assert!(inline_budget < segmented_budget, "the inline page must be the narrower of the two budgets");
    for document in [concrete_forest_example_document(), nakagin_example_document(), capsule_dream_example_document()] {
        let bytes = export_fixture::puzzle5d_export_json(&document).len();
        let publication = export_fixture::puzzle5d_export_publication(&document).expect("publication resolves");
        let arm = match publication {
            export_fixture::Puzzle5dExportPublication::Inline(_) => "inline",
            export_fixture::Puzzle5dExportPublication::Segmented(_) => "segmented",
            export_fixture::Puzzle5dExportPublication::Refused(_) => "refused",
        };
        let expected = if bytes <= inline_budget {
            "inline"
        } else if bytes <= segmented_budget {
            "segmented"
        } else {
            "refused"
        };
        assert_eq!(arm, expected, "{} is {bytes} B", document.label.clone().unwrap_or_default());
    }
    // 🗂️ Concrete Forest is the inline case and Capsule Dream the refusal case; if either stopped being
    // so the budgets moved, and this law would otherwise pass while proving nothing.
    assert!(export_fixture::puzzle5d_export_json(&concrete_forest_example_document()).len() <= inline_budget, "Concrete Forest is the inline case");
    assert!(export_fixture::puzzle5d_export_json(&capsule_dream_example_document()).len() > segmented_budget, "Capsule Dream is the refusal case");
}

/// 📤️ LAW: the download filename is the document's own label as a slug, so exporting two examples
/// never lands as two files with one name; a document with no label keeps the app-generic name.
#[test]
fn export_filename_follows_the_document_label() {
    assert_eq!(export_fixture::puzzle5d_export_filename(&concrete_forest_example_document()), "concrete-forest.json");
    assert_eq!(export_fixture::puzzle5d_export_filename(&nakagin_example_document()), "nakagin-capsule-tower.json");
    assert_eq!(export_fixture::puzzle5d_export_filename(&capsule_dream_example_document()), "capsule-dream.json");
    assert_eq!(export_fixture::puzzle5d_export_filename(&empty_document()), "puzzle-5d.json");
}

/// 📥️ LAW: a chunk that does not CLOSE its run stages and changes nothing — no document edit, no
/// history row — and the closing chunk lands the whole document as ONE edit.
#[semio_framework_async_macros::async_test]
async fn import_stages_every_chunk_and_only_the_closing_one_edits_the_document() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": "" })), None).expect("empty document");
    assert_eq!(part_count(&app), 0);
    // 📄️ The law needs a document that really chunks — the first shipped one whose export spans more
    // than one wire page, so it keeps holding whichever example grows past the page next.
    let (target, pages) = [concrete_forest_example_document(), nakagin_example_document()]
        .into_iter()
        .find_map(|document| {
            let pages = import_fixture::puzzle5d_import_chunks(&export_fixture::puzzle5d_export_json(&document));
            (pages.len() > 1).then_some((document, pages))
        })
        .expect("a shipped document whose export spans more than one wire page");
    let count = pages.len();
    for (index, page) in pages.iter().enumerate() {
        let args = dsl::json!({ "payload": page.as_str(), "name": "chunked.json", "chunk": index as f64, "chunkCount": count as f64 });
        let result = dispatch(&mut app, "importFixture", Some(&args), None).expect("import chunk");
        if index + 1 < count {
            assert!(result.mutations.is_empty(), "chunk {index} of {count} staged and must edit nothing");
            assert_eq!(part_count(&app), 0, "chunk {index} of {count} must leave the document untouched");
        }
    }
    assert_eq!(part_count(&app), target.parts.len(), "the closing chunk landed the whole document");
    semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
    assert_eq!(part_count(&app), 0, "the whole import is ONE undoable edit");
    close_app(&mut app);
}

/// 📥️ LAW: every refusal answers. A document over the import budget, a chunk that skips its cursor
/// and a payload that is not a puzzle 5d document each publish a named notice and leave the document
/// exactly as it was — never a silent no-op, and never a fault.
#[semio_framework_async_macros::async_test]
async fn every_refused_import_publishes_a_notice_and_changes_nothing() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let cases = [
        // 🕳️ A chunk past the cursor with no open run.
        ("gap", dsl::json!({ "payload": "{\"schema\":\"puzzle.5d\"", "name": "gap.json", "chunk": 3.0, "chunkCount": 4.0 })),
        // 📦️ A run claiming more chunks than the whole budget admits.
        ("envelope", dsl::json!({ "payload": "{}", "name": "huge.json", "chunk": 0.0, "chunkCount": (import_fixture::PUZZLE5D_IMPORT_MAXIMUM_CHUNKS + 1) as f64 })),
        // 🔤️ One whole chunk that is not a puzzle 5d document.
        ("payload", dsl::json!({ "payload": "{\"schema\":\"note.v1\",\"body\":\"\"}", "name": "note.json", "chunk": 0.0, "chunkCount": 1.0 })),
    ];
    for (case, args) in cases {
        let result = dispatch(&mut app, "importFixture", Some(&args), None).expect("a refused import answers, it never faults");
        assert!(result.mutations.is_empty(), "the {case} refusal emits no mutation");
        assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "the {case} refusal is visible: {:?}", result.requested_effects);
        assert_eq!(projection_of(&app), before, "the {case} refusal leaves the document alone");
    }
    close_app(&mut app);
}

/// 📥️ LAW: the staging area is keyed by `(name, chunkCount)` — a chunk the run already admitted is a
/// RETRANSMISSION acknowledged at the cursor it stands on, not a gap that costs the whole upload.
#[test]
fn a_retransmitted_import_chunk_is_acknowledged_at_the_cursor() {
    let envelope = |chunk: usize| import_fixture::Puzzle5dImportEnvelope { name: "retransmit.json".into(), chunk, chunk_count: 3 };
    assert_eq!(import_fixture::stage_import_chunk(&envelope(0), "{\"schema\":\"puzzle.5d\","), Ok(import_fixture::Puzzle5dImportStep::Staged { next_chunk: 1, chunk_count: 3 }));
    assert_eq!(import_fixture::stage_import_chunk(&envelope(1), "\"parts\":[],"), Ok(import_fixture::Puzzle5dImportStep::Staged { next_chunk: 2, chunk_count: 3 }));
    assert_eq!(import_fixture::stage_import_chunk(&envelope(1), "\"parts\":[],"), Ok(import_fixture::Puzzle5dImportStep::Staged { next_chunk: 2, chunk_count: 3 }), "a retransmission holds the cursor");
    assert_eq!(import_fixture::stage_import_chunk(&envelope(2), "\"fasteners\":[]}"), Ok(import_fixture::Puzzle5dImportStep::Complete(vec!["{\"schema\":\"puzzle.5d\",".into(), "\"parts\":[],".into(), "\"fasteners\":[]}".into()])));
    assert!(import_fixture::staged_import_runs().iter().all(|run| run.0 != "retransmit.json"), "a closed run releases its slot");
}

/// 🗂️ LAW: `openImportFixture` asks the HOST for a file and edits nothing — the picker re-dispatches
/// `importFixture`, which is the verb that owns the edit.
#[semio_framework_async_macros::async_test]
async fn open_import_fixture_requests_a_file_and_edits_nothing() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let result = dispatch(&mut app, "openImportFixture", None, None).expect("openImportFixture");
    assert!(result.mutations.is_empty(), "a file picker is not a document edit");
    assert_eq!(projection_of(&app), before);
    let requested = result
        .requested_effects
        .iter()
        .find_map(|effect| match effect {
            Effect::RequestFileOpen { import_action, accept, multiple, .. } => Some((import_action.clone(), accept.clone(), *multiple)),
            _ => None,
        })
        .expect("openImportFixture requests a file open");
    assert_eq!(requested.0, "importFixture", "the picker re-dispatches the import verb");
    assert!(requested.1.contains("json"));
    assert!(!requested.2, "one document at a time");
    close_app(&mut app);
}
//#endregion 📤️📥️DocumentIO

//#region 🗨️AddPartDialog
/// 🗨️ LAW: `openAddPartDialog` opens the DECLARED dialog and edits nothing; the dialog submits the
/// existing `addPartKind` operation, so the dialog is a form over a verb, never a second verb.
#[semio_framework_async_macros::async_test]
async fn open_add_part_dialog_opens_the_declared_dialog_and_edits_nothing() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let result = dispatch(&mut app, "openAddPartDialog", None, None).expect("openAddPartDialog");
    assert!(result.mutations.is_empty(), "opening a dialog is not a document edit");
    assert_eq!(projection_of(&app), before);
    let opened = result.requested_effects.iter().find_map(|effect| match effect {
        Effect::OpenDialog { dialog_id, .. } => Some(dialog_id.clone()),
        _ => None,
    });
    assert_eq!(opened.as_deref(), Some(open_add_part_dialog::PUZZLE5D_ADD_PART_DIALOG));
    let definition = create_puzzle5d_app();
    let dialog = definition.dialogs.iter().find(|dialog| dialog.id == open_add_part_dialog::PUZZLE5D_ADD_PART_DIALOG).expect("the opened dialog is declared");
    assert_eq!(dialog.submit_action.as_str(), "addPartKind", "the dialog submits the existing add-part verb");
    close_app(&mut app);
}

/// 📇️ Every `ActionDefinition` the built manifest carries — bare app actions are cloned onto every
/// window kind by `build_definition`, so one window's set is the whole declared vocabulary.
fn declared_actions(definition: &semio_framework_plugin::AppDefinition) -> Vec<&semio_framework_plugin::ActionDefinition> {
    let mut seen = std::collections::BTreeMap::new();
    for window in definition.window_kinds.iter() {
        for action in &window.actions {
            seen.entry(action.id.as_str()).or_insert(action);
        }
    }
    seen.into_values().collect()
}

/// 🗨️ LAW: the dialog enumerates LIVE part kinds from the shipped documents' own `kindCatalogs` —
/// every option is a kind some document really declares, the literal placeholder is gone, and the
/// dialog and the standalone arg form offer the SAME select so they cannot drift.
#[semio_framework_async_macros::async_test]
async fn add_part_dialog_enumerates_live_part_kinds() {
    let definition = create_puzzle5d_app();
    let dialog = definition.dialogs.iter().find(|dialog| dialog.id == open_add_part_dialog::PUZZLE5D_ADD_PART_DIALOG).expect("addPart dialog");
    let arg = dialog.args.iter().find(|arg| arg.id == "partKind").expect("the dialog declares the partKind select");
    assert!(arg.required, "a kind must be picked before the dialog submits");
    let options = |arg: &semio_framework_plugin::ActionArgDef| {
        let semio_framework_plugin::ArgSchema::String { options, .. } = &arg.schema else { panic!("the partKind arg must stay a string select") };
        options.iter().map(|option| option.value.clone()).collect::<Vec<_>>()
    };
    let offered = options(arg);
    let offered: Vec<&str> = offered.iter().map(String::as_str).collect();
    assert!(!offered.is_empty() && offered.len() <= PUZZLE5D_PART_KIND_OPTIONS_MAX);
    assert!(!offered.contains(&"Part"), "the static placeholder kind is gone: {offered:?}");
    let declared: std::collections::BTreeSet<String> = [concrete_forest_example_document(), nakagin_example_document(), capsule_dream_example_document()]
        .iter()
        .flat_map(|document| {
            document
                .kind_catalogs
                .as_ref()
                .and_then(|catalogs| catalogs.get("parts"))
                .and_then(serde_json::Value::as_array)
                .map(|entries| entries.iter().filter_map(|entry| entry.get("id").and_then(serde_json::Value::as_str).map(str::to_string)).collect::<Vec<_>>())
                .unwrap_or_default()
        })
        .chain(
            [concrete_forest_example_document(), nakagin_example_document(), capsule_dream_example_document()]
                .iter()
                .flat_map(|document| document.parts.iter().map(|part| part.part_kind.clone()).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
        .collect();
    assert!(offered.iter().all(|option| declared.contains(*option)), "every offered kind is declared by a shipped catalog or inferred from a shipped part: {offered:?}");
    assert!(declared.iter().any(|kind| offered.contains(&kind.as_str())), "at least one real catalog kind is reachable");
    let actions = declared_actions(&definition);
    let form = actions.iter().find(|action| action.id == "addPartKind").expect("addPartKind is declared");
    assert!(!form.in_palette, "the parametrized verb is not a second palette row beside the dialog");
    assert_eq!(options(form.args.iter().find(|arg| arg.id == "partKind").expect("the arg form declares the same select")), offered.iter().map(|id| (*id).to_string()).collect::<Vec<_>>(), "the dialog and the arg form offer the SAME kinds");
    let prompt = actions.iter().find(|action| action.id == "openAddPartDialog").expect("openAddPartDialog is declared");
    assert!(prompt.in_palette, "the dialog row is the one the palette offers");
}
//#endregion 🗨️AddPartDialog

//#region 🖱️ContextMenuRows
/// 🖱️ Builds one context-menu request for a surface selection of `(domain, ids)` groups.
fn context_menu_of(app: &mut Puzzle5dApp, groups: Vec<(&str, Vec<String>)>) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget {
            surface_id: world3d::WINDOW_KIND_ID.into(),
            kind: "world3d".into(),
            hits: vec![],
            selection: groups.into_iter().map(|(domain, ids)| ContextMenuSelectionGroup { domain: domain.into(), ids }).collect(),
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    semio_framework::io::resolve_ready(app.context_menu(&request, &Default::default()))
}

/// 🖱️ Every action id a menu row carries, at any depth.
fn context_menu_actions(rows: &[semio_framework_plugin::ContextMenuItemSpec]) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack: Vec<&semio_framework_plugin::ContextMenuItemSpec> = rows.iter().collect();
    while let Some(row) = stack.pop() {
        if let Some(action) = row.action.as_deref() {
            out.push(action.to_string());
        }
        if let Some(children) = row.children.as_ref() {
            stack.extend(children.iter());
        }
    }
    out
}

/// 🖱️ LAW: EVERY context-menu row resolves to something the app can really dispatch — a declared
/// action classified `Migrated` (an unmigrated verb is hard-dead at the dispatch gate, so a row
/// pointing at one is a visibly dead menu entry), or one of the four framework-reserved verbs this
/// menu is allowed to name. This is the law the 3d `zoomToSelection` defect would have caught.
#[semio_framework_async_macros::async_test]
async fn every_context_menu_row_resolves_to_a_live_verb() {
    let definition = create_puzzle5d_app();
    let actions = declared_actions(&definition);
    let migrated: std::collections::BTreeSet<&str> = actions
        .iter()
        .filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated)
        .map(|action| action.id.as_str())
        .collect();
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let part_id = first_part_id(&app);
    let projection = projection_of(&app);
    let grip_id = projection
        .get("parts")
        .and_then(Value::as_array)
        .and_then(|parts| parts.iter().find_map(|part| Some(puzzle5d_grip_full_id(part.get("id")?.as_str()?, part.get("grips")?.as_array()?.first()?.get("id")?.as_str()?))))
        .expect("a shipped part carries a grip");
    let fastener_id = projection.get("fasteners").and_then(Value::as_array).and_then(|rows| rows.first()).and_then(|row| row.get("id")).and_then(Value::as_str).expect("a shipped fastener").to_string();
    let surfaces = [
        vec![],
        vec![(PUZZLE5D_GRANULARITY_PART, vec![part_id.clone()])],
        vec![(PUZZLE5D_GRANULARITY_GRIP, vec![grip_id.clone()])],
        vec![(PUZZLE5D_GRANULARITY_FASTENER, vec![fastener_id.clone()])],
    ];
    for groups in surfaces {
        let rows = context_menu_of(&mut app, groups.clone());
        assert!(!rows.is_empty(), "every selection granularity offers rows, {groups:?} offered none");
        for action in context_menu_actions(&rows) {
            assert!(
                migrated.contains(action.as_str()) || PUZZLE5D_CONTEXT_MENU_RESERVED_ACTIONS.contains(&action.as_str()),
                "context-menu row {action:?} for {groups:?} is neither a Migrated app action nor a declared reserved verb"
            );
        }
    }
    close_app(&mut app);
}

/// 🖱️ LAW: the rows offered depend on WHAT is selected — an empty surface offers the no-subject
/// verbs, a part the selection vocabulary, a grip the suggestions entry point, a fastener its own
/// delete. Each branch names the verbs that branch exists for.
#[semio_framework_async_macros::async_test]
async fn context_menu_rows_follow_the_selected_granularity() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let part_id = first_part_id(&app);
    let projection = projection_of(&app);
    let grip_id = projection
        .get("parts")
        .and_then(Value::as_array)
        .and_then(|parts| parts.iter().find_map(|part| Some(puzzle5d_grip_full_id(part.get("id")?.as_str()?, part.get("grips")?.as_array()?.first()?.get("id")?.as_str()?))))
        .expect("a shipped part carries a grip");
    let fastener_id = projection.get("fasteners").and_then(Value::as_array).and_then(|rows| rows.first()).and_then(|row| row.get("id")).and_then(Value::as_str).expect("a shipped fastener").to_string();

    let empty = context_menu_actions(&context_menu_of(&mut app, vec![]));
    for expected in ["selectAll", "paste", "openAddPartDialog"] {
        assert!(empty.contains(&expected.to_string()), "an empty surface offers {expected}, got {empty:?}");
    }
    assert!(!empty.contains(&"deleteSelection".to_string()), "nothing selected means nothing to delete: {empty:?}");

    let part = context_menu_actions(&context_menu_of(&mut app, vec![(PUZZLE5D_GRANULARITY_PART, vec![part_id])]));
    for expected in ["duplicateSelection", "copy", "cut", "selectSameKindSelection", "focusSelection", "setSelectionFlag", "deleteSelection"] {
        assert!(part.contains(&expected.to_string()), "a part selection offers {expected}, got {part:?}");
    }

    let grip_rows = context_menu_of(&mut app, vec![(PUZZLE5D_GRANULARITY_GRIP, vec![grip_id.clone()])]);
    let grip = context_menu_actions(&grip_rows);
    assert!(grip.contains(&"targetBrushSuggestions".to_string()), "one selected grip offers the suggestions entry point, got {grip:?}");
    let suggest = grip_rows.iter().find(|row| row.id == "suggest").expect("the suggest row");
    assert_eq!(suggest.args.as_ref().and_then(|args| args.get("fullId")).and_then(dsl::DslValue::as_str), Some(grip_id.as_str()), "the suggest row names the grip it was opened on");

    let fastener_rows = context_menu_of(&mut app, vec![(PUZZLE5D_GRANULARITY_FASTENER, vec![fastener_id.clone()])]);
    let fastener = context_menu_actions(&fastener_rows);
    assert_eq!(fastener, vec!["deleteFastener".to_string()], "a fastener offers exactly its own delete, got {fastener:?}");
    let delete = fastener_rows.iter().find(|row| row.id == "delete").expect("the delete row");
    assert_eq!(delete.args.as_ref().and_then(|args| args.get("id")).and_then(dsl::DslValue::as_str), Some(fastener_id.as_str()), "the delete row names the fastener it was opened on");
    assert_eq!(delete.destructive, Some(true));
    close_app(&mut app);
}
//#endregion 🖱️ContextMenuRows

//#region 📋️ClipboardLaws
/// 🔒️ LAW: a CUT copies a locked part but never removes it — a lock exists to refuse a destructive
/// gesture, and the fasteners of a surviving part survive with it. The unlocked siblings still go.
#[test]
fn a_cut_copies_a_locked_part_but_never_removes_it() {
    let mut document = nakagin_example_document();
    assert!(document.parts.len() >= 2, "the locked-cut law needs at least two parts");
    let locked_id = document.parts[0].id.clone();
    let free_id = document.parts[1].id.clone();
    document.parts[0].part_2d.locked = Some(true);
    let snapshot = Puzzle5dPlaySnapshot(serde_json::to_value(&document).expect("document serializes"));
    let ids = vec![locked_id.clone(), free_id.clone()];
    let fragment = puzzle5d_copy_fragment(&snapshot, &ids, &[]).expect("copy both parts");
    assert!(fragment.dsl_text.contains(locked_id.as_str()), "the locked part is still COPIED");
    let operations = puzzle5d_cut_operations(&snapshot, &ids, &[]);
    let deleted: Vec<&str> = operations
        .iter()
        .filter_map(|operation| match operation {
            Puzzle5dMutation::DeletePart(delete) => Some(delete.id.as_str()),
            _ => None,
        })
        .collect();
    assert!(deleted.contains(&free_id.as_str()), "the unlocked part is cut: {deleted:?}");
    assert!(!deleted.contains(&locked_id.as_str()), "the locked part is NOT cut: {deleted:?}");
}

/// 📋️ LAW: a paste preserves BOTH poses and offsets both by the same delta, keeps the fasteners
/// between copied parts with their endpoints remapped onto the fresh ids, and lands as ONE edit.
#[test]
fn a_paste_preserves_both_poses_and_the_fasteners_between_copied_parts() {
    let document = nakagin_example_document();
    let with_fastener = document.fasteners.first().cloned().expect("the shipped example connects parts");
    let endpoints = vec![owning_part_id_local(&with_fastener.source).to_string(), owning_part_id_local(&with_fastener.target).to_string()];
    let (parts, fasteners) = copy_selection_local(&document, &endpoints, &[]);
    assert!(fasteners.iter().any(|fastener| fastener.id == with_fastener.id), "the fastener BETWEEN two copied parts comes along");
    let delta = (7.0, -3.0);
    let (fresh_parts, fresh_fasteners) = paste_selection_local(&document, &parts, &fasteners, delta);
    assert_eq!(fresh_parts.len(), parts.len());
    for (source, pasted) in parts.iter().zip(&fresh_parts) {
        assert_ne!(source.id, pasted.id, "a pasted part carries a FRESH id");
        assert!(document.parts.iter().all(|existing| existing.id != pasted.id), "a fresh id collides with nothing live");
        assert_eq!((pasted.part_2d.x, pasted.part_2d.y), (source.part_2d.x + delta.0, source.part_2d.y + delta.1), "the flat pose moved by the delta");
        assert_eq!([pasted.part_3d.origin[0], pasted.part_3d.origin[1]], [source.part_3d.origin[0] + delta.0, source.part_3d.origin[1] + delta.1], "the volume pose moved by the SAME delta");
        assert_eq!(pasted.part_3d.origin[2], source.part_3d.origin[2], "the delta is planar");
        assert_eq!(pasted.grips.len(), source.grips.len(), "a pasted part keeps its grips");
    }
    let fresh_ids: Vec<&str> = fresh_parts.iter().map(|part| part.id.as_str()).collect();
    for fastener in &fresh_fasteners {
        assert!(fresh_ids.contains(&owning_part_id_local(&fastener.source)), "an endpoint was remapped onto a fresh part");
        assert!(fresh_ids.contains(&owning_part_id_local(&fastener.target)), "an endpoint was remapped onto a fresh part");
    }
}

/// 📋️ LAW: a default paste (no explicit position) still MOVES the fragment in both poses, so a paste
/// never lands exactly under the original where it is invisible.
#[test]
fn the_default_paste_placement_offsets_the_fragment_in_both_poses() {
    let document = nakagin_example_document();
    let ids: Vec<String> = document.parts.iter().take(2).map(|part| part.id.clone()).collect();
    let (parts, _) = copy_selection_local(&document, &ids, &[]);
    let placement = PastePlacement { anchor: PasteAnchor::Centroid, position: None };
    let delta = paste_delta_2d(&parts, &document.parts, &placement);
    assert!(delta != (0.0, 0.0), "the default placement is a real offset, got {delta:?}");
    let (fresh, _) = paste_selection_local(&document, &parts, &[], delta);
    for (source, pasted) in parts.iter().zip(&fresh) {
        assert_ne!((pasted.part_2d.x, pasted.part_2d.y), (source.part_2d.x, source.part_2d.y), "the flat pose moved");
        assert_ne!([pasted.part_3d.origin[0], pasted.part_3d.origin[1]], [source.part_3d.origin[0], source.part_3d.origin[1]], "the volume pose moved with it");
    }
}
//#endregion 📋️ClipboardLaws
