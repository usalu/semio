/// 🛟️ Safe area and incoming layout: the chrome panel safe area, the overlay row beside floating panels and incoming layout identity, maximize and pruning.

/// 🛟️ The chrome-panel SAFE AREA, from the shared `chromePanelSafeArea` rows — the rule that keeps a
/// live surface's overlay row (the World3d compute pill and its `Cancel`) reachable while an anchored chrome panel paints over the same corner. React's `chromePanelSafeArea`
/// answers the identical rows; the defect it exists for is `📓️react-oracle-hardening-2026-09-14.md` §4.3,
/// where the `top-right` Tool runs panel at (1137, 3) 300×120 swallowed every press meant for
/// `Frame visible` and the preview `Cancel`.
#[test]
fn a_chrome_panel_reserves_a_safe_area_for_the_surface_overlay_row() {
    let fixture: Value = serde_json::from_str(CHROME_PANEL_SAFE_AREA_FIXTURE).expect("🛟️ the General UI safe-area fixture parses");
    let rows = fixture.as_array().expect("🛟️ chromePanelSafeArea rows");
    assert!(!rows.is_empty(), "🛟️ the safe-area corpus is not empty");
    let read_rect = |value: &Value| {
        let box_ = value.as_array().expect("🛟️ a box is [x, y, w, h]");
        Rect::new(box_[0].as_f64().unwrap() as f32, box_[1].as_f64().unwrap() as f32, box_[2].as_f64().unwrap() as f32, box_[3].as_f64().unwrap() as f32)
    };
    for row in rows {
        let id = row["id"].as_str().expect("🛟️ every row names itself");
        let affordance = read_rect(&row["affordance"]);
        let host = read_rect(&row["host"]);
        let panels: Vec<Rect> = row["panels"].as_array().expect("🛟️ panels").iter().map(read_rect).collect();
        let anchor = PanelAnchor::ALL.iter().copied().find(|candidate| candidate.as_str() == row["anchor"].as_str().unwrap()).expect("🛟️ a known anchor");
        let yield_axis = match row["yield"].as_str().unwrap() {
            "inline" => SafeAreaYield::Inline,
            "block" => SafeAreaYield::Block,
            _ => SafeAreaYield::Either,
        };
        let gap = row["gap"].as_f64().unwrap() as f32;
        let reversed: Vec<Rect> = panels.iter().rev().copied().collect();
        for order in [&panels, &reversed] {
            let safe_area = chrome_panel_safe_area(affordance, host, anchor, order, yield_axis, gap);
            assert_eq!(safe_area.inline, row["expected"]["inline"].as_f64().unwrap() as f32, "🛟️ {id}: inline reserve");
            assert_eq!(safe_area.block, row["expected"]["block"].as_f64().unwrap() as f32, "🛟️ {id}: block reserve");
            if safe_area.inline == 0.0 && safe_area.block == 0.0 { continue; }
            let cleared = Rect::new(affordance.x + if anchor.horizontal() == "right" { -safe_area.inline } else { safe_area.inline }, affordance.y + if anchor.vertical() == "bottom" { -safe_area.block } else { safe_area.block }, affordance.w, affordance.h);
            assert!(cleared.x >= host.x && cleared.y >= host.y && cleared.x + cleared.w <= host.x + host.w && cleared.y + cleared.h <= host.y + host.h, "🛟️ {id}: placement stays entirely inside the actual host");
            for panel in &panels {
                let expanded = if safe_area.inline > 0.0 { Rect::new(panel.x - gap, panel.y, panel.w + 2.0 * gap, panel.h) } else { Rect::new(panel.x, panel.y - gap, panel.w, panel.h + 2.0 * gap) };
                assert!(!(cleared.x < expanded.x + expanded.w && cleared.x + cleared.w > expanded.x && cleared.y < expanded.y + expanded.h && cleared.y + cleared.h > expanded.y), "🛟️ {id}: placement preserves the axis gap against every panel");
            }
        }
    }
}

/// 🛟️ The same rule where the shell actually places chrome: an open LEFT floating panel covers a dock
/// window's top-left corner, which is exactly where `surface_overlay_controls_for` anchors the World3d
/// `Cancel` — so the whole overlay row moves right past the panel instead
/// of painting under it, and moves back the moment the panel closes.
#[test]
fn the_overlay_row_steps_clear_of_an_open_floating_panel() {
    let theme = semio_framework_os_renderer_wgpu_host_base::resolve_theme("light");
    let computing = "{\"computing\":true,\"phase\":\"meshingFaces\",\"cancellable\":true,\"cancelAction\":\"cancelPreviewEval\",\"progress\":{\"unitsDone\":3,\"unitsTotal\":8}}";
    let bounds = Rect::new(4.0, 44.0, 952.0, 592.0);
    let worlds = [("window:procedural-preview", bounds, Some(computing))];
    let graphs = [("window:procedural-flow", bounds)];
    let panel = Rect::new(4.0, 44.0, 300.0, 592.0);

    let flush_pills = surface_status_pills_for(&worlds, &[], &theme, false);
    let flush_controls = surface_overlay_controls_for(&graphs, &worlds, &[], &theme, false);
    assert_eq!(flush_pills[0].1.x, bounds.x + theme.gap_standard, "🛟️ with no panel open the row stays flush in its own corner");
    assert_eq!(
        flush_controls[0].1[0],
        flush_pills[0].1.x + flush_pills[0].1.w + theme.gap_standard,
        "🛟️ the cancel FOLLOWS the pill in the same row — React paints the cancel button inside the status container, after the phase and progress spans (`🌐️World3dHost/🟦️.tsx:4001,4018`, one `flex items-center gap-single` row)"
    );

    let reserved_pills = surface_status_pills_for(&worlds, &[panel], &theme, false);
    let reserved_controls = surface_overlay_controls_for(&graphs, &worlds, &[panel], &theme, false);
    let pill_rect = reserved_pills[0].1;
    assert!(pill_rect.x >= panel.x + panel.w, "🛟️ the status pill starts past the open panel's right edge");
    assert_eq!(pill_rect.y, flush_pills[0].1.y, "🛟️ a full-height column is cleared inline, never by dropping the row");
    assert!(pill_rect.x + pill_rect.w <= bounds.x + bounds.w, "🛟️ the reserved pill still paints inside its own surface");
    for (control, anchor) in &reserved_controls {
        assert!(anchor[0] >= panel.x + panel.w, "🛟️ {} starts past the open panel's right edge", control.control_id);
    }
    let cancel = reserved_controls.iter().find(|(control, _)| control.control_id.starts_with("shell.world3d.cancel")).expect("🛟️ the cancel control is offered");
    assert!(cancel.1[0] >= pill_rect.x + pill_rect.w, "🛟️ the cancel still follows the pill on the reserved row");
}

#[test]
fn incoming_layout_identity_resets_maximize_and_identical_sync_preserves_it() {
    let law: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️maximize-resync/🔣️.json")).unwrap();
    let layout = |weights: &Value, change: Option<&Value>| {
        let children = law["windows"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(index, id)| {
                let id = id.as_str().unwrap().to_string();
                let mut windows = vec![semio_framework_os_renderer_wgpu_dock::DockStackTab::new(id.clone())];
                if id == "b" {
                    windows[0].template_id = change.and_then(|row| row["templateId"].as_str()).map(str::to_string);
                    if let Some(extra) = change.and_then(|row| row["extraId"].as_str()) {
                        windows.push(semio_framework_os_renderer_wgpu_dock::DockStackTab::instance(extra, "b", WindowStackCorner::TopLeft));
                    }
                }
                let stack = semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows, active: id };
                (stack, weights[index].as_f64().unwrap() as f32)
            })
            .collect();
        WindowLayout { root: semio_framework_os_renderer_wgpu_dock::dock_node_to_layout_root(&semio_framework_os_renderer_wgpu_dock::DockNode::Row(children)) }
    };
    for case in law["cases"].as_array().unwrap() {
        let mut shell = host_test_shell();
        let prototype = shell.docs.session.as_ref().unwrap().app.window_kinds.first().clone();
        let kinds = |ids: &Value, label: &str| {
            WindowKinds::try_from(
                ids.as_array()
                    .unwrap()
                    .iter()
                    .filter(|id| law["windows"].as_array().unwrap().contains(id))
                    .map(|id| {
                        let mut kind = prototype.clone();
                        kind.id = id.as_str().unwrap().into();
                        kind.label = if id == "a" { LocalizedLabel::native(label, label) } else { LocalizedLabel::native("B", "B") };
                        kind
                    })
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        };
        shell.docs.session.as_mut().unwrap().app.window_kinds = kinds(&law["windows"], "A");
        shell.docs.session.as_mut().unwrap().app.default_layout = Some(layout(&law["initialWeights"], None));
        shell.dock.layout_override = (case["source"] == "override").then(|| layout(&law["initialWeights"], None));
        shell.windows.active_window_id = Some("b".into());
        SessionCtx::of(&mut shell).sync_dock();
        let before = shell.dock.layout_override.clone();
        assert!(shell.dock.focus_active_window(&shell.windows));
        assert_eq!(shell.dock.layout_override, before, "{}: maximize is ephemeral", case["id"]);
        assert_eq!(shell.dock.dock.maximized_stack, Some(vec![law["maximized"].as_u64().unwrap() as usize]));
        if case["source"] == "override" {
            shell.dock.layout_override = Some(layout(&case["weights"], Some(case)));
        } else {
            shell.docs.session.as_mut().unwrap().app.default_layout = Some(layout(&case["weights"], Some(case)));
        }
        shell.docs.session.as_mut().unwrap().app.window_kinds = kinds(&case["windows"], case["layoutLabel"].as_str().unwrap());
        SessionCtx::of(&mut shell).sync_dock();
        let expected = case["maximized"].as_u64().map(|index| vec![index as usize]);
        assert_eq!(shell.dock.dock.maximized_stack, expected, "{}: matches React prop resynchronization", case["id"]);
        assert_eq!(shell.windows.active_window_id.as_deref(), Some("b"), "{}: focus remains on its live instance", case["id"]);
        if let Some(template) = case["templateId"].as_str() {
            assert_eq!(shell.windows.world_projection_template.get("b").map(String::as_str), Some(template));
        }
        if expected.is_none() {
            assert!(shell.dock.focus_active_window(&shell.windows));
        }
        SessionCtx::of(&mut shell).sync_dock();
        assert_eq!(shell.dock.dock.maximized_stack, Some(vec![law["maximized"].as_u64().unwrap() as usize]), "{}: unchanged refresh preserves a later maximize", case["id"]);
    }
    let retitle = &law["localeRetitle"];
    let mut shell = host_test_shell();
    let prototype = shell.docs.session.as_ref().unwrap().app.window_kinds.first().clone();
    shell.docs.session.as_mut().unwrap().app.window_kinds = WindowKinds::try_from(
        law["windows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|id| {
                let mut kind = prototype.clone();
                kind.id = id.as_str().unwrap().into();
                kind.label = if id == "a" { LocalizedLabel::native("A", "A") } else { LocalizedLabel::native("B", "B") };
                kind
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let mut seeded = layout(&law["initialWeights"], Some(retitle));
    if let semio_framework_ui_layout::layout::WindowLayoutRoot::Axis(axis) = &mut seeded.root {
        if let semio_framework_ui_layout::layout::WindowLayoutChild::Stack(stack) = &mut axis.children[1] {
            stack.children[1].title = retitle["bakedTitle"].as_str().map(str::to_string);
        }
    }
    shell.dock.layout_override = Some(seeded);
    shell.windows.active_window_id = Some("b".into());
    shell.prefs.locale_id = "en".into();
    SessionCtx::of(&mut shell).sync_dock();
    assert!(shell.dock.focus_active_window(&shell.windows));
    shell.prefs.locale_id = retitle["locale"].as_str().unwrap().into();
    SessionCtx::of(&mut shell).sync_dock();
    assert_eq!(shell.dock.dock.maximized_stack, Some(vec![retitle["maximized"].as_u64().unwrap() as usize]));
    assert_eq!(shell.dock.dock_chrome_maps(&shell.prefs, &shell.docs, &shell.windows).0[retitle["extraId"].as_str().unwrap()], retitle["retainedTitle"].as_str().unwrap());
    SessionCtx::of(&mut shell).sync_dock();
    assert_eq!(shell.dock.dock.maximized_stack, Some(vec![1]));
}

#[test]
fn incoming_layout_prunes_unknown_windows_against_the_react_instance_roster() {
    fn paths(node: &semio_framework_os_renderer_wgpu_dock::DockNode, path: String, out: &mut Vec<String>) {
        match node {
            semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows, .. } => {
                if !windows.is_empty() {
                    out.push(path);
                }
            }
            semio_framework_os_renderer_wgpu_dock::DockNode::Row(children) | semio_framework_os_renderer_wgpu_dock::DockNode::Column(children) => {
                for (index, (child, _)) in children.iter().enumerate() {
                    paths(child, if path.is_empty() { index.to_string() } else { format!("{path}.{index}") }, out);
                }
            }
        }
    }
    let law: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️maximize-resync/🔣️.json")).unwrap();
    for case in law["ingressCases"].as_array().unwrap() {
        let mut shell = host_test_shell();
        let prototype = shell.docs.session.as_ref().unwrap().app.window_kinds.first().clone();
        shell.docs.session.as_mut().unwrap().app.window_kinds = WindowKinds::try_from(
            law["windows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|id| {
                    let mut kind = prototype.clone();
                    kind.id = id.as_str().unwrap().into();
                    kind
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
        let layout = |stacks: &Value| {
            let children: Vec<Value> = stacks
                .as_array()
                .unwrap()
                .iter()
                .map(|stack| {
                    let leaves: Vec<Value> = stack
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|leaf| {
                            let mut leaf = leaf.clone();
                            leaf["kind"] = Value::String("window".into());
                            leaf
                        })
                        .collect();
                    serde_json::json!({ "kind": "stack", "children": leaves })
                })
                .collect();
            serde_json::from_value(serde_json::json!({ "root": { "kind": "row", "children": children } })).unwrap()
        };
        if let Some(previous) = case.get("previousStacks") {
            shell.dock.layout_override = Some(layout(previous));
            SessionCtx::of(&mut shell).sync_dock();
        }
        shell.dock.layout_override = Some(layout(&case["stacks"]));
        SessionCtx::of(&mut shell).sync_dock();
        let expected: Vec<String> = case["windows"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().into()).collect();
        assert_eq!(shell.dock.dock.collect_window_ids(), expected, "{}", case["id"]);
        let mut stack_paths = Vec::new();
        paths(&shell.dock.dock.root, String::new(), &mut stack_paths);
        assert_eq!(serde_json::to_value(stack_paths).unwrap(), case["stackPaths"], "{}: surviving axis shape", case["id"]);
        let instances = session_window_instances(shell.docs.session.as_ref().unwrap(), &shell.dock.dock);
        let extras: Vec<&str> = instances.iter().filter(|instance| !law["windows"].as_array().unwrap().iter().any(|id| id == &instance.id)).map(|instance| instance.id.as_str()).collect();
        assert_eq!(serde_json::to_value(extras).unwrap(), case["extras"], "{}: published roster", case["id"]);
        for instance in instances {
            assert!(shell.docs.session.as_ref().unwrap().app.window_kinds.iter().any(|kind| kind.id == instance.window_kind_id));
        }
        assert_eq!(serde_json::to_value(&shell.windows.world_projection_template).unwrap(), case["templates"], "{}: projections only for declared kinds", case["id"]);
        SessionCtx::of(&mut shell).sync_dock();
        assert_eq!(shell.dock.dock.collect_window_ids(), expected, "{}: stable repeat", case["id"]);
    }
}
