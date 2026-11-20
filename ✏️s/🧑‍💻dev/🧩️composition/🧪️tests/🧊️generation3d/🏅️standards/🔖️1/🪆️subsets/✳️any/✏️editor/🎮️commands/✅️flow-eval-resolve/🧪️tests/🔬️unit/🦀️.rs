//! 🧠️ Original evaluation replies seed the retained cache and settle only their addressed hop.
use super::*;
use crate::editor_domain::editor_laws::context::retire_flow_eval_session;
use crate::preview_reply::{evaluation_payload, evaluation_response, receive_evaluation};
use semio_framework_artifact_flow_flow::neural::ColdRetire;

#[test]
fn eval_result_seeds_the_node_cache_and_owes_the_run_one_hop() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut session = FlowEvalSession::new();
    let cache = session.neural_cache();
    let node_hash = 0x5eed_c0de_u64;
    let window = ("procedural-preview-test", semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let output = serde_json::json!({"geometry":"brep:solid-1","count":3});
    assert!(!cache.contains(node_hash));
    for id in [window.0, "unrelated-preview"] {
        assert!(session.arm_window_tick(id, crate::preview_reply::TURN).expect("window tick admission").0);
        session.begin_window_tick(id, crate::preview_reply::TURN).map(drop).expect("original window latch turn");
        session.note_window_tick_outcome(id, semio_s_artifact_procedural_3d_preview_eval::tick_is_unfinished(false, 1), crate::preview_reply::TURN).map(drop).expect("original window latch turn");
        session.note_window_extensions_in_flight(id, 1, crate::preview_reply::TURN).map(drop).expect("original window latch turn");
    }
    let payload = evaluation_payload(window, node_hash, "flow-extension-brep",
        evaluation_response(true, "complete", (3, 3), output.clone()));
    receive_evaluation(&mut session, payload).expect("original evaluation reply folds");
    let cached = cache.get(node_hash).expect("seeded Dictionary");
    let observed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&cached)).unwrap();
    assert_eq!(observed, output);
    cached.retire_cold();
    drop(cache);
    assert_eq!((session.window_extensions_in_flight(window.0), session.window_tick_owed(window.0)), (0, true));
    assert_eq!(session.window_extensions_in_flight("unrelated-preview"), 1);
    retire_flow_eval_session(session);
}
