// 🖼️ Facade, frame, storm and mounted layout laws: the apply tree and frame laws, the retained paint walk, the dispatch click, the resize and interactive storms, the large layout shaping job and the mounted layout snapshot, theme and revision laws.

//#region 🔖️FacadeTests
#[test]
fn apply_tree_then_frame_produces_a_non_empty_draw_list() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("main", &stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("hi"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]));

    assert!(ui.needs_frame(), "a freshly applied tree must report needing a frame");
    drive_layout(&mut ui, "main", 400.0, 400.0, &mut atlas);
    let draw = ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None).expect("frame must produce a draw list once a tree was applied");
    let total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    assert!(total > 0, "expected the text node to emit at least one glyph instance");
}

#[test]
fn frame_before_any_apply_tree_returns_none() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    assert!(ui.frame::<RecordingSceneHost>("nonexistent", 400.0, 400.0, &mut atlas, None, None).is_none());
}

#[test]
fn retained_paint_walk_yields_one_node_or_scalar_per_step_in_tree_order() {
    let mut tree = UiTree::new();
    let root = tree.insert_child(None, retained_walk_leaf(0, 0, "root"));
    let first = tree.insert_child(Some(root), retained_walk_leaf(1, 0, "first"));
    let second = tree.insert_child(Some(root), retained_walk_leaf(1, 1, "second"));
    let nested = tree.insert_child(Some(first), retained_walk_leaf(2, 0, "nested"));
    let expected = [root, first, nested, second];
    let mut observed = [None; 4];
    let mut observed_len = 0;
    let mut walk = RetainedPaintWalk::new(&tree, root);
    let mut complete = false;
    for _ in 0..16 {
        match walk.step(&tree) {
            RetainedPaintWalkStep::Visit(node, _, _, _) => {
                observed[observed_len] = Some(node);
                observed_len += 1;
            }
            RetainedPaintWalkStep::Scalar => {}
            RetainedPaintWalkStep::Complete => {
                complete = true;
                break;
            }
            RetainedPaintWalkStep::DepthFault => panic!("bounded tree should not exhaust retained depth credits"),
        }
    }
    assert!(complete);
    assert_eq!(observed, expected.map(Some));
}

#[test]
fn retained_paint_walk_depth_cap_plus_one_faults_without_dynamic_spill() {
    let mut tree = UiTree::new();
    let root = tree.insert_child(None, retained_walk_leaf(0, 0, "root"));
    let mut parent = root;
    for ordinal in 1..=RETAINED_PAINT_DEPTH_CREDITS {
        let ordinal = match u32::try_from(ordinal) {
            Ok(ordinal) => ordinal,
            Err(_) => panic!("retained depth credit must fit a node ordinal"),
        };
        parent = tree.insert_child(Some(parent), retained_walk_leaf(1, ordinal, "child"));
    }
    let mut walk = RetainedPaintWalk::new(&tree, root);
    let mut faulted = false;
    for _ in 0..(RETAINED_PAINT_DEPTH_CREDITS * 3) {
        if matches!(walk.step(&tree), RetainedPaintWalkStep::DepthFault) {
            faulted = true;
            break;
        }
    }
    assert!(faulted);
}

#[test]
fn needs_frame_is_false_once_a_stable_tree_has_been_framed() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    let ui_node = stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("hi"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]);
    ui.apply_tree("main", &ui_node);
    drive_layout(&mut ui, "main", 400.0, 400.0, &mut atlas);
    ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None);
    assert!(!ui.needs_frame(), "nothing changed since the last frame, so no frame should be needed");

    ui.apply_tree("main", &ui_node);
    assert!(!ui.needs_frame(), "re-applying an identical tree must set zero dirty flags (reconcile's own golden rule)");
}

#[test]
fn dispatch_event_emits_a_button_click_command_once() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("main", &stack_ui(vec![button_ui("go", "Go")]));
    drive_layout(&mut ui, "main", 400.0, 400.0, &mut atlas);
    ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None);

    ui.dispatch_event("main", UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let commands = ui.dispatch_event("main", UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { intent, .. } if intent.descriptor() == action())));
    assert!(ui.drain_commands().is_empty(), "synchronous commands have the return value as their single consumption owner");
}

#[test]
fn set_window_layout_wires_into_the_facades_shell() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    ui.set_window_layout(crate::wgpu::even_window_layout(&["app.viewport".to_string()]));
    assert!(ui.shell().window_layout().is_some());
}

#[test]
fn resize_storm_coalesces_to_one_latest_surface_job() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("resize", &stack_ui(vec![button_ui("go", "Go")]));
    for width in 1..=2_000 {
        ui.set_viewport("resize", width as f32, 480.0);
    }
    assert_eq!(ui.layout_queues.iter().map(SurfaceLaneRing::len).sum::<usize>(), 1, "a resize storm must retain one coalesced surface entry");

    drive_layout(&mut ui, "resize", 2_000.0, 480.0, &mut atlas);
    let root = ui.tree("resize").and_then(|tree| tree.root).expect("root");
    assert_eq!(ui.tree("resize").and_then(|tree| tree.accepted_layout(root)).expect("accepted root").width, 2_000.0);
}

#[test]
fn interactive_storm_does_not_starve_background_surface_lane() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("interactive", &stack_ui(vec![button_ui("go", "Go")]));
    ui.apply_tree("background", &stack_ui(vec![button_ui("bg", "Background")]));
    ui.set_surface_lane("interactive", SurfaceLane::Interactive);
    ui.set_surface_lane("background", SurfaceLane::Background);
    ui.set_viewport("interactive", 400.0, 400.0);
    ui.set_viewport("background", 400.0, 400.0);

    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let pool = test_layout_pool();
    let mut preview_sequence = 0;
    let mut background_progress_at = None;
    for slice in 0..12 {
        ui.set_viewport("interactive", 401.0 + slice as f32, 400.0);
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), test_clock, &mut preview_sequence,&mut actual_retained_progress);
        let step = ui.step_layouts(&pool, &mut atlas, &mut cx);
        if matches!(step, UiLayoutStep::Yielded { ref window_id, .. } | UiLayoutStep::Ready { ref window_id, .. } if window_id.as_ref() == "background") {
            background_progress_at = Some(slice);
            break;
        }
    }
    assert!(background_progress_at.is_some_and(|slice| slice < LANE_WHEEL.len()), "the weighted wheel must service background within one six-slot cycle");
}

/// 🧵️ LAW: a 1,025-node text workload is admitted ONE work unit per slice — never a batch whose size
/// grows with the tree.
///
/// This used to assert a WALL-CLOCK ceiling (`max_slice < 8ms`) and it measured the machine, not the
/// engine: it failed at 22.9 ms, 27.5 ms, 31.5 ms and 174 ms whenever peer cargos were resident and
/// passed on a quiet box, so a red run said nothing about the code
/// (`📓️w2-w6-integration.md` §6, `📓️w8a`, `📓️w9a`). The eight milliseconds were only ever a PROXY for
/// the property that actually holds the frame budget: each `step_layouts` call advances the admission
/// cursor by exactly one node or one scalar, so a slice costs the same whether the tree has ten nodes
/// or ten thousand. That is what is asserted here, in units the engine itself reports
/// (`UiLayoutStep::Yielded { nodes, glyphs }`), and it is machine-independent.
///
/// `admit_node_one` answers `(1, 0)` for the node it admits, `admit_text_one` answers `(0, 1)` for the
/// single scalar it shapes, and `unwind_one` answers `(1, 0)` when it pops a frame — hence two node
/// units per node in the tree, one glyph unit per scalar in every label, and never two of anything in
/// one slice.
///
/// **React ref:** React's own layout is synchronous and unsliced; this engine slices because it owns
/// the frame budget React gets from the browser for free, which is why the chunking — not a duration —
/// is the law.
#[test]
fn large_layout_and_shaping_job_admits_one_work_unit_per_slice() {
    let labels: Vec<String> = (0..1_024).map(|index| format!("node-{index}")).collect();
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    let children = labels.iter().map(|label| UiNode::Text(UiTextNode { value: Label::data(label.clone()), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })).collect();
    ui.apply_tree("large", &stack_ui(children));
    ui.set_viewport("large", 1_920.0, 1_080.0);

    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let pool = test_layout_pool();
    let mut preview_sequence = 0;
    let mut slices = 0;
    let mut widest_slice = (0usize, 0usize);
    let mut admitted = (0usize, 0usize);
    loop {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), test_clock, &mut preview_sequence,&mut actual_retained_progress);
        let step = ui.step_layouts(&pool, &mut atlas, &mut cx);
        slices += 1;
        if let UiLayoutStep::Yielded { stage, nodes, glyphs, .. } = step {
            widest_slice = (widest_slice.0.max(nodes), widest_slice.1.max(glyphs));
            if stage == "Layout.CollectNodes" {
                admitted = (admitted.0 + nodes, admitted.1 + glyphs);
            }
            continue;
        }
        if matches!(step, UiLayoutStep::Idle) {
            break;
        }
    }
    let tree = ui.tree("large").expect("large tree");
    let root = tree.root.expect("large root");
    let children: Vec<_> = tree.children(root).collect();
    let scalars: usize = labels.iter().map(|label| label.chars().count()).sum();
    assert!(slices > 10_000, "the 1,025-node/text workload must be observably chunked, got {slices} slices");
    assert_eq!(widest_slice, (1, 1), "no slice may admit more than one node and one glyph, got {widest_slice:?}");
    assert_eq!(admitted, (2 * (children.len() + 1), scalars), "admission must visit and unwind every node once and shape every label scalar once");
    assert_eq!(tree.accepted_layout(root).expect("accepted root").width, 1_920.0);
    assert!(tree.accepted_layout(*children.last().expect("last child")).expect("accepted last child").y > tree.accepted_layout(children[0]).expect("accepted first child").y);
}

#[test]
fn mounted_layout_surface_max_plus_one_returns_exact_owner_without_mutation() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    for index in 0..UI_LAYOUT_SURFACE_SLOTS {
        let id = SurfaceId::try_from(format!("mounted-{index}")).unwrap_or_else(|_| panic!("bounded mounted surface"));
        assert!(ui.windows.try_admit(id).is_ok());
    }
    let before: UiFixedList<SurfaceId, UI_LAYOUT_SURFACE_SLOTS> = ui.windows.ids().cloned().fold(UiFixedList::default(), |mut ids, id| {
        assert_eq!(ids.try_push(id), Ok(()));
        ids
    });
    let owner = SurfaceId::try_from("mounted-max-plus-one").unwrap_or_else(|_| panic!("bounded rejected surface"));
    let rejected = ui.windows.try_admit(owner.clone()).unwrap_err();
    assert_eq!(rejected.id, owner);
    assert_eq!(ui.windows.ids().cloned().collect::<Vec<_>>(), before.iter().cloned().collect::<Vec<_>>());
}

#[test]
fn mounted_layout_equal_theme_does_not_invalidate_or_requeue() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    ui.apply_tree("theme", &stack_ui(vec![button_ui("same", "Same")]));
    let before_generation = ui.windows.get("theme").map(|window| window.layout_generation);
    let before_theme_revision = ui.windows.get("theme").map(|window| window.theme_revision);
    let before_queue = ui.layout_queues.iter().map(SurfaceLaneRing::len).sum::<usize>();
    ui.set_theme(ui.theme());
    assert_eq!(ui.windows.get("theme").map(|window| window.layout_generation), before_generation);
    assert_eq!(ui.windows.get("theme").map(|window| window.theme_revision), before_theme_revision);
    assert_eq!(ui.layout_queues.iter().map(SurfaceLaneRing::len).sum::<usize>(), before_queue);
}

#[test]
fn changed_theme_propagates_one_fixed_surface_slot_per_opportunity() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    ui.apply_tree("theme-a", &stack_ui(vec![button_ui("a", "A")]));
    ui.apply_tree("theme-b", &stack_ui(vec![button_ui("b", "B")]));
    let before_a = ui.windows.get("theme-a").map(|window| window.theme_revision);
    let before_b = ui.windows.get("theme-b").map(|window| window.theme_revision);
    let mut changed = ui.theme();
    changed.gap_standard += 1.0;
    ui.set_theme(changed);
    assert_eq!(ui.windows.get("theme-a").map(|window| window.theme_revision), before_a);
    assert_eq!(ui.windows.get("theme-b").map(|window| window.theme_revision), before_b);
    assert!(ui.drive_theme_propagation_one());
    assert!(ui.theme_propagation.as_ref().is_some_and(|cursor| cursor.phase == ThemePropagationPhase::Validate && cursor.slot == 1));
    assert_eq!(ui.windows.get("theme-a").map(|window| window.theme_revision), before_a);
    assert_eq!(ui.windows.get("theme-b").map(|window| window.theme_revision), before_b);
}

#[test]
fn mounted_layout_atomic_snapshot_keeps_last_valid_geometry_until_fresh_swap() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("atomic", &stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("atomic"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), button_ui("target", "Target")]));
    drive_layout(&mut ui, "atomic", 320.0, 200.0, &mut atlas);
    let tree = ui.tree("atomic").unwrap_or_else(|| panic!("atomic tree"));
    let root = tree.root.unwrap_or_else(|| panic!("atomic root"));
    let old_generation = tree.accepted_layout_generation();
    let old_layout = tree.accepted_layout(root).unwrap_or_default();
    ui.set_viewport("atomic", 960.0, 540.0);
    let pool = test_layout_pool();
    let cancel = semio_framework_job::CancelToken::root_now();
    let operation = semio_framework_job::allocate_operation_id();
    let mut preview_sequence = 0;
    let mut swaps = 0;
    for _ in 0..100_000 {
        let before = ui.tree("atomic").map(UiTree::accepted_layout_generation).unwrap_or_default();
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), test_clock, &mut preview_sequence,&mut actual_retained_progress);
        let step = ui.step_layouts(&pool, &mut atlas, &mut cx);
        let tree = ui.tree("atomic").unwrap_or_else(|| panic!("atomic retained tree"));
        let after = tree.accepted_layout_generation();
        swaps += usize::from(before != after);
        if matches!(step, UiLayoutStep::Ready { .. }) {
            assert_eq!(tree.accepted_layout(root).unwrap_or_default().width, 960.0);
            break;
        }
        assert_eq!(after, old_generation);
        assert_eq!(tree.accepted_layout(root).unwrap_or_default(), old_layout);
    }
    assert_eq!(swaps, 1);
    assert!(ui.progressive_layout_preview("atomic").is_some());
    assert!(ui.progressive_glyph_preview("atomic").is_some());
}

#[test]
fn mounted_layout_revision_max_refuses_theme_tree_and_viewport_without_alias() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let original = stack_ui(vec![button_ui("original", "Original")]);
    ui.apply_tree("max", &original);
    let original_theme = ui.theme();
    if let Some(window) = ui.windows.get_mut("max") {
        window.theme_revision = u64::MAX;
    }
    let mut changed_theme = original_theme;
    changed_theme.gap_standard += 1.0;
    ui.set_theme(changed_theme);
    assert_eq!(theme_layout_identity(&ui.theme()), theme_layout_identity(&original_theme));
    if let Some(window) = ui.windows.get_mut("max") {
        window.theme_revision = 1;
        window.viewport_revision = u64::MAX;
        window.layout_generation = u64::MAX - 1;
    }
    let before_viewport = ui.viewport("max");
    ui.set_viewport("max", 777.0, 333.0);
    assert_eq!(ui.viewport("max"), before_viewport);
    if let Some(window) = ui.windows.get_mut("max") {
        window.viewport_revision = 1;
        window.layout_generation = 7;
        window.revision = u64::MAX;
    }
    ui.apply_tree("max", &stack_ui(vec![button_ui("changed", "Changed")]));
    assert_eq!(ui.tree_revision("max"), Some(u64::MAX));
    assert!(ui.tree("max").and_then(|tree| tree.root).and_then(|root| ui.tree("max").and_then(|tree| tree.node(root))).is_some_and(|node| node.spec.0 == original));
}

#[test]
fn mounted_layout_replay_and_resize_supersede_are_deterministic() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    let input = stack_ui(vec![button_ui("replay", "Replay")]);
    ui.apply_tree("replay", &input);
    drive_layout(&mut ui, "replay", 640.0, 480.0, &mut atlas);
    let root = ui.tree("replay").and_then(|tree| tree.root).unwrap_or_else(|| panic!("replay root"));
    let first = ui.tree("replay").and_then(|tree| tree.accepted_layout(root)).unwrap_or_default();
    ui.set_viewport("replay", 641.0, 480.0);
    ui.set_viewport("replay", 640.0, 480.0);
    drive_layout(&mut ui, "replay", 640.0, 480.0, &mut atlas);
    let second = ui.tree("replay").and_then(|tree| tree.accepted_layout(root)).unwrap_or_default();
    assert_eq!(first, second);
    assert_eq!(second.width, 640.0);
}
//#endregion 🔖️FacadeTests
