import { toolJobStaticRows, toolJobDispositions, toolJobProofs, toolJobAppOwnedRows, toolJobProofCatalogFailures, toolJobProductionProtocolSource, toolJobGlobalPayloadStoreInventory, toolJobGlobalPayloadStoreIsSharedFixedAuthority, toolJobActorProgressOverlayExact, toolJobFem2dMountedSessionExact, toolJobScanThenMonolithRows, toolJobFrameworkReservedRoutesExact, toolJobDecodeAfterAdmission, toolJobLimitsMatch, toolJobExternalCancellationOwned, toolJobRuntimeProofQualified, toolJobQualifiedProofBeforeDecode, toolJobLiveInstanceIsolated, toolJobTypedDispatchExact, toolJobWireDispatchExact, TOOL_JOB_RESERVED_IDS, toolJobImportPreparationBounded, toolJobFullOperationBounded, toolJobStoreOneItemPublicationBounded, toolJobEphemeralOneItemPublicationBounded, toolJobPublicationFreshnessBeforeEveryTurn, toolJobPublicationContracts, toolJobPublicationAuthorityReady, toolJobTypedRouteFailsClosedBeforePreparation, toolJobTypedPersistentFoundation, toolJobDropCancellationBounded, toolJobCancellationScopesBounded, toolJobHardBoundedCloseExact, toolJobErasedCloseTerminalExact, toolJobRuntimeCloseCallbackBounded, toolJobDetachedOutputOwnershipExact, toolJobLiveConstructionCleanupExact, toolJobSnapshotRetirementBounded, toolJobVcsOwnedDisposalExplicit, toolJobRuntimeRegistryFixedClose, toolJobReactorCloseBounded, toolJobOpaqueFutureProductionFailClosed, toolJobSegmentedQueueHardBounded, toolJobSegmentedTerminalDrainExact, toolJobPuzzleReservedRoutesExact, toolJobMediaExportBounded, toolJobImmutableOperationRootsExact, toolJobChildContentRootExact, toolJobMemberStoreOwnerExact, toolJobArtifactStoreStructuralOwnersExact, toolJobHistoryLedgerAdmissionExact, toolJobArtifactResolutionCandidateExact, toolJobArtifactEditMessageLedgerExact, toolJobArtifactEnvelopeOwnedCodecExact, toolJobPresentationEnvelopeCallerRetainedExact, toolJobWriterEnvelopeCallerRetainedExact, toolJobJackEnvelopeCallerRetainedExact, toolJobTrinityRewriteEnvelopeCallerRetainedExact, toolJobGisMapEnvelopeCallerRetainedExact, toolJobRasterEnvelopeCallerRetainedExact, toolJobDrawingEnvelopeCallerRetainedExact, toolJobPeerInteractionRootsExact, toolJobPagedIngressExact, toolJobUniversalRetainedOwnershipExact, toolJobLayoutColdRelayRetainedExact } from "../../../../../📜️script.ts";
import { toolJobCheckpointSelfTests } from "../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-checkpoint/🟦️.ts";
import { toolJobScalarConfigCohortSelfTests } from "../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts";
import { storeCanonicalEditSealerSelfTests } from "../../../../🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts";
import { toolJobPuzzleReservedRoutesSelfTests } from "../../../../../✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️tool-job-puzzle-reserved-routes/🟦️.ts";
import { toolJobOwnerFactoryResolutionSelfTests } from "../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-owner-factory-resolution/🟦️.ts";
import { toolJobFactoryProofJoinSelfTests } from "../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-factory-proof-join/🟦️.ts";
import { toolJobLatestWinsSelfTests } from "../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts";
import { toolJobMicrosecondBudgetSelfTests } from "../../⏱️budget/🧪️tests/🔬️tool-job-microsecond-budget/🟦️.ts";
import { toolJobTelemetryContentionSelfTests } from "../../../⏱️trace/⏱️clock/🧪️tests/🔬️tool-job-telemetry-contention/🟦️.ts";
import { toolJobCooperativeMaintenanceSelfTests } from "../../../⏳️async/🤝️cooperative/🧪️tests/🔬️tool-job-cooperative-maintenance/🟦️.ts";
import { cadPresenceRetirementSelfTests } from "../../../../../✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts";
import { proceduralGenerationRootSelfTests } from "../../../../🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🧪️tests/🔬️procedural-generation-root/🟦️.ts";
import { flowTypedRetirementSelfTests } from "../../../../🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🔬️flow-typed-retirement/🟦️.ts";
import { flowSelectedCopySelfTests } from "../../../../🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/🔬️flow-selected-copy/🟦️.ts";

/** 🧪️ Executes tool job coverage policy assertions. */
export function toolJobCoverageSelfTests(): number {
  const evaluate = (sources: Record<string, string>) => {
    const files = new Map(Object.entries(sources));
    const rows = toolJobStaticRows(files);
    const dispositions = toolJobDispositions(files);
    const proofs = toolJobProofs(files);
    const appOwned = new Set(toolJobAppOwnedRows(files).map((row) => `${row.ownerFile}\0${row.toolId}`));
    const catalogClean = toolJobProofCatalogFailures(files, rows, dispositions, proofs).length === 0;
    const registered = new Set(rows.map((row) => row.id));
    const remaining = rows.filter((row) => {
      const disposition = dispositions.get(`${row.file}\0${row.id}`);
      const unbounded = /for\s+item\s+in\s+items\b|while\s+true\b/.test(sources[row.file] ?? "");
      const proof = catalogClean ? proofs.find((candidate) => candidate.ownerFile === row.file && candidate.ownerTypeName.length > 0 && candidate.controllerId.length > 0 && candidate.factory.length > 0 && candidate.toolId === row.id && candidate.documentSchema.length > 0) : undefined;
      return !["BatchOnlyPendingRewrite", "ForbiddenFromUi", "Deleted"].includes(disposition ?? "") && !(disposition === "Migrated" && proof && !unbounded && appOwned.has(`${row.file}\0${row.id}`));
    });
    for (const [file, source] of files) {
      for (const match of source.matchAll(/dispatch\(\s*"([^"]+)"/g)) if (!registered.has(match[1]!)) remaining.push({ file, id: match[1]!, source: "literal" });
    }
    return remaining;
  };
  const fixtures: readonly [string, Record<string, string>][] = [
    ["old-775-default", { "plugin.rs": 'app_commands! { "one" => one::Payload, "two" => two::Payload }' }],
    ["missing-alias-fallback", { "framework.rs": 'fn bounded_first_step_contract(id: &str) { match id { "exact" => Some(semio_framework::ToolExecutionContract::bounded_first_step(64, 1, 1, 64, 100)), _ => None } }', "plugin.rs": 'app_commands! { "exact" => exact::Payload }\n.action_interactive_job("exact", InteractiveJobClassification::Migrated)\ndispatch("alias");' }],
    ["nonmacro-puzzle", { "puzzle.rs": '.mutation("scramble", label, kind)' }],
    ["unbounded-handler", { "framework.rs": 'fn bounded_first_step_contract(id: &str) { match id { "export" => Some(semio_framework::ToolExecutionContract::bounded_first_step(64, 1, 1, 64, 100)), _ => None } }', "plugin.rs": 'app_commands! { "export" => export::Payload }\n.action_interactive_job("export", InteractiveJobClassification::Migrated)\nfn handle(items: &[u8]) { for item in items { work(item); } }' }],
    ["default-migrated-builder", { "manifest.rs": 'fn bounded_catalog() { definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated; }', "plugin.rs": 'app_commands! { "solve" => solve::Payload }' }],
  ];
  for (const [name, sources] of fixtures) if (evaluate(sources).length === 0) throw new Error(`[verify interactivity tool-jobs] self-test ${name} was falsely accepted.`);
  const checkpointChecks = toolJobCheckpointSelfTests();
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
    "fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {}",
    "fn terminal_is_empty(&self) -> bool {}",
    "pub fn maintenance_step() {}",
    "pub fn close_step() {}",
    "pub fn terminal_is_empty() {}",
    "fn step(&mut self, budget: JobBudget) { graph.step(&mut cx); mesh.step(&mut cx); assembly.step(&mut cx); pcg.step(&mut cx); current_identity(self.identity.app_instance_id); commit_authority_matches(); self.cancel.is_cancelled_now(); JobStep::Done(output); }",
    "AssemblyJobConstruction::new_owned(model); AssemblyCsrBuild::new(assembly); PcgJobConstruction::new(operation, matrix);",
    "assembly_build.close_step(maximum_bytes);",
    "pub fn reconcile(doc: &ArtifactView) { doc.take_snapshot_read(); Effect::CancelJob; Effect::SpawnJob; JobPlacement::Isolated; }",
    "mounted_revision_restart_keeps_cancel_before_spawn",
    "mounted_close_and_capacity_are_fixed_and_terminal_witnessed",
  ].join("\n");
  const femMountedEditor = "fn pending_effects() { crate::editor::fem2d::session::reconcile(doc); } fn mounted_job_maintenance_step() {} fn mounted_job_close_step() {} fn mounted_jobs_terminal_is_empty() {} fn mounted_job_prepare_snapshot_read() {} session::with_live_visual(doc.render_operation(), |visual| model_window::render_with_progress(doc.snapshot, camera, visual));";
  const femMountedModel = "pub struct Fem2dVisualJob {} Fem2dVisualJobStage::ReserveSnapshot; canvas2d_snapshot_begin(); self.output.admit_page(); canvas2d_snapshot_seal(token); Fem2dVisualJobStage::OrderRegionKey; Fem2dVisualJobStage::OrderElementKey; fn order_field_one(&mut self, visual: &Fem2dLiveVisual) {} Fem2dVisualJobStage::BuildRegion; fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {} fn mounted_visual_output_exact_maximum_plus_one_and_page_handback() {} fn fem2d_visual_job_maximum_plus_one_rejects_before_owner_transfer() {} fn fem2d_visual_job_stale_cancel_fault_and_device_close_preserve_last_valid() {} fn fem2d_visual_job_replay_accessibility_and_each_step_are_bounded() { assert!(packet_equal(&first, &second)); assert!(started.elapsed().as_micros() < 8_000); } pub fn render_with_progress() { progress.map(Fem2dMountedVisualLease::snapshot); }";
  const femMountedMesh = "MeshJobStage::ReservePreparation; MeshJobStage::ReserveEdgeAuthorities; fn advance_face_classification(&mut self) { triangulation.triangles.get(self.face_cursor); } MeshJobStage::Classify => { self.advance_face_classification(); } MeshJobStage::Finalize => {} fn mesh_mounted_classification_indexes_admit_maximum_reject_plus_one_and_close_exactly() {}";
  const femMountedRoot = 'pub fn plugin() { crate::editor::fem2d::session::initialize(); Plugin::<FemApps>::builder("fem"); }';
  const femMountedGlue = "✏️editor/🧵️session/🦀️.rs";
  const femMountedJobs = "pub trait BoundedJob {} JobBody::Bounded(owner); #[cfg(not(test))]; JobBody::ExplicitStateMachineRequired;";
  const femMountedAnalyses = "pub struct AssemblyJobConstruction {} AssemblyConstructionStage::ValidateNodePairs; AssemblyConstructionStage::DiscoverDofs; AssemblyConstructionStage::CommitDofOwner; struct PendingElementBuild {} PendingElementBuildStage::ReserveIndices; PendingElementBuildStage::ReservePositions; PendingElementBuildStage::ReserveStiffnessCredit; PendingElementBuildStage::AllocateStiffness; PendingElementBuildStage::ObserveStiffnessBacking => {} PendingElementBuildStage::AdmitStiffnessBacking; fn advance_element_build(&mut self) {} fn reclaim_element_owner(&mut self) {} fn mounted_element_build_reserves_and_reclaims_one_exact_owner_per_turn() {} fn mounted_element_stiffness_observes_before_admit_and_retires_rejected_backing() {} pub struct AssemblyCsrBuild {}";
  const femMountedReactor = "Event::JobProgress { job, .. }; JOB_RENDER_BINDINGS.with; .try_surface(binding.instance, surface);";
  const femMountedStore = "pub fn commit_authority_matches() {} fn publish_authority() {} pub struct SnapshotReadReturn {} pub fn return_to_registry_witness() {}";
  const femMountedFramework = "let snapshot_is_admitted = { A::mounted_job_prepare_snapshot_read(render_operation, snapshot.as_ref()) }; self.store.snapshot_read();";
  const femExact = (session = femMountedSession, editor = femMountedEditor, reactor = femMountedReactor, store = femMountedStore, framework = femMountedFramework, analyses = femMountedAnalyses, model = femMountedModel, mesh = femMountedMesh) =>
    toolJobFem2dMountedSessionExact(session, editor, model, mesh, femMountedRoot, femMountedGlue, femMountedJobs, analyses, reactor, store, framework);
  if (!femExact()) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-valid was falsely rejected.");
  if (femExact(femMountedSession.replace("current: [Option<CurrentSession>; SESSION_ACTIVE_CAPACITY]", "current: HashMap<u32, CurrentSession>"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-resizable-session-registry was falsely accepted.");
  if (femExact(femMountedSession.replace("canonical_base_revision: [u8; 32],", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-truncated-base-authority was falsely accepted.");
  if (femExact(femMountedSession.replace("job & !FEM2D_JOB_COUNTER_MAXIMUM != FEM2D_JOB_TAG;", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-job-id-aba-substitution was falsely accepted.");
  if (femExact(femMountedSession.replace("fn step_one(&mut self, snapshot: &Fem2dSnapshot) {}", "fn whole_snapshot() {}"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-whole-nested-input-preflight was falsely accepted.");
  if (femExact(femMountedSession.replace("commit_authority_matches();", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-unvalidated-commit was falsely accepted.");
  if (femExact(femMountedSession.replace("Effect::CancelJob;", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-revision-restart-without-cancel was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor.replace("session::with_live_visual(doc.render_operation(), |visual| model_window::render_with_progress(doc.snapshot, camera, visual));", "model_window::render_with_progress(doc.snapshot, camera, None);"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-live-visual-not-consumed was falsely accepted.");
  if (femExact(femMountedSession.replace("pcg.step(&mut cx);", "while pcg.running() { pcg.step(&mut cx); }"))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-run-to-completion-child was falsely accepted.");
  if (femExact(femMountedSession, femMountedEditor, femMountedReactor.replace(".try_surface(binding.instance, surface);", ""))) throw new Error("[verify interactivity tool-jobs] self-test mounted-fem2d-progress-without-render-invalidation was falsely accepted.");
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
  const exactProof = (owner: string, controller: string, id: string, raw = 64) => {
    const ownerType = `Owner${owner.replaceAll(/[^A-Za-z0-9]/g, "")}`;
    return `impl ArtifactEditor for ${ownerType} { const DOCUMENT_SCHEMA: &'static str = "fixture.document"; }\nbounded_first_step_tool_proofs! { owner: semio_framework_plugin::EditorApp<${ownerType}>, owner_file: "${owner}", controller: "${controller}", document_schema: "fixture.document", factory: "BoundedFirstStepCommandJobFactory", tools: { "${id}" => semio_framework::ToolExecutionContract::bounded_first_step(${raw}, 1, 1, 64, 100), } }`;
  };
  const scanThenMonolithSource = `${exactProof("owner.rs", "owner@1/*#editor", "scan").replace("bounded_first_step(64, 1, 1, 64, 100)", "resumable(64, 1, 1, 64, 100, 1, 1)")}
fn scan_retained_reduce(command: &Command) { command.dispatch(); }
impl ArtifactCommandWork<EditorApp<Ownerownerrs>> for ScanWork {
  fn step(&mut self, command: &Command) { ArtifactCommandWorkStep::Progress { stage: "fixture-command-scan", preview: b"{}" }; scan_retained_reduce(command); }
}`;
  const scanThenMonolithFiles = new Map([["owner.rs", scanThenMonolithSource]]);
  const scanThenMonolithProofs = toolJobProofs(scanThenMonolithFiles);
  if (toolJobScanThenMonolithRows(scanThenMonolithFiles, scanThenMonolithProofs).length !== 1)
    throw new Error("[verify interactivity tool-jobs] self-test scan-then-monolithic reducer was falsely accepted.");
  if (toolJobScanThenMonolithRows(new Map([["owner.rs", scanThenMonolithSource.replace("scan_retained_reduce(command);", "workspace.step_one(command);")]]), scanThenMonolithProofs).length !== 0)
    throw new Error("[verify interactivity tool-jobs] self-test segmented workspace reducer was falsely rejected.");
  const restoredFinalDispatch = scanThenMonolithSource.replace("scan_retained_reduce(command);", "workspace.step_one(command);").replace("workspace.step_one(command);", "command.dispatch();");
  if (toolJobScanThenMonolithRows(new Map([["owner.rs", restoredFinalDispatch]]), scanThenMonolithProofs).length !== 1)
    throw new Error("[verify interactivity tool-jobs] self-test restored final one-shot dispatch was falsely accepted.");
  const duplicateOwner = {
    "owner-a.rs": `app_commands! { "same" => same::Payload }\n.action_interactive_job("same", InteractiveJobClassification::Migrated)\n${exactProof("owner-a.rs", "owner-a@1/*#editor", "same")}`,
    "owner-b.rs": 'app_commands! { "same" => same::Payload }\n.action_interactive_job("same", InteractiveJobClassification::Migrated)',
  };
  const duplicateRemaining = evaluate(duplicateOwner);
  if (duplicateRemaining.length !== 2) throw new Error("[verify interactivity tool-jobs] self-test declaration-without-proof was falsely accepted.");
  const validCatalog = { "owner.rs": `app_commands! { "bounded" => bounded::Payload }\n.action_interactive_job("bounded", InteractiveJobClassification::Migrated)\n${exactProof("owner.rs", "owner@1/*#editor", "bounded")}` };
  if (evaluate(validCatalog).length !== 1) throw new Error("[verify interactivity tool-jobs] self-test proof-only-route was falsely accepted as executable.");
  const typedVariantCatalog = { "owner.rs": `const BOUNDED_ROUTE: &str = "bounded";\npuzzle3d_command_variants! { One = BOUNDED_ROUTE }\n.action_interactive_job("bounded", InteractiveJobClassification::Migrated)\n${exactProof("owner.rs", "owner@1/*#editor", "bounded")}` };
  if (evaluate(typedVariantCatalog).length !== 1) throw new Error("[verify interactivity tool-jobs] self-test typed-command-variant-proof-only route was falsely accepted as executable.");
  const unresolvedTypedVariant = new Map([["owner.rs", "puzzle3d_command_variants! { One = COPIED_UNRESOLVED_ROUTE }"]]);
  if (toolJobStaticRows(unresolvedTypedVariant).length !== 0) throw new Error("[verify interactivity tool-jobs] self-test unresolved typed-command variant was falsely accepted.");
  const sharedProof = exactProof("owner.rs", "owner@1/*#editor", "bounded").replace(
    /tools:\s*\{\s*"bounded"\s*=>\s*(semio_framework::ToolExecutionContract::bounded_first_step\([^)]*\)),\s*\}/,
    'contract: $1, tools: ["bounded"]',
  );
  const sharedCatalog = { "owner.rs": `app_commands! { "bounded" => bounded::Payload }\n.action_interactive_job("bounded", InteractiveJobClassification::Migrated)\n${sharedProof}` };
  if (evaluate(sharedCatalog).length !== 1) throw new Error("[verify interactivity tool-jobs] self-test shared-contract-proof-only route was falsely accepted as executable.");
  const executableOwner = `
${validCatalog["owner.rs"].replace("impl ArtifactEditor for Ownerownerrs { const DOCUMENT_SCHEMA: &'static str = \"fixture.document\"; }", "").replace('factory: "BoundedFirstStepCommandJobFactory",', 'factory: "AppCommandJobFactory", factory_type: AppCommandJobFactory,')}
const APP_COMMAND_TOOL_IDS: &'static [&'static str] = &["bounded"];
struct AppCommandJobFactory;
impl ToolJobFactory for AppCommandJobFactory {
  type Payload = Payload;
  type Job = Job;
  fn execution_contract(&self) -> ToolExecutionContract { contract() }
  fn create_job(&mut self, operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { job(operation, payload) }
}
impl ArtifactOwnedToolJobFactory for AppCommandJobFactory {
  type Owner = semio_framework_plugin::EditorApp<Ownerownerrs>;
  const TOOL_IDS: &'static [&'static str] = APP_COMMAND_TOOL_IDS;
}
impl ArtifactEditor for Ownerownerrs {
  const DOCUMENT_SCHEMA: &'static str = "fixture.document";
  fn register_tool_job_factories(registry: &mut Registry) { registry.register(AppCommandJobFactory::new("owner")); }
  fn build_tool_job(request: Request) -> Result<Option<ToolOperationSpec>, Fault> { build(request) }
}`;
  if (evaluate({ "owner.rs": executableOwner }).length !== 0) throw new Error("[verify interactivity tool-jobs] self-test exact app-owned executable route was falsely rejected.");
  const directExecutableOwner = executableOwner
    .replaceAll("Ownerownerrs", "DirectOwner")
    .replaceAll("impl ArtifactEditor for DirectOwner", "impl ArtifactApp for DirectOwner")
    .replaceAll("semio_framework_plugin::EditorApp<DirectOwner>", "DirectOwner");
  if (evaluate({ "owner.rs": directExecutableOwner }).length !== 0)
    throw new Error("[verify interactivity tool-jobs] self-test direct ArtifactApp executable route was falsely rejected.");
  if (evaluate({ "owner.rs": directExecutableOwner.replace("type Owner = DirectOwner;", "type Owner = ForgedDirectOwner;") }).length === 0)
    throw new Error("[verify interactivity tool-jobs] self-test forged direct ArtifactApp owner was falsely accepted.");
  const qualifiedSchemaOwner = `const FIXTURE_DOCUMENT_SCHEMA: &str = "fixture.document";\n${executableOwner.replace(
    'const DOCUMENT_SCHEMA: &\'static str = "fixture.document";',
    "const DOCUMENT_SCHEMA: &'static str = crate::schema::FIXTURE_DOCUMENT_SCHEMA;",
  )}`;
  const qualifiedSchemaFiles = new Map([["owner.rs", qualifiedSchemaOwner]]);
  const qualifiedSchemaFailures = toolJobProofCatalogFailures(
    qualifiedSchemaFiles,
    toolJobStaticRows(qualifiedSchemaFiles),
    toolJobDispositions(qualifiedSchemaFiles),
    toolJobProofs(qualifiedSchemaFiles),
  );
  if (qualifiedSchemaFailures.some((failure) => failure.includes("forged bounded reducer document schema")))
    throw new Error("[verify interactivity tool-jobs] self-test qualified document-schema constant was falsely rejected.");
  if (evaluate({ "owner.rs": executableOwner.replace("registry.register(AppCommandJobFactory::new(\"owner\"));", "") }).length === 0)
    throw new Error("[verify interactivity tool-jobs] self-test app-owned factory without registration was falsely accepted.");
  if (evaluate({ "owner.rs": executableOwner.replace("fn build_tool_job(request: Request) -> Result<Option<ToolOperationSpec>, Fault> { build(request) }", "") }).length === 0)
    throw new Error("[verify interactivity tool-jobs] self-test app-owned factory without builder was falsely accepted.");
  const duplicateCatalog = { "owner.rs": validCatalog["owner.rs"]!.replace('tools: { "bounded" =>', 'tools: { "bounded" => semio_framework::ToolExecutionContract::bounded_first_step(64, 1, 1, 64, 100), "bounded" =>') };
  if (evaluate(duplicateCatalog).length === 0) throw new Error("[verify interactivity tool-jobs] self-test duplicate-proof was falsely accepted.");
  const forgedOwner = { "owner.rs": validCatalog["owner.rs"]!.replace("EditorApp<Ownerownerrs>", "EditorApp<ForgedOwner>") };
  if (evaluate(forgedOwner).length === 0) throw new Error("[verify interactivity tool-jobs] self-test forged-owner was falsely accepted.");
  const forgedSchema = { "owner.rs": validCatalog["owner.rs"]!.replace('document_schema: "fixture.document"', 'document_schema: "forged.document"') };
  if (evaluate(forgedSchema).length === 0) throw new Error("[verify interactivity tool-jobs] self-test forged-schema was falsely accepted.");
  const forgedFactory = { "owner.rs": validCatalog["owner.rs"]!.replace('factory: "BoundedFirstStepCommandJobFactory"', 'factory: "ForgedFactory"') };
  if (evaluate(forgedFactory).length === 0) throw new Error("[verify interactivity tool-jobs] self-test forged-factory was falsely accepted.");
  const extraProof = { "owner.rs": exactProof("owner.rs", "owner@1/*#editor", "extra") };
  if (toolJobProofCatalogFailures(new Map(Object.entries(extraProof)), [], new Map(), toolJobProofs(new Map(Object.entries(extraProof)))).length === 0) throw new Error("[verify interactivity tool-jobs] self-test extra-proof was falsely accepted.");
  const reservedBypass = 'async fn dispatch_action() { interactive-job.unknown-key; return self.dispatch_framework_reserved_action(action, args, meta).await; } async fn dispatch_framework_reserved_action() { self.store.dispatch(command); }';
  if (toolJobFrameworkReservedRoutesExact(reservedBypass)) throw new Error("[verify interactivity tool-jobs] self-test registryless-reserved-bypass was falsely accepted.");
  const decodeBeforeLookup = "async fn dispatch_action() { A::command_from_action(action, args).await; admit_command_json(action, args); } async fn dispatch_command() { A::command_from_action(action, args).await; admit_command_json(action, args); } async fn handle_intent_frame() { A::command_from_intent(intent).await; admit_command_wire(owner, tool); }";
  if (toolJobDecodeAfterAdmission(decodeBeforeLookup)) throw new Error("[verify interactivity tool-jobs] self-test decode-before-lookup was falsely accepted.");
  const mismatchProofs = toolJobProofs(new Map([["owner.rs", exactProof("owner.rs", "owner@1/*#editor", "bounded", 64)]]));
  if (toolJobLimitsMatch(mismatchProofs, new Map([["owner@1/*#editor\0bounded", 128]]))) throw new Error("[verify interactivity tool-jobs] self-test contract-mismatch was falsely accepted.");
  const externalCancellation = "let cancellation = JobScope::root(); session.step(cancel.cancel_token());";
  if (toolJobExternalCancellationOwned(externalCancellation)) throw new Error("[verify interactivity tool-jobs] self-test external-cancellation was falsely accepted.");
  const unqualifiedRuntime = "fn bounded_first_step_contract(document_schema, id) {} fn bounded_first_step_public_wire_limit(command_id) {}";
  if (toolJobRuntimeProofQualified(unqualifiedRuntime)) throw new Error("[verify interactivity tool-jobs] self-test runtime-owner-proof-drop was falsely accepted.");
  const proofAfterDecode = "async fn admit_command_json() { bounded_json_items(); qualified_tool_proof(); } async fn handle_intent_frame() { serde_json::to_value(); qualified_tool_proof(); }";
  if (toolJobQualifiedProofBeforeDecode(proofAfterDecode)) throw new Error("[verify interactivity tool-jobs] self-test qualified-proof-after-decode was falsely accepted.");
  const delegatedQualifiedProof =
    'async fn admit_command_json_with_proof(proof: QualifiedToolProof) { let key = proof.key(); begin_exact_wire(); serde_json::to_writer(); proof.admits::<A>(&admission); } async fn admit_command_json() { let proof = self.qualified_tool_proof(verb)?; admit_command_json_with_proof(proof, verb, args); } async fn admit_host_configuration_json() { let proof = self.qualified_host_configuration_tool_proof(verb)?; admit_command_json_with_proof(proof, verb, args); } async fn handle_intent_frame() { qualified_tool_proof(verb); FaultCode::new("interactive-job.intent-raw-owner-required"); }';
  if (!toolJobQualifiedProofBeforeDecode(delegatedQualifiedProof)) throw new Error("[verify interactivity tool-jobs] self-test delegated-qualified-proof route was falsely rejected.");
  if (toolJobQualifiedProofBeforeDecode(delegatedQualifiedProof.replace("let proof = self.qualified_tool_proof(verb)?; admit_command_json_with_proof(proof, verb, args);", "admit_command_json_with_proof(proof, verb, args); let proof = self.qualified_tool_proof(verb)?;")))
    throw new Error("[verify interactivity tool-jobs] self-test delegated app proof after helper was falsely accepted.");
  if (toolJobQualifiedProofBeforeDecode(delegatedQualifiedProof.replace("let proof = self.qualified_host_configuration_tool_proof(verb)?; admit_command_json_with_proof(proof, verb, args);", "admit_command_json_with_proof(proof, verb, args); let proof = self.qualified_host_configuration_tool_proof(verb)?;")))
    throw new Error("[verify interactivity tool-jobs] self-test delegated host proof after helper was falsely accepted.");
  if (toolJobQualifiedProofBeforeDecode(delegatedQualifiedProof.replace("begin_exact_wire(); serde_json::to_writer();", "serde_json::to_writer(); begin_exact_wire();")))
    throw new Error("[verify interactivity tool-jobs] self-test delegated helper decode before wire admission was falsely accepted.");
  const copiedOwnerStrings = 'struct QualifiedBoundedFirstStepProof; fn bounded_first_step_proof(controller_id: &str, factory: &str, tool_id: &str, document_schema: &str) { proof.owner_file != ""; } tool_job_registration(A::APP_ID, A::DOCUMENT_SCHEMA);';
  if (toolJobRuntimeProofQualified(copiedOwnerStrings)) throw new Error("[verify interactivity tool-jobs] self-test copied-owner-without-compiler-witness was falsely accepted.");
  const controllerScopedOperation = 'let app_id = app.instance_id().await.to_string(); format!("{}/{verb}/{}", self.tool_job_controller_id, base_revision.0); app_instance_id: meta.instance_id;';
  if (toolJobLiveInstanceIsolated(controllerScopedOperation)) throw new Error("[verify interactivity tool-jobs] self-test controller-scoped-runtime-instance was falsely accepted.");
  const typedAliasFallback = "pub fn dispatch(&self) { let exact = inner.aliases.get(&key); inner.factory_by_key.get(&exact); }";
  if (toolJobTypedDispatchExact(typedAliasFallback)) throw new Error("[verify interactivity tool-jobs] self-test typed-alias-fallback was falsely accepted.");
  const wireAliasFallback = "pub fn dispatch_wire(&self) { let exact = inner.aliases.get(&key); inner.factory_by_key.get(&exact); }";
  if (toolJobWireDispatchExact(wireAliasFallback)) throw new Error("[verify interactivity tool-jobs] self-test wire-alias-fallback was falsely accepted.");
  const genericReservedTerminal = `${TOOL_JOB_RESERVED_IDS.map((id, index) => `framework_reserved_job!(Job${index}, Factory${index}, "${id}", ${index}, 64, 1, 1, 64);`).join("\n")} impl FrameworkReservedCursor { fn step(&mut self) {} }`;
  if (toolJobFrameworkReservedRoutesExact(genericReservedTerminal)) throw new Error("[verify interactivity tool-jobs] self-test generic-reserved-terminal was falsely accepted.");
  const missingReservedFactory = TOOL_JOB_RESERVED_IDS.slice(1).map((id, index) => `framework_reserved_job!(Job${index}, Factory${index}, "${id}", ${index}, 64, 1, 1, 64);`).join("\n");
  if (toolJobFrameworkReservedRoutesExact(missingReservedFactory)) throw new Error("[verify interactivity tool-jobs] self-test missing-reserved-factory was falsely accepted.");
  const monolithicBinary = `${TOOL_JOB_RESERVED_IDS.map((id, index) => `framework_reserved_job!(Job${index}, Factory${index}, "${id}", ${index}, 64, 1, 1, 64);`).join("\n")} async fn dispatch_import_media() { A::import_media().await; } async fn dispatch_config_command_inner() { decode_op(); }`;
  if (toolJobFrameworkReservedRoutesExact(monolithicBinary)) throw new Error("[verify interactivity tool-jobs] self-test monolithic-reserved-binary was falsely accepted.");
  const earlyPermitFinish = `${TOOL_JOB_RESERVED_IDS.map((id, index) => `framework_reserved_job!(Job${index}, Factory${index}, "${id}", ${index}, 64, 1, 1, 64);`).join("\n")} struct FrameworkReservedCommitPermit; async fn dispatch_action() { interactive-job.unknown-key; return self.dispatch_framework_reserved_action(action, args, meta).await; } async fn dispatch_framework_reserved_action() { permit.finish(); let result = if HISTORY_ACTION_IDS {} }`;
  if (toolJobFrameworkReservedRoutesExact(earlyPermitFinish)) throw new Error("[verify interactivity tool-jobs] self-test early-reserved-permit-finish was falsely accepted.");
  const envelopeOnlyReserved = "struct Job { raw: Vec<u8>, envelope_cursor: usize } impl InteractiveJob for Job { fn step() { Ok(self.raw.clone()); } }";
  if (toolJobFrameworkReservedRoutesExact(envelopeOnlyReserved)) throw new Error("[verify interactivity tool-jobs] self-test envelope-only-reserved-job was falsely accepted.");
  const postJobMonolith = "async fn dispatch_framework_reserved_action() { run_framework_reserved_job().await; dispatch_history_action().await; } fn ensure_reserved_emit_bounded() { serde_json::to_vec(&emit); }";
  if (toolJobFrameworkReservedRoutesExact(postJobMonolith)) throw new Error("[verify interactivity tool-jobs] self-test post-job-monolithic-operation was falsely accepted.");
  const preJobImportSerialization = "async fn dispatch_import_media() { let raw = serde_json::to_vec(&(port, media)); build_artifact_reserved_media_job(port, media, raw); }";
  if (toolJobImportPreparationBounded(preJobImportSerialization)) throw new Error("[verify interactivity tool-jobs] self-test pre-job-import-serialization was falsely accepted.");
  const escapedTypedPreparation = "async fn dispatch_typed_command_inner() { refresh_cache().await; draft_store.snapshot().await; let session = WorkerJobSession::new(job); dispatch_emit().await; }";
  if (toolJobFullOperationBounded(escapedTypedPreparation)) throw new Error("[verify interactivity tool-jobs] self-test typed-preparation-and-commit-outside-job was falsely accepted.");
  const fakePhaseCursor = 'struct TypedCommandFullOperationJob; impl<A: ArtifactApp> InteractiveJob for TypedCommandFullOperationJob<A> { fn step() { set_stage("typed-command-prepare"); set_stage("typed-command-reducer"); set_stage("typed-command-output-validation"); set_stage("typed-command-ephemeral"); set_stage("typed-command-emit"); set_stage("typed-command-expose"); } } async fn dispatch_typed_command_inner() { let session = WorkerJobSession::new(job); }';
  if (toolJobFullOperationBounded(fakePhaseCursor)) throw new Error("[verify interactivity tool-jobs] self-test fake-full-operation-phase-cursor was falsely accepted.");
  const monolithicHugeOutput = 'struct TypedCommandFullOperationJob; impl<A: ArtifactApp> semio_framework_job::InteractiveJob for TypedCommandFullOperationJob<A> { fn step() { cx.is_cancelled(); cx.should_yield(); max_decoded_items; max_output_bytes; PreviewReady; CheckpointReady; validate_commit(base_revision, generation); set_stage("typed-command-prepare"); set_stage("typed-command-reducer"); set_stage("typed-command-output-validation"); serde_json::to_vec(&emit.effects); for child in emit.child_emits.iter() {} set_stage("typed-command-ephemeral"); set_stage("typed-command-emit"); set_stage("typed-command-expose"); } } async fn dispatch_typed_command_inner() { let session = WorkerJobSession::new(job); }';
  if (toolJobFullOperationBounded(monolithicHugeOutput)) throw new Error("[verify interactivity tool-jobs] self-test monolithic-huge-output-and-child-validation was falsely accepted.");
  const monolithicReducer = 'struct TypedCommandFullOperationJob; impl<A: ArtifactApp> semio_framework_job::InteractiveJob for TypedCommandFullOperationJob<A> { fn step() { cx.is_cancelled(); cx.should_yield(); max_decoded_items; max_output_bytes; PreviewReady; CheckpointReady; validate_commit(base_revision, generation); set_stage("typed-command-prepare"); set_stage("typed-command-reducer"); resolve_ready(A::ephemeral(command, doc)); resolve_ready(A::handle(command, doc)); set_stage("typed-command-output-validation"); set_stage("typed-command-ephemeral"); set_stage("typed-command-emit"); set_stage("typed-command-expose"); } } async fn dispatch_typed_command_inner() { let session = WorkerJobSession::new(job); }';
  if (toolJobFullOperationBounded(monolithicReducer)) throw new Error("[verify interactivity tool-jobs] self-test one-shot-generic-reducer-inside-worker was falsely accepted.");
  const boundedFirstStepReducer = monolithicReducer
    .replace("max_decoded_items; max_output_bytes;", "max_decoded_items; max_work_units_per_step; max_output_bytes; max_step_micros; ToolExecutionShape::BoundedFirstStep;")
    .replace("async fn dispatch_typed_command_inner()", "fn construct() { match proof { QualifiedToolProof::Bounded(_) => { TypedCommandFullOperationJob::<A>; } } } async fn dispatch_typed_command_inner()");
  if (!toolJobFullOperationBounded(boundedFirstStepReducer)) throw new Error("[verify interactivity tool-jobs] self-test exact bounded-first-step generic reducer was falsely rejected.");
  const noStageWatchdog = monolithicHugeOutput.replace("cx.is_cancelled(); cx.should_yield();", "").replace("serde_json::to_vec(&emit.effects); for child in emit.child_emits.iter() {}", "");
  if (toolJobFullOperationBounded(noStageWatchdog)) throw new Error("[verify interactivity tool-jobs] self-test full-operation-without-stage-watchdog was falsely accepted.");
  const staleExposure = noStageWatchdog.replace("validate_commit(base_revision, generation);", "").replace("fn step() { ", "fn step() { cx.is_cancelled(); cx.should_yield(); ");
  if (toolJobFullOperationBounded(staleExposure)) throw new Error("[verify interactivity tool-jobs] self-test full-operation-without-stale-result-validation was falsely accepted.");
  const retainedStorePublication = `
pub trait ArtifactStoreOneItemPreparationFactory {}
pub trait ArtifactStoreOneItemPreparation {}
pub struct ArtifactStoreOneItemPrepared;
pub enum ArtifactStoreOneItemPublicationPhase { Prepare, Commit, Outbound, Retiring, Complete }
pub struct ArtifactStoreOneItemPublication<P, Mutation> { generation: u64, maximum_items: usize, maximum_bytes: usize, marker: PhantomData<(P, Mutation)> }
impl<P, Mutation> ArtifactStoreOneItemPublication<P, Mutation> {
  fn begin_close(&mut self) {}
  fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) { let items = maximum_items.min(1); if items > 0 && maximum_bytes > 0 { self.maximum_items = items; } }
  fn terminal_is_empty(&self) -> bool { self.phase == ArtifactStoreOneItemPublicationPhase::Complete && self.marker.is_empty() && self.maximum_items == 0 && self.maximum_bytes == 0 && self.generation == 0 }
}
impl<P, Mutation> Drop for ArtifactStoreOneItemPublication<P, Mutation> { fn drop(&mut self) { assert!(self.terminal_is_empty()); } }
impl<P, Mutation> ArtifactStore<P, Mutation> {
  fn begin_apply_one(&mut self) -> ArtifactStoreOneItemPublication<P, Mutation> { ArtifactStoreOneItemPublication { generation, maximum_items, maximum_bytes, marker: PhantomData } }
  fn advance_apply_one(&mut self, publication: &mut ArtifactStoreOneItemPublication<P, Mutation>) { publication.generation += 1; }
  fn cancel_apply_one(&mut self, publication: &mut ArtifactStoreOneItemPublication<P, Mutation>) { publication.begin_close(); }
}`;
  const retainedPluginPublication = `
struct Host {
  artifact_one_item_factory: Option<ArtifactFactory>,
  config_one_item_factory: Option<ConfigFactory>,
  draft_one_item_factory: Option<DraftFactory>,
  unsupported_publication_contracts: Contracts,
}
struct MountedTypedCommandFullOperation<A> { pending_artifact_publication: Option<ArtifactStoreOneItemPublication<A::Snapshot, A::Mutation>> }
async fn publish_mounted_typed_operation_unit() {
  self.artifact_one_item_factory.as_deref();
  self.config_one_item_factory.as_deref();
  self.draft_one_item_factory.as_deref();
  mounted.pending_artifact_publication = Some(self.store.begin_apply_one());
  self.store.advance_apply_one(mounted.pending_artifact_publication.as_mut().unwrap());
}`;
  if (!toolJobStoreOneItemPublicationBounded(retainedStorePublication, retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test retained Store one-item publication was falsely rejected.");
  if (toolJobStoreOneItemPublicationBounded(retainedStorePublication.replace("pub trait ArtifactStoreOneItemPreparationFactory {}", ""), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication without a domain preparation factory was falsely accepted.");
  if (toolJobStoreOneItemPublicationBounded(retainedStorePublication.replace("publication.generation += 1;", "apply_command(publication);"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication that restores whole apply_command was falsely accepted.");
  if (toolJobStoreOneItemPublicationBounded(retainedStorePublication.replace("publication.generation += 1;", "cursor.try_reserve_exact(total); publication.generation += 1;"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication that hides whole-vector growth in a phase was falsely accepted.");
  if (toolJobStoreOneItemPublicationBounded(retainedStorePublication.replace("fn terminal_is_empty", "fn terminal_owner_remains"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication without terminal-empty witness was falsely accepted.");
  if (toolJobStoreOneItemPublicationBounded(retainedStorePublication.replace("impl<P, Mutation> Drop for ArtifactStoreOneItemPublication", "impl<P, Mutation> Closed for ArtifactStoreOneItemPublication"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication without terminal Drop enforcement was falsely accepted.");
  if (toolJobStoreOneItemPublicationBounded(retainedStorePublication.replace("self.maximum_items = items;", "owners.clear(); self.maximum_items = items;"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication with whole-vector close was falsely accepted.");
  if (toolJobStoreOneItemPublicationBounded(retainedStorePublication, retainedPluginPublication.replace("advance_apply_one", "apply_one")))
    throw new Error("[verify interactivity tool-jobs] self-test plugin publisher that restores monolithic apply_one was falsely accepted.");
  if (toolJobStoreOneItemPublicationBounded(retainedStorePublication, retainedPluginPublication.replace("self.artifact_one_item_factory.as_deref();", "A::build_artifact_store_one_item_preparation_factory();")))
    throw new Error("[verify interactivity tool-jobs] self-test per-publication durable factory reconstruction was falsely accepted.");
  const retainedEphemeralPublication = `
pub trait ArtifactEphemeralOneItemPreparationFactory {}
pub trait ArtifactEphemeralOneItemPreparation {}
pub struct ArtifactEphemeralOneItemPrepared;
pub struct ArtifactEphemeralOneItemPublication<P, Mutation>(PhantomData<(P, Mutation)>);
impl<P, Mutation> ArtifactEphemeralOneItemPublication<P, Mutation> {
  fn close_step(&mut self) { let released = 1; }
  fn terminal_is_empty(&self) -> bool { true }
}
impl<P, Mutation> Drop for ArtifactEphemeralOneItemPublication<P, Mutation> { fn drop(&mut self) { assert!(self.terminal_is_empty()); } }
impl<P, Mutation> PresenceStore<P, Mutation> {
  pub fn begin_publish_one(&self) {}
  pub fn advance_publish_one(&mut self) { let retirement = previous; publication = retirement; }
  pub fn cancel_publish_one(&mut self) {}
}
impl<P, Mutation> TransientStore<P, Mutation> {
  pub fn begin_publish_one(&self) {}
  pub fn advance_publish_one(&mut self) { let retirement = previous; publication = retirement; }
  pub fn cancel_publish_one(&mut self) {}
}`;
  const retainedEphemeralPublisher = `
struct Host {
  presence_one_item_factory: Option<PresenceFactory>,
  transient_one_item_factory: Option<TransientFactory>,
  presence_local_root_retirement_factory: Option<PresenceRetirementFactory>,
  transient_local_root_retirement_factory: Option<TransientRetirementFactory>,
  unsupported_publication_contracts: Contracts,
}
fn qualified_tool_proof(&self, verb: &str) { if let Some(lane) = self.unsupported_publication_contracts.get(verb) { reject(lane); } }
async fn publish_mounted_typed_operation_unit() {
  PendingArtifactStorePublication::Presence;
  PendingArtifactStorePublication::Transient;
  self.presence_one_item_factory.as_deref();
  self.transient_one_item_factory.as_deref();
  self.presence_local_root_retirement_factory.clone();
  self.transient_local_root_retirement_factory.clone();
  presence_store.begin_publish_one();
  presence_store.advance_publish_one();
  transient_store.begin_publish_one();
  transient_store.advance_publish_one();
}`;
  if (!toolJobEphemeralOneItemPublicationBounded(retainedEphemeralPublication, retainedEphemeralPublisher))
    throw new Error("[verify interactivity tool-jobs] self-test retained Presence/Transient one-item publication was falsely rejected.");
  if (toolJobEphemeralOneItemPublicationBounded(retainedEphemeralPublication.replace("let retirement = previous;", "drop(previous);"), retainedEphemeralPublisher))
    throw new Error("[verify interactivity tool-jobs] self-test immediate ephemeral displaced-root drop was falsely accepted.");
  if (toolJobEphemeralOneItemPublicationBounded(retainedEphemeralPublication, retainedEphemeralPublisher.replace("transient_store.advance_publish_one();", "transient_store.apply_one();")))
    throw new Error("[verify interactivity tool-jobs] self-test monolithic Transient publication was falsely accepted.");
  if (toolJobEphemeralOneItemPublicationBounded(retainedEphemeralPublication, retainedEphemeralPublisher.replace("self.presence_one_item_factory.as_deref();", "A::build_presence_store_one_item_preparation_factory();")))
    throw new Error("[verify interactivity tool-jobs] self-test per-publication Presence factory reconstruction was falsely accepted.");
  const freshPublicationTurn = `
async fn publish_mounted_typed_operation_unit() {
  let live_revision = self.store.content_revision_now();
  validate_commit(&mounted.operation, live_revision);
  if mounted.canonical_revision != live_revision { return Err(stale); }
  if let Some(pending) = mounted.pending_artifact_publication.as_mut() { advance(pending); }
  self.store.begin_apply_one();
}`;
  if (!toolJobPublicationFreshnessBeforeEveryTurn(freshPublicationTurn))
    throw new Error("[verify interactivity tool-jobs] self-test publication freshness before every resumed or new turn was falsely rejected.");
  const staleResumedPublicationTurn = freshPublicationTurn
    .replace("  validate_commit(&mounted.operation, live_revision);\n", "")
    .replace(
      "  if let Some(pending) = mounted.pending_artifact_publication.as_mut() { advance(pending); }",
      "  if let Some(pending) = mounted.pending_artifact_publication.as_mut() { advance(pending); }\n  validate_commit(&mounted.operation, live_revision);",
    );
  if (toolJobPublicationFreshnessBeforeEveryTurn(staleResumedPublicationTurn))
    throw new Error("[verify interactivity tool-jobs] self-test resumed publication advanced before freshness validation was falsely accepted.");
  if (toolJobPublicationFreshnessBeforeEveryTurn(freshPublicationTurn.replace("mounted.canonical_revision != live_revision", "false")))
    throw new Error("[verify interactivity tool-jobs] self-test publication freshness without canonical/live revision comparison was falsely accepted.");
  const helperFreshPublicationTurn = `
fn typed_operation_document_is_fresh(operation: &Operation, canonical_revision: Revision, live_revision: Revision, live_generation: u64) -> bool {
  canonical_revision == live_revision && matches!(validate_commit(operation, live_revision, Generation(live_generation)), CommitValidation::Accepted)
}
async fn publish_mounted_typed_operation_unit() {
  if !typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation) { return Err(stale); }
  if let Some(pending) = mounted.pending_artifact_publication.as_mut() { advance(pending); }
  self.store.begin_apply_one();
}`;
  if (!toolJobPublicationFreshnessBeforeEveryTurn(helperFreshPublicationTurn))
    throw new Error("[verify interactivity tool-jobs] self-test exact extracted publication freshness guard was falsely rejected.");
  if (toolJobPublicationFreshnessBeforeEveryTurn(helperFreshPublicationTurn.replace("validate_commit(operation", "accept_without_validation(operation")))
    throw new Error("[verify interactivity tool-jobs] self-test extracted publication freshness guard without commit validation was falsely accepted.");
  const hostOnlyPublication = `const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "setView", lanes: &[ArtifactToolPublicationLane::HostOnly] }];`;
  const hostOnlyContracts = toolJobPublicationContracts(hostOnlyPublication, hostOnlyPublication, new Map());
  if (hostOnlyContracts?.get("setView")?.join(",") !== "HostOnly" || !toolJobPublicationAuthorityReady("", hostOnlyContracts.get("setView")!))
    throw new Error("[verify interactivity tool-jobs] self-test exact host-only publication contract was falsely rejected.");
  const namedHostOnlyPublication = `const VIEW_PUBLICATIONS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "setView", lanes: &[ArtifactToolPublicationLane::HostOnly] }]; const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = VIEW_PUBLICATIONS;`;
  if (toolJobPublicationContracts(namedHostOnlyPublication, namedHostOnlyPublication.slice(namedHostOnlyPublication.indexOf("const PUBLICATION_CONTRACTS")), new Map())?.get("setView")?.join(",") !== "HostOnly")
    throw new Error("[verify interactivity tool-jobs] self-test named exact publication contract was falsely rejected.");
  const mixedHostPublication = hostOnlyPublication.replace("ArtifactToolPublicationLane::HostOnly]", "ArtifactToolPublicationLane::HostOnly, ArtifactToolPublicationLane::Config]");
  if (toolJobPublicationContracts(mixedHostPublication, mixedHostPublication, new Map()))
    throw new Error("[verify interactivity tool-jobs] self-test mixed host-only/Store publication contract was falsely accepted.");
  const configPublication = hostOnlyPublication.replace("setView", "setLocale").replace("HostOnly", "Config");
  const configContracts = toolJobPublicationContracts(configPublication, configPublication, new Map())!;
  if (toolJobPublicationAuthorityReady("", configContracts.get("setLocale")!))
    throw new Error("[verify interactivity tool-jobs] self-test Config publication without an app-owned preparation factory was falsely accepted.");
  const configOwner = "fn build_config_store_one_item_preparation_factory() -> Option<Factory> { Some(Factory::new()) }";
  if (!toolJobPublicationAuthorityReady(configOwner, configContracts.get("setLocale")!))
    throw new Error("[verify interactivity tool-jobs] self-test exact Config publication preparation authority was falsely rejected.");
  const presencePublication = hostOnlyPublication.replace("setView", "setPresence").replace("HostOnly", "Presence");
  const presenceLanes = toolJobPublicationContracts(presencePublication, presencePublication, new Map())!.get("setPresence")!;
  const presencePreparationOnly = "fn build_presence_store_one_item_preparation_factory() -> Option<Factory> { Some(Factory::new()) }";
  if (toolJobPublicationAuthorityReady(presencePreparationOnly, presenceLanes))
    throw new Error("[verify interactivity tool-jobs] self-test Presence publication without displaced-root retirement authority was falsely accepted.");
  const presenceComplete = `${presencePreparationOnly} fn build_presence_local_root_retirement_factory() -> Option<Factory> { Some(Factory::new()) }`;
  if (!toolJobPublicationAuthorityReady(presenceComplete, presenceLanes))
    throw new Error("[verify interactivity tool-jobs] self-test exact Presence preparation and root-retirement authority was falsely rejected.");
  const unguardedIncompleteRoute = "async fn dispatch_typed_command_inner() { refresh_cache().await; let session = WorkerJobSession::new(job); }";
  if (toolJobTypedRouteFailsClosedBeforePreparation(unguardedIncompleteRoute)) throw new Error("[verify interactivity tool-jobs] self-test incomplete-typed-route-without-preparation-guard was falsely accepted.");
  const terminalLoopTypedFoundation = "struct ActiveToolCommand<A>; struct VcsArtifactApp<A> { tool_operations: ArtifactFixedRegistry<ActiveToolCommand<A>> } impl<A: ArtifactApp> ActiveToolCommand<A> { fn drive_worker_step() { session.try_submit_step(pool, semio_framework_async::Lane::Interactive); pending.try_recv(); ActiveToolCommandStage::CommitReady; } } async fn dispatch_typed_command_inner() { require_complete_tool_operation_pipeline(&admission)?; let operation_id = allocate_operation_id(); if !self.tool_operations.can_insert(operation_id.0) {} refresh_cache().await; let session = WorkerJobSession::new(job); self.tool_operations.insert_admitted(operation_id, active); let outcome = loop { session.step(&pool).await; }; active.drive_worker_step(&pool)?; DslValue::String(operation_id.0.to_string()); }";
  if (toolJobTypedPersistentFoundation(terminalLoopTypedFoundation)) throw new Error("[verify interactivity tool-jobs] self-test typed-persistent-foundation-with-terminal-loop was falsely accepted.");
  const wholeMapDropCancellation = "impl<A: ArtifactApp> Drop for VcsArtifactApp<A> { fn drop() { resolve_ready(self.tool_cancellations.cancel_all()); operations.drain(); } }";
  if (toolJobDropCancellationBounded(wholeMapDropCancellation)) throw new Error("[verify interactivity tool-jobs] self-test whole-map-drop-cancellation was falsely accepted.");
  const collectionCancellation = "impl ToolCancellationHandle { fn begin() { live.iter().filter().collect::<Vec<_>>(); } fn cancel_document() { live.keys(); } fn cancel_scope_generation() {} }";
  if (toolJobCancellationScopesBounded(collectionCancellation)) throw new Error("[verify interactivity tool-jobs] self-test collection-wide-cancellation was falsely accepted.");
  const stringKeyedCancellation = "impl ToolCancellationHandle { fn begin(document: String) { let mut live: HashMap<String, Token> = HashMap::new(); live.insert(document, token); } fn cancel_document() {} fn cancel_scope_generation() {} }";
  if (toolJobCancellationScopesBounded(stringKeyedCancellation)) throw new Error("[verify interactivity tool-jobs] self-test string-keyed-resizable-cancellation was falsely accepted.");
  const blockingCancellation = "impl ToolCancellationHandle { fn begin() { let mut state = self.state.lock().unwrap(); state.token.cancel_now(); } fn cancel_document() {} fn cancel_scope_generation() {} }";
  if (toolJobCancellationScopesBounded(blockingCancellation)) throw new Error("[verify interactivity tool-jobs] self-test blocking-lock-held-cancellation was falsely accepted.");
  const implicitCloseDestruction = "pub struct ArtifactDocumentAuthority(pub u32); struct VcsArtifactApp { media_exports: ArtifactFixedRegistry<ActiveMediaExport>, segmented_downloads: ArtifactFixedRegistry<ArtifactDownloadOutput> } plugin_destroy_app(runtime, *numeric_instance);";
  if (toolJobHardBoundedCloseExact(implicitCloseDestruction, implicitCloseDestruction)) throw new Error("[verify interactivity tool-jobs] self-test implicit-close-field-destruction was falsely accepted.");
  const erasedCloseWithoutWitness = "trait PluginApp { fn close_step() -> PluginCloseStep; } trait ArtifactReservedJob { fn close_step() -> PluginCloseStep; } fn run_runtime_close_turn() { if close_step() == Complete { drop(app); } }";
  if (toolJobErasedCloseTerminalExact(erasedCloseWithoutWitness)) throw new Error("[verify interactivity tool-jobs] self-test erased-close-without-terminal-witness was falsely accepted.");
  const ordinaryFixedOptionSlots = "trait PluginApp { fn close_terminal_is_empty(&self) -> bool; } trait ArtifactReservedJob { fn terminal_is_empty(&self) -> bool; } struct RuntimeInstanceRegistry<T> { slots: Box<[Option<(u32, T)>]> } struct ArtifactFixedRegistry<T> { slots: Box<[Option<(u64, T)>]> }";
  if (toolJobErasedCloseTerminalExact(ordinaryFixedOptionSlots)) throw new Error("[verify interactivity tool-jobs] self-test fixed-capacity-option-slot-implicit-drop was falsely accepted.");
  const completeWithoutReservedWitness = "trait ArtifactReservedJob { fn terminal_is_empty(&self) -> bool; } fn close_step() { if step == Complete { drop(self.inner.take()); } }";
  if (toolJobErasedCloseTerminalExact(completeWithoutReservedWitness)) throw new Error("[verify interactivity tool-jobs] self-test reserved-complete-ignored-terminal-witness was falsely accepted.");
  const closeWithoutUniqueCell = "trait PluginApp { fn close_terminal_is_empty(&self) -> bool; } fn run_runtime_close_turn() { if app.close_terminal_is_empty() { drop(detached); } }";
  if (toolJobErasedCloseTerminalExact(closeWithoutUniqueCell)) throw new Error("[verify interactivity tool-jobs] self-test runtime-close-without-unique-cell-ownership was falsely accepted.");
  const unmeasuredTerminalHandoff = "const RUNTIME_CLOSE_INNER_GRANT_MS: u64 = 8; const RUNTIME_CLOSE_CALLBACK_WALL_US: u64 = 8_000; impl<PA: PluginApp> semio_framework_job::InteractiveJob for RuntimeCloseCleanupJob<PA> { fn step() { try_lock(); } } fn run_runtime_close_turn_inner() { drive_step(StepBudget::new(1, now.saturating_add(RUNTIME_CLOSE_INNER_GRANT_MS))); drop(instance); } fn run_runtime_close_turn() { run_runtime_close_turn_inner(state); } fn runtime_close_nonterminal_status() {}";
  if (toolJobRuntimeCloseCallbackBounded(unmeasuredTerminalHandoff)) throw new Error("[verify interactivity tool-jobs] self-test runtime-close-terminal-handoff-outside-watchdog was falsely accepted.");
  const contentionConsumesLivelock = "const RUNTIME_CLOSE_INNER_GRANT_MS: u64 = 2; const RUNTIME_CLOSE_CALLBACK_WALL_US: u64 = 8_000; impl<PA: PluginApp> semio_framework_job::InteractiveJob for RuntimeCloseCleanupJob<PA> { fn step() { if let Err(std::sync::TryLockError::WouldBlock) = try_lock() { self.contended = true; } Err(std::sync::TryLockError::Poisoned(_)); } } fn run_runtime_close_turn_inner() { StepBudget::new(1, now.saturating_add(RUNTIME_CLOSE_INNER_GRANT_MS)); } fn run_runtime_close_turn() { let started = std::time::Instant::now(); run_runtime_close_turn_inner(state); let elapsed_us = started.elapsed().as_micros(); if elapsed_us > RUNTIME_CLOSE_CALLBACK_WALL_US { state.status.store(RUNTIME_CLOSE_FAULT); } } fn runtime_close_nonterminal_status() { stalled_steps.fetch_add(1); if contended { return RUNTIME_CLOSE_READY; } } repeated_transient_close_lock_contention_never_consumes_structural_livelock_credit structural_zero_progress_exhausts_its_exact_close_credit";
  if (toolJobRuntimeCloseCallbackBounded(contentionConsumesLivelock)) throw new Error("[verify interactivity tool-jobs] self-test transient-close-contention-consumed-livelock-credit was falsely accepted.");
  const fakeCleanupQueue = `${implicitCloseDestruction} struct ArtifactCloseCleanupJob; impl semio_framework_job::InteractiveJob for ArtifactCloseCleanupJob {} const ARTIFACT_CLOSE_CLEANUP_ITEMS_PER_STEP: usize = 1; const ARTIFACT_CLOSE_CLEANUP_BYTES_PER_STEP: usize = 4096; fn handoff_close_cleanup(&mut self) { std::mem::replace(&mut self.media_exports); std::mem::replace(&mut self.segmented_downloads); }`;
  if (toolJobHardBoundedCloseExact(fakeCleanupQueue, fakeCleanupQueue)) throw new Error("[verify interactivity tool-jobs] self-test cleanup-enqueue-without-saturation-ownership was falsely accepted.");
  const replacingLiveOwner = "impl<T> ArtifactFixedRegistry<T> { fn insert(&mut self, id: u64, value: T) -> Result<Option<T>, T> { let previous = self.slots[index].replace((id, value)); Ok(previous) } fn can_insert(&self, id: u64) -> bool { true } fn insert_admitted(&mut self, id: u64, value: T) {} }";
  if (toolJobDetachedOutputOwnershipExact(replacingLiveOwner)) throw new Error("[verify interactivity tool-jobs] self-test occupied-live-owner-replacement was falsely accepted.");
  const detachedMediaDrop = "impl<T> ArtifactFixedRegistry<T> { fn insert(&mut self, id: u64, value: T) -> Result<(), T> { if self.entry(index).is_some() { return Err(value); } Ok(()) } fn can_insert(&self, id: u64) -> bool { true } fn insert_admitted(&mut self, id: u64, value: T) {} } async fn submit_owned_media_export() { let operation_id = semio_framework_job::allocate_operation_id(); if !self.media_exports.can_insert(operation_id.0) || !self.media_closures.can_insert(operation_id.0) || !self.snapshot_retirements.can_insert(operation_id.0) {} ArtifactSnapshotCloseLease::new(); self.snapshot_retirements.insert_admitted(operation_id.0, snapshot_retention); self.media_exports.insert_admitted(operation_id.0, active); } fn finish_media_poll() { drop(active); } async fn poll_owned_media_export() { if !self.media_closures.can_insert(handle.operation_id.0) {} let active = self.media_exports.remove(handle.operation_id.0); drop(active); } async fn cancel_owned_media_export() { let active = self.media_exports.remove(handle.operation_id.0); drop(active); } async fn dispatch_typed_command_inner() {}";
  if (toolJobDetachedOutputOwnershipExact(detachedMediaDrop)) throw new Error("[verify interactivity tool-jobs] self-test detached-media-stack-drop was falsely accepted.");
  const rejectedSegmentedDrop = detachedMediaDrop.replace("async fn dispatch_typed_command_inner() {}", "async fn dispatch_typed_command_inner() { let operation_id = semio_framework_job::allocate_operation_id(); ArtifactOutputChunks::new(maximum); if self.segmented_downloads.insert(operation_id.0, download).is_err() { return Err(fault); } }");
  if (toolJobDetachedOutputOwnershipExact(rejectedSegmentedDrop)) throw new Error("[verify interactivity tool-jobs] self-test rejected-segmented-download-stack-drop was falsely accepted.");
  const constructionEnvelopeWithoutLivePump = "trait PluginApp { fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>; } async fn submit_owned_media_export() { self.media_closures.insert_admitted(operation_id, active); A::build_media_export_job(request)?; } fn close_step() { self.media_closures.remove(operation_id); }";
  if (toolJobLiveConstructionCleanupExact(constructionEnvelopeWithoutLivePump, "")) throw new Error("[verify interactivity tool-jobs] self-test failed-media-construction-without-live-cleanup-pump was falsely accepted.");
  const inlineLiveCleanup = "trait PluginApp { fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>; } fn poll_kernel(runtime) { runtime.app.maintenance_step(1, 4096); plugin_step_live_cleanup(runtime); } struct RuntimeLiveCleanupJob<PA: PluginApp>;";
  if (toolJobLiveConstructionCleanupExact(inlineLiveCleanup, inlineLiveCleanup)) throw new Error("[verify interactivity tool-jobs] self-test inline-live-cleanup-outside-worker-watchdog was falsely accepted.");
  const detachBeforeCloseGeneration = "async fn plugin_destroy_app() { let cell = instances.take(instance_id); let generation = checked_runtime_close_generation(runtime.close_generation.get())?; } exhausted_close_generation_is_rejected_before_exact_owner_detachment";
  if (toolJobLiveConstructionCleanupExact(detachBeforeCloseGeneration, detachBeforeCloseGeneration)) throw new Error("[verify interactivity tool-jobs] self-test close-generation-admission-after-owner-detach was falsely accepted.");
  const blockedCleanupClaimsRelease = "fn maintenance_step() { match active.close_step() { PluginCloseStep::Blocked { .. } => PluginCloseStep::Pending { released_items: 1, released_bytes: 0 } } } permanently_blocked_live_cleanup_faults_without_claiming_released_ownership";
  if (toolJobLiveConstructionCleanupExact(blockedCleanupClaimsRelease, blockedCleanupClaimsRelease)) throw new Error("[verify interactivity tool-jobs] self-test blocked-live-cleanup-falsely-claimed-release was falsely accepted.");
  const weakSnapshotLease = "pub trait ArtifactSnapshotDisposer<T>: Send {} struct VcsArtifactApp { snapshot_retirements: ArtifactFixedRegistry<ArtifactSnapshotCloseRetention>, close_snapshot_cursor: usize } impl Lease { fn can_release(snapshot: &Arc<T>) { Arc::strong_count(snapshot) > 1; } } snapshot_a_survives_cache_b_and_only_the_bounded_retirement_owner_performs_final_drop";
  if (toolJobSnapshotRetirementBounded(weakSnapshotLease)) throw new Error("[verify interactivity tool-jobs] self-test weak-count-snapshot-retirement was falsely accepted.");
  const activeOwnedSnapshot = "pub trait ArtifactSnapshotDisposer<T>: Send {} struct VcsArtifactApp { snapshot_retirements: ArtifactFixedRegistry<ArtifactSnapshotCloseRetention>, close_snapshot_cursor: usize } struct ActiveMediaExport { snapshot_retention: ArtifactSnapshotCloseRetention } fn close_step() { drop(active); } snapshot_a_survives_cache_b_and_only_the_bounded_retirement_owner_performs_final_drop";
  if (toolJobSnapshotRetirementBounded(activeOwnedSnapshot)) throw new Error("[verify interactivity tool-jobs] self-test active-owned-last-snapshot-root was falsely accepted.");
  const snapshotCompleteWithoutEmptyWitness = "pub trait ArtifactSnapshotDisposer<T>: Send { fn close_step() -> PluginCloseStep { PluginCloseStep::Complete } } struct VcsArtifactApp { snapshot_retirements: ArtifactFixedRegistry<ArtifactSnapshotCloseRetention>, close_snapshot_cursor: usize } snapshot_a_survives_cache_b_and_only_the_bounded_retirement_owner_performs_final_drop";
  if (toolJobSnapshotRetirementBounded(snapshotCompleteWithoutEmptyWitness)) throw new Error("[verify interactivity tool-jobs] self-test snapshot-complete-without-terminal-empty-witness was falsely accepted.");
  const ownedDisposerWithoutTerminal = "pub trait ArtifactOwnedDisposer<T>: Send { fn close_step(&mut self, owner: &mut T, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>; fn terminal_is_empty(&self, owner: &T) -> bool; } fn drive_artifact_owned_disposer() { drop(disposer.take()); disposer_ref.terminal_is_empty(owner); } impl PluginApp for VcsArtifactApp { fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> { drive_artifact_owned_disposer(\"document-store\"); drive_artifact_owned_disposer(\"config-store\"); drive_artifact_owned_disposer(\"draft-store\"); drive_artifact_owned_disposer(\"presence-store\"); drive_artifact_owned_disposer(\"transient-store\"); FaultCode::new(\"interactive-job.close-app-retained-fields-missing\"); } }";
  if (toolJobVcsOwnedDisposalExplicit(ownedDisposerWithoutTerminal)) throw new Error("[verify interactivity tool-jobs] self-test owned-store-disposer-release-before-terminal-witness was falsely accepted.");
  const missingOwnedLane = ownedDisposerWithoutTerminal.replace('drop(disposer.take()); disposer_ref.terminal_is_empty(owner);', 'FaultCode::new("interactive-job.close-owned-disposer-missing"); FaultCode::new("interactive-job.close-owned-terminal-not-empty"); disposer_ref.terminal_is_empty(owner); drop(disposer.take());').replace('drive_artifact_owned_disposer("presence-store");', "");
  if (toolJobVcsOwnedDisposalExplicit(missingOwnedLane)) throw new Error("[verify interactivity tool-jobs] self-test missing-owned-store-disposal-lane was falsely accepted.");
  const resizableRuntimeInstances = "struct PluginRuntime<PA> { instances: LocalAsyncMutex<Vec<std::sync::Arc<RuntimeAppCell<PA>>>>, instance_actors: HashMap<u32, String> } async fn plugin_destroy_app() { list.iter().position(); list.remove(index); } plugin_destroy_app(runtime, *numeric_instance);";
  if (toolJobRuntimeRegistryFixedClose(resizableRuntimeInstances, resizableRuntimeInstances)) throw new Error("[verify interactivity tool-jobs] self-test resizable-scanning-runtime-instance-close was falsely accepted.");
  const scanningReactorClose = "static OPEN_INSTANCES: RefCell<Vec<(u32, String)>>; static TASK_RECORDS: RefCell<HashMap<TaskId, TaskRecord>>; static TASK_KEYS: RefCell<HashMap<(u32, String), TaskId>>; static INSTANCE_QUOTAS: RefCell<HashMap<u32, Quota>>; pub(crate) fn cancel_instance_tasks() { records.iter().filter().collect::<Vec<_>>(); } pub fn cancel_instance(&self) { instance_of.iter().filter(); } pub fn cancel(&self) { ready.retain(); free.contains(); } for waker in state.waiters.drain(..) {}";
  if (toolJobReactorCloseBounded(scanningReactorClose, scanningReactorClose, scanningReactorClose)) throw new Error("[verify interactivity tool-jobs] self-test scanning-reactor-task-request-close was falsely accepted.");
  const fixedOptionReactorClose = "struct ReactorFixedSlots<T> { values: Box<[Option<T>]> } struct Inner { slots: Box<[Option<SlotEntry>]>, outbound: VecDeque<(u32, Effect)> } fn cancel_instance_step() { drop(slots[index].take()); }";
  if (toolJobReactorCloseBounded(fixedOptionReactorClose, fixedOptionReactorClose, fixedOptionReactorClose)) throw new Error("[verify interactivity tool-jobs] self-test fixed-option-reactor-request-implicit-drop was falsely accepted.");
  const genericFutureReactor = "static REACTOR_EXECUTOR: executor::ColdFutureExecutor; pub trait ReactorTask { fn step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep; fn close_step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep; fn terminal_is_empty(&self) -> bool; }";
  if (toolJobReactorCloseBounded(genericFutureReactor, genericFutureReactor, genericFutureReactor)) throw new Error("[verify interactivity tool-jobs] self-test generic-future-production-reactor was falsely accepted.");
  const rejectedTaskDrop = "pub fn admit(&self, task: Box<dyn ReactorTask>) -> Result<TaskId, Box<dyn ReactorTask>> { Err(task) } rejected_reactor_task_is_bounded_disposed_without_drop blocked_reactor_task_does_not_starve_ready_peer reactor_executor_shutdown_drains_every_slot_before_terminal_drop stale_generation_cannot_commit";
  if (toolJobReactorCloseBounded(rejectedTaskDrop, rejectedTaskDrop, rejectedTaskDrop)) throw new Error("[verify interactivity tool-jobs] self-test rejected-reactor-task-drop-escape was falsely accepted.");
  const missingTerminalProof = "pub trait ReactorTask { fn step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep; fn close_step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep; } fn close_instance_step() { drop(task); }";
  if (toolJobReactorCloseBounded(missingTerminalProof, missingTerminalProof, missingTerminalProof)) throw new Error("[verify interactivity tool-jobs] self-test reactor-terminal-shell-without-empty-proof was falsely accepted.");
  const blockedStarvation = "pub fn run_until_deadline(&self) { match task.step() { ReactorTaskStep::Blocked { .. } => break, _ => {} } }";
  if (toolJobReactorCloseBounded(blockedStarvation, blockedStarvation, blockedStarvation)) throw new Error("[verify interactivity tool-jobs] self-test blocked-reactor-task-starvation was falsely accepted.");
  const implicitExecutorDrop = "struct ReactorExecutor { slots: Box<[Option<Box<dyn ReactorTask>>]> } impl Drop for ReactorExecutor { fn drop(&mut self) {} }";
  if (toolJobReactorCloseBounded(implicitExecutorDrop, implicitExecutorDrop, implicitExecutorDrop)) throw new Error("[verify interactivity tool-jobs] self-test implicit-reactor-executor-slot-drop was falsely accepted.");
  const productionJobFuture = "static JOBS_EXECUTOR: super::executor::ColdFutureExecutor = super::executor::ColdFutureExecutor::new(); async fn spawn_job() { let future = run(ctx, input, restored); JOBS_EXECUTOR.spawn(future); }";
  if (toolJobOpaqueFutureProductionFailClosed(productionJobFuture)) throw new Error("[verify interactivity tool-jobs] self-test production-opaque-job-future was falsely accepted.");
  const dynamicallyGrowingChunks = "impl ArtifactOutputChunks { fn push(&self, chunk: Vec<u8>) { state.chunks.push_back(chunk); } fn seal(&self) { sealed.store(true); } fn take_chunk(&self) { state.chunks.pop_front(); } }";
  if (toolJobSegmentedQueueHardBounded(dynamicallyGrowingChunks)) throw new Error("[verify interactivity tool-jobs] self-test dynamically-growing-segmented-chunks was falsely accepted.");
  const boxedTerminalWalk = "struct ArtifactOutputChunksState { chunks: Box<[Option<Vec<u8>>]> } impl ArtifactOutputChunks { fn push(&self, chunk: Vec<u8>) { checked_add(chunk.len()); if chunk.len() > ARTIFACT_OUTPUT_CHUNK_BYTES {} } fn seal(&self) {} fn take_chunk(&self) {} }";
  if (toolJobSegmentedQueueHardBounded(boxedTerminalWalk)) throw new Error("[verify interactivity tool-jobs] self-test segmented-terminal-capacity-drop-walk was falsely accepted.");
  const racySeal = "struct ArtifactFixedQueue<T>; impl ArtifactOutputChunks { fn push(&self, chunk: Vec<u8>) { self.inner.state.try_lock(); checked_add(chunk.len()); if chunk.len() > ARTIFACT_OUTPUT_CHUNK_BYTES {} state.chunks.push(chunk); } fn seal(&self) { self.inner.sealed.compare_exchange(false, true); self.inner.bytes.load(); } fn take_chunk(&self) { state.chunks.pop(); } }";
  if (toolJobSegmentedQueueHardBounded(racySeal)) throw new Error("[verify interactivity tool-jobs] self-test segmented-append-after-seal-race was falsely accepted.");
  const prematureSegmentRemoval = "async fn take_segmented_download_chunk(&mut self, operation_id: u64) { let chunk = output.chunks.take_chunk()?; if output.chunks.chunks_remaining() == 0 { self.segmented_downloads.remove(&operation_id); } } segmented_download_remains_addressable_until_terminal_none_is_observed";
  if (toolJobSegmentedTerminalDrainExact(prematureSegmentRemoval)) throw new Error("[verify interactivity tool-jobs] self-test premature-segmented-terminal-removal was falsely accepted.");
  const genericPluginReserved = 'puzzle5d_reserved_factory!(CopyFactory, "copy", "copy.v1"); puzzle5d_reserved_factory!(CutFactory, "cut", "cut.v1"); puzzle5d_reserved_factory!(PasteFactory, "paste", "paste.v1"); puzzle5d_reserved_factory!(ImportFactory, "import-media", "import.v1"); impl ArtifactEditor for Puzzle5dPlayApp { fn register_tool_job_factories() {} fn build_reserved_tool_job() { ArtifactReservedToolInput::Media; ArtifactReservedToolJob::new(GenericJob); } } impl InteractiveJob for GenericJob { fn step() { CheckpointReady(()); cx.is_cancelled(); } }';
  if (toolJobPuzzleReservedRoutesExact(genericPluginReserved, "")) throw new Error("[verify interactivity tool-jobs] self-test generic-plugin-reserved-job was falsely accepted.");
  const terminalMediaSerialization = "async fn poll_owned_media_export() { pending_step.try_recv(); serde_json::to_vec(&media); active.output_credit.validate_terminal(); validate_media_export_structure(&media); }";
  if (toolJobMediaExportBounded(terminalMediaSerialization)) throw new Error("[verify interactivity tool-jobs] self-test terminal-media-serialization was falsely accepted.");
  const underCredit = "async fn poll_owned_media_export() { pending_step.try_recv(); active.output_credit.validate_terminal(); } struct RuntimeAppCell<PA: PluginApp>; dropping_a_pending_owner_restores_the_instance_collection_and_wakes_the_waiter; a_pending_instance_does_not_block_an_unrelated_instance_cell; submitted_first_step_can_be_polled_pending_without_panicking_or_double_submitting;";
  if (toolJobMediaExportBounded(underCredit)) throw new Error("[verify interactivity tool-jobs] self-test unsealed-media-output-credit was falsely accepted.");
  const terminalBatchFlatten = "async fn poll_owned_media_export() { pending_step.try_recv(); active.output_credit.validate_terminal(); validate_media_export_structure(&result, &active.output_chunks); result.into_batch_media(); }";
  if (toolJobMediaExportBounded(terminalBatchFlatten)) throw new Error("[verify interactivity tool-jobs] self-test terminal-batch-flatten was falsely accepted.");
  const copyableChunks = "impl ArtifactOutputChunks { fn push(chunk: Vec<u8>) { chunks.push(chunk); } fn take_chunk() { chunks.remove(0); } } async fn poll_owned_media_export() { pending_step.try_recv(); active.output_credit.validate_terminal(); validate_media_export_structure(&result, &active.output_chunks); }";
  if (toolJobMediaExportBounded(copyableChunks)) throw new Error("[verify interactivity tool-jobs] self-test copyable-unbounded-output-chunks was falsely accepted.");
  const foreignOutput = "impl ArtifactOutputChunks { fn push(chunk: Vec<u8>) { checked_add(chunk.len()); if chunk.len() > ARTIFACT_OUTPUT_CHUNK_BYTES {} state.chunks.push_back(chunk); } fn take_chunk() { state.chunks.pop_front(); } fn same_operation() { std::sync::Arc::ptr_eq; } } async fn poll_owned_media_export() { pending_step.try_recv(); active.output_credit.validate_terminal(); validate_media_export_structure(&result, &active.output_chunks); }";
  if (toolJobMediaExportBounded(foreignOutput)) throw new Error("[verify interactivity tool-jobs] self-test foreign-segmented-output-authority was falsely accepted.");
  const clonedOperationRoots = "struct PresenceStore<P> { local: P } struct TransientStore<P> { current: P } impl<P: Clone> PresenceStore<P> { pub fn local_root(&self) -> Arc<P> { Arc::new(self.local.clone()) } } impl<P: Clone> TransientStore<P> { pub fn current_root(&self) -> Arc<P> { Arc::new(self.current.clone()) } } pub fn snapshot_root(&self) -> Arc<P> { Arc::new(self.current.as_ref().clone()) }";
  if (toolJobImmutableOperationRootsExact(clonedOperationRoots)) throw new Error("[verify interactivity tool-jobs] self-test operation-root-capture-cloned-the-whole-store was falsely accepted.");
  const hashMapChildRoot = "pub struct ChildContentView { root: Option<std::sync::Arc<HashMap<(String, String), Child>>> } struct VcsArtifactApp { child_content_root: ChildContentView } async fn dispatch_typed_command_inner() { let children = self.child_content_root.clone(); }";
  if (toolJobChildContentRootExact(hashMapChildRoot, hashMapChildRoot)) throw new Error("[verify interactivity tool-jobs] self-test resizable-string-key-child-root was falsely accepted.");
  const defaultSnapshotRetirement = "pub trait ErasedSnapshotRetirement: Send { fn terminal_is_empty(&self) -> bool { true } } pub trait SpaceMember { fn retire_snapshot_read_erased(&mut self, snapshot: ErasedSnapshotRead) -> Result<Box<dyn ErasedSnapshotRetirement>, SnapshotRetirementRejected> { Ok(Box::new(DefaultRetirement(snapshot))) } } struct VcsArtifactApp { child_content_retirements: ArtifactFixedRegistry<ChildContentView> }";
  if (toolJobChildContentRootExact(defaultSnapshotRetirement, defaultSnapshotRetirement)) throw new Error("[verify interactivity tool-jobs] self-test default-erased-snapshot-retirement-without-owned-terminal-witness was falsely accepted.");
  const cloneableSnapshotCapability = "#[derive(Clone)]\npub struct SnapshotRead<T> { owner: Arc<T> } #[derive(Clone)]\npub struct ErasedSnapshotRead { owner: Arc<dyn Any> } pub trait ErasedSnapshotRetirement: Send { fn terminal_is_empty(&self) -> bool; } pub trait SnapshotRetirementFactory<P>: Send + Sync {} entry.snapshot.clone()";
  if (toolJobChildContentRootExact(cloneableSnapshotCapability, cloneableSnapshotCapability)) throw new Error("[verify interactivity tool-jobs] self-test cloneable-snapshot-read-last-owner-drop-escape was falsely accepted.");
  const hashMapChildOwners = "pub struct ChildContentView { root: Option<std::sync::Arc<ChildContentRoot>> } struct VcsArtifactApp<M> { child_content_root: std::mem::ManuallyDrop<ChildContentView>, child_content_retirements: ArtifactFixedRegistry<ChildContentRetirement>, children: HashMap<(String,String),(ArtifactDialect,M)> } async fn dispatch_typed_command_inner() { let children = ChildContentView::clone(&self.child_content_root); }";
  if (toolJobChildContentRootExact(hashMapChildOwners, hashMapChildOwners)) throw new Error("[verify interactivity tool-jobs] self-test immutable-child-root-with-resizable-deep-member-map was falsely accepted.");
  const childRegistryWithoutGeneration = "struct ChildMemberRegistry<M> { slots: Box<[MaybeUninit<M>]> } fn admit(&self) -> usize {} fn cancel_admission(&mut self, index: usize) {} impl<M> Drop for ChildMemberRegistry<M> { fn drop(&mut self) {} }";
  if (toolJobChildContentRootExact(childRegistryWithoutGeneration, childRegistryWithoutGeneration)) throw new Error("[verify interactivity tool-jobs] self-test fixed-child-registry-without-generation-or-terminal-drop was falsely accepted.");
  const permissiveMemberClose = "pub trait SpaceMember { fn close_owned_step(&mut self) -> SnapshotRetirementStep { SnapshotRetirementStep::Complete } fn close_owned_terminal_is_empty(&self) -> bool { true } }";
  if (toolJobChildContentRootExact(permissiveMemberClose, permissiveMemberClose)) throw new Error("[verify interactivity tool-jobs] self-test default-blanket-child-member-close-proof was falsely accepted.");
  const noOpOwnerRegistryDrop = "struct ArtifactFixedRegistry<T>; impl<T> Drop for ArtifactFixedRegistry<T> { fn drop(&mut self) {} } struct ChildMemberRegistry<M>; impl<M> Drop for ChildMemberRegistry<M> { fn drop(&mut self) {} }";
  if (toolJobChildContentRootExact(noOpOwnerRegistryDrop, noOpOwnerRegistryDrop)) throw new Error("[verify interactivity tool-jobs] self-test no-op-fixed-owner-registry-drop was falsely accepted.");
  const optionalMemberOwners = "pub struct MemberStoreOwners<P, Mutation>; pub trait MemberStoreOwner<Mutation> { fn member_store_owners() -> MemberStoreOwners<Self, Mutation> { blanket() } } pub trait ArtifactStoreOwnedDisposer<P, Mutation>: Send { fn terminal_is_empty(&self, store: &ArtifactStore<P, Mutation>) -> bool; }";
  if (toolJobMemberStoreOwnerExact(optionalMemberOwners, optionalMemberOwners)) throw new Error("[verify interactivity tool-jobs] self-test optional-default-member-store-owner was falsely accepted.");
  const snapshotOnlyMemberOwners = "pub struct MemberStoreOwners<P, Mutation> { snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>> } pub trait MemberStoreOwner<Mutation> { fn member_store_owners() -> MemberStoreOwners<Self, Mutation>; } fn install_member_store_owners_exact() { self.snapshot_retirement_factory = Some(owners.snapshot_retirement); } semio_subset_table!(member_owners);";
  if (toolJobMemberStoreOwnerExact(snapshotOnlyMemberOwners, snapshotOnlyMemberOwners)) throw new Error("[verify interactivity tool-jobs] self-test member-store-owner-without-whole-store-disposer was falsely accepted.");
  const wrapperOnlyMemberOwners = "pub struct MemberStoreOwners<P, Mutation> { snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>, store_disposer: Box<dyn ArtifactStoreOwnedDisposer<P, Mutation>> } pub trait MemberStoreOwner<Mutation> { fn member_store_owners() -> MemberStoreOwners<Self, Mutation>; } fn install_semio_snapshot_retirement() {}";
  if (toolJobMemberStoreOwnerExact(wrapperOnlyMemberOwners, wrapperOnlyMemberOwners)) throw new Error("[verify interactivity tool-jobs] self-test bypassable-wrapper-only-member-owner was falsely accepted.");
  const monolithicHistoryOwnerDrop = "pub trait ArtifactOwnedValueRetirementFactory<T>: Send + Sync {} pub struct ArtifactStoreCloseView<'a, P, Mutation>; fn close_step() { drop(store.envelope.vcs.edits); } SemioStoreClosePhase::HistoryMutations SemioStoreClosePhase::HistoryEdits";
  if (toolJobMemberStoreOwnerExact(monolithicHistoryOwnerDrop, monolithicHistoryOwnerDrop)) throw new Error("[verify interactivity tool-jobs] self-test member-store-whole-history-drop-without-retained-edit-cursor was falsely accepted.");
  const resizableStructuralStore = "struct ArtifactStoreConflictRetirement; fn close_take_conflict_retirement(&mut self) {} fn close_take_tail_snapshot_retirement(&mut self) {} fn close_take_current_snapshot_retirement(&mut self) {} fn close_take_final_envelope_retirement(&mut self) {} edit_messages: BTreeMap<String, Vec<MutationMessage>> *self.current = next; *self.envelope = next; SemioStoreClosePhase::StructuralOwners => Err(\"semio member store lacks its required fixed causal-index disassembly cursor\".into()) pub fn terminal_is_empty(&self) -> bool { true } envelopes: std::collections::HashMap<String, MutationEnvelope> applied: std::collections::HashSet<String>";
  if (toolJobArtifactStoreStructuralOwnersExact(resizableStructuralStore, resizableStructuralStore, resizableStructuralStore, resizableStructuralStore)) throw new Error("[verify interactivity tool-jobs] self-test resizable-artifact-store-structural-owner-and-direct-replacement was falsely accepted.");
  const shallowDirectFreeStructuralStore = "struct ArtifactStoreConflictRetirement; struct ArtifactStoreBackboneRetirement; fn close_take_conflict_retirement(&mut self) {} fn close_take_tail_snapshot_retirement(&mut self) {} fn close_take_current_snapshot_retirement(&mut self) {} fn close_take_final_envelope_retirement(&mut self) {} fn close_take_backbone_retirement(&mut self) {} fn close_take_causal_owner_retirement(&mut self) {} artifact store backbone retirement reached Drop before every exact URI, queue, message, channel, and byte owner was terminal-empty pub fn terminal_is_empty(&self) -> bool { true } pub const MUTATION_DAG_CAPACITY: usize = 8_192; pub fn take_one_close_owner(&mut self) -> Option<MutationDagCloseOwner> {} MutationDagInsertRejected { error: MutationDagError::Capacity, envelope } SemioStoreClosePhase::CausalIndex";
  if (toolJobArtifactStoreStructuralOwnersExact(shallowDirectFreeStructuralStore, shallowDirectFreeStructuralStore, shallowDirectFreeStructuralStore, shallowDirectFreeStructuralStore)) throw new Error("[verify interactivity tool-jobs] self-test direct-free-artifact-store-without-retained-displacement-authority was falsely accepted.");
  const partialAtomicStructuralStore = "struct ArtifactStoreDocumentRootCommitAuthority<P, Mutation>; fn prepare_document_root_commit(&mut self) {} fn commit_document_roots_retained() {} struct ArtifactEditMessageIndex; self.envelope.edit_messages.remove(index); self.edit_messages.rebuild(&self.envelope.edit_messages); ManuallyDrop<Vec<MutationEnvelope>> semio mutation owner has no generated field-by-field retirement cursor";
  if (toolJobArtifactStoreStructuralOwnersExact(partialAtomicStructuralStore, partialAtomicStructuralStore, partialAtomicStructuralStore, partialAtomicStructuralStore)) throw new Error("[verify interactivity tool-jobs] self-test partial-root-transaction-with-shifting-ledger-and-opaque-mutation-drop was falsely accepted.");
  const resizableArtifactVcsHistory = `${partialAtomicStructuralStore} #[derive(Clone, Deserialize)] #[serde(rename_all = "camelCase")] pub struct ArtifactVcs<P, Mutation> { pub edits: Vec<Edit<Mutation>>, pub changes: Vec<Change>, pub checkpoints: Vec<Checkpoint>, pub alternatives: Vec<Alternative> }`;
  if (toolJobArtifactStoreStructuralOwnersExact(resizableArtifactVcsHistory, resizableArtifactVcsHistory, resizableArtifactVcsHistory, resizableArtifactVcsHistory)) throw new Error("[verify interactivity tool-jobs] self-test resizable-cloneable-artifact-VCS-history-owner was falsely accepted.");
  const unreservedFixedArtifactVcsHistory = "pub struct ArtifactHistoryLedger<T>; pub fn try_push(&mut self, value: T) -> Result<ArtifactHistoryKey, T>; pub struct OwnedSchemaBoundedArrayAuthority<T> { values: Option<ArtifactHistoryLedger<T>> } values.try_push(value);";
  if (toolJobHistoryLedgerAdmissionExact(unreservedFixedArtifactVcsHistory, unreservedFixedArtifactVcsHistory)) throw new Error("[verify interactivity tool-jobs] self-test fixed-history-ledger-without-preconstruction-reservation-or-exact-handback was falsely accepted.");
  const historyReservationAcrossAwait = "pub struct ArtifactHistoryReservation; pub fn reserve_one(&mut self) -> Result<ArtifactHistoryReservation, ()>; pub fn cancel_reservation(&mut self, reservation: ArtifactHistoryReservation); pub fn insert_reserved(&mut self, reservation: ArtifactHistoryReservation, value: T); reservation.authority != self.authority() fixed_history_reservation_returns_exact_rejected_owner_and_blocks_aba pub fn content_addressed_checkpoint_id_with_pending_change() pending_change_checkpoint_hash_is_byte_identical_before_history_reservation ArtifactCommand::CommitCheckpoint { message, authors } => { let change_reservation = self.reserve_change_history_slot()?; let id = content_addressed_checkpoint_id().await; } ArtifactCommand::CreateAlternative { name } => {} reservation: Option<ArtifactHistoryReservation> let reservation = match values.reserve_one() values.insert_reserved(reservation, value) values.cancel_reservation(reservation) fn reserve_owner_slots(&mut self, count: usize) fn push_owner_reserved(&mut self fn reserve_edit_history_slot(&mut self) fn insert_reserved_edit_history(&mut self) displaced_owner_reservations_preserve_capacity_generation_and_interrupted_close rejected_history_mutation_and_metadata_owners_close_under_one_item_grants";
  if (toolJobHistoryLedgerAdmissionExact(historyReservationAcrossAwait, historyReservationAcrossAwait)) throw new Error("[verify interactivity tool-jobs] self-test checkpoint-history-reservation-survived-an-await was falsely accepted.");
  const unretainedResolutionCandidate = "async fn resolution_candidate(&self) -> Self {} let mut candidate = self.resolution_candidate().await; candidate.ingest_remote(envelope).await?; adopt_resolution_candidate(candidate).await?;";
  if (toolJobArtifactResolutionCandidateExact(unretainedResolutionCandidate)) throw new Error("[verify interactivity tool-jobs] self-test rejected-resolution-candidate-without-exact-retained-close-authority was falsely accepted.");
  const vecRejectedEditMessageLedger = "struct ArtifactEditMessageLedger { buckets: Box<[u16]>, generations: Box<[u64]> } struct ArtifactEditMessageLedgerRejected { entries: Vec<EditMessages> } const ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES: usize = 4_096; const ARTIFACT_EDIT_MESSAGE_LEDGER_BYTES: usize = 8192 * 4096; fn admit() { self.generations[slot].checked_add(1); artifact edit-message ledger contains a duplicate identity } impl ErasedSnapshotRetirement for ArtifactEditMessageLedgerRejected {} rejected artifact edit-message authority reached Drop before every exact payload owner was cursor-retired artifact edit-message fixed ledger reached Drop before every exact payload owner was cursor-retired";
  if (toolJobArtifactEditMessageLedgerExact(vecRejectedEditMessageLedger)) throw new Error("[verify interactivity tool-jobs] self-test fixed-ledger-with-ordinary-drop-rejected-batch was falsely accepted.");
  const wrappingGenerationEditMessageLedger = "struct ArtifactEditMessageLedger { buckets: Box<[u16]>, generations: Box<[u64]> } struct ArtifactEditMessageLedgerRejected { entries: std::mem::ManuallyDrop<Vec<crate::os_spr::EditMessages>> } const ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES: usize = 4_096; const ARTIFACT_EDIT_MESSAGE_LEDGER_BYTES: usize = 8192 * 4096; fn admit() { self.generations[slot].wrapping_add(1); artifact edit-message ledger contains a duplicate identity } impl ErasedSnapshotRetirement for ArtifactEditMessageLedgerRejected {} rejected artifact edit-message authority reached Drop before every exact payload owner was cursor-retired artifact edit-message fixed ledger reached Drop before every exact payload owner was cursor-retired";
  if (toolJobArtifactEditMessageLedgerExact(wrappingGenerationEditMessageLedger)) throw new Error("[verify interactivity tool-jobs] self-test fixed-ledger-with-wrapping-aba-generation was falsely accepted.");
  const serdeEnvelopeCodec = "#[derive(Serialize, Deserialize)] #[serde(rename_all = \"camelCase\")] pub struct ArtifactEnvelope<P, Mutation>; pub struct ArtifactEnvelopeDecodeAuthority<P, Mutation>; struct ArtifactEnvelopeDecodePage; struct ArtifactEnvelopeDecodeRejected; impl<P, Mutation> semio_framework_job::InteractiveJob for ArtifactEnvelopeDecodeAuthority<P, Mutation> {} impl ErasedSnapshotRetirement for ArtifactEnvelopeDecodeRejected {} const ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES: usize = 4_096; const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES: usize = 4096; artifact envelope decode authority reached Drop before terminal publication or retained close artifact envelope decode rejection reached Drop before every exact page owner was cursor-retired";
  if (toolJobArtifactEnvelopeOwnedCodecExact(serdeEnvelopeCodec, new Map())) throw new Error("[verify interactivity tool-jobs] self-test owned-envelope-codec-retained-public-deserialize was falsely accepted.");
  const envelopeCodecMarkers = "pub struct ArtifactEnvelope<P, Mutation>; pub struct ArtifactEnvelopeDecodeAuthority<P, Mutation>; struct ArtifactEnvelopeDecodePage; struct ArtifactEnvelopeDecodeRejected; impl<P, Mutation> semio_framework_job::InteractiveJob for ArtifactEnvelopeDecodeAuthority<P, Mutation> {} impl ErasedSnapshotRetirement for ArtifactEnvelopeDecodeRejected {} const ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES: usize = 4_096; const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES: usize = 4096; artifact envelope decode authority reached Drop before terminal publication or retained close artifact envelope decode rejection reached Drop before every exact page owner was cursor-retired";
  const directEnvelopeSerdeCaller = new Map([["plugin.rs", "let envelope: DemoEnvelope = serde_json::from_str(&json)?;"]]);
  if (toolJobArtifactEnvelopeOwnedCodecExact(envelopeCodecMarkers, directEnvelopeSerdeCaller)) throw new Error("[verify interactivity tool-jobs] self-test owned-envelope-codec-with-direct-production-serde-caller was falsely accepted.");
  const placeholderEnvelopeCaller = new Map([["plugin.rs", "let envelope: DemoEnvelope = store::reject_whole_buffer_artifact_envelope_ingress(&json)?;"]]);
  if (toolJobArtifactEnvelopeOwnedCodecExact(envelopeCodecMarkers, placeholderEnvelopeCaller)) throw new Error("[verify interactivity tool-jobs] self-test owned-envelope-codec-with-fail-closed-placeholder-caller was falsely accepted.");
  const ordinaryDropEnvelopeShell = `${envelopeCodecMarkers} pub struct ArtifactEnvelopeOwners<P, Mutation>; pub struct ArtifactEnvelope<P, Mutation> { owners: ArtifactEnvelopeOwners<P, Mutation> } pub trait ArtifactEnvelopeCompletedRecordTarget<P, Mutation>; fn try_publish_to(&mut self, target: &mut dyn ArtifactEnvelopeCompletedRecordTarget<P, Mutation>) -> bool {} pub fn try_request_close(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) {}`;
  if (toolJobArtifactEnvelopeOwnedCodecExact(ordinaryDropEnvelopeShell, new Map())) throw new Error("[verify interactivity tool-jobs] self-test owned-envelope-codec-with-ordinary-deep-terminal-shell was falsely accepted.");
  const wholeVectorSchemaPages = `${envelopeCodecMarkers} pub struct OwnedSchemaDecodePages { pages: Vec<Vec<u8>> } pub struct OwnedSchemaTokenCursor; pub struct OwnedSchemaRecordSpec; pub fn admit_page(&mut self, page: OwnedSchemaDecodePage) -> Result<(), (OwnedSchemaDecodeAdmissionFault, OwnedSchemaDecodePage)> {} pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaTokenStep {} pub fn close_step(&mut self, maximum_pages: usize) -> SnapshotRetirementStep {} schema-json.unknown-field schema-json.duplicate-field schema-json.invalid-utf8`;
  if (toolJobArtifactEnvelopeOwnedCodecExact(wholeVectorSchemaPages, new Map())) throw new Error("[verify interactivity tool-jobs] self-test owned-envelope-codec-with-whole-vector-schema-pages was falsely accepted.");
  const wholeBufferEnvelopeConstructor = `${envelopeCodecMarkers} pub struct OwnedSchemaDecodePages { slots: Box<[std::mem::MaybeUninit<OwnedSchemaDecodePage>]> } pub struct OwnedSchemaTokenCursor; pub struct OwnedSchemaRecordSpec; pub fn admit_page(&mut self, page: OwnedSchemaDecodePage) -> Result<(), (OwnedSchemaDecodeAdmissionFault, OwnedSchemaDecodePage)> {} pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaTokenStep {} pub fn close_step(&mut self, maximum_pages: usize) -> SnapshotRetirementStep {} schema-json.unknown-field schema-json.duplicate-field schema-json.invalid-utf8 ArtifactEnvelopeDecodeAuthority::new(bytes: Vec<u8>)`;
  if (toolJobArtifactEnvelopeOwnedCodecExact(wholeBufferEnvelopeConstructor, new Map())) throw new Error("[verify interactivity tool-jobs] self-test owned-envelope-codec-with-post-lift-whole-buffer-constructor was falsely accepted.");
  const unbudgetedSchemaDecoder = `${envelopeCodecMarkers} pub struct OwnedSchemaDecodePages { slots: Box<[std::mem::MaybeUninit<OwnedSchemaDecodePage>]> } pub struct OwnedSchemaTokenCursor; pub struct OwnedSchemaRecordSpec; pub fn admit_page(&mut self, page: OwnedSchemaDecodePage) -> Result<(), (OwnedSchemaDecodeAdmissionFault, OwnedSchemaDecodePage)> {} pub fn step(&mut self) -> OwnedSchemaTokenStep {} pub fn close_step(&mut self, maximum_pages: usize) -> SnapshotRetirementStep {} schema-json.unknown-field schema-json.duplicate-field schema-json.invalid-utf8`;
  if (toolJobArtifactEnvelopeOwnedCodecExact(unbudgetedSchemaDecoder, new Map())) throw new Error("[verify interactivity tool-jobs] self-test owned-envelope-codec-with-unbudgeted-tokenizer was falsely accepted.");
  const wholeStringFieldDecode = `${envelopeCodecMarkers} pub struct OwnedSchemaDecodePages { slots: Box<[std::mem::MaybeUninit<OwnedSchemaDecodePage>]> } pub struct OwnedSchemaTokenCursor; pub struct OwnedSchemaRecordSpec; pub struct OwnedSchemaStringAuthority<const MAXIMUM_BYTES: usize>; pub fn admit_page(&mut self, page: OwnedSchemaDecodePage) -> Result<(), (OwnedSchemaDecodeAdmissionFault, OwnedSchemaDecodePage)> {} pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaTokenStep {} pub fn close_step(&mut self, maximum_pages: usize) -> SnapshotRetirementStep {} fn accept_field_token() { serde_json::from_slice(token_bytes); } fn finish_record(&mut self, cx: &mut semio_framework_job::StepContext<'_>) {} fn terminal_is_empty(&self) -> bool; schema-json.stale-string-authority schema-json.string-byte-capacity schema-json.unknown-field schema-json.duplicate-field schema-json.invalid-utf8`;
  if (toolJobArtifactEnvelopeOwnedCodecExact(wholeStringFieldDecode, new Map())) throw new Error("[verify interactivity tool-jobs] self-test owned-envelope-codec-with-whole-string-field-decode was falsely accepted.");
  const implicitFieldOwner = `${envelopeCodecMarkers} pub struct OwnedSchemaDecodePages { slots: Box<[std::mem::MaybeUninit<OwnedSchemaDecodePage>]> } pub struct OwnedSchemaTokenCursor; pub struct OwnedSchemaRecordSpec; pub struct OwnedSchemaStringAuthority<const MAXIMUM_BYTES: usize>; pub fn admit_page(&mut self, page: OwnedSchemaDecodePage) -> Result<(), (OwnedSchemaDecodeAdmissionFault, OwnedSchemaDecodePage)> {} pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaTokenStep {} pub fn close_step(&mut self, maximum_pages: usize) -> SnapshotRetirementStep {} fn accept_field_token() {} fn finish_record(&mut self, cx: &mut semio_framework_job::StepContext<'_>) {} schema-json.stale-string-authority schema-json.string-byte-capacity schema-json.unknown-field schema-json.duplicate-field schema-json.invalid-utf8`;
  if (toolJobArtifactEnvelopeOwnedCodecExact(implicitFieldOwner, new Map())) throw new Error("[verify interactivity tool-jobs] self-test owned-envelope-codec-without-required-terminal-field-owner was falsely accepted.");
  const rawPresentationEnvelopeCaller = "pub struct PresentationEnvelopeOwnedFieldCatalog; impl store::ArtifactEnvelopeOwnedFieldCatalog<PresentationSnapshot, PresentationMutation> for PresentationEnvelopeOwnedFieldCatalog {} pub fn begin_materialize_presentation_projection() -> (PresentationEnvelopeMaterializeJob, PresentationProjectionCompletion); materialize_presentation_projection_json";
  if (toolJobPresentationEnvelopeCallerRetainedExact(rawPresentationEnvelopeCaller, "materializePresentationProjectionJson", "")) throw new Error("[verify interactivity tool-jobs] self-test Presentation-envelope-caller-with-raw-job-and-whole-string-Wasm-export was falsely accepted.");
  const retainedWriterInitializer = [
    "struct WriterStoreInitializationAuthority",
    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<WriterSnapshot, WriterMutation> for WriterStoreInitializationAuthority",
    "WriterStoreInitializationPhase::ValidateEditPair",
    "WriterStoreInitializationPhase::SeedHistory",
    "WriterStoreInitializationPhase::BuildCandidate",
    "ArtifactStoreInitializationRuntime::new",
    "ArtifactStore::from_initialized_runtime_with_owners",
    "self.generation.0.checked_add(1)",
    "Writer store initialization authority reached Drop before exact candidate handoff or retained rejection close",
    "writer_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally",
    "writer_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty",
  ].join("\n");
  const retainedWriterEditor = "fn build_document_store_initialization_job( writer_document_store_initialization_job(envelope, operation, generation) writer_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed writer_live_envelope_cancel_closes_retained_pages_without_publication";
  const retainedWriterWasm = [
    "pub struct WriterEnvelopeLoadHandle",
    "pub fn begin_envelope_load(",
    "begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)",
    "source: &js_sys::Uint8Array",
    "pub fn admit_envelope_page(",
    "admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)",
    "pub fn seal_envelope_load(",
    "seal_artifact_envelope_ingress(handle.runtime_handle())",
    "pub fn poll_envelope_load(",
    "app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)",
    "advance_artifact_envelope_load(handle.runtime_handle())",
    "acknowledge_artifact_store_replacement(handle.runtime_handle())",
    "pub fn cancel_envelope_load(",
    "pub fn close_step(&self)",
  ].join("\n");
  const retainedWriterPlugin = [
    "envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>",
    "pub fn begin_artifact_envelope_ingress(",
    "pub fn preflight_artifact_envelope_ingress_page(",
    "pub fn construct_and_admit_artifact_envelope_ingress_page<Build>(",
    "pub fn seal_artifact_envelope_ingress(",
    "pub fn advance_artifact_envelope_load(",
    "fn drive_envelope_ingress(",
    "saturated decoder keeps the sealed ingress owner in its original fixed slot for retry",
    "session: std::mem::ManuallyDrop<Option<semio_framework_job::MountedWorkerJobSession<ArtifactStoreInitializationJob<P, Mutation>>>>",
    "session_rejected: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ArtifactStoreInitializationJob<P, Mutation>>>>",
    "MountedWorkerJobSession::try_new(job, params)",
    "self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))",
    "artifact store replacement reached Drop before initializer/candidate/displaced-store ownership was terminal empty",
    "artifact store initialization job reached Drop before exact candidate handoff or terminal retained close",
    "artifact envelope ingress reached Drop before every admitted page was transferred or bounded-closed",
    "artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots",
    "artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant",
  ].join("\n");
  if (!toolJobWriterEnvelopeCallerRetainedExact(retainedWriterInitializer, retainedWriterEditor, retainedWriterWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Writer-envelope-route was falsely rejected.");
  if (toolJobWriterEnvelopeCallerRetainedExact(retainedWriterInitializer, retainedWriterEditor, retainedWriterWasm.replace("source: &js_sys::Uint8Array", "source: &[u8]"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Writer-post-lift-dynamic-page was falsely accepted.");
  if (toolJobWriterEnvelopeCallerRetainedExact(retainedWriterInitializer, retainedWriterEditor, retainedWriterWasm.replace("acknowledge_artifact_store_replacement(handle.runtime_handle())", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Writer-completion-without-exact-ack was falsely accepted.");
  if (toolJobWriterEnvelopeCallerRetainedExact(retainedWriterInitializer, retainedWriterEditor, retainedWriterWasm, retainedWriterPlugin.replace("self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))", "drop(job)"))) throw new Error("[verify interactivity tool-jobs] self-test Writer-false-terminal-initializer-drop was falsely accepted.");
  const retainedJackStore = [
    "pub fn artifact_owned_spr_edit_history_decoder",
    "struct ArtifactOwnedSprMutationArrayAuthority",
    "self.scalar_entry",
    "artifact-spr.mutation-array-cancelled",
    "id_digest: [u8; 32]",
    'Self::push_revision_record(&mut self.revision.applied, identity_digest, b"applied", &id, edit_digest)?',
    "SPR edit decode reached Drop before exact publication or bounded retirement",
    "pub fn preflight_page_bytes(&self, page_bytes: usize)",
    "pub fn admit_preflighted_page(&mut self, page: OwnedSchemaDecodePage)",
  ].join("\n");
  const retainedJackCodec = [
    "pub struct JackEnvelopeOwnedFieldCatalog",
    "artifact_owned_spr_edit_history_decoder",
    "struct JackMutationDecodeAuthority",
    "OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>",
    "struct JackSnapshotCloneAuthority",
    "struct JackStoreInitializationAuthority",
    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<JackSnapshot, TrinityGraphMutation> for JackStoreInitializationAuthority",
    "JackStoreInitializationPhase::ValidateEditPair",
    "JackStoreInitializationPhase::SeedHistory",
    "JackStoreInitializationPhase::BuildCandidate",
    "ArtifactStoreInitializationRuntime::new",
    "ArtifactStore::from_initialized_runtime_with_owners",
    "self.generation.0.checked_add(1)",
    "Jack store initialization authority reached Drop before exact candidate handoff or retained rejection close",
    "jack_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally",
    "jack_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty",
    "jack_nested_mutation_and_child_snapshot_retire_one_exact_owner_per_grant",
  ].join("\n");
  const retainedJackEditor = "fn build_document_store_initialization_job( jack_document_store_initialization_job(envelope, operation, generation) jack_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed jack_live_envelope_cancel_closes_retained_pages_without_publication";
  const retainedJackWasm = retainedWriterWasm.replace("WriterEnvelopeLoadHandle", "JackEnvelopeLoadHandle");
  if (!toolJobJackEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedJackWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Jack-envelope-route was falsely rejected.");
  if (toolJobJackEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec.replace("artifact_owned_spr_edit_history_decoder", "artifact_bounded_history_entry_decoder"), retainedJackEditor, retainedJackWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Jack-domain-only-whole-edit-decoder was falsely accepted.");
  if (toolJobJackEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedJackWasm.replace("source: &js_sys::Uint8Array", "source: &[u8]"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Jack-post-lift-dynamic-page was falsely accepted.");
  if (toolJobJackEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedJackWasm.replace("acknowledge_artifact_store_replacement(handle.runtime_handle())", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Jack-completion-without-exact-ack was falsely accepted.");
  if (toolJobJackEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedJackWasm, retainedWriterPlugin.replace("self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))", "drop(job)"))) throw new Error("[verify interactivity tool-jobs] self-test Jack-false-terminal-initializer-drop was falsely accepted.");
  if (toolJobJackEnvelopeCallerRetainedExact(retainedJackStore.replace("self.scalar_entry", "let raw = serde_json::from_slice(bytes)"), retainedJackCodec, retainedJackEditor, retainedJackWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Jack-string-mutation-whole-buffer-bypass was falsely accepted.");
  const retainedTrinityRewrite = [
    "type TrinityRewriteApp = VcsArtifactApp<EditorApp<TrinityJackPlayApp>>",
    "pub struct TrinityRewriteEnvelopeLoadHandle",
    "fn runtime_handle(&self) -> ArtifactEnvelopeDecodeOperationHandle",
    "TRINITY_REWRITE_ENVELOPE_MAXIMUM_PAGES",
    "TRINITY_REWRITE_ENVELOPE_MAXIMUM_BYTES",
    "trinity_rewrite_envelope_credits_admit_exact_caps_and_reject_zero_or_plus_one",
    "pub async fn new()",
    "VcsArtifactApp::new(EditorApp::<TrinityJackPlayApp>::default()).await",
    "pub fn begin_envelope_load(",
    "begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)",
    "source: &js_sys::Uint8Array",
    "pub enum TrinityRewriteEnvelopePageFault",
    "pub struct TrinityRewriteEnvelopePageAdmission",
    "TrinityRewriteCallerPageOwner::new(owner)",
    "source.clone()",
    "construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len",
    "let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]",
    "source.copy_to(&mut bytes[..len])",
    "ArtifactEnvelopeDecodePage::from_preflighted_array(bytes, len)",
    "pub fn retry_envelope_page(",
    "pub fn is_same_page(",
    "pub fn take_page(&mut self) -> Option<js_sys::Uint8Array>",
    "pub fn close_step(&mut self) -> bool",
    "pub fn terminal_is_empty(&self) -> bool",
    "trinity_rewrite_rejected_page_preserves_pointer_content_and_retry_owner",
    "trinity_rewrite_page_cap_plus_one_rejects_before_owner_construction",
    "trinity_rewrite_stale_generation_and_slot_aba_never_match",
    "trinity_rewrite_checked_out_drop_preserves_raw_caller_authority",
    "trinity_rewrite_rejected_page_close_retires_one_owner_per_grant",
    "seal_artifact_envelope_ingress(handle.runtime_handle())",
    "app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)",
    "advance_artifact_envelope_load(handle.runtime_handle())",
    "acknowledge_artifact_store_replacement(handle.runtime_handle())",
    "cancel_artifact_envelope_load(handle.runtime_handle())",
    "close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)",
  ].join("\n");
  if (!toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Trinity-Rewrite-envelope-route was falsely rejected.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, `${retainedTrinityRewrite}\nenvelope_json: Option<String>`, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-whole-buffer-constructor-bypass was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("source: &js_sys::Uint8Array", "source: &[u8]"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-post-lift-dynamic-page was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("fn runtime_handle(&self) -> ArtifactEnvelopeDecodeOperationHandle", "fn operation_only(&self) -> u64"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-generation-handle-erasure was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]", "let mut bytes = Vec::new()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-fixed-page-owner-removal was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("pub struct TrinityRewriteEnvelopePageAdmission", "pub struct ErasedEnvelopePageAdmission"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-typed-rejected-page-result-erasure was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("TrinityRewriteCallerPageOwner::new(owner)", "drop(owner)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-rejected-page-ordinary-drop was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("source.clone()", "Uint8Array::new(source)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-rejected-page-byte-clone was falsely accepted.");
  if (
    toolJobTrinityRewriteEnvelopeCallerRetainedExact(
      retainedJackStore,
      retainedJackCodec,
      retainedJackEditor,
      retainedTrinityRewrite.replace(
        "construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len\nlet mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]\nsource.copy_to(&mut bytes[..len])",
        "let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]\nsource.copy_to(&mut bytes[..len])\nconstruct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len",
      ),
      retainedWriterPlugin,
    )
  )
    throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-page-copy-before-preflight was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("pub fn take_page(&mut self) -> Option<js_sys::Uint8Array>", "fn discard_page(&mut self)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-rejected-page-retrieval-removal was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("pub fn retry_envelope_page(", "fn drop_retry_owner("), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-rejected-page-retry-removal was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("pub fn close_step(&mut self) -> bool", "fn drop_rejected_page(&mut self)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-rejected-page-close-removal was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("seal_artifact_envelope_ingress(handle.runtime_handle())", "submit_unsealed(handle.runtime_handle())"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-unsealed-ingress was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)", "app.run_to_completion()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-unbounded-poll-turn was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("acknowledge_artifact_store_replacement(handle.runtime_handle())", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-terminal-without-exact-ack was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("cancel_artifact_envelope_load(handle.runtime_handle())", "drop(handle)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-cancel-owner-drop was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)", "close_all()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-bulk-close was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("trinity_rewrite_envelope_credits_admit_exact_caps_and_reject_zero_or_plus_one", "trinity_rewrite_envelope_accepts_unbounded_credits"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-cap-plus-one-fixture-missing was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite, retainedWriterPlugin.replace("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots", "artifact_envelope_ingress_drops_plus_one"))) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-fixed-registry-exact-handback-fixture-missing was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite, retainedWriterPlugin.replace("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant", "artifact_envelope_ingress_bulk_close"))) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-one-page-close-fixture-missing was falsely accepted.");
  const retainedGisMapCodec = [
    "pub struct GisMapEnvelopeOwnedFieldCatalog",
    "artifact_owned_spr_edit_history_decoder",
    "GisMapSnapshotDecodeAuthority",
    "GisMapMutationDecodeAuthority",
    "OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>",
    "struct GisMapSnapshotCloneAuthority",
    "struct GisMapStoreInitializationAuthority",
    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<GisMapSnapshot, GisMapMutation> for GisMapStoreInitializationAuthority",
    "GisMapStoreInitializationPhase::ValidateEditPair",
    "GisMapStoreInitializationPhase::SeedHistory",
    "GisMapStoreInitializationPhase::BuildCandidate",
    "ArtifactStoreInitializationRuntime::new",
    "ArtifactStore::from_initialized_runtime_with_owners",
    "self.generation.0.checked_add(1)",
    "value.positions.pop()",
    "value.routes.pop()",
    "value.regions.pop()",
    "Self::child_step(&mut value.drawing",
    "Self::child_step(image",
    "Self::child_step(&mut value.value",
    "dsl::DslValue::Array(values)",
    "dsl::DslValue::Object(values)",
    "CreatePosition(payload)",
    "DeletePosition(payload)",
    "ReorderPositions(payload)",
    "ReplacePositionData(payload)",
    "CreateRoute(payload)",
    "DeleteRoute(payload)",
    "ReorderRoutes(payload)",
    "ReplaceRouteData(payload)",
    "CreateRegion(payload)",
    "DeleteRegion(payload)",
    "ReorderRegions(payload)",
    "ReplaceRegionData(payload)",
    "GIS store initialization authority reached Drop before exact candidate handoff or retained rejection close",
    "gis_map_store_initializer_publishes_next_generation_and_candidate_closes_incrementally",
    "gis_map_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty",
    "gis_map_nested_value_mutation_and_all_child_handles_retire_one_owner_per_grant",
    "gis_map_all_twelve_mutation_variants_preserve_catalog_order_and_zero_grant_ownership",
  ].join("\n");
  const retainedGisMapEditor = "fn build_document_store_initialization_job( gis_map_document_store_initialization_job(envelope, operation, generation) gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed gis_map_live_envelope_cancel_closes_retained_pages_without_publication duplicate GIS load acknowledgement is a no-op";
  const retainedGisMapWasm = retainedWriterWasm.replace("WriterEnvelopeLoadHandle", "GisMapEnvelopeLoadHandle");
  if (!toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec, retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-GIS-Map-envelope-route was falsely rejected.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("artifact_owned_spr_edit_history_decoder", "artifact_bounded_history_entry_decoder"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-domain-only-whole-edit-decoder was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec, retainedGisMapEditor, retainedGisMapWasm.replace("source: &js_sys::Uint8Array", "source: &[u8]"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-post-lift-dynamic-page was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec, retainedGisMapEditor, retainedGisMapWasm.replace("acknowledge_artifact_store_replacement(handle.runtime_handle())", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-completion-without-exact-ack was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec, retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin.replace("self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))", "drop(job)"))) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-false-terminal-initializer-drop was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("self.generation.0.checked_add(1)", "self.generation.0 + 1"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-unchecked-generation-publication was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("Self::child_step(&mut value.drawing", "drop(value.drawing)"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-drawing-child-deep-drop was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("dsl::DslValue::Object(values)", "drop(value)"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-nested-value-deep-drop was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("ReplaceRegionData(payload)", "drop(payload)"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-mutation-catalog-hole was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("gis_map_all_twelve_mutation_variants_preserve_catalog_order_and_zero_grant_ownership", "gis_map_partial_mutation_catalog"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-zero-grant-catalog-fixture-missing was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec, retainedGisMapEditor, `${retainedGisMapWasm}\nenvelope_json: &str`, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-whole-buffer-ingress-bypass was falsely accepted.");
  const retainedRasterCodec = [
    "pub struct RasterEnvelopeOwnedFieldCatalog",
    "artifact_owned_spr_edit_history_decoder",
    "RasterSnapshotDecodeAuthority",
    "RasterMutationDecodeAuthority",
    "OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>",
    "struct RasterSnapshotBoundsAuthority",
    "struct RasterSnapshotCloneAuthority",
    "struct RasterMutationDigestAuthority",
    "struct RasterMutationCandidateAuthority",
    "RASTER_RETIREMENT_STACK_CAPACITY",
    "pages: std::mem::ManuallyDrop<[Option<Box<RasterRetirementFramePage>>; RASTER_RETIREMENT_STACK_PAGE_COUNT]>",
    "pending_push: std::mem::ManuallyDrop<Option<RasterRetirementOwner>>",
    "struct RasterStoreInitializationAuthority",
    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<RasterSnapshot, RasterMutation> for RasterStoreInitializationAuthority",
    "RasterStoreInitializationPhase::ValidateEditPair",
    "RasterStoreInitializationPhase::SeedHistory",
    "RasterStoreInitializationPhase::BuildCandidate",
    "if cx.should_yield()",
    "if cx.should_yield()",
    "if cx.should_yield()",
    "fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool",
    "cx.consume_fuel(1)",
    "self.cancel_requested || cx.is_cancelled()",
    "ArtifactStoreInitializationRuntime::new",
    "ArtifactStore::from_initialized_runtime_with_owners",
    "self.generation.0.checked_add(1)",
    "value.layers.pop()",
    "value.assets.take_last_entry()",
    "dsl::DslValue::Array(values)",
    "dsl::DslValue::Object(values)",
    "CreateLayer(payload)",
    "DeleteLayer(payload)",
    "ReorderLayers(payload)",
    "RenameLayer(payload)",
    "ChangeLayerVisible(payload)",
    "ChangeLayerOpacity(payload)",
    "ChangeLayerBlendMode(payload)",
    "MoveLayer(payload)",
    "ResizeLayer(payload)",
    "ChangeLayerAdjustmentKind(payload)",
    "AddLayerAsset(payload)",
    "RemoveLayerAsset(payload)",
    "Raster store initialization authority reached Drop before exact candidate handoff or retained rejection close",
    "raster_store_initializer_publishes_next_generation_and_candidate_closes_incrementally",
    "raster_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty",
    "raster_store_initializer_zero_budget_advances_no_owner_or_phase",
    "raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant",
    "raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant",
    "raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner",
    "raster_snapshot_bounds_and_clone_advance_one_pre_admitted_unit_with_low_nonzero_fuel",
    "raster_expired_deadline_advances_no_bounds_clone_or_mutation_owner",
    "raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic",
    "raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal",
    "raster_retirement_uses_allocation_capacity_and_fixed_iterative_depth",
    "raster_nested_owner_item_and_byte_capacity_plus_one_reject_before_clone",
    "raster_empty_bounds_and_mounted_sixty_four_fuel_progress_across_second_map_page",
    "raster_retirement_page_credit_is_claimed_before_allocation_and_returned_with_backing",
    "raster_owned_map_removal_returns_exact_pair_and_populated_drop_refuses",
    "const RASTER_MAXIMUM_CONTROL_BYTES: usize = RASTER_MAXIMUM_CONTROL_BACKINGS * RASTER_CONTROL_BACKING_BYTES",
    "source_control_bytes: usize",
    "self.source_control_bytes = RASTER_MAXIMUM_CONTROL_BYTES",
    "raster-store.control-backing-double-reservation",
    "static RASTER_RETIREMENT_PROCESS_PAGES: std::sync::atomic::AtomicUsize",
    "static RASTER_INITIALIZATION_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize",
    "static RASTER_STANDALONE_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize",
    "held_items: usize",
    "held_bytes: usize",
    "control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>",
    "control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>",
    "let control = RasterStandaloneControlCredit::try_claim().ok();",
    "let control = RasterStandaloneControlCredit::try_claim().ok();",
    "fn claim_control_if_available(&mut self) -> Result<bool, String>",
    "Err(\"raster-store.standalone-control-capacity\") => Ok(false)",
    "control_returned: bool",
    "remaining: RASTER_NON_STACK_CONTROL_BACKINGS",
    "control_reservation: std::mem::ManuallyDrop<Option<RasterInitializationControlReservation>>",
    "normal completion returns every non-stack process control credit",
    "standalone Box control credit is returned before terminal-empty",
    "standalone Arc and inner Box control credits return before terminal-empty",
    "fn reserve_page_credit(&mut self, page_index: usize)",
    "compare_exchange(current, next",
    "self.return_page_credit(page_index)?",
    "pages: std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]>",
    "Raster owned map reached Drop before every entry and page backing was explicitly retired",
    "Populated Raster owned maps require the retained page clone authority",
    "Raster maps require the retained page decoder",
    "Populated Raster owned map DSL materialization is forbidden; interactive production routes require the retained page output authority",
    "dsl::FieldValue::Map(Vec::new())",
    "pub fn remove_entry(&mut self, key: &str) -> Option<RasterOwnedMapEntry<V>>",
    "RasterOwnedMapInsert::Replaced",
    "raster_empty_asset_map_retirement_has_no_hidden_allocation_release",
    "value.assets.take_last_entry()",
    "value.assets.take_empty_page_backing()",
    "RasterRetirementOwner::AssetMapPage(page)",
    "RasterRetirementOwner::ValueMapPage(page)",
    "RasterOwnedMap::<V>::conservative_page_credit_bytes()",
    "fn observe_candidate_capacity(",
    "fn raster_exact_string_from_parts(",
    "const RASTER_CONTROL_BACKING_BYTES: usize = RASTER_OWNED_FIELD_BYTES",
    "const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = RASTER_RETIREMENT_STACK_PAGE_COUNT + RASTER_NON_STACK_CONTROL_BACKINGS",
    "RasterRetirementOwner::BoxedLayer(Some(layer))",
    "const RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY: usize = RASTER_RETIREMENT_LAYER_FRAMES + RASTER_RETIREMENT_VALUE_FRAMES + RASTER_RETIREMENT_WRAPPER_FRAMES",
    "const RASTER_RETIREMENT_STACK_CAPACITY: usize = RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY + RASTER_RETIREMENT_REJECTED_OWNER_MARGIN",
    "raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly",
    "raster_observed_capacity_and_combined_retirement_depth_are_exact",
    "raster_box_and_arc_control_backings_require_and_report_fixed_credit",
    "raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation",
    "raster_arc_factory_full_saturation_preserves_exact_producer_through_every_control_phase",
    "raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact",
    "impl<V> dsl::ToValue for RasterOwnedMap<V>",
    "impl<V> dsl::FromValue for RasterOwnedMap<V>",
    "Populated Raster owned map serialization is forbidden; interactive production routes require the retained page output authority",
    "    pub assets: RasterOwnedMap<RasterAssetChild>,",
    "    pub assets: RasterOwnedMap<RasterAssetChild>,",
    "        params: RasterOwnedMap<dsl::DslValue>,",
    "raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact",
    "pub(crate) fn require_empty_output_shell(&self) -> Result<(), &'static str>",
    "self.layers.is_empty() && self.assets.is_empty()",
    "Populated Raster snapshot output is forbidden; interactive production routes require the retained page output authority",
    "pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String",
    "pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String",
    "fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)",
    "fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)",
    "fn print_raster_snapshot_body(s: &RasterSnapshot) -> String",
    "fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>",
    'assert!(map.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    'assert!(params.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    'assert!(map.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    'assert!(params.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    'assert!(list.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    'assert!(list.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    "s.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);",
    "s.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);",
    "self.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);",
    "self.require_empty_output_shell().map_err(|error| store::PackError::Schema(error.to_owned()))?;",
    "fn raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact()",
    "let plus_one_param_value_pointer = plus_one_param_value.as_ptr();",
    "let rejected_param = params.insert",
    "params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value))",
    "let rejected_param_value = match &rejected_param.value",
    'assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");',
    "RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) })",
    "let plus_one_asset_child_pointer = plus_one_asset_child.child_id.as_ptr();",
    "let rejected_asset = assets",
    "assets.insert(plus_one_asset_key, plus_one_asset_child)",
    'assert_eq!(rejected_asset.value.child_id.as_ptr(), plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");',
    "RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) })",
    ...["bmp", "png", "tiff", "jpg"].map(
      (format) =>
        `pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let image = raster_composite_image(snapshot)?; let target = semio_image_to_format(&image, ${format.toUpperCase()}_DIALECT)?; semio_s_artifact_stdio_${format}::io::encode_${format}(&target) }`,
    ),
    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let image = raster_composite_image(snapshot)?; let gif89a = semio_image_to_format(&image, GIF89A_DIALECT)?; semio_s_artifact_stdio_gif::standards::v87a::subsets::any::io::encode_gif(&gif87a::from_89a(&gif89a)) }",
    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let (svg, _width, _height) = crate::artifacts::raster::io::raster_document_json_to_svg(snapshot)?; Ok(svg.into_bytes()) }",
    'pub const RASTER_PDF_EXPORT_UNSUPPORTED: &str = "pdf export not supported for a raster document";',
    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let _ = snapshot; Err(RASTER_PDF_EXPORT_UNSUPPORTED.to_string()) }",
    'pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str = "dwg export not supported for a raster document";',
    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let _ = snapshot; Err(RASTER_DWG_EXPORT_UNSUPPORTED.to_string()) }",
    "std::mem::size_of::<RasterOwnedRetirement>() <= RASTER_CONTROL_BACKING_BYTES",
    "std::mem::size_of::<RasterRetirementFramePage>() <= RASTER_CONTROL_BACKING_BYTES",
    "fn raster_maximum_combined_layer_and_value_depth_retires_to_terminal()",
    "Ok(Some((previous_key, previous)))",
    "Ok(Some((previous_key, previous)))",
    "Ok(Some((previous_key, previous)))",
    "replacement returns the exact displaced pair",
    "assert_eq!(previous_key.as_ptr(), old_key_pointer)",
    "candidate_disposer",
    "let released_bytes = value.capacity();",
    "let bytes = value.capacity();",
  ].join("\n");
  const retainedRasterEditor = [
    "fn build_envelope_decode_owner_bundle() raster_envelope_decode_owner_bundle() fn build_document_store_initialization_job( raster_document_store_initialization_job(envelope, operation, generation)",
    "fn admit_raster_envelope(app: &mut VcsArtifactApp<EditorApp<RasterPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {",
    "let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect(\"Raster live envelope ingress credits\");",
    "for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {",
    "let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];",
    "let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect(\"bounded Raster live envelope page\");",
    "app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!(\"Raster live envelope page admission failed: {fault}\"));",
    "assert!(app.seal_artifact_envelope_ingress(handle).expect(\"Raster live envelope seal/submit\"));",
    "app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect(\"one Raster live maintenance turn\");",
    "let poll = app.advance_artifact_envelope_load(handle).expect(\"Raster live load advancement\");",
    "match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect(\"Raster fixture envelope retirement\") {",
    "async fn raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {",
    "assert_eq!(handle.generation, base_generation);",
    "assert!(app.acknowledge_artifact_store_replacement(handle).expect(\"first exact Raster load acknowledgement\"));",
    "assert!(!app.acknowledge_artifact_store_replacement(handle).expect(\"duplicate Raster load acknowledgement is a no-op\"));",
    "async fn raster_live_envelope_cancel_closes_retained_pages_without_publication() {",
    "app.cancel_artifact_envelope_load(handle).expect(\"cancel exact Raster ingress\");",
  ].join("\n");
  if (!toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Raster-envelope-route was falsely rejected.");
  for (const signature of [
    "pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String",
    "pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String",
    "fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)",
    "fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)",
    "fn print_raster_snapshot_body(s: &RasterSnapshot) -> String",
    "fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>",
  ]) {
    const suspended = signature.replace("fn ", "async fn ");
    if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace(signature, suspended), retainedRasterEditor, retainedWriterPlugin))
      throw new Error(`[verify interactivity tool-jobs] self-test Raster-immediate-signature ${signature} was falsely accepted as async.`);
  }
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace('let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Raster live envelope ingress credits");\n', ""), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-page-admitted-without-ingress-credits was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace('let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Raster live envelope page");\napp.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Raster live envelope page admission failed: {fault}"));', 'app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Raster live envelope page admission failed: {fault}"));\nlet page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Raster live envelope page");'), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-page-admitted-before-bounded-construction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]", "let mut bytes = Vec::new()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-fixed-page-owner-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("assert_eq!(handle.generation, base_generation);", "let _ = handle;"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-generation-handle-erasure was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("-> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {", "{"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-untyped-ingress-handle was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {", "let chunk = wire;"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-single-whole-wire-page was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.seal_artifact_envelope_ingress(handle)", "app.submit_all()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unsealed-ingress-submit was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)", "app.run_to_completion()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unpumped-load-progress was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.advance_artifact_envelope_load(handle)", "app.await_load()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unpolled-load-terminal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("duplicate Raster load acknowledgement is a no-op", "second Raster acknowledgement replaces the store again"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-repeatable-load-acknowledgement was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("async fn raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed()", "async fn raster_envelope_loads()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-live-submit-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("async fn raster_live_envelope_cancel_closes_retained_pages_without_publication()", "async fn raster_envelope_cancels()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-live-cancel-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.generation.0.checked_add(1)", "self.generation.0 + 1"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unchecked-generation-publication was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("if cx.should_yield()", "if false"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-deadline-fuel-guard-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.cancel_requested || cx.is_cancelled()", "self.cancel_requested"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-worker-cancel-guard-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_store_initializer_zero_budget_advances_no_owner_or_phase", "raster_zero_budget_advances_work"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-zero-budget-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("RemoveLayerAsset(payload)", "drop(payload)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mutation-catalog-hole was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("dsl::DslValue::Object(values)", "drop(value)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-nested-value-deep-drop was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant", "raster_shallow_snapshot_fixture"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-nested-close-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant", "raster_partial_owner_fixture"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-cap-terminal-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner", "raster_unbounded_page_fixture"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-page-plus-one-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len())", "store::ArtifactEnvelopeDecodePage::try_from_vec(bytes.to_vec(), chunk.len())"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-dynamic-page-route was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, `${retainedRasterEditor}\nenvelope_json: &str`, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-whole-buffer-bypass was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replaceAll("acknowledge_artifact_store_replacement(handle)", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-terminal-without-exact-ack was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.cancel_artifact_envelope_load(handle)", "drop(handle)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-cancel-owner-drop was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)", "close_all()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-bulk-close was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin.replace("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots", "artifact_envelope_ingress_drops_plus_one"))) throw new Error("[verify interactivity tool-jobs] self-test Raster-fixed-registry-exact-handback-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin.replace("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant", "artifact_envelope_ingress_bulk_close"))) throw new Error("[verify interactivity tool-jobs] self-test Raster-one-page-close-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("struct RasterSnapshotBoundsAuthority", "struct RasterPostCloneBoundsAuthority"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-recursive-preflight-authority-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool", "fn raster_post_work_fuel(cx: &mut semio_framework_job::StepContext<'_>) -> bool"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-pre-work-fuel-authority-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("cx.consume_fuel(1)", "cx.consume_fuel(RASTER_CONTROL_BACKING_BYTES as u64)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-64-fuel-byte-charge was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.source_control_bytes = RASTER_MAXIMUM_CONTROL_BYTES", "self.source_bytes = RASTER_MAXIMUM_NESTED_BYTES"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-payload-double-counting was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("static RASTER_INITIALIZATION_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize", "static RASTER_UNTRACKED_CONTROLS: std::sync::atomic::AtomicUsize"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-non-stack-process-control-reservation-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("static RASTER_STANDALONE_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize", "static RASTER_UNTRACKED_STANDALONE_CONTROLS: std::sync::atomic::AtomicUsize"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-standalone-Box-Arc-control-reservation-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>", "control: Option<RasterStandaloneControlCredit>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-standalone-control-terminal-shell-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("let control = RasterStandaloneControlCredit::try_claim().ok();", "let control = RasterStandaloneControlCredit::try_claim().expect(\"saturation panic\");"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-saturated-constructor-panic-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Err(\"raster-store.standalone-control-capacity\") => Ok(false)", "Err(code) => panic!(\"{code}\")"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-saturated-retirement-resume-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("control_returned: bool", "control_returned_without_witness: ()"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-Arc-control-return-witness-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("normal completion returns every non-stack process control credit", "normal completion leaks process control credit"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-process-control-zero-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("fn reserve_page_credit(&mut self, page_index: usize)", "fn allocate_page_without_credit(&mut self, page_index: usize)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-retirement-page-credit-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pages: std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]>", "pages: [Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-fail-closed-shell-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\npub fn remove(&mut self, key: &str)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-key-discarding-remove-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Populated Raster owned maps require the retained page clone authority", "Raster clone allocates map pages"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-whole-clone-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Raster maps require the retained page decoder", "Raster serde allocates map pages"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-uncredited-map-serde-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Populated Raster owned map DSL materialization is forbidden; interactive production routes require the retained page output authority", "ordinary populated DSL materialization is allowed"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-DSL-fail-closure-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("dsl::FieldValue::Map(Vec::new())", "let mut entries = Vec::with_capacity(self.length); for (key, value) in self { entries.push((key.clone(), value.to_value())); } dsl::FieldValue::Map(entries)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-uncredited-DSL-loop-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nserde_json::to_vec(source)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-whole-recursive-encode-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nsource.clone()`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-whole-recursive-clone-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\noperation.diff(current)\ndiff.apply(current)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-monolithic-history-apply-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("candidate_disposer", "discarded_candidate"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-complete-candidate-retirement-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("let released_bytes = value.capacity();", "let released_bytes = value.len();"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-string-capacity-retirement-erasure was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("let bytes = value.capacity();", "let bytes = value.len();"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-byte-capacity-retirement-erasure was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pages: std::mem::ManuallyDrop<[Option<Box<RasterRetirementFramePage>>; RASTER_RETIREMENT_STACK_PAGE_COUNT]>", "active: Box<RasterOwnedRetirement>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-fixed-iterative-retirement-erasure was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic", "raster_small_mutation_clones_deep_snapshot"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-low-fuel-deep-mutation-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal", "raster_cancel_drops_completed_candidate"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-cancel-after-complete-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_nested_owner_item_and_byte_capacity_plus_one_reject_before_clone", "raster_nested_owner_capacity_without_plus_one"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-nested-owner-capacity-plus-one-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_empty_asset_map_retirement_has_no_hidden_allocation_release", "raster_empty_asset_map_bulk_drop"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-empty-map-allocation-release-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("value.assets.take_empty_page_backing()", "drop(std::mem::take(&mut value.assets))"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-page-retirement-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("fn observe_candidate_capacity(", "fn trust_requested_candidate_capacity("), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-observed-capacity-admission-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("const RASTER_CONTROL_BACKING_BYTES: usize = RASTER_OWNED_FIELD_BYTES", "const RASTER_CONTROL_BACKING_BYTES: usize = 0"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-backing-credit-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = RASTER_RETIREMENT_STACK_PAGE_COUNT + RASTER_NON_STACK_CONTROL_BACKINGS", "const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = 16"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-backing-count-regression was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("RASTER_RETIREMENT_LAYER_FRAMES + RASTER_RETIREMENT_VALUE_FRAMES + RASTER_RETIREMENT_WRAPPER_FRAMES", "RASTER_MAXIMUM_NESTED_DEPTH * 2 + 8"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-combined-retirement-depth-regression was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY + RASTER_RETIREMENT_REJECTED_OWNER_MARGIN", "RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-owner-retirement-margin-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly", "raster_empty_map_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-cap-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_observed_capacity_and_combined_retirement_depth_are_exact", "raster_requested_capacity_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-observed-capacity-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_box_and_arc_control_backings_require_and_report_fixed_credit", "raster_unreported_control_drop"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-owner-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation", "raster_standalone_saturation_panics"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-standalone-max-plus-one-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_arc_factory_full_saturation_preserves_exact_producer_through_every_control_phase", "raster_arc_saturation_drops_producer"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-Arc-every-control-phase-saturation-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_empty_dsl_map_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-DSL-hostile-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_empty_serde_map_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-serde-hostile-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("    pub assets: RasterOwnedMap<RasterAssetChild>,", "    pub assets: std::collections::HashMap<String, RasterAssetChild>,"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unguarded-map-field-escape was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("impl<V> dsl::ToValue for RasterOwnedMap<V>", "impl<V: dsl::ToValue> dsl::ToValue for RasterOwnedMapEntry<V>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-owned-map-output-authority-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("impl<V> dsl::FromValue for RasterOwnedMap<V>", "impl<V: dsl::FromValue> dsl::FromValue for RasterOwnedMapEntry<V>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-owned-map-decode-authority-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n#[cfg_attr(test, serde(serialize_with = "crate::artifacts::raster::serialize_empty_owned_map"))]`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-dangling-serde-oracle-route was falsely accepted.");
  const restoredRasterOwnedMapSerializeLoop = `impl<V: serde::Serialize> serde::Serialize for RasterOwnedMap<V> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.length))?;
        for (key, value) in self {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}`;
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterOwnedMapSerializeLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-populated-serde-loop-bound-restoration was falsely accepted.");
  const restoredRasterEncAssetMapLoop = `pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String {
    format!("[{}]", map.iter().map(|(k, v)| format!("[{},{}]", enc_str(k), enc_child(v))).collect::<Vec<_>>().join(","))
}`;
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterEncAssetMapLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-text-asset-map-whole-loop-restoration was falsely accepted.");
  const restoredRasterEncParamsLoop = `pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String {
    format!("[{}]", params.iter().map(|(k, v)| format!("[{},{}]", enc_str(k), hex_encode(&serde_json::to_vec(v).unwrap_or_default()))).collect::<Vec<_>>().join(","))
}`;
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterEncParamsLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-text-parameter-map-whole-loop-restoration was falsely accepted.");
  const restoredRasterWriteAssetMapLoop = `fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>) {
    store::pack_rt::write_varint_u64(out, map.len() as u64);
    for (k, v) in map {
        write_str_lp(out, k);
        write_child(out, v);
    }
}`;
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterWriteAssetMapLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-pack-asset-map-whole-loop-restoration was falsely accepted.");
  const restoredRasterWriteParamsLoop = `fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>) {
    store::pack_rt::write_varint_u64(out, params.len() as u64);
    for (k, v) in params {
        write_str_lp(out, k);
        write_bytes_lp(out, &serde_json::to_vec(v).unwrap_or_default());
    }
}`;
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterWriteParamsLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-pack-parameter-map-whole-loop-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("semio_s_artifact_stdio_png::io::encode_png(&target)", "Ok(<RasterSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-DSL-print-under-foreign-extension-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("let image = raster_composite_image(snapshot)?;", "let image = SemioImage::default();"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-document-composite-bypass was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("crate::artifacts::raster::io::raster_document_json_to_svg(snapshot)?", 'Ok::<_, String>((String::from("<svg/>"), 0u32, 0u32))?'), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-vector-exporter-composite-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Err(RASTER_PDF_EXPORT_UNSUPPORTED.to_string())", "Ok(Vec::new())"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-silent-empty-output-instead-of-typed-decline was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str =", "const DWG_UNSUPPORTED: &str ="), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-decline-reason-constant-erasure was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {", "fn unmounted_serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-entry-point-unmounting was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);", "let _ = self;"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-DSL-output-preflight-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("s.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);\n", ""), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-body-output-preflight-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.require_empty_output_shell().map_err(|error| store::PackError::Schema(error.to_owned()))?;", "let _ = self;"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-pack-output-preflight-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_empty_snapshot_output_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-snapshot-output-hostile-fixture-removal was falsely accepted.");
  const rasterRejectedParamValueIdentity = 'assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");';
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace(rasterRejectedParamValueIdentity, ""), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-value-identity-removal was falsely accepted.");
  if (
    toolJobRasterEnvelopeCallerRetainedExact(
      retainedJackStore,
      retainedRasterCodec.replace(rasterRejectedParamValueIdentity, 'assert_eq!(rejected_param.key.as_ptr(), plus_one_param_pointer, "rejected output parameter returns only the key allocation");'),
      retainedRasterEditor,
      retainedWriterPlugin,
    )
  )
    throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-value-identity-substitution was falsely accepted.");
  const rasterRejectedParamMovedInsert = "params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value))";
  const rasterRejectedParamValueBinding = "let rejected_param_value = match &rejected_param.value";
  const rasterParamBindingBeforeInsertion = retainedRasterCodec
    .replace(rasterRejectedParamValueBinding, "")
    .replace(rasterRejectedParamMovedInsert, `${rasterRejectedParamValueBinding}\n${rasterRejectedParamMovedInsert}`);
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterParamBindingBeforeInsertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-binding-before-insertion was falsely accepted.");
  const rasterRejectedParamRetirement = "RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) })";
  const rasterParamRetirementBeforeAssertion = retainedRasterCodec
    .replace(rasterRejectedParamRetirement, "")
    .replace(rasterRejectedParamValueIdentity, `${rasterRejectedParamRetirement}\n${rasterRejectedParamValueIdentity}`);
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterParamRetirementBeforeAssertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-retirement-before-identity was falsely accepted.");
  const rasterRejectedAssetChildIdentity = 'assert_eq!(rejected_asset.value.child_id.as_ptr(), plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");';
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace(rasterRejectedAssetChildIdentity, ""), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-child-identity-removal was falsely accepted.");
  if (
    toolJobRasterEnvelopeCallerRetainedExact(
      retainedJackStore,
      retainedRasterCodec.replace(rasterRejectedAssetChildIdentity, 'assert_eq!(rejected_asset.key.as_ptr(), plus_one_asset_pointer, "rejected output asset returns only the key allocation");'),
      retainedRasterEditor,
      retainedWriterPlugin,
    )
  )
    throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-child-identity-substitution was falsely accepted.");
  const rasterRejectedAssetMovedInsert = "assets.insert(plus_one_asset_key, plus_one_asset_child)";
  const rasterAssetBindingBeforeInsertion = retainedRasterCodec
    .replace(rasterRejectedAssetChildIdentity, "")
    .replace(rasterRejectedAssetMovedInsert, `${rasterRejectedAssetChildIdentity}\n${rasterRejectedAssetMovedInsert}`);
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterAssetBindingBeforeInsertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-binding-before-insertion was falsely accepted.");
  const rasterRejectedAssetRetirement = "RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) })";
  const rasterAssetRetirementBeforeAssertion = retainedRasterCodec
    .replace(rasterRejectedAssetRetirement, "")
    .replace(rasterRejectedAssetChildIdentity, `${rasterRejectedAssetRetirement}\n${rasterRejectedAssetChildIdentity}`);
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterAssetRetirementBeforeAssertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-retirement-before-identity was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("std::mem::size_of::<RasterRetirementFramePage>() <= RASTER_CONTROL_BACKING_BYTES", "true"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-page-size-proof-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_maximum_combined_layer_and_value_depth_retires_to_terminal", "raster_separate_depth_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-combined-depth-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("RasterOwnedMapInsert::Replaced", "RasterOwnedMapInsert::DiscardedKey"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-replacement-key-owner-handback-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("replacement returns the exact displaced pair", "replacement drops incoming key"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-replacement-pointer-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nsource.assets.clone()`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-retained-map-whole-clone-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nserde_json::to_value(&source.assets)\nsource.assets.to_value()`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-retained-map-serde-dsl-loop-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\ndrop(populated_map)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-ordinary-drop-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin.replace("artifact store initialization job reached Drop before exact candidate handoff or terminal retained close", "artifact initializer job silently drops retained candidate"))) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-initializer-drop-refusal-removal was falsely accepted.");
  const retainedDrawingCodec = [
    "pub struct DrawingEnvelopeOwnedFieldCatalog",
    "artifact_owned_spr_edit_history_decoder",
    "DrawingSnapshotDecodeAuthority",
    "DrawingMutationDecodeAuthority",
    "OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>",
    "const DRAWING_MAXIMUM_NESTED_ITEMS: usize = 4_096",
    "const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64",
    "struct DrawingSnapshotBoundsAuthority",
    "struct DrawingFixedOwnerCensus",
    "slots: [DrawingOwnerCreditSlot; DRAWING_MAXIMUM_NESTED_ITEMS]",
    "const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16",
    "const DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY: usize = 64",
    "const DRAWING_MUTATION_ARENA_POOL_CAPACITY: usize = 4",
    "struct DrawingMutationArenaOwner",
    "struct DrawingMutationArenaPool {",
    "DrawingMutationArenaProcessState::Building",
    "#[cfg(test)]\n    fn try_new()",
    "struct DrawingMutationArenaOwnerBuilder",
    "struct DrawingMutationArenaPoolBootstrap",
    "struct DrawingMutationArenaBootstrapJob",
    "fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep",
    "DrawingMutationArenaProcessState::Inert",
    "DrawingMutationArenaBootstrapAdmission::fixed()",
    "DrawingMutationArenaBootstrapStep::Blocked",
    "pub fn request_drawing_mutation_arena_pool() {",
    "DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true",
    "DrawingMutationArenaProcessState::Inert",
    "}",
    "pub fn drawing_mutation_arena_pool_fault",
    "fn borrow_drawing_mutation_arena() {",
    "request_drawing_mutation_arena_pool()",
    "DrawingMutationArenaBorrowError::NotReady",
    "}",
    "//#endregion",
    "DrawingStoreInitializationPhase::InitializeArena",
    "DrawingStoreInitializationPhase::InitializeArena => {",
    "match self.arena_bootstrap_job.step(cx)",
    "if let Some(undo) = self.source_undo {",
    "self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::CloseSourceUndo)?;",
    "DrawingMutationArenaProcessTransition::Retire",
    "drawing_mutation_arena_pool_fault",
    "borrow_drawing_mutation_arena_from",
    "drawing-store.mutation-arena-pool-saturated",
    "drawing-store.mutation-arena-stale-generation",
    "owner.terminal_is_empty()",
    "arena_return_phase: u8",
    "Ok(Some(false)) => return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })",
    "reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY)",
    "pages.try_reserve_exact(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY)",
    "page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES)",
    "total.checked_add(page.capacity())",
    "fn write_overlay_string",
    "target.capacity() < source.len()",
    "struct DrawingMutationOverlayPatch",
    "drawing-store.mutation-overlay-owner-changed",
    "struct DrawingAssetBoundsCursor",
    "assets.range::<str, _>",
    "drawing-store.preflight-depth-capacity",
    "drawing-store.preflight-item-capacity",
    "drawing-store.preflight-byte-capacity",
    "struct DrawingLayerCloneAuthority",
    "self.bounds.step(source, cx)",
    "layer.step(source.layers.get(self.index)",
    "target.children.push(Self::skeleton(child)?)",
    "struct DrawingStoreInitializationAuthority",
    "struct DrawingSemanticDigestCredit",
    "semantic: Option<semio_framework_hash::Sha256>",
    'digest.observe(b"drawing.semantic.sha256")',
    "struct DrawingFillDigestAuthority",
    "struct DrawingStrokeDigestAuthority",
    "struct DrawingPathSegmentDigestAuthority",
    "struct DrawingLayerVariantDigestAuthority",
    "struct DrawingLayerDigestAuthority",
    "struct DrawingMutationDigestAuthority",
    "struct DrawingMutationAggregateReservation",
    "mutation_source_items",
    "mutation_derived_items",
    "duplicate_candidate_items",
    "source_owner_bytes",
    "derived_owner_bytes",
    "DRAWING_MUTATION_AGGREGATE_ITEMS",
    "DRAWING_MUTATION_AGGREGATE_BYTES",
    "DRAWING_MUTATION_RETAINED_PAGE_ITEMS",
    "DRAWING_MUTATION_RETAINED_PAGE_BYTES",
    "drawing-store.mutation-aggregate-item-capacity",
    "drawing-store.mutation-aggregate-byte-capacity",
    "struct DrawingMutationCandidateAuthority",
    "drawing-store.mutation-candidate-stale-authority",
    "drawing-store.mutation-candidate-cancelled",
    "struct DrawingDuplicateRewriteAuthority",
    "material: [u8; DRAWING_DUPLICATE_MATERIAL_BYTES]",
    "self.hash_cursor + DRAWING_OWNED_FIELD_BYTES",
    'update(b"semio.drawing.duplicate-id.v1")',
    "update(&(self.id_len as u64).to_be_bytes())",
    "update(&(self.name_len as u64).to_be_bytes())",
    "pending_name: std::mem::ManuallyDrop<Option<String>>",
    "drawing-store.duplicate-destination-capacity",
    "DrawingMutationCandidatePhase::PreflightSource",
    "DrawingMutationCandidatePhase::PreflightMutation",
    "struct DrawingContainerRebuildAuthority",
    "const DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY",
    "enum DrawingContainerRebuildMove",
    "rebuild.rollback_step()?",
    "DrawingContainerRebuildRole::CloseSourceUndo",
    "source_undo: Option<DrawingContainerSourceUndo>",
    "source.capacity() < output_capacity",
    'self.source.as_mut().ok_or("drawing-store.container-source")?.push(value)',
    "DrawingStoreInitializationPhase::ValidateEditId",
    "DrawingStoreInitializationPhase::ValidateEditMeta",
    "DrawingStoreInitializationPhase::PrepareApplied",
    "DrawingStoreInitializationPhase::PrepareRedo",
    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawingSnapshot, DrawingMutation> for DrawingStoreInitializationAuthority",
    "DrawingStoreInitializationPhase::ValidateEditPair",
    "DrawingStoreInitializationPhase::SeedHistory",
    "DrawingStoreInitializationPhase::BuildCandidate",
    "ArtifactStore::from_initialized_runtime_with_owners",
    "ArtifactStoreInitializationOwnerCatalog::try_new()",
    "ArtifactStoreInitializationRuntime::new_with_owner_catalog",
    "DrawingStoreInitializationPhase::MoveInitialOwner",
    "self.generation.0.checked_add(1)",
    "DrawingRetirementOwner::Layer(value)",
    "DrawingRetirementOwner::Fill(fill)",
    "DrawingRetirementOwner::Stroke(stroke)",
    "DrawingRetirementOwner::Segments(values)",
    "DrawingRetirementOwner::Stops(values)",
    "DrawingRetirementOwner::Points(values)",
    "DrawingRetirementOwner::AssetEntry",
    "FillStyle::Solid { color }",
    "FillStyle::LinearGradient { x1, y1, x2, y2, stops }",
    "FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops }",
    "stop.offset",
    "stop.color",
    "value.color[(self.phase - 1) as usize]",
    "value.width",
    "observe_owned_string(digest, 246, &value.cap",
    "observe_owned_string(digest, 247, &value.join",
    "value.dash.is_some()",
    "base.visible",
    "base.locked",
    "base.opacity",
    "observe_owned_string(digest, 106, &base.blend_mode",
    "base.transform.rotation",
    "observe_owned_string(digest, 340, &value.shape_kind",
    "value.rect.is_some()",
    "value.ellipse.is_some()",
    "value.circle.is_some()",
    "value.line.is_some()",
    "value.polygon.is_some()",
    "PathSegment::Move",
    "PathSegment::Line",
    "PathSegment::Quad",
    "PathSegment::Cubic",
    "PathSegment::Arc",
    "PathSegment::Close",
    "observe_owned_string(digest, 382, &value.content",
    "observe_owned_string(digest, 390, &value.image_key",
    "observe_owned_string(digest, 410, &value.operation",
    "value.children.len()",
    "observe_owned_string(digest, 420, &value.source_key",
    "value.params.threshold",
    "Self::variant(mutation)",
    "value.visible",
    "value.locked",
    "value.opacity",
    "observe_owned_string(digest, 3, &value.blend_mode",
    "observe_owned_string(digest, 3, &value.new_name",
    "value.transform.rotation",
    "value.fill.as_ref()",
    "value.stroke.as_ref()",
    "observe_owned_string(digest, 3, &value.boolean_operation",
    "value.params.simplify_epsilon",
    "value.parent_id.is_some()",
    "value.index",
    "value.layer",
    "SetLayerVisible(payload)",
    "SetLayerLocked(payload)",
    "SetLayerOpacity(payload)",
    "SetLayerBlendMode(payload)",
    "RenameLayer(payload)",
    "UpdateLayerTransform(payload)",
    "ReplaceLayerFill(payload)",
    "ReplaceLayerStroke(payload)",
    "SetLayerBooleanOperation(payload)",
    "UpdateLayerTraceParams(payload)",
    "CreateLayer(payload)",
    "DuplicateLayer(payload)",
    "DeleteLayer(payload)",
    "ReorderLayer(payload)",
    "retained_drawing_mutation_candidate_covers_all_fourteen_variants_and_returns_exact_owners",
    "retained_drawing_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty",
    "retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner",
    "retained_drawing_schema_digest_distinguishes_every_nested_semantic_field",
    "retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback",
    "retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision",
    "retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba",
    "retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner",
    "retained_drawing_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid",
    "retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot",
    "retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots",
    "retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant",
    "retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close",
    "retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool",
    "retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close",
    "retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation",
    "retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed",
  ].join("\n");
  const retainedDrawingEditor = [
    "drawing_document_store_initialization_job(envelope, operation, generation)",
    "impl Default for DrawingPlayApp {",
    "request_drawing_mutation_arena_pool()",
    "}",
    "impl ArtifactEditor for DrawingPlayApp",
    "arena_boot_fault: Option<&'static str>",
    "drawing_mutation_arena_pool_fault",
    "drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed",
    "drawing_live_envelope_cancel_closes_retained_pages_without_publication",
    "drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate",
    "drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle",
    '"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer',
    "pub struct DrawingEnvelopeLoadHandle",
    "source: &js_sys::Uint8Array",
    "begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)",
    "admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)",
    "seal_artifact_envelope_ingress(handle.runtime_handle())",
    "app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)",
    "advance_artifact_envelope_load(handle.runtime_handle())",
    "acknowledge_artifact_store_replacement(handle.runtime_handle())",
    "cancel_artifact_envelope_load(handle.runtime_handle())",
  ].join("\n");
  if (!toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Drawing-envelope-route was falsely rejected.");
  const retainedDrawingProductionEditor = retainedDrawingEditor
    .replace("drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed", "")
    .replace("drawing_live_envelope_cancel_closes_retained_pages_without_publication", "")
    .replace("drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate", "")
    .replace("drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle", "")
    .replace('"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer', "");
  if (!toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingProductionEditor, retainedWriterPlugin, retainedDrawingEditor)) {
    throw new Error("[verify interactivity tool-jobs] self-test production-stripped Drawing editor with raw cfg(test) law witnesses was falsely rejected.");
  }
  const retainedDrawingSemanticFields = [
    "FillStyle::Solid { color }",
    "FillStyle::LinearGradient { x1, y1, x2, y2, stops }",
    "FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops }",
    "stop.offset",
    "stop.color",
    "value.color[(self.phase - 1) as usize]",
    "value.width",
    "observe_owned_string(digest, 246, &value.cap",
    "observe_owned_string(digest, 247, &value.join",
    "value.dash.is_some()",
    "base.visible",
    "base.locked",
    "base.opacity",
    "observe_owned_string(digest, 106, &base.blend_mode",
    "base.transform.rotation",
    "observe_owned_string(digest, 340, &value.shape_kind",
    "value.rect.is_some()",
    "value.ellipse.is_some()",
    "value.circle.is_some()",
    "value.line.is_some()",
    "value.polygon.is_some()",
    "PathSegment::Move",
    "PathSegment::Line",
    "PathSegment::Quad",
    "PathSegment::Cubic",
    "PathSegment::Arc",
    "PathSegment::Close",
    "observe_owned_string(digest, 382, &value.content",
    "observe_owned_string(digest, 390, &value.image_key",
    "observe_owned_string(digest, 410, &value.operation",
    "value.children.len()",
    "observe_owned_string(digest, 420, &value.source_key",
    "value.params.threshold",
    "Self::variant(mutation)",
    "value.visible",
    "value.locked",
    "value.opacity",
    "observe_owned_string(digest, 3, &value.blend_mode",
    "observe_owned_string(digest, 3, &value.new_name",
    "value.transform.rotation",
    "value.fill.as_ref()",
    "value.stroke.as_ref()",
    "observe_owned_string(digest, 3, &value.boolean_operation",
    "value.params.simplify_epsilon",
    "value.parent_id.is_some()",
    "value.index",
    "value.layer",
  ];
  for (const field of retainedDrawingSemanticFields) {
    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace(field, "omitted-semantic-field"), retainedDrawingEditor, retainedWriterPlugin)) {
      throw new Error(`[verify interactivity tool-jobs] self-test Drawing-semantic-digest-omission-${field} was falsely accepted.`);
    }
  }
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("semantic: Option<semio_framework_hash::Sha256>", "semantic: u64"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-non-collision-resistant-digest was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace('digest.observe(b"drawing.semantic.sha256")', "digest.observe(&field)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-unsealed-semantic-digest was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationAggregateReservation", "struct DrawingPerAllocationGuess"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-missing-aggregate-reservation was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingFixedOwnerCensus", "struct DrawingResizableOwnerGuess"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fixed-owner-census-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16", "let overlay_pages = Vec::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fixed-overlay-page-arena-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES)", "page.reserve(source.len())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-overlay-page-not-preadmitted was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("total.checked_add(page.capacity())", "total.checked_add(page.len())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-overlay-allocator-capacity-not-reconciled was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("fn write_overlay_string", "fn clone_overlay_string"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-overlay-owner-not-moved was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY)", "let reverse = Vec::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fixed-container-owner-catalog-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationArenaPool {", "struct PerCandidateArenaAllocation {"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-process-arena-pool-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingMutationArenaProcessState::Building", "DrawingMutationArenaOwner::try_new()?"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-arena-allocation-before-process-admission was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nDrawingMutationArenaOwner::try_new()?;`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-question-mark-partial-bootstrap-drop was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("#[cfg(test)]\n    fn try_new()", "fn try_new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-synchronous-bootstrap-loop-was-production-reachable was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("borrow_drawing_mutation_arena_from", "allocate_drawing_mutation_arena_per_candidate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-candidate-did-not-borrow-fixed-arena was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("owner.terminal_is_empty()", "true"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-nonterminal-arena-return-was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("arena_return_phase: u8", "return_whole_arena_bundle: bool"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-arena-return-was-not-one-root-per-grant was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-arena-stale-generation", "accept-stale-arena-return"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-arena-ABA-generation-check-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nbase.name.try_reserve_exact(suffix.len());`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-post-admission-duplicate-name-reserve was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("pending_name: std::mem::ManuallyDrop<Option<String>>", "pending_name: String"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-name-owner-not-retained was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("source.capacity() < output_capacity", "false"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-container-owner-capacity-not-preflighted was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace('self.source.as_mut().ok_or("drawing-store.container-source")?.push(value)', 'self.output.as_mut().unwrap().push(value)'), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-container-scratch-owner-published-instead-of-returned was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba", "drawing-string-only-pool-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-process-pool-handback-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner", "drawing-string-only-name-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-duplicate-name-owner-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("request_drawing_mutation_arena_pool()", "initialize_drawing_mutation_arena_pool()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-app-default-advanced-bootstrap was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true", "Vec::<u8>::new().try_reserve_exact(64); DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-request-path-allocated-before-governed-turn was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("fn borrow_drawing_mutation_arena() {\nrequest_drawing_mutation_arena_pool()", "fn borrow_drawing_mutation_arena() {\nbootstrap.step(cx); request_drawing_mutation_arena_pool()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-borrow-path-advanced-bootstrap was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationOverlayPatch", "struct DrawingWholeCandidateRebuild"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-overlay-patch-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("mutation_derived_items", "semantic_totals_times_copies"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-derived-owner-credit-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nfn exact_for_test() {}`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fabricated-boundary-authority was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nsource.assets.iter().nth(index)`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-asset-rescan-cursor was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore.replace("id_digest: [u8; 32]", "id: String"), retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-shared-history-id-clone-owner was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingDuplicateRewriteAuthority", "fn rewrite_duplicate_in_one_step"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-staged-duplicate-authority-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace('update(b"semio.drawing.duplicate-id.v1")', "update(&material_len.to_be_bytes())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-domain-frame-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("update(&(self.id_len as u64).to_be_bytes())", "update(&(self.material_len as u64).to_be_bytes())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-id-length-frame-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("update(&(self.name_len as u64).to_be_bytes())", "update(&[])"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-name-length-frame-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-candidate-cancelled", "close-only-cancel"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-candidate-cancellation-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-aggregate-item-capacity", "unchecked-aggregate-items"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-aggregate-item-plus-one was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-aggregate-byte-capacity", "unchecked-aggregate-bytes"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-aggregate-byte-plus-one was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DRAWING_MUTATION_RETAINED_PAGE_ITEMS", "uncredited-retained-pages"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-page-item-credit-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DRAWING_MUTATION_RETAINED_PAGE_BYTES", "uncredited-retained-page-bytes"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-page-byte-credit-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingMutationCandidatePhase::PreflightSource", "DrawingMutationCandidatePhase::Clone"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-clone-before-source-preflight was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingMutationCandidatePhase::PreflightMutation", "DrawingMutationCandidatePhase::Clone"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-clone-before-mutation-preflight was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::ValidateEditMeta", "mutation_meta.iter().any"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-metadata-scan was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::PrepareApplied", "DrawingStoreInitializationPhase::CommitApplied"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-applied-id-actor-multi-clone-grant was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::PrepareRedo", "DrawingStoreInitializationPhase::CommitRedo"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-redo-id-multi-clone-grant was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_schema_digest_distinguishes_every_nested_semantic_field", "drawing-string-name-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-semantic-authority-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback", "drawing-aggregate-string-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-real-aggregate-saturation-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision", "drawing-duplicate-name-only-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-split-boundary-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid", "drawing-only-cancels-before-start"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-stage-cancel-stale-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("rebuild.rollback_step()?", "rebuild.close_forward_step()?"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-active-rebuild-close-did-not-rollback was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::CloseSourceUndo)?;", "self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::Destination)?;"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-source-undo-close-authority-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot", "drawing-only-cancels-before-owner-move"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-every-rebuild-move-fault-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots", "drawing-does-not-restore-source-handoff"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-reorder-source-undo-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationArenaOwnerBuilder", "fn try_new_arena_owner()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-bootstrap-owner-builder-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationArenaPoolBootstrap", "fn build_pool_with_question_mark()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-bootstrap-pool-owner-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("match self.arena_bootstrap_job.step(cx)", "match Ok::<bool, &'static str>(true)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-bootstrap-not-driven-by-governed-store-turn was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant", "drawing-bootstrap-only-tests-success"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-allocation-fault-matrix-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close", "drawing-bootstrap-drops-completed-bundles"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-bundle-fault-matrix-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool", "drawing-bootstrap-loops-to-completion"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-governed-progress-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close", "drawing-bootstrap-skips-aggregate-plus-one"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-aggregate-rejection-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("drawing_mutation_arena_pool_fault", "ignore_arena_boot_fault"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-app-bootstrap-fault-was-not-surfaced was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation", "drawing-default-allocates-on-second-app"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-default-borrow-no-allocation-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed", "drawing-bootstrap-ignores-governed-cancellation"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-governed-bootstrap-adversarial-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nmutation_meta.iter().any(|meta| true)`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-resurrected-whole-metadata-scan was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("artifact_owned_spr_edit_history_decoder", "artifact_bounded_history_entry_decoder"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-domain-only-whole-edit-decoder was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64", "let depth = Vec::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-resizable-recursion-stack was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.preflight-byte-capacity", "unchecked_nested_bytes"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-nested-byte-preflight-hole was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("target.children.push(Self::skeleton(child)?)", "target.children.push(child.clone())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-recursive-layer-clone was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingRetirementOwner::AssetEntry", "drop(asset)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-asset-owner-drop was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("ReorderLayer(payload)", "drop(payload)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-mutation-catalog-hole was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\noperation.encode_op()`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-operation-encode-before-credit was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nlet next = snapshot.clone();`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-snapshot-clone-replay was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nserde_json::from_value(value)`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-serde-mutation-reconstruction was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::ValidateEditId", "skip_final_edit_id"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-single-final-edit-id-preflight-hole was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("ArtifactStoreInitializationOwnerCatalog::try_new()", "ArtifactStoreInitializationRuntime::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-store-owner-catalog-not-preadmitted was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::MoveInitialOwner", "DrawingStoreInitializationPhase::CloneInitial"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-initial-owner-clone-resurrected was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationCandidateAuthority", "struct SynchronousDrawDiffApply"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-mutation-candidate-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner", "drawing_string_predicate_only"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-authority-close-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate", "drawing_skips_final_edit_id"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-final-edit-id-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle", "drawing-close-only-after-success"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-staged-cancel-stale-ack-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace('"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer', '"forwards": []'), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-populated-history-route-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("source: &js_sys::Uint8Array", "source: &[u8]"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-post-lift-dynamic-page was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("acknowledge_artifact_store_replacement(handle.runtime_handle())", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-completion-without-exact-ack was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, `${retainedDrawingEditor}\nenvelope_json: Option<String>`, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-buffer-constructor-bypass was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin.replace("self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))", "drop(job)"))) throw new Error("[verify interactivity tool-jobs] self-test Drawing-false-terminal-initializer-drop was falsely accepted.");
  const synchronousPeerRoster = "struct PeerPresenceRoot; struct VcsArtifactApp { peer_presence: Arc<PeerPresenceRoot> } impl PluginApp for VcsArtifactApp { async fn adopt_presence(&mut self, peers: &[PresencePeer]) { self.peer_presence = Arc::new(PeerPresenceRoot::from_peers(peers)); } } async fn dispatch_typed_command_inner() { let presence_peers = self.presence_store.peers().await.into_iter().map(|(actor, presence)| (actor.to_string(), presence.clone())).collect(); } let mut decoded: Vec<protocol::PresencePeer> = Vec::with_capacity(peers.len());";
  if (toolJobPeerInteractionRootsExact(synchronousPeerRoster, synchronousPeerRoster, synchronousPeerRoster)) throw new Error("[verify interactivity tool-jobs] self-test synchronous-whole-peer-roster-publication was falsely accepted.");
  const preadmissionRosterDecode = "pub struct PresenceRosterWire; async fn plugin_exchange(commands: &[Vec<u8>]) { let command = decode_app_command(bytes).await; reserve_presence_ingress(seq); }";
  if (toolJobPeerInteractionRootsExact(preadmissionRosterDecode, preadmissionRosterDecode, preadmissionRosterDecode)) throw new Error("[verify interactivity tool-jobs] self-test preadmission-whole-presence-decode was falsely accepted.");
  const silentMalformedPeer = "if let Ok(peer) = decoded { publish(peer); }";
  if (toolJobPeerInteractionRootsExact(silentMalformedPeer, silentMalformedPeer, silentMalformedPeer)) throw new Error("[verify interactivity tool-jobs] self-test silent-malformed-presence-peer was falsely accepted.");
  const partialTypedCommit = "presence_store.publish_peer_commit(typed); validate_generation(); peer_presence = root;";
  if (toolJobPeerInteractionRootsExact(partialTypedCommit, partialTypedCommit, partialTypedCommit)) throw new Error("[verify interactivity tool-jobs] self-test presence-typed-partial-commit was falsely accepted.");
  const cloneableRosterOwner = "#[derive(Clone, Debug, PartialEq)]\npub struct PresenceRosterWire;";
  if (toolJobPeerInteractionRootsExact(cloneableRosterOwner, cloneableRosterOwner, cloneableRosterOwner)) throw new Error("[verify interactivity tool-jobs] self-test cloneable-whole-roster-owner was falsely accepted.");
  const stalePresenceCommit = "fn publish() { debug_assert_eq!(generation, live); peer_presence = root; }";
  if (toolJobPeerInteractionRootsExact(stalePresenceCommit, stalePresenceCommit, stalePresenceCommit)) throw new Error("[verify interactivity tool-jobs] self-test debug-only-presence-generation-check was falsely accepted.");
  const legacyPresenceMutation = "pub async fn adopt_peer(&mut self) {} pub async fn remove_peer(&mut self) {} pub async fn expire_peers(&mut self) {}";
  if (toolJobPeerInteractionRootsExact(legacyPresenceMutation, legacyPresenceMutation, legacyPresenceMutation)) throw new Error("[verify interactivity tool-jobs] self-test legacy-whole-presence-mutation-escape was falsely accepted.");
  const wholeVectorPages = "pub struct FixedCommandPage; pages: std::collections::VecDeque<Vec<u8>>; pub instance: PluginInstanceId;";
  if (toolJobPagedIngressExact(wholeVectorPages, wholeVectorPages, wholeVectorPages, wholeVectorPages, wholeVectorPages, wholeVectorPages, wholeVectorPages, wholeVectorPages, wholeVectorPages)) throw new Error("[verify interactivity tool-jobs] self-test paged-ingress-whole-vector-owner was falsely accepted.");
  const onePageOnly = "CommandIngressOwner::GenericAssembly page_count == 1";
  if (toolJobPagedIngressExact(onePageOnly, onePageOnly, onePageOnly, onePageOnly, onePageOnly, onePageOnly, onePageOnly, onePageOnly, onePageOnly)) throw new Error("[verify interactivity tool-jobs] self-test generic-command-one-page-only was falsely accepted.");
  const dynamicWitPage = "record command-page-block { word-0: u64 } bytes: list<u8> block-63: command-page-block";
  if (toolJobPagedIngressExact(dynamicWitPage, dynamicWitPage, dynamicWitPage, dynamicWitPage, dynamicWitPage, dynamicWitPage, dynamicWitPage, dynamicWitPage, dynamicWitPage)) throw new Error("[verify interactivity tool-jobs] self-test dynamic-list-command-page-lift was falsely accepted.");
  const unretainedCaller = "CommandBatchDriver close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)";
  if (toolJobPagedIngressExact(unretainedCaller, unretainedCaller, unretainedCaller, unretainedCaller, unretainedCaller, unretainedCaller, unretainedCaller, unretainedCaller, unretainedCaller)) throw new Error("[verify interactivity tool-jobs] self-test command-driver-without-terminal-close-registry was falsely accepted.");
  const faultAsSuccess = "presence_terminal CommandComplete";
  if (toolJobPagedIngressExact(faultAsSuccess, faultAsSuccess, faultAsSuccess, faultAsSuccess, faultAsSuccess, faultAsSuccess, faultAsSuccess, faultAsSuccess, faultAsSuccess)) throw new Error("[verify interactivity tool-jobs] self-test presence-terminal-fault-as-success was falsely accepted.");
  const aggregateGenericDecode = "pub struct FixedCommandPage; pub struct PagedCommandReader; fn plugin_exchange() { encoded.contiguous_if_single_page(); protocol::decode_app_command(bytes); }";
  if (toolJobPagedIngressExact(aggregateGenericDecode, aggregateGenericDecode, aggregateGenericDecode, aggregateGenericDecode, aggregateGenericDecode, aggregateGenericDecode, aggregateGenericDecode, aggregateGenericDecode, aggregateGenericDecode)) throw new Error("[verify interactivity tool-jobs] self-test aggregate-generic-command-decode was falsely accepted.");
  const malformedOwnerDrop = "struct PagedAppCommandDecodeCursor; fn step() { let command = read_bounded_bytes(4096)?; drop(command); }";
  if (toolJobPagedIngressExact(malformedOwnerDrop, malformedOwnerDrop, malformedOwnerDrop, malformedOwnerDrop, malformedOwnerDrop, malformedOwnerDrop, malformedOwnerDrop, malformedOwnerDrop, malformedOwnerDrop)) throw new Error("[verify interactivity tool-jobs] self-test malformed-generic-owner-drop was falsely accepted.");
  const closeAsSuccess = "command.cancel(); command_terminal_fault = None; CommandComplete";
  if (toolJobPagedIngressExact(closeAsSuccess, closeAsSuccess, closeAsSuccess, closeAsSuccess, closeAsSuccess, closeAsSuccess, closeAsSuccess, closeAsSuccess, closeAsSuccess)) throw new Error("[verify interactivity tool-jobs] self-test cancelled-generic-command-as-success was falsely accepted.");
  const noInterruptedAssemblyClose = "CommandIngressOwner::GenericAssembly; close_instances.contains(instance); retained = None;";
  if (toolJobPagedIngressExact(noInterruptedAssemblyClose, noInterruptedAssemblyClose, noInterruptedAssemblyClose, noInterruptedAssemblyClose, noInterruptedAssemblyClose, noInterruptedAssemblyClose, noInterruptedAssemblyClose, noInterruptedAssemblyClose, noInterruptedAssemblyClose)) throw new Error("[verify interactivity tool-jobs] self-test interrupted-generic-assembly-drop was falsely accepted.");
  const nestedBatchOwners = "pub struct CommandEnvelopeSet; struct CommandBatch { commands: std::collections::VecDeque<CommandEnvelope> } pub struct CommandDriverRegistry; pub fn prepare_suspend() {} pub struct RejectedCommandBuildRegistry;";
  if (toolJobPagedIngressExact(nestedBatchOwners, nestedBatchOwners, nestedBatchOwners, nestedBatchOwners, nestedBatchOwners, nestedBatchOwners, nestedBatchOwners, nestedBatchOwners, nestedBatchOwners)) throw new Error("[verify interactivity tool-jobs] self-test retained-batch-nested-command-owners was falsely accepted.");
  const unretainedBuildRejection = "pub struct FixedCommandPage; struct CommandBatchEntry; pages: std::collections::VecDeque<FixedCommandPage>; pub struct CommandDriverRegistry; pub fn prepare_suspend() {} map_err(|(fault, _owner)| fault)";
  if (toolJobPagedIngressExact(unretainedBuildRejection, unretainedBuildRejection, unretainedBuildRejection, unretainedBuildRejection, unretainedBuildRejection, unretainedBuildRejection, unretainedBuildRejection, unretainedBuildRejection, unretainedBuildRejection)) throw new Error("[verify interactivity tool-jobs] self-test rejected-command-build-owner-drop was falsely accepted.");
  const retainedAppFrame = "PendingResponsePage response: Option<Result<store::AppFrame, Fault>> RejectedCommandBuildRegistry<1>";
  if (toolJobPagedIngressExact(retainedAppFrame, retainedAppFrame, retainedAppFrame, retainedAppFrame, retainedAppFrame, retainedAppFrame, retainedAppFrame, retainedAppFrame, retainedAppFrame)) throw new Error("[verify interactivity tool-jobs] self-test MCP-retained-deep-AppFrame-response was falsely accepted.");
  const noSuspensionAuthority = "pub struct CommandDriverRegistry; struct CommandBatchEntry; pages: std::collections::VecDeque<FixedCommandPage>; pub struct RejectedCommandBuildRegistry; run_turn().await";
  if (toolJobPagedIngressExact(noSuspensionAuthority, noSuspensionAuthority, noSuspensionAuthority, noSuspensionAuthority, noSuspensionAuthority, noSuspensionAuthority, noSuspensionAuthority, noSuspensionAuthority, noSuspensionAuthority)) throw new Error("[verify interactivity tool-jobs] self-test caller-await-without-suspended-owner was falsely accepted.");
  const noQueuedShutdown = "retained_command_closes RejectedCommandBuildRegistry<1>";
  if (toolJobPagedIngressExact(noQueuedShutdown, noQueuedShutdown, noQueuedShutdown, noQueuedShutdown, noQueuedShutdown, noQueuedShutdown, noQueuedShutdown, noQueuedShutdown, noQueuedShutdown)) throw new Error("[verify interactivity tool-jobs] self-test WGPU-command-owner-without-queued-maintenance was falsely accepted.");
  const resizableBlockingCallerQueue = "const KERNEL_REQUEST_QUEUE_CAPACITY: usize = 64; struct KernelRequestQueue { pending: Mutex<std::collections::VecDeque<(KernelRequest, Arc<ResponseSlot>)>> } KernelRequest::CloseRejectedCommandBuild";
  if (toolJobPagedIngressExact(resizableBlockingCallerQueue, resizableBlockingCallerQueue, resizableBlockingCallerQueue, resizableBlockingCallerQueue, resizableBlockingCallerQueue, resizableBlockingCallerQueue, resizableBlockingCallerQueue, resizableBlockingCallerQueue, resizableBlockingCallerQueue)) throw new Error("[verify interactivity tool-jobs] self-test WGPU-resizable-blocking-command-request-queue was falsely accepted.");
  const noQueueCommandCredits = "const KERNEL_REQUEST_QUEUE_CAPACITY: usize = 64; slots: [Option<(KernelRequest, Arc<ResponseSlot>)>; KERNEL_REQUEST_QUEUE_CAPACITY]; self.state.try_lock(); fn shutdown_step(&self, maximum_bytes: usize) {} yield_kernel_maintenance_turn";
  if (toolJobPagedIngressExact(noQueueCommandCredits, noQueueCommandCredits, noQueueCommandCredits, noQueueCommandCredits, noQueueCommandCredits, noQueueCommandCredits, noQueueCommandCredits, noQueueCommandCredits, noQueueCommandCredits)) throw new Error("[verify interactivity tool-jobs] self-test WGPU-fixed-request-queue-without-command-byte-page-credit was falsely accepted.");
  const noQueueShutdownCursor = "const KERNEL_REQUEST_QUEUE_CAPACITY: usize = 64; slots: [Option<(KernelRequest, Arc<ResponseSlot>)>; KERNEL_REQUEST_QUEUE_CAPACITY]; self.state.try_lock(); command_pages; command_bytes; yield_kernel_maintenance_turn";
  if (toolJobPagedIngressExact(noQueueShutdownCursor, noQueueShutdownCursor, noQueueShutdownCursor, noQueueShutdownCursor, noQueueShutdownCursor, noQueueShutdownCursor, noQueueShutdownCursor, noQueueShutdownCursor, noQueueShutdownCursor)) throw new Error("[verify interactivity tool-jobs] self-test WGPU-fixed-request-queue-without-bounded-shutdown-cursor was falsely accepted.");
  const deepCreateRequest = "KernelRequest::CreateApp { wasm_path: PathBuf, plugin_id: String, app_id: String }";
  if (toolJobPagedIngressExact(deepCreateRequest, deepCreateRequest, deepCreateRequest, deepCreateRequest, deepCreateRequest, deepCreateRequest, deepCreateRequest, deepCreateRequest, deepCreateRequest)) throw new Error("[verify interactivity tool-jobs] self-test WGPU-create-request-without-owned-close-cursor was falsely accepted.");
  const wholeEventBatch = "KernelRequest::Exchange { instance: u32, events: Vec<Event> }";
  if (toolJobPagedIngressExact(wholeEventBatch, wholeEventBatch, wholeEventBatch, wholeEventBatch, wholeEventBatch, wholeEventBatch, wholeEventBatch, wholeEventBatch, wholeEventBatch)) throw new Error("[verify interactivity tool-jobs] self-test WGPU-whole-event-batch-request-owner was falsely accepted.");
  const rejectedEventsDrop = "struct QueuedKernelEvent; try_from_events(events).map_err(|owner| drop(owner))";
  if (toolJobPagedIngressExact(rejectedEventsDrop, rejectedEventsDrop, rejectedEventsDrop, rejectedEventsDrop, rejectedEventsDrop, rejectedEventsDrop, rejectedEventsDrop, rejectedEventsDrop, rejectedEventsDrop)) throw new Error("[verify interactivity tool-jobs] self-test WGPU-rejected-event-owner-drop was falsely accepted.");
  const fireAndForgetDestroy = "fn destroy_app(&self, instance: u32) { let _rejected_close = self.queue.try_push(KernelRequest::DestroyApp { instance }, Arc::new(ResponseSlot::default()), None); }";
  if (toolJobPagedIngressExact(fireAndForgetDestroy, fireAndForgetDestroy, fireAndForgetDestroy, fireAndForgetDestroy, fireAndForgetDestroy, fireAndForgetDestroy, fireAndForgetDestroy, fireAndForgetDestroy, fireAndForgetDestroy)) throw new Error("[verify interactivity tool-jobs] self-test WGPU-fire-and-forget-destroy-owner-loss was falsely accepted.");
  const unpollableDestroy = "struct KernelCloseSubmissionRegistry; slots: Mutex<[Option<(u32, u64, Arc<KernelCloseSubmission>)>; KERNEL_CLOSE_SUBMISSION_CAPACITY]>; fn destroy_app() { owner.finish(KernelCloseStatus::Complete); }";
  if (toolJobPagedIngressExact(unpollableDestroy, unpollableDestroy, unpollableDestroy, unpollableDestroy, unpollableDestroy, unpollableDestroy, unpollableDestroy, unpollableDestroy, unpollableDestroy)) throw new Error("[verify interactivity tool-jobs] self-test WGPU-close-registry-without-pollable-exact-owner-handle was falsely accepted.");
  const shutdownDropsClose = "pub(crate) struct KernelCloseHandle; pub(crate) fn begin_destroy_app(&self, instance: u32) -> KernelCloseHandle; KernelRequest::DestroyApp { owner: self.clone() }; owner.finish(KernelCloseStatus::Complete); fn shutdown_step() { drop(owner); }";
  if (toolJobPagedIngressExact(shutdownDropsClose, shutdownDropsClose, shutdownDropsClose, shutdownDropsClose, shutdownDropsClose, shutdownDropsClose, shutdownDropsClose, shutdownDropsClose, shutdownDropsClose)) throw new Error("[verify interactivity tool-jobs] self-test WGPU-queue-shutdown-dropped-close-completion-owner was falsely accepted.");
  const universalJob = `
pub trait InteractiveJob: Send { fn step(&mut self); fn begin_close(&mut self); fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize); fn terminal_is_empty(&self) -> bool; }
struct ExactJob;
impl InteractiveJob for ExactJob { fn step(&mut self) {} fn begin_close(&mut self) {} fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) {} fn terminal_is_empty(&self) -> bool { true } }
struct WorkerJobAuthority { preadmitted_fault: Option<RetainedJobPayload> }
fn admission() { preadmitted_static_payload(&payload_ledger, stream, bytes, source); authority.preadmitted_fault.take().expect("worker panic retains its pre-admitted terminal fault page"); }
fn worker_panic_and_quiet_wake_publish_one_durable_terminal_intent() { assert_eq!(returned_fault_pointer, preadmitted_fault_pointer); }
fn checked_out_and_worker_begin_close_transitions_report_exact_zero_release() { assert_eq!(batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES), WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 }); assert_eq!(batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES), WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
fn child_counter() { "child begin-close transfers control without claiming an owner release"; }
pub struct WorkerJobSessionAdmissionRejected { fault_source: ManuallyDrop<Option<JobPayloadPageSource>> }
impl WorkerJobSession { pub fn try_step_on_caller(&self) {} }
impl BatchJobSession { pub fn try_new(job: J, params: BatchJobParams) -> Result<Self, WorkerJobSessionAdmissionRejected<J>> {} }
`;
  const universalNative = `
const NATIVE_IO_PATH_CAPACITY: usize = 256;
pub struct NativePathSet { entries: std::mem::ManuallyDrop<[Option<PathBuf>; NATIVE_IO_PATH_CAPACITY]> } pub struct NativeModifiedSet { entries: std::mem::ManuallyDrop<[Option<(PathBuf, std::time::SystemTime)>; NATIVE_IO_PATH_CAPACITY]> }
enum NativeIoState { ReadingModified { paths: NativePathSet, modified: NativeModifiedSet }, ClosingScanFault, ClosingModifiedFault }
impl NativeIoJob { pub fn retained_request_backing_identity(&self) -> Option<*const u8> {} }
fn path_set_max_plus_one_identity_zero_grant_and_job_close_are_exact() { assert_eq!(returned.as_os_str().as_encoded_bytes().as_ptr(), plus_one_pointer); }
`;
  const universalWgpu = `
const RENDERER_IO_SESSION_SLOTS: usize = semio_framework_job::WORKER_JOB_SESSION_SLOTS + 1;
struct Slot { generation: std::sync::atomic::AtomicU64, cancel_requested: std::sync::atomic::AtomicBool }
struct Prepared { session: Option<semio_framework_job::BatchJobSession<ui_wgpu::wgpu::PreparedRenderJob>>, rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ui_wgpu::wgpu::PreparedRenderJob>> }
struct FrameBuildHandle { session: Option<semio_framework_job::WorkerJobSession<ActiveFrameBuild>>, rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ActiveFrameBuild>> }
fn take_prepared() { checked_out_job_mut()?.take_packet(); }
fn renderer_io_with_node() { slot.generation.load(std::sync::atomic::Ordering::Acquire) != generation; }
fn pump_renderer_io_sessions(maximum_sessions: usize) {}
fn mounted_registry_max_plus_one_zero_pump_drop_and_generation_are_exact() { assert_eq!(returned_pointer, plus_one_request_pointer); assert_eq!(pump_renderer_io_sessions(0), 0); assert_eq!(pump_renderer_io_sessions(1), 1, "one host turn advances one mounted control opportunity"); }
impl std::future::Future for RendererIoHandle { fn poll(&mut self, cx: &mut Context) { renderer_io_with_node(); node.waker = Some(cx.waker().clone()); } }
impl Presenter { pub(crate) fn present_step(&mut self) { pump_renderer_io_sessions(1); pump_worker_job_retirements(1, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES); } }
`;
  const universalAnimate = `
pub struct PresentationEnvelopeMaterializeJob { fault_writer: std::mem::ManuallyDrop<Option<semio_framework_job::RetainedJobPayloadWriter>>, retained_nested_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>> }
impl InteractiveJob for PresentationEnvelopeMaterializeJob { fn step(&mut self) {} fn begin_close(&mut self) {} fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) {} fn terminal_is_empty(&self) -> bool { true } }
pub struct PresentationEnvelopeMaterializeHandle {
  rejected: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSessionAdmissionRejected<PresentationEnvelopeMaterializeJob>>>,
  pending: Option<semio_framework_job::WorkerJobTicket>,
  retained_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>>,
}
fn submit_materialize_presentation_projection() { WorkerJobSession::try_new(job, params); }
fn retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack() { registry.close_step(operation, generation, &pool, 0, 0); }
fn retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output() { registry.close_step(operation, generation, &pool, 0, 0); }
`;
  const universal = (job = universalJob, nativeIo = universalNative, wgpu = universalWgpu, animate = universalAnimate) => toolJobUniversalRetainedOwnershipExact(job, nativeIo, wgpu, new Map([["job.rs", job], ["native_io.rs", nativeIo], ["wgpu.rs", wgpu], ["animate.rs", animate]]));
  if (!universal()) throw new Error("[verify interactivity tool-jobs] self-test universal-retained-ownership-valid was falsely rejected.");
  if (universal(universalJob.replace("fn begin_close(&mut self) {}", "fn close_now(&mut self) {}"))) throw new Error("[verify interactivity tool-jobs] self-test universal-job-missing-mandatory-close was falsely accepted.");
  if (universal(`${universalJob}\nWorkerJobSession::new(job, params);`)) throw new Error("[verify interactivity tool-jobs] self-test universal-legacy-session-constructor was falsely accepted.");
  if (universal(`${universalJob}\nsemio_framework_job::run_to_completion(job);`)) throw new Error("[verify interactivity tool-jobs] self-test universal-legacy-terminal-drain was falsely accepted.");
  if (universal(`${universalJob}\nsemio_framework_job::drive_step(job, params);`)) throw new Error("[verify interactivity tool-jobs] self-test universal-direct-drive-bypasses-retained-session was falsely accepted.");
  if (universal(`${universalJob}\nuse semio_framework_job::{drive_step, StepBudget};\nfn bypass() { drive_step(job, params); }`)) throw new Error("[verify interactivity tool-jobs] self-test universal-imported-direct-drive-bypasses-retained-session was falsely accepted.");
  if (universal(`${universalJob}\nfn bypass() { job::drive_step(job, params); }`)) throw new Error("[verify interactivity tool-jobs] self-test universal-aliased-direct-drive-bypasses-retained-session was falsely accepted.");
  if (universal(`${universalJob}\nstruct FrameBuildHandle { runtime_in_flight: Option<Receiver<RuntimeFrameResult>> }`)) throw new Error("[verify interactivity tool-jobs] self-test universal-frame-async-receiver-bypasses-retained-session was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ActiveFrameBuild>>", "rejected: Option<String>"))) throw new Error("[verify interactivity tool-jobs] self-test universal-frame-admission-rejection-owner-missing was falsely accepted.");
  if (universal(universalJob.replace("preadmitted_static_payload(&payload_ledger", "retained_static_payload(&payload_ledger"))) throw new Error("[verify interactivity tool-jobs] self-test universal-post-work-fault-allocation was falsely accepted.");
  if (universal(universalJob.replace("assert_eq!(returned_fault_pointer, preadmitted_fault_pointer);", "assert_eq!(fault.detail.len(), 26);"))) throw new Error("[verify interactivity tool-jobs] self-test universal-preadmitted-fault-identity-proof-missing was falsely accepted.");
  if (universal(universalJob.replace("released_items: 0, released_bytes: 0 }); assert_eq!(batch.close_step", "released_items: 1, released_bytes: 0 }); assert_eq!(batch.close_step"))) throw new Error("[verify interactivity tool-jobs] self-test universal-begin-close-exact-counter-proof-missing was falsely accepted.");
  if (universal(universalJob, `${universalNative}\nenum Bad { Paths(Vec<PathBuf>) }`)) throw new Error("[verify interactivity tool-jobs] self-test native-I/O-resizable-path-output was falsely accepted.");
  if (universal(universalJob, `${universalNative}\nenum Bad { WriteBytes { bytes: Vec<u8> } }`)) throw new Error("[verify interactivity tool-jobs] self-test native-I/O-whole-write-buffer was falsely accepted.");
  if (universal(universalJob, universalNative.replace("std::mem::ManuallyDrop<[Option<PathBuf>; NATIVE_IO_PATH_CAPACITY]>", "[Option<PathBuf>; NATIVE_IO_PATH_CAPACITY]"))) throw new Error("[verify interactivity tool-jobs] self-test native-I/O-path-set-deep-drop-was falsely accepted.");
  if (universal(universalJob, universalNative.replace("assert_eq!(returned.as_os_str().as_encoded_bytes().as_ptr(), plus_one_pointer);", "assert_eq!(returned, expected_path);"))) throw new Error("[verify interactivity tool-jobs] self-test native-I/O-plus-one-path-identity-proof-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("node.waker = Some(cx.waker().clone());", "node.waker = Some(cx.waker().clone()); session.try_submit_step();"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-future-advances-worker-session was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("pump_renderer_io_sessions(1);", ""))) throw new Error("[verify interactivity tool-jobs] self-test renderer-host-pump-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("slot.generation.load(std::sync::atomic::Ordering::Acquire) != generation;", "true;"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-generation-recheck-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("assert_eq!(returned_pointer, plus_one_request_pointer);", "assert!(returned_pointer.is_null());"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-plus-one-request-identity-proof-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("assert_eq!(pump_renderer_io_sessions(0), 0);", "pump_renderer_io_sessions(1);"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-zero-grant-proof-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ui_wgpu::wgpu::PreparedRenderJob>>", "rejected: Option<String>"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-prepared-admission-rejection-owner-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("checked_out_job_mut()?.take_packet()", "job.take_packet()"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-prepared-packet-bypasses-checked-out-owner was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("WorkerJobSession::try_new(job, params)", "WorkerJobSession::new(job, params)"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-mounted-session-fallible-admission-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("pending: Option<semio_framework_job::WorkerJobTicket>", "pending: Option<semio_framework_async::oneshot::Receiver<semio_framework_job::StepOutcome>>"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-mounted-session-ticket-owner-replaced-by-receiver was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("retained_nested_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>>", "discard_nested_outcome: bool"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-nested-output-close-owner-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("fault_writer: std::mem::ManuallyDrop<Option<semio_framework_job::RetainedJobPayloadWriter>>", "fault: Option<Vec<u8>>"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-retained-fault-writer-replaced-by-Vec was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack", "retained_presentation_envelope_caller_fault_drops_owner"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-fault-zero-grant-fixture-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output", "retained_presentation_envelope_caller_cancel_drops_owner"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-cancel-zero-grant-fixture-missing was falsely accepted.");
  const retainedLayoutExport = `
pub struct LayoutExportJob; struct LayoutExportPublication { bytes: [u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], writer: Option<RetainedJobPayloadWriter> }
fn encode_checkpoint() -> Result<[u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], String> {}
fn layout_retained_publication_zero_grant_and_exact_writer_close_are_exact() {}
fn begin_close(&mut self) { self.close_stage = LayoutExportCloseStage::Publication; }
fn close_step(&mut self) {} fn terminal_is_empty(&self) -> bool {}
snapshot_placeholder: Option<Arc<LayoutSnapshot>>
`;
  const retainedLayoutWasm = `
pub struct LayoutExportOperation; const LAYOUT_EXPORT_REJECTION_SLOTS: usize = 8;
enum Slot { ClosingSession { session: WorkerJobSession<LayoutExportJob>, snapshot_owner: Arc<LayoutSnapshot> } }
fn submit() { WorkerJobSession::try_new(job, params); session.try_step_on_caller(); session.take_outcome(ticket); session.take_terminal(); owner.resume(); owner.begin_close(); outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES); retain_layout_export_session(); retain_layout_export_rejection(); }
`;
  const retainedColdRelay = `
enum GuestRelayCompletion { Stepped(Result<GuestRelayStepCompletion, TurnFault>), Rejected(GuestRelayOwnedBytes), Fault(GuestRelayOwnedBytes) }
enum GuestRelayStepCompletion { Running { progress: Option<GuestRelayOwnedBytes> }, Done { output: GuestRelayOwnedBytes }, Failed { error: GuestRelayOwnedBytes } }
struct GuestRelayCompletionSender { slot: Arc<GuestRelayCompletionSlot> }
struct GuestColdRelayJob; struct GuestRelayPublication { writer: Option<semio_framework_job::RetainedJobPayloadWriter> }
const GUEST_RELAY_MOUNTED_SLOTS: usize = 16;
struct GuestRelayMountedOutput { storage: Box<[std::mem::MaybeUninit<u8>; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]> }
struct GuestRelayMountedRegistry { next_generation: std::sync::atomic::AtomicU64 }
fn reserve() { match generation { u64::MAX => Some(0), generation => Some(generation + 1) }; }
fn guest_cold_relay_registry_max_plus_one_generation_and_zero_pump_are_exact() { assert_eq!(output.storage.as_ptr(), output_identity); }
fn guest_cold_relay_publication_max_plus_one_selects_retained_fault_without_losing_source() {}
impl std::future::Future for GuestRelayMountedFuture { fn poll() { self.registry.pump(self.index, self.generation, context.waker()); } }
fn begin_close(&mut self) {} fn terminal_is_empty(&self) -> bool {}
async fn run_job_on_worker() { WorkerJobSession::try_new(relay, params); self.relay_registry.mount(index, mounted_generation, owner); }
`;
  if (!toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay)) throw new Error("[verify interactivity tool-jobs] self-test Layout-and-cold-relay-retained-valid was falsely rejected.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport.replace("writer: Option<RetainedJobPayloadWriter>", "bytes: Vec<u8>"), retainedLayoutWasm, retainedColdRelay)) throw new Error("[verify interactivity tool-jobs] self-test Layout-retained-publication-writer-removal was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm.replace("WorkerJobSession::try_new(job, params)", "WorkerJobSession::new(job, params)"), retainedColdRelay)) throw new Error("[verify interactivity tool-jobs] self-test Layout-Wasm-fallible-admission-removal was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm.replace("retain_layout_export_session();", "drop(session);"), retainedColdRelay)) throw new Error("[verify interactivity tool-jobs] self-test Layout-Wasm-live-drop-retirement-removal was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("slot: Arc<GuestRelayCompletionSlot>", "receiver: oneshot::Receiver<GuestRelayCompletion>"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-oneshot-restoration was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("Rejected(GuestRelayOwnedBytes)", "Rejected(Vec<u8>)"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-direct-Vec-completion-restoration was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("Stepped(Result<GuestRelayStepCompletion, TurnFault>)", "Stepped(Result<JobStep, TurnFault>)"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-indirect-Vec-step-completion-restoration was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("Box<[std::mem::MaybeUninit<u8>; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]>", "Vec<u8>"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-mounted-output-backing-resized was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("u64::MAX => Some(0)", "generation => Some(generation.wrapping_add(1))"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-mounted-generation-wrap-restoration was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("assert_eq!(output.storage.as_ptr(), output_identity);", "assert_eq!(generation, generation);"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-mounted-output-identity-proof-removal was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("const GUEST_RELAY_MOUNTED_SLOTS: usize = 16", "slots: Vec<GuestRelayMountedSession>"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-fixed-mounted-registry-removal was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("self.relay_registry.mount(index, mounted_generation, owner);", "loop { session.step().await; }"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-run-loop-restoration was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("guest_cold_relay_registry_max_plus_one_generation_and_zero_pump_are_exact", "guest_cold_relay_registry_smoke"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-max-plus-one-generation-fixture-removal was falsely accepted.");
  const scalarConfig = toolJobScalarConfigCohortSelfTests();
  const sealer = storeCanonicalEditSealerSelfTests();
  return fixtures.length + 398 + checkpointChecks + toolJobPuzzleReservedRoutesSelfTests() + toolJobOwnerFactoryResolutionSelfTests() + toolJobFactoryProofJoinSelfTests() + toolJobLatestWinsSelfTests() + toolJobMicrosecondBudgetSelfTests() + toolJobTelemetryContentionSelfTests() + toolJobCooperativeMaintenanceSelfTests() + cadPresenceRetirementSelfTests() + proceduralGenerationRootSelfTests() + flowTypedRetirementSelfTests() + flowSelectedCopySelfTests() + scalarConfig.routes + scalarConfig.mutationOracles + scalarConfig.hostileCases + sealer.grants + sealer.schemaHostiles + sealer.sourceHostiles + sealer.digestOracles + sealer.mapGrants + sealer.mapSchemaHostiles + sealer.mapSourceHostiles + sealer.mapDigestOracles + sealer.readerChecks;
}
