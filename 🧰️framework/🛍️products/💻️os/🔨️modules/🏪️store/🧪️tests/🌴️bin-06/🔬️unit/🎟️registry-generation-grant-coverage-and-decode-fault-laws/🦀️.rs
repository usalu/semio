/// 🎟️ Registry generation, grant coverage and decode fault laws: the live read factory admission, the shared grant coverage, the field close failure receipts, the original wallet after a child fault and the decode fault descriptor.

#[test]
fn live_read_factory_admission_preserves_exact_registry_generation_on_refusal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let policy = &fixture["grant"];
    let grant = RetainedCloneGrant { maximum_items: policy["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes: policy["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes: policy["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes: policy["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth: policy["maximumDepth"].as_u64().unwrap() as usize };
    let mut registry = Some(semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new());
    let original = Arc::new(17i32);
    let factory: Arc<dyn ArtifactOwnedValueRetirementFactory<i32>> = Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<i32>::default());
    let lease = registry.as_ref().unwrap().try_issue(Arc::clone(&original)).unwrap();
    let index = lease.slot_index();let generation = lease.slot_generation();
    let mut read = ErasedSnapshotRead::new(Arc::clone(&original),lease);
    let pointer = read.get::<i32>().unwrap() as *const i32;
    let birth = ReturnedSnapshotReadRetirement::<i32>::constructor_capacity_bytes();
    assert!(birth<=grant.maximum_capacity_bytes);
    for denied in [RetainedCloneGrant { maximum_items: 0,..grant },RetainedCloneGrant { maximum_capacity_bytes: birth-1,..grant },RetainedCloneGrant { maximum_depth: 0,..grant }] {
        let (result,allocated,released) = semio_framework_trace_memory_testing::observe_backing(||read.admit_retirement::<i32>(registry.as_ref().unwrap(),&factory,denied));
        assert!(result.is_err());assert_eq!((allocated,released),(0,0));
        assert_eq!(read.get::<i32>().unwrap() as *const i32,pointer);
        assert!(registry.as_ref().unwrap().contains(index,generation));
        assert_eq!(registry.as_ref().unwrap().occupied_count(),1);
    }
    let (result,allocated,released) = semio_framework_trace_memory_testing::observe_backing(||read.admit_retirement::<i32>(registry.as_ref().unwrap(),&factory,grant));
    let(owner,progress)=result.unwrap();assert_eq!((allocated,released),(birth,0));assert_eq!(progress,RetainedCloneProgress { copied_items: 1,retained_capacity_bytes: birth,..Default::default() });
    assert!(read.terminal_is_empty());assert!(!registry.as_ref().unwrap().contains(index,generation));assert_eq!(registry.as_ref().unwrap().occupied_count(),0);assert_eq!(Arc::as_ptr(&original),pointer);
    let mut owner=Some(owner);
    for _ in 0..32 {if owner.is_none(){break;}let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||artifact_retirement_box_close_step(&mut owner,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
    assert!(owner.is_none());
    let cleanup = RetainedCloneGrant { maximum_release_bytes: fixture["census"]["maximumCloseBytes"].as_u64().unwrap() as usize,..grant };
    let mut registry=registry.take().map(SnapshotReadRegistryAliasRetirement::new);
    for _ in 0..32 {if registry.is_none(){break;}snapshot_registry_alias_close_step(&mut registry,cleanup).unwrap();}
    assert!(registry.is_none());
}

#[test]
fn current_shared_grant_coverage_preserves_every_denied_axis() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let row = &fixture["grantCoverage"]; let d = &row["demand"];
    let demand = semio_framework_value::RetirementDemand { copy_bytes: d["copyBytes"].as_u64().unwrap() as usize, capacity_bytes: d["capacityBytes"].as_u64().unwrap() as usize, release_bytes: d["releaseBytes"].as_u64().unwrap() as usize, depth: d["depth"].as_u64().unwrap() as usize };
    let supplied = physical_test_close_grant(); assert!(artifact_retirement_grant_covers(supplied,demand));
    for row in row["denials"].as_array().unwrap() {
        let mut denied = supplied; let value = row["value"].as_u64().unwrap() as usize;
        match row["axis"].as_str().unwrap() { "maximumItems" => denied.maximum_items=value, "maximumCopyBytes" => denied.maximum_copy_bytes=value, "maximumCapacityBytes" => denied.maximum_capacity_bytes=value, "maximumReleaseBytes" => denied.maximum_release_bytes=value, "maximumDepth" => denied.maximum_depth=value, _ => panic!("unknown plain physical axis") }
        let (covered,born,freed)=semio_framework_trace_memory_testing::observe_backing(||artifact_retirement_grant_covers(denied,demand));assert!(!covered);assert_eq!((born,freed),(0,0));
    }
}

#[test]
fn field_close_failure_preserves_real_receipts_through_the_original_decode_job() {
    use semio_framework_job::InteractiveJob;
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    struct FailedField { owner: ControlledRetirement<Vec<u8>>, fail_once: bool }
    impl ArtifactEnvelopeFieldDecoder<(), ()> for FailedField {
        fn accept_field_token(&mut self, _:u16, _:OwnedSchemaToken, _:bool, _: &OwnedSchemaRecordCursor, _: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep,OwnedSchemaDecodeDiagnostic> { Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete) }
        fn finish_record(&mut self, _: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep,OwnedSchemaDecodeDiagnostic> { Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete) }
        fn maximum_close_byte_demand(&self)->usize { 65_536 }
        fn maximum_retained_close_bytes(&self)->usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES }
        fn next_close_copy_byte_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic> { Ok(self.owner.next_copy_byte_demand().unwrap()) }
        fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,OwnedSchemaDecodeDiagnostic> { Ok(self.owner.next_capacity_byte_demand(copy).unwrap()) }
        fn next_close_release_byte_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic> { Ok(self.owner.next_release_byte_demand().unwrap()) }
        fn next_close_depth_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic> { Ok(self.owner.next_depth_demand().unwrap()) }
        fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OwnedSchemaDecodeDiagnostic> {
            let step=self.owner.step(grant).unwrap();
            if std::mem::take(&mut self.fail_once) { return Err(OwnedSchemaDecodeDiagnostic { code:"field.close-after-original-birth",offset:0,line:0,column:0,path:OwnedSchemaPath::ROOT,retained_progress:step.progress(), refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated }); }
            Ok(step)
        }
        fn terminal_is_empty(&self)->bool { self.owner.terminal_is_empty() }
    }
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🚪️io/🚫️refusal/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(corpus["nativeLaw"],"field_close_failure_preserves_real_receipts_through_the_original_decode_job");
    let mut caller=OriginalEnvelopeCaller::admit_cleanup("failed-field");
    let mut cx=caller.context();
    let registry=ArtifactEnvelopeFieldDecoderRegistry::new();
    let field=FailedField { owner:ControlledRetirement::new(vec![7;1024]).unwrap(),fail_once:true };
    let mut job=ArtifactEnvelopeDecodeAuthority::<(),()>::try_new(artifact_envelope_decode_test_cursor(&[b"{}"]),&registry,Box::new(field)).unwrap_or_else(|_|panic!("original field authority admission"));
    let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65_536,maximum_release_bytes:65_536,maximum_depth:64};
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cx.with_retained_loan(grant,|cx|job.close_step(cx)));
    let progress=cx.retained_progress();
    let failure=step.expect_err("original failed field remains typed");assert_eq!(failure.primary.retained_progress(),progress);assert!(failure.receipt_refusal.is_none());
    assert!(allocated>0);assert_eq!(progress.retained_capacity_bytes,allocated);assert_eq!(progress.released_bytes,released);assert_eq!(progress.copied_items,1);assert!(progress.fits(grant));assert!(!job.terminal_is_empty());
    let mut detached=None;
    for _ in 0..4096 {drive_test_envelope_field_return(&registry,&mut detached);if job.terminal_is_empty(){break}let before=cx.retained_progress();let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||cx.with_retained_loan(grant,|cx|job.close_step(cx)).unwrap());assert_eq!(cx.retained_progress(),before.checked_add(step.progress()).unwrap());assert!(step.progress().fits(grant));assert_eq!((born,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
    assert!(job.terminal_is_empty());assert!(registry.terminal_is_empty());
    drop(cx);
    caller.retire(grant);
}

#[test]
fn normal_decode_release_records_original_wallet_even_after_child_fault(){
 use semio_framework_job::{InteractiveJob,StepContextOwner,StepBudget,OperationId,Generation};
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 struct WalletField{first:bool,old:Vec<u8>,owned:Option<ControlledRetirement<Vec<u8>>>}
 impl WalletField{fn demand(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if self.first{return Ok(semio_framework_value::RetirementDemand{copy_bytes:23,capacity_bytes:64,release_bytes:32,depth:3})}let owner=self.owned.as_ref().unwrap();Ok(semio_framework_value::RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}}
 impl ArtifactEnvelopeFieldDecoder<(),()>for WalletField{
  fn accept_field_token(&mut self,_:u16,_:OwnedSchemaToken,_:bool,_:&OwnedSchemaRecordCursor,_:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactEnvelopeFieldDecodeStep,OwnedSchemaDecodeDiagnostic>{Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)}
  fn finish_record(&mut self,_:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactEnvelopeFieldDecodeStep,OwnedSchemaDecodeDiagnostic>{Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete)}
  fn maximum_close_byte_demand(&self)->usize{65536}
  fn maximum_retained_close_bytes(&self)->usize{ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES}
  fn next_close_copy_byte_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic>{self.demand(0).map(|d|d.copy_bytes).map_err(|e|OwnedSchemaDecodeDiagnostic::before("wallet.copy",OwnedSchemaPath::ROOT).with_native(e))}
  fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,OwnedSchemaDecodeDiagnostic>{self.demand(copy).map(|d|d.capacity_bytes).map_err(|e|OwnedSchemaDecodeDiagnostic::before("wallet.capacity",OwnedSchemaPath::ROOT).with_native(e))}
  fn next_close_release_byte_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic>{self.demand(0).map(|d|d.release_bytes).map_err(|e|OwnedSchemaDecodeDiagnostic::before("wallet.release",OwnedSchemaPath::ROOT).with_native(e))}
  fn next_close_depth_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic>{self.demand(0).map(|d|d.depth).map_err(|e|OwnedSchemaDecodeDiagnostic::before("wallet.depth",OwnedSchemaPath::ROOT).with_native(e))}
  fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OwnedSchemaDecodeDiagnostic>{
   if !self.first{return self.owned.as_mut().unwrap().step(grant).map_err(|e|OwnedSchemaDecodeDiagnostic::before("wallet.cleanup",OwnedSchemaPath::ROOT).with_native(e))}
   if !artifact_retirement_grant_covers(grant,self.demand(grant.maximum_copy_bytes).unwrap()){return Ok(RetainedCloneStep::Progress(Default::default()))}
   let mut original=Vec::new();original.try_reserve_exact(64).unwrap();original.extend_from_slice(b"original field payload!");let freed=self.old.capacity();drop(std::mem::take(&mut self.old));let born=original.capacity();self.owned=Some(ControlledRetirement::new(original).unwrap());self.first=false;
   Err(OwnedSchemaDecodeDiagnostic::before("field.wallet-after-real-turn",OwnedSchemaPath::ROOT).with_progress(RetainedCloneProgress{copied_items:1,copied_bytes:23,retained_capacity_bytes:born,released_bytes:freed}))
  }
  fn terminal_is_empty(&self)->bool{!self.first&&self.owned.as_ref().is_some_and(ControlledRetirement::terminal_is_empty)}
 }
 let mut cleanup_caller=OriginalEnvelopeCaller::admit_cleanup("wallet-cleanup");let mut cleanup_cx=cleanup_caller.context();
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🚪️io/🚫️refusal/⏱️step/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let supplied:RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();let registry=ArtifactEnvelopeFieldDecoderRegistry::new();let mut job=ArtifactEnvelopeDecodeAuthority::<(),()>::try_new(artifact_envelope_decode_test_cursor(&[b"{}"]),&registry,Box::new(WalletField{first:true,old:vec![7;32],owned:None})).unwrap_or_else(|_|panic!("original wallet field admission"));job.begin_release(ArtifactEnvelopeDecodeState::ReleaseCancelled);
 let((mut owner,receipt),born,freed)=semio_framework_trace_memory_testing::observe_backing(||StepContextOwner::new(OperationId(98146),Generation(17),supplied).unwrap());assert!(receipt.fits(supplied));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(born,freed));let((cancel,receipt),born,freed)=semio_framework_trace_memory_testing::observe_backing(||semio_framework_job::CancelToken::admit_root(supplied).unwrap().unwrap());assert!(receipt.fits(supplied));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(born,freed));
 for axis in fixture["deniedAxes"].as_array().unwrap(){let mut grant=supplied;match axis.as_str().unwrap(){"maximumItems"=>grant.maximum_items=0,"maximumCopyBytes"=>grant.maximum_copy_bytes=0,"maximumCapacityBytes"=>grant.maximum_capacity_bytes=0,"maximumReleaseBytes"=>grant.maximum_release_bytes=0,"maximumDepth"=>grant.maximum_depth=0,_=>panic!("closed original grant axis")};let mut receipt=RetainedCloneProgress::default();let mut sequence=0;let mut cx=owner.context(StepBudget::new(7,1500,grant),cancel.clone(),||Some(1000),&mut sequence,&mut receipt).unwrap();cx.set_stage("wallet.heap-warmup");let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||job.step(&mut cx));assert!(matches!(step,Ok(None)),"denied axis {axis} answered {:?}",step.as_ref().map(|value|value.is_some()).map_err(|error|(&error.primary,&error.receipt_refusal)));assert_eq!((born,freed),(0,0),"denied axis {axis}");assert_eq!(cx.retained_progress(),Default::default());assert_eq!(cx.fuel_remaining(),7);assert_eq!(job.released_field_bytes(),0);assert!(matches!(job.state,ArtifactEnvelopeDecodeState::ReleaseCancelled));}
 let mut receipt=RetainedCloneProgress::default();let mut sequence=0;let mut cx=owner.context(StepBudget::new(7,1500,supplied),cancel.clone(),||Some(1000),&mut sequence,&mut receipt).unwrap();let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||job.step(&mut cx));let failure=step.expect_err("funded original field failure remains complete");assert_eq!((born,freed),(64,32));let field_receipt:RetainedCloneProgress=serde_json::from_value(fixture["receipt"].clone()).unwrap();assert_eq!(failure.primary.retained_progress(),field_receipt);assert!(failure.receipt_refusal.is_none()&&failure.context_refusal.is_none());assert_eq!(cx.retained_progress(),field_receipt,"the failed field posts once and publishes no yield outcome");assert_eq!(cx.operation(),OperationId(98146));assert_eq!(cx.generation(),Generation(17));assert_eq!(cx.deadline_us(),1500);assert_eq!(cx.fuel_remaining(),6);assert_eq!(job.released_field_bytes(),32);let ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic)=job.state else{panic!("real field fault remains attached")};assert_eq!(diagnostic.code,"field.wallet-after-real-turn");assert_eq!(diagnostic.retained_progress,field_receipt);drop(cx);
 let mut detached=None;for _ in 0..4096{drive_test_envelope_field_return(&registry,&mut detached);if job.terminal_is_empty(){break}let before=cleanup_cx.retained_progress();let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||cleanup_cx.with_retained_loan(supplied,|cx|job.close_step(cx)).unwrap());assert_eq!(cleanup_cx.retained_progress(),before.checked_add(step.progress()).unwrap());assert!(step.progress().fits(supplied));assert_eq!((born,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(job.terminal_is_empty());assert!(registry.terminal_is_empty());while !owner.terminal_is_empty(){let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(supplied));assert!(step.progress().fits(supplied));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(born,freed));}retire_original_envelope_cancel(cancel,supplied);drop(cleanup_cx);cleanup_caller.retire(supplied);
}

#[test]
fn decode_fault_descriptor_borrows_original_payload_until_paid_nested_close() {
    use semio_framework_job::{InteractiveJob, JobOutcomeKind, JobOutcomeView, StepContextOwner, StepBudget, OperationId, Generation};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../🚪️io/🚫️refusal/📬️outcome/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let wallet: serde_json::Value = serde_json::from_str(include_str!("../../../../🚪️io/🚫️refusal/⏱️step/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let original_grant: RetainedCloneGrant = serde_json::from_value(wallet["grant"].clone()).unwrap();
    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let mut job = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[b"{}"]), &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    ).unwrap_or_else(|_| panic!("original fault decoder admission"));
    let diagnostic = ArtifactEnvelopeDecodeAuthority::<(), ()>::diagnostic("field.wallet-after-real-turn");
    assert_eq!(diagnostic.code, law["diagnostic"].as_str().unwrap());
    job.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic));
    let((mut owner,receipt),born,freed)=semio_framework_trace_memory_testing::observe_backing(||StepContextOwner::new(OperationId(98146),Generation(17),original_grant).unwrap());assert!(receipt.fits(original_grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(born,freed));
    let mut sequence = 0;
    let mut detached = None;
    let mut descriptor = None;
    let((cancel,receipt),born,freed)=semio_framework_trace_memory_testing::observe_backing(||semio_framework_job::CancelToken::admit_root(original_grant).unwrap().unwrap());assert!(receipt.fits(original_grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(born,freed));
    for _ in 0..4096 {
        drive_test_envelope_field_return(&registry, &mut detached);
        let mut receipt = RetainedCloneProgress::default();
        let mut cx = owner.context(StepBudget::new(7, 1500, original_grant), cancel.clone(), || Some(1000), &mut sequence, &mut receipt).unwrap();
        let result = job.step(&mut cx).unwrap().map(|result| result.into_descriptor());
        assert!(cx.retained_progress().fits(original_grant));
        let Some(mut result) = result else { continue };
        if result.kind() == JobOutcomeKind::Fault {
            descriptor = Some(result);
            break;
        }
        assert_eq!(result.kind(), JobOutcomeKind::Yield);
        assert!(matches!(result.acknowledge(original_grant), RetainedCloneStep::Complete(_)));
    }
    let mut descriptor = descriptor.expect("original fault is published after exact source owners close");
    assert!(registry.terminal_is_empty());
    assert!(!job.terminal_is_empty());
    let original = job.fault.published().unwrap() as *const _;
    for _ in 0..2 {
        let (view, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| job.borrow_outcome(&descriptor).unwrap());
        let JobOutcomeView::Fault { detail, admission } = view else { panic!("exact fault descriptor") };
        assert_eq!((born, freed), (law["borrowHeapBytes"].as_u64().unwrap() as usize, 0));
        assert_eq!(detail as *const _, original);
        assert!(std::ptr::eq(admission, descriptor.admission()));
        assert_eq!(detail.single_page().unwrap(), law["diagnostic"].as_str().unwrap().as_bytes());
    }
    assert!(matches!(descriptor.acknowledge(original_grant), RetainedCloneStep::Complete(_)));
    drop(descriptor);
    let child_depth = job.fault.retirement_demands().unwrap().depth;
    assert_eq!(job.next_close_depth_demand().unwrap(), child_depth + law["parentDepth"].as_u64().unwrap() as usize);
    for denied in [RetainedCloneGrant { maximum_items: 0, ..original_grant }, RetainedCloneGrant { maximum_depth: child_depth, ..original_grant }] {
        let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(7,1500,denied),cancel.clone(),||Some(1000),&mut sequence,&mut receipt).unwrap();
        let (step, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| job.close_step(&mut cx).unwrap());assert_eq!(step.progress(),cx.retained_progress());
        assert_eq!((born, freed), (0, 0));
        assert_eq!(step.progress(), RetainedCloneProgress::default());
        assert_eq!(job.fault.published().unwrap() as *const _, original);
        assert!(!job.terminal_is_empty());
    }
    for _ in 0..4096 {
        if job.terminal_is_empty() { break; }
        let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(7,1500,original_grant),cancel.clone(),||Some(1000),&mut sequence,&mut receipt).unwrap();
        let (step, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| job.close_step(&mut cx).unwrap());assert_eq!(step.progress(),cx.retained_progress());
        assert!(step.progress().fits(original_grant));
        assert_eq!((step.progress().retained_capacity_bytes, step.progress().released_bytes), (born, freed));
    }
    assert!(job.terminal_is_empty());
    while !owner.terminal_is_empty() {
        let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(original_grant));assert!(step.progress().fits(original_grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(born,freed));
    }
    retire_original_envelope_cancel(cancel,original_grant);
}
