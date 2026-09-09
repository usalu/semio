mod document_window_replacement_tests {
    use super::*;

    #[test]
    fn retained_window_input_document_scope_renewal_cancels_old_and_admits_new_work() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🪟️window/🫧️transient/🧫️fixtures/🪟️retained-window-input/🔣️.json"))).unwrap();
        let expected = &fixture["documentReplacement"];
        let key = |id| ToolOperationKey {
            app_instance_id: 11,
            document: ArtifactDocumentAuthority(11),
            operation_id: semio_framework_job::OperationId(id),
            base_revision: semio_framework_job::RevisionId(0),
            generation: semio_framework_job::Generation(0),
        };
        let mut handle = ToolCancellationHandle::default();
        let mut old = handle.begin_keyed(key(1)).unwrap();
        let old_single = handle.begin(key(2)).unwrap();
        handle.renew_scope_generation();
        let fresh = handle.begin_keyed(key(3)).unwrap();
        assert_eq!(old.token.is_cancelled_now(), expected["oldWorkCancelled"].as_bool().unwrap());
        assert!(old_single.token.is_cancelled_now());
        assert!(old.try_claim_publication().is_none());
        assert!(old.rebind_keyed(semio_framework_job::RevisionId(1), semio_framework_job::Generation(1)).is_err());
        assert_eq!(fresh.token.is_cancelled_now(), expected["newWorkCancelled"].as_bool().unwrap());
        assert!(fresh.try_claim_publication().is_some());
        old.finish();
        old_single.finish();
        assert!(!fresh.token.is_cancelled_now());
        assert!(fresh.try_claim_publication().is_some());
        fresh.finish();
        assert_eq!(handle.active_operation_count(), 0);
        eprintln!("[DEBUG] replacement cancels old keyed and single work, rejects rebase, admits fresh work, and releases all cancellation owners");
    }
}
