//! 🎯️ LAW: completed chrome hits remain candidates until the matching pixels are accepted.

use super::*;

#[derive(serde::Deserialize)]
struct ControlFixture {
    id: String,
    rect: [f32; 4],
    binding: Option<String>,
}

#[derive(serde::Deserialize)]
struct ControlsFixture {
    presented: ControlFixture,
    candidate: ControlFixture,
}

#[derive(serde::Deserialize)]
struct ReplacementFixture {
    presented: ControlFixture,
    moved: ControlFixture,
    overlap: ControlFixture,
}

#[derive(serde::Deserialize)]
struct ExpectFixture {
    routed: Vec<String>,
    refused: Vec<String>,
    #[serde(rename = "presentedGeneration")]
    presented_generation: u64,
}

#[derive(serde::Deserialize)]
struct RowFixture {
    id: String,
    operations: Vec<String>,
    expect: ExpectFixture,
}

#[derive(serde::Deserialize)]
struct RetainedSelectFixture {
    key: String,
    value: String,
    binding: String,
    items: Vec<String>,
}

#[derive(serde::Deserialize)]
struct RetainedNumberStepperFixture {
    key: String,
    value: f64,
    step: f64,
    binding: String,
}

#[derive(serde::Deserialize)]
struct RetainedKeyboardFixture {
    key: String,
    binding: String,
}

#[derive(serde::Deserialize)]
struct RetainedAccessibilityFixture {
    key: String,
    label: String,
    value: String,
    binding: String,
}

#[derive(serde::Deserialize)]
struct RetainedPair<T> {
    presented: T,
    candidate: T,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetainedControlsFixture {
    select: RetainedPair<RetainedSelectFixture>,
    number_stepper: RetainedPair<RetainedNumberStepperFixture>,
    keyboard: RetainedPair<RetainedKeyboardFixture>,
    accessibility: RetainedPair<RetainedAccessibilityFixture>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InputAuthorityFixture {
    controls: ControlsFixture,
    replacement: ReplacementFixture,
    retained_controls: RetainedControlsFixture,
    rows: Vec<RowFixture>,
    laws: Vec<String>,
}

fn fixture() -> InputAuthorityFixture {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🎯️presented-input-authority/🔣️.json")).expect("presented input authority fixture")
}

fn hit(control: &ControlFixture) -> HitTarget<ActionDescriptor> {
    let event = control.binding.as_ref().map(|binding| ActionDescriptor {
        controller_id: "framework".into(),
        action: "setDriverSaveLabel".into(),
        args: crate::action_args_json!({ "value": binding }),
    });
    HitTarget { rect: Rect::new(control.rect[0], control.rect[1], control.rect[2], control.rect[3]), event, control_id: Some(control.id.clone()), kind: HitKind::Input, drag_axis: None, drag_data: None }
}

fn center(control: &ControlFixture) -> (f32, f32) {
    (control.rect[0] + control.rect[2] * 0.5, control.rect[1] + control.rect[3] * 0.5)
}

fn retained_select_record(control: &RetainedSelectFixture) -> ui_contract::UiNodeRecord {
    let items = control.items.iter().map(|value| serde_json::json!({ "value": value, "label": value })).collect::<Vec<_>>();
    serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": control.key,
        "component": { "type": "select", "value": control.value, "items": items },
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": "Theme" },
        "bindings": [{
            "trigger": "change",
            "action": { "scope": "framework", "name": control.binding, "version": 1 }
        }],
        "children": []
    }))
    .expect("retained presentation Select record")
}

fn retained_number_stepper_record(control: &RetainedNumberStepperFixture) -> ui_contract::UiNodeRecord {
    serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": control.key,
        "component": { "type": "numberStepper", "value": control.value, "step": control.step, "uniform": false },
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": "Scale" },
        "bindings": [{
            "trigger": "delta",
            "action": { "scope": "framework", "name": control.binding, "version": 1 }
        }],
        "children": []
    }))
    .expect("retained presentation NumberStepper record")
}

fn retained_input_record(control: &RetainedKeyboardFixture) -> ui_contract::UiNodeRecord {
    serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": control.key,
        "component": { "type": "input", "kind": "text", "value": "", "commit": "enter" },
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": "Keyboard owner" },
        "bindings": [{
            "trigger": "commit",
            "action": { "scope": "framework", "name": control.binding, "version": 1 }
        }],
        "children": []
    }))
    .expect("retained presentation Input record")
}

fn retained_accessibility_select_record(control: &RetainedAccessibilityFixture) -> ui_contract::UiNodeRecord {
    serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": control.key,
        "component": { "type": "select", "value": control.value, "items": [{ "value": "system", "label": "System" }, { "value": "dark", "label": "Dark" }] },
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": control.label },
        "bindings": [{
            "trigger": "change",
            "action": { "scope": "framework", "name": control.binding, "version": 1 }
        }],
        "children": []
    }))
    .expect("retained presentation accessibility Select record")
}

fn retained_panel_shell(surface: &str) -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.dock_tabs = ShellDock::default();
    shell.panel_anchors = std::array::from_fn(|_| PanelAnchorState::default());
    shell.dock_tabs.tabs_mut(PanelAnchor::BottomRight).push(DockTabNode::leaf(surface, "Settings", "settings", 0));
    *shell.anchor_state_mut(PanelAnchor::BottomRight) = PanelAnchorState { visible: true, size: 300.0, path: vec![surface.into()] };
    shell
}

fn retained_button_record(key: &str, binding: &str) -> ui_contract::UiNodeRecord {
    serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": key,
        "component": { "type": "button", "label": binding, "icon": "circle-dot" },
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": binding },
        "bindings": [{
            "trigger": "activate",
            "action": { "scope": "framework", "name": "setDriverSaveLabel", "version": 1 },
            "args": { "value": binding }
        }],
        "children": []
    }))
    .expect("retained presentation button record")
}

fn paint_retained_button_candidate(
    shell: &mut ShellState,
    surface: &str,
    document: &UiDocumentLease,
    body: Rect,
    input: &mut InputState<ActionDescriptor>,
) {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let theme = Theme::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = std::mem::take(&mut shell.world3d_states);
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut cursor = UiDocumentFrameCursor::default();
    let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
        let mut context = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
        let mut hosts = crate::scenes::SceneEngineHosts { world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: surface };
        let done = render_ui_document_step(&mut cursor, document, body, &mut context, surface, "s.test.presented-input", ui_wgpu::wgpu::UiDriverDrag::Handle, &mut hosts);
        assert!(done || !cursor.terminal_is_fault(), "the retained presentation document faulted in phase {}", cursor.phase_name());
        done
    });
    assert!(complete, "the retained presentation document painted within its opportunity ceiling");
    shell.world3d_states = world3d_states;
    shell.register_retained_body_hits(surface, body, input);
}

fn click(shell: &mut ShellState, control: &ControlFixture, input: &mut InputState<ActionDescriptor>, theme: &Theme) {
    let point = center(control);
    semio_framework_async::block_on(shell.handle_pointer_button(point.0, point.1, true, 0, input, theme)).expect("retained input press");
    semio_framework_async::block_on(shell.handle_pointer_button(point.0, point.1, false, 0, input, theme)).expect("retained input release");
}

fn admitted_bindings(input: &mut InputState<ActionDescriptor>) -> Vec<String> {
    crate::collect_fixture_actions(input)
        .into_iter()
        .filter(|action| action.action == "setDriverSaveLabel")
        .filter_map(|action| match action.args {
            Some(semio_framework::DslValue::Object(entries)) => entries.into_iter().find_map(|(key, value)| {
                (key == "value").then_some(value).and_then(|value| match value {
                    semio_framework::DslValue::String(value) => Some(value),
                    _ => None,
                })
            }),
            _ => None,
        })
        .collect()
}

fn publish_presented_button(
    shell: &mut ShellState,
    input: &mut InputState<ActionDescriptor>,
    surface: &str,
    key: &str,
    control: &ControlFixture,
) -> UiDocumentLease {
    let document = shell
        .publish_surface_records(surface, vec![retained_button_record(key, control.binding.as_deref().expect("button binding"))])
        .expect("retained button document publishes");
    paint_retained_button_candidate(shell, surface, &document, hit(control).rect, input);
    document
}

fn publish_retained_candidate(
    shell: &mut ShellState,
    input: &mut InputState<ActionDescriptor>,
    surface: &str,
    record: ui_contract::UiNodeRecord,
    body: Rect,
) -> UiDocumentLease {
    let document = shell.publish_surface_records(surface, vec![record]).expect("retained candidate document publishes");
    paint_retained_button_candidate(shell, surface, &document, body, input);
    document
}

#[test]
fn a_completed_chrome_registry_is_not_pointer_authority_before_its_pixels_are_presented() {
    let fixture = fixture();
    assert_eq!(fixture.laws.len(), 10, "the neutral fixture declares every closed presentation branch");
    let row = fixture.rows.iter().find(|row| row.id == "a-completed-chrome-walk-remains-candidate-while-presentation-is-pending").expect("pending presentation fixture row");
    assert_eq!(row.operations, ["presentInitial", "completeCandidateChrome", "pressCandidate", "pressPresented"]);
    assert_eq!(row.expect.presented_generation, 1);

    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();

    input.register_hit(hit(&fixture.controls.presented));
    shell.retained_hit_windows_staging.insert(fixture.controls.presented.id.clone(), ("surface.presented".into(), Rect::new(0.0, 0.0, 10.0, 10.0)));
    let initial = shell.seal_presented_input_candidate(&theme).expect("initial candidate witness");
    assert!(shell.acknowledge_presented_input(&mut input, initial));
    assert_eq!(input.hit_generation(), 1, "the initial accepted frame owns generation one");

    while input.retire_hit_step() {}
    input.register_hit(hit(&fixture.controls.candidate));
    shell.retained_hit_windows_staging.insert(fixture.controls.candidate.id.clone(), ("surface.candidate".into(), Rect::new(20.0, 0.0, 10.0, 10.0)));
    let _candidate = shell.seal_presented_input_candidate(&theme).expect("pending candidate witness");

    let candidate = center(&fixture.controls.candidate);
    let presented = center(&fixture.controls.presented);
    let routed = [input.hit_at(candidate.0, candidate.1), input.hit_at(presented.0, presented.1)].into_iter().flatten().filter_map(|target| target.control_id.clone()).collect::<Vec<_>>();
    let refused = [&fixture.controls.candidate, &fixture.controls.presented]
        .into_iter()
        .filter(|control| {
            let point = center(control);
            input.hit_at(point.0, point.1).is_none()
        })
        .map(|control| control.id.clone())
        .collect::<Vec<_>>();

    assert_eq!(routed, row.expect.routed, "only controls whose pixels were accepted route");
    assert_eq!(refused, row.expect.refused, "successor-only controls remain non-interactive while its packet is pending");
    assert_eq!(input.hit_generation(), row.expect.presented_generation, "a completed chrome candidate cannot advance the presented input generation");
    assert_eq!(shell.retained_hit_windows.get(&fixture.controls.presented.id).map(|(owner, _)| owner.as_str()), Some("surface.presented"), "the owner map stays on the same accepted frame as the hits");
    assert!(!shell.retained_hit_windows.contains_key(&fixture.controls.candidate.id));
}

#[test]
fn presentation_acceptance_and_abort_are_generation_bound() {
    let fixture = fixture();
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();

    input.register_hit(hit(&fixture.controls.presented));
    shell.retained_hit_windows_staging.insert(fixture.controls.presented.id.clone(), ("surface.presented".into(), Rect::new(0.0, 0.0, 10.0, 10.0)));
    let first = shell.seal_presented_input_candidate(&theme).expect("first candidate witness");
    let stale = PresentedInputCandidateWitness(first.0.saturating_add(1));
    assert!(!shell.acknowledge_presented_input(&mut input, stale), "a stale presenter cannot publish another frame's candidate");
    assert!(shell.acknowledge_presented_input(&mut input, first));
    while input.retire_hit_step() {}

    input.register_hit(hit(&fixture.controls.candidate));
    shell.retained_hit_windows_staging.insert(fixture.controls.candidate.id.clone(), ("surface.candidate".into(), Rect::new(20.0, 0.0, 10.0, 10.0)));
    let second = shell.seal_presented_input_candidate(&theme).expect("second candidate witness");
    assert_ne!(first, second, "successive presentations receive distinct witnesses even when host input generation does not change");
    assert!(shell.acknowledge_presented_input(&mut input, second));
    assert_eq!(shell.retained_hit_windows.get(&fixture.controls.candidate.id).map(|(owner, _)| owner.as_str()), Some("surface.candidate"), "acceptance swaps hits and their owner map together");
    let presented = center(&fixture.controls.presented);
    let candidate = center(&fixture.controls.candidate);
    assert!(input.hit_at(presented.0, presented.1).is_none());
    assert_eq!(input.hit_at(candidate.0, candidate.1).and_then(|target| target.control_id.as_deref()), Some("fixture.candidate"));

    while input.retire_hit_step() {}
    input.register_hit(hit(&fixture.controls.presented));
    shell.retained_hit_windows_staging.insert(fixture.controls.presented.id.clone(), ("surface.aborted".into(), Rect::new(0.0, 0.0, 10.0, 10.0)));
    let aborted = shell.seal_presented_input_candidate(&theme).expect("aborted candidate witness");
    assert!(shell.discard_presented_input_candidate(aborted));
    assert!(!shell.acknowledge_presented_input(&mut input, aborted), "an aborted witness cannot be acknowledged later");
    assert!(input.hit_at(presented.0, presented.1).is_none());
    assert_eq!(input.hit_at(candidate.0, candidate.1).and_then(|target| target.control_id.as_deref()), Some("fixture.candidate"));
    assert_eq!(shell.retained_hit_windows.get(&fixture.controls.candidate.id).map(|(owner, _)| owner.as_str()), Some("surface.candidate"), "aborting a successor preserves the accepted owner map");
    assert_eq!(shell.presented_input_epoch, second.0, "abort preserves the exact accepted presentation epoch");
}

#[test]
fn actual_retained_pointer_dispatch_keeps_the_presented_position_and_binding_until_acceptance() {
    let fixture = fixture();
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    let presented = &fixture.replacement.presented;
    let moved = &fixture.replacement.moved;
    let overlap = &fixture.replacement.overlap;
    let presented_surface = "presented-input.same-key";
    let overlap_surface = "presented-input.overlap";

    let mut presented_document = shell
        .publish_surface_records(presented_surface, vec![retained_button_record("stable-button", presented.binding.as_deref().expect("presented binding"))])
        .expect("presented retained document publishes");
    crate::interpreter::begin_accessibility_visible_documents();
    paint_retained_button_candidate(&mut shell, presented_surface, &presented_document, hit(presented).rect, &mut input);
    let presented_witness = shell.seal_presented_input_candidate(&theme).expect("presented candidate witness");
    assert!(shell.acknowledge_presented_input(&mut input, presented_witness));

    let mut moved_document = shell
        .publish_surface_records(presented_surface, vec![retained_button_record("stable-button", moved.binding.as_deref().expect("moved binding"))])
        .expect("moved retained document publishes");
    let mut overlap_document = shell
        .publish_surface_records(overlap_surface, vec![retained_button_record("overlap-button", overlap.binding.as_deref().expect("overlap binding"))])
        .expect("overlap retained document publishes");
    crate::interpreter::begin_accessibility_visible_documents();
    paint_retained_button_candidate(&mut shell, presented_surface, &moved_document, hit(moved).rect, &mut input);
    paint_retained_button_candidate(&mut shell, overlap_surface, &overlap_document, hit(overlap).rect, &mut input);
    let moved_witness = shell.seal_presented_input_candidate(&theme).expect("moved candidate witness");

    click(&mut shell, presented, &mut input, &theme);
    assert_eq!(admitted_bindings(&mut input), ["presented-binding"], "the old pixels retain the old event binding while the successor waits");
    click(&mut shell, moved, &mut input, &theme);
    assert!(admitted_bindings(&mut input).is_empty(), "the moved candidate cannot dispatch before its pixels are accepted");

    assert!(!shell.acknowledge_presented_input(&mut input, moved_witness), "presented interaction makes the pre-click candidate witness stale");
    assert!(shell.discard_presented_input_candidate(moved_witness));
    while input.retire_hit_step() {}
    crate::interpreter::begin_accessibility_visible_documents();
    paint_retained_button_candidate(&mut shell, presented_surface, &moved_document, hit(moved).rect, &mut input);
    paint_retained_button_candidate(&mut shell, overlap_surface, &overlap_document, hit(overlap).rect, &mut input);
    let rebased_witness = shell.seal_presented_input_candidate(&theme).expect("rebased moved candidate witness");
    assert!(shell.acknowledge_presented_input(&mut input, rebased_witness));
    click(&mut shell, overlap, &mut input, &theme);
    assert_eq!(admitted_bindings(&mut input), ["overlap-binding"], "the accepted overlap replaces the old binding at the old pixels");
    click(&mut shell, moved, &mut input, &theme);
    assert_eq!(admitted_bindings(&mut input), ["moved-binding"], "the same-key successor dispatches its new binding only from its accepted position");

    while !presented_document.close_step() {}
    while !moved_document.close_step() {}
    while !overlap_document.close_step() {}
}

#[test]
fn presented_button_release_without_down_does_not_activate() {
    let fixture = fixture();
    let control = &fixture.replacement.presented;
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    crate::interpreter::begin_accessibility_visible_documents();
    let mut document = publish_presented_button(&mut shell, &mut input, "presented-input.no-down", "no-down", control);
    let witness = shell.seal_presented_input_candidate(&theme).expect("button candidate witness");
    assert!(shell.acknowledge_presented_input(&mut input, witness));

    let point = center(control);
    semio_framework_async::block_on(shell.handle_pointer_button_for(ui_render::PointerId(71), point.0, point.1, false, 0, &mut input, &theme)).expect("unmatched release routes");

    assert!(admitted_bindings(&mut input).is_empty(), "a release without an admitted press cannot activate a presented button");
    while !document.close_step() {}
}

#[test]
fn presented_button_cancelled_press_cannot_activate_on_later_release() {
    let fixture = fixture();
    let control = &fixture.replacement.presented;
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    crate::interpreter::begin_accessibility_visible_documents();
    let mut document = publish_presented_button(&mut shell, &mut input, "presented-input.cancel", "cancel", control);
    let witness = shell.seal_presented_input_candidate(&theme).expect("button candidate witness");
    assert!(shell.acknowledge_presented_input(&mut input, witness));

    let pointer = ui_render::PointerId(72);
    let point = center(control);
    semio_framework_async::block_on(shell.handle_pointer_button_for(pointer, point.0, point.1, true, 0, &mut input, &theme)).expect("presented press routes");
    shell.handle_pointer_cancel_for(pointer, &mut input);
    semio_framework_async::block_on(shell.handle_pointer_button_for(pointer, point.0, point.1, false, 0, &mut input, &theme)).expect("post-cancel release routes");

    assert!(admitted_bindings(&mut input).is_empty(), "cancellation retires the exact presented button press owner");
    while !document.close_step() {}
}

#[test]
fn presented_button_press_a_release_b_activates_neither() {
    let fixture = fixture();
    let control_a = &fixture.replacement.presented;
    let control_b = &fixture.replacement.moved;
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    crate::interpreter::begin_accessibility_visible_documents();
    let mut document_a = publish_presented_button(&mut shell, &mut input, "presented-input.press-a", "press-a", control_a);
    let mut document_b = publish_presented_button(&mut shell, &mut input, "presented-input.release-b", "release-b", control_b);
    let witness = shell.seal_presented_input_candidate(&theme).expect("button candidate witness");
    assert!(shell.acknowledge_presented_input(&mut input, witness));

    let pointer = ui_render::PointerId(73);
    let point_a = center(control_a);
    let point_b = center(control_b);
    semio_framework_async::block_on(shell.handle_pointer_button_for(pointer, point_a.0, point_a.1, true, 0, &mut input, &theme)).expect("button A press routes");
    semio_framework_async::block_on(shell.handle_pointer_button_for(pointer, point_b.0, point_b.1, false, 0, &mut input, &theme)).expect("button B release routes");

    assert!(admitted_bindings(&mut input).is_empty(), "a release over a different presented button cannot borrow the first button's press");
    while !document_a.close_step() {}
    while !document_b.close_step() {}
}


#[test]
fn pending_select_uses_the_presented_options_and_binding_for_keyboard_input() {
    let fixture = fixture();
    let control = &fixture.retained_controls.select;
    let surface = "presented-input.select";
    let body = Rect::new(0.0, 0.0, 240.0, 32.0);
    let theme = Theme::default();
    let mut shell = retained_panel_shell(surface);
    let mut input = InputState::<ActionDescriptor>::default();
    crate::interpreter::begin_accessibility_visible_documents();
    let mut presented = publish_retained_candidate(&mut shell, &mut input, surface, retained_select_record(&control.presented), body);
    let witness = shell.seal_presented_input_candidate(&theme).expect("presented Select witness");
    assert!(shell.acknowledge_presented_input(&mut input, witness));
    let target = ui_render::AccessibilityTarget { window_id: surface.into(), window_generation: 1, node_id: 1, node_key: control.presented.key.clone() };
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).expect("presented Select focus"));

    crate::interpreter::begin_accessibility_visible_documents();
    let mut candidate = publish_retained_candidate(&mut shell, &mut input, surface, retained_select_record(&control.candidate), Rect::new(260.0, 0.0, 240.0, 32.0));
    let pending = shell.seal_presented_input_candidate(&theme).expect("pending Select witness");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).expect("presented Select opens");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).expect("presented Select commits");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), [control.presented.binding.as_str()], "keyboard dispatch remains on the accepted Select revision");
    assert!(shell.discard_presented_input_candidate(pending));
    while !presented.close_step() {}
    while !candidate.close_step() {}
}

#[test]
fn pending_number_stepper_uses_the_presented_value_step_and_binding() {
    let fixture = fixture();
    let control = &fixture.retained_controls.number_stepper;
    let surface = "presented-input.number-stepper";
    let body = Rect::new(0.0, 0.0, 240.0, 32.0);
    let theme = Theme::default();
    let mut shell = retained_panel_shell(surface);
    let mut input = InputState::<ActionDescriptor>::default();
    crate::interpreter::begin_accessibility_visible_documents();
    let mut presented = publish_retained_candidate(&mut shell, &mut input, surface, retained_number_stepper_record(&control.presented), body);
    let witness = shell.seal_presented_input_candidate(&theme).expect("presented NumberStepper witness");
    assert!(shell.acknowledge_presented_input(&mut input, witness));
    let target = ui_render::AccessibilityTarget { window_id: surface.into(), window_generation: 1, node_id: 1, node_key: control.presented.key.clone() };
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).expect("presented NumberStepper focus"));

    crate::interpreter::begin_accessibility_visible_documents();
    let mut candidate = publish_retained_candidate(&mut shell, &mut input, surface, retained_number_stepper_record(&control.candidate), Rect::new(260.0, 0.0, 240.0, 32.0));
    let pending = shell.seal_presented_input_candidate(&theme).expect("pending NumberStepper witness");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::ArrowUp, &PointerModifiers::default(), &mut input)).expect("presented NumberStepper increments");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), [control.presented.binding.as_str()], "NumberStepper dispatch retains the accepted value, step and binding");
    assert!(shell.discard_presented_input_candidate(pending));
    while !presented.close_step() {}
    while !candidate.close_step() {}
}

#[test]
fn pending_candidate_cannot_replace_presented_keyboard_focus_or_tab_authority() {
    let fixture = fixture();
    let control = &fixture.retained_controls.keyboard;
    let surface = "presented-input.keyboard";
    let body = Rect::new(0.0, 0.0, 240.0, 32.0);
    let theme = Theme::default();
    let mut shell = retained_panel_shell(surface);
    let mut input = InputState::<ActionDescriptor>::default();
    crate::interpreter::begin_accessibility_visible_documents();
    let mut presented = publish_retained_candidate(&mut shell, &mut input, surface, retained_input_record(&control.presented), body);
    let witness = shell.seal_presented_input_candidate(&theme).expect("presented keyboard witness");
    assert!(shell.acknowledge_presented_input(&mut input, witness));
    let target = ui_render::AccessibilityTarget { window_id: surface.into(), window_generation: 1, node_id: 1, node_key: control.presented.key.clone() };
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).expect("presented Input focus"));

    crate::interpreter::begin_accessibility_visible_documents();
    let mut candidate = publish_retained_candidate(&mut shell, &mut input, surface, retained_input_record(&control.candidate), body);
    let pending = shell.seal_presented_input_candidate(&theme).expect("pending keyboard witness");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Char("x".into()), &PointerModifiers::default(), &mut input)).expect("presented Input text");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Enter, &PointerModifiers::default(), &mut input)).expect("presented Input commit");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), [control.presented.binding.as_str()], "keyboard dispatch stays on the visible retained revision");
    assert!(shell.discard_presented_input_candidate(pending));
    while !presented.close_step() {}
    while !candidate.close_step() {}
}

#[test]
fn pending_candidate_cannot_replace_the_presented_accessibility_projection_or_action() {
    let fixture = fixture();
    let control = &fixture.retained_controls.accessibility;
    let surface = "presented-input.accessibility";
    let body = Rect::new(0.0, 0.0, 240.0, 32.0);
    let theme = Theme::default();
    let mut shell = retained_panel_shell(surface);
    let mut input = InputState::<ActionDescriptor>::default();
    crate::interpreter::begin_accessibility_visible_documents();
    let mut presented = publish_retained_candidate(&mut shell, &mut input, surface, retained_accessibility_select_record(&control.presented), body);
    let witness = shell.seal_presented_input_candidate(&theme).expect("presented accessibility witness");
    assert!(shell.acknowledge_presented_input(&mut input, witness));
    let target = ui_render::AccessibilityTarget { window_id: surface.into(), window_generation: 1, node_id: 1, node_key: control.presented.key.clone() };

    crate::interpreter::begin_accessibility_visible_documents();
    let mut candidate = publish_retained_candidate(&mut shell, &mut input, surface, retained_accessibility_select_record(&control.candidate), body);
    let pending = shell.seal_presented_input_candidate(&theme).expect("pending accessibility witness");
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Value(control.presented.value.clone()), &mut input)).expect("presented accessibility Value dispatch"));
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), [control.presented.binding.as_str()], "assistive input dispatch stays on the accepted projection");
    assert!(shell.discard_presented_input_candidate(pending));
    while !presented.close_step() {}
    while !candidate.close_step() {}
}

#[test]
fn presented_interaction_invalidates_a_candidate_built_from_an_older_input_epoch() {
    let fixture = fixture();
    let control = &fixture.replacement.presented;
    let surface = "presented-input.stale-interaction";
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    crate::interpreter::begin_accessibility_visible_documents();
    let mut presented = publish_presented_button(&mut shell, &mut input, surface, "interaction-owner", control);
    let witness = shell.seal_presented_input_candidate(&theme).expect("presented interaction witness");
    assert!(shell.acknowledge_presented_input(&mut input, witness));

    crate::interpreter::begin_accessibility_visible_documents();
    let mut candidate = publish_presented_button(&mut shell, &mut input, surface, "interaction-owner", control);
    let pending = shell.seal_presented_input_candidate(&theme).expect("pending interaction witness");
    let point = center(control);
    semio_framework_async::block_on(shell.handle_pointer_button_for(ui_render::PointerId(74), point.0, point.1, true, 0, &mut input, &theme)).expect("presented press mutates interaction state");
    assert!(!shell.acknowledge_presented_input(&mut input, pending), "a candidate based on the pre-press interaction epoch cannot replace the visible gesture owner");
    assert!(shell.discard_presented_input_candidate(pending));
    shell.handle_pointer_cancel_for(ui_render::PointerId(74), &mut input);
    while !presented.close_step() {}
    while !candidate.close_step() {}
}
