/// 👷️ Decoder worker, session recovery and asset transfer: the worker session saturation child, the unadmitted session recovery, the decoder worker boundary, the ready response cancel and return, the credited page transfer and the frame candidate that never advances the decoder.

/// 🧫️ Marks the child process that runs [`an_independent_decoder_job_recovers_its_response_from_an_unadmitted_session`] alone.
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
    use semio_framework_job_worker::{WorkerJobDemandError, WorkerJobSession, WORKER_JOB_SESSION_SLOTS};
    use semio_framework_job_worker_authority::{BatchDriveConfig, BatchJobParams};
    struct Occupied;
    impl InteractiveJob for Occupied {
        fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
            JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from)
        }
        fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
            descriptor.yielded()
        }
        fn begin_close(&mut self) {}
        fn close_step(&mut self, _: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
            Ok(InteractiveJobCloseStep::Complete { progress: Default::default() })
        }
        fn terminal_is_empty(&self) -> bool {
            true
        }
    }
    const STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: usize::MAX / 4, maximum_copy_bytes: usize::MAX / 4, maximum_capacity_bytes: usize::MAX / 4, maximum_release_bytes: usize::MAX / 4, maximum_depth: 64 };
    const TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 22, maximum_release_bytes: 1 << 22, maximum_depth: 32 };
    let identity = (OperationId(76), Generation(1));
    let mut caller = OriginalJobFixtureCaller::admit(identity.0, identity.1, STOCK, default_now_us, u64::MAX, u64::MAX);
    fn params(caller: &mut OriginalJobFixtureCaller) -> BatchJobParams {
        BatchJobParams {
            operation: OperationId(76),
            generation: Generation(1),
            cancel: caller.admit_alias(TURN),
            now_us: default_now_us,
            config: BatchDriveConfig { retained: TURN, site: "decoder_admission_law", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: INTERACTIVE_LANE_WALL_US },
        }
    }
    let mut occupied = Vec::with_capacity(WORKER_JOB_SESSION_SLOTS);
    for _ in 0..WORKER_JOB_SESSION_SLOTS {
        let mut job = Some(Occupied);
        let mut original = Some(params(&mut caller));
        let admitted = caller.turn(TURN, |cx| WorkerJobSession::try_admit_owned(&mut job, &mut original, cx).map_err(OriginalTurnFailure::from)).expect("the original admission turn is not refused");
        occupied.push(admitted.unwrap_or_else(|| panic!("fixed session admits its owner")).0);
    }
    let (authority, probe) = owned_decoder_fixture();
    let token = probe.owner().owner().token();
    let bytes = probe.owner().owner().received_bytes();
    let mut job = Some(RendererAssetDecodeJob::new(probe, Arc::new(AtomicBool::new(false))));
    let mut original = Some(params(&mut caller));
    let refused = caller.turn(TURN, |cx| WorkerJobSession::try_admit_owned(&mut job, &mut original, cx).map_err(OriginalTurnFailure::from)).expect("the original admission turn is not refused");
    assert!(refused.is_none(), "maximum plus one must stay unadmitted");
    assert!(job.is_some() && original.is_some(), "the refused admission leaves both original owners with the caller");
    let recovered = job.as_ref().and_then(RendererAssetDecodeJob::take_probe).expect("the sole unadmitted job returns its exact response");
    assert_eq!(recovered.owner().owner().token(), token);
    assert_eq!(recovered.owner().owner().received_bytes(), bytes);
    assert_eq!(recovered.observed_bytes, 0);
    assert!(job.as_ref().and_then(RendererAssetDecodeJob::take_probe).is_none());
    let mut unadmitted = job.take().expect("the unadmitted job remains owned");
    unadmitted.begin_close();
    for _ in 0..64 {
        if unadmitted.terminal_is_empty() {
            break;
        }
        caller.turn(TURN, |cx| unadmitted.close_step(cx)).expect("the unadmitted job close turn is not refused");
    }
    assert!(unadmitted.terminal_is_empty());
    drop(unadmitted);
    let mut retirement = semio_framework_async::CancelTokenRetirement::from_token(original.take().expect("the refused parameters remain owned").cancel);
    for _ in 0..64 {
        if retirement.terminal_is_empty() {
            break;
        }
        caller
            .turn(TURN, |cx| {
                match retirement.return_alias_step(cx.original_cancel_token(), cx.retained_grant()) {
                    Ok(step) => {
                        cx.consume_retained(step.progress())?;
                        cx.consume_fuel(1);
                    }
                    Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)) => {}
                    Err(semio_framework_async::CancelTokenRetirementError::Refused(error)) => return Err(OriginalTurnFailure::from(error)),
                }
                Ok(())
            })
            .expect("the refused alias return turn is not refused");
    }
    assert!(retirement.terminal_is_empty());
    drop(retirement);
    for mut session in occupied {
        let _ = session.begin_close();
        for _ in 0..64 {
            if session.terminal_is_empty() {
                break;
            }
            caller
                .turn(TURN, |cx| {
                    match session.has_original_cancel_alias_witness(cx.original_cancel_token()) {
                        Ok(true) => {
                            if let Some(step) = session.return_original_cancel_alias_step(cx.original_cancel_token(), cx.retained_grant()).map_err(|error| match error {
                                WorkerJobDemandError::Refused(error) => OriginalTurnFailure::from(error),
                                WorkerJobDemandError::Contention(_) => OriginalTurnFailure::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "occupied session alias remains checked out")),
                            })? {
                                cx.consume_retained(step.progress())?;
                                cx.consume_fuel(1);
                            }
                        }
                        Ok(false) => {
                            session.close_step(cx)?;
                        }
                        Err(WorkerJobDemandError::Contention(_)) => {}
                        Err(WorkerJobDemandError::Refused(error)) => return Err(OriginalTurnFailure::from(error)),
                    }
                    Ok(())
                })
                .expect("the occupied session close turn is not refused");
        }
        assert!(session.terminal_is_empty());
    }
    caller.retire(STOCK);
    close_owned_decoder_fixture(authority, recovered);
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
    let mut sequence = 0;
    let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(75), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(u64::MAX, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence, &mut context_retained);
    let invocation = RendererAssetDecodeInvocation::admit(&mut cx).expect("the original context admits its cancellation alias").expect("the unbounded original context funds the alias");
    let mut decoder = RendererAssetDecodeSession::new(RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)), Some(wake), invocation);
    let deadline = Instant::now() + Duration::from_secs(10);
    while decoder.boundary.is_none() && Instant::now() < deadline {
        decoder.pump_one(&mut cx).expect("the original decoder turn is not refused");
        std::thread::yield_now();
    }
    let completed = decoder.boundary.as_ref().map(|(_, result)| *result);
    if cancel_after_ready {
        decoder.cancel();
    }
    while decoder.session.is_some() && Instant::now() < deadline {
        decoder.pump_one(&mut cx).expect("the original decoder turn is not refused");
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
    use semio_framework_job::{JobPayloadStream, RetainedJobPayloadWriter};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).unwrap();
    assert_eq!(law["pageBytes"].as_u64().unwrap() as usize, WORLD_ASSET_RESPONSE_PAGE_BYTES);
    for case in law["cases"].as_array().unwrap() {
        let length = case["bytes"].as_u64().unwrap() as usize;
        let bytes: Vec<u8> = (0..length).map(|index| (index % law["patternPeriod"].as_u64().unwrap() as usize) as u8).collect();
        let mut authority = WorldAssetIoAuthority::default();
        authority.reserve(1, 1, WorldAssetRequestKind::Glb, "native-page.glb", length).unwrap();
        let mut fetch = RendererAssetFetchOwner::Shared(authority.take_next().unwrap());
        let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput);
        let mut sequence = 0;
        for source in bytes.chunks(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
            let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
            let mut context =
                semio_framework_job::StepContext::new(semio_framework_job::OperationId(74), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut context_retained);
            let mut page = writer.admit_page(&mut context).unwrap();
            page.write(source).unwrap();
            page.commit();
        }
        let mut payload = writer.finish().unwrap_or_else(|_| panic!("the fixture owns complete native pages"));
        let result = push_renderer_asset_page(&mut fetch, &mut payload);
        let source_released = payload.terminal_is_empty();
        let retained_source = payload.len();
        while !payload.terminal_is_empty() {
            let _ = payload.close_step(semio_framework_job::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES, maximum_capacity_bytes: 0, maximum_release_bytes: 2 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 3 });
        }
        let observed = fetch.owner().received_bytes();
        let decoded = if result.is_ok() {
            let RendererAssetFetchOwner::Shared(owner) = &mut fetch else { unreachable!() };
            owner.seal().unwrap();
            owner.decode_page().unwrap().unwrap().bytes().to_vec()
        } else {
            Vec::new()
        };
        fetch.begin_close();
        let RendererAssetFetchOwner::Shared(owner) = fetch else { unreachable!() };
        assert!(authority.return_owner(owner).is_ok());
        authority.begin_close();
        for _ in 0..4096 {
            if authority.close_step() {
                break;
            }
        }
        assert!(authority.terminal_is_empty());
        println!("native response transferred {observed}/{length} bytes with result {result:?}");
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap());
        assert_eq!(observed as u64, case["transferredBytes"].as_u64().unwrap());
        if result.is_ok() {
            assert_eq!(decoded, bytes);
        }
        assert_eq!(source_released, case["accepted"].as_bool().unwrap());
        assert_eq!(retained_source, if result.is_ok() { 0 } else { length });
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
        native_plugin_mtimes: Default::default(),
        native_hot_swap_scan: None,
        native_hot_swap_modified: None,
        native_hot_swap_cursor: 0,
        native_reload_pending: false,
    });
    *runtime.0.asset_probe.lock().unwrap() = Some(RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)));
    let operation = semio_framework_job::OperationId(71);
    let generation = semio_framework_job::Generation(0);
    let mut candidate = FrameTransaction::new(Default::default(), operation, generation, semio_framework_job::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 65_536, maximum_release_bytes: 16_777_216, maximum_depth: 64 });
    let mut sequence = 0;
    let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut context_retained);
    let result = candidate.step(&runtime, &Arc::downgrade(&runtime.0), &mut context);
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
        if runtime.close_renderer_asset_step() {
            break;
        }
    }
    assert!(matches!(result, AppFrameTransactionStep::Superseded));
    assert_eq!(observed as u64, law["expected"]["frameDecodeUnits"].as_u64().unwrap());
}
