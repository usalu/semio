use super::*;
use semio_framework_pack_json::ArtifactCanonicalJsonNode;

#[test]
fn snapshot_publication_original_build_six_frontiers_preserve_native_pointers_and_paid_cancellation(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📦️assembly/🔣️.json")).unwrap();
    let grant=ArtifactStoreOneItemGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
    for stop in 0..=law["originalFrontiers"].as_array().unwrap().len(){
        let authority=Arc::new(ArtifactStoreOneItemLiveAuthority{operation:semio_framework_job::OperationId(1),generation:semio_framework_job::Generation(1),base_revision:[0;32],base_applied_edit_count:0,next_sequence_number:1,next_clock:crate::os_spr::HybridLogicalTimestamp::new(1,0),actor:"actor α\0😀".into(),line:None,group_id:None,stamped_edit_id:None});
        let((mutation,inverse),initial)=observe(||(BirthMutation{text:"forward α\0😀".repeat(8192)},vec![BirthMutation{text:"first inverse".repeat(8192)},BirthMutation{text:"second inverse".repeat(8192)}]));
        let forward_pointer=mutation.text.as_ptr();let inverse_pointer=inverse[0].text.as_ptr();
        let(result,source_birth)=observe(||RetainedCloneSource::admit_owned(mutation,SharedControlledRetirement::lease(Arc::clone(&authority)),grant.retained_grant()));let(source,_)=result.map_err(|(error,_,_)|error).unwrap();
        let mut owner=RetainedClonePreparation::<u64,BirthMutation,BirthEdit>{pending_base:None,pending_base_close:None,pending_mutation_close:None,pending_mutation:None,pending_mutation_authority:None,edit:None,source:None,clone_cursor:None,clone_handoff:None,copied:Some(7),edit_cursor:None,foreign_step_presence:None,inverse:Some(inverse),publication_post:None,publication_post_close:None,publication_inverse:None,publication_metadata:None,publication_metadata_close:None,publication_edit:None,publication_edit_close:None,build_stage:0,mutation:Some(source),authority:Some(Arc::clone(&authority)),sealer:None,mutation_retirement:None,snapshot_retirement:None,copied_close:None,inverse_close:None,authority_close:None,factory_close:None,footprint:ArtifactStoreOneItemFootprint{retained_bytes:usize::MAX,work_items:2},retained_capacity_bytes:0,maximum_depth:4096,checkpoint:Default::default(),ownership:Default::default(),seal_base:Default::default(),phase:RetainedClonePreparationPhase::Build,cancelled:false,closing:false};
        let mut births=source_birth.requested_bytes;let mut releases=source_birth.released_bytes;
        for _ in 0..100000{if owner.build_stage as usize==stop{break;}let stage=owner.build_stage;let(result,heap)=observe(||owner.build_sealer(ArtifactStoreOneItemGrant{maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0,..grant}).unwrap());assert!(!result);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.build_stage,stage);let(result,heap)=observe(||owner.build_sealer(grant).unwrap());assert!(result);let receipt=owner.ownership;assert!(receipt.fits(grant.retained_grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=heap.requested_bytes;releases+=heap.released_bytes;if let Some(source)=owner.mutation.as_ref(){if let Ok(original)=source.try_borrow(){assert_eq!(original.get().text.as_ptr(),forward_pointer);}}if let Some(original)=owner.pending_mutation.as_ref(){assert_eq!(original.text.as_ptr(),forward_pointer);}if let Some(inverse)=owner.publication_inverse.as_ref(){assert_eq!(inverse.remaining()[0].text.as_ptr(),inverse_pointer);}if let Some(post)=owner.publication_post.as_ref(){assert_eq!(**post,7);}}
        assert_eq!(owner.build_stage as usize,stop);owner.begin_close();
        for _ in 0..100000{if owner.terminal_is_empty(){break;}let(step,heap)=observe(||owner.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant.retained_grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=heap.requested_bytes;releases+=heap.released_bytes;}
        assert!(owner.terminal_is_empty());let(_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(releases,initial.requested_bytes+births);
    }
    eprintln!("[DEBUG] six actual original publication construction frontiers preserve forward/inverse native pointers; fixed4096 copy/capacity denial heap0; separately paid post/inverse frame closure and finalDrop0");
}

#[test]
fn lifecycle_fixture_is_schema_first() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/📦️lifecycle/🔣️.json")).expect("retained clone preparation fixture");
    assert_eq!(fixture["cases"].as_array().expect("lifecycle cases").len(), 6);
    assert!(fixture["largeCapacity"]["stringByteLength"].as_u64().expect("large string byte length") > fixture["grant"]["maximumBytes"].as_u64().expect("per-turn byte grant"));
    assert_eq!(fixture["largeCapacity"]["expectedCode"], "retained-clone.step-grant-too-small");
    let maximum = fixture["grant"]["maximumBytes"].as_u64().unwrap() as usize;
    for grant in [RetainedCloneGrant::one_capacity_turn(maximum, 64), RetainedCloneGrant::one_payload_turn(maximum, 64), RetainedCloneGrant::one_release_turn(maximum, 64)] {
        assert_eq!(grant.maximum_capacity_bytes + grant.maximum_copy_bytes + grant.maximum_release_bytes, fixture["grant"]["combinedCapacityCopyAndReleaseMaximum"].as_u64().unwrap() as usize);
    }
}

#[test]
fn snapshot_clone_original_inline_custody_observes_exact_granted_birth_work_and_release() {
    fn run<T: RetireOwned>(value: T) {
        let mut original = Some(value);
        let mut owner = None;
        let initial = pending::demands(&original, &owner, 0).unwrap();
        assert_eq!(initial.copy_bytes, size_of::<T>());
        let below = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: initial.copy_bytes.saturating_sub(1), maximum_depth: initial.depth, ..Default::default() };
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| pending::close(&mut original, &mut owner, below).unwrap().unwrap());
        assert_eq!(step.progress(), RetainedCloneProgress::default());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(original.is_some());
        for _ in 0..100_000 {
            if original.is_none() && owner.is_none() {
                let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner.take()));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                return;
            }
            let (demand, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                let body = pending::demands(&original, &owner, 0).unwrap().copy_bytes;
                pending::demands(&original, &owner, body).unwrap()
            });
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(demand.copy_bytes + demand.capacity_bytes + demand.release_bytes <= 4096);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (zero, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| pending::close(&mut original, &mut owner, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap().unwrap());
            assert_eq!(zero.progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| pending::close(&mut original, &mut owner, grant).unwrap().unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            if grant.maximum_copy_bytes > 0 && grant.maximum_release_bytes == 0 { assert_eq!(heap.released_bytes, 0); }
        }
        panic!("snapshot clone original inline custody did not close under exact demands");
    }
    run(7u64);
    run("original\0Ä🧩".to_string());
    run((0..33).collect::<Vec<u32>>());
    println!("[DEBUG] snapshot clone original scalar/text/sequence retains undergrant and closes under exact separate work/birth/release/depth");
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct BirthEdit;
struct BirthCursor;
struct BirthMutation { text: String }
semio_framework_value::artifact_retire_struct!(BirthMutation { text });
impl ArtifactCanonicalJson for BirthMutation {}
impl ArtifactCanonicalJsonTree for BirthMutation {
    fn canonical_tree_node(&self)->Result<ArtifactCanonicalJsonNode<'_>,ValueError>{Ok(ArtifactCanonicalJsonNode::Object(1))}
    fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{if ordinal==0{Ok(&self.text)}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original birth mutation has one field"))}}
    fn canonical_tree_key(&self,ordinal:usize)->Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>,ValueError>{if ordinal==0{Ok("text".into())}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original birth mutation has one key"))}}
}
static BIRTH_PREFLIGHT_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
impl RetainedCloneEdit<u64, BirthMutation> for BirthEdit {
    type Cursor = BirthCursor;
    fn preflight(&self, _: &BirthMutation, _: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        BIRTH_PREFLIGHT_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 16384 })
    }
    fn begin_demand(&self) -> RetainedCloneBirthDemand { RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 } }
    fn snapshot_cursor_birth_demand(&self) -> RetainedCloneBirthDemand { self.begin_demand() }
    fn begin(&self, grant: RetainedCloneGrant) -> Result<(BirthCursor, RetainedCloneProgress), ValueError> { Ok((BirthCursor, self.begin_demand().admit(grant)?)) }
}
impl RetainedCloneEditCursor<u64, BirthMutation> for BirthCursor {
    fn foreign_step_presence(&self)->bool{false}
    fn advance(&mut self, _: RetainedCloneRef<'_, u64>, _: &mut u64, _: RetainedCloneRef<'_, BirthMutation>, _: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> { unreachable!() }
    fn take_inverse(&mut self) -> Option<Vec<BirthMutation>> { None }
    fn cancel(&mut self) {}
    fn begin_close(&mut self) -> bool { true }
    fn close_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn terminal_is_empty(&self) -> bool { true }
}

#[test]
fn snapshot_clone_preparation_birth_returns_original_request_before_domain_work() {
    use crate::os_store::{SnapshotReadRegistryHandle, ArtifactStoreOneItemLiveAuthority};
    use semio_framework_trace::observe_heap_allocations_on_this_thread;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎟️preparation-birth/🔣️.json")).unwrap();
    BIRTH_PREFLIGHT_CALLS.store(0, std::sync::atomic::Ordering::Relaxed);
    let factory = RetainedClonePreparationFactory::new(Arc::new(BirthEdit), Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<BirthMutation>::default()), Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::<u64>::default()), 64).unwrap();
    let registry = SnapshotReadRegistryHandle::new();
    let root = Arc::new(7u64);
    let lease = registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_| panic!("original registry admission"));
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority { operation: semio_framework_job::OperationId(1), generation: semio_framework_job::Generation(3), base_revision: [0;32], base_applied_edit_count: 0, next_sequence_number: 1, next_clock: crate::os_spr::HybridLogicalTimestamp::new(1, 0), actor: "original actor".into(), line: None, group_id: None, stamped_edit_id: None });
    let (mutation, original_heap) = observe_heap_allocations_on_this_thread(|| BirthMutation { text: "original α\0😀".into() });
    let mutation_pointer = mutation.text.as_ptr();
    let mut request = ArtifactStoreOneItemPreparationRequest { operation: semio_framework_job::OperationId(1), generation: semio_framework_job::Generation(3), base_revision: [0;32], lane: HistoryLane::Document, authority: Arc::clone(&authority), base: SnapshotRead::new(Arc::clone(&root), lease), mutation, mutation_retirement:Arc::clone(&factory.mutation_retirement),snapshot_retirement:Arc::clone(&factory.snapshot_retirement) };
    let (demand, heap) = observe_heap_allocations_on_this_thread(|| factory.begin_demand(&request.mutation, request.lane).unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
    assert_eq!(demand.capacity_bytes, size_of::<RetainedClonePreparation<u64, BirthMutation, BirthEdit>>());
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 3, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: 97, maximum_depth: 7 };
    for currency in law["refusals"].as_array().unwrap() {
        let denied = match currency.as_str().unwrap() { "items" => ArtifactStoreOneItemGrant { maximum_items: 0, ..grant }, "capacity" => ArtifactStoreOneItemGrant { maximum_capacity_bytes: demand.capacity_bytes-1, ..grant }, "depth" => ArtifactStoreOneItemGrant { maximum_depth: 0, ..grant }, _ => unreachable!() };
        let (result, heap) = observe_heap_allocations_on_this_thread(|| factory.begin(request, denied));
        request = result.err().unwrap().1;
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
        assert_eq!(request.mutation.text.as_ptr(), mutation_pointer);
        assert!(std::ptr::eq(request.base.get(), root.as_ref()));
        assert!(Arc::ptr_eq(&request.authority, &authority));
        assert_eq!(BIRTH_PREFLIGHT_CALLS.load(std::sync::atomic::Ordering::Relaxed), law["preflight"]["afterRefusalCalls"].as_u64().unwrap() as usize);
    }
    let (result, heap) = observe_heap_allocations_on_this_thread(|| factory.begin(request, grant));
    let (mut owner, birth) = result.map_err(|(error,_)| error).unwrap();
    assert!(birth.fits(grant.retained_grant()));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (demand.capacity_bytes,0));
    assert_eq!(birth.retained_capacity_bytes, demand.capacity_bytes);
    assert_eq!((birth.copied_bytes, birth.released_bytes), (0,0));
    assert_eq!(BIRTH_PREFLIGHT_CALLS.load(std::sync::atomic::Ordering::Relaxed), law["preflight"]["afterBirthCalls"].as_u64().unwrap() as usize);
    owner.begin_close();
    let mut births = birth.retained_capacity_bytes;
    let mut released = 0;
    for _ in 0..10000 {
        if owner.terminal_is_empty() { break; }
        let copy = owner.next_close_copy_byte_demand().unwrap();
        let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: owner.next_close_release_byte_demand().unwrap(), maximum_depth: owner.next_close_depth_demand().unwrap() };
        let (step, heap) = observe_heap_allocations_on_this_thread(|| owner.close_step(grant).unwrap());
        assert!(step.progress().fits(grant.retained_grant()));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        births += heap.requested_bytes;
        released += heap.released_bytes;
    }
    assert!(owner.terminal_is_empty());
    let frame_grant = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: demand.capacity_bytes, maximum_depth: 1, ..Default::default() };
    let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,frame_grant.maximum_release_bytes));
    released += heap.released_bytes;
    assert_eq!(released, original_heap.requested_bytes+births);
    eprintln!("[DEBUG] original preparation request preserved on all birth refusals; frame={} original={} fundedBirths={births} released={released} preflight=1", demand.capacity_bytes, original_heap.requested_bytes);
}
