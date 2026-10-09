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
    }

    struct RetirementWindow;

    impl crate::WindowTransientOwner for RetirementWindow {
        const WINDOW_KIND_ID: &'static str = "canvas";
        type State = crate::publication_fixture::PublicationTransient;
        type Mutation = crate::publication_fixture::PublicationTransientMutation;

        fn build_owners() -> crate::component::window_transient::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
            crate::window_transient_owners::owners()
        }
    }

    #[test]
    fn retained_window_input_retirement_reaches_later_document_generations() {
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪟️retained-window-input/🔣️.json")).unwrap();
        let p=&fixture["retirementPolicy"];
        let grant=RetainedCloneGrant{maximum_items:p["items"].as_u64().unwrap()as usize,maximum_copy_bytes:p["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:p["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:p["release"].as_u64().unwrap()as usize,maximum_depth:p["depth"].as_u64().unwrap()as usize};
        let mut blocked = WindowTransientOwnerRegistry::for_document_generation(1);
        blocked.register::<RetirementWindow>().unwrap();
        let view = ViewModel { window_id: Some("first".into()), window_instances: vec![semio_framework::ViewWindowInstance { id: "first".into(), window_kind_id: "canvas".into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
        let held = blocked.capture(Some(&view)).unwrap().unwrap();
        let mut ready = WindowTransientOwnerRegistry::for_document_generation(2);
        ready.register::<RetirementWindow>().unwrap();
        let mut registries = ArtifactFixedRegistry::new();
        registries.insert_admitted(1, blocked);
        registries.insert_admitted(2, ready);
        let mut maintenance_cursor = 0;
        let mut close_cursor = 0;
        assert_eq!(retire_document_window_registry_step(&mut registries, &mut maintenance_cursor, RetainedCloneGrant::default(),false).unwrap(), PluginLifecycleStep::Progress(Default::default()));
        assert_eq!(maintenance_cursor, 0);
        for _ in 0..p["maximumTurns"].as_u64().unwrap() {
            if registries.get(2).is_none(){break;}
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retire_document_window_registry_step(&mut registries, &mut maintenance_cursor,grant,false).unwrap());
            if let Some(progress)=step.progress(){assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
            assert!(registries.get(1).is_some());
        }
        assert!(registries.get(1).is_some());
        assert!(registries.get(2).is_none());
        assert_eq!(close_cursor, 0);
        drop(held);
        for _ in 0..p["maximumTurns"].as_u64().unwrap() {
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retire_document_window_registry_step(&mut registries, &mut close_cursor,grant,true).unwrap());
            if let Some(progress)=step.progress(){assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
            if matches!(step,PluginLifecycleStep::Complete(_)) {
                break;
            }
        }
        assert!(registries.is_empty());
        assert_eq!(registries.empty_backing_byte_demand(),Some(0));
        eprintln!("[DEBUG] zero grant preserves document cursor; original blocked generation permits later retirement; admitted close releases all original registry backing");
    }

    #[test]
    fn original_displaced_window_registry_retains_empty_physical_backing_until_exact_release(){
        let(mut registries,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(ArtifactFixedRegistry::<WindowTransientOwnerRegistry>::new);
        let backing=registries.empty_backing_byte_demand().unwrap();assert!(backing>0);assert_eq!((birth.requested_bytes,birth.released_bytes),(backing,0));
        let pointer=registries.slots.as_ptr();let mut cursor=0;
        let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:backing,maximum_depth:1,..Default::default()};
        assert_eq!(retire_document_window_registry_demands(&registries,cursor,0,false).unwrap(),RetirementDemand::default());
        assert_eq!(retire_document_window_registry_demands(&registries,cursor,0,true).unwrap(),RetirementDemand{release_bytes:backing,depth:1,..Default::default()});
        for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_release_bytes:backing-1,..grant},RetainedCloneGrant{maximum_release_bytes:0,maximum_capacity_bytes:backing,maximum_copy_bytes:backing,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retire_document_window_registry_step(&mut registries,&mut cursor,denied,true).unwrap());
            assert_eq!(step,PluginLifecycleStep::Progress(Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(registries.slots.as_ptr(),pointer);assert_eq!(registries.empty_backing_byte_demand(),Some(backing));assert_eq!(cursor,0);
        }
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retire_document_window_registry_step(&mut registries,&mut cursor,grant,true).unwrap());
        assert_eq!(step,PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:backing,..Default::default()}));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,backing));assert_eq!(registries.empty_backing_byte_demand(),Some(0));
        let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(registries));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        eprintln!("[DEBUG] original displaced window registry physical={backing} exact refusal custody and separate final backing release terminalDrop0");
    }
}
