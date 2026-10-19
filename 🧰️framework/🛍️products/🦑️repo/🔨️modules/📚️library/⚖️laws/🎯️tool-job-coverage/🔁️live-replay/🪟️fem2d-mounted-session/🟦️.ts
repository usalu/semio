import { toolJobRustBlock } from "../../📋️inventory/🟦️.ts";

/** 🏗️ Phase-6 mounted FEM2D job ownership and live-route contract. `session`, `editor`, `model`, `mesh` and `analyses` are the source families of the carved crates with their canonical test files, `glue` is the hub composition manifest, `reactor` the reactor family, `store` the os Store with the value snapshot-read crate and `frameworkPlugin` the plugin family. */
export function toolJobFem2dMountedSessionExact(session: string, editor: string, model: string, mesh: string, pluginRoot: string, glue: string, jobs: string, analyses: string, reactor: string, store: string, frameworkPlugin: string): boolean {
  const reconcileStart = session.indexOf("pub fn reconcile(");
  const reconcileOpen = reconcileStart < 0 ? -1 : session.indexOf("{", reconcileStart);
  const reconcile = reconcileOpen < 0 ? undefined : toolJobRustBlock(session, reconcileOpen);
  const stepStart = session.indexOf("pub(super) fn step(&mut self, budget: JobBudget, cx: &mut StepContext<'_>) -> JobStep");
  const stepOpen = stepStart < 0 ? -1 : session.indexOf("{", stepStart);
  const step = stepOpen < 0 ? undefined : toolJobRustBlock(session, stepOpen);
  const pluginStart = pluginRoot.indexOf("pub fn plugin()");
  const pluginOpen = pluginStart < 0 ? -1 : pluginRoot.indexOf("{", pluginStart);
  const plugin = pluginOpen < 0 ? undefined : toolJobRustBlock(pluginRoot, pluginOpen);
  const snapshotAdmission = frameworkPlugin.indexOf("let snapshot_is_admitted = {");
  const snapshotIssue = frameworkPlugin.indexOf("self.stores.store.snapshot_read()", snapshotAdmission);
  const renderStart = model.indexOf("pub fn render_with_progress(");
  const renderOpen = renderStart < 0 ? -1 : model.indexOf("{", renderStart);
  const render = renderOpen < 0 ? undefined : toolJobRustBlock(model, renderOpen);
  const classifyStart = mesh.lastIndexOf("MeshJobStage::Classify =>");
  const classifyEnd = classifyStart < 0 ? -1 : mesh.indexOf("MeshJobStage::Finalize =>", classifyStart);
  const classify = classifyEnd < 0 ? "" : mesh.slice(classifyStart, classifyEnd);
  return (
    session.includes('pub const FEM2D_MOUNTED_JOB_KIND: &str = "semio.fem2d.mounted-analysis"') &&
    session.includes("const SESSION_ACTIVE_CAPACITY: usize = 32") &&
    session.includes("const SESSION_SHELL_CAPACITY: usize = 64") &&
    session.includes("const SESSION_MAXIMUM_INPUT_ITEMS: usize = 4_096") &&
    session.includes("const SESSION_MAXIMUM_INPUT_BYTES: usize = 4 * 1_024 * 1_024") &&
    session.includes("struct MountedProcessOwnerCatalog") &&
    session.includes("claims: [MountedOwnerClaim; 30]") &&
    session.includes("MountedOwnerClass::AssemblyDofStrings") &&
    session.includes("MountedOwnerClass::MeshPreparationIndexVector") &&
    session.includes("MountedOwnerClass::MeshTriangulationWorkspaceVectors") &&
    session.includes("MountedOwnerClass::MeshEdgeIndexVectors") &&
    session.includes("process_owner_inventory_admits_exact_maximum_and_returns_exact_credit") &&
    session.includes("const FEM2D_JOB_TAG: u64 = 0xf2d0_0000_0000_0000") &&
    session.includes("job & !FEM2D_JOB_COUNTER_MAXIMUM != FEM2D_JOB_TAG") &&
    session.includes("shells: [Rc<RefCell<Option<MountedState>>>; SESSION_SHELL_CAPACITY]") &&
    session.includes("current: [Option<CurrentSession>; SESSION_ACTIVE_CAPACITY]") &&
    session.includes("retiring: [Option<u16>; SESSION_SHELL_CAPACITY]") &&
    session.includes("free: [u16; SESSION_SHELL_CAPACITY]") &&
    session.includes("canonical_base_revision: [u8; 32]") &&
    session.includes("preflight: [Option<PendingSnapshotAdmission>; SESSION_ACTIVE_CAPACITY]") &&
    session.includes("struct SnapshotAdmissionCursor") &&
    session.includes("fn step_one(&mut self, snapshot: &Fem2dSnapshot)") &&
    session.includes("credit_items: [usize; SESSION_SHELL_CAPACITY]") &&
    session.includes("credit_bytes: [usize; SESSION_SHELL_CAPACITY]") &&
    !session.includes("region.holes.iter().map") &&
    !session.includes("case.loads.iter().map") &&
    !session.includes("combination.terms.iter().map") &&
    session.includes("register_bounded_job_kind(FEM2D_MOUNTED_JOB_KIND") &&
    session.includes("fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, ValueError>") &&
    session.includes("fn terminal_is_empty(&self) -> bool") &&
    session.includes("pub fn maintenance_step(") &&
    session.includes("pub fn close_step(") &&
    session.includes("pub fn terminal_is_empty(") &&
    session.includes("visual_job_candidate: Option<Fem2dVisualJob>") &&
    session.includes("visual_rejected: Option<Fem2dVisualJob>") &&
    session.includes("visual_current: Option<Fem2dMountedVisualLease>") &&
    session.includes("visual_displaced: Option<Fem2dMountedVisualLease>") &&
    session.includes("fn publish_visual_candidate(&mut self, candidate: Fem2dMountedVisualLease) -> Result<(), Fem2dMountedVisualLease>") &&
    session.includes("return_to_registry_witness()") &&
    session.includes("witness.terminal_is_empty()") &&
    !!step &&
    (step.body.match(/\.step\(cx\)/g) ?? []).length === 4 &&
    !step.body.includes("while ") &&
    step.body.includes("current_identity(self.identity.app_instance_id)") &&
    step.body.includes("commit_authority_matches(") &&
    step.body.includes("self.cancel.is_cancelled_now()") &&
    step.body.indexOf("commit_authority_matches(") < step.body.indexOf("JobStep::Done(output)") &&
    !!reconcile &&
    reconcile.body.includes("doc.take_snapshot_read()") &&
    reconcile.body.includes("Effect::CancelJob") &&
    reconcile.body.includes("Effect::SpawnJob") &&
    reconcile.body.indexOf("Effect::CancelJob") < reconcile.body.indexOf("Effect::SpawnJob") &&
    reconcile.body.includes("JobPlacement::Isolated") &&
    !reconcile.body.includes("run_to_completion") &&
    !reconcile.body.includes("block_on") &&
    editor.includes("fn pending_effects(") &&
    editor.includes("semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::reconcile(doc)") &&
    editor.includes("session::with_live_visual(doc.render_operation()") &&
    editor.includes("model_window::render_with_progress(canvas, &window.camera, visual, &interaction, window_id.as_deref(), active_utility)") &&
    editor.includes("fn mounted_job_maintenance_step(") &&
    editor.includes("fn mounted_job_close_step(") &&
    editor.includes("fn mounted_jobs_terminal_is_empty(") &&
    editor.includes("fn mounted_job_prepare_snapshot_read(") &&
    !!render &&
    render.body.includes("progress.map(Fem2dMountedVisualLease::snapshot)") &&
    render.body.includes("fem2d_structure_layers_with(doc") &&
    !render.body.includes("fem2d_live_visual_layers") &&
    !render.body.includes("fem2d_region_triangles") &&
    !render.body.includes("serde_json") &&
    !render.body.includes("sort") &&
    !render.body.includes("collect") &&
    model.includes("pub struct Fem2dVisualJob") &&
    model.includes("Fem2dVisualJobStage::ReserveSnapshot") &&
    model.includes("canvas2d_snapshot_begin(") &&
    model.includes("self.output.admit_page(") &&
    model.includes("canvas2d_snapshot_seal(token)") &&
    model.includes("Fem2dVisualJobStage::OrderRegionKey") &&
    model.includes("Fem2dVisualJobStage::OrderElementKey") &&
    model.includes("fn order_field_one(&mut self, visual: &Fem2dLiveVisual)") &&
    model.includes("Fem2dVisualJobStage::BuildRegion") &&
    model.includes("fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize)") &&
    model.includes("mounted_visual_output_exact_maximum_plus_one_and_page_handback") &&
    model.includes("fem2d_visual_job_maximum_plus_one_rejects_before_owner_transfer") &&
    model.includes("fem2d_visual_job_stale_cancel_fault_and_device_close_preserve_last_valid") &&
    model.includes("fem2d_visual_job_replay_accessibility_and_each_step_are_bounded") &&
    model.includes("assert!(packet_equal(&first, &second))") &&
    model.includes("started.elapsed().as_micros() < 8_000") &&
    classify.includes("self.advance_face_classification()") &&
    !classify.includes("collect(") &&
    !classify.includes("Vec<") &&
    mesh.includes("fn advance_face_classification(&mut self)") &&
    mesh.includes("triangulation.triangles.get(self.face_cursor)") &&
    mesh.includes("MeshJobStage::ReservePreparation") &&
    mesh.includes("MeshJobStage::ReserveEdgeAuthorities") &&
    mesh.includes("mesh_edge_authority_uses_completed_faces_and_closes_exact_backing") &&
    !!plugin &&
    plugin.body.indexOf("semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::initialize()") < plugin.body.indexOf('PluginBuilder::<_, FemApps>::new("fem")') &&
    glue.includes("semio-s-artifact-fem-2d-editor-session = { workspace = true }") &&
    jobs.includes("pub trait BoundedJob") &&
    jobs.includes("JobBody::Bounded(owner)") &&
    !jobs.includes("#[cfg(not(test))]") &&
    !jobs.includes("JobBody::ExplicitStateMachineRequired") &&
    analyses.includes("pub struct AssemblyJobConstruction") &&
    analyses.includes("AssemblyConstructionStage::ValidateNodePairs") &&
    analyses.includes("AssemblyConstructionStage::DiscoverDofs") &&
    analyses.includes("AssemblyConstructionStage::CommitDofOwner") &&
    analyses.includes("struct PendingElementBuild") &&
    analyses.includes("PendingElementBuildStage::ReserveIndices") &&
    analyses.includes("PendingElementBuildStage::ReservePositions") &&
    analyses.includes("PendingElementBuildStage::ReserveStiffnessCredit") &&
    analyses.includes("PendingElementBuildStage::AllocateStiffness") &&
    analyses.includes("PendingElementBuildStage::ObserveStiffnessBacking =>") &&
    analyses.includes("PendingElementBuildStage::AdmitStiffnessBacking") &&
    analyses.includes("mounted_element_stiffness_observes_before_admit_and_retires_rejected_backing") &&
    analyses.includes("fn advance_element_build(&mut self)") &&
    analyses.includes("fn reclaim_element_owner(&mut self)") &&
    analyses.includes("mounted_element_build_reserves_and_reclaims_one_exact_owner_per_turn") &&
    !analyses.includes("pending_node_ids: Vec<String>") &&
    analyses.includes("pub struct AssemblyCsrBuild") &&
    session.includes("AssemblyJobConstruction::new_owned") &&
    session.includes("AssemblyCsrBuild::new") &&
    session.includes("PcgJobConstruction::new") &&
    session.includes("assembly_build.close_step(maximum_bytes)") &&
    !step.body.includes("AssemblyJob::new_owned(") &&
    !step.body.includes("into_full_matrix(") &&
    !step.body.includes("PcgJob::new(") &&
    reactor.includes("Event::JobProgress { job, .. }") &&
    reactor.includes("JOB_RENDER_BINDINGS.with") &&
    reactor.includes("dirty_background_surfaces(runtime, binding.instance, &mut *dirty).await?") &&
    reactor.includes("dirty.try_surface(instance, surface)") &&
    store.includes("pub fn commit_authority_matches") &&
    store.includes("fn publish_authority") &&
    store.includes("pub struct SnapshotReadReturn") &&
    store.includes("pub fn return_to_registry_witness") &&
    snapshotAdmission >= 0 &&
    snapshotIssue > snapshotAdmission &&
    frameworkPlugin.slice(snapshotAdmission, snapshotIssue).includes("A::mounted_job_prepare_snapshot_read(render_operation, snapshot.as_ref())") &&
    session.includes("mounted_revision_restart_keeps_cancel_before_spawn") &&
    session.includes("mounted_close_and_capacity_are_fixed_and_terminal_witnessed")
  );
}
