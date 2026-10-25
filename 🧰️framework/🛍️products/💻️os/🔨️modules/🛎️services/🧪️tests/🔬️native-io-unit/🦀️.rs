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

struct OriginalIoFixture { operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, caller: Option<semio_framework_job::OriginalJobFixtureCaller>, decoder: semio_framework_job::OriginalJobDecodeOwner }
impl OriginalIoFixture {
    fn new() -> Self { let operation = semio_framework_job::allocate_operation_id(); let generation = semio_framework_job::Generation(1); Self { operation, generation, caller: Some(semio_framework_job::OriginalJobFixtureCaller::admit(operation, generation, fixture_native_io_grant(), fixture_io_clock, u64::MAX, fixture_io_policy().1)), decoder: semio_framework_job::OriginalJobDecodeOwner::empty() } }
    fn turn<R>(&mut self, run: impl FnOnce(&mut StepContext<'_>) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> { self.limited(fixture_native_io_grant(), run) }
    fn limited<R>(&mut self, grant: RetainedCloneGrant, run: impl FnOnce(&mut StepContext<'_>) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> { self.caller.as_mut().unwrap().turn_with_decoder(grant, &mut self.decoder, run) }
    fn close_job(&mut self, job: &mut NativeIoJob) {
        job.begin_close();
        for _ in 0..16384 { if job.terminal_is_empty() { return; } self.turn(|cx| job.close_step(cx)).unwrap(); }
        assert!(job.terminal_is_empty());
    }
    fn close_value(&mut self, value: &mut NativeIoValue) {
        for _ in 0..16384 { if value.terminal_is_empty() { return; } self.turn(|cx| { let demand = value.retirement_demands()?; semio_framework_job::close_original_retained_step(cx, demand, |grant| value.close_step(grant).map(|step| match step { InteractiveJobCloseStep::Pending { progress } | InteractiveJobCloseStep::Complete { progress } => semio_framework_value::RetainedCloneStep::Progress(progress), InteractiveJobCloseStep::Blocked => semio_framework_value::RetainedCloneStep::Progress(Default::default()) }))?; Ok(()) }).unwrap(); }
        assert!(value.terminal_is_empty());
    }
}
impl Drop for OriginalIoFixture {
    fn drop(&mut self) {
        if std::thread::panicking() { return; }
        let caller = self.caller.as_mut().unwrap();
        for _ in 0..16384 { if self.decoder.terminal_is_empty() { break; } caller.turn(fixture_native_io_grant(), |cx| { let demand = self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?; semio_framework_job::close_original_retained_step(cx, demand, |grant| self.decoder.close_step(grant))?; Ok(()) }).unwrap(); }
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
    Page { bytes: Vec<u8>, eof: bool },
    Paths(Vec<PathBuf>),
    Modified(Vec<(PathBuf, std::time::SystemTime)>),
}

fn materialize_test_value(value: NativeIoValue, fixture: &mut OriginalIoFixture) -> TestNativeIoValue {
    match value {
        NativeIoValue::Bytes(bytes) => { drop(payload_vec(bytes, fixture)); TestNativeIoValue::Other }
        NativeIoValue::Page { bytes, eof } => TestNativeIoValue::Page { bytes: payload_vec(bytes, fixture), eof },
        NativeIoValue::Paths(paths) => { let owned = paths.entries[..paths.length].iter().flatten().cloned().collect(); fixture.close_value(&mut NativeIoValue::Paths(paths)); TestNativeIoValue::Paths(owned) }
        NativeIoValue::Modified(entries) => { let owned = entries.entries[..entries.length].iter().flatten().cloned().collect(); fixture.close_value(&mut NativeIoValue::Modified(entries)); TestNativeIoValue::Modified(owned) }
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
    let mut session = None;
    for _ in 0..16384 { fixture.turn(|cx| { if let Some((original, _)) = semio_framework_job_worker::BatchJobSession::try_admit_owned(&mut job, &mut params, cx)? { session = Some(original); } Ok(()) }).unwrap(); if session.is_some() { break; } }
    assert!(job.is_none() && params.is_none());
    let mut session = session.expect("original native I/O session admission");
    let result;
    let mut turns = 0;
    loop {
        turns += 1;
        assert!(turns < 16384, "native I/O never published its terminal original outcome");
        fixture.turn(|cx| session.step_on_original_caller(cx)).unwrap();
        assert!(session.checkout_outcome(), "exact original native I/O outcome checkout");
        let (issued, posted) = session.take_checked_out_retained_step_receipt().expect("actual original caller step receipt");
        assert!(posted.fits(issued));
        let Some(terminal) = session.checked_out_outcome().unwrap().map(|view| view.is_terminal()) else { session.resume().unwrap(); continue; };
        for _ in 0..16384 {
            let done = fixture.turn(|cx| { let demand = session.retirement_demands(cx.retained_grant().maximum_copy_bytes).expect("exact original outcome acknowledgement quote"); let mut complete = false; semio_framework_job::close_original_retained_step(cx, demand, |grant| { let step = session.acknowledge_outcome(grant); complete = matches!(step, semio_framework_value::RetainedCloneStep::Complete(_)); Ok(step) })?; Ok(complete) }).unwrap();
            if done { break; }
        }
        if terminal {
            let job = session.checked_out_job_mut().expect("same original terminal job");
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
                let step = session.return_original_cancel_alias_step(cx.original_cancel_token(), cx.retained_grant()).unwrap().unwrap(); cx.consume_retained(step.progress())?; cx.consume_fuel(1);
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
  if started{fixture.turn(|cx|job.step(cx).map(|outcome|{assert!(outcome.is_none());})).unwrap();assert!(matches!(job.state,NativeIoState::Scanning{..}));fixture.turn(|cx|job.close_step(cx)).unwrap();assert!(job.request_path.is_none());}
  let(target,demand)=job.close_target().unwrap();assert!(matches!(target,NativeIoCloseTarget::Extension|NativeIoCloseTarget::RequestExtension));assert_eq!(demand.release_bytes,capacity);
  if owned{let denied=RetainedCloneGrant{maximum_release_bytes:capacity-1,..fixture_native_io_grant()};let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.limited(denied,|cx|job.close_step(cx)));assert!(matches!(answer.unwrap(),InteractiveJobCloseStep::Pending{progress}if progress==Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(job.close_target().unwrap().1.release_bytes,capacity);}
  let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.turn(|cx|job.close_step(cx)));answer.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));assert_eq!(fixture.caller.as_ref().unwrap().receipt().released_bytes,capacity);assert!(job.request_extension.is_none());if let NativeIoState::Pending(NativeIoRequest::ScanDirectory{extension,..})=&job.state{assert!(extension.is_none());}fixture.close_job(&mut job);assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job)).1.released_bytes,0);
 }}std::fs::remove_dir(root).unwrap();
}

#[test]
fn original_selected_path_keeps_exact_directory_backing_until_paid_supplier_return(){
 static RETURNS:OriginalNativePathReturnRegistry<1>=OriginalNativePathReturnRegistry::with_return_wake(||{});
 let root=std::env::temp_dir().join(format!("semio-original-selected-{}",std::process::id()));std::fs::create_dir_all(&root).unwrap();let expected=root.join("selected.wasm");std::fs::write(&expected,b"selected").unwrap();let mut fixture=OriginalIoFixture::new();let mut job=NativeIoJob::new(NativeIoRequest::ScanDirectory{path:root.clone(),directories_only:false,extension:Some(std::borrow::Cow::Borrowed("wasm")),first_only:true});
 for _ in 0..64{fixture.turn(|cx|{if let Some(outcome)=job.step(cx)?{let mut descriptor=outcome.into_descriptor();semio_framework_job::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|grant|Ok(descriptor.acknowledge(grant)))?;assert!(descriptor.is_acknowledged());}Ok(())}).unwrap();if job.result_status().is_some(){break}}
 let Some(Ok(NativeIoValue::Paths(paths)))=job.result.as_ref()else{panic!("actual directory result")};let path=paths.entries[paths.length-1].as_ref().unwrap();let pointer=path.as_os_str().as_encoded_bytes().as_ptr();let capacity=path.capacity();let mut reservation=fixture.turn(|cx|RETURNS.reserve(cx)).unwrap().unwrap();let mut output=None;
 let mut foreign=OriginalIoFixture::new();assert!(foreign.turn(|cx|job.supply_result_last_path(&mut reservation,&mut output,cx)).is_err());assert!(output.is_none());assert_eq!(job.result_status(),Some(NativeIoResultStatus::Paths{count:1}));
 assert!(!fixture.limited(RetainedCloneGrant{maximum_items:0,..fixture_native_io_grant()},|cx|job.supply_result_last_path(&mut reservation,&mut output,cx)).unwrap());assert!(output.is_none());assert!(fixture.turn(|cx|job.supply_result_last_path(&mut reservation,&mut output,cx)).unwrap());drop(reservation);let output=output.unwrap();let identity=output.source_identity();assert_eq!(output.path(),expected.as_path());assert_eq!(output.path().as_os_str().as_encoded_bytes().as_ptr(),pointer);fixture.close_job(&mut job);assert!(RETURNS.retains_source(identity));assert!(!RETURNS.has_returned_source(identity));
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(output));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(RETURNS.has_returned_source(identity));let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.limited(RetainedCloneGrant{maximum_release_bytes:capacity-1,..fixture_native_io_grant()},|cx|RETURNS.close_returned_step(cx)));assert!(matches!(answer.unwrap(),InteractiveJobCloseStep::Pending{progress}if progress==Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.turn(|cx|RETURNS.close_returned_step(cx)));answer.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));assert_eq!(fixture.caller.as_ref().unwrap().receipt().released_bytes,capacity);for _ in 0..16{if !RETURNS.retains_source(identity){break}fixture.turn(|cx|RETURNS.close_returned_step(cx)).unwrap();}assert!(!RETURNS.retains_source(identity));assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job)).1.released_bytes,0);std::fs::remove_file(expected).unwrap();std::fs::remove_dir(root).unwrap();
}

#[test]
fn original_stream_extent_refuses_oversize_before_payload_birth(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🧵️job/📬️outcome/📮️return/🔣️.json")).unwrap();
    let mut fixture=OriginalIoFixture::new();
    for row in law["cases"].as_array().unwrap(){let length=row["bytes"].as_u64().unwrap();let accepted=row["accepted"].as_bool().unwrap();let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.turn(|cx|{let bytes=row["stockBytes"].as_u64().unwrap()as usize;let total=cx.retained_grant();cx.bind_original_retained_total(RetainedCloneGrant{maximum_capacity_bytes:bytes.min(total.maximum_capacity_bytes),maximum_release_bytes:bytes.min(total.maximum_release_bytes),..total})?;Ok(validate_original_stream_extent(length,cx))}));assert_eq!(answer.unwrap().is_ok(),accepted);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(fixture.caller.as_ref().unwrap().receipt(),Default::default());}
    let path=std::env::temp_dir().join(format!("semio-native-stream-ceiling-{}",std::process::id()));let length=semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES as u64+1;File::create(&path).unwrap().set_len(length).unwrap();
    for page in[false,true]{let request=if page{NativeIoRequest::ReadPage{path:path.clone(),offset:0,max_bytes:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES}}else{NativeIoRequest::ReadBytes(path.clone())};let mut job=NativeIoJob::new(request);let pointer=job.retained_request_backing_identity().unwrap();fixture.turn(|cx|job.step(cx).map(|_|())).unwrap();if page{assert!(matches!(job.state,NativeIoState::ReadingPage{..}));assert!(job.result.is_none());}else{assert!(matches!(job.result,Some(Ok(NativeIoValue::StreamRefused(NativeIoStreamRefusal{length:actual,maximum_bytes})))if actual==length&&maximum_bytes==semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES as u64));assert!(matches!(job.state,NativeIoState::Finished));}assert!(job.fault_writer.is_none()&&job.fault_detail.is_none());assert_eq!(fixture.caller.as_ref().unwrap().receipt().retained_capacity_bytes,0);assert_eq!(job.retained_request_backing_identity(),Some(pointer));fixture.close_job(&mut job);}
    std::fs::remove_file(path).unwrap();
}

#[test]
fn original_page_restart_keeps_one_source_path_and_stock_through_result_retirement(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🧵️job/📬️outcome/📮️return/🔣️.json")).unwrap();let restart=&law["restart"];let mut source=vec![0u8;restart["fileBytes"].as_u64().unwrap()as usize];for(index,byte)in source.iter_mut().enumerate(){*byte=(index%251)as u8;}
    let path=std::env::temp_dir().join(format!("semio-native-page-restart-{}",std::process::id()));std::fs::write(&path,&source).unwrap();let mut job=NativeIoJob::new(NativeIoRequest::ReadPage{path:path.clone(),offset:0,max_bytes:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES});let pointer=job.retained_request_backing_identity().unwrap();let mut fixture=OriginalIoFixture::new();let identity=fixture.turn(|cx|Ok((cx.operation(),cx.generation(),cx.payload_authority_identity(),cx.original_source_parent()?.source_identity()?))).unwrap();
    for(index,offset)in restart["offsets"].as_array().unwrap().iter().enumerate(){let offset=offset.as_u64().unwrap();
        if index!=0{for denied in[RetainedCloneGrant{maximum_items:0,..fixture_native_io_grant()},RetainedCloneGrant{maximum_copy_bytes:0,..fixture_native_io_grant()},RetainedCloneGrant{maximum_depth:0,..fixture_native_io_grant()}]{let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture.limited(denied,|cx|job.restart_read_page(offset,semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES,cx)));assert!(!answer.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(job.retained_request_backing_identity(),Some(pointer));}assert!(fixture.turn(|cx|job.restart_read_page(offset,semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES,cx)).unwrap());}
        let mut complete=false;for turn in 0..4096{let outcome=fixture.turn(|cx|{assert_eq!((cx.operation(),cx.generation(),cx.payload_authority_identity(),cx.original_source_parent()?.source_identity()?),identity);job.step(cx).map(|outcome|outcome.map(semio_framework_job::JobOutcomeBorrow::into_descriptor))}).unwrap();if let Some(mut descriptor)=outcome{complete=matches!(descriptor.kind(),semio_framework_job::JobOutcomeKind::Complete);fixture.turn(|cx|{semio_framework_job::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|grant|Ok(descriptor.acknowledge(grant)))?;Ok(())}).unwrap();assert!(descriptor.is_acknowledged());drop(descriptor);if complete{break}}assert!(turn<4095);}assert!(complete);
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
    let source=fixture_native_io_grant();let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(1),source,semio_framework_job::default_now_us,u64::MAX,fixture_io_policy().1);
    let mut paths = NativePathSet::new();
    for index in 0..NATIVE_IO_PATH_CAPACITY {
        paths.try_push(PathBuf::from(format!("/retained-native-path-{index:04}"))).expect("fixed path capacity");
    }
    let plus_one = PathBuf::from("/retained-native-path-plus-one");
    let plus_one_pointer = plus_one.as_os_str().as_encoded_bytes().as_ptr();
    let returned = paths.try_push(plus_one).expect_err("maximum plus one returns exact path owner");
    assert_eq!(returned.as_os_str().as_encoded_bytes().as_ptr(), plus_one_pointer);
    drop(returned);
    let mut job = NativeIoJob::new(NativeIoRequest::Modified(paths));
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
    assert!(run(NativeIoRequest::ReadBytes(path.clone())).is_err());
    let TestNativeIoValue::Page { bytes: first, eof: false } = run(NativeIoRequest::ReadPage { path: path.clone(), offset: 0, max_bytes: 16 * 1024 }).unwrap() else { panic!("first page") };
    assert_eq!(first, bytes[..16 * 1024]);
    let offset = (bytes.len() - 7) as u64;
    let TestNativeIoValue::Page { bytes: last, eof: true } = run(NativeIoRequest::ReadPage { path: path.clone(), offset, max_bytes: 16 * 1024 }).unwrap() else { panic!("last page") };
    assert_eq!(last, bytes[bytes.len() - 7..]);
    assert!(run(NativeIoRequest::ReadPage { path: path.clone(), offset: 0, max_bytes: 64 * 1024 + 1 }).is_err());
    let TestNativeIoValue::Paths(mut paths) = run(NativeIoRequest::ScanDirectory { path: root.clone(), directories_only: false, extension: Some("wasm".into()), first_only: true }).unwrap() else { panic!("scan value") };
    assert_eq!(paths.pop(), Some(path.clone()));
    let mut modified_paths = NativePathSet::new();
    modified_paths.try_push(path).expect("one modified path");
    let TestNativeIoValue::Modified(mut modified) = run(NativeIoRequest::Modified(modified_paths)).unwrap() else { panic!("modified value") };
    assert_eq!(modified.len(), 1);
    drop(modified.pop());
    std::fs::remove_dir_all(root).unwrap();
}


#[test]
fn close_preserves_original_empty_path_and_error_backing_until_exact_release() {
    let source=fixture_native_io_grant();let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(1),source,semio_framework_job::default_now_us,u64::MAX,fixture_io_policy().1);
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
    assert!(fixture.turn(|cx| job.step(cx).map(|outcome| outcome.is_none())).unwrap());
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
        let mut job = if fault {
            let mut job = NativeIoJob::new(NativeIoRequest::ProcessResidentBytes);
            job.state = NativeIoState::ClosingScanFault { paths: NativePathSet::new(), rejected: Some(path), error: "original retained fault".to_string() };
            job
        } else {
            let mut paths = NativePathSet::new();
            paths.try_push(path).unwrap();
            let mut job = NativeIoJob::new(NativeIoRequest::ProcessResidentBytes);
            job.state = NativeIoState::ReadingModified { paths, modified: NativeModifiedSet::new() };
            job
        };
        let mut fixture = OriginalIoFixture::new();
        if !fault { fixture.turn(|cx| job.step(cx).map(|outcome| { assert!(outcome.is_none()); })).unwrap(); }
        for denied in [RetainedCloneGrant { maximum_items: 0, ..fixture_native_io_grant() }, RetainedCloneGrant { maximum_release_bytes: capacity - 1, ..fixture_native_io_grant() }, RetainedCloneGrant { maximum_depth: 0, ..fixture_native_io_grant() }] {
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| fixture.limited(denied, |cx| job.step(cx).map(|outcome| assert!(outcome.is_none()))).unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let retained = if fault { match &job.state { NativeIoState::ClosingScanFault { rejected: Some(path), .. } => path, _ => panic!("original rejected path") } } else { job.retired_path.as_ref().unwrap() };
            assert_eq!(retained.as_os_str().as_encoded_bytes().as_ptr(), pointer);
        }
        let before = fixture.caller.as_ref().unwrap().cumulative_receipt();
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| fixture.turn(|cx| job.step(cx).map(|outcome| assert!(outcome.is_none()))).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, capacity));
        assert_eq!(fixture.caller.as_ref().unwrap().cumulative_receipt().released_bytes - before.released_bytes, capacity);
        fixture.close_job(&mut job);
    }
}
