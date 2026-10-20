/** 🧯️ The process global payload store inventory, the actor progress overlay and the mounted fem2d session checks. */
import { toolJobProductionProtocolSource, toolJobGlobalPayloadStoreInventory, toolJobGlobalPayloadStoreIsSharedFixedAuthority, toolJobActorProgressOverlayExact, toolJobFem2dMountedSessionExact } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";

/** 🗃 Checks the process global payload owner inventory and the exact shared fixed authority exemption. */
export function globalPayloadStoreInventoryChecks() {
  const globalStoreFixture = toolJobProductionProtocolSource(`
thread_local! {
  static SCRATCH: RefCell<HashMap<String, Vec<u8>>> = RefCell::new(HashMap::new());
  static RETAINED_BRIDGE: RefCell<Option<Vec<u8>>> = RefCell::new(None);
}

static LIVE_LEASES: OnceLock<Mutex<[Option<Lease>; 4]>> = OnceLock::new();
static SHARED_LIVE: OnceLock<Mutex<semio_framework_job::FixedOperationRegistry<Lease, 4>>> = OnceLock::new();
#[cfg(test)]
fn hostile_owner() -> &'static Mutex<[Option<Vec<u8>>; 4]> {
  static HOSTILES: OnceLock<Mutex<[Option<Vec<u8>>; 4]>> = OnceLock::new();
  HOSTILES.get_or_init(|| Mutex::new([None, None, None, None]))
}
`);
  const globalStoreInventory = toolJobGlobalPayloadStoreInventory(new Map([["fixture.rs", globalStoreFixture]]));
  const globalStoreOwners = globalStoreInventory.flatMap((row) => row.owners.map((owner) => `${owner.name}:${owner.kind}`));
  const globalStoreExemptions = globalStoreInventory.filter(toolJobGlobalPayloadStoreIsSharedFixedAuthority);
  if (
    globalStoreInventory.length !== 3 ||
    !globalStoreOwners.includes("SCRATCH:child-content-scratch") ||
    !globalStoreOwners.includes("RETAINED_BRIDGE:abi-bridge-retention") ||
    !globalStoreOwners.includes("LIVE_LEASES:fixed-operation-registry") ||
    globalStoreOwners.some((owner) => owner.startsWith("HOSTILES:")) ||
    globalStoreExemptions.length !== 1 ||
    globalStoreExemptions[0]?.owners[0]?.name !== "SHARED_LIVE" ||
    toolJobGlobalPayloadStoreIsSharedFixedAuthority({ file: "fixture.rs", line: 1, text: "thread_local! {", owners: [{ name: "FORGED", rustType: "RefCell<FixedOperationRegistry<Lease, 4>>", kind: "fixed-operation-registry" }] })
  )
    throw new Error("[verify interactivity tool-jobs] process-global payload owner inventory or exact shared fixed-authority exemption drifted.");
}

/** 📶 Checks the actor, shard and wgpu job progress overlay sources and every single-edit mutation of them. */
export function actorProgressOverlayChecks() {
  const actorProgress = [
    "//#region 🎨️JobProgressOverlay",
    "pub const JOB_PROGRESS_ACTIVE_CAPACITY: usize = 64",
    "pub const JOB_PROGRESS_RETIREMENT_CAPACITY: usize = 128",
    "pub const JOB_PROGRESS_PAGE_MAXIMUM_BYTES: usize = 16 * 1024",
    "pub const JOB_PROGRESS_TOTAL_MAXIMUM_BYTES: usize = 4 * 1024 * 1024",
    "pub const JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS: usize = 512",
    "active: [JobProgressSlot; JOB_PROGRESS_ACTIVE_CAPACITY]",
    "retirements: [JobProgressRetirementSlot; JOB_PROGRESS_RETIREMENT_CAPACITY]",
    "pub fn begin_operation(",
    "pub fn preflight( cx.operation().0 != identity.operation cx.generation().0 != identity.generation !live.accepts(identity)",
    "pub fn publish_reserved(",
    "pub fn acknowledge(",
    "pub fn abort(",
    "pub fn retain_rejected(",
    "pub fn take(",
    "impl Drop for JobProgressCheckout",
    "pub fn begin_close_actor(",
    "pub fn close_step(",
    "pub fn terminal_is_empty(",
    "job_progress_preview_is_distinct_owned_and_checked_out_drop_hands_back_exactly",
    "abort must restore the last valid preview owner rather than exposing the staged replacement",
    "job_progress_fixed_capacity_and_aba_admission_fail_closed",
    "job_progress_mounted_aggregate_item_and_byte_caps_reject_plus_one_exactly",
    "job_progress_page_boundary_plus_one_stale_order_and_cancel_preserve_owner",
    "job_progress_commit_validates_live_authority_and_rejected_close_is_incremental",
    "job_progress_replay_is_deterministic_for_identical_publications",
    "//#endregion 🎨️JobProgressOverlay",
  ].join("\n");
  const shardProgress = [
    "authority: JobTurn,",
    "job_authorities: HashMap<(u64, u64), JobAuthority>",
    "replay_seeds: [Option<MountedReplaySeed>; JOB_REPLAY_SEED_SLOT_CAPACITY]",
    "MountedReplaySeed::new(actor_id, job, authority, request, placement, kind, input)",
    "*slot = Some(seed)",
    "self.runtime.start_job(instance, job, &kind, input).await",
    "self.job_authorities.insert((actor, seed.job), JobAuthority { turn: seed.authority, request:",
    "ShardOutcome::Job { actor: actor_id, authority: authority.turn, request: authority.request, placement, publication: JobPublication",
    "replay_seed_max_plus_one_returns_the_exact_spawn_owners_unchanged",
    "mounted_replay_rejects_wrong_route_seed_generation_and_worker_before_work_and_closes_one_owner_per_sub_eight_ms_opportunity",
  ].join("\n");
  const wgpuProgress = [
    "job_progress: JobProgressOverlayStore",
    "rejected_job_progress: [Option<JobProgressRejected>; 64]",
    "self.job_progress.preflight(&mut context, actor, &publication, live)",
    "self.job_progress.publish_reserved(&mut context, admission, publication, live)",
    "self.job_progress.acknowledge(receipt)",
    "pending_job_progress_presentations: [Option<PendingJobProgressPresentation>; JOB_PROGRESS_PRESENTATION_CAPACITY]",
    "slots: [JobProgressPresentationSlot; JOB_PROGRESS_PRESENTATION_CAPACITY]",
    "admission_sequence: u64",
    "next_admission_sequence: u64",
    "let index = self.oldest_ready_index()?",
    "AppPresentPhase::ProgressAcknowledge",
    "KernelRequest::AcknowledgeJobProgress { token }",
    "progress.acknowledge_presented()",
    "fn close_realm_progress_step(&mut self) -> bool",
    "self.job_progress.begin_close_all()",
    "self.pending_job_progress_presentations.iter().position(Option::is_some)",
    'job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").terminal_is_empty()',
    "fn begin_fault_close(&mut self, actor: ActorId)",
    "mounted_job_progress_presentation_bridge_is_fixed_fifo_and_generation_checked",
    "cancelled_head_does_not_strand_later_ready_presentations",
    "self.job_progress.abort(receipt)",
    "fn publish_captured_job_progress(&mut self, actor: ActorId, authority: JobTurn, publication: JobPublication) { stable_identity_matches; self.job_progress.begin_operation(actor, authority.job, live); self.job_progress.preflight(&mut context, actor, &publication, live); self.job_progress.publish_reserved(&mut context, admission, publication, live); }",
    "async fn destroy_app_step(&mut self, instance: u32) { self.pending_job_progress_presentations; self.job_progress.actor_terminal_is_empty(actor); }",
    "for outcome in outcomes { match outcome { ShardOutcome::Job { actor: reported, authority, request, placement, publication } => { self.begin_job_replay_capture(ActorId(reported), authority, request, placement, self.worker_count, worker_slot, publication); } _ => {} } }",
    "self.publish_captured_job_progress(actor, authority, publication)",
  ].join("\n");
  if (!toolJobActorProgressOverlayExact(actorProgress, shardProgress, wgpuProgress)) throw new Error("[verify interactivity tool-jobs] self-test actor-progress-overlay-valid was falsely rejected.");
  if (toolJobActorProgressOverlayExact(actorProgress.replace("active: [JobProgressSlot; JOB_PROGRESS_ACTIVE_CAPACITY]", "active: HashMap<(ActorId, u64), JobProgressSlot>"), shardProgress, wgpuProgress)) throw new Error("[verify interactivity tool-jobs] self-test actor-progress-overlay-resizable-active-registry was falsely accepted.");
  if (toolJobActorProgressOverlayExact(actorProgress.replace("cx.generation().0 != identity.generation", "false"), shardProgress, wgpuProgress)) throw new Error("[verify interactivity tool-jobs] self-test actor-progress-overlay-missing-generation-validation was falsely accepted.");
  if (toolJobActorProgressOverlayExact(actorProgress.replace("job_progress_mounted_aggregate_item_and_byte_caps_reject_plus_one_exactly", "job_progress_aggregate_caps_unmounted"), shardProgress, wgpuProgress)) throw new Error("[verify interactivity tool-jobs] self-test actor-progress-overlay-unmounted-aggregate-caps was falsely accepted.");
  if (toolJobActorProgressOverlayExact(actorProgress, shardProgress.replace("self.job_authorities.insert((actor, seed.job), JobAuthority { turn: seed.authority, request:", ""), wgpuProgress)) throw new Error("[verify interactivity tool-jobs] self-test autonomous-shard-job-without-independent-authority was falsely accepted.");
  if (toolJobActorProgressOverlayExact(actorProgress, shardProgress, wgpuProgress.replace("AppPresentPhase::ProgressAcknowledge", "AppPresentPhase::Fullscreen"))) throw new Error("[verify interactivity tool-jobs] self-test progress-ACK-before-presenter-adoption was falsely accepted.");
  if (toolJobActorProgressOverlayExact(actorProgress, shardProgress, wgpuProgress.replace("self.job_progress.begin_close_all()", ""))) throw new Error("[verify interactivity tool-jobs] self-test realm-close-without-progress-retirement was falsely accepted.");
  if (toolJobActorProgressOverlayExact(actorProgress, shardProgress, wgpuProgress.replace('job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").terminal_is_empty()', "true"))) throw new Error("[verify interactivity tool-jobs] self-test realm-close-without-presentation-terminal-witness was falsely accepted.");
  if (toolJobActorProgressOverlayExact(actorProgress, shardProgress, wgpuProgress.replace("let index = self.oldest_ready_index()?", "let index = self.take_cursor"))) throw new Error("[verify interactivity tool-jobs] self-test cancelled-presentation-head-strands-ready-owner was falsely accepted.");
}

/** 🔩 Checks the mounted fem2d session sources and every single-edit mutation of them. */
export function fem2dMountedSessionChecks() {
  const femMountedSession = [
    'pub const FEM2D_MOUNTED_JOB_KIND: &str = "semio.fem2d.mounted-analysis";',
    "const SESSION_ACTIVE_CAPACITY: usize = 32;",
    "const SESSION_SHELL_CAPACITY: usize = 64;",
    "const SESSION_MAXIMUM_INPUT_ITEMS: usize = 4_096;",
    "const SESSION_MAXIMUM_INPUT_BYTES: usize = 4 * 1_024 * 1_024;",
    "struct MountedProcessOwnerCatalog { claims: [MountedOwnerClaim; 30] }",
    "MountedOwnerClass::AssemblyDofStrings;",
    "MountedOwnerClass::MeshPreparationIndexVector; MountedOwnerClass::MeshTriangulationWorkspaceVectors; MountedOwnerClass::MeshEdgeIndexVectors;",
    "fn process_owner_inventory_admits_exact_maximum_and_returns_exact_credit() {}",
    "const FEM2D_JOB_TAG: u64 = 0xf2d0_0000_0000_0000;",
    "job & !FEM2D_JOB_COUNTER_MAXIMUM != FEM2D_JOB_TAG;",
    "shells: [Rc<RefCell<Option<MountedState>>>; SESSION_SHELL_CAPACITY],",
    "current: [Option<CurrentSession>; SESSION_ACTIVE_CAPACITY],",
    "retiring: [Option<u16>; SESSION_SHELL_CAPACITY],",
    "free: [u16; SESSION_SHELL_CAPACITY],",
    "canonical_base_revision: [u8; 32],",
    "preflight: [Option<PendingSnapshotAdmission>; SESSION_ACTIVE_CAPACITY],",
    "struct SnapshotAdmissionCursor {}",
    "fn step_one(&mut self, snapshot: &Fem2dSnapshot) {}",
    "credit_items: [usize; SESSION_SHELL_CAPACITY],",
    "credit_bytes: [usize; SESSION_SHELL_CAPACITY],",
    "visual_job_candidate: Option<Fem2dVisualJob>, visual_rejected: Option<Fem2dVisualJob>, visual_current: Option<Fem2dMountedVisualLease>, visual_displaced: Option<Fem2dMountedVisualLease>,",
    "fn publish_visual_candidate(&mut self, candidate: Fem2dMountedVisualLease) -> Result<(), Fem2dMountedVisualLease> {}",
    "snapshot.return_to_registry_witness(); witness.terminal_is_empty();",
    "register_bounded_job_kind(FEM2D_MOUNTED_JOB_KIND, factory);",
    "fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, ValueError> {}",
    "fn terminal_is_empty(&self) -> bool {}",
    "pub fn maintenance_step() {}",
    "pub fn close_step() {}",
    "pub fn terminal_is_empty() {}",
    "pub(super) fn step(&mut self, budget: JobBudget, cx: &mut StepContext<'_>) -> JobStep { graph.step(cx); mesh.step(cx); assembly.step(cx); pcg.step(cx); current_identity(self.identity.app_instance_id); commit_authority_matches(); self.cancel.is_cancelled_now(); JobStep::Done(output); }",
    "AssemblyJobConstruction::new_owned(model); AssemblyCsrBuild::new(assembly); PcgJobConstruction::new(operation, matrix);",
    "assembly_build.close_step(maximum_bytes);",
    "pub fn reconcile(doc: &ArtifactView) { doc.take_snapshot_read(); Effect::CancelJob; Effect::SpawnJob; JobPlacement::Isolated; }",
    "mounted_revision_restart_keeps_cancel_before_spawn",
    "mounted_close_and_capacity_are_fixed_and_terminal_witnessed",
  ].join("\n");
  const femMountedEditor = "fn pending_effects() { semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::reconcile(doc); } fn mounted_job_maintenance_step() {} fn mounted_job_close_step() {} fn mounted_jobs_terminal_is_empty() {} fn mounted_job_prepare_snapshot_read() {} session::with_live_visual(doc.render_operation(), |visual| model_window::render_with_progress(canvas, &window.camera, visual, &interaction, window_id.as_deref(), active_utility));";
  const femMountedModel = "pub struct Fem2dVisualJob {} Fem2dVisualJobStage::ReserveSnapshot; canvas2d_snapshot_begin(); self.output.admit_page(); canvas2d_snapshot_seal(token); Fem2dVisualJobStage::OrderRegionKey; Fem2dVisualJobStage::OrderElementKey; fn order_field_one(&mut self, visual: &Fem2dLiveVisual) {} Fem2dVisualJobStage::BuildRegion; fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {} fn mounted_visual_output_exact_maximum_plus_one_and_page_handback() {} fn fem2d_visual_job_maximum_plus_one_rejects_before_owner_transfer() {} fn fem2d_visual_job_stale_cancel_fault_and_device_close_preserve_last_valid() {} fn fem2d_visual_job_replay_accessibility_and_each_step_are_bounded() { assert!(packet_equal(&first, &second)); assert!(started.elapsed().as_micros() < 8_000); } pub fn render_with_progress() { fem2d_structure_layers_with(doc, interaction); progress.map(Fem2dMountedVisualLease::snapshot); }";
  const femMountedMesh = "MeshJobStage::ReservePreparation; MeshJobStage::ReserveEdgeAuthorities; fn advance_face_classification(&mut self) { triangulation.triangles.get(self.face_cursor); } MeshJobStage::Classify => { self.advance_face_classification(); } MeshJobStage::Finalize => {} fn mesh_edge_authority_uses_completed_faces_and_closes_exact_backing() {}";
  const femMountedRoot = 'pub fn plugin() { semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::initialize(); PluginBuilder::<_, FemApps>::new("fem"); }';
  const femMountedGlue = "semio-s-artifact-fem-2d-editor-session = { workspace = true }";
  const femMountedJobs = "pub trait BoundedJob {} JobBody::Bounded(owner);";
  const femMountedAnalyses = "pub struct AssemblyJobConstruction {} AssemblyConstructionStage::ValidateNodePairs; AssemblyConstructionStage::DiscoverDofs; AssemblyConstructionStage::CommitDofOwner; struct PendingElementBuild {} PendingElementBuildStage::ReserveIndices; PendingElementBuildStage::ReservePositions; PendingElementBuildStage::ReserveStiffnessCredit; PendingElementBuildStage::AllocateStiffness; PendingElementBuildStage::ObserveStiffnessBacking => {} PendingElementBuildStage::AdmitStiffnessBacking; fn advance_element_build(&mut self) {} fn reclaim_element_owner(&mut self) {} fn mounted_element_build_reserves_and_reclaims_one_exact_owner_per_turn() {} fn mounted_element_stiffness_observes_before_admit_and_retires_rejected_backing() {} pub struct AssemblyCsrBuild {}";
  const femMountedReactor = "Event::JobProgress { job, .. }; JOB_RENDER_BINDINGS.with; dirty_background_surfaces(runtime, binding.instance, &mut *dirty).await?; dirty.try_surface(instance, surface);";
  const femMountedStore = "pub fn commit_authority_matches() {} fn publish_authority() {} pub struct SnapshotReadReturn {} pub fn return_to_registry_witness() {}";
  const femMountedFramework = "let snapshot_is_admitted = { A::mounted_job_prepare_snapshot_read(render_operation, snapshot.as_ref()) }; self.stores.store.snapshot_read();";
  const femExact = (session = femMountedSession, editor = femMountedEditor, reactor = femMountedReactor, store = femMountedStore, framework = femMountedFramework, analyses = femMountedAnalyses, model = femMountedModel, mesh = femMountedMesh) =>
    toolJobFem2dMountedSessionExact(session, editor, model, mesh, femMountedRoot, femMountedGlue, femMountedJobs, analyses, reactor, store, framework);
  if (!femExact()) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-valid was falsely rejected.");
  if (femExact(femMountedSession.replace("current: [Option<CurrentSession>; SESSION_ACTIVE_CAPACITY]", "current: HashMap<u32, CurrentSession>"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-resizable-session-registry was falsely accepted.");
  if (femExact(femMountedSession.replace("canonical_base_revision: [u8; 32],", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-truncated-base-authority was falsely accepted.");
  if (femExact(femMountedSession.replace("job & !FEM2D_JOB_COUNTER_MAXIMUM != FEM2D_JOB_TAG;", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-job-id-aba-substitution was falsely accepted.");
  if (femExact(femMountedSession.replace("fn step_one(&mut self, snapshot: &Fem2dSnapshot) {}", "fn whole_snapshot() {}"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-whole-nested-input-preflight was falsely accepted.");
  if (femExact(femMountedSession.replace("commit_authority_matches();", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-unvalidated-commit was falsely accepted.");
  if (femExact(femMountedSession.replace("Effect::CancelJob;", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-revision-restart-without-cancel was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor.replace("session::with_live_visual(doc.render_operation(), |visual| model_window::render_with_progress(canvas, &window.camera, visual, &interaction, window_id.as_deref(), active_utility));", "model_window::render_with_progress(canvas, &window.camera, None, &interaction, window_id.as_deref(), active_utility);"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-live-visual-not-consumed was falsely accepted.");
  if (femExact(femMountedSession.replace("pcg.step(cx);", "while pcg.running() { pcg.step(cx); }"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-run-to-completion-child was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor.replace("dirty_background_surfaces(runtime, binding.instance, &mut *dirty).await?;", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-progress-without-render-invalidation was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore, femMountedFramework.replace("A::mounted_job_prepare_snapshot_read(render_operation, snapshot.as_ref())", "true"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-snapshot-issued-before-retained-census was falsely accepted.");
  if (femExact(femMountedSession.replace("assembly_build.close_step(maximum_bytes);", "self.assembly_build = None;"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-interrupted-builder-close-was-wholesale was falsely accepted.");
  if (femExact(femMountedSession.replace("AssemblyJobConstruction::new_owned(model)", "AssemblyJob::new_owned(model)"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-monolithic-assembly-plan-constructor was falsely accepted.");
  if (femExact(femMountedSession.replace("credit_bytes: [usize; SESSION_SHELL_CAPACITY],", "credit_bytes: Vec<usize>,"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-process-byte-credit-was-resizable was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore.replace("fn publish_authority() {}", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-store-authority-was-not-atomic was falsely accepted.");
  if (femExact(femMountedSession.replace("claims: [MountedOwnerClaim; 30]", "blanket_process_items: usize"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-blanket-process-credit-was falsely accepted.");
  if (femExact(femMountedSession.replace("MountedOwnerClass::AssemblyDofStrings;", "MountedOwnerClass::Unknown;"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-missing-assembly-string-owner-credit-was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore, femMountedFramework, femMountedAnalyses.replace("struct PendingElementBuild {}", "fn begin_element() { let node_ids = element.node_ids(); }"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-whole-element-constructor-was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore, femMountedFramework, femMountedAnalyses.replace("AssemblyConstructionStage::CommitDofOwner;", "plan.dof_map.order.push((node_id.clone(), dof));"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-dof-owner-clone-and-commit-collapsed-was falsely accepted.");
  if (femExact(femMountedSession.replace("fn process_owner_inventory_admits_exact_maximum_and_returns_exact_credit() {}", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-process-credit-boundary-handback-fixture-was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore, femMountedFramework, femMountedAnalyses, femMountedModel.replace("progress.map(Fem2dMountedVisualLease::snapshot)", "fem2d_live_visual_layers(doc, progress)")))
    throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-whole-visual-encoder-restored-was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore, femMountedFramework, femMountedAnalyses, femMountedModel.replace("fn order_field_one(&mut self, visual: &Fem2dLiveVisual)", "fn fields_sort()")))
    throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-visual-stable-order-cursor-removed-was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore, femMountedFramework, femMountedAnalyses, femMountedModel.replace("mounted_visual_output_exact_maximum_plus_one_and_page_handback", "mounted_visual_capacity_smoke")))
    throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-visual-max-plus-one-law-removed-was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore, femMountedFramework, femMountedAnalyses, femMountedModel.replace("fem2d_visual_job_stale_cancel_fault_and_device_close_preserve_last_valid", "visual_close_smoke")))
    throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-stale-cancel-close-law-removed-was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore, femMountedFramework, femMountedAnalyses, femMountedModel.replace("assert!(packet_equal(&first, &second))", "assert!(true)")))
    throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-replay-law-removed-was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore, femMountedFramework, femMountedAnalyses, femMountedModel.replace("started.elapsed().as_micros() < 8_000", "true")))
    throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-timing-law-removed-was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore, femMountedFramework, femMountedAnalyses, femMountedModel, femMountedMesh.replace("triangulation.triangles.get(self.face_cursor)", "triangulation.triangles.iter().collect()")))
    throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-mesh-classification-collect-restored-was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore, femMountedFramework, femMountedAnalyses.replace("PendingElementBuildStage::ObserveStiffnessBacking", "PendingElementBuildStage::Complete")))
    throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-stiffness-observation-removed-was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor, femMountedStore.replace("pub fn return_to_registry_witness() {}", "")))
    throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-store-return-witness-removed-was falsely accepted.");
  if (femExact(femMountedSession.replace("-> Result<(), Fem2dMountedVisualLease>", "-> bool")))
    throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-full-visual-slot-lost-exact-handback-was falsely accepted.");
}
