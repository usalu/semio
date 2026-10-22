use super::*;

#[test]
fn granted_fold_backing_birth_waits_before_frame_and_vector_allocation() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for count in law["lengths"].as_array().unwrap() {
        let count = count.as_u64().unwrap() as usize;
        let mut job = HistoryFoldJob::new(move |control| async move {
            let mut bytes = control.vector::<u8>(count).await?;
            assert_eq!(bytes.len(), 0);
            assert_eq!(bytes.capacity(), count);
            bytes.extend(std::iter::repeat_n(37, count));
            drop(bytes);
            Ok(())
        });
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| job.step(job.next_step_grant(1, 4096).unwrap(),4096, &mut || false).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let grant = job.next_step_grant(1, 4096).unwrap();
        let expected = owned_retirement_birth_bytes::<Vec<u8>>() + count;
        assert_eq!(grant.maximum_capacity_bytes, expected);
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }, RetainedCloneGrant { maximum_capacity_bytes: expected - 1, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| job.step(denied,4096, &mut || false));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(job.next_step_grant(1, 4096).unwrap().maximum_capacity_bytes, expected);
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| job.step(grant,4096, &mut || false).unwrap());
        let HistoryFoldJobStep::Pending { progress, .. } = step else { panic!("original frame must close before handoff") };
        assert_eq!((heap.requested_bytes, heap.released_bytes), (expected, 0));
        assert_eq!(progress.retained_capacity_bytes, expected);
        assert!(progress.fits(grant));
        close_fold_and_check_original_grants(&mut job);
    }
}

#[test]
fn granted_fold_waiting_backing_cancellation_never_allocates_the_unborn_vector() {
    let mut job = HistoryFoldJob::new(move |control| async move { let bytes = control.vector::<u8>(65537).await?; drop(bytes); Ok(()) });
    job.step(job.next_step_grant(1, 7).unwrap(),7, &mut || false).unwrap();
    assert!(job.control.0.pending_construction_capacity.load(Ordering::Relaxed) > 65537);
    job.request_cancel();
    assert_eq!(job.next_step_grant(1, 7).unwrap().maximum_capacity_bytes, 0);
    close_fold_and_check_original_grants(&mut job);
}

#[test]
fn granted_trunk_identity_matches_the_cold_blake3_wire_oracle() {
    for text in ["", "artifact", "🧩️/ä/共同", &"x".repeat(65537)] {
        let document = ArtifactId(text.into());
        let expected = trunk_alternative_id(&document);
        let mut material = Vec::new();
        semio_framework_replication_wire_core::write_str(&mut material, "semio.history.trunk");
        semio_framework_replication_wire_core::write_str(&mut material, &document.0);
        let oracle = format!("trunk-{}", &blake3::hash(&material).to_hex()[..16]);
        assert_eq!(expected, oracle);
        let actual = HistoryFoldJob::new(|control| async move { copy_history_trunk(&document, &control).await }).finish_cold().unwrap();
        assert_eq!(actual, expected);
    }
}
