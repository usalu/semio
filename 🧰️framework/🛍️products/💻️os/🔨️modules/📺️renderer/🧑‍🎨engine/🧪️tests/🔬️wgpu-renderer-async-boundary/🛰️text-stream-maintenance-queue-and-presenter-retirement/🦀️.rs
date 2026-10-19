/// 🛰️ Text stream, maintenance queue and presenter retirement: the incomplete text stream, the accepted maintenance queue, the interrupted execution, the dropped deferred jobs, the frame action maximum, the runtime presentation and raster operation authorities and the presenter retirement contract.

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_incomplete_text_stream_waits_for_external_ingress_without_a_worker_self_wake() {
    let mut interaction = frame_maintenance_test_owner(83, 0).interaction.take().expect("text interaction owner");
    assert!(!interaction.has_pending_text_work(), "an empty text authority has no runnable work");
    interaction.start_text_operation(71, 6).expect("text stream admission");
    assert!(!interaction.has_pending_text_work(), "a reserved stream awaiting its first page is externally blocked");
    interaction.push_text_operation(71, "abc".to_string()).expect("first text page");
    assert!(!interaction.has_pending_text_work(), "a partial stream awaits its next ingress page without self-waking");
    interaction.push_text_operation(71, "def".to_string()).expect("final text page");
    interaction.commit_text_operation(71).expect("text stream commit");
    assert!(interaction.has_pending_text_work(), "a committed stream owns runnable text-buffer work");

    for _ in 0..128 {
        if !interaction.has_pending_text_work() {
            break;
        }
        interaction.drive_text_operation();
    }
    assert!(!interaction.has_pending_text_work(), "the committed stream reaches an idle direct-continuation state");

    interaction.start_text_operation(72, 3).expect("cancelled text stream admission");
    interaction.cancel_text_operations();
    assert!(interaction.has_pending_text_work(), "explicit cancellation remains runnable while it retires the reserved owner");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn accepted_frame_maintenance_queue_drop_publishes_exact_generation_owner() {
    let registry = FrameExecutionRegistry::new();
    let owner = frame_maintenance_test_owner(91, 1);
    let identity = owner.interaction.as_ref().map(|interaction| interaction.shell.plugins.plugin_filter.as_ptr());
    let cell = Arc::new(FrameExecutionOwnerCell::new(owner));
    assert!(registry.try_publish(91, cell.clone()).is_ok());
    assert!(!registry.abandon(92));
    assert!(registry.abandon(91));
    let Some((generation, recovered_cell)) = registry.try_take_returned() else { panic!("abandoned execution owner") };
    assert_eq!(generation, 91);
    assert!(Arc::ptr_eq(&cell, &recovered_cell));
    let Some(recovered) = recovered_cell.try_take() else { panic!("exact abandoned owner") };
    assert_eq!(recovered.interaction.as_ref().map(|interaction| interaction.shell.plugins.plugin_filter.as_ptr()), identity);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn interrupted_frame_maintenance_execution_restores_before_incremental_recovery() {
    let registry = FrameExecutionRegistry::new();
    let owner = frame_maintenance_test_owner(93, 2);
    let cell = Arc::new(FrameExecutionOwnerCell::new(owner));
    assert!(registry.try_publish(93, cell.clone()).is_ok());
    let Some(running_cell) = registry.try_begin(93) else { panic!("running execution cell") };
    let Some(mut running_owner) = running_cell.try_take() else { panic!("running exact owner") };
    let Some(cursor) = running_owner.cursor.as_mut() else { panic!("running cursor") };
    cursor.begin_close();
    running_cell.restore_taken(running_owner);
    assert!(registry.abandon(93));
    let Some((generation, recovered_cell)) = registry.try_take_returned() else { panic!("interrupted execution owner") };
    assert_eq!(generation, 93);
    let Some(mut recovered) = recovered_cell.try_take() else { panic!("interrupted exact owner") };
    let Some(cursor) = recovered.cursor.as_mut() else { panic!("interrupted cursor") };
    let mut caller = crate::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let before = cursor.actions.len;
    assert!(!caller.turn(|cx|cursor.close_step(cx)).unwrap());
    assert_eq!(before - cursor.actions.len, 1);
    while !caller.turn(|cx| cursor.close_step(cx)).unwrap() {}
    caller.finish();
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn dropped_ordinary_frame_deferred_jobs_recover_the_exact_pair_and_close_remaining_actions_one_per_step() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📮️wgpu-frame-deferred-ownership/🔣️.json")).expect("frame deferred ownership fixture");
    for law in fixture["cases"].as_array().expect("ownership cases") {
        let generation = if law["dropAt"] == "queued" { 101 } else { 102 };
        let mut maintenance = frame_maintenance_test_owner(generation, 3);
        let interaction = maintenance.interaction.take().expect("exact interaction owner");
        let mut cursor = maintenance.cursor.take().expect("exact cursor owner");
        cursor.shell_maintenance = false;
        cursor.pump_sync = false;
        cursor.flush_tutorial = false;
        let current = cursor.take_next().expect("current deferred action");
        let mut owner = FrameDeferredExecutionOwner::new(interaction, cursor, current);
        let registry = FrameExecutionRegistry::new();
        let cell = Arc::new(FrameExecutionOwnerCell::new(owner));
        assert!(registry.try_publish(generation, cell.clone()).is_ok());
        if law["dropAt"] == "running" {
            let running = registry.try_begin(generation).expect("running deferred owner");
            owner = running.try_take().expect("running exact owner");
            let Some(FrameDeferredWork::Action(action)) = owner.work.take() else { panic!("current action owner") };
            action.cancel();
            running.restore_taken(owner);
        }
        assert!(registry.abandon(generation));
        let (_, recovered) = registry.try_take_returned().expect("abandoned deferred owner");
        let mut recovered = recovered.try_take().expect("recovered exact owner");
        recovered.cancel_current();
        let (_, mut cursor) = recovered.take_pair().expect("recovered exact pair");
        assert_eq!(cursor.actions.len, law["remainingActions"].as_u64().unwrap() as usize);
        cursor.begin_close();
        let mut caller = crate::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
        let mut close_steps = 0_u64;
        loop {
            close_steps += 1;
            if caller.turn(|cx| cursor.close_step(cx)).unwrap() {
                break;
            }
        }
        assert!(close_steps > law["remainingActions"].as_u64().unwrap(), "typed descriptor fields retire through separate original frontiers");
        caller.finish();
        assert!(cursor.terminal_is_empty());
        assert_eq!(law["currentReceiptSettlements"], 1);
        assert_eq!((law["readyReservations"].as_u64(), law["inFlightReservations"].as_u64()), (Some(0), Some(0)));
    }
}

#[test]
fn frame_action_max_plus_one_returns_the_exact_owner() {
    let mut actions = FrameActionOwners::default();
    for index in 0..WORLD3D_DEADLINE_CAPACITY {
        assert!(actions.try_push(ActionDescriptor { controller_id: index.to_string(), action: "admitted".to_string(), args: None }).is_ok());
    }
    let rejected = ActionDescriptor { controller_id: "exact-owner".to_string(), action: "commit".to_string(), args: None };
    let identity = rejected.controller_id.as_ptr();
    let Err(rejected) = actions.try_push(rejected) else { panic!("MAX + 1 must refuse") };
    assert_eq!(rejected.controller_id.as_ptr(), identity);
}

#[test]
fn asset_poll_does_not_accumulate_completed_response_vectors() {
    assert!(!LIBRARY_SOURCE().contains("let mut fetched_glb = Vec::new()"));
    assert!(!LIBRARY_SOURCE().contains("let mut fetched_map = Vec::new()"));
    assert!(!LIBRARY_SOURCE().contains("let mut fetched_ui_images = Vec::new()"));
}

/// 🔗️ The presenter contract's own cross-reference to the law below, checked by the COMPILER rather
/// than by a source-text match: the census in `presenter_retirement_contract` only states that
/// freshness is decided once because this law states that the authority and a candidate witness move
/// independently. Renaming or deleting the law fails the build instead of quietly narrowing the
/// contract, which is exactly how the old text-marker version went stale.
const _: fn() = runtime_presentation_authority_and_candidate_identity_change_independently;

#[test]
fn runtime_presentation_authority_and_candidate_identity_change_independently() {
    let authority = RuntimePresentationAuthority::new();
    authority.observe_input_generation(7);
    let unchanged_candidate = authority.current();
    assert!(authority.matches(unchanged_candidate.scene_revision, unchanged_candidate.input_generation));

    authority.mark_scene_changed();
    assert_eq!(unchanged_candidate, RuntimePresentationWitness { scene_revision: 1, input_generation: 7 });
    assert!(!authority.matches(unchanged_candidate.scene_revision, unchanged_candidate.input_generation));

    let unchanged_authority = authority.current();
    let changed_candidate = RuntimePresentationWitness { scene_revision: unchanged_authority.scene_revision + 1, input_generation: unchanged_authority.input_generation + 1 };
    assert_eq!(authority.current(), unchanged_authority);
    assert!(!authority.matches(changed_candidate.scene_revision, changed_candidate.input_generation));

    authority.observe_input_generation(8);
    assert!(authority.witness_for(7).is_none());
    assert_eq!(authority.witness_for(8), Some(authority.current()));
}

#[test]
fn runtime_raster_operation_authority_rejects_stale_duplicate_and_device_close_ack() {
    let authority = RuntimeRasterOperationAuthority::new();
    let first = authority.begin(3, 5).expect("independent raster operation");
    assert_eq!(first.scene_revision, 3);
    assert_eq!(first.preview_generation, 5);
    assert!(authority.matches(first));
    assert!(authority.begin(3, 5).is_err());
    let stale = semio_framework_ui_wgpu_raster::RasterTextureWitness { operation: first.operation.wrapping_add(1), ..first };
    assert!(!authority.matches(stale));
    assert!(authority.release(stale).is_err());
    assert!(authority.matches(first));
    authority.release(first).expect("device close returns exact operation owner");
    assert!(authority.current().is_none());
    assert!(authority.release(first).is_err());
    let second = authority.begin(3, 5).expect("next operation");
    assert!(second.operation > first.operation);
}

#[test]
fn runtime_raster_operation_authority_exhausts_permanently_without_aba() {
    let authority = RuntimeRasterOperationAuthority::new();
    authority.0.next_operation.store(u64::MAX - 1, Ordering::Release);

    let penultimate = authority.begin(8, 13).expect("MAX-1 operation");
    assert_eq!(penultimate.operation, u64::MAX - 1);
    authority.release(penultimate).expect("release MAX-1");

    let last = authority.begin(8, 13).expect("MAX operation");
    assert_eq!(last.operation, u64::MAX);
    assert!(authority.begin(8, 13).is_err(), "live MAX remains occupied");
    authority.release(last).expect("release MAX");
    assert!(authority.begin(8, 13).is_err(), "exhaustion remains permanent after release");
    assert!(authority.current().is_none());
    assert!(authority.0.exhausted.load(Ordering::Acquire));
    assert_eq!(authority.0.next_operation.load(Ordering::Acquire), u64::MAX);
}

/// 🎟️ LAW (source census): the presentation authority is read ONCE per frame, by the BUILD, and the
/// presenter is then bound to that frozen pair for the rest of the presentation.
///
/// The census walks the four hand-offs that make that one claim true:
///
/// 1. **The build asks for its own generation.** `AppFrameTransaction::step` refuses to advance when
///    `presentation_witness_for(self.generation.0)` answers nothing (a superseded input generation),
///    and the `AppFrameTransactionPhase::Build` arm asks again to mint the `FrameBuildCursor`. Two
///    reads, both keyed on the transaction's OWN generation, and the second one writes what it read
///    into the authority with `admit_build_presentation_witness` — the build declares the pair it
///    was admitted under instead of leaving the presenter to guess.
/// 2. **The presenter reads `admitted()`, never `current()`.** `current()` is a moving target:
///    `mark_scene_changed` fires from every runtime completion the host pumps, several per tick, and
///    `observe_presentation_input_generation` is republished on every `build_and_publish_snapshot`.
///    A presenter that re-read the live pair at admission refused packets that had been built
///    correctly against the authority as it stood when the build started and turned them into
///    surface faults (`prepared render revision is stale: live=35, packet=27` →
///    `worker-present-failed`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
/// 3. **Acknowledgement compares against the FROZEN witness.** `AppPresentPhase::BeginGpu` mints one
///    `RasterTextureWitness` from the admitted pair; the acknowledge phase compares the packet to
///    THAT witness, so a presentation that legitimately spans more than one host tick is never
///    faulted by an authority that moved underneath it.
/// 4. **Every owner is returned by a step, never dropped** — the prepared witness ladder, the fixed
///    mesh GPU table, and the host's `close_world_owners_step` drain.
///
/// **React ref:** R3F's demand frameloop is the same contract. `invalidate()`
/// (`🌐️World3dHost/🟦️.tsx:2358` and its twelve siblings) marks the scene changed and the next raf
/// renders the scene AS IT STANDS when the render begins; React never refuses a queued frame because
/// something invalidated again in between — the later `invalidate` supersedes it with a new frame.
/// Freshness is the producer's decision on both targets.
fn presenter_retirement_contract(glue: &str, prepared: &str, gpu: &str, draw: &str, host: &str, winit: &str, laws: &str) -> bool {
    let table = &draw[draw.find("pub struct MeshGpuTable").unwrap_or(0)..draw.find("pub const WORLD_GLOBALS_SLOT_SIZE").unwrap_or(draw.len())];
    prepared.contains("pub struct PreparedPresenterWitness")
        && prepared.contains("pub fn stage_presented")
        && prepared.contains("pub fn pending_presented")
        && prepared.contains("pub fn acknowledge_presented")
        && prepared.contains("pub fn abort_pending")
        && prepared.contains("pub fn take_last_valid")
        && prepared.contains("pub fn retire_step(&mut self) -> bool")
        && !prepared.contains("Option<Arc<PreparedRenderPacket>>")
        && glue.contains("AppPresentPhase::Acknowledge")
        && glue.contains("AppPresentPhase::Stage")
        && glue.contains("AppPresentPhase::Aborted")
        && glue.contains("self.gate.stage_presented(packet)")
        && glue.contains("self.gate.pending_presented(witness)")
        && glue.contains("self.gate.acknowledge_presented(witness)")
        && glue.contains("struct RuntimePresentationAuthority")
        && glue.contains("presentation_authority: RuntimePresentationAuthority")
        && glue.matches(concat!("runtime.presentation_", "witness_for(self.generation.0)")).count() == 2
        && glue.contains("if base_witness != current_witness {")
        && glue.contains("self.base_witness = Some(current_witness);")
        && glue.contains(concat!("runtime.admit_build_presentation_", "witness(presentation_witness);"))
        && glue.contains("self.build_cursor = Some(FrameBuildCursor::new(presentation_witness, self.retained));")
        && glue.contains("presentation_witness.scene_revision")
        && glue.contains("presentation_witness.input_generation")
        && glue.matches(concat!("let expected = self.presentation_", "authority.admitted();")).count() == 1
        && !glue.contains(concat!("let expected = self.presentation_", "authority.current();"))
        && glue.contains("begin_prepared(&token, &self.gate, packet, expected.scene_revision, expected.input_generation)")
        && glue.contains("begin_prepared_offscreen(token, &self.gate, packet, expected.scene_revision, expected.input_generation)")
        && glue.contains("self.raster_operation_authority.begin(expected.scene_revision, expected.input_generation)")
        && glue.contains(concat!("raster_witness.scene_revision != packet.scene_revision() || raster_witness.", "preview_generation != packet.preview_generation()"))
        && glue.matches("mark_scene_changed();").count() == 4
        && !glue.contains(concat!("let revision = packet.", "scene_revision();"))
        && !glue.contains(concat!("let generation = packet.", "preview_generation();"))
        && !glue.contains(concat!("scene_revision: packet.", "scene_revision()"))
        && !glue.contains(concat!("input_generation: packet.", "preview_generation()"))
        && glue.contains("struct AppPresentedRetirement")
        && glue.contains("acknowledged_eviction")
        && glue.contains("acknowledged_upload_scan")
        && glue.contains("acknowledged_versions")
        && glue.contains("previous.retire_step()")
        && glue.contains("if !gpu.close_mesh_upload_step()")
        && glue.contains("self.gpu.close_mesh_table_step()")
        && glue.contains("pub(crate) fn admit_next_frame")
        && winit.contains(".admit_next_frame(||match frame_build.poll_runtime_on_original_caller")
        && winit.contains("observe_presentation_input_generation(build_generation.0)")
        && !winit.contains("begin_present(frame).is_err()")
        && host.contains("presenter.close_world_owners_step()")
        && host.contains("presenter.world_owners_terminal_is_empty()")
        && draw.contains("pub const MESH_GPU_TABLE_CAPACITY: usize = 256")
        && draw.contains("slots: [Option<MeshGpuEntry<T>>; MESH_GPU_TABLE_CAPACITY]")
        && draw.contains("pub fn evict_mesh_step")
        && draw.contains("pub fn evict_mesh_except_step")
        && laws.contains("fn mesh_gpu_retirement_preserves_acknowledged_versions()")
        && draw.contains("pub fn close_step(&mut self) -> bool")
        && laws.contains("fn fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner()")
        && !table.contains("HashMap")
        && !table.contains(".retain(")
        && gpu.contains("self.mesh_store.evict_mesh_except_step(key, keep_versions)")
        && gpu.contains("self.mesh_store.close_step()")
}

#[test]
fn presenter_ack_retirement_source_mutations_are_denied() {
    assert!(presenter_retirement_contract(LIBRARY_SOURCE(), PREPARED_SOURCE(), GPU_SOURCE(), DRAW_SOURCE(), OS_HOST_SOURCE(), WINT_APP_SOURCE(), DRAW_LAWS_SOURCE()));
    for (source, from, to, once) in PRESENTER_RETIREMENT_MUTATIONS {
        let mutated = if *once { source.text().replacen(from, to, 1) } else { source.text().replace(from, to) };
        assert_ne!(mutated.as_str(), source.text(), "mutation '{from}' must actually rewrite {source:?} — a no-op edit proves nothing");
        let (glue, prepared, gpu, draw, host, laws, winit) = source.substitute(&mutated);
        assert!(!presenter_retirement_contract(glue, prepared, gpu, draw, host, winit, laws), "{source:?} mutation '{from}' -> '{to}' was accepted by the presenter retirement contract");
    }
}

/// 🧬️ Which source of the presenter ladder a mutation rewrites.
#[derive(Clone, Copy, Debug)]
enum PresenterContractSource {
    Glue,
    Prepared,
    Gpu,
    Draw,
    Host,
    Laws,
    Winit,
}

impl PresenterContractSource {
    fn text(self) -> &'static str {
        match self {
            PresenterContractSource::Glue => LIBRARY_SOURCE(),
            PresenterContractSource::Prepared => PREPARED_SOURCE(),
            PresenterContractSource::Gpu => GPU_SOURCE(),
            PresenterContractSource::Draw => DRAW_SOURCE(),
            PresenterContractSource::Host => OS_HOST_SOURCE(),
            PresenterContractSource::Laws => DRAW_LAWS_SOURCE(),
            PresenterContractSource::Winit => WINT_APP_SOURCE(),
        }
    }

    fn substitute<'a>(self, mutated: &'a str) -> (&'a str, &'a str, &'a str, &'a str, &'a str, &'a str, &'a str) {
        let mut sources = [LIBRARY_SOURCE(), PREPARED_SOURCE(), GPU_SOURCE(), DRAW_SOURCE(), OS_HOST_SOURCE(), DRAW_LAWS_SOURCE(), WINT_APP_SOURCE()];
        sources[self as usize] = mutated;
        (sources[0], sources[1], sources[2], sources[3], sources[4], sources[5], sources[6])
    }
}

/// 🧪️ Every rewrite `presenter_retirement_contract` must refuse — `(source, from, to, replace_once)`.
/// Each one is a real regression this ladder has already suffered or a shortcut it invites, so a
/// clause that stops discriminating shows up here as an accepted mutation rather than as silence.
const PRESENTER_RETIREMENT_MUTATIONS: &[(PresenterContractSource, &str, &str, bool)] = &[
    (PresenterContractSource::Glue, "AppPresentPhase::Acknowledge", "AppPresentPhase::Fullscreen", false),
    (PresenterContractSource::Glue, "self.gate.acknowledge_presented(witness)", "drop(witness); self.gate.acknowledge_presented_unchecked()", false),
    (PresenterContractSource::Glue, "previous.retire_step()", "drop(previous)", false),
    (PresenterContractSource::Glue, "AppPresentPhase::Stage", "AppPresentPhase::Render", false),
    (PresenterContractSource::Glue, "AppPresentPhase::Aborted", "AppPresentPhase::Render", false),
    (PresenterContractSource::Glue, "let expected = self.presentation_authority.admitted();", "let expected = self.presentation_authority.current();", false),
    (PresenterContractSource::Glue, "let expected = self.presentation_authority.admitted();", "let expected = RuntimePresentationWitness { scene_revision: packet.scene_revision(), input_generation: packet.preview_generation() };", true),
    (PresenterContractSource::Glue, "runtime.admit_build_presentation_witness(presentation_witness);", "", false),
    (PresenterContractSource::Glue, "runtime.presentation_witness_for(self.generation.0)", "Some(RuntimePresentationWitness { scene_revision: self.generation.0, input_generation: self.generation.0 })", false),
    (PresenterContractSource::Glue, "if base_witness != current_witness {", "if false {", false),
    (PresenterContractSource::Glue, "raster_witness.scene_revision != packet.scene_revision() || raster_witness.preview_generation != packet.preview_generation()", "false", false),
    (PresenterContractSource::Glue, "mark_scene_changed();", "", true),
    (PresenterContractSource::Glue, "if !gpu.close_mesh_upload_step()", "if false", false),
    (PresenterContractSource::Prepared, "pub fn abort_pending", "fn abandon_pending", false),
    (PresenterContractSource::Prepared, "last_valid: Option<PreparedRenderPacket>", "last_valid: Option<Arc<PreparedRenderPacket>>", false),
    (PresenterContractSource::Prepared, "pub fn retire_step(&mut self) -> bool", "pub fn retire_all(&mut self)", false),
    (PresenterContractSource::Gpu, "self.mesh_store.evict_mesh_except_step(key, keep_versions)", "self.mesh_store.evict_mesh_step(key)", false),
    (PresenterContractSource::Gpu, "self.mesh_store.close_step()", "drop(&mut self.mesh_store)", false),
    (PresenterContractSource::Draw, "MESH_GPU_TABLE_CAPACITY: usize = 256", "MESH_GPU_TABLE_CAPACITY: usize = usize::MAX", false),
    (PresenterContractSource::Draw, "FixedMeshGpuRegistry<GpuMeshBuffers>", "std::collections::HashMap<String, GpuMeshBuffers>", false),
    (PresenterContractSource::Draw, "pub fn close_step(&mut self) -> bool", "pub fn close_all(&mut self) -> bool", false),
    (PresenterContractSource::Draw, "pub fn evict_mesh_except_step", "pub fn evict_mesh_all", false),
    (PresenterContractSource::Laws, "fn mesh_gpu_retirement_preserves_acknowledged_versions()", "fn mesh_gpu_retirement_drops_everything()", false),
    (PresenterContractSource::Laws, "fn fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner()", "fn fixed_mesh_gpu_registry_grows()", false),
    (PresenterContractSource::Host, "presenter.world_owners_terminal_is_empty()", "true", false),
    (PresenterContractSource::Host, "presenter.close_world_owners_step()", "drop(presenter)", false),
    (PresenterContractSource::Winit, ".admit_next_frame(||match frame_build.poll_runtime_on_original_caller", "", false),
    (PresenterContractSource::Winit, "self.runtime.observe_presentation_input_generation(build_generation.0);", "", false),
];
