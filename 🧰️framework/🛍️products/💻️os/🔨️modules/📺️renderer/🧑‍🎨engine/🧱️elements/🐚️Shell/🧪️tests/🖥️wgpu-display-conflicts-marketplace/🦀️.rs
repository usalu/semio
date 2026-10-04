//! 🖥️ LAW: the four panel bodies React has and this renderer had not — the two Display leaves
//! (`framework.display.windows` / `framework.display.layout`, whose dock tabs existed with no router
//! arm at all), the Conflicts leaf's REAL projection, the Tool leaf's measures and the Marketplace
//! leaf's install/uninstall lane — publish retained bodies carrying React's own control ids.
//!
//! Oracles, read off React directly: `buildDisplayWindowsTree`/`buildDisplayLayoutTree`/
//! `buildConflictsTree`/`buildMarketplaceTree` (`📌️ChromePanels/🟦️.tsx`), `buildToolTree` +
//! `windowMeasuresToTreeItems` (`🛠️ShellHelpers/🟦️.tsx`) and `createWorldProjectionTemplates`
//! (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`).

use super::*;

/// 🧪️ The panel-anchor fixture's own shell (one `space` bridge, a bound `studio` session), which is
/// what makes these bodies exercise a REAL session rather than the no-session fallbacks.
fn display_shell() -> ShellState {
    super::panel_anchor_model_tests::host_test_shell()
}

fn display_order_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json"))).expect("window lifecycle fixture")
}

/// 🧾️ Every record key one shell-owned leaf's published body carries, in publication order — each
/// tail is React's own control id (`panel_ui_records` qualifies the key with its surface).
fn published_keys(shell: &ShellState, tab_id: &str, node: &UiNode) -> Vec<String> {
    panel_ui_records(tab_id, node).expect("the leaf body projects").into_iter().map(|record| record.key.as_str().to_string()).collect()
}

/// 🖥️ **The P0 pin.** Both Display leaves are shell-owned AND routed: before this packet the two dock
/// tabs existed while `publish_shell_panel_document` had no arm for either id, so selecting one
/// published nothing at all.
#[test]
fn both_display_leaves_are_shell_owned_and_publish_a_body() {
    let mut shell = display_shell();
    let leaves = shell.shell_owned_panel_leaves();
    for tab_id in [FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, FRAMEWORK_DISPLAY_LAYOUT_TAB_ID] {
        assert!(leaves.contains(&tab_id.to_string()), "🖥️ '{tab_id}' must be a shell-owned leaf");
        let published = shell.publish_shell_panel_document(tab_id).expect("the Display leaf publishes");
        assert!(published.is_some(), "🖥️ '{tab_id}' must publish a retained body, not `Ok(None)`");
    }
}

/// 🖥️ **The Windows census.** One section per window kind carrying React's own
/// `framework.display.windows.<kind>` id, its plain `.kind` leaf, and — only for a `world-3d`
/// surface — the whole `createWorldProjectionTemplates` taxonomy under React's nested id grammar.
#[test]
fn the_display_windows_leaf_publishes_reacts_kind_and_projection_ids() {
    let mut shell = display_shell();
    let keys = published_keys(&shell, FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &shell.build_display_windows_ui());
    assert!(keys.iter().any(|key| key.ends_with("framework.display.windows.main")), "🖥️ a section per window kind, got {keys:?}");
    assert!(keys.iter().any(|key| key.ends_with("framework.display.windows.main.kind")), "🖥️ the plain kind leaf carries React's own id");
    assert!(!keys.iter().any(|key| key.contains(".projection.")), "🖥️ a canvas-2d kind offers no projection rows, exactly as React's `surfaceKind === \"world-3d\"` gate says");

    let session = shell.session.as_mut().expect("the fixture binds a session");
    session.app.window_kinds.first_mut().surface_kind = ui_wgpu::wgpu::SurfaceKind::World3d;
    let keys = published_keys(&shell, FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &shell.build_display_windows_ui());
    for react_id in [
        "framework.display.windows.main.projection.parallel",
        "framework.display.windows.main.projection.parallel.orthographic",
        "framework.display.windows.main.projection.parallel.axonometric",
        "framework.display.windows.main.projection.parallel.axonometric.axonometricIsometric",
        "framework.display.windows.main.projection.perspective",
    ] {
        assert!(keys.iter().any(|key| key.ends_with(react_id)), "🔀️ '{react_id}' is React's own nested template id, got {keys:?}");
    }
    let row_ids: Vec<&str> = keys.iter().filter_map(|key| key.split_once('/').map(|(_, id)| id)).filter(|id| id.starts_with("framework.display.windows.")).collect();
    assert!(row_ids.iter().all(|id| semio_framework::is_element_id(id)), "🆔️ every Display row id is one element id, as React's `childElementId` composes it: {row_ids:?}");
    let perspective = keys.iter().position(|key| key.ends_with("framework.display.windows.main.projection.perspective")).expect("the perspective branch");
    let parallel = keys.iter().position(|key| key.ends_with("framework.display.windows.main.projection.parallel")).expect("the parallel branch");
    let kind_row = keys.iter().position(|key| key.ends_with("framework.display.windows.main.kind")).expect("the kind leaf");
    assert!(perspective < parallel && parallel < kind_row, "🔃️ an Up-flow Tree authors the inverse sibling order so paint resolves kind, Parallel, Perspective top-to-bottom");
}

struct DisplayTreeSceneHost;

impl ui_wgpu::wgpu::SceneHost for DisplayTreeSceneHost {
    fn paint_slot_step(
        &mut self,
        slot: &ui_wgpu::wgpu::SceneSlot<'_>,
        cursor: &mut ui_wgpu::wgpu::ScenePaintCursor,
        _draw: &mut ui_wgpu::wgpu::DrawList,
        _atlas: &mut ui_wgpu::wgpu::FontAtlas,
        _icons: Option<&ui_wgpu::wgpu::IconAtlas>,
    ) -> ui_wgpu::wgpu::ScenePaintStep {
        match cursor.bind(slot.node) {
            Ok(true) => cursor.finish(),
            Ok(false) => ui_wgpu::wgpu::ScenePaintStep::Pending,
            Err(_) => ui_wgpu::wgpu::ScenePaintStep::Fault,
        }
    }
}

fn settle_display_tree(engine: &mut ui_wgpu::wgpu::Ui, atlas: &mut ui_wgpu::wgpu::FontAtlas, witness: u64) {
    settle_tree_surface(engine, atlas, FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, witness);
}

fn settle_tree_surface(engine: &mut ui_wgpu::wgpu::Ui, atlas: &mut ui_wgpu::wgpu::FontAtlas, surface: &str, witness: u64) {
    settle_tree_surface_at_height(engine, atlas, surface, witness, 240.0);
}

fn settle_tree_surface_at_height(engine: &mut ui_wgpu::wgpu::Ui, atlas: &mut ui_wgpu::wgpu::FontAtlas, surface: &str, witness: u64, height: f32) {
    engine.set_viewport(surface, 300.0, height);
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    let reconciled = (0..16_384).any(|_| {
        let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(witness), semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), || Some(0), &mut preview_sequence);
        match engine.step_document_reconcile(surface, "framework", &mut cx) {
            ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Pending => false,
            ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Complete => true,
            step => panic!("Display branch reconcile answered {step:?}"),
        }
    });
    assert!(reconciled, "Display's accepted interaction rebases into its next paint candidate");
    for _ in 0..64 {
        for _ in 0..16_384 {
            let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut preview_sequence);
            if matches!(engine.step_layouts(&pool, atlas, &mut cx), ui_wgpu::wgpu::UiLayoutStep::Idle) {
                break;
            }
        }
        if !engine.layout_is_dirty(surface) {
            break;
        }
        engine.request_layout(surface);
    }
    assert!(!engine.layout_is_dirty(surface), "retained Tree layout settles");
    for _ in 0..131_072 {
        match engine.frame_step::<DisplayTreeSceneHost>(surface, 300.0, height, atlas, None, None) {
            ui_wgpu::wgpu::UiFrameStep::Pending => {}
            ui_wgpu::wgpu::UiFrameStep::Ready => {
                assert!(engine.seal_presented_input_candidate(witness, &[surface.to_string()]));
                assert!(engine.acknowledge_presented_input(witness));
                return;
            }
            step => panic!("Display branch paint answered {step:?}"),
        }
    }
    panic!("Display branch paint did not settle");
}

fn press_display_tree(engine: &mut ui_wgpu::wgpu::Ui, rect: ui_wgpu::wgpu::Rect) {
    let x = rect.x + rect.w * 0.5;
    let y = rect.y + rect.h * 0.5;
    engine.dispatch_event(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, ui_wgpu::wgpu::UiEvent::PointerDown { x, y, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: ui_wgpu::wgpu::EventModifiers::default() });
    engine.dispatch_event(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, ui_wgpu::wgpu::UiEvent::PointerUp { x, y, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: ui_wgpu::wgpu::EventModifiers::default() });
}

#[test]
fn an_expandable_display_template_publishes_a_real_gutter_toggle_and_retires_its_children_when_closed() {
    let mut shell = display_shell();
    shell.session.as_mut().expect("Display fixture session").app.window_kinds.first_mut().surface_kind = ui_wgpu::wgpu::SurfaceKind::World3d;
    let mut body = shell.build_display_windows_ui();
    let UiNode::Stack(panel) = &mut body else { panic!("Display body is a panel stack") };
    let UiNode::Tree(tree) = panel.children.first_mut().expect("Display panel tree") else { panic!("Display panel child is a Tree") };
    tree.sections.first_mut().expect("Display kind section").default_open = Some(true);

    let parallel_id = "framework.display.windows.main.projection.parallel";
    let child_id = "framework.display.windows.main.projection.parallel.orthographic";
    let chevron_id = format!("tree.chevron.{FRAMEWORK_DISPLAY_WINDOWS_TAB_ID}/{parallel_id}");
    let child_label_id = format!("tree.label.{FRAMEWORK_DISPLAY_WINDOWS_TAB_ID}/{child_id}");
    let mut engine = ui_wgpu::wgpu::Ui::new(semio_framework_ui_locale::Locale::En);
    let records = panel_ui_records(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &body).expect("Display's authored tree projects into retained records");
    let mut document = ui_wgpu::wgpu::tree::UiDocumentTree::new(ui_contract::UiDocumentLeaseHeader {
        generation: 1,
        surface: SurfaceId::try_from(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID).unwrap(),
        revision: ui_contract::UiRevision(1),
        root: records.first().expect("Display document root").id,
        layout_epoch: 0,
        node_count: records.len(),
    })
    .expect("Display retained document header");
    for record in records {
        document.try_upsert_record(record).expect("Display retained record admits");
    }
    assert!(engine.publish_document(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, document));
    engine.set_window_flow(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, ui_contract::UiFlow::for_anchor(ui_contract::Anchor::Bottom));
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    settle_display_tree(&mut engine, &mut atlas, 1);

    let expected: Vec<String> = display_order_fixture()["displayResolvedOrder"]["topToBottomIds"]
        .as_array()
        .expect("neutral Display order")
        .iter()
        .filter_map(|id| id.as_str())
        .filter(|id| id.ends_with(".kind") || id.ends_with(".projection.parallel") || id.ends_with(".projection.perspective"))
        .map(|id| id.replace("puzzle3dMain", "main"))
        .collect();
    let mut painted: Vec<(f32, String)> = expected
        .iter()
        .map(|id| {
            let control_id = format!("tree.label.{FRAMEWORK_DISPLAY_WINDOWS_TAB_ID}/{id}");
            let rect = engine
                .window_hit_targets(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID)
                .iter()
                .find(|hit| hit.control_id == control_id)
                .unwrap_or_else(|| panic!("Display row '{id}' publishes its painted label hit"))
                .rect;
            (rect.y, id.clone())
        })
        .collect();
    painted.sort_by(|left, right| left.0.total_cmp(&right.0));
    assert_eq!(painted.into_iter().map(|(_, id)| id).collect::<Vec<_>>(), expected, "the retained Up-flow Tree's accepted physical row rectangles match React's mounted top-to-bottom order");

    let chevron = engine
        .window_hit_targets(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID)
        .iter()
        .find(|hit| hit.control_id == chevron_id)
        .unwrap_or_else(|| panic!("a painted childful Display row publishes its own fold gutter; hits={:?}", engine.window_hit_targets(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID).iter().map(|hit| hit.control_id.as_str()).collect::<Vec<_>>()))
        .rect;
    assert!(!engine.window_hit_targets(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID).iter().any(|hit| hit.control_id == child_label_id), "the closed Parallel branch publishes no child hit");

    press_display_tree(&mut engine, chevron);
    settle_display_tree(&mut engine, &mut atlas, 2);
    assert!(engine.window_hit_targets(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID).iter().any(|hit| hit.control_id == child_label_id), "the gutter opens Parallel and publishes Orthographic");

    let chevron = engine.window_hit_targets(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID).iter().find(|hit| hit.control_id == chevron_id).expect("the open branch keeps its gutter").rect;
    press_display_tree(&mut engine, chevron);
    settle_display_tree(&mut engine, &mut atlas, 3);
    assert!(!engine.window_hit_targets(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID).iter().any(|hit| hit.control_id == child_label_id), "closing Parallel retires Orthographic from the published hit generation");
}

#[test]
fn the_expanded_display_taxonomy_paints_in_the_mounted_react_order() {
    let fixture = display_order_fixture();
    let expected: Vec<&str> = fixture["displayResolvedOrder"]["topToBottomIds"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect();
    assert!(expected.iter().all(|id| semio_framework::is_element_id(id)), "🆔️ the shared order names element ids only: {expected:?}");
    let mut shell = display_shell();
    let kind = shell.session.as_mut().unwrap().app.window_kinds.first_mut();
    kind.id = fixture["displayResolvedOrder"]["windowKindId"].as_str().unwrap().into();
    kind.surface_kind = ui_wgpu::wgpu::SurfaceKind::World3d;
    let body = shell.build_display_windows_ui();
    let mut records = panel_ui_records(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &body).expect("Display records");
    for record in &mut records {
        match &mut record.component {
            ui_contract::Component::TreeItem(props) => props.default_open = Some(true),
            ui_contract::Component::TreeSection(props) => props.default_open = Some(true),
            _ => {}
        }
    }
    let mut document = ui_wgpu::wgpu::tree::UiDocumentTree::new(ui_contract::UiDocumentLeaseHeader {
        generation: 1,
        surface: SurfaceId::try_from(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID).unwrap(),
        revision: ui_contract::UiRevision(1),
        root: records[0].id,
        layout_epoch: 0,
        node_count: records.len(),
    }).expect("Display header");
    for record in records {
        document.try_upsert_record(record).expect("Display record admits");
    }
    let mut engine = ui_wgpu::wgpu::Ui::new(semio_framework_ui_locale::Locale::En);
    assert!(engine.publish_document(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, document));
    engine.set_window_flow(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, ui_contract::UiFlow::for_anchor(ui_contract::Anchor::Bottom));
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    settle_tree_surface_at_height(&mut engine, &mut atlas, FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, 1, 720.0);
    let mut painted: Vec<(f32, &str)> = expected.iter().map(|id| {
        let control_id = format!("tree.label.{FRAMEWORK_DISPLAY_WINDOWS_TAB_ID}/{id}");
        let hit = engine.window_hit_targets(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID).iter().find(|hit| hit.control_id == control_id).unwrap_or_else(|| panic!("missing painted Display row {id}"));
        (hit.rect.y, *id)
    }).collect();
    painted.sort_by(|left, right| left.0.total_cmp(&right.0));
    assert_eq!(painted.into_iter().map(|(_, id)| id).collect::<Vec<_>>(), expected);
}

/// 🖥️ **The Layout census.** React's save section (name box + Save, disabled on a blank name), the
/// list section, `groupPath` folding, and a user layout's own `framework.display.delete.<id>` action.
#[test]
fn the_display_layout_leaf_publishes_the_save_box_and_both_layout_rosters() {
    let mut shell = display_shell();
    shell.layout_save_label.clear();
    shell.user_layouts = vec![ui_wgpu::wgpu::NamedLayout { id: "user-7".into(), label: "Mine".into(), icon_id: None, layout: shell.dock.to_window_layout(), origin: "user".into(), group_path: None }];
    if let Some(session) = shell.session.as_mut() {
        session.app.named_layouts =
            vec![ui_wgpu::wgpu::NamedLayout { id: "review".into(), label: "Review".into(), icon_id: None, layout: ui_wgpu::wgpu::create_stack_layout(&["main".to_string()], None), origin: "builtin".into(), group_path: Some(vec!["Work".into()]) }];
    }
    let node = shell.build_display_layout_ui();
    let keys = published_keys(&shell, FRAMEWORK_DISPLAY_LAYOUT_TAB_ID, &node);
    for react_id in [
        "framework.display.layout.save",
        "framework.display.layout.save.label",
        "framework.display.saveLabel",
        "framework.display.layout.save.action",
        "framework.display.save",
        "framework.display.layout.list",
        "framework.display.layout.group.Work",
        "framework.display.layout.review",
        "framework.display.layout.group.saved",
        "framework.display.layout.user-7",
        "framework.display.delete.user-7",
    ] {
        assert!(keys.iter().any(|key| key.ends_with(react_id)), "🖥️ '{react_id}' is React's own id, got {keys:?}");
    }
    let records = panel_ui_records(FRAMEWORK_DISPLAY_LAYOUT_TAB_ID, &node).expect("the layout body projects");
    let save = records.iter().find(|record| record.key.as_str().ends_with("framework.display.save")).expect("the Save button");
    assert!(save.disabled, "🖥️ React disables Save on a blank name (`!host.layoutSaveLabel.trim()`)");
    shell.layout_save_label = "Mine".into();
    let records = panel_ui_records(FRAMEWORK_DISPLAY_LAYOUT_TAB_ID, &shell.build_display_layout_ui()).expect("the layout body projects");
    let save = records.iter().find(|record| record.key.as_str().ends_with("framework.display.save")).expect("the Save button");
    assert!(!save.disabled, "🖥️ a typed name enables Save");
}

/// ⚖️ **The Conflicts projection.** With no open conflict the leaf still paints React's own
/// unavailable row; with one it paints the real row plus React's `.accept`/`.discard` pair — the gap
/// the leaf carried since W2c ("never paints a conflict either").
#[test]
fn the_conflicts_leaf_paints_the_live_roster_and_reacts_resolution_pair() {
    let mut shell = display_shell();
    let keys = published_keys(&shell, FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, &shell.build_settings_conflicts_ui());
    assert!(keys.iter().any(|key| key.ends_with("framework.settings.conflicts.empty.row")), "⚖️ an empty roster is React's own unavailable row");

    shell.open_conflicts = vec![ShellConflictRow { id: "conflict-1".into(), quarantined: true, code: "mergeQuarantined".into(), message: "two actors edited the same joint".into(), preview_after: r#"{\n  "incoming": true\n}"#.into() }];
    shell.selected_conflict_id = Some("conflict-1".into());
    let node = shell.build_settings_conflicts_ui();
    let keys = published_keys(&shell, FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, &node);
    for react_id in ["framework.settings.conflicts.table", "framework.settings.conflicts.conflict-1", "framework.settings.conflicts.conflict-1.accept", "framework.settings.conflicts.conflict-1.discard"] {
        assert!(keys.iter().any(|key| key.ends_with(react_id)), "⚖️ '{react_id}' is React's own id, got {keys:?}");
    }
    assert!(!keys.iter().any(|key| key.ends_with("framework.settings.conflicts.empty.row")), "⚖️ an open conflict replaces the unavailable row");
    let records = panel_ui_records(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, &node).expect("the conflicts body projects");
    let row = records.iter().find(|record| record.key.as_str().ends_with("framework.settings.conflicts.conflict-1")).expect("the conflict row");
    let ui_contract::Component::TreeItem(props) = &row.component else { panic!("⚖️ a conflict is a tree row") };
    let label = props.label.0.as_str();
    assert!(label.contains("mergeQuarantined") && label.contains("two actors edited the same joint"), "⚖️ the row prints the worst message's code and text, got '{label}'");
}

/// 🛠️ **The Tool measures pin.** A tool leaf's body is its own published measures under the measures'
/// own ids — React's `windowMeasuresToTreeItems(toolMeasuresByToolId[id])`, which this renderer could
/// not reach at all before the program bridge grew its `tool_measures_section` reader.
#[test]
fn a_tool_leaf_publishes_its_own_measures_under_their_own_ids() {
    let mut shell = display_shell();
    let tab_id = format!("{FRAMEWORK_TOOL_PANEL_TAB_PREFIX}fill");
    let keys = published_keys(&shell, &tab_id, &shell.build_tool_panel_ui("fill"));
    assert!(keys.iter().any(|key| key.ends_with("tool.fill.options")), "🛠️ the headerless options section is React's own `tool.<id>.options`");
    assert_eq!(keys.iter().filter(|key| key.contains("/tool.fill.")).count(), 2, "🛠️ the stable panel root and empty options section are the only tool-owned records without measures");

    shell.tool_measures.insert(
        "fill".into(),
        vec![WindowMeasure::Select {
            id: "fill.material".into(),
            label: Some("Material".into()),
            value: "oak".into(),
            items: vec![ui_wgpu::wgpu::MeasureSelectItem { id: "oak".into(), value: "oak".into(), label: "Oak".into() }],
            on_change: ActionDescriptor { controller_id: "test".into(), action: "setFillMaterial".into(), args: None },
        }],
    );
    let node = shell.build_tool_panel_ui("fill");
    let keys = published_keys(&shell, &tab_id, &node);
    assert_eq!(keys.iter().filter(|key| key.ends_with("fill.material")).count(), 2, "🛠️ the measure publishes its row AND its control, both under the measure's own id, got {keys:?}");
    let records = panel_ui_records(&tab_id, &node).expect("the tool body projects");
    let control = records.iter().find(|record| matches!(record.component, ui_contract::Component::Select(_))).expect("the measure's own editor");
    assert!(control.bindings.iter().any(|binding| format!("{:?}", binding.action).contains("setFillMaterial")), "🛠️ the control dispatches the measure's OWN action, not a shell verb");
}

/// 🛍️ One Marketplace row's retained record and its TreeItem props, addressed by React's own row id.
fn marketplace_row<'a>(records: &'a [ui_contract::UiNodeRecord], row_id: &str) -> (&'a ui_contract::UiNodeRecord, &'a ui_contract::TreeItemProps) {
    let key = format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/{row_id}");
    let record = records.iter().find(|record| record.key.as_str() == key).unwrap_or_else(|| panic!("🛍️ the row '{row_id}' is materialised"));
    let ui_contract::Component::TreeItem(props) = &record.component else { panic!("🛍️ '{row_id}' is a Tree row") };
    (record, props)
}

/// 🛍️ The verbs one row's actions name, with their disabled flags, in paint order.
fn marketplace_verbs(props: &ui_contract::TreeItemProps) -> Vec<(String, bool)> {
    props.row_actions.iter().map(|action| (action.verb.as_str().to_string(), action.disabled)).collect()
}

/// 🪟️ Points the Marketplace's roster window at `plugin_id`'s row, as the tree window observer would after a scroll.
fn marketplace_window_at(shell: &mut ShellState, plugin_id: &str) {
    let index = shell.marketplace_roster().iter().position(|id| id == plugin_id).unwrap_or_else(|| panic!("🛍️ '{plugin_id}' is on the roster")) as u32;
    let key = format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/{MARKETPLACE_LOCAL_SECTION_ID}");
    shell.tree_windows.report(FRAMEWORK_MARKETPLACE_TAB_ID, &[ui_wgpu::wgpu::tree_window::TreeWindowRequest { key, offset: index, rows: 1 }], 20, 0);
}

/// 🛍️ **The Marketplace action lane.** A resident plugin offers React's reload/uninstall pair, a plugin only the activation
/// catalogue knows offers Install — each a ROW ACTION on the row's ONE target (`framework`, `{ pluginId }`) — and the session's own
/// program may never be uninstalled (React's `canUninstall`): its Uninstall paints and announces disabled.
#[test]
fn the_marketplace_leaf_offers_reacts_install_reload_and_uninstall_verbs() {
    let mut shell = display_shell();
    marketplace_window_at(&mut shell, "space");
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("the marketplace body projects");
    let (_, space) = marketplace_row(&records, "framework.marketplace.plugin.space");
    assert_eq!(marketplace_verbs(space), vec![("reloadPlugin".to_string(), false), ("uninstallPlugin".to_string(), true)], "🛍️ the session's OWN program is never uninstallable — React's `canUninstall`");
    let target = serde_json::to_value(space.target.as_ref().expect("🎯️ the row's one target")).expect("target json");
    assert_eq!((target["scope"].as_str(), target["args"]["pluginId"].as_str()), (Some("framework"), Some("space")), "🎯️ every verb fires on the row's one target: {target}");
    assert!(target.get("activation").is_none(), "🎯️ an action-only row has no activation verb");

    let installable = crate::program_bridge::PLUGIN_ARTIFACT_KIND_ACTIVATIONS.iter().map(|(_, plugin_id)| *plugin_id).find(|plugin_id| *plugin_id != "space").expect("the catalogue claims a plugin this shell does not hold");
    marketplace_window_at(&mut shell, installable);
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("the marketplace body projects");
    let (_, row) = marketplace_row(&records, &format!("framework.marketplace.plugin.{installable}"));
    assert_eq!(marketplace_verbs(row), vec![("installPlugin".to_string(), false)], "🛍️ a non-resident plugin offers React's Install verb");

    assert!(!shell.uninstall_plugin("space"), "🛍️ uninstalling the running program is refused");
    assert_eq!(shell.plugins.len(), 1, "🛍️ and the roster is untouched");
}

/// 🚫️ **The disabled row action law** (U6 × WG11, W1E-1): the retained accessibility projection announces the session program's
/// Uninstall as the virtual button `<row>::row-action::1` with `disabled == true` — present, never actionable, still focusable
/// and described by its localized reason (en/de), while its enabled sibling carries none.
#[test]
fn the_marketplace_announces_a_disabled_uninstall_as_a_disabled_row_action_button() {
    for (locale, reason) in [("en", "The open document uses this plugin."), ("de", "Das geöffnete Dokument verwendet dieses Plugin.")] {
        let mut shell = display_shell();
        shell.locale_id = locale.into();
        let nodes = marketplace_accessibility_nodes(&mut shell);
        let uninstall = nodes.iter().find(|node| node.key == format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/framework.marketplace.plugin.space::row-action::1")).expect("the Uninstall row action");
        assert!(uninstall.disabled && uninstall.focusable && uninstall.description.as_deref() == Some(reason), "💬️ {locale}: a disabled row action names its reason and stays reachable: {uninstall:?}");
        let reload = nodes.iter().find(|node| node.key == format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/framework.marketplace.plugin.space::row-action::0")).expect("the Reload row action");
        assert_eq!(reload.description, None, "💬️ {locale}: an enabled row action carries no reason");
    }
    let mut shell = display_shell();
    let nodes = marketplace_accessibility_nodes(&mut shell);
    let uninstall_key = format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/framework.marketplace.plugin.space::row-action::1");
    let uninstall = nodes.iter().find(|node| node.key == uninstall_key).unwrap_or_else(|| panic!("🚫️ the Uninstall row action is announced: {:?}", nodes.iter().map(|node| node.key.as_str()).collect::<Vec<_>>()));
    assert_eq!(uninstall.role, "button");
    assert!(uninstall.disabled && !uninstall.actionable, "🚫️ a disabled row action announces disabled and is never actionable");
    let reload = nodes.iter().find(|node| node.key == format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/framework.marketplace.plugin.space::row-action::0")).expect("the Reload row action");
    assert!(!reload.disabled && reload.actionable, "🔁️ the enabled sibling stays actionable");
}

/// ♿️ The mounted Marketplace's retained accessibility projection, with the session program's row in view.
fn marketplace_accessibility_nodes(shell: &mut ShellState) -> Vec<ui_contract::AccessibilityProjectionNode> {
    marketplace_window_at(shell, "space");
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("the marketplace body projects");
    let mut document = ui_wgpu::wgpu::tree::UiDocumentTree::new(ui_contract::UiDocumentLeaseHeader {
        generation: 1,
        surface: SurfaceId::try_from(FRAMEWORK_MARKETPLACE_TAB_ID).unwrap(),
        revision: ui_contract::UiRevision(1),
        root: records.first().expect("marketplace document root").id,
        layout_epoch: 0,
        node_count: records.len(),
    })
    .expect("marketplace document header");
    for record in records {
        document.try_upsert_record(record).expect("marketplace record admits");
    }
    let mut engine = ui_wgpu::wgpu::Ui::new(semio_framework_ui_locale::Locale::En);
    assert!(engine.publish_document(FRAMEWORK_MARKETPLACE_TAB_ID, document));
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    settle_tree_surface(&mut engine, &mut atlas, FRAMEWORK_MARKETPLACE_TAB_ID, 1);
    ui_wgpu::wgpu::accessibility::accessibility_projection(engine.tree(FRAMEWORK_MARKETPLACE_TAB_ID).expect("the mounted Marketplace"))
}

/// 🪟️ **The windowed roster law** (P5 × U6): the roster is ONE windowed Tree section — whatever its length it projects inside
/// the retained node ceiling, it materialises no more than the body's window budget, and the window the tree window observer
/// reports is exactly the slice the leaf republishes (here: the roster's LAST row).
#[test]
fn the_marketplace_streams_its_roster_as_one_windowed_section() {
    let mut shell = display_shell();
    let section_key = format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/{MARKETPLACE_LOCAL_SECTION_ID}");
    let section = |records: &[ui_contract::UiNodeRecord]| -> (ui_contract::TreeWindow, usize) {
        let record = records.iter().find(|record| record.key.as_str() == section_key).expect("🪟️ the roster section");
        let ui_contract::Component::TreeSection(props) = &record.component else { panic!("🪟️ a Tree section") };
        (*props.window.as_ref().expect("🪟️ the roster section is windowed"), record.children.len())
    };
    let roster = shell.marketplace_roster();
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("🪟️ the whole roster projects inside the retained node ceiling");
    let (window, rows) = section(&records);
    assert_eq!((window.total as usize, window.offset), (roster.len(), 0), "🪟️ the window spans the whole roster and opens at its first row");
    assert!(rows <= MARKETPLACE_WINDOW_NODE_BUDGET && rows <= roster.len(), "🪟️ the first paint materialises inside the window budget: {rows}");

    let last = roster.last().expect("the fixture roster holds plugins").clone();
    marketplace_window_at(&mut shell, &last);
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("🪟️ the scrolled roster projects");
    let (window, rows) = section(&records);
    assert_eq!((window.offset as usize, rows), (roster.len() - 1, 1), "🪟️ the reported window is the slice the leaf republishes");
    marketplace_row(&records, &format!("framework.marketplace.plugin.{last}"));
}

#[test]
fn the_marketplace_projects_the_store_record_under_its_declared_host_with_enablement_and_uninstall() {
    let mut shell = display_shell();
    let record: ShellExtensionStoreRecord = serde_json::from_str(include_str!("../../../../../../🔌️plugin/🏪️store/📥️installation/🧫️fixtures/🔣️.json")).expect("the neutral Store record fixture");
    let host = shell.plugins.first().expect("fixture host").plugin_id.clone();
    shell.project_extension_record(ShellExtensionStoreRecord { extends_host: host.clone(), ..record.clone() }).expect("record admits");
    marketplace_window_at(&mut shell, &host);
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("the marketplace body projects");
    let keys: Vec<&str> = records.iter().map(|record| record.key.as_str()).collect();
    for id in ["framework.marketplace.extensions.install", "framework.marketplace.extensions.install.url", "framework.marketplace.extensions.install.file"] {
        assert!(keys.iter().any(|key| key.ends_with(id)), "the Store projection keeps React's Marketplace id {id}, got {keys:?}");
    }
    let (_, extension) = marketplace_row(&records, &format!("framework.marketplace.plugin.{host}.extension.{}", record.extension_id));
    assert_eq!(marketplace_verbs(extension), vec![("setExtensionEnabled".to_string(), false), ("uninstallExtension".to_string(), false)], "an enabled available extension offers disable and uninstall");
    let target = serde_json::to_value(extension.target.as_ref().expect("🎯️ the extension row's one target")).expect("target json");
    assert_eq!((target["args"]["extensionId"].as_str(), target["args"]["enabled"].as_bool()), (Some(record.extension_id.as_str()), Some(false)), "🎯️ the toggle's next state lives in the target: {target}");
    assert_eq!(shell.extensions.len(), 1);
    assert!(shell.extensions[0].enabled);
    assert_eq!(shell.extensions[0].load_status, ShellExtensionLoadStatus::Available);
}

#[test]
fn the_marketplace_keeps_orphaned_and_failed_store_packages_visible_without_executable_controls() {
    let mut shell = display_shell();
    let mut record: ShellExtensionStoreRecord = serde_json::from_str(include_str!("../../../../../../🔌️plugin/🏪️store/📥️installation/🧫️fixtures/🔣️.json")).expect("the neutral Store record fixture");
    record.extends_host = "missing.host".into();
    shell.project_extension_record(record.clone()).expect("record admits");
    shell.extensions[0].load_status = ShellExtensionLoadStatus::Failed("fixture load fault".into());
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("the orphaned package projects");
    let keys: Vec<&str> = records.iter().map(|record| record.key.as_str()).collect();
    assert!(keys.iter().any(|key| key.ends_with("framework.marketplace.missing-host.missing.host")), "an absent declared host has its own visible React-parity section, got {keys:?}");
    let (_, extension) = marketplace_row(&records, &format!("framework.marketplace.plugin.missing.host.extension.{}", record.extension_id));
    assert_eq!(marketplace_verbs(extension), vec![("setExtensionEnabled".to_string(), true), ("uninstallExtension".to_string(), false)], "a failed package cannot be enabled until it admits a program, and remains uninstallable");
}

#[test]
fn conflict_resolution_buttons_are_inline_controls_before_row_selection() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🎛️inline-tree-controls/🔣️.json")).unwrap();
    let mut shell = display_shell();
    shell.locale_id = "en".into();
    let source = &fixture["conflict"];
    shell.open_conflicts = vec![ShellConflictRow {
        id: source["id"].as_str().unwrap().into(),
        quarantined: source["quarantined"].as_bool().unwrap(),
        code: source["code"].as_str().unwrap().into(),
        message: source["message"].as_str().unwrap().into(),
        preview_after: r#"{
  "incoming": true
}"#
        .into(),
    }];
    shell.selected_conflict_id = None;
    let records = panel_ui_records(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, &shell.build_settings_conflicts_ui()).unwrap();
    let row_id = fixture["rowId"].as_str().unwrap();
    assert_eq!(records.iter().filter(|record| matches!(record.component, ui_contract::Component::TreeItem(_))).count(), fixture["expected"]["rowCount"].as_u64().unwrap() as usize);
    let row = records.iter().find(|record| record.key.as_str().ends_with(row_id)).unwrap();
    let toolbar = row
        .children
        .iter()
        .filter_map(|id| records.iter().find(|record| record.id == *id))
        .find(|record| matches!(&record.component, ui_contract::Component::Container(props) if props.role == ui_contract::ContainerRole::Toolbar))
        .expect("the conflict row owns its inline toolbar before selection");
    let ui_contract::Component::TreeItem(props) = &row.component else { panic!("the conflict is one TreeItem row") };
    assert_eq!(props.inline_toolbar, Some(toolbar.id), "the row explicitly identifies its semantic inline toolbar");
    assert!(matches!(&toolbar.layout, ui_contract::LayoutSpec::Stack(layout) if layout.axis == ui_contract::Axis::Horizontal));
    for expected in fixture["controls"].as_array().unwrap() {
        let key = format!("{row_id}.{}", expected["suffix"].as_str().unwrap());
        let button = records.iter().find(|record| record.key.as_str().ends_with(&key)).unwrap();
        let ui_contract::Component::Button(props) = &button.component else { panic!("the resolution affordance is an actual Button") };
        assert_eq!(props.label.0.as_str(), expected["label"].as_str().unwrap());
        assert!(!button.disabled);
        assert!(toolbar.children.iter().any(|id| *id == button.id));
        assert!(button.bindings.iter().any(|binding| binding.trigger == ui_contract::Trigger::Activate && binding.action.name.as_str() == "resolveConflict"));
    }
}

#[test]
fn selected_conflict_owns_a_real_diff_view_detail_in_the_accepted_frame() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🆚️diff-view-produced-surface/🔣️.json")).expect("DiffView producer fixture");
    let mut shell = display_shell();
    shell.locale_id = "en".into();
    shell.open_conflicts = fixture["conflicts"]
        .as_array()
        .expect("conflict rows")
        .iter()
        .map(|row| ShellConflictRow {
            id: row["id"].as_str().unwrap().into(),
            quarantined: row["quarantined"].as_bool().unwrap(),
            code: row["code"].as_str().unwrap().into(),
            message: row["message"].as_str().unwrap().into(),
            preview_after: row["after"].as_str().unwrap().into(),
        })
        .collect();
    shell.selected_conflict_id = Some(fixture["selectedConflictId"].as_str().unwrap().into());
    let records = panel_ui_records(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, &shell.build_settings_conflicts_ui()).expect("conflicts project");
    let selected = records.iter().find(|record| record.key.as_str().ends_with(fixture["selectedConflictId"].as_str().unwrap())).expect("selected conflict row");
    let ui_contract::Component::TreeItem(selected_props) = &selected.component else { panic!("selected conflict remains one TreeItem") };
    let detail_id = selected_props.detail.expect("selected row explicitly owns its detail");
    assert!(selected.children.iter().any(|child| *child == detail_id), "detail is a direct semantic child");
    let detail = records.iter().find(|record| record.id == detail_id).expect("related detail record");
    let ui_contract::Component::Surface(surface) = &detail.component else { panic!("detail relation targets a Surface") };
    assert_eq!(surface.kind, ui_contract::SurfaceKind::DiffView);
    let scene: ui_wgpu::wgpu::DiffViewScene = ui_wgpu::wgpu::decode_surface_doc(surface).expect("DiffView scene decodes");
    assert_eq!(scene.after, fixture["conflicts"][0]["after"].as_str().unwrap());
    assert!(records.iter().filter(|record| matches!(&record.component, ui_contract::Component::TreeItem(props) if props.detail.is_some())).count() == 1, "only the selected conflict owns a detail");

    let mut document = ui_wgpu::wgpu::tree::UiDocumentTree::new(ui_contract::UiDocumentLeaseHeader {
        generation: 1,
        surface: SurfaceId::try_from(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID).unwrap(),
        revision: ui_contract::UiRevision(1),
        root: records.first().expect("conflict document root").id,
        layout_epoch: 0,
        node_count: records.len(),
    })
    .expect("conflict document header");
    for record in records {
        document.try_upsert_record(record).expect("conflict record admits");
    }
    let mut engine = ui_wgpu::wgpu::Ui::new(semio_framework_ui_locale::Locale::En);
    assert!(engine.publish_document(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, document));
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    settle_tree_surface(&mut engine, &mut atlas, FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, 1);
    let hit = (0..240).step_by(4).flat_map(|y| (0..300).step_by(4).map(move |x| (x as f32, y as f32))).find_map(|(x, y)| engine.scene_at(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, x, y)).expect("accepted frame exposes the selected DiffView slot");
    assert_eq!(hit.kind, ui_wgpu::wgpu::SurfaceKind::DiffView);
    assert!(hit.rect.w > 0.0 && hit.rect.h > 0.0, "the accepted detail slot has real geometry: {:?}", hit.rect);
}

#[test]
fn conflict_resolution_buttons_share_the_row_and_win_its_pointer_band() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🎛️inline-tree-controls/🔣️.json")).unwrap();
    let mut shell = display_shell();
    shell.locale_id = "en".into();
    let source = &fixture["conflict"];
    shell.open_conflicts = vec![ShellConflictRow {
        id: source["id"].as_str().unwrap().into(),
        quarantined: source["quarantined"].as_bool().unwrap(),
        code: source["code"].as_str().unwrap().into(),
        message: source["message"].as_str().unwrap().into(),
        preview_after: r#"{
  "incoming": true
}"#
        .into(),
    }];
    let records = panel_ui_records(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, &shell.build_settings_conflicts_ui()).expect("conflict controls project");
    let mut document = ui_wgpu::wgpu::tree::UiDocumentTree::new(ui_contract::UiDocumentLeaseHeader {
        generation: 1,
        surface: SurfaceId::try_from(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID).unwrap(),
        revision: ui_contract::UiRevision(1),
        root: records.first().expect("conflict document root").id,
        layout_epoch: 0,
        node_count: records.len(),
    })
    .expect("conflict document header");
    for record in records {
        document.try_upsert_record(record).expect("conflict record admits");
    }
    let mut engine = ui_wgpu::wgpu::Ui::new(semio_framework_ui_locale::Locale::En);
    assert!(engine.publish_document(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, document));
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    settle_tree_surface(&mut engine, &mut atlas, FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, 1);

    let row_id = fixture["rowId"].as_str().unwrap();
    let hits = engine.window_hit_targets(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID);
    let row_index = hits.iter().position(|hit| hit.kind == ui_wgpu::wgpu::HitKind::TreeItem && hit.control_id.ends_with(row_id)).expect("one conflict row hit");
    let row = hits[row_index].rect;
    let mut button_indices = Vec::new();
    for expected in fixture["controls"].as_array().unwrap() {
        let key = format!("{row_id}.{}", expected["suffix"].as_str().unwrap());
        let index = hits.iter().position(|hit| hit.kind == ui_wgpu::wgpu::HitKind::Button && hit.control_id.ends_with(&key)).unwrap_or_else(|| panic!("{key} publishes a Button hit"));
        let button = hits[index].rect;
        assert!(index > row_index, "the descendant Button is registered after the row so reverse hit-testing resolves it first");
        assert!(button.w > 0.0 && button.h > 0.0 && button.x >= row.x && button.y >= row.y && button.x + button.w <= row.x + row.w && button.y + button.h <= row.y + row.h, "{key} stays inside the one row: button={button:?}, row={row:?}");
        button_indices.push(index);
    }
    let first = hits[button_indices[0]].rect;
    let second = hits[button_indices[1]].rect;
    assert!((first.y - second.y).abs() < 0.01 && (first.h - second.h).abs() < 0.01 && first.x + first.w <= second.x + 0.01, "the labeled Buttons form one horizontal toolbar: {first:?}, {second:?}");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    for hit in hits {
        input.register_hit(hit.to_hit_target());
    }
    input.publish_hits();
    for index in button_indices {
        let button = hits[index].rect;
        let resolved = input.hit_at(button.x + button.w * 0.5, button.y + button.h * 0.5).expect("button centre resolves");
        assert_eq!(resolved.kind, ui_wgpu::wgpu::HitKind::Button, "the row selection target does not steal the inline Button");
    }
}

#[test]
fn display_projection_disclosures_reflow_the_actual_hugging_shell_panel() {
    let fixture = display_order_fixture();
    let mut shell = display_shell();
    let kind = shell.session.as_mut().unwrap().app.window_kinds.first_mut();
    kind.id = fixture["displayResolvedOrder"]["windowKindId"].as_str().unwrap().into();
    kind.surface_kind = ui_wgpu::wgpu::SurfaceKind::World3d;
    let anchor = PanelAnchor::BottomLeft;
    shell.screen_w = 1_600.0;
    shell.screen_h = 1_000.0;
    shell.dock_tabs.tabs_mut(anchor).clear();
    shell.dock_tabs.tabs_mut(anchor).push(DockTabNode::branch("framework.category.display", "Display", "layout", 0, vec![DockTabNode::leaf(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, "Windows", "app-window", 0)]));
    shell.anchor_state_mut(anchor).path = vec!["framework.category.display".into(), FRAMEWORK_DISPLAY_WINDOWS_TAB_ID.into()];
    shell.anchor_state_mut(anchor).visible = true;
    let records = panel_ui_records(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &shell.build_display_windows_ui()).unwrap();
    let document = shell.publish_surface_records(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, records).unwrap();
    shell.panel_documents.insert(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID.into(), document);
    let icons = IconAtlas::default();
    let mut atlas = FontAtlas::builtin();
    let mut resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let body = shell.body_rect(&theme);
    let mut paint = |shell: &mut ShellState, input: &mut InputState<ActionDescriptor>| {
        while input.retire_hit_step() {}
        shell.retained_hit_windows_staging.clear();
        let mut draw = DrawList::default();
        draw.set_screen_height(shell.screen_h);
        let mut cursor = ShellChromeChildCursor::default();
        assert!((0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| shell.render_panel_step(&mut cursor, anchor, &mut draw, None, &mut atlas, &icons, input, &theme, body, &mut resources)));
        shell.publish_retained_hit_registry(input);
    };
    assert_eq!(fixture["displayBranchPublication"]["rules"]["huggingPanelReflowsBeforePublication"], true);
    let base = fixture["displayBranchPublication"]["section"]["id"].as_str().unwrap();
    let section_id = format!("section.chevron.{FRAMEWORK_DISPLAY_WINDOWS_TAB_ID}/{base}");
    let parallel_id = format!("tree.chevron.{FRAMEWORK_DISPLAY_WINDOWS_TAB_ID}/{base}.projection.parallel");
    let orthographic_id = format!("tree.label.{FRAMEWORK_DISPLAY_WINDOWS_TAB_ID}/{base}.projection.parallel.orthographic");
    for _ in 0..3 { paint(&mut shell, &mut input); }
    for id in [section_id, parallel_id] {
        let hits: Vec<_> = input.hits().iter().filter_map(|hit| hit.control_id.clone()).collect();
        let hit = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(id.as_str())).unwrap_or_else(|| panic!("Display physical disclosure {id} missing: {hits:?}")).clone();
        let (x, y) = (hit.rect.x + hit.rect.w * 0.5, hit.rect.y + hit.rect.h * 0.5);
        assert_eq!(input.hit_at(x, y).and_then(|hit| hit.control_id.as_deref()), Some(id.as_str()));
        semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).unwrap();
        semio_framework_async::block_on(shell.handle_pointer_button(x, y, false, 0, &mut input, &theme)).unwrap();
        paint(&mut shell, &mut input);
        println!("[DEBUG] actual Display disclosure {id}: intrinsic={:?} accepted={:?}", crate::interpreter::retained_content_height(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID), input.hits().iter().filter_map(|hit| hit.control_id.clone()).collect::<Vec<_>>());
    }
    assert!(input.hits().iter().any(|hit| hit.control_id.as_deref() == Some(orthographic_id.as_str())), "actual hugging Display publishes Orthographic after one click per disclosure");
}
