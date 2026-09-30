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
    let mut ui = Ui::new();
    assert!(ui.publish_document(window, document(snapshot, 1)), "{window}: the document publishes");
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    let mut reconciled = false;
    for _ in 0..4096 {
        let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(4096, u64::MAX), cancel.clone(), test_clock, &mut sequence);
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
    let mut settled = false;
    for _ in 0..200_000 {
        let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
        if matches!(ui.step_layouts(&pool, &mut atlas, &mut cx), UiLayoutStep::Idle) {
            settled = true;
            break;
        }
    }
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
    assert_eq!(cases.len(), 48, "the shape groups hold every accept case of the corpus");
    for (group, case, snapshot, expectation) in &cases {
        let window = format!("conformance.{case}");
        let (ui, instances) = mount(&window, snapshot);
        let tree = ui.tree(&window).expect("window tree");
        assert!(tree.root.is_some(), "{group}/{case}: a root mounts");
        for shape in expectation["tree"]["shape"].as_array().expect("expected shape") {
            let id = UiNodeId(shape["id"].as_u64().expect("shape id"));
            assert!(tree.document_node(id).is_some(), "{group}/{case}: record {id:?} mounts");
        }
        if ["slider-with-snaps", "stepper-precision", "vector-input", "color-input", "reference-list", "dialog-choices", "slider", "number-stepper", "dialog"].contains(&case.as_str()) {
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
    let ticks: Vec<f32> = crate::wgpu::slider::slider_tick_rects(rail, 0.0, 10.0, &snaps, 2.0, 8.0).map(|tick| tick.x + tick.w * 0.5).collect();
    assert_eq!(ticks, vec![35.0, 60.0, 85.0]);
    for flow in [ui_contract::FlowInline::Ltr, ui_contract::FlowInline::Rtl] {
        let bounds = crate::wgpu::geometry::Rect::new(0.0, 0.0, 240.0, 24.0);
        let rail = crate::wgpu::layout::slider_presentation(bounds, 0.0, 0.0, 10.0, flow).rail;
        for (tick, snap) in crate::wgpu::slider::slider_tick_rects(rail, 0.0, 10.0, &snaps, 2.0, 8.0).zip(&snaps) {
            let thumb = crate::wgpu::layout::slider_presentation(bounds, *snap, 0.0, 10.0, flow).thumb;
            assert!(((tick.x + tick.w * 0.5) - (thumb.x + thumb.w * 0.5)).abs() < 1e-3, "{flow:?}: the tick of detent {snap} sits under the thumb resting on it");
        }
    }
}

/// 🎯️ Precision rides a stepper and a number field into their retained nodes.
#[test]
fn precision_reaches_the_stepper_and_every_vector_axis() {
    let stepper = read(&corpus_dir().join("🧩️component/🎯️stepper-precision/📸️snapshot.json"));
    let (ui, _) = mount("conformance.stepper", &stepper);
    let UiNode::NumberStepper(node) = node_of(&ui, "conformance.stepper", 0) else { panic!("a stepper node") };
    assert_eq!((node.precision, crate::wgpu::stepper::stepper_value_text(node.value, node.precision)), (Some(2), "2.50".to_string()));
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
