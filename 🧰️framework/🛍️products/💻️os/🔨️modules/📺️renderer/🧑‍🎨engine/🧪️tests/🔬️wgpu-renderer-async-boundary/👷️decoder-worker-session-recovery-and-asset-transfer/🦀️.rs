/// 👷️ Decoder worker, session recovery and asset transfer: the worker session saturation child, the unadmitted session recovery, the decoder worker boundary, the ready response cancel and return, the credited page transfer and the frame candidate that never advances the decoder.

/// 🧫️ Marks the child process that runs [`an_independent_decoder_job_recovers_its_response_from_an_unadmitted_session`] alone.
use semio_framework_value::RetainedCloneStep;
use semio_framework_trace::InteractiveStage;
use semio_framework_job::StepContext;
use semio_framework_job::OriginalTurnFailure;
use semio_framework_job::JobOutcomeDescriptor;
#[cfg(not(target_arch = "wasm32"))]
const WORKER_SESSION_SATURATION_CHILD: &str = "SEMIO_RENDERER_WORKER_SESSION_SATURATION_CHILD";

/// 🎟️ Session saturation leaves the decoder job and its parameters with the caller, returns the unchanged decoder response and retires both owners in paid turns. The law owns every
/// process-wide [`semio_framework_job_worker::WORKER_JOB_SESSION_SLOTS`] admission at once, so it runs alone in a child process of this
/// test binary: beside it, parallel laws of the same process admit sessions too — one lost race refused a neighbour's admission,
/// the other dropped a rejected admission mid-unwind and aborted the whole binary.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_independent_decoder_job_recovers_its_response_from_an_unadmitted_session() {
    if std::env::var_os(WORKER_SESSION_SATURATION_CHILD).is_none() {
        let law = format!("{}::an_independent_decoder_job_recovers_its_response_from_an_unadmitted_session", module_path!().split_once("::").expect("crate-rooted test module").1);
        let output = std::process::Command::new(std::env::current_exe().unwrap()).args([law.as_str(), "--exact", "--test-threads=1"]).env(WORKER_SESSION_SATURATION_CHILD, "1").output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(output.status.success() && stdout.contains("test result: ok. 1 passed"), "the isolated saturation law passes alone: {stdout}{}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    use semio_framework_job::*;
    use semio_framework_job_worker::{WORKER_JOB_SESSION_SLOTS, MountedWorkerJobSession};
    use semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource;
    struct Occupied;
    impl InteractiveJob for Occupied {
        fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
        fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,semio_framework_value::ValueError>{descriptor.yielded()}
        fn begin_close(&mut self){}
        fn close_step(&mut self,_:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        fn terminal_is_empty(&self)->bool{true}
    }
    fn config()->semio_framework_job_worker_authority::BatchDriveConfig{semio_framework_job_worker_authority::BatchDriveConfig{retained:renderer_law_stock(),site:"decoder_admission_law",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:INTERACTIVE_LANE_WALL_US}}
    let mut caller=OriginalRendererFixture::new();let mut occupied=Vec::with_capacity(WORKER_JOB_SESSION_SLOTS);
    for _ in 0..WORKER_JOB_SESSION_SLOTS {
        let mut source=caller.turn(|cx|Ok(RefusedWorkerSource::pending(Occupied,(cx.operation(),cx.generation())))).unwrap();
        let mut admitted=None;for _ in 0..16{admitted=caller.turn(|cx|source.admit_pending(config(),cx)).unwrap();if admitted.is_some(){break}}
        assert!(source.terminal_is_empty());occupied.push(admitted.expect("real original worker slot admits its exact owner"));
    }
    let (authority,probe)=owned_decoder_fixture();let token=probe.owner().owner().token();let bytes=probe.owner().owner().received_bytes();
    let mut rejected=caller.turn(|cx|Ok(RefusedWorkerSource::pending(RendererAssetDecodeJob::new(probe),(cx.operation(),cx.generation())))).unwrap();
    for _ in 0..4{assert!(caller.turn(|cx|rejected.admit_pending(config(),cx)).unwrap().is_none());}
    let recovered=rejected.job().unwrap().take_probe().expect("maximum-plus-one retains its original exact response");assert_eq!(recovered.owner().owner().token(),token);assert_eq!(recovered.owner().owner().received_bytes(),bytes);assert_eq!(recovered.observed_bytes,0);assert!(rejected.job().unwrap().take_probe().is_none());
    for _ in 0..65536{caller.turn(|cx|{rejected.close_step(cx)?;Ok(())}).unwrap();if rejected.terminal_is_empty(){break}}assert!(rejected.terminal_is_empty());
    for session in &mut occupied {
        session.begin_close();
        for _ in 0..65536 {
            caller.turn(|cx| {
                let witness=cx.borrowed_original_cancel_token()?;
                if matches!(session.has_original_cancel_alias_witness(witness),Ok(true)){
                    close_original_retained_step(cx,semio_framework_value::RetirementDemand{depth:1,..Default::default()},|grant|session.return_original_cancel_alias_step(witness,grant).map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(|error|match error{semio_framework_job_worker::WorkerJobDemandError::Refused(error)=>error,semio_framework_job_worker::WorkerJobDemandError::Contention(_)=>semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original saturation cancellation return contended")}))?;
                } else { session.close_step(cx)?; }Ok(())
            }).unwrap();if session.terminal_is_empty(){break}
        }
        assert!(session.terminal_is_empty());
    }
    close_owned_decoder_fixture(authority,recovered);
}

#[cfg(not(target_arch = "wasm32"))]
fn decoder_worker_boundary(cancel_after_ready: bool) {
    use std::time::{Duration, Instant};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json")).unwrap();
    let bytes = b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\"></svg>";
    let mut authority = WorldAssetIoAuthority::default();
    authority.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "ready-cancel.svg", bytes.len()).unwrap();
    let mut owner = authority.take_next().unwrap();
    owner.push_page(WorldAssetResponsePage::try_from_owned(bytes.to_vec()).unwrap()).unwrap();
    owner.seal().unwrap();
    authority.return_owner(owner).unwrap();
    let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).unwrap();
    let token = owner.token();
    let wakes = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed_wakes = wakes.clone();
    let wake: RuntimeHostWaker = Arc::new(move || {
        observed_wakes.fetch_add(1, Ordering::AcqRel);
    });
    let mut caller = OriginalRendererFixture::new();
    let mut decoder = caller.turn(|cx|Ok(RendererAssetDecodeSession::new(RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)), cx))).unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while decoder.boundary.is_none() && Instant::now() < deadline {
        if caller.turn(|cx|decoder.pump_one(cx)).unwrap() { wake(); }
        std::thread::yield_now();
    }
    let completed = decoder.boundary.as_ref().map(|(_, result)| *result);
    if cancel_after_ready {
        decoder.cancel();
    }
    while (decoder.session.is_some() || decoder.source.is_some()) && Instant::now() < deadline {
        if caller.turn(|cx|decoder.pump_one(cx)).unwrap() { wake(); }
        std::thread::yield_now();
    }
    let (probe, outcome) = decoder.take_boundary().expect("the exact completed worker returns its response");
    let observed_token = probe.owner().owner().token();
    let observed_bytes = probe.observed_bytes;
    assert!(decoder.terminal_is_empty());
    close_owned_decoder_fixture(authority, probe);
    assert_eq!(completed, Some(Some(RendererAssetDecodeResult::Ready)));
    assert_eq!(observed_token, token);
    assert_eq!(observed_bytes, bytes.len());
    assert!(wakes.load(Ordering::Acquire) > 0);
    assert_eq!(law["cancelledDecode"]["afterClose"], "cancelled");
    assert_eq!(outcome, Some(if cancel_after_ready { RendererAssetDecodeResult::Cancelled } else { RendererAssetDecodeResult::Ready }));
}

/// 🛑️ A real worker completion remains cancellable until its response crosses the host boundary.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_independent_decoder_job_cancels_a_ready_response_before_host_handback() {
    decoder_worker_boundary(true);
}

/// 📬️ Session metadata retirement preserves a successful worker result when no cancellation occurred.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_independent_decoder_job_returns_a_ready_response_after_session_retirement() {
    decoder_worker_boundary(false);
}

/// 📄️ Native transport transfers a complete credited response page into its exact request.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_asset_response_transfers_each_credited_page_without_losing_its_bytes() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).unwrap();
    assert_eq!(law["pageBytes"].as_u64().unwrap()as usize,WORLD_ASSET_RESPONSE_PAGE_BYTES);
    for case in law["cases"].as_array().unwrap(){
        let length=case["bytes"].as_u64().unwrap()as usize;let period=law["patternPeriod"].as_u64().unwrap()as usize;
        let bytes:Vec<u8>=(0..length).map(|index|(index%period)as u8).collect();let pointer=bytes.as_ptr();
        let mut authority=WorldAssetIoAuthority::default();authority.reserve(1,1,WorldAssetRequestKind::Glb,"native-page.glb",length).unwrap();let mut fetch=RendererAssetFetchOwner::Shared(authority.take_next().unwrap());
        let mut rejected=None;let accepted=match WorldAssetResponsePage::try_from_owned(bytes){Ok(page)=>{assert_eq!(page.bytes().as_ptr(),pointer);fetch.owner_mut().push_page(page).unwrap();true},Err(bytes)=>{assert_eq!(bytes.as_ptr(),pointer);rejected=Some(bytes);false}};
        let observed=fetch.owner().received_bytes();let source_released=rejected.is_none();let retained_source=rejected.as_ref().map_or(0,Vec::len);
        if accepted{let RendererAssetFetchOwner::Shared(owner)=&mut fetch else{unreachable!()};owner.seal().unwrap();let decoded=owner.decode_page().unwrap().unwrap().bytes();assert_eq!(decoded.len(),length);assert!(decoded.iter().enumerate().all(|(index,byte)|*byte==(index%period)as u8));}
        fetch.begin_close();let RendererAssetFetchOwner::Shared(owner)=fetch else{unreachable!()};assert!(authority.return_owner(owner).is_ok());authority.begin_close();for _ in 0..4096{if authority.close_step(){break}}assert!(authority.terminal_is_empty());
        if let Some(bytes)=rejected.as_ref(){let capacity=bytes.capacity();let mut caller=OriginalRendererFixture::new();caller.turn(|cx|{semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{release_bytes:capacity,depth:1,..Default::default()},|_|{rejected.take();Ok(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()}))})?;Ok(())}).unwrap();assert!(rejected.is_none());}
        assert_eq!(accepted,case["accepted"].as_bool().unwrap());assert_eq!(observed as u64,case["transferredBytes"].as_u64().unwrap());assert_eq!(source_released,accepted);assert_eq!(retained_source,if accepted{0}else{length});
        eprintln!("[DEBUG] exact World response page boundary {observed}/{length}, retained source on refusal, original backing pointer and authored byte pattern preserved");
    }
}

/// 🧵️ A retained catalogue response belongs to the decoder even while a frame candidate advances.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_frame_candidate_never_advances_the_independently_owned_asset_decoder() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json")).unwrap();
    let bytes = semio_framework_os_renderer_wgpu_test_support::repo_bytes("🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb");
    let mut authority = WorldAssetIoAuthority::default();
    authority.reserve(1, 1, WorldAssetRequestKind::Glb, "catalogue-frame-isolation.glb", bytes.len()).unwrap();
    let mut owner = authority.take_next().unwrap();
    for page in bytes.chunks(16 * 1024) {
        owner.push_page(WorldAssetResponsePage::try_from_owned(page.to_vec()).unwrap()).unwrap();
    }
    owner.seal().unwrap();
    authority.return_owner(owner).unwrap();
    let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).unwrap();
    let runtime = RuntimeMailbox::new(AppRuntime {
        atlas: FontAtlas::builtin(),
        icons: IconAtlas::default(),
        icon_rebuild: None,
        icon_raster_scale: 1.0,
        interaction: None,
        checkout: Default::default(),
        draw: DrawList::default(),
        overlay: DrawList::default(),
        pending_frame_deferred: None,
        frame_actions: FrameActionOwners::default(),
        pending_frame_maintenance_refusal: None,
        plugin_modules_root: Default::default(),
        native_hot_swap_scan: None,
        native_hot_swap_program: None,
        native_hot_swap_cursor: 0,
        native_reload_pending: false,
    });
    *runtime.0.asset_probe.lock().unwrap() = Some(RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)));
    let operation = semio_framework_job::OperationId(71);
    let generation = semio_framework_job::Generation(0);
    let mut candidate = FrameTransaction::new(Default::default(), operation, generation, semio_framework_job::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 65_536, maximum_release_bytes: 16_777_216, maximum_depth: 64 });
    let mut caller = OriginalRendererFixture::with_identity(operation, generation);
    let result = caller.turn(|cx| Ok(candidate.step(&runtime, &Arc::downgrade(&runtime.0), cx))).unwrap();
    let mut probe = runtime.0.asset_probe.lock().unwrap().take().expect("the decoder retains its exact response");
    let observed = probe.observed_bytes;
    for _ in 0..4096 {
        if candidate.close_step() {
            break;
        }
    }
    assert!(candidate.terminal_is_empty());
    probe.begin_close();
    for _ in 0..262_144 {
        if probe.close_step() {
            break;
        }
    }
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().expect("the response retires before observation assertions") else {
        panic!("fixture response owner changed");
    };
    authority.finish(owner).unwrap();
    authority.begin_close();
    for _ in 0..4096 {
        if authority.close_step() {
            break;
        }
    }
    assert!(authority.terminal_is_empty());
    for _ in 0..4096 {
        if caller.turn(|cx|runtime.close_renderer_asset_step(cx)).unwrap() {
            break;
        }
    }
    assert!(matches!(result, AppFrameTransactionStep::Superseded));
    assert_eq!(observed as u64, law["expected"]["frameDecodeUnits"].as_u64().unwrap());
}
