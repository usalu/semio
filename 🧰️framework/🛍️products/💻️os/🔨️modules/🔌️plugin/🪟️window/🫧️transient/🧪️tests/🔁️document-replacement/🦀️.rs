mod document_window_replacement_tests {
    use super::*;

    /// 🪢️ Runs one displaced-window retirement turn as a genuine original producer invocation and observes its exact heap traffic.
    fn effect_turn<R>(grant: RetainedCloneGrant, turn: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> R) -> (R, semio_framework_trace::HeapAllocationObservation) {
        use semio_framework_value::{RetainedCloneStep, RetainedWorkPoll, ValueError};
        use semio_framework_value_testing_admission::{driven, BankWallet, EffectBank};
        let mut bank = EffectBank::new(BankWallet::new(RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 1 << 20 }, 1 << 30));
        let (mut turn, mut output) = (Some(turn), None);
        let (poll, _) = driven::<ValueError>(&mut bank, grant, 1 << 20, |effect, reply, _| {
            let turn = turn.take().ok_or_else(|| ValueError::invariant("fixture operation runs once"))?;
            output = Some(semio_framework_trace::observe_heap_allocations_on_this_thread(|| turn(effect)));
            Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))
        });
        assert!(matches!(poll, RetainedWorkPoll::Complete(ref done) if done.result.is_ok()), "fixture effect turn did not complete");
        assert!(bank.into_wallet().is_ok(), "fixture bank closed with a live original action");
        output.expect("fixture operation ran")
    }

    fn caller() -> semio_framework_job::OriginalJobFixtureCaller {
        semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), stock(), || Some(1000), 1500, 10_000)
    }
    fn stock() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1 << 10, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 } }

    #[test]
    fn retained_window_input_document_scope_renewal_cancels_old_and_admits_new_work() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🪟️window/🫧️transient/🧪️tests/🧫️fixtures/🪟️retained-window-input/🔣️.json"))).unwrap();
        let expected = &fixture["documentReplacement"];
        let key =
            |id| ToolOperationKey { app_instance_id: 11, document: ArtifactDocumentAuthority(11), operation_id: semio_framework_job::OperationId(id), base_revision: semio_framework_job::RevisionId(0), generation: semio_framework_job::Generation(0) };
        let mut cancellations = semio_plugin_testing_laws::artifact_app_laws::KeyedCancellationFixture::open();
        let mut old = cancellations.begin_keyed(key(1));
        let mut old_single = cancellations.begin(key(2));
        let mut renewed = false;
        for _ in 0..100_000 {
            let cx = cancellations.scope.cx();
            if cancellations.issuer.renew_scope_generation(cx).unwrap() { renewed = true; break; }
        }
        assert!(renewed, "the scope generation renewal settles");
        let mut fresh = cancellations.begin_keyed(key(3));
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
        assert_eq!(cancellations.handle().active_operation_count(), 0);
        cancellations.close_lease(old);
        cancellations.close_lease(old_single);
        cancellations.close_lease(fresh);
        cancellations.close();
    }

    struct RetirementWindow;

    impl semio_plugin_window_transient::window_transient::WindowTransientOwner for RetirementWindow {
        fn owners_birth_demands()->semio_framework_value::RetirementDemand{semio_plugin_window_transient::window_transient::transfer_owners_birth_demands::<Self::State,Self::Mutation>()}
        const WINDOW_KIND_ID: &'static str = "canvas";
        type State = semio_plugin_testing_fixtures_publication::publication_fixture::transient::PublicationTransient;
        type Mutation = semio_plugin_testing_fixtures_publication::publication_fixture::transient::mutations::PublicationTransientMutation;

        fn build_owners() -> semio_plugin_window_transient::window_transient::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
            semio_plugin_testing_fixtures_publication::window_transient_owners::owners()
        }
    }

    #[test]
    fn retained_window_input_retirement_reaches_later_document_generations() {
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪟️retained-window-input/🔣️.json")).unwrap();
        let p=&fixture["retirementPolicy"];
        let grant=RetainedCloneGrant{maximum_items:p["items"].as_u64().unwrap()as usize,maximum_copy_bytes:p["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:p["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:p["release"].as_u64().unwrap()as usize,maximum_depth:p["depth"].as_u64().unwrap()as usize};
        let mut blocked = WindowTransientOwnerRegistry::for_document_generation(1);
        blocked.register::<RetirementWindow>().unwrap();
        let view = ViewModel { window_id: Some("first".into()), window_instances: vec![semio_framework::ViewWindowInstance { id: "first".into(), window_kind_id: "canvas".into() }], ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) };
        let held = blocked.capture(Some(&view)).unwrap().unwrap();
        let mut ready = WindowTransientOwnerRegistry::for_document_generation(2);
        ready.register::<RetirementWindow>().unwrap();
        let mut caller = caller();
        let mut registries = ArtifactFixedRegistry::new();
        registries.insert_admitted(1, blocked);
        registries.insert_admitted(2, ready);
        let mut maintenance_cursor = 0;
        let mut close_cursor = 0;
        assert_eq!(effect_turn(RetainedCloneGrant::default(),|effect|retire_document_window_registry_step(&mut registries, &mut maintenance_cursor, effect,false).unwrap()).0, PluginLifecycleStep::Progress(Default::default()));
        assert_eq!(maintenance_cursor, 0);
        for _ in 0..p["maximumTurns"].as_u64().unwrap() {
            if registries.get(2).is_none(){break;}
            let(step,heap)=effect_turn(grant,|effect|retire_document_window_registry_step(&mut registries, &mut maintenance_cursor,effect,false).unwrap());
            if let Some(progress)=step.progress(){assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
            assert!(registries.get(1).is_some());
        }
        assert!(registries.get(1).is_some());
        assert!(registries.get(2).is_none());
        assert_eq!(close_cursor, 0);
        drop(held);
        for _ in 0..p["maximumTurns"].as_u64().unwrap() {
            let(step,heap)=effect_turn(grant,|effect|retire_document_window_registry_step(&mut registries, &mut close_cursor,effect,true).unwrap());
            if let Some(progress)=step.progress(){assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
            if matches!(step,PluginLifecycleStep::Complete(_)) {
                break;
            }
        }
        assert!(registries.is_empty());
        assert_eq!(registries.empty_backing_byte_demand(),Some(0));
        caller.retire(stock());
    }

    #[test]
    fn original_displaced_window_registry_retains_empty_physical_backing_until_exact_release(){
        let(mut registries,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(ArtifactFixedRegistry::<WindowTransientOwnerRegistry>::new);
        let backing=registries.empty_backing_byte_demand().unwrap();assert!(backing>0);assert_eq!((birth.requested_bytes,birth.released_bytes),(backing,0));
        let mut caller = caller();
        let pointer=registries.slots.as_ptr();let mut cursor=0;
        let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:backing,maximum_depth:1,..Default::default()};
        assert_eq!(retire_document_window_registry_demands(&registries,cursor,0,false).unwrap(),RetirementDemand::default());
        assert_eq!(retire_document_window_registry_demands(&registries,cursor,0,true).unwrap(),RetirementDemand{release_bytes:backing,depth:1,..Default::default()});
        for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_release_bytes:backing-1,..grant},RetainedCloneGrant{maximum_release_bytes:0,maximum_capacity_bytes:backing,maximum_copy_bytes:backing,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
            let(step,heap)=effect_turn(denied,|effect|retire_document_window_registry_step(&mut registries,&mut cursor,effect,true).unwrap());
            assert_eq!(step,PluginLifecycleStep::Progress(Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(registries.slots.as_ptr(),pointer);assert_eq!(registries.empty_backing_byte_demand(),Some(backing));assert_eq!(cursor,0);
        }
        let(step,heap)=effect_turn(grant,|effect|retire_document_window_registry_step(&mut registries,&mut cursor,effect,true).unwrap());
        assert_eq!(step,PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:backing,..Default::default()}));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,backing));assert_eq!(registries.empty_backing_byte_demand(),Some(0));
        let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(registries));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        caller.retire(stock());
    }
}
