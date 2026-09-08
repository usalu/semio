mod artifact_media_export_credit_tests {
    use super::*;

    #[test]
    fn segmented_output_accepts_exact_cap_and_rejects_plus_one_or_foreign_authority() {
        let chunks = ArtifactOutputChunks::new(4);
        assert!(chunks.take_chunk().is_err());
        assert_eq!(chunks.push(vec![1, 2, 3, 4]), Ok(4));
        assert!(chunks.push(vec![5]).is_err());
        assert_eq!(chunks.seal(), Ok(4));
        assert_eq!(chunks.chunks_remaining(), 1);

        let foreign = ArtifactOutputChunks::new(4);
        assert_eq!(foreign.push(vec![1, 2, 3, 4]), Ok(4));
        assert_eq!(foreign.seal(), Ok(4));
        let result = ArtifactMediaExportResult::structured(MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, "", foreign.clone()).expect("sealed foreign result");
        assert!(validate_media_export_structure(&result, &chunks, 4, 4).is_err());
        let download = ArtifactDownloadOutput::new("exact.bin", "application/octet-stream", Some("base64".into()), foreign).expect("bounded sealed download");
        assert_eq!(download.handle_encoding(), "semio-segmented-handle-v1:base64");

        assert_eq!(chunks.take_chunk().expect("sealed output"), Some(vec![1, 2, 3, 4]));
        assert_eq!(chunks.take_chunk().expect("drained output"), None);
        let oversized = ArtifactOutputChunks::new(ARTIFACT_OUTPUT_CHUNK_BYTES + 1);
        assert!(oversized.push(vec![0; ARTIFACT_OUTPUT_CHUNK_BYTES + 1]).is_err());
    }

    #[test]
    fn final_poll_credit_accepts_maximum_and_rejects_plus_one_without_allocating_media() {
        let credit = ArtifactMediaExportCredit::new(8);
        let chunks = ArtifactOutputChunks::new(8);
        assert_eq!(chunks.push(b"123456".to_vec()), Ok(6));
        assert_eq!(chunks.seal(), Ok(6));
        assert_eq!(credit.credit(8), Ok(8));
        assert_eq!(credit.validate_terminal(), Ok(8));
        let maximum = ArtifactMediaExportResult::structured(MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, "2d", chunks.clone()).expect("sealed maximum result");
        assert!(validate_media_export_structure(&maximum, &chunks, credit.bytes(), 8).is_ok());
        assert!(credit.credit(1).is_err());
        assert!(credit.validate_terminal().is_err());

        let under_credited = ArtifactMediaExportCredit::new(8);
        assert_eq!(under_credited.credit(8), Ok(8));
        let plus_one_chunks = ArtifactOutputChunks::new(9);
        assert_eq!(plus_one_chunks.push(b"1234567".to_vec()), Ok(7));
        assert_eq!(plus_one_chunks.seal(), Ok(7));
        let plus_one = ArtifactMediaExportResult::structured(maximum.media_type, "2d", plus_one_chunks.clone()).expect("sealed plus-one result");
        assert!(validate_media_export_structure(&plus_one, &plus_one_chunks, under_credited.bytes(), 8).is_err());
    }

    #[test]
    fn segmented_output_preallocates_exact_former_growth_boundary_and_drains_terminal_storage_to_zero() {
        const SLOTS: usize = 65;
        let maximum = SLOTS * ARTIFACT_OUTPUT_CHUNK_BYTES;
        let chunks = ArtifactOutputChunks::new(maximum);
        assert_eq!(chunks.slot_capacity(), SLOTS);
        for index in 0..SLOTS {
            assert_eq!(chunks.push(vec![index as u8; ARTIFACT_OUTPUT_CHUNK_BYTES]), Ok((index + 1) * ARTIFACT_OUTPUT_CHUNK_BYTES));
        }
        assert!(chunks.push(vec![0]).is_err());
        assert_eq!(chunks.seal(), Ok(maximum));
        for index in 0..SLOTS {
            assert_eq!(chunks.take_chunk().expect("preadmitted slot"), Some(vec![index as u8; ARTIFACT_OUTPUT_CHUNK_BYTES]));
        }
        assert_eq!(chunks.chunks_remaining(), 0);
        assert_eq!(chunks.take_chunk().expect("terminal queue"), None);

        struct DropSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
        impl Drop for DropSentinel {
            fn drop(&mut self) {
                self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut queue = ArtifactFixedQueue::new(SLOTS);
        for _ in 0..SLOTS {
            assert!(queue.push(DropSentinel(drops.clone())).is_ok());
        }
        for _ in 0..SLOTS {
            drop(queue.pop().expect("sentinel slot"));
        }
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), SLOTS);
        drop(queue);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), SLOTS, "dropping a fully drained maximum-slot queue must not revisit capacity slots");
    }

    #[test]
    fn segmented_output_seal_is_linearly_ordered_with_push() {
        let chunks = ArtifactOutputChunks::new(ARTIFACT_OUTPUT_CHUNK_BYTES);
        let state = chunks.inner.state.try_lock().expect("exclusive producer authority");
        let contender = chunks.clone();
        let result = std::thread::spawn(move || contender.seal()).join().expect("seal contender returns");
        assert_eq!(result.expect_err("seal must not race an active producer").code.0, "interactive-job.segmented-output-busy");
        assert!(!chunks.is_sealed());
        drop(state);
        assert_eq!(chunks.push(vec![1; ARTIFACT_OUTPUT_CHUNK_BYTES]), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES));
        assert_eq!(chunks.seal(), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES));
        assert_eq!(chunks.bytes(), ARTIFACT_OUTPUT_CHUNK_BYTES);
    }

    #[test]
    fn snapshot_a_survives_cache_b_and_only_the_bounded_retirement_owner_performs_final_drop() {
        struct DropItem(std::sync::Arc<std::sync::atomic::AtomicUsize>);
        impl Drop for DropItem {
            fn drop(&mut self) {
                self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
        struct SnapshotDisposer;
        impl ArtifactSnapshotDisposer<Vec<DropItem>> for SnapshotDisposer {
            fn close_step(&mut self, snapshot: &mut Option<std::sync::Arc<Vec<DropItem>>>, maximum_items: usize, _maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
                if maximum_items == 0 {
                    return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                }
                let Some(owner) = snapshot.as_mut() else { return Ok(PluginCloseStep::Complete) };
                let Some(items) = std::sync::Arc::get_mut(owner) else { return Ok(PluginCloseStep::Blocked { reason: "snapshot remains externally owned" }) };
                if items.pop().is_some() {
                    return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
                }
                drop(snapshot.take());
                Ok(PluginCloseStep::Complete)
            }

            fn terminal_is_empty(&self, snapshot: &Option<std::sync::Arc<Vec<DropItem>>>) -> bool {
                snapshot.is_none()
            }
        }

        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let cache_a = std::sync::Arc::new(vec![DropItem(drops.clone()), DropItem(drops.clone())]);
        let job_a = cache_a.clone();
        let (lease, mut retirement_a) = ArtifactSnapshotCloseLease::new(&cache_a, Box::new(SnapshotDisposer));
        let cache_b = std::sync::Arc::new(Vec::<DropItem>::new());
        drop(cache_a);
        assert!(lease.can_release(&job_a), "runtime retention keeps exact A alive after cache advances to B");
        drop(job_a);
        drop(lease);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0, "job and lease release must not become final for snapshot A");
        assert_eq!(retirement_a.close_step(1, 0).expect("first bounded A disposal"), PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(retirement_a.close_step(1, 0).expect("second bounded A disposal"), PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        assert_eq!(retirement_a.close_step(1, 0).expect("terminal A disposal"), PluginCloseStep::Complete);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 2);
        assert!(std::sync::Arc::strong_count(&cache_b) == 1, "cache B is independent of retired A authority");
    }
}
