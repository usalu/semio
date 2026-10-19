use super::*;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use store::os_store::document_materialize::create_document_envelope;
use store::os_store::store_schema::ArtifactCommand;

#[semio_framework_async_macros::async_test]
async fn standalone_history_close_resumes_past_the_original_ledger_page() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/♻️store-close/🔣️.json")).unwrap();
    for (row, scheduled) in corpus["histories"].as_array().unwrap().iter().flat_map(|row| [(row, true), (row, false)]) {
        let edits = row["edits"].as_u64().unwrap() as usize;
        let mut store = puzzle2d_store(create_document_envelope(semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records::PUZZLE_2D_SCHEMA, "close-law", Puzzle2dSnapshot::default(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
        for index in 0..edits {
            let mutation = crate::standards::v1::subsets::any::schema::mutations::change_manifest_id(Some(format!("manifest-{index}").into()));
            store.dispatch(ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
        }
        assert_eq!(store.mutation_ops().unwrap().len(), edits);
        if !scheduled {
            close_puzzle2d_store(&mut store).unwrap();
            assert!(store.close_owned_store_terminal_is_empty());
            close_puzzle2d_store(&mut store).unwrap();
            eprintln!("[DEBUG] standalone history close edits={edits} scheduled=false terminal=true");
            continue;
        }
        let mut turns = 0;
        while !store.close_owned_store_terminal_is_empty() {
            let demand = store.close_owned_demands(corpus["maximumBodyBytes"].as_u64().unwrap() as usize).unwrap();
            let grant = RetainedCloneGrant { maximum_items: corpus["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: demand.copy_bytes.max(corpus["maximumBodyBytes"].as_u64().unwrap() as usize), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            let witness = store.close_owned_phase_witness();
            let denied = close_puzzle2d_store_step(&mut store, RetainedCloneGrant { maximum_items: corpus["deniedItems"].as_u64().unwrap() as usize, ..grant }).unwrap();
            assert_eq!(denied.progress(), Some(RetainedCloneProgress::default()));
            assert_eq!(store.close_owned_phase_witness(), witness);
            let step = close_puzzle2d_store_step(&mut store, grant).unwrap();
            assert!(step.progress().expect("a funded close step has progress").fits(grant));
            turns += 1;
            assert!(turns < 100_000, "a funded close cursor must make progress");
        }
        if edits == 400 { assert!(turns > 384); }
        eprintln!("[DEBUG] standalone history close edits={edits} scheduled=true turns={turns} terminal=true");
    }
}
