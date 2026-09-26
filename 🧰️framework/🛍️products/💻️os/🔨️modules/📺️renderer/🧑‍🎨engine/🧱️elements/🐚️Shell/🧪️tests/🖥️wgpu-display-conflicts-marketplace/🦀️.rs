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
        "framework.display.windows.main.projection.parallel.axonometric.axonometric-isometric",
        "framework.display.windows.main.projection.perspective",
    ] {
        assert!(keys.iter().any(|key| key.ends_with(react_id)), "🔀️ '{react_id}' is React's own nested template id, got {keys:?}");
    }
    let kind_row = keys.iter().position(|key| key.ends_with("framework.display.windows.main.kind")).expect("the kind leaf");
    let parallel = keys.iter().position(|key| key.ends_with("framework.display.windows.main.projection.parallel")).expect("the parallel branch");
    assert!(kind_row < parallel, "🔃️ React RENDERS the plain kind leaf first, then Parallel, then Perspective");
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
    engine.set_viewport(surface, 300.0, 240.0);
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
        match engine.frame_step::<DisplayTreeSceneHost>(surface, 300.0, 240.0, atlas, None, None) {
            ui_wgpu::wgpu::UiFrameStep::Pending => {}
            ui_wgpu::wgpu::UiFrameStep::Ready => {
                assert!(engine.seal_presented_input_candidate(witness, &[surface.to_string()]));
                assert!(engine.acknowledge_presented_input(witness));
                return;
            },
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
    let mut engine = ui_wgpu::wgpu::Ui::new();
    let records = panel_ui_records(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &body).expect("Display's authored tree projects into retained records");
    let mut document = ui_wgpu::wgpu::tree::UiDocumentTree::new(ui_contract::UiDocumentLeaseHeader {
        generation: 1,
        surface: SurfaceId::try_from(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID).unwrap(),
        revision: ui_contract::UiRevision(1),
        root: records.first().expect("Display document root").id,
        layout_epoch: 0,
        node_count: records.len(),
    }).expect("Display retained document header");
    for record in records {
        document.try_upsert_record(record).expect("Display retained record admits");
    }
    assert!(engine.publish_document(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, document));
    engine.set_window_flow(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, ui_contract::UiFlow::for_anchor(ui_contract::Anchor::Bottom));
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    settle_display_tree(&mut engine, &mut atlas, 1);

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

/// 🛍️ **The Marketplace action lane.** A resident plugin offers React's reload/uninstall pair, a
/// plugin only the activation catalogue knows offers Install, and the session's own program may never
/// be uninstalled (React's `canUninstall`).
#[test]
fn the_marketplace_leaf_offers_reacts_install_reload_and_uninstall_verbs() {
    let shell = display_shell();
    let node = shell.build_marketplace_ui();
    let keys = published_keys(&shell, FRAMEWORK_MARKETPLACE_TAB_ID, &node);
    for react_id in ["framework.marketplace.source.local", "framework.marketplace.plugin.space", "framework.marketplace.plugin.space.reload", "framework.marketplace.plugin.space.uninstall"] {
        assert!(keys.iter().any(|key| key.ends_with(react_id)), "🛍️ '{react_id}' is React's own id, got {keys:?}");
    }
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &node).expect("the marketplace body projects");
    let uninstall = records.iter().find(|record| record.key.as_str().ends_with("framework.marketplace.plugin.space.uninstall")).expect("the uninstall row");
    assert!(uninstall.disabled, "🛍️ the session's OWN program is never uninstallable — React's `canUninstall`");

    let installable = crate::program_bridge::PLUGIN_ARTIFACT_KIND_ACTIVATIONS.iter().map(|(_, plugin_id)| *plugin_id).find(|plugin_id| *plugin_id != "space").expect("the catalogue claims a plugin this shell does not hold");
    let install_id = format!("framework.marketplace.plugin.{installable}.install");
    assert!(keys.iter().any(|key| key.ends_with(install_id.as_str())), "🛍️ a non-resident plugin offers React's Install verb, got {keys:?}");

    let mut shell = shell;
    assert!(!shell.uninstall_plugin("space"), "🛍️ uninstalling the running program is refused");
    assert_eq!(shell.plugins.len(), 1, "🛍️ and the roster is untouched");
}

#[test]
fn the_marketplace_projects_the_store_record_under_its_declared_host_with_enablement_and_uninstall() {
    let mut shell = display_shell();
    let record: ShellExtensionStoreRecord = serde_json::from_str(include_str!("../../../../../../🔌️plugin/🏪️store/📥️installation/🧫️fixtures/🔣️.json")).expect("the neutral Store record fixture");
    let host = shell.plugins.first().expect("fixture host").plugin_id.clone();
    shell.project_extension_record(ShellExtensionStoreRecord { extends_host: host.clone(), ..record.clone() }).expect("record admits");
    let keys = published_keys(&shell, FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui());
    for id in [
        "framework.marketplace.extensions.install".to_string(),
        "framework.marketplace.extensions.install.url".to_string(),
        "framework.marketplace.extensions.install.file".to_string(),
        format!("framework.marketplace.plugin.{host}.extension.{}", record.extension_id),
        format!("framework.marketplace.extension.{}.enable", record.extension_id),
        format!("framework.marketplace.extension.{}.uninstall", record.extension_id),
    ] {
        assert!(keys.iter().any(|key| key.ends_with(&id)), "the Store projection keeps React's Marketplace id {id}, got {keys:?}");
    }
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
    let node = shell.build_marketplace_ui();
    let keys = published_keys(&shell, FRAMEWORK_MARKETPLACE_TAB_ID, &node);
    assert!(keys.iter().any(|key| key.ends_with("framework.marketplace.missing-host.missing.host")), "an absent declared host has its own visible React-parity section, got {keys:?}");
    assert!(keys.iter().any(|key| key.ends_with(&format!("framework.marketplace.plugin.missing.host.extension.{}", record.extension_id))), "the failed Store package remains visible under its declared host");
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &node).expect("the orphaned package projects");
    let extension_id = record.extension_id;
    let enable = records.iter().find(|record| record.key.as_str().ends_with(&format!("framework.marketplace.extension.{extension_id}.enable"))).expect("the enable control remains visible");
    assert!(enable.disabled, "a failed package cannot be enabled until it admits a program");
    assert!(records.iter().any(|record| record.key.as_str().ends_with(&format!("framework.marketplace.extension.{extension_id}.uninstall")) && !record.disabled), "a failed package remains uninstallable");
}

#[test]
fn conflict_resolution_buttons_are_inline_controls_before_row_selection() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🎛️inline-tree-controls/🔣️.json")).unwrap();
    let mut shell = display_shell();
    shell.locale_id = "en".into();
    let source = &fixture["conflict"];
    shell.open_conflicts = vec![ShellConflictRow {
        id: source["id"].as_str().unwrap().into(),
        quarantined: source["quarantined"].as_bool().unwrap(),
        code: source["code"].as_str().unwrap().into(),
        message: source["message"].as_str().unwrap().into(), preview_after: r#"{
  "incoming": true
}"#.into(),
    }];
    shell.selected_conflict_id = None;
    let records = panel_ui_records(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, &shell.build_settings_conflicts_ui()).unwrap();
    let row_id = fixture["rowId"].as_str().unwrap();
    assert_eq!(records.iter().filter(|record| matches!(record.component, ui_contract::Component::TreeItem(_))).count(), fixture["expected"]["rowCount"].as_u64().unwrap() as usize);
    let row = records.iter().find(|record| record.key.as_str().ends_with(row_id)).unwrap();
    let toolbar = row.children.iter().filter_map(|id| records.iter().find(|record| record.id == *id)).find(|record| matches!(&record.component, ui_contract::Component::Container(props) if props.role == ui_contract::ContainerRole::Toolbar)).expect("the conflict row owns its inline toolbar before selection");
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
    let mut engine = ui_wgpu::wgpu::Ui::new();
    assert!(engine.publish_document(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, document));
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    settle_tree_surface(&mut engine, &mut atlas, FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, 1);
    let hit = (0..240)
        .step_by(4)
        .flat_map(|y| (0..300).step_by(4).map(move |x| (x as f32, y as f32)))
        .find_map(|(x, y)| engine.scene_at(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, x, y))
        .expect("accepted frame exposes the selected DiffView slot");
    assert_eq!(hit.kind, ui_wgpu::wgpu::SurfaceKind::DiffView);
    assert!(hit.rect.w > 0.0 && hit.rect.h > 0.0, "the accepted detail slot has real geometry: {:?}", hit.rect);
}

#[test]
fn conflict_resolution_buttons_share_the_row_and_win_its_pointer_band() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🎛️inline-tree-controls/🔣️.json")).unwrap();
    let mut shell = display_shell();
    shell.locale_id = "en".into();
    let source = &fixture["conflict"];
    shell.open_conflicts = vec![ShellConflictRow {
        id: source["id"].as_str().unwrap().into(),
        quarantined: source["quarantined"].as_bool().unwrap(),
        code: source["code"].as_str().unwrap().into(),
        message: source["message"].as_str().unwrap().into(), preview_after: r#"{
  "incoming": true
}"#.into(),
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
    let mut engine = ui_wgpu::wgpu::Ui::new();
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
