//! 🧪️ LAW: the wgpu target mounts and paints every accept case of the language-neutral conformance corpus
//! (`🧬️contract/🧫️fixtures/🧪️conformance/`) — the same `📇️catalog.json` the Rust contract harness
//! (`🔬️conformance-unit`) and the React interpreter (`🧪️unknown-component-placeholder`) walk — and carries
//! each component's own facts into its retained node: a slider's detents paint one tick each, a stepper and
//! a number field carry their precision, a recipe keeps its keys, a dialog's choices stay buttons.

use super::*;
use crate::wgpu::engine::{Ui, UiLayoutStep};
use crate::wgpu::text::FontAtlas;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiRevision};

fn corpus_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧬️contract/🧫️fixtures/🧪️conformance")
}

fn read(path: &std::path::Path) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))).unwrap_or_else(|error| panic!("parse {path:?}: {error}"))
}

/// 📇️ Every accept case of the shape groups, as `(group, case, snapshot, expectation)`.
fn accept_cases() -> Vec<(String, String, serde_json::Value, serde_json::Value)> {
    let catalog = read(&corpus_dir().join("📇️catalog.json"));
    let mut cases = Vec::new();
    for group in ["🧩️component", "🖥️composite", "📐️layout", "♿️accessibility"] {
        for (case, directory) in catalog["groups"][group]["cases"].as_object().expect("catalog cases") {
            let folder = corpus_dir().join(group).join(directory.as_str().expect("case directory"));
            cases.push((group.to_string(), case.clone(), read(&folder.join(catalog["roles"]["snapshot"].as_str().expect("snapshot role"))), read(&folder.join(catalog["roles"]["expect"].as_str().expect("expect role")))));
        }
    }
    cases
}

fn document(snapshot: &serde_json::Value, generation: u64) -> UiDocumentTree {
    let nodes = snapshot["nodes"].as_array().expect("snapshot nodes");
    let header = UiDocumentLeaseHeader {
        generation,
        surface: SurfaceId::try_from(snapshot["surface"].as_str().expect("surface")).expect("surface id"),
        revision: UiRevision(snapshot["revision"].as_u64().expect("revision")),
        root: UiNodeId(snapshot["root"].as_u64().expect("root")),
        layout_epoch: snapshot["layoutEpoch"].as_u64().expect("layout epoch"),
        node_count: nodes.len(),
    };
    let mut document = UiDocumentTree::new(header).expect("corpus header admits");
    for node in nodes {
        document.try_upsert_record(serde_json::from_value(node.clone()).expect("corpus record deserializes")).expect("corpus record admits");
    }
    document
}

fn test_clock() -> Option<u64> {
    Some(0)
}

/// 🎬️ Paints no scene — a corpus surface falls through to `paint`'s own placeholder chrome.
struct NoSceneHost;

impl crate::wgpu::scene_slots::SceneHost for NoSceneHost {
    fn paint_slot_step(&mut self, _slot: &crate::wgpu::scene_slots::SceneSlot<'_>, cursor: &mut crate::wgpu::scene_slots::ScenePaintCursor, _draw: &mut crate::wgpu::draw::DrawList, _atlas: &mut FontAtlas, _icons: Option<&crate::wgpu::draw::IconAtlas>) -> crate::wgpu::scene_slots::ScenePaintStep {
        cursor.finish()
    }
}

/// 🌳️ Publishes, reconciles, lays out and paints one snapshot in its own window; answers the engine and
/// the painted UI instance count.
fn mount(window: &str, snapshot: &serde_json::Value) -> (Ui, usize) {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    assert!(ui.publish_document(window, document(snapshot, 1)), "{window}: the document publishes");
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    let mut reconciled = false;
    for _ in 0..4096 {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(4096, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), test_clock, &mut sequence,&mut actual_retained_progress);
        match ui.step_document_reconcile(window, "conformance", &mut cx) {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => {
                reconciled = true;
                break;
            }
            UiDocumentReconcileStep::Fault(fault) => panic!("{window}: reconcile faulted: {fault:?}"),
        }
    }
    assert!(reconciled, "{window}: reconcile terminates");
    let mut atlas = FontAtlas::builtin();
    ui.set_viewport(window, 960.0, 720.0);
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    let mut preview_sequence = 0;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    let settled = loop {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), test_clock, &mut preview_sequence,&mut actual_retained_progress);
        if matches!(ui.step_layouts(&pool, &mut atlas, &mut cx), UiLayoutStep::Idle) {
            break true;
        }
        if std::time::Instant::now() > deadline {
            break false;
        }
        std::thread::yield_now();
    };
    assert!(settled, "{window}: layout settles");
    let draw = ui.frame::<NoSceneHost>(window, 960.0, 720.0, &mut atlas, None, None).unwrap_or_else(|| panic!("{window}: a mounted document paints"));
    let instances = draw.layers.iter().map(|layer| layer.ui_instances.len() + layer.vector_vertices.len() + layer.raster_instances.len()).sum();
    (ui, instances)
}

fn node_of<'a>(ui: &'a Ui, window: &str, id: u64) -> &'a UiNode {
    let tree = ui.tree(window).expect("window tree");
    let node = tree.document_node(UiNodeId(id)).and_then(|node| tree.node(node)).unwrap_or_else(|| panic!("{window}: record {id} mounted"));
    &node.spec.0
}

/// 🧪️ Every accept case mounts its full record set; every number-control and recipe case also paints.
/// (A bare root `TreeItem` paints only inside its `Tree`, so painting is pinned where a case stands alone.)
#[test]
fn every_accept_case_of_the_corpus_mounts_every_record() {
    let cases = accept_cases();
    assert_eq!(cases.len(), 55, "the shape groups hold every accept case of the corpus");
    for (group, case, snapshot, expectation) in &cases {
        let window = format!("conformance.{case}");
        let (ui, instances) = mount(&window, snapshot);
        let tree = ui.tree(&window).expect("window tree");
        assert!(tree.root.is_some(), "{group}/{case}: a root mounts");
        for shape in expectation["tree"]["shape"].as_array().expect("expected shape") {
            let id = UiNodeId(shape["id"].as_u64().expect("shape id"));
            assert!(tree.document_node(id).is_some(), "{group}/{case}: record {id:?} mounts");
        }
        if ["slider-with-snaps", "stepper-precision", "vector-input", "color-input", "reference-list", "dialog-choices", "tree-row-recipes", "slider", "number-stepper", "dialog", "dial-with-snaps", "log-slider", "display-factor", "stepper-detents", "hard-bound-refusal"].contains(&case.as_str()) {
            assert!(instances > 0, "{group}/{case}: the mounted document paints");
        }
    }
}

/// 🧲️ A slider's detents reach the retained node and paint exactly one tick each.
#[test]
fn slider_with_snaps_paints_one_tick_per_detent() {
    let snapshot = read(&corpus_dir().join("🧩️component/🧲️slider-with-snaps/📸️snapshot.json"));
    let snaps: Vec<f64> = snapshot["nodes"][0]["component"]["snaps"].as_array().expect("snaps").iter().map(|snap| snap.as_f64().expect("snap")).collect();
    let (ui, with_ticks) = mount("conformance.snaps", &snapshot);
    let UiNode::Slider(slider) = node_of(&ui, "conformance.snaps", 0) else { panic!("a slider node") };
    assert_eq!(slider.snaps, snaps);
    let mut bare = snapshot.clone();
    bare["nodes"][0]["component"].as_object_mut().expect("component").remove("snaps");
    let (_, without_ticks) = mount("conformance.bare", &bare);
    assert_eq!(with_ticks - without_ticks, snaps.len(), "one tick instance per detent");
    let rail = crate::wgpu::geometry::Rect::new(10.0, 0.0, 100.0, 4.0);
    let ticks: Vec<f32> = crate::wgpu::slider::slider_tick_rects(rail, 0.0, 10.0, ui_contract::UiNumberScale::Linear, &snaps, 2.0, 8.0).map(|tick| tick.x + tick.w * 0.5).collect();
    assert_eq!(ticks, vec![35.0, 60.0, 85.0]);
    for flow in [ui_contract::FlowInline::Ltr, ui_contract::FlowInline::Rtl] {
        let bounds = crate::wgpu::geometry::Rect::new(0.0, 0.0, 240.0, 24.0);
        let rail = crate::wgpu::layout::slider_presentation(bounds, 0.0, 0.0, 10.0, flow).rail;
        for (tick, snap) in crate::wgpu::slider::slider_tick_rects(rail, 0.0, 10.0, ui_contract::UiNumberScale::Linear, &snaps, 2.0, 8.0).zip(&snaps) {
            let thumb = crate::wgpu::layout::slider_presentation(bounds, *snap, 0.0, 10.0, flow).thumb;
            assert!(((tick.x + tick.w * 0.5) - (thumb.x + thumb.w * 0.5)).abs() < 1e-3, "{flow:?}: the tick of detent {snap} sits under the thumb resting on it");
        }
    }
}

/// 🌲️ A tree row whose content is a recipe, or more than one node, renders that content in its value column the way
/// React's property row does: no placeholder row is made of it, the row grows one line per content leaf and label, and
/// every leaf lays out on its own line inside the row, in document order, where a pointer finds it and Tab reaches it.
#[test]
fn tree_rows_render_recipe_content_in_their_value_column() {
    let (ui, painted) = mount("conformance.tree-row", &read(&corpus_dir().join("🖥️composite/🌲️tree-row-recipes/📸️snapshot.json")));
    assert!(painted > 0, "the tree paints");
    let tree = ui.tree("conformance.tree-row").expect("window tree");
    let UiNode::Tree(spec) = node_of(&ui, "conformance.tree-row", 0) else { panic!("a tree") };
    let rows: Vec<(String, Option<u16>, bool, bool)> = spec.sections[0].items.iter().map(|item| (item.id.clone(), item.content_lines, item.control.is_some(), item.items.is_some())).collect();
    assert_eq!(rows, [("tint.row".to_string(), Some(3), false, false), ("offset.row".to_string(), Some(6), false, false), ("targets.row".to_string(), Some(4), false, false), ("replay.row".to_string(), Some(2), false, false)], "each row's content counts its lines and makes no placeholder row");
    let rect = |id: u64| tree.document_node(UiNodeId(id)).and_then(|node| tree.absolute_rect(node)).unwrap_or_else(|| panic!("record {id} is laid out"));
    for (row, leaves) in [(2, vec![4, 5, 6]), (7, vec![10, 12]), (13, vec![16, 17, 19, 20]), (21, vec![22, 23])] {
        let row_rect = rect(row);
        let mut previous: Option<crate::wgpu::geometry::Rect> = None;
        for leaf in leaves {
            let leaf_rect = rect(leaf);
            assert!(leaf_rect.w > 0.0 && leaf_rect.h > 0.0, "record {leaf} has a box");
            assert!(leaf_rect.y >= row_rect.y - 0.5 && leaf_rect.y + leaf_rect.h <= row_rect.y + row_rect.h + 0.5, "record {leaf} lies inside its row {row}");
            assert!(leaf_rect.x + leaf_rect.w <= row_rect.x + row_rect.w + 0.5 && leaf_rect.x > row_rect.x + row_rect.w * 0.3, "record {leaf} lies in the row's value column");
            if let Some(previous) = previous {
                assert!(leaf_rect.y >= previous.y + previous.h - 0.5, "record {leaf} sits below the line before it");
            }
            previous = Some(leaf_rect);
        }
    }
    for (upper, lower) in [(2, 7), (7, 13), (13, 21)] {
        assert!(rect(lower).y >= rect(upper).y + rect(upper).h - 0.5, "row {lower} starts below row {upper}");
    }
    let root = tree.root.expect("a mounted root");
    for id in [4, 5, 6, 10, 12, 16, 17, 19, 20] {
        let leaf = rect(id);
        let hit = crate::wgpu::events::hit_test(tree, root, leaf.x + leaf.w * 0.5, leaf.y + leaf.h * 0.5).and_then(|node| tree.document_id(node));
        assert_eq!(hit, Some(UiNodeId(id)), "a pointer at record {id} finds it");
    }
    let mut ui = ui;
    let mut reached = Vec::new();
    for _ in 0..16 {
        ui.dispatch_event("conformance.tree-row", crate::wgpu::events::UiEvent::KeyDown { key: "Tab".into(), modifiers: Default::default() });
        let tree = ui.tree("conformance.tree-row").expect("window tree");
        if let Some((focused, _)) = tree.document_bindings().iter().find(|(_, node)| tree.node(*node).is_some_and(|node| node.flags.contains(NodeFlags::FOCUSED))) {
            reached.push(focused.0);
        }
    }
    for id in [5, 6, 10, 12, 16, 17, 19, 20] {
        assert!(reached.contains(&id), "Tab reaches record {id}: {reached:?}");
    }
}

/// 🎯️ Precision rides a stepper and a number field into their retained nodes.
#[test]
fn precision_reaches_the_stepper_and_every_vector_axis() {
    let stepper = read(&corpus_dir().join("🧩️component/🎯️stepper-precision/📸️snapshot.json"));
    let (ui, _) = mount("conformance.stepper", &stepper);
    let UiNode::NumberStepper(node) = node_of(&ui, "conformance.stepper", 0) else { panic!("a stepper node") };
    assert_eq!((node.precision, crate::wgpu::stepper::stepper_value_text(node.value, node.precision, node.display_factor)), (Some(2), "2.50".to_string()));
    let vector = read(&corpus_dir().join("🖥️composite/🧭️vector-input/📸️snapshot.json"));
    let (ui, _) = mount("conformance.vector", &vector);
    for id in [2, 4] {
        let UiNode::Input(input) = node_of(&ui, "conformance.vector", id) else { panic!("a number field") };
        assert_eq!((input.input_kind.as_str(), input.precision, input.step, input.snaps.clone()), ("number", Some(1), Some(0.5), vec![0.0]));
    }
}

/// 🎨️ The `color_input` recipe mounts as a swatch, a hex field and an alpha slider, and the swatch paints the colour its
/// hex parses to through the shared colour law.
#[test]
fn color_input_mounts_a_swatch_hex_field_and_alpha_slider() {
    let (ui, _) = mount("conformance.color", &read(&corpus_dir().join("🖥️composite/🎨️color-input/📸️snapshot.json")));
    let UiNode::Input(swatch) = node_of(&ui, "conformance.color", 1) else { panic!("a swatch field") };
    let UiNode::Input(hex) = node_of(&ui, "conformance.color", 2) else { panic!("a hex field") };
    let UiNode::Slider(alpha) = node_of(&ui, "conformance.color", 3) else { panic!("an alpha slider") };
    assert_eq!((swatch.input_kind.as_str(), swatch.value.as_str(), hex.input_kind.as_str(), hex.value.as_str()), ("color", "#ff8000", "text", "#ff800080"));
    assert_eq!((alpha.min, alpha.max, alpha.step, alpha.value), (0.0, 1.0, 0.01, 0.5));
    let bounds = crate::wgpu::geometry::Rect::new(0.0, 0.0, 120.0, 24.0);
    let (square, color, text) = crate::wgpu::paint::color_input_swatch(bounds, &swatch.value).expect("a colour swatch");
    assert_eq!(color, crate::wgpu::theme::Rgba::from_srgb8(255, 128, 0, 255));
    assert!(square.x >= bounds.x && square.x + square.w <= text.x && text.x + text.w <= bounds.x + bounds.w, "the swatch leads the hex text inside the field");
    assert!(crate::wgpu::paint::color_input_swatch(bounds, "not a colour").is_none());
}

/// 🧷️🔀️ The recipes keep their keyed buttons: reference chips and a dialog's choices mount as buttons.
#[test]
fn reference_chips_and_dialog_choices_mount_as_buttons() {
    for (case, window, ids) in [("🖥️composite/🧷️reference-list/📸️snapshot.json", "conformance.references", vec![2, 3, 5]), ("🖥️composite/🔀️dialog-choices/📸️snapshot.json", "conformance.choices", vec![7, 8, 9])] {
        let (ui, _) = mount(window, &read(&corpus_dir().join(case)));
        for id in ids {
            assert!(matches!(node_of(&ui, window, id), UiNode::Button(_)), "{case}: record {id} is a button");
        }
    }
}

/// ♿️ The number-control, recipe and dialog cases publish the corpus's own accessibility on this target: every record
/// keeps its liveness and visibility, every interactive record is named and described exactly as the corpus declares
/// (a slider and a stepper by their field, a vector axis by its axis label, a reference chip by its remove label, a
/// destructive choice with its consequence), and the number controls announce the roles a reader operates them by.
/// These are the controls the Rust-built History editor composes (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
#[test]
fn the_history_editor_controls_project_their_corpus_accessibility() {
    for (case, window, roles) in [
        ("🧩️component/🧲️slider-with-snaps", "a11y.snaps", vec![(0, "slider")]),
        ("🧩️component/🎯️stepper-precision", "a11y.stepper", vec![(0, "spinbutton")]),
        ("🖥️composite/🧭️vector-input", "a11y.vector", vec![]),
        ("🖥️composite/🎨️color-input", "a11y.color", vec![(3, "slider")]),
        ("🖥️composite/🌲️tree-row-recipes", "a11y.tree-row", vec![(6, "slider"), (16, "button"), (17, "button"), (19, "button"), (23, "progressbar")]),
        ("🖥️composite/🧷️reference-list", "a11y.references", vec![(2, "button"), (3, "button"), (5, "button")]),
        ("🖥️composite/🔀️dialog-choices", "a11y.choices", vec![(7, "button"), (8, "button"), (9, "button")]),
    ] {
        let folder = corpus_dir().join(case);
        let (ui, _) = mount(window, &read(&folder.join("📸️snapshot.json")));
        let expectation = read(&folder.join("🎯️expect.json"));
        let projection = crate::wgpu::accessibility::accessibility_projection(ui.tree(window).expect("window tree"));
        for expected in expectation["accessibility"].as_array().expect("accessibility expectations") {
            let id = expected["id"].as_u64().expect("record id");
            let node = projection.iter().find(|node| node.node_id == id).unwrap_or_else(|| panic!("{case}: record {id} is projected"));
            assert_eq!(node.live, expected["live"].as_str().expect("live"), "{case}/{id}");
            assert_eq!(node.hidden, expected["hidden"].as_bool().expect("hidden"), "{case}/{id}");
            if node.focusable || node.actionable {
                assert_eq!(node.label.as_deref(), expected["label"].as_str(), "{case}/{id}: an interactive record is named as the corpus declares");
                assert!(node.label.as_deref().is_some_and(|label| !label.is_empty()), "{case}/{id}: no interactive record is nameless");
                assert_eq!(node.description.as_deref(), expected["description"].as_str(), "{case}/{id}");
            }
        }
        for (id, role) in roles {
            assert_eq!(projection.iter().find(|node| node.node_id == id).map(|node| node.role.as_str()), Some(role), "{case}/{id}");
        }
    }
}

/// ⚖️ LAW (React's host presence overlay `notes`): a peer's note on a record is announced after that tree row's own
/// description (joined by ` · `) or as its description when it has none, on every window of the engine, without touching
/// the document; an unchanged table changes nothing and an empty one restores the rows.
#[test]
fn peer_notes_are_announced_after_a_tree_rows_description() {
    let window = "a11y.tree-row";
    let (mut ui, _) = mount(window, &read(&corpus_dir().join("🖥️composite/🌲️tree-row-recipes/📸️snapshot.json")));
    let described = |ui: &Ui, key: &str| crate::wgpu::accessibility::accessibility_projection(ui.tree(window).expect("window tree")).into_iter().find(|node| node.key == key).unwrap_or_else(|| panic!("{key} is projected")).description;
    let before = [("offset.row", described(&ui, "offset.row")), ("tint.row", described(&ui, "tint.row"))];
    let notes = vec![("offset.row".to_string(), "Ada is editing this in time travel".to_string()), ("tint.row".to_string(), "Ada is editing this in time travel · Bo is editing this in time travel".to_string())];
    assert!(ui.set_presence_notes(&notes), "the notes reach the window");
    assert!(!ui.set_presence_notes(&notes), "an unchanged table changes nothing");
    for (key, description) in &before {
        let note = &notes.iter().find(|(noted, _)| noted == key).expect("noted").1;
        let expected = description.as_ref().map_or_else(|| note.clone(), |description| format!("{description} · {note}"));
        assert_eq!(described(&ui, key).as_deref(), Some(expected.as_str()), "{key}");
        assert_eq!(ui.tree(window).expect("window tree").presence_note(key), Some(note.as_str()));
    }
    assert_eq!(ui.tree(window).expect("window tree").presence_note("targets.row"), None);
    assert!(ui.set_presence_notes(&[]));
    for (key, description) in before {
        assert_eq!(described(&ui, key), description, "{key}: an empty table restores the row");
    }
}

/// 🧭️ LAW (G6): the dial, log, display-factor, detent and hard-bound cases carry every facet into their retained nodes and
/// paint it — a degree dial with one radial tick per detent, its needle and pointer angle snapping onto 90°, a log slider's
/// ticks at their log positions, a radian stepper reading `45 °`, page keys stopping on detents, and typed values beyond a
/// hard bound refused with the producer's message naming it.
#[test]
fn the_g6_number_control_cases_carry_and_paint_every_facet() {
    let dial = read(&corpus_dir().join("🧩️component/🧭️dial-with-snaps/📸️snapshot.json"));
    let (ui, with_ticks) = mount("conformance.dial", &dial);
    let UiNode::Slider(angle) = node_of(&ui, "conformance.dial", 0) else { panic!("a dial node") };
    assert_eq!((angle.appearance, angle.unit.as_deref(), angle.display_unit.as_deref(), angle.snaps.len()), (ui_contract::SliderAppearance::Dial, Some("rad"), Some("°"), 5));
    assert_eq!((angle.readout(angle.value), angle.unit_label()), ("90".to_string(), Some("90 °".to_string())));
    assert_eq!(angle.limits, Some(ui_contract::UiNumberLimits::default()), "a dial without hard bounds has open limits");
    let mut bare = dial.clone();
    bare["nodes"][0]["component"].as_object_mut().expect("component").remove("snaps");
    let (_, without_ticks) = mount("conformance.dial.bare", &bare);
    assert!(with_ticks > without_ticks, "the dial paints its detent ticks");
    let track = crate::wgpu::geometry::Rect::new(0.0, 0.0, 200.0, 40.0);
    let face = crate::wgpu::slider::dial_face(track);
    let ticks: Vec<[f32; 4]> = crate::wgpu::slider::dial_tick_lines(face, angle).collect();
    assert_eq!(ticks.len(), 5, "one radial tick per detent");
    assert!((ticks[3][0] - face.cx).abs() < 1e-3 && ticks[3][1] < face.cy && ticks[3][3] < ticks[3][1], "the 90° tick points up: {:?}", ticks[3]);
    assert!(ticks[2][0] > face.cx && (ticks[2][1] - face.cy).abs() < 1e-3, "the 0° tick points to three o'clock: {:?}", ticks[2]);
    let towards = |degrees: f64| crate::wgpu::slider::dial_point(face, degrees.to_radians(), 0.9);
    let (x, y) = towards(88.0);
    assert_eq!(crate::wgpu::slider::slider_node_pointer_value(angle, track, x, y), std::f64::consts::FRAC_PI_2, "a pointer at 88° lands on the 90° detent");
    let (x, y) = towards(-120.0);
    let pointed = crate::wgpu::slider::slider_node_pointer_value(angle, track, x, y);
    assert!((pointed.to_degrees() + 120.0).abs() < 1e-6, "a pointer at -120° reads -120°: {}", pointed.to_degrees());
    let arrow = ui_contract::slider_key_value(0.0, angle.min, angle.max, angle.step, angle.precision, angle.display_factor, angle.snaps.iter().copied(), ui_contract::SliderKey::Increment, false);
    assert_eq!(ui_contract::ui_number_display_text(arrow, angle.display_factor, None), "1", "an arrow steps one degree");
    let page = ui_contract::slider_key_value(0.0, angle.min, angle.max, angle.step, angle.precision, angle.display_factor, angle.snaps.iter().copied(), ui_contract::SliderKey::PageUp, false);
    assert_eq!(page, std::f64::consts::FRAC_PI_2, "a page key jumps to the next detent");

    let log = read(&corpus_dir().join("🧩️component/📈️log-slider/📸️snapshot.json"));
    let (ui, _) = mount("conformance.log", &log);
    let UiNode::Slider(factor) = node_of(&ui, "conformance.log", 0) else { panic!("a log slider node") };
    assert_eq!((factor.scale, factor.precision, factor.readout(factor.value)), (ui_contract::UiNumberScale::Log, Some(2), "1.00".to_string()));
    let rail = crate::wgpu::geometry::Rect::new(0.0, 0.0, 100.0, 4.0);
    let centres: Vec<f64> = crate::wgpu::slider::slider_tick_rects(rail, factor.min, factor.max, factor.scale, &factor.snaps, 2.0, 8.0).map(|tick| f64::from(tick.x + tick.w * 0.5)).collect();
    for (centre, snap) in centres.iter().zip(&factor.snaps) {
        assert!((centre - (snap / 0.1).ln() / 100f64.ln() * 100.0).abs() < 1e-3, "the {snap} tick sits at its log position: {centre}");
    }
    assert!((centres[2] - 50.0).abs() < 1e-3, "the unit factor ticks the middle of the log axis");
    assert_eq!(crate::wgpu::events::typed_number("-5", factor.display_factor, factor.precision, [factor.value], Some(factor.min), Some(factor.max), factor.limits.as_ref()), Err("Must be greater than 0".to_string()));
    assert_eq!(crate::wgpu::events::typed_number("20", factor.display_factor, factor.precision, [factor.value], Some(factor.min), Some(factor.max), factor.limits.as_ref()), Ok(20.0), "a soft travel admits a typed value beyond it");

    let heading = read(&corpus_dir().join("🧩️component/🔁️display-factor/📸️snapshot.json"));
    let (ui, _) = mount("conformance.heading", &heading);
    let UiNode::NumberStepper(stepper) = node_of(&ui, "conformance.heading", 0) else { panic!("a stepper node") };
    assert_eq!((stepper.value_text(stepper.value), stepper.unit.as_deref()), ("45 °".to_string(), Some("°")));
    assert_eq!(crate::wgpu::events::typed_number("90", stepper.display_factor, stepper.precision, std::iter::once(stepper.value).chain(stepper.snaps.iter().copied()), stepper.min, stepper.max, stepper.limits.as_ref()), Ok(std::f64::consts::FRAC_PI_2), "a typed 90 keeps the exact π/2 detent");

    let gap = read(&corpus_dir().join("🧩️component/📍️stepper-detents/📸️snapshot.json"));
    let (ui, _) = mount("conformance.gap", &gap);
    let UiNode::NumberStepper(stepper) = node_of(&ui, "conformance.gap", 0) else { panic!("a stepper node") };
    assert_eq!((stepper.snaps.clone(), stepper.value_text(stepper.value)), (vec![0.0, 5.0, 10.0], "2.0 mm".to_string()));
    assert_eq!(ui_contract::ui_number_key_value(stepper.value, stepper.min, stepper.max, stepper.step, stepper.precision, stepper.display_factor, stepper.snaps.iter().copied(), ui_contract::SliderKey::PageUp, false), 5.0);
    assert_eq!(crate::wgpu::events::typed_number("12", None, stepper.precision, [stepper.value], stepper.min, stepper.max, stepper.limits.as_ref()), Err("Must be at most 10 mm".to_string()));

    let width = read(&corpus_dir().join("🧩️component/⛔️hard-bound-refusal/📸️snapshot.json"));
    let (ui, _) = mount("conformance.width", &width);
    let UiNode::Input(field) = node_of(&ui, "conformance.width", 0) else { panic!("a number field") };
    let limits = field.limits.as_ref().expect("the field carries its limits");
    for (typed, verdict) in [("-1", Err("Must be greater than 0".to_string())), ("0", Err("Must be greater than 0".to_string())), ("120", Err("Must be at most 100".to_string())), ("4.5", Ok(4.5)), ("x", Err(String::new()))] {
        assert_eq!(crate::wgpu::events::typed_number(typed, field.display_factor, field.precision, [4.0], field.min, field.max, Some(limits)), verdict, "{typed}");
    }
}

/// 💬️ LAW (coordinator decision on tree rows, shared with React's Interpreter law over the same case): in
/// `🖥️composite/💬️row-semantics` a property row's description is the description of the control it holds, and a disabled row
/// action stays focusable and tabbable, is not actionable, and names its reason as its description — its dispatch refused by
/// the contract twin.
#[test]
fn row_descriptions_reach_their_controls_and_disabled_row_actions_stay_focusable_with_their_reason() {
    let folder = corpus_dir().join("🖥️composite/💬️row-semantics");
    let window = "a11y.row-semantics";
    let snapshot = read(&folder.join("📸️snapshot.json"));
    let (ui, _) = mount(window, &snapshot);
    let semantics = read(&folder.join("🎯️expect.json"))["rowSemantics"].clone();
    let projection = crate::wgpu::accessibility::accessibility_projection(ui.tree(window).expect("window tree"));
    let projected = |id: u64| projection.iter().find(|node| node.node_id == id).unwrap_or_else(|| panic!("record {id} is projected: {projection:?}"));
    for described in semantics["describedControls"].as_array().expect("described controls") {
        let control = projected(described["control"].as_u64().expect("control id"));
        assert!(control.focusable, "the control is reachable: {control:?}");
        assert_eq!(control.description.as_deref(), described["description"].as_str(), "the row's description names its control");
    }
    for disabled in semantics["disabledRowActions"].as_array().expect("disabled row actions") {
        let row = projected(disabled["row"].as_u64().expect("row id"));
        let key = format!("{}{}{}", row.key, crate::wgpu::accessibility::ROW_ACTION_KEY_INFIX, disabled["index"].as_u64().expect("action index"));
        let action = projection.iter().find(|node| node.key == key).unwrap_or_else(|| panic!("{key} is projected"));
        assert_eq!((action.label.as_deref(), action.disabled, action.focusable, action.tabbable, action.actionable, action.description.as_deref()), (disabled["label"].as_str(), true, true, true, false, disabled["reason"].as_str()), "{key}");
        let record = snapshot["nodes"].as_array().expect("nodes").iter().find(|node| node["id"] == disabled["row"]).expect("row record");
        let target: ui_contract::RowTarget = serde_json::from_value(record["component"]["target"].clone()).expect("row target");
        let actions: Vec<ui_contract::RowAction> = serde_json::from_value(record["component"]["rowActions"].clone()).expect("row actions");
        assert!(target.action_binding(&actions[disabled["index"].as_u64().expect("action index") as usize]).is_err(), "{key}: a disabled action never dispatches");
    }
}

/// 💬️ LAW (audit W1E-1, `💬️row-semantics` `revealReason`, the wgpu half of React's `[data-slot="row-action-reason"][data-revealed]`):
/// a disabled row action's reason stays its description and is SHOWN as a hint anchored to its icon after each `on` trigger — the
/// pointer entering the icon, keyboard focus on the action, a press that dispatches nothing — and hidden after each `off` trigger:
/// the pointer leaving the icon, blur, Escape.
#[test]
fn a_disabled_row_action_reveals_its_reason_on_hover_focus_and_press() {
    use crate::wgpu::events::{AccessibilityUiEvent, UiEvent};
    let folder = corpus_dir().join("🖥️composite/💬️row-semantics");
    let window = "a11y.row-reason";
    let (mut ui, _) = mount(window, &read(&folder.join("📸️snapshot.json")));
    let semantics = read(&folder.join("🎯️expect.json"))["rowSemantics"].clone();
    let generation = ui.surface_generation(window).expect("published surface generation");
    for disabled in semantics["disabledRowActions"].as_array().expect("disabled row actions") {
        let (row, index, reason) = (disabled["row"].as_u64().expect("row id"), disabled["index"].as_u64().expect("action index") as usize, disabled["reason"].as_str().expect("reason"));
        let triggers = |side: &str| disabled["revealReason"][side].as_array().expect("revealReason").iter().map(|trigger| trigger.as_str().expect("trigger").to_string()).collect::<Vec<_>>();
        assert_eq!((triggers("on"), triggers("off")), (vec!["hover".to_string(), "focus".into(), "press".into()], vec!["leave".to_string(), "blur".into(), "escape".into()]), "the corpus names the triggers this law drives");
        let projection = crate::wgpu::accessibility::accessibility_projection(ui.tree(window).expect("window tree"));
        let row_key = projection.iter().find(|node| node.node_id == row).expect("row projected").key.clone();
        let key = format!("{row_key}{}{index}", crate::wgpu::accessibility::ROW_ACTION_KEY_INFIX);
        let shown = |ui: &Ui| ui.revealed_row_reason(window);
        let expected = Some((ui_contract::UiNodeId(row), index, reason.to_string()));
        let icon = ui.row_action_icon_rect(window, ui_contract::UiNodeId(row), index).expect("the action paints an icon");
        let (x, y) = (icon.x + icon.w * 0.5, icon.y + icon.h * 0.5);
        assert_eq!(shown(&ui), None, "no hint before any trigger");

        let _ = ui.dispatch_pointer_event(window, 1, UiEvent::PointerMove { x, y, modifiers: Default::default() });
        assert_eq!(shown(&ui), expected, "hover: the pointer on the icon shows the reason");
        let _ = ui.dispatch_pointer_event(window, 1, UiEvent::PointerMove { x: icon.x - 200.0, y, modifiers: Default::default() });
        assert_eq!(shown(&ui), None, "leave: the pointer off the icon hides it");

        let focused = ui.dispatch_accessibility_event(window, generation, row, &key, AccessibilityUiEvent::Focus).expect("focus admitted");
        assert!(!focused.iter().any(|command| matches!(command, crate::wgpu::events::UiCommand::App { .. })) && shown(&ui) == expected, "focus: keyboard focus on the action shows it at once");
        let _ = ui.dispatch_accessibility_event(window, generation, row, &key, AccessibilityUiEvent::Blur).expect("blur admitted");
        assert_eq!(shown(&ui), None, "blur: focus leaving hides it");

        let pressed = ui.dispatch_accessibility_event(window, generation, row, &key, AccessibilityUiEvent::Activate).expect("activation admitted");
        assert!(!pressed.iter().any(|command| matches!(command, crate::wgpu::events::UiCommand::App { .. })), "press: a disabled action never dispatches");
        assert_eq!(shown(&ui), expected, "press: activating it shows the reason");
        let _ = ui.dispatch_event(window, UiEvent::KeyDown { key: "Escape".into(), modifiers: Default::default() });
        assert_eq!(shown(&ui), None, "escape: hides it");

        let _ = ui.dispatch_pointer_event(window, 1, UiEvent::PointerDown { x, y, button: crate::wgpu::events::PointerButton::Primary, modifiers: Default::default() });
        let released = ui.dispatch_pointer_event(window, 1, UiEvent::PointerUp { x, y, button: crate::wgpu::events::PointerButton::Primary, modifiers: Default::default() });
        assert!(!released.iter().any(|command| matches!(command, crate::wgpu::events::UiCommand::App { .. })) && shown(&ui) == expected, "press: a click on the icon shows the reason and dispatches nothing");
        let projection = crate::wgpu::accessibility::accessibility_projection(ui.tree(window).expect("window tree"));
        assert_eq!(projection.iter().find(|node| node.key == key).and_then(|node| node.description.clone()).as_deref(), Some(reason), "the reason stays the action's description throughout");
    }
}

/// 🔘️ LAW (audit W1E-2, `💬️row-semantics` `selectedRows`, wgpu half of React's `aria-selected`): an option row of a single-choice
/// list announces its choice as the row's selected state in the ARIA mirror, and the chosen row paints the selected fill (its
/// retained item is selected), never by its icon alone; the unchosen option announces `selected: false` and paints unselected.
#[test]
fn option_rows_announce_and_paint_their_selected_state() {
    let folder = corpus_dir().join("🖥️composite/💬️row-semantics");
    let window = "a11y.row-selected";
    let (ui, _) = mount(window, &read(&folder.join("📸️snapshot.json")));
    let rows = read(&folder.join("🎯️expect.json"))["rowSemantics"]["selectedRows"].as_array().expect("selected rows").clone();
    assert!(rows.iter().any(|row| row["selected"] == true) && rows.iter().any(|row| row["selected"] == false), "the corpus names a chosen and an unchosen option");
    let tree = ui.tree(window).expect("window tree");
    let projection = crate::wgpu::accessibility::accessibility_projection(tree);
    for row in &rows {
        let (id, selected) = (row["row"].as_u64().expect("row id"), row["selected"].as_bool().expect("selected"));
        let node = projection.iter().find(|node| node.node_id == id).unwrap_or_else(|| panic!("option row {id} is projected"));
        assert_eq!((node.role.as_str(), node.selected), ("treeitem", Some(selected)), "option row {id} announces its choice");
        let item = tree.document_node(ui_contract::UiNodeId(id)).and_then(|node| tree.authored_tree_item(node)).unwrap_or_else(|| panic!("option row {id} mounts a retained item"));
        assert_eq!(item.presence.selected, selected, "option row {id} paints the selected fill exactly when chosen");
    }
}
