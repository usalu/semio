//! 🔺️ Editor preview completion laws over retained reply and ledger custody.
use crate::preview_reply::{cancel, receive, response, retain};
use semio_os_flow::{FlowEvalSession, preview_tessellate_node_hash};
use semio_plugin::MeshData;

const WINDOW: (&str, &str) = ("procedural-preview-test", "preview");
const HANDLE: &str = "brep:solid-1";
const COARSE: f64 = 0.05;
const FINE: f64 = 0.01;

fn triangle(tolerance: f64) -> MeshData {
    let fixture = crate::preview_reply::lod_fixture();
    assert_eq!(fixture["handle"], HANDLE);
    let row = fixture["lods"].as_array().unwrap().iter().find(|row| row["tolerance"].as_f64() == Some(tolerance)).unwrap();
    MeshData {
        positions: serde_json::from_value(row["positions"].clone()).unwrap(),
        indices: serde_json::from_value(row["indices"].clone()).unwrap(),
        ..Default::default()
    }
}

fn admit(session: &mut FlowEvalSession, tolerance: f64) -> u64 {
    let hash = preview_tessellate_node_hash(HANDLE, tolerance.to_bits());
    assert!(session.note_pending_tessellate(hash, HANDLE.into(), tolerance));
    hash
}

fn complete(session: &mut FlowEvalSession, tolerance: f64, mesh: &MeshData) {
    receive(session, response(WINDOW, HANDLE, tolerance, "complete", (3, 3), Some(mesh))).expect("canonical mesh folds");
}

#[test]
fn tessellate_result_resolves_the_pending_handle() {
    let mut session = FlowEvalSession::new();
    let mesh = triangle(COARSE);
    let hash = admit(&mut session, FINE);
    assert!(session.preview_mesh(HANDLE).is_none());
    complete(&mut session, FINE, &mesh);
    assert_eq!(session.preview_mesh_for_tolerance(HANDLE, FINE), Some(&mesh));
    assert!(!session.preview_tessellate_is_pending(hash));
    assert!(!session.note_pending_tessellate(hash, HANDLE.into(), FINE));
    session.retire_cold();
}

#[test]
fn a_partial_step_updates_progress_without_publishing_mesh() {
    let mut session = FlowEvalSession::new();
    let hash = admit(&mut session, FINE);
    assert!(session.arm_window_tick(WINDOW.0, crate::preview_reply::TURN).expect("window tick admission").0);
    session.begin_window_tick(WINDOW.0, crate::preview_reply::TURN).map(drop).expect("original window latch turn");
    session.note_window_tick_outcome(WINDOW.0, semio_s_artifact_procedural_3d_preview_eval::tick_is_unfinished(false, 1), crate::preview_reply::TURN).map(drop).expect("original window latch turn");
    session.note_window_extensions_in_flight(WINDOW.0, 1, crate::preview_reply::TURN).map(drop).expect("original window latch turn");
    receive(&mut session, response(WINDOW, HANDLE, FINE, "meshingFaces", (12, 30), None)).expect("typed progress folds");
    let status = session.preview_tessellate_status();
    assert_eq!((status.units_done, status.units_total), (12, 30));
    assert_eq!(status.phase.tag(), "meshingFaces");
    assert!(session.window_tick_owed(WINDOW.0));
    assert!(session.preview_mesh(HANDLE).is_none());
    assert!(!session.preview_tessellate_is_pending(hash));
    assert!(session.note_pending_tessellate(hash, HANDLE.into(), FINE));
    session.retire_cold();
}

#[test]
fn complete_mesh_replaces_the_earlier_lod_as_one_cache_entry() {
    let mut session = FlowEvalSession::new();
    let coarse = triangle(COARSE);
    let fine = triangle(FINE);
    let coarse_hash = admit(&mut session, COARSE);
    complete(&mut session, COARSE, &coarse);
    admit(&mut session, FINE);
    assert_eq!(session.preview_mesh(HANDLE), Some(&coarse));
    assert!(!session.note_pending_tessellate(coarse_hash, HANDLE.into(), COARSE));
    complete(&mut session, FINE, &fine);
    assert_eq!(session.preview_mesh_for_tolerance(HANDLE, FINE), Some(&fine));
    assert!(session.preview_mesh_for_tolerance(HANDLE, COARSE).is_none());
    assert_eq!(session.preview_mesh_handles().count(), 1);
    session.retire_cold();
}

#[test]
fn retention_keeps_the_pinned_request_until_its_reply_commits() {
    let mut session = FlowEvalSession::new();
    let hash = admit(&mut session, FINE);
    let pinned = session.in_flight_preview_handles().map(str::to_owned).collect();
    retain(&mut session, pinned);
    assert!(session.preview_tessellate_is_pending(hash));
    let mesh = triangle(COARSE);
    complete(&mut session, FINE, &mesh);
    assert_eq!(session.preview_mesh(HANDLE), Some(&mesh));
    session.retire_cold();
}

#[test]
fn a_removed_request_discards_its_reply_even_after_a_new_lod_is_admitted() {
    let mut session = FlowEvalSession::new();
    let old_hash = admit(&mut session, COARSE);
    retain(&mut session, Vec::new());
    assert!(!session.preview_tessellate_is_pending(old_hash));
    let new_hash = admit(&mut session, FINE);
    let mesh = triangle(COARSE);
    complete(&mut session, COARSE, &mesh);
    assert!(session.preview_mesh(HANDLE).is_none());
    assert!(session.preview_tessellate_is_pending(new_hash));
    complete(&mut session, FINE, &mesh);
    assert_eq!(session.preview_mesh_for_tolerance(HANDLE, FINE), Some(&mesh));
    session.retire_cold();
}

#[test]
fn an_invalid_canonical_mesh_never_reaches_the_renderer() {
    let mut session = FlowEvalSession::new();
    admit(&mut session, FINE);
    use semio_framework_value::DslValue;
    let mut payload = response(WINDOW, HANDLE, FINE, "complete", (3, 3), Some(&triangle(COARSE)));
    let DslValue::Object(fields) = &mut payload.output else { unreachable!() };
    let DslValue::Object(mesh) = &mut fields.iter_mut().find(|(key, _)| key == "mesh").unwrap().1 else { unreachable!() };
    mesh.iter_mut().find(|(key, _)| key == "indices").unwrap().1 =
        crate::preview_reply::dto(crate::preview_reply::lod_fixture()["invalidIndices"].clone());
    assert!(receive(&mut session, payload).is_err());
    assert!(session.preview_mesh(HANDLE).is_none());
    cancel(&mut session, WINDOW.0);
    session.retire_cold();
}

#[test]
fn unknown_node_hash_resolves_nothing() {
    let mut session = FlowEvalSession::new();
    session.note_window_tick_outcome(WINDOW.0, false, crate::preview_reply::TURN).expect("window is idle");
    complete(&mut session, FINE, &triangle(COARSE));
    assert!(session.preview_mesh(HANDLE).is_none());
    assert!(!session.window_tick_owed(WINDOW.0));
    session.retire_cold();
}
