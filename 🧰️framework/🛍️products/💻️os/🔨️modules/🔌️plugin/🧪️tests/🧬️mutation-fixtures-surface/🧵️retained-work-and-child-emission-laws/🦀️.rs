/// 🧵️ Retained work and child emission laws of the surface fixture: the retained command fuel and fallback charges, the exact birth admission and the private typed lanes and group row frames of child emission.

#[semio_framework_async_macros::async_test]
async fn retained_command_work_borrows_original_fuel_and_fallback_charges_once() {
    use crate::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
    use semio_framework_job::InteractiveJob;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧵️retained-command/🧪️tests/🧫️fixtures/⛽️work-unit-authority.json")).unwrap();
    struct Probe { observed: Option<std::sync::Arc<std::sync::atomic::AtomicU64>>, consumes: u64, closing: bool }
    impl ArtifactCommandWork<EditorApp<SurfaceEditorFixture>> for Probe {
        fn tool_id(&self) -> &'static str { SURFACE_TOOL_ID }
        fn extent(&self, _: &SurfaceEditorCommand, _: &SurfaceSnapshot, _: &protocol::InteractionState, _: Option<&crate::tool_contract::ArtifactOwnedToolJobContext<EditorApp<SurfaceEditorFixture>>>) -> Option<usize> { Some(1) }
        fn step(&mut self, _: &ArtifactCommandInputs<'_, EditorApp<SurfaceEditorFixture>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<SurfaceEditorFixture>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
            self.observed.as_ref().unwrap().store(cx.fuel_remaining(), std::sync::atomic::Ordering::SeqCst);
            cx.consume_fuel(self.consumes);
            Ok(ArtifactCommandWorkStep::Complete(Emit::default()))

        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
        fn begin_close(&mut self) { self.closing = true; }
        fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
            use semio_framework_job::{InteractiveJobCloseStep as Step,RetainedCloneProgress};
            if !self.closing{return Ok(Step::Blocked)}let Some(original)=self.observed.as_ref()else{return Ok(Step::Complete{progress:Default::default()})};let copy=std::mem::size_of::<Option<std::sync::Arc<std::sync::atomic::AtomicU64>>>();let physical=semio_framework_value::shared_retirement_allocation_bytes::<std::sync::atomic::AtomicU64>();let demand=semio_framework_value::RetirementDemand{copy_bytes:copy,release_bytes:if std::sync::Arc::strong_count(original)==1{physical}else{0},depth:1,..Default::default()};
            semio_framework_job::close_original_retained_step(cx,demand,|_|{if std::sync::Arc::weak_count(self.observed.as_ref().unwrap())!=0{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original work probe retains an outstanding weak source"))}let unique=std::sync::Arc::into_inner(self.observed.take().unwrap()).is_some();Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:if unique{physical}else{0},..Default::default()}))})
        }

        fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if self.observed.is_some(){std::mem::size_of::<Option<std::sync::Arc<std::sync::atomic::AtomicU64>>>()}else{0})}
        fn next_close_capacity_byte_demand(&self,_copy:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
        fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if self.observed.as_ref().is_some_and(|original|std::sync::Arc::strong_count(original)==1){semio_framework_value::shared_retirement_allocation_bytes::<std::sync::atomic::AtomicU64>()}else{0})}
        fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(usize::from(self.observed.is_some()))}
        fn terminal_frame_release_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}
        fn terminal_is_empty(&self) -> bool { self.closing && self.observed.is_none() }
    }
    for row in fixture["cases"].as_array().unwrap() {
        let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();let mut cx=original_caller.context().with_original_decoder(&mut decoder);let recipient=cx.original_recipient_identity();
        let mut app=new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),crate::app_testing::artifact_app_laws::fixture_mounted_policy(),&mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
        let observed = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(u64::MAX));
        let work = Probe { observed: Some(observed.clone()), consumes: row["domainConsumes"].as_u64().unwrap(), closing: false };
        let completion = ArtifactToolCompletion::new();
        let consumer = completion.clone();
        let payload = ArtifactRetainedCommandPayload::new(ArtifactRetainedCommandInputs {
            command: SurfaceEditorCommand::Increment,
            snapshot_read:app.stores.store.snapshot_read().expect("Store issues the actual command donor"),
            snapshot:app.stores.store.snapshot_owner(), config: std::sync::Arc::new(NoConfig::default()), history: std::sync::Arc::new(HistoryView::empty()),
            interaction_state: std::sync::Arc::new(protocol::InteractionState::default()), interaction_hover: std::sync::Arc::new(Default::default()), context: None,
            operation: crate::app_views::AppOperationContext { app_instance_id: 1, parent_document_id: "unit-authority".into(), operation_id:cx.operation().0,generation:cx.generation().0,canonical_base_revision:[0;32],retained:SURFACE_ONE_UNIT_POLICY, authoring_seed: "unit-authority".into() }, completion,
        }, |_| SURFACE_TOOL_ID, 32, 1, Box::new(work));
        let mut job = ArtifactRetainedCommandJob::new(payload);
        assert!(job.test_admit_original_source(&mut cx).expect("actual original source binds the preborn fixture job"));
        let mut observed_work=false;
        for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
            let before=cx.fuel_remaining();let mut outcome=None;cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|{outcome=Some(crate::app_testing::artifact_app_laws::fixture_step(&mut job,same));Ok(())}).unwrap();assert!(!matches!(outcome.unwrap(),crate::app_testing::artifact_app_laws::FixtureStepOutcome::Fault(_)));
            if observed.load(std::sync::atomic::Ordering::SeqCst)!=u64::MAX{assert!(before>1);assert_eq!(observed.load(std::sync::atomic::Ordering::SeqCst),before);assert_eq!(cx.fuel_remaining(),before-row["workDebit"].as_u64().unwrap());observed_work=true;break}
        }
        assert!(observed_work);assert_eq!(cx.original_recipient_identity(),recipient);
        job.begin_close();
        for _ in 0..100_000 {
            let step=cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|job.close_step(same)).expect("original retained command close");assert!(step.progress().fits(SURFACE_ONE_UNIT_POLICY));match step{semio_framework_job::InteractiveJobCloseStep::Complete{..}=>break,semio_framework_job::InteractiveJobCloseStep::Pending{..}=>{},other=>panic!("one-unit original fixed policy retained its frontier: {other:?}")}
        }
        assert!(job.terminal_is_empty());
        assert!(consumer.take_emit().unwrap().is_none());
        crate::app_testing::mutation_fixture::job_close::retire_completion(consumer,&mut cx,SURFACE_ONE_UNIT_POLICY);
        close_registered_fixture_app(&mut app,crate::app_testing::artifact_app_laws::fixture_mounted_policy(),&mut cx).unwrap();assert_eq!(cx.original_recipient_identity(),recipient);let fuel=cx.fuel_remaining();drop(cx);let fuel=original_caller.retire_decoder(&mut decoder,fuel).unwrap();original_caller.retire(fuel).unwrap();
    }
}

#[test]
fn surface_owned_mutation_admission_forwards_exact_birth_and_preserves_every_denied_original() {
    use crate::app_contract::ArtifactApp;
    use crate::surface_contract::ViewerApp;
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    fn verify<A: ArtifactApp<Mutation = SurfaceMutation>>(count: usize) {
        let birth = A::owned_mutation_batch_birth_bytes().expect("selected surface declares its exact borrowed admission");
        let mut values = Some((0..count).map(|index| SetSurfaceCount { value: index as i32 }.into()).collect::<Vec<SurfaceMutation>>());
        let pointer = values.as_ref().unwrap().as_ptr();
        let capacity = values.as_ref().unwrap().capacity();
        for (items, bytes) in [(0, birth), (1, birth - 1)] {
            let (result, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| A::admit_owned_mutation_batch(&mut values, RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: 0, maximum_capacity_bytes: bytes, maximum_release_bytes: 0, maximum_depth: 8 }).unwrap());
            assert!(result.is_none());
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            assert_eq!(values.as_ref().unwrap().as_ptr(), pointer);
            assert_eq!((values.as_ref().unwrap().len(), values.as_ref().unwrap().capacity()), (count, capacity));
        }
        let ((mut owner, admitted), events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| A::admit_owned_mutation_batch(&mut values, RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: birth, maximum_release_bytes: 0, maximum_depth: 8 }).unwrap().expect("exact admitted surface authority"));
        assert!(values.is_none());
        assert_eq!(admitted.retained_capacity_bytes, birth);
        assert_eq!((events.requested_bytes, events.released_bytes), (birth, 0));
        assert_eq!(owner.mutations::<SurfaceMutation>().unwrap().as_ptr(), pointer);
        let mut released = 0;
        for _ in 0..count * 8 + 32 {
            let demand = owner.next_demands(64).unwrap();
            assert!(demand.copy_bytes <= 64 && demand.depth <= 8);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: 8 };
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_granted(grant).unwrap());
            let progress = match step { RetainedCloneStep::Progress(progress) => progress, RetainedCloneStep::Complete(progress) => progress };
            assert!(progress.fits(grant));
            assert_eq!(events.requested_bytes, progress.retained_capacity_bytes);
            assert_eq!(events.released_bytes, progress.released_bytes);
            released += progress.released_bytes;
            if owner.terminal_is_empty() { break; }
        }
        assert!(owner.terminal_is_empty());
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🏪️store/🧩️composition/📬️publication/🤝️group/🧪️tests/🧫️fixtures/📚️history-cases/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let count = row["historyEntries"].as_u64().unwrap() as usize;
        verify::<EditorApp<SurfaceEditorFixture>>(count);
        verify::<ViewerApp<SurfaceViewerFixture>>(count);
    }
}

#[semio_framework_async_macros::async_test]
async fn child_emission_private_typed_lanes_stage_three_original_batches_and_retire_all_cancelled_sources() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    use crate::app_contract::ArtifactApp;
    use crate::child_content::PrivateOwnedPublicationLane;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use store::SpaceMember;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🌱️genesis/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut stops: Vec<Option<usize>> = corpus["cancelBeforeCommit"].as_array().unwrap().iter().map(|stop| Some(stop.as_u64().unwrap() as usize)).collect();
    stops.push(None);
    for stop in stops {
        let mut apps = Vec::new();
        let mut lanes = Vec::new();
        let mut owner = semio_framework_os_kernel::os_vcs::ArtifactGroupVisibilityOwner::new();
        let visibility = owner.view();
        let grant = store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 262_144, maximum_capacity_bytes: 262_144, maximum_release_bytes: 262_144, maximum_depth: 64 };
        for _ in 0..3 {
            let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
            let (generation, revision) = app.stores.store.one_item_publication_identity();
            let mut original = Some(vec![SurfaceMutation::from(SetSurfaceCount { value: 1 })]);
            let pointer = original.as_ref().unwrap().as_ptr();
            let birth = <EditorApp<SurfaceEditorFixture> as ArtifactApp>::owned_mutation_batch_birth_bytes().unwrap();
            let (mutations, _) = <EditorApp<SurfaceEditorFixture> as ArtifactApp>::admit_owned_mutation_batch(&mut original, RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: birth, maximum_release_bytes: 0, maximum_depth: 64 }).unwrap().unwrap();
            let request = store::MemberStoreOwnedBatchRequest { operation: semio_framework_job::OperationId(7101), expected_generation: generation, expected_revision: revision, actor: protocol::LOCAL_ACTOR_ID.into(), group_id: Some("private-neutral-three-lane".into()), transaction: None, mutations };
            let mut lane = PrivateOwnedPublicationLane::new(request);
            let admitted = lane.next_capacity_byte_demand(&*app.stores.store).unwrap();
            for (items, bytes) in [(0, admitted), (1, admitted - 1)] {
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| lane.advance(&mut app.stores.store, &visibility, store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: items, maximum_capacity_bytes: bytes, ..grant }).unwrap());
                assert_eq!(step, store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
                assert_eq!(events.requested_bytes, 0);
                assert_eq!(events.released_bytes, 0);
                assert_eq!(lane.request().unwrap().mutations.mutations::<SurfaceMutation>().unwrap().as_ptr(), pointer);
            }
            apps.push(app);
            lanes.push(lane);
        }
        let mut turns = 0;
        let mut index = 0;
        while index < lanes.len() && stop.is_none_or(|stop| turns < stop) {
            lanes[index].advance(&mut *apps[index].store, &visibility, grant).unwrap();
            if lanes[index].prepared_for_staging() {
                let identity = lanes[index].prepared_edit_id().unwrap();
                assert!(!identity.is_empty());
                let pointer = identity.as_ptr();
                let prepared = lanes[index].prepared_publication().unwrap();
                assert_eq!(prepared.prepared_operation_count(false), Some(1));
                assert!(prepared.prepared_operation(false, 0).unwrap().is::<SurfaceMutation>());
                assert!(prepared.prepared_operation(false, 1).is_none());
                let operation_pointer = prepared.prepared_operation(false, 0).unwrap().downcast_ref::<SurfaceMutation>().unwrap() as *const SurfaceMutation;
                let (same_operation, operation_heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| prepared.prepared_operation(false, 0).unwrap().downcast_ref::<SurfaceMutation>().unwrap() as *const SurfaceMutation);
                assert_eq!(same_operation, operation_pointer);
                assert_eq!((operation_heap.requested_bytes, operation_heap.released_bytes), (0, 0));
                let (same_pointer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| lanes[index].prepared_edit_id().unwrap().as_ptr());
                assert_eq!(same_pointer, pointer);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                let paused = store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 0, ..grant };
                assert_eq!(lanes[index].advance(&mut apps[index].store, &visibility, paused).unwrap(), store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
                assert_eq!(lanes[index].prepared_edit_id().unwrap().as_ptr(), pointer);
            }
            assert_eq!(apps.iter().map(|app| app.stores.store.snapshot_ref().count).collect::<Vec<_>>(), vec![0; 3]);
            turns += 1;
            assert!(turns < 4096);
            if lanes[index].staged() { index += 1; }
        }
        if stop.is_none() {
            assert!(lanes.iter().all(PrivateOwnedPublicationLane::staged));
            assert!(lanes.len() <= corpus["source"]["maximumDeclaredChildren"].as_u64().unwrap() as usize + 1);
            let (_, decision_heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                assert!(owner.commit());
                for index in 0..3 {
                    assert!(matches!(lanes[index].adopt(&mut apps[index].store, &visibility, grant).unwrap(), store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(..)));
                    assert_eq!(apps[index].store.generation_now(), 1);
                    assert_eq!(apps[index].store.snapshot_ref().count, 1);
                }
            });
            assert_eq!((decision_heap.requested_bytes, decision_heap.released_bytes), (0, 0));
        } else { assert!(owner.abort()); }
        for index in 0..3 {
            for _ in 0..4096 {
                let demand = lanes[index].retirement_demands(grant.maximum_copy_bytes).unwrap();
                assert!(demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth);
                let exact = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
                let step = lanes[index].close_step(&mut *apps[index].store, exact).unwrap();
                assert!(step.progress().fits(exact));
                if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) { break; }
            }
            assert!(lanes[index].terminal_is_empty());
            assert_eq!(apps[index].store.snapshot_ref().count, i32::from(stop.is_none()));
        }
        drop(lanes);
        for mut app in apps { close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close"); }
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn child_emission_private_group_row_frames_preserve_original_sources_on_every_input_cancel() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    use crate::app_emission::{ChildEmit, ChildEmitGenesis};
    use crate::child_emit_preparation::OwnedChildEmit;
    use crate::child_content::{PrivateChildGroupSource, PrivateChildPublicationGroup};
    use semio_framework_artifact_reference::{ArtifactRef, ArtifactDialect};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 262_144, maximum_release_bytes: 262_144, maximum_depth: 64 };
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🌱️genesis/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for parent_touched in corpus["receipts"]["parentTouched"].as_array().unwrap().iter().map(|value| value.as_bool().unwrap()) {
    for stop in corpus["ownerInput"]["cancelStops"].as_array().unwrap().iter().map(|stop| stop.as_u64().unwrap() as usize) {
        let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
        let parent_values: Vec<i32> = if parent_touched { serde_json::from_value(corpus["ownerInput"]["parentMutations"].clone()).unwrap() } else { Vec::new() };
        let child_values: Vec<i32> = serde_json::from_value(corpus["ownerInput"]["childMutations"].clone()).unwrap();
        let (parent, _) = store::MemberStoreOwnedBatch::try_new(parent_values, grant).unwrap();
        let (child, _) = store::MemberStoreOwnedBatch::try_new(child_values, grant).unwrap();
        let target = &corpus["source"]["reference"];
        let metadata = ChildEmit { genesis: Some(ChildEmitGenesis { reference: ArtifactRef { artifact_id: target["artifactId"].as_str().unwrap().into(), dialect: ArtifactDialect { artifact_kind: target["dialect"]["artifactKind"].as_str().unwrap().into(), standard: target["dialect"]["standard"].as_str().unwrap().into(), subset: target["dialect"]["subset"].as_str().unwrap().into() } }, initial_pack: serde_json::to_vec(&corpus["ownerInput"]["initialPack"]).unwrap() }), owner: String::new(), slot: corpus["source"]["slot"].as_str().unwrap().into(), child_id: corpus["source"]["childId"].as_str().unwrap().into(), ops: Vec::new(), op_schema: semio_framework::kernel::SchemaId("private.owned".into()), labels: Vec::new() };
        let sources = PrivateChildGroupSource { parent, parent_touched, children: vec![OwnedChildEmit::new(metadata, child)], transaction: Some(protocol::TransactionRef { id: "private-neutral-three-lane".into(), tool: "neutral-owner".into() }), group_id: corpus["receipts"]["groupId"].as_str().unwrap().into() };
        let (mut group, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| match PrivateChildPublicationGroup::<store::NoMembers>::try_new(sources) { Ok(group) => group, Err(_) => panic!("one neutral original child is within the fixed bound") });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(group.child_count(), 1);
        assert_eq!(group.parent_touched(), parent_touched);
        assert!(group.prepared_operation_schema_parts(0, 0).is_none());
        assert!(group.prepared_operation_schema_parts(1, 0).is_none());
        assert!(PrivateChildPublicationGroup::<store::NoMembers>::frame_birth_bytes() <= 262_144);
        assert!(PrivateChildPublicationGroup::<store::NoMembers>::row_birth_bytes() <= 262_144);
        let dialect = ArtifactDialect { artifact_kind: "neutral.parent".into(), standard: "1".into(), subset: "*".into() };
        let private_genesis_actor = semio_framework_value::SharedUtf8::from("actor:private-genesis");
        for _ in 0..stop {
            let capacity = group.next_input_capacity_byte_demand(&app.child_composition.children, "neutral-parent-α", &dialect, &private_genesis_actor).unwrap();
            assert!(capacity <= grant.maximum_capacity_bytes);
            for denied in [Some(RetainedCloneGrant { maximum_items: 0, ..grant }), capacity.checked_sub(1).map(|maximum_capacity_bytes| RetainedCloneGrant { maximum_capacity_bytes, ..grant })].into_iter().flatten() {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| group.advance_inputs(&app.child_composition.children, "neutral-parent-α", &dialect, &private_genesis_actor, semio_framework_job::OperationId(7391), semio_framework_job::Generation(3), 1_000_000, 1, denied).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| group.advance_inputs(&app.child_composition.children, "neutral-parent-α", &dialect, &private_genesis_actor, semio_framework_job::OperationId(7391), semio_framework_job::Generation(3), 1_000_000, 1, grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        }
        let mut original_group_caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(7391),semio_framework_job::Generation(3),grant,||Some(1),1_000_000,1);
        let mut graph = app.child_composition.composition.graph_mut().await;
        for _ in 0..100_000 {
            if group.terminal_is_empty() { break; }
            let demand = group.retirement_demands(grant.maximum_copy_bytes).unwrap();
            assert!(demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth);
            let exact=grant;
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| original_group_caller.turn(exact,|same|semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(same,exact,demand,|incoming,loan|{let step=group.close_step(&mut app.stores.store,&mut app.child_composition.children,&mut graph,&app.child_composition.child_content_root,incoming,loan)?;Ok((semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(step.progress()),false))}).map_err(Into::into)).unwrap());
            assert!(step.progress().unwrap().fits(exact));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().unwrap().retained_capacity_bytes, step.progress().unwrap().released_bytes));
        }
        assert!(group.terminal_is_empty());
        drop(graph);drop(group);original_group_caller.retire(grant);
        close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");
    }
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}
