//! ⚖️ Flow owners retire under quoted grants: a retained mesh encoding job retires mid-flight, and a map delta retires its keyed entries, both through the typed-command input lane.

#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_framework_test_typed_input_law as typed_input_law;

use semio_framework_os_flow::mesh::MeshPackEncodingJob;
use protocol::MapDelta;
use typed_input_law::typed_input_retirement_law;

fn golden_mesh() -> semio_framework::MeshData {
    semio_framework::MeshData {
        positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        normals: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
        indices: vec![0, 1, 2],
        face_ids: vec![7],
        edge_positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        edge_ids: vec![11],
        ..Default::default()
    }
}

/// 🧱️ A mesh encoding job retires before and during its encoding, owning mesh, bytes and metadata cursor.
#[test]
fn a_mesh_pack_encoding_job_retires_through_the_typed_command_lane() {
    let fresh = MeshPackEncodingJob::new(golden_mesh(), 64_000_000).expect("the golden mesh admits its encoder");
    let fresh_law = typed_input_retirement_law("mesh pack encoding job, fresh", fresh);
    let mut running = MeshPackEncodingJob::new(golden_mesh(), 64_000_000).expect("the golden mesh admits its encoder");
    for _ in 0..8 {
        if running.step(1).expect("a bounded encoding turn").is_some() { break; }
    }
    let running_law = typed_input_retirement_law("mesh pack encoding job, running", running);
    assert!(fresh_law.turns > 0 && running_law.turns > 0, "retiring costs paid turns");
}

/// 🗃️ A map delta retires its keyed entries exactly, empty or merged from several changes.
#[test]
fn a_map_delta_retires_through_the_typed_command_lane() {
    let empty = typed_input_retirement_law("map delta, empty", MapDelta::<String>::default());
    let mut delta = MapDelta::set("alpha".to_string(), "first value".to_string());
    delta.absorb(MapDelta::set("beta".to_string(), "second value".to_string()));
    delta.absorb(MapDelta::remove("gamma".to_string()));
    let merged = typed_input_retirement_law("map delta, merged", delta);
    assert!(merged.turns > empty.turns, "entries cost paid turns");
}
