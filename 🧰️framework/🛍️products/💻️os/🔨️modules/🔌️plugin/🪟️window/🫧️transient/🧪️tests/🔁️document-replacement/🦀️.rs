mod document_window_replacement_tests {
    use super::*;

    #[test]
    fn retained_window_input_document_scope_renewal_cancels_old_and_admits_new_work() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🪟️window/🫧️transient/🧫️fixtures/🪟️retained-window-input/🔣️.json"))).unwrap();
        let expected = &fixture["documentReplacement"];
        let key =
            |id| ToolOperationKey { app_instance_id: 11, document: ArtifactDocumentAuthority(11), operation_id: semio_framework_job::OperationId(id), base_revision: semio_framework_job::RevisionId(0), generation: semio_framework_job::Generation(0) };
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

    struct RetirementWindow;

    impl crate::WindowTransientOwner for RetirementWindow {
        const WINDOW_KIND_ID: &'static str = "canvas";
        type State = crate::publication_fixture::PublicationTransient;
        type Mutation = crate::publication_fixture::PublicationTransientMutation;

        fn build_owners() -> crate::component::window_transient::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
            crate::window_transient_testkit::owners()
        }
    }

    #[test]
    fn retained_window_input_retirement_reaches_later_document_generations() {
        let mut blocked = WindowTransientOwnerRegistry::for_document_generation(1);
        blocked.register::<RetirementWindow>().unwrap();
        let view = ViewModel { window_id: Some("first".into()), window_instances: vec![semio_framework::ViewWindowInstance { id: "first".into(), window_kind_id: "canvas".into() }], ..Default::default() };
        let held = blocked.capture(Some(&view)).unwrap().unwrap();
        let mut ready = WindowTransientOwnerRegistry::for_document_generation(2);
        ready.register::<RetirementWindow>().unwrap();
        let mut registries = ArtifactFixedRegistry::new();
        registries.insert_admitted(1, blocked);
        registries.insert_admitted(2, ready);
        let mut maintenance_cursor = 0;
        let mut close_cursor = 0;
        assert_eq!(retire_document_window_registry_step(&mut registries, &mut maintenance_cursor, 0, 0).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(maintenance_cursor, 0);
        for _ in 0..8 {
            retire_document_window_registry_step(&mut registries, &mut maintenance_cursor, 1, 0).unwrap();
        }
        assert!(registries.get(1).is_some());
        assert!(registries.get(2).is_none());
        assert_eq!(close_cursor, 0);
        drop(held);
        for _ in 0..32 {
            if retire_document_window_registry_step(&mut registries, &mut close_cursor, 1, 4096).unwrap() == PluginCloseStep::Complete {
                break;
            }
        }
        assert!(registries.is_empty());
        eprintln!("[DEBUG] zero grant preserves document cursor; blocked first generation permits later retirement; close cursor drains returned owners");
    }
}
