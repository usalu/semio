use super::*;
use std::fs::File;
use semio_framework_job::StepContext;
use semio_framework_job::OriginalTurnFailure;

fn fixture_io_policy() -> &'static (RetainedCloneGrant, u64, u64, u64) {
    static POLICY: std::sync::OnceLock<(RetainedCloneGrant, u64, u64, u64)> = std::sync::OnceLock::new();
    POLICY.get_or_init(|| { let law: serde_json::Value = serde_json::from_str(include_str!("../../🚪️native-io/🎟️original/🔣️.json")).unwrap(); let source = &law["source"]; (serde_json::from_value(source["grant"].clone()).unwrap(), source["fuel"].as_u64().unwrap(), source["stepFuel"].as_u64().unwrap(), source["maximumStepMicroseconds"].as_u64().unwrap()) })
}
fn fixture_native_io_grant() -> RetainedCloneGrant { fixture_io_policy().0 }
fn fixture_io_clock() -> Option<u64> { Some(0) }

struct OriginalIoFixture { operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, caller: Option<semio_framework_job::OriginalJobFixtureCaller>, decoder: semio_framework_job::OriginalJobDecodeOwner, origin:semio_framework_job::OriginalJobReturnOrigin, returned:Option<semio_framework_job::OriginalJobOutcome>, continuation:Option<semio_framework_job::OriginalJobContinuation>, step_receipt:RetainedCloneProgress }
impl OriginalIoFixture {
    fn new() -> Self { let operation=semio_framework_job::allocate_operation_id();let generation=semio_framework_job::Generation(1);let mut original=Self{operation,generation,caller:Some(semio_framework_job::OriginalJobFixtureCaller::admit(operation,generation,fixture_native_io_grant(),fixture_io_clock,fixture_io_policy().3,fixture_io_policy().1)),decoder:semio_framework_job::OriginalJobDecodeOwner::empty(),origin:semio_framework_job::OriginalJobReturnOrigin::empty(),returned:None,continuation:None,step_receipt:Default::default()};original.prepare_step_origin().unwrap();original }
    fn turn<R:Copy>(&mut self, run: impl FnOnce(&mut StepContext<'_>) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> { self.limited(fixture_native_io_grant(), run) }
    fn limited<R:Copy>(&mut self, grant: RetainedCloneGrant, run: impl FnOnce(&mut StepContext<'_>) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> { self.caller.as_mut().unwrap().turn_with_decoder(grant, &mut self.decoder, run) }
    fn prepare_step_origin(&mut self)->Result<(),OriginalTurnFailure>{
        for _ in 0..16384{if self.origin.body_ready(){return Ok(())}let origin=&mut self.origin;self.caller.as_mut().unwrap().turn_with_decoder(fixture_native_io_grant(),&mut self.decoder,|cx|origin.admit(cx))?;}assert!(self.origin.body_ready());Ok(())
    }
    fn record_step_receipt(&mut self,before:RetainedCloneProgress){let after=self.caller.as_ref().unwrap().cumulative_receipt();self.step_receipt=RetainedCloneProgress{copied_items:after.copied_items.checked_sub(before.copied_items).unwrap(),copied_bytes:after.copied_bytes.checked_sub(before.copied_bytes).unwrap(),retained_capacity_bytes:after.retained_capacity_bytes.checked_sub(before.retained_capacity_bytes).unwrap(),released_bytes:after.released_bytes.checked_sub(before.released_bytes).unwrap()};}
    fn step(&mut self,job:&mut NativeIoJob)->Result<Option<semio_framework_job::JobOutcomeKind>,OriginalTurnFailure>{self.step_limited(fixture_native_io_grant(),job)}
    fn step_limited(&mut self,grant:RetainedCloneGrant,job:&mut NativeIoJob)->Result<Option<semio_framework_job::JobOutcomeKind>,OriginalTurnFailure>{
        let before=self.caller.as_ref().unwrap().cumulative_receipt();
        if !self.origin.body_ready(){let origin=&mut self.origin;let ready=self.caller.as_mut().unwrap().turn_with_decoder(grant,&mut self.decoder,|cx|origin.admit(cx))?;if !ready{self.record_step_receipt(before);return Ok(None)}}
        let origin=&mut self.origin;let returned=&mut self.returned;let continuation=&mut self.continuation;
        self.caller.as_mut().unwrap().turn_with_decoder(grant,&mut self.decoder,|cx|{let mut verdict=None;match semio_framework_job::drive_step(job,cx,origin.receiver()?,"native_io_original_fixture",semio_framework_job::InteractiveStage::InteractiveStep,&mut verdict)?{semio_framework_job::OriginalJobReturn::Preparing|semio_framework_job::OriginalJobReturn::Pending=>{},semio_framework_job::OriginalJobReturn::Outcome(outcome)=>*returned=Some(outcome.into_descriptor()),semio_framework_job::OriginalJobReturn::Continuation(capsule)=>*continuation=Some(capsule)}Ok(())})?;
        for _ in 0..16384{if self.returned.is_none(){break}let origin=&mut self.origin;let returned=&mut self.returned;self.caller.as_mut().unwrap().turn_with_decoder(grant,&mut self.decoder,|cx|origin.receive_outcome(cx,returned))?;}assert!(self.returned.is_none());
        for _ in 0..16384{if self.continuation.is_none(){break}let origin=&mut self.origin;let continuation=&mut self.continuation;self.caller.as_mut().unwrap().turn_with_decoder(grant,&mut self.decoder,|cx|origin.receive_continuation(cx,continuation))?;}assert!(self.continuation.is_none());assert!(self.origin.continuation().is_none(),"native fixture retained an unexpected original continuation");
        let kind=self.origin.outcome().map(|outcome|outcome.kind());if kind.is_some(){let mut acknowledged=false;for _ in 0..16384{acknowledged|=self.origin.outcome().is_some_and(|outcome|outcome.is_acknowledged());let origin=&mut self.origin;if self.caller.as_mut().unwrap().turn_with_decoder(grant,&mut self.decoder,|cx|origin.acknowledge_outcome(cx))?{break}}assert!(acknowledged);assert!(self.origin.outcome().is_none());}
        self.record_step_receipt(before);Ok(kind)
    }
    fn close_job(&mut self, job: &mut NativeIoJob) {
        job.begin_close();
        for _ in 0..16384 { if job.terminal_is_empty() { return; } self.turn(|cx| job.close_step(cx)).unwrap(); }
        assert!(job.terminal_is_empty());
    }
    fn close_value(&mut self, value: &mut NativeIoValue) {
        for _ in 0..16384 { if value.terminal_is_empty() { return; } self.turn(|cx| { let demand = value.retirement_demands()?; semio_framework_job::close_original_retained_step(cx, demand, |grant| { let turn=value.close_step(grant); semio_framework_value::RetainedWorkOutcome { work:turn.work,result:turn.result.map(|step| match step { InteractiveJobCloseStep::Pending { progress } | InteractiveJobCloseStep::Complete { progress } => semio_framework_value::RetainedCloneStep::Progress(progress), InteractiveJobCloseStep::Blocked => semio_framework_value::RetainedCloneStep::Progress(Default::default()) }) } })?; Ok(()) }).unwrap(); }
        assert!(value.terminal_is_empty());
    }
}
impl Drop for OriginalIoFixture {
    fn drop(&mut self) {
        if std::thread::panicking() { return; }
        let caller = self.caller.as_mut().unwrap();
        for _ in 0..16384{if let Some(_)=self.returned.as_ref(){let origin=&mut self.origin;let returned=&mut self.returned;caller.turn(fixture_native_io_grant(),|cx|origin.receive_outcome(cx,returned)).unwrap();continue}if let Some(_)=self.continuation.as_ref(){let origin=&mut self.origin;let continuation=&mut self.continuation;caller.turn(fixture_native_io_grant(),|cx|origin.receive_continuation(cx,continuation)).unwrap();continue}if self.origin.terminal_is_empty(){break}caller.turn(fixture_native_io_grant(),|cx|self.origin.close_step(cx)).unwrap();}
        assert!(self.returned.is_none()&&self.continuation.is_none()&&self.origin.terminal_is_empty());
        for _ in 0..16384 { if self.decoder.terminal_is_empty() { break; } caller.turn(fixture_native_io_grant(), |cx| { let demand = self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?; self.decoder.close_on_source(cx)?; Ok(()) }).unwrap(); }
        assert!(self.decoder.terminal_is_empty());
        self.caller.take().unwrap().retire(fixture_native_io_grant());
    }
}

fn payload_vec(mut payload: semio_framework_job_payload::RetainedJobPayload, fixture: &mut OriginalIoFixture) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(payload.len());
    let mut reader = payload.reader();
    while let Some(page) = reader.read_page(1, semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES) { bytes.extend_from_slice(page); }
    for _ in 0..16384 { if payload.terminal_is_empty() { return bytes; } fixture.turn(|cx| { let demand = payload.retirement_demands()?; semio_framework_job::close_original_retained_step(cx, demand, |grant| payload.close_step(grant))?; Ok(()) }).unwrap(); }
    panic!("original native payload did not finish same-source close");
}

enum TestNativeIoValue {
    Other,
    Bytes(Vec<u8>),
    Page { bytes: Vec<u8>, eof: bool },
    Paths(Vec<PathBuf>),
    Modified(Vec<(PathBuf, std::time::SystemTime)>),
}

fn materialize_test_value(value: NativeIoValue, fixture: &mut OriginalIoFixture) -> TestNativeIoValue {
    match value {
        NativeIoValue::Bytes(bytes) => TestNativeIoValue::Bytes(payload_vec(bytes, fixture)),
        NativeIoValue::Page { bytes, eof } => TestNativeIoValue::Page { bytes: payload_vec(bytes, fixture), eof },
        NativeIoValue::Paths(paths) => { let owned = paths.as_slice().to_vec(); fixture.close_value(&mut NativeIoValue::Paths(paths)); TestNativeIoValue::Paths(owned) }
        NativeIoValue::Modified(entries) => { let owned = entries.as_slice().to_vec(); fixture.close_value(&mut NativeIoValue::Modified(entries)); TestNativeIoValue::Modified(owned) }
        NativeIoValue::ResidentBytes(_) | NativeIoValue::StreamRefused(_) => TestNativeIoValue::Other,
    }
}

fn run(request: NativeIoRequest) -> Result<TestNativeIoValue, String> {
    let mut fixture = OriginalIoFixture::new();
    let grant = fixture_native_io_grant();
    let mut job = Some(NativeIoJob::new(request));
    let operation = fixture.operation;
    let generation = fixture.generation;
    let mut params = Some(semio_framework_job_worker_authority::BatchJobParams { operation, generation, cancel: fixture.caller.as_mut().unwrap().admit_alias(grant), config: semio_framework_job_worker_authority::BatchDriveConfig { retained: grant, site: "native_io_test", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: fixture_io_policy().2, step_budget_us: fixture_io_policy().3 }, now_us: fixture_io_clock });
    let mut session = None;let mut admission=semio_framework_job_worker::WorkerJobSessionAdmission::<NativeIoJob,semio_framework_job_worker::BatchJobSession<NativeIoJob>>::new();
    for _ in 0..16384 { fixture.turn(|cx|admission.advance(&mut job,&mut params,&mut session,cx)).unwrap();if session.is_some(){break} }
    assert!(admission.terminal_is_empty());assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(admission)).1.released_bytes,0);
    assert!(job.is_none() && params.is_none());
    let mut session = session.expect("original native I/O session admission");
    let result;
    let mut turns = 0;let mut observed_buffer=None;
    loop {
        turns += 1;
        assert!(turns < 16384, "native I/O never published its terminal original outcome");
        fixture.turn(|cx| session.step_on_original_caller(cx)).unwrap();
        assert!(session.checkout_outcome(), "exact original native I/O outcome checkout");
        let (issued, posted) = session.take_checked_out_retained_step_receipt().expect("actual original caller step receipt");
        assert!(posted.fits(issued));
        if let Some(job)=session.checked_out_job_mut(){if let Some(buffer)=job.read_buffer.as_deref(){let pointer=buffer as*const _ as usize;if let Some(original)=observed_buffer{assert_eq!(pointer,original,"actual parent moved its stable read backing");}else{observed_buffer=Some(pointer);}}}
        let Some(terminal) = session.checked_out_outcome().unwrap().map(|view| view.is_terminal()) else { session.resume().unwrap(); continue; };
        for _ in 0..16384 {
            let done = fixture.turn(|cx| { let _demand=session.retirement_demands(cx.retained_grant().maximum_copy_bytes).expect("exact original outcome acknowledgement quote");session.acknowledge_outcome(cx) }).unwrap();
            if done { break; }
        }
        if terminal {
            let job = session.checked_out_job_mut().expect("same original terminal job");
            if matches!(job.result.as_ref(),Some(Ok(NativeIoValue::Bytes(bytes)|NativeIoValue::Page{bytes,..}))if bytes.len()>0){assert!(observed_buffer.is_some());}
            result = match job.result.as_ref() { Some(Err(error)) => Err(error.clone()), Some(Ok(_)) => Ok(materialize_test_value(job.take_result().unwrap().unwrap(), &mut fixture)), None => panic!("native terminal owner lost structured result") };
            break;
        }
        session.resume().unwrap();
    }
    session.begin_close();
    for _ in 0..16384 {
        if session.terminal_is_empty() { return result; }
        fixture.turn(|cx| {
            if session.has_original_cancel_alias_witness(cx.original_cancel_token()).unwrap() {
                let turn=session.return_original_cancel_alias_step(cx.original_cancel_token(),cx.retained_grant());let result=turn.result.map(|step|step.expect("same original witnessed cancellation alias")).map_err(|error|match error{semio_framework_job_worker::WorkerJobDemandError::Refused(error)=>error,semio_framework_job_worker::WorkerJobDemandError::Contention(_)=>semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original native session alias remains checked out")});semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work:turn.work})?;
            } else { session.close_step(cx)?; }
            Ok(())
        }).unwrap();
    }
    panic!("original native I/O mounted session did not finish paid close");
}

#[test]
fn request_and_completion_are_send() {
    fn assert_send<T: Send>() {}
    assert_send::<NativeIoJob>();
}

#[test]
fn original_scan_extension_closes_only_owned_capacity_before_and_after_start(){
 let root=std::env::temp_dir().join(format!("semio-original-extension-{}",std::process::id()));std::fs::create_dir_all(&root).unwrap();
 for started in[false,true]{for owned in[false,true]{
  let mut text=String::with_capacity(96);text.push_str("wasm");let extension=if owned{std::borrow::Cow::Owned(text)}else{drop(text);std::borrow::Cow::Borrowed("wasm")};let capacity=extension_capacity(&extension);let mut fixture=OriginalIoFixture::new();let mut job=NativeIoJob::new(NativeIoRequest::ScanDirectory{path:root.clone(),directories_only:false,extension:Some(extension),first_only:true});
  if started{assert!(fixture.step(&mut job).unwrap().is_none());assert!(matches!(job.state,NativeIoState::Scanning{..}));fixture.turn(|cx|job.close_step(cx)).unwrap();assert!(job.request_path.is_none());}
  let(target,demand)=job.close_target().unwrap();assert!(matches!(target,NativeIoCloseTarget::Extension|NativeIoCloseTarget::RequestExtension));assert_eq!(demand.release_bytes,capacity);
  if owned{let denied=RetainedCloneGrant{maximum_release_bytes:capacity-1,..fixture_native_io_grant()};let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.limited(denied,|cx|job.close_step(cx)));assert!(matches!(answer.unwrap(),InteractiveJobCloseStep::Pending{progress}if progress==Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(job.close_target().unwrap().1.release_bytes,capacity);}
  let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.turn(|cx|job.close_step(cx)));answer.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));assert_eq!(fixture.caller.as_ref().unwrap().receipt().released_bytes,capacity);assert!(job.request_extension.is_none());if let NativeIoState::Pending(NativeIoRequest::ScanDirectory{extension,..})=&job.state{assert!(extension.is_none());}fixture.close_job(&mut job);assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job)).1.released_bytes,0);
 }}std::fs::remove_dir(root).unwrap();
}

#[test]
fn original_selected_path_keeps_exact_directory_backing_until_paid_supplier_return(){
 static RETURNS:OriginalNativePathReturnRegistry<1>=OriginalNativePathReturnRegistry::with_return_wake(||{});
 let root=std::env::temp_dir().join(format!("semio-original-selected-{}",std::process::id()));std::fs::create_dir_all(&root).unwrap();let expected=root.join("selected.wasm");std::fs::write(&expected,b"selected").unwrap();let mut fixture=OriginalIoFixture::new();let mut job=NativeIoJob::new(NativeIoRequest::ScanDirectory{path:root.clone(),directories_only:false,extension:Some(std::borrow::Cow::Borrowed("wasm")),first_only:true});
 for _ in 0..64{fixture.step(&mut job).unwrap();if job.result_status().is_some(){break}}
 let Some(Ok(NativeIoValue::Paths(paths)))=job.result.as_ref()else{panic!("actual directory result")};let path=paths.as_slice().last().unwrap();let pointer=path.as_os_str().as_encoded_bytes().as_ptr();let capacity=path.capacity();let mut reservation=None;for turn in 0..16{if fixture.turn(|cx|RETURNS.reserve(&mut reservation,cx)).unwrap(){break}assert!(turn<15)}let mut reservation=reservation.unwrap();let mut output=None;
 let mut foreign=OriginalIoFixture::new();assert!(foreign.turn(|cx|job.supply_result_last_path(&mut reservation,&mut output,cx)).is_err());assert!(output.is_none());assert_eq!(job.result_status(),Some(NativeIoResultStatus::Paths{count:1}));
 assert!(!fixture.limited(RetainedCloneGrant{maximum_items:0,..fixture_native_io_grant()},|cx|job.supply_result_last_path(&mut reservation,&mut output,cx)).unwrap());assert!(output.is_none());assert!(!fixture.turn(|cx|job.supply_result_last_path(&mut reservation,&mut output,cx)).unwrap());assert!(output.is_none());assert_eq!(job.result_status(),Some(NativeIoResultStatus::Paths{count:0}));assert!(fixture.turn(|cx|job.supply_result_last_path(&mut reservation,&mut output,cx)).unwrap());assert!(fixture.turn(|cx|job.supply_result_last_path(&mut reservation,&mut output,cx)).unwrap());drop(reservation);let output=output.unwrap();let identity=output.source_identity();assert_eq!(output.path(),expected.as_path());assert_eq!(output.path().as_os_str().as_encoded_bytes().as_ptr(),pointer);fixture.close_job(&mut job);assert!(RETURNS.retains_source(identity));assert!(!RETURNS.has_returned_source(identity));
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(output));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(RETURNS.has_returned_source(identity));for _ in 0..2{let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.limited(RetainedCloneGrant{maximum_items:0,..fixture_native_io_grant()},|cx|cx.with_original_work_loan(0,cx.original_absolute_deadline_us(),|cx|{let work=cx.original_work_units_spent();let step=RETURNS.close_returned_step(cx)?;assert_eq!(step.progress().copied_items,1);assert_eq!(cx.original_work_units_spent()-work,1);assert_eq!(cx.fuel_remaining(),0);Ok(step)})));answer.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(RETURNS.retains_source(identity));}let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.limited(RetainedCloneGrant{maximum_release_bytes:capacity-1,..fixture_native_io_grant()},|cx|RETURNS.close_returned_step(cx)));assert!(matches!(answer.unwrap(),InteractiveJobCloseStep::Pending{progress}if progress==Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.turn(|cx|RETURNS.close_returned_step(cx)));answer.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));assert_eq!(fixture.caller.as_ref().unwrap().receipt().released_bytes,capacity);for _ in 0..16{if !RETURNS.retains_source(identity){break}fixture.turn(|cx|RETURNS.close_returned_step(cx)).unwrap();}assert!(!RETURNS.retains_source(identity));assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job)).1.released_bytes,0);std::fs::remove_file(expected).unwrap();std::fs::remove_dir(root).unwrap();
}

#[test]
fn original_stream_extent_refuses_oversize_before_payload_birth(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🧵️job/📬️outcome/📮️return/🔣️.json")).unwrap();
    let mut fixture=OriginalIoFixture::new();
    for row in law["cases"].as_array().unwrap(){let length=row["bytes"].as_u64().unwrap();let accepted=row["accepted"].as_bool().unwrap();let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.turn(|cx|{let bytes=row["stockBytes"].as_u64().unwrap()as usize;let total=cx.retained_grant();cx.bind_original_retained_total(RetainedCloneGrant{maximum_capacity_bytes:bytes.min(total.maximum_capacity_bytes),maximum_release_bytes:bytes.min(total.maximum_release_bytes),..total})?;Ok(validate_original_stream_extent(length,cx))}));assert_eq!(answer.unwrap().is_ok(),accepted);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(fixture.caller.as_ref().unwrap().receipt(),Default::default());}
    let path=std::env::temp_dir().join(format!("semio-native-stream-ceiling-{}",std::process::id()));let length=semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES as u64+1;File::create(&path).unwrap().set_len(length).unwrap();
    for page in[false,true]{let request=if page{NativeIoRequest::ReadPage{path:path.clone(),offset:0,max_bytes:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES}}else{NativeIoRequest::ReadBytes(path.clone())};let mut job=NativeIoJob::new(request);let pointer=job.retained_request_backing_identity().unwrap();for turn in 0..16384{if page&&matches!(job.state,NativeIoState::ReadingPage{..})||!page&&job.result.is_some(){break}fixture.step(&mut job).unwrap();assert_eq!(fixture.step_receipt.retained_capacity_bytes,0);assert!(turn<16383);}if page{assert!(matches!(job.state,NativeIoState::ReadingPage{..}));assert!(job.result.is_none());}else{assert!(matches!(job.result,Some(Ok(NativeIoValue::StreamRefused(NativeIoStreamRefusal{length:actual,maximum_bytes})))if actual==length&&maximum_bytes==semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES as u64));assert!(matches!(job.state,NativeIoState::Finished));}assert!(job.fault_writer.is_none()&&job.fault_detail.is_none());assert_eq!(fixture.caller.as_ref().unwrap().receipt().retained_capacity_bytes,0);assert_eq!(job.retained_request_backing_identity(),Some(pointer));fixture.close_job(&mut job);}
    std::fs::remove_file(path).unwrap();
}

#[test]
fn original_page_restart_keeps_one_source_path_and_stock_through_result_retirement(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🧵️job/📬️outcome/📮️return/🔣️.json")).unwrap();let restart=&law["restart"];let mut source=vec![0u8;restart["fileBytes"].as_u64().unwrap()as usize];for(index,byte)in source.iter_mut().enumerate(){*byte=(index%251)as u8;}
    let path=std::env::temp_dir().join(format!("semio-native-page-restart-{}",std::process::id()));std::fs::write(&path,&source).unwrap();let mut job=NativeIoJob::new(NativeIoRequest::ReadPage{path:path.clone(),offset:0,max_bytes:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES});let pointer=job.retained_request_backing_identity().unwrap();let mut fixture=OriginalIoFixture::new();let identity=fixture.turn(|cx|Ok((cx.operation(),cx.generation(),cx.payload_authority_identity(),cx.original_source_parent()?.source_identity()?))).unwrap();
    for(index,offset)in restart["offsets"].as_array().unwrap().iter().enumerate(){let offset=offset.as_u64().unwrap();
        if index!=0{for denied in[RetainedCloneGrant{maximum_items:0,..fixture_native_io_grant()},RetainedCloneGrant{maximum_copy_bytes:0,..fixture_native_io_grant()},RetainedCloneGrant{maximum_depth:0,..fixture_native_io_grant()}]{let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.limited(denied,|cx|job.restart_read_page(offset,semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES,cx)));assert!(!answer.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(job.retained_request_backing_identity(),Some(pointer));}assert!(fixture.turn(|cx|job.restart_read_page(offset,semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES,cx)).unwrap());}
        let mut complete=false;for turn in 0..4096{fixture.turn(|cx|{assert_eq!((cx.operation(),cx.generation(),cx.payload_authority_identity(),cx.original_source_parent()?.source_identity()?),identity);Ok(())}).unwrap();complete=matches!(fixture.step(&mut job).unwrap(),Some(semio_framework_job::JobOutcomeKind::Complete));if complete{break}assert!(turn<4095);}assert!(complete);
        let length=restart["lengths"][index].as_u64().unwrap()as usize;let eof=restart["eof"][index].as_bool().unwrap();assert_eq!(job.result_status(),Some(NativeIoResultStatus::Page{length,pages:1,eof}));let Some(Ok(NativeIoValue::Page{bytes,..}))=job.result.as_ref()else{panic!("original page retained")};assert_eq!(bytes.payload_authority_identity(),Some(identity.2));assert_eq!(bytes.single_page().unwrap(),&source[offset as usize..offset as usize+length]);
        for turn in 0..64{if job.result.is_none(){break}let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.turn(|cx|job.close_result_step(cx)));answer.unwrap();let receipt=fixture.caller.as_ref().unwrap().receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(job.retained_request_backing_identity(),Some(pointer));assert!(turn<63);}assert!(job.result.is_none());
    }
    fixture.close_job(&mut job);assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job)).1.released_bytes,0);std::fs::remove_file(path).unwrap();
}

#[test]
fn resident_memory_observation_does_not_spawn_a_process() {
    let value = process_resident_bytes();
    assert!(value.is_none() || value.is_some_and(|bytes| bytes > 0));
}

#[test]
fn path_set_max_plus_one_identity_zero_grant_and_job_close_are_exact() {
    let source=fixture_native_io_grant();let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(1),source,fixture_io_clock,fixture_io_policy().3,fixture_io_policy().1);
    let mut paths = NativePathSet::new();
    for index in 0..NATIVE_IO_PATH_CAPACITY {
        let mut input=Some(PathBuf::from(format!("/retained-native-path-{index:04}")));
        caller.turn(source,|cx|{let turn=paths.push_from(&mut input,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(())}).unwrap();assert!(input.is_none());
    }
    let plus_one = PathBuf::from("/retained-native-path-plus-one");
    let plus_one_pointer = plus_one.as_os_str().as_encoded_bytes().as_ptr();
    let mut returned=Some(plus_one);
    assert!(caller.turn(source,|cx|{let turn=paths.push_from(&mut returned,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(())}).is_err());
    assert_eq!(returned.as_ref().unwrap().as_os_str().as_encoded_bytes().as_ptr(),plus_one_pointer);
    let capacity=returned.as_ref().unwrap().capacity();
    caller.turn(source,|cx|{semio_framework_job::close_original_retained_step(cx,RetirementDemand{release_bytes:capacity,depth:1,..Default::default()},|_|{drop(returned.take());semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()})),work:1}})?;Ok(())}).unwrap();
    let mut job = NativeIoJob::new(NativeIoRequest::Modified(NativePathInput::Rows(paths)));
    job.begin_close();
    assert_eq!(caller.turn(RetainedCloneGrant::default(),|cx|job.close_step(cx)).unwrap(),InteractiveJobCloseStep::Pending{progress:Default::default()});
    let mut released = 0;
    while !job.terminal_is_empty() {
        if let InteractiveJobCloseStep::Pending { progress } = caller.turn(fixture_native_io_grant(),|cx|job.close_step(cx)).unwrap() {
            assert!(progress.copied_items <= 1);
            released += progress.copied_items;
        }
    }
    assert!(released >= NATIVE_IO_PATH_CAPACITY);caller.retire(source);
}

#[test]
fn chunked_read_write_scan_and_modified_round_trip() {
    let root = std::env::temp_dir().join(format!("semio-native-io-{}", std::process::id()));
    let path = root.join("fixture.wasm");
    let bytes = vec![0xA5; 96 * 1024 + 17];
    std::fs::create_dir_all(&root).expect("test fixture directory");
    std::fs::write(&path, &bytes).expect("test-only filesystem oracle");
    let TestNativeIoValue::Bytes(full)=run(NativeIoRequest::ReadBytes(path.clone())).unwrap()else{panic!("full original byte output")};
    assert_eq!(full,bytes);
    assert_eq!(serde_json::to_value(&full).unwrap(),serde_json::to_value(&bytes).unwrap());
    let TestNativeIoValue::Page { bytes: first, eof: false } = run(NativeIoRequest::ReadPage { path: path.clone(), offset: 0, max_bytes: 16 * 1024 }).unwrap() else { panic!("first page") };
    assert_eq!(first, bytes[..16 * 1024]);
    let offset = (bytes.len() - 7) as u64;
    let TestNativeIoValue::Page { bytes: last, eof: true } = run(NativeIoRequest::ReadPage { path: path.clone(), offset, max_bytes: 16 * 1024 }).unwrap() else { panic!("last page") };
    assert_eq!(last, bytes[bytes.len() - 7..]);
    assert!(run(NativeIoRequest::ReadPage { path: path.clone(), offset: 0, max_bytes: 64 * 1024 + 1 }).is_err());
    let TestNativeIoValue::Paths(mut paths) = run(NativeIoRequest::ScanDirectory { path: root.clone(), directories_only: false, extension: Some("wasm".into()), first_only: true }).unwrap() else { panic!("scan value") };
    assert_eq!(paths.pop(), Some(path.clone()));
    let expected_time=std::fs::metadata(&path).unwrap().modified().unwrap();
    for single in[false,true]{
        let request=if single{NativeIoRequest::Modified(NativePathInput::Single(path.clone()))}else{
            let mut source=OriginalIoFixture::new();let mut paths=NativePathSet::new();let mut input=Some(path.clone());
            source.turn(|cx|{let turn=paths.push_from(&mut input,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(())}).unwrap();assert!(input.is_none());NativeIoRequest::Modified(NativePathInput::Rows(paths))
        };
        let TestNativeIoValue::Modified(mut modified)=run(request).unwrap()else{panic!("modified value")};
        assert_eq!(modified.len(),1);assert_eq!(modified.pop(),Some((path.clone(),expected_time)));
    }
    std::fs::remove_dir_all(root).unwrap();
}


#[test]
fn close_preserves_original_empty_path_and_error_backing_until_exact_release() {
    let source=fixture_native_io_grant();let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(1),source,fixture_io_clock,fixture_io_policy().3,fixture_io_policy().1);
    let mut path = PathBuf::with_capacity(128);
    path.push("original"); path.clear();
    let pointer = path.as_os_str().as_encoded_bytes().as_ptr();
    let capacity = path.capacity();
    let mut job = NativeIoJob::new(NativeIoRequest::ReadBytes(path));
    job.begin_close();
    assert_eq!(job.next_close_release_byte_demand().unwrap(), capacity);
    let granted = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: capacity, maximum_depth: 1, ..RetainedCloneGrant::default() };
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧮️compute-retained/🔣️.json")).unwrap();
    for (index,denied) in [RetainedCloneGrant { maximum_items: 0, ..granted }, RetainedCloneGrant { maximum_release_bytes: capacity - 1, ..granted }, RetainedCloneGrant { maximum_depth: 0, ..granted }].into_iter().enumerate() {
        assert_eq!(fixture["nativeIoDenials"][index]["progress"],serde_json::json!({"copiedItems":0,"copiedBytes":0,"retainedCapacityBytes":0,"releasedBytes":0}));
        assert_eq!(caller.turn(denied,|cx|job.close_step(cx)).unwrap(), InteractiveJobCloseStep::Pending{progress:Default::default()});
        assert_eq!(job.retained_request_backing_identity(), Some(pointer));
        assert_eq!(job.next_close_release_byte_demand().unwrap(), capacity);
    }
    assert_eq!(caller.turn(granted,|cx|job.close_step(cx)).unwrap().progress(), RetainedCloneProgress { copied_items: 1, released_bytes: capacity, ..RetainedCloneProgress::default() });
    let mut error = String::with_capacity(256);error.push_str("failed");error.clear();
    let capacity = error.capacity();let pointer = error.as_ptr();
    job.state = NativeIoState::Finished;job.result = Some(Err(error));
    let denied = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: capacity - 1, maximum_depth: 1, ..RetainedCloneGrant::default() };
    assert_eq!(caller.turn(denied,|cx|job.close_step(cx)).unwrap(),InteractiveJobCloseStep::Pending{progress:Default::default()});
    assert_eq!(job.result.as_ref().unwrap().as_ref().unwrap_err().as_ptr(), pointer);
    let step = caller.turn(RetainedCloneGrant { maximum_release_bytes: capacity, ..denied },|cx|job.close_step(cx)).unwrap();
    assert_eq!(step.progress().released_bytes, capacity);
    assert_eq!(job.result.as_ref().unwrap().as_ref().unwrap_err().capacity(), 0);
    assert!(!job.terminal_is_empty());
    assert_eq!(caller.turn(RetainedCloneGrant { maximum_items: 1, maximum_depth: 1, ..RetainedCloneGrant::default() },|cx|job.close_step(cx)).unwrap().progress().copied_items, 1);
    assert!(job.terminal_is_empty());
    caller.retire(source);
}

#[test]
fn started_read_keeps_its_original_path_until_exact_same_source_release() {
    let root = std::env::temp_dir().join(format!("semio-io-path-owner-{}", std::process::id()));
    std::fs::write(&root, b"original source").unwrap();
    let mut path = PathBuf::with_capacity(128);
    path.push(&root);
    let pointer = path.as_os_str().as_encoded_bytes().as_ptr();
    let capacity = path.capacity();
    let mut job = NativeIoJob::new(NativeIoRequest::ReadBytes(path));
    let mut fixture = OriginalIoFixture::new();
    let before = fixture.caller.as_ref().unwrap().fuel_remaining();
    assert!(fixture.step(&mut job).unwrap().is_none());
    assert_eq!(fixture.caller.as_ref().unwrap().fuel_remaining(), before - 1);
    assert_eq!(job.retained_request_backing_identity(), Some(pointer));
    job.begin_close();
    for denied in [RetainedCloneGrant { maximum_items: 0, ..fixture_native_io_grant() }, RetainedCloneGrant { maximum_release_bytes: capacity - 1, ..fixture_native_io_grant() }, RetainedCloneGrant { maximum_depth: 0, ..fixture_native_io_grant() }] {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| fixture.limited(denied, |cx| job.close_step(cx)).unwrap());
        assert_eq!(step.progress(), RetainedCloneProgress::default());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(job.retained_request_backing_identity(), Some(pointer));
    }
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| fixture.turn(|cx| job.close_step(cx)).unwrap());
    assert_eq!(step.progress().released_bytes, capacity);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, capacity));
    fixture.close_job(&mut job);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(job));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    std::fs::remove_file(root).unwrap();
}

#[test]
fn missing_modified_path_and_fault_rejection_keep_backing_until_funded_driver_close() {
    for fault in [false, true] {
        let mut path = PathBuf::with_capacity(128);
        path.push(format!("__semio_missing_native_io_{}_{}__", std::process::id(), fault));
        let capacity = path.capacity();
        let pointer = path.as_os_str().as_encoded_bytes().as_ptr();
        let mut fixture = OriginalIoFixture::new();
        let mut job = if fault {
            let mut job = NativeIoJob::new(NativeIoRequest::ProcessResidentBytes);
            job.state = NativeIoState::ClosingScanFault { paths: NativePathSet::new(), rejected: Some(path), error: "original retained fault".to_string() };
            job
        } else {
            let mut paths = NativePathSet::new();
            let mut input=Some(path);fixture.turn(|cx|{let turn=paths.push_from(&mut input,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(())}).unwrap();assert!(input.is_none());
            let mut job = NativeIoJob::new(NativeIoRequest::ProcessResidentBytes);
            job.state = NativeIoState::ReadingModified { paths, modified: NativeModifiedSet::new() };
            job
        };
        if !fault {for turn in 0..4{if job.retired_path.is_some(){break}assert!(fixture.step(&mut job).unwrap().is_none());assert!(turn<3)}}
        for denied in [RetainedCloneGrant { maximum_items: 0, ..fixture_native_io_grant() }, RetainedCloneGrant { maximum_release_bytes: capacity - 1, ..fixture_native_io_grant() }, RetainedCloneGrant { maximum_depth: 0, ..fixture_native_io_grant() }] {
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| assert!(fixture.step_limited(denied,&mut job).unwrap().is_none()));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let retained = if fault { match &job.state { NativeIoState::ClosingScanFault { rejected: Some(path), .. } => path, _ => panic!("original rejected path") } } else { job.retired_path.as_ref().unwrap() };
            assert_eq!(retained.as_os_str().as_encoded_bytes().as_ptr(), pointer);
        }
        let before = fixture.caller.as_ref().unwrap().cumulative_receipt();
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| assert!(fixture.step(&mut job).unwrap().is_none()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, capacity));
        assert_eq!(fixture.caller.as_ref().unwrap().cumulative_receipt().released_bytes - before.released_bytes, capacity);
        fixture.close_job(&mut job);
    }
}


#[test]
fn original_batch_parent_reads_neutral_payloads_through_real_host_and_stable_backing(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🚪️native-io/🎟️original/payload.json")).unwrap();assert!(law["expected"]["realReturnClaim"].as_bool().unwrap());assert!(law["stableBuffer"]["sameBackingAcrossMoves"].as_bool().unwrap());let source=law["payload"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect::<Vec<_>>();let large=law["fullRead"]["existingLargeBytes"].as_u64().unwrap()as usize;let path=std::env::temp_dir().join(format!("semio-original-parent-{}",std::process::id()));
 for expected in[Vec::new(),source.clone(),(0..large).map(|index|(index%251)as u8).collect()]{std::fs::write(&path,&expected).unwrap();let TestNativeIoValue::Bytes(actual)=run(NativeIoRequest::ReadBytes(path.clone())).unwrap()else{panic!("actual Batch parent byte outcome")};let oracle=std::fs::read(&path).unwrap();assert_eq!(actual,oracle);assert_eq!(serde_json::to_value(&actual).unwrap(),serde_json::to_value(&expected).unwrap());}
 std::fs::write(&path,&source).unwrap();let offset=law["readPage"]["offset"].as_u64().unwrap();let maximum=law["readPage"]["maximumBytes"].as_u64().unwrap()as usize;let TestNativeIoValue::Page{bytes:actual,..}=run(NativeIoRequest::ReadPage{path:path.clone(),offset,max_bytes:maximum}).unwrap()else{panic!("actual Batch parent page outcome")};let oracle=std::fs::read(&path).unwrap();assert_eq!(actual,&oracle[offset as usize..offset as usize+maximum]);std::fs::remove_file(path).unwrap();
}
