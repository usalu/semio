//! 🧪️ Actual-grant session byte retirement, final cache ownership, and strict lifecycle guards.

use super::*;
use crate::dag::{DagCamera, DagHostRetirement, IoPortSpec};

//#region 🧪️SessionRetirement
fn close(mut session: FlowEvalSession, maximum_bytes: usize) -> usize {
    use semio_framework_job::InteractiveJobCloseStep as Step;
    session.begin_close();
    let mut released = 0;
    assert!(matches!(session.close_step(0, maximum_bytes), Step::Blocked));
    assert!(matches!(session.close_step(1, 0), Step::Blocked));
    for _ in 0..1_000_000 {
        match session.close_step(1, maximum_bytes) {
            Step::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= maximum_bytes);
                released += released_bytes;
            }
            Step::Complete => {
                assert!(session.terminal_is_empty());
                return released;
            }
            Step::Blocked => panic!("positive session grant blocked"),
        }
    }
    panic!("session retirement did not reach terminal-empty")
}

#[test]
fn session_semantic_bytes_larger_than_production_grant_retire_exactly_across_workers() {
    let fixture = crate::os_pack::json::parse(include_str!("../../🔣️.json")).unwrap();
    for maximum_bytes in [1, 64, 4096] {
        let text = fixture.get("text").and_then(|v| v.get("text")).and_then(crate::os_pack::json::Value::as_str).unwrap().repeat(fixture.get("text").and_then(|v| v.get("repeat")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
        let preview =
            fixture.get("preview").and_then(|v| v.get("text")).and_then(crate::os_pack::json::Value::as_str).unwrap().repeat(fixture.get("preview").and_then(|v| v.get("repeat")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
        let mut session = FlowEvalSession::new();
        session.eval_json = String::with_capacity(fixture.get("text").and_then(|v| v.get("reservedCapacity")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
        session.eval_json.push_str(&text);
        assert!(session.eval_json.capacity() > session.eval_json.len());
        session.preview_mesh_json_by_handle.insert("mesh".into(), preview.clone());
        session.pending_tessellate_by_hash.insert(1, "pending".into());
        session.live_geometry_handles.insert("geometry".into());
        session.previous_channels = Some(EvalChannels { outputs: BTreeMap::from([("output".into(), Dictionary::new().insert("label", NeuralValue::Atom(Atom::String(preview))))]), inputs: BTreeMap::new() });
        session.neural_cache().seed(1, Dictionary::new().insert("label", NeuralValue::Atom(Atom::String(text))));
        let released = std::thread::spawn(move || close(session, maximum_bytes)).join().unwrap();
        assert_eq!(released, fixture.get("expected").and_then(|v| v.get("releasedBytes")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
    }
}

#[test]
fn empty_reserved_text_does_not_require_capacity_sized_credit() {
    let mut session = FlowEvalSession::new();
    session.eval_json = String::with_capacity(65536);
    assert_eq!(close(session, 1), 2);
}

#[test]
fn live_session_drop_is_rejected_without_recursive_payload_destruction() {
    assert!(std::panic::catch_unwind(|| drop(FlowEvalSession::new())).is_err());
}

#[test]
fn host_retirement_reports_no_credit_and_retained_fault_without_false_pending() {
    let mut retirement = FlowHostRetirement::new(FlowHost::default());
    assert_eq!(retirement.close_page(0, 4096), Err(FlowHostRetirementFault::NoCredit));
    assert_eq!(retirement.close_page(1, 0), Err(FlowHostRetirementFault::NoCredit));
    retirement.state.faulted = true;
    assert_eq!(retirement.close_page(1, 4096), Err(FlowHostRetirementFault::Failed));
    assert!(!retirement.terminal_nonopaque_is_empty());
    retirement.state.faulted = false;
    for _ in 0..4096 {
        if retirement.close_page(1, 4096).unwrap() {
            break;
        }
    }
    assert!(retirement.terminal_nonopaque_is_empty());
}

fn dag_retirement_fixture() -> (DagHostRetirement, usize) {
    let fixture = crate::os_pack::json::parse(include_str!("../../🔣️.json")).unwrap();
    let dag_fixture = fixture.get("dag").unwrap();
    let repeat = dag_fixture.get("repeat").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize;
    let text = dag_fixture.get("text").and_then(crate::os_pack::json::Value::as_str).unwrap().repeat(repeat);
    let minimum_bytes = dag_fixture.get("minimumUtf8Bytes").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize;
    let node = DagNodeSpec {
        id: dag_fixture.get("nodeId").and_then(crate::os_pack::json::Value::as_str).unwrap().into(),
        name: dag_fixture.get("nodeName").and_then(crate::os_pack::json::Value::as_str).unwrap().into(),
        abbreviation: "RN".into(),
        icon: "note".into(),
        x: 0.0,
        y: 0.0,
        width: 320.0,
        height: 180.0,
        operator_kind: None,
        properties: PropertyBag::new(),
        kind: DagNodeKind::Note { text, output: IoPortSpec::simple("out", "note") },
    };
    let host = DagHost::from_fixture_without_layout(DagFixture {
        schema: dag_fixture.get("schemaText").and_then(crate::os_pack::json::Value::as_str).unwrap().into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![node],
        edges: Vec::new(),
    });
    (DagHostRetirement::new(host), minimum_bytes)
}

fn close_dag(mut retirement: dag::DagHostRetirement, maximum_bytes: usize) -> (usize, usize, usize, bool) {
    assert_eq!(retirement.close_step(0, maximum_bytes), dag::DagRetirementStep::Blocked);
    assert_eq!(retirement.close_step(1, 0), dag::DagRetirementStep::Blocked);
    let mut credited_total = 0usize;
    let mut released_total = 0usize;
    let mut outstanding_credit = 0usize;
    let mut turns = 0usize;
    for _ in 0..1_000_000 {
        turns += 1;
        match retirement.close_step(1, maximum_bytes) {
            dag::DagRetirementStep::Blocked => return (credited_total, released_total, turns, false),
            dag::DagRetirementStep::Pending { released_items, credited_bytes, released_bytes } => {
                assert!(released_items <= 1);
                assert!(credited_bytes <= maximum_bytes);
                credited_total += credited_bytes;
                outstanding_credit += credited_bytes;
                assert!(released_bytes <= outstanding_credit, "physical backing release remains covered by retained credit");
                outstanding_credit -= released_bytes;
                released_total += released_bytes;
            }
            dag::DagRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                assert_eq!(outstanding_credit, 0);
                return (credited_total, released_total, turns, true);
            }
        }
    }
    panic!("DAG retirement did not reach terminal-empty")
}

#[test]
fn session_close_dag_host_retirement_preserves_exact_owner_and_byte_grants() {
    let mut reference = None;
    for maximum_bytes in [1, 64, 4096] {
        let (retirement, minimum_bytes) = dag_retirement_fixture();
        let (credited, released, _, complete) = std::thread::spawn(move || close_dag(retirement, maximum_bytes)).join().unwrap();
        assert!(complete);
        assert!(released >= minimum_bytes);
        assert_eq!(credited, released);
        if let Some(reference) = reference {
            assert_eq!(released, reference);
        } else {
            reference = Some(released);
        }
    }
}

#[test]
fn session_close_dag_host_nonterminal_drop_refuses_recursive_release() {
    let (retirement, _) = dag_retirement_fixture();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(retirement))).is_err());
}

#[test]
fn session_close_vector_scene_retirement_retains_and_reuses_exact_slot() {
    use crate::infinite::canvas::{advance_opaque_scene_retirement, append_svg_document, publish_opaque_scene_retirement, reserve_opaque_scene_retirement, Affine, BezPath, Color, FillRule, OpaqueSceneRetirementStep, Scene, SvgDocument};

    let fixture = crate::os_pack::json::parse(include_str!("../../🔣️.json")).unwrap();
    let capacity = fixture.get("scene").and_then(|value| value.get("retirementCapacity")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize;
    let retained_commands = fixture.get("scene").and_then(|value| value.get("retainedCommands")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize;
    let retained_path_elements = fixture.get("scene").and_then(|value| value.get("retainedPathElements")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize;
    let retained_vello_rects = fixture.get("scene").and_then(|value| value.get("retainedVelloRects")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize;
    for index in 0..=capacity {
        let token = reserve_opaque_scene_retirement().expect("terminal vector scene retirement slot is reusable");
        let mut scene = Scene::new();
        for _ in 0..if index == 1 { retained_commands } else { 1 } {
            scene.pop_layer();
        }
        if index == 0 {
            let mut svg = String::from("<svg xmlns='http://www.w3.org/2000/svg' width='256' height='256'>");
            for rect in 0..retained_vello_rects {
                svg.push_str(&format!("<rect x='{}' y='{}' width='1' height='1' fill='black'/>", rect % 256, rect / 256));
            }
            svg.push_str("</svg>");
            let document = SvgDocument::parse_icons(&svg).expect("retained Vello fragment SVG remains canonical");
            append_svg_document(&mut scene, &document);
        }
        if index == 1 {
            let mut path = BezPath::new();
            path.move_to((0.0, 0.0));
            for point in 0..retained_path_elements {
                path.line_to((point as f64, point as f64));
            }
            scene.fill(FillRule::NonZero, Affine::IDENTITY, Color::from_rgba8(0, 0, 0, 255), None, &path);
        }
        publish_opaque_scene_retirement(token, scene);
        assert_eq!(advance_opaque_scene_retirement(token, 0, usize::MAX), OpaqueSceneRetirementStep::Blocked);
        let mut credited_total = 0usize;
        let mut released_total = 0usize;
        let mut outstanding_credit = 0usize;
        loop {
            match advance_opaque_scene_retirement(token, 1, 4096) {
                OpaqueSceneRetirementStep::Blocked | OpaqueSceneRetirementStep::Fault => panic!("positive scene credit lost the exact cursor"),
                OpaqueSceneRetirementStep::Pending { released_items, credited_bytes, released_bytes } => {
                    assert!(released_items <= 1 && credited_bytes <= 4096);
                    credited_total += credited_bytes;
                    outstanding_credit += credited_bytes;
                    assert!(released_bytes <= outstanding_credit);
                    outstanding_credit -= released_bytes;
                    released_total += released_bytes;
                }
                OpaqueSceneRetirementStep::Complete { released_items, credited_bytes, released_bytes } => {
                    assert_eq!(released_items, 1);
                    assert!(credited_bytes <= 4096);
                    credited_total += credited_bytes;
                    outstanding_credit += credited_bytes;
                    assert!(released_bytes <= outstanding_credit);
                    outstanding_credit -= released_bytes;
                    released_total += released_bytes;
                    break;
                }
            }
        }
        assert_eq!(outstanding_credit, 0);
        assert_eq!(credited_total, released_total);
        assert!(released_total > 0);
        if index <= 1 {
            assert!(released_total > 4096, "large scene or Vello backing crosses multiple retained credits before actual terminal release");
        }
    }
}
//#endregion 🧪️SessionRetirement
