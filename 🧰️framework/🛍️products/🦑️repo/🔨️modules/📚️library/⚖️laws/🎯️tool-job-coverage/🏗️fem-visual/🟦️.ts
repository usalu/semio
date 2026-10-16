/** 🧭️ Tool-job coverage: FEM live visual publication law.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
 */
import { toolJobRustBlock } from "../📋️inventory/🟦️.ts";

// 👁️ Exact mounted FEM 2D/3D fixed-page visual construction and prepared publication contract.
export function toolJobFemLiveVisualPublicationExact(
  fem2dModel: string,
  fem2dSession: string,
  fem3dSession: string,
  fem3dEditor: string,
  fem3dModel: string,
  fem3dResults: string,
  fem3dViewer: string,
  fem3dViewerModel: string,
  femPluginRoot: string,
  femGlue: string,
  femSparse: string,
  frameworkPlugin: string,
  frameworkWorld: string,
  worldSnapshot: string,
  canvasSnapshot: string,
  canvasRenderer: string,
  femAnalyses: string,
  femMesh: string,
): boolean {
  const rustBody = (source: string, signature: string, start = 0) => {
    const found = source.indexOf(signature, start);
    const open = found < 0 ? -1 : source.indexOf("{", found);
    return open < 0 ? undefined : toolJobRustBlock(source, open);
  };
  const region = (source: string, first: string, last: string) => {
    const start = source.indexOf(first);
    const end = start < 0 ? -1 : source.indexOf(last, start);
    return start < 0 || end < 0 ? "" : source.slice(start, end);
  };
  const ordered = (source: string, tokens: string[]) => {
    let cursor = -1;
    for (const token of tokens) {
      cursor = source.indexOf(token, cursor + 1);
      if (cursor < 0) return false;
    }
    return true;
  };
  const stages = [
    "ReserveSnapshot",
    "ReadProgressScalar",
    "OrderRegionKey",
    "BuildRegion",
    "OrderElementKey",
    "BuildMeshElement",
    "BuildAssemblyMark",
    "BuildLoadGlyph",
    "BuildSupportGlyph",
    "BuildDisplacementEntry",
    "BuildResidualEntry",
    "BuildReactionEntry",
    "BuildContourEntry",
    "BuildModeEntry",
    "BuildLabelEntry",
    "SealPages",
    "ValidateFreshness",
    "PublishLease",
    "RetireDisplacedLease",
  ];
  const stageTokens = (prefix: string) => stages.map((stage) => prefix + "::" + stage);
  const twoStorage = region(fem2dModel, "struct Fem2dFixedPacketPages", "//#endregion 🧵️MountedVisualJob");
  const twoJob = region(fem2dModel, "pub struct Fem2dVisualJob", "//#endregion 🧵️MountedVisualJob");
  const twoPageClose = rustBody(twoStorage, "fn close_step(&mut self, maximum_bytes: usize)");
  const twoLeaseClose = rustBody(twoStorage, "pub(crate) fn close_step(&mut self, maximum_bytes: usize)", twoStorage.indexOf("impl Fem2dMountedVisualLease"));
  const twoWrite = rustBody(twoStorage, "fn write_str(&mut self, value: &str)");
  const twoDrive = rustBody(fem2dSession, "fn drive_visual_one(");
  const threePageJob = region(fem3dSession, "pub struct Fem3dSolverView", "//#endregion 📦️PageVisualJob");
  const threeStep = rustBody(threePageJob, "fn step_one(&mut self, doc: &Fem3dSnapshot");
  const threeJobClose = rustBody(threePageJob, "fn close_step(&mut self, maximum_bytes: usize)", threePageJob.indexOf("impl Fem3dPageVisualJob"));
  const threeMounted = rustBody(fem3dSession, "fn step(&mut self, budget: JobBudget)");
  const threeReconcile = rustBody(fem3dSession, "pub fn reconcile(");
  const preflight = region(fem3dSession, "impl SnapshotPreflight", "struct PendingSnapshot");
  const preflightCharge = rustBody(preflight, "fn charge(");
  const preflightStep = rustBody(preflight, "fn step_one(&mut self, snapshot: &Fem3dSnapshot)");
  const modelRender = rustBody(fem3dModel, "pub fn render_with_progress(");
  const resultsRender = rustBody(fem3dResults, "pub fn render_with_progress(");
  const viewerRender = rustBody(fem3dViewerModel, "pub fn render(");
  const editorRender = rustBody(fem3dEditor, "fn render(");
  const viewerDispatch = rustBody(fem3dViewer, "fn render(");
  const viewerApp = region(frameworkPlugin, "pub struct ViewerApp<V: ArtifactViewer>", "//#endregion 🔖️ViewerApp");
  const prepared = rustBody(frameworkWorld, "pub fn step_world3d_snapshot(");
  const cleanRender = (body: string) => [".to_string(", "fem3d_scene_parts(", "mesh_preview", "serde_json", ".sort", ".collect"].every((token) => !body.includes(token));
  const twoWork = twoDrive?.body.slice(twoDrive.body.indexOf("let Some(snapshot)")) ?? "";
  const threeWork = threeMounted?.body.slice(threeMounted.body.indexOf("let Some(snapshot)")) ?? "";
  const preflightGuard = preflightCharge?.body.indexOf("if usize::from(page_items)") ?? -1;
  const preflightCommit = preflightCharge?.body.indexOf("self.page_items[page] = page_items") ?? -1;
  const twoPacket = region(fem2dModel, "struct Fem2dFixedPacketPages", "//#endregion 🧵️MountedVisualJob");
  const twoMountedRender = rustBody(fem2dModel, "pub fn render_with_progress(");
  const twoPacketClose = rustBody(twoPacket, "fn close_step(&mut self, maximum_bytes: usize)");
  const numerical = region(fem3dSession, "enum Fem3dNumericalStage", "struct FixedOrder");
  const numericalStep = rustBody(numerical, "fn step(&mut self, doc: &Fem3dSnapshot");
  const numericalClose = rustBody(numerical, "fn close_step(&mut self, maximum_bytes: usize)");
  const numericalFixedAdmit = rustBody(numerical, "fn admit_one(&mut self, target: usize)");
  const analysisFixedAdmit = rustBody(femAnalyses, "fn admit_one(admitted: &mut usize, target: usize, maximum: usize)");
  const meshFixedAdmit = rustBody(femMesh, "pub fn admit_one(&mut self, target: usize)");
  const scalarFixedAdmit = rustBody(femSparse, "pub fn admit_one(&mut self, target: usize)");
  const fixedOwnerLaw = rustBody(fem3dSession, "fn fem3d_numerical_fixed_owner_maximum_plus_one_refuses_unchanged_and_closes_one_slot()");
  const mountedClose = rustBody(fem3dSession, "fn close_step(&mut self, maximum_bytes: usize) -> PluginCloseStep");
  const canvasRender = rustBody(canvasRenderer, "fn render_canvas_2d(");
  const canvasClose = rustBody(canvasSnapshot, "pub fn canvas2d_snapshot_close_step(");
  const statusConsumer = region(frameworkWorld, "pub fn step_world3d_snapshot(", "pub fn close_world3d_snapshot_apply_step(");
  const solverAdmit = rustBody(threePageJob, "fn admit_page(&mut self, page: usize, initialized: bool, backing: &mut Fem3dBackingCredit)");
  const orderAdmission = rustBody(threePageJob, "fn new(backing: &mut Fem3dBackingCredit) -> Result<Self, ()>", threePageJob.indexOf("impl<const N: usize> FixedOrder<N>"));
  const mountedFail = rustBody(fem3dSession, "fn fail(&mut self, detail: Vec<u8>)");
  const retainedFault = rustBody(numerical, "fn retain_fault(&mut self, payload: RetainedJobPayload)");
  const snapshotBegin = rustBody(worldSnapshot, "pub fn world3d_snapshot_begin(");
  const snapshotClaim = rustBody(worldSnapshot, "pub fn world3d_snapshot_claim_draw_permit(");
  const snapshotRecovery = rustBody(worldSnapshot, "pub fn world3d_snapshot_recovery_close_step(");
  const solverDrop = rustBody(threePageJob, "fn drop(&mut self)", threePageJob.indexOf("impl Drop for Fem3dSolverView"));
  const leaseDrop = rustBody(threePageJob, "fn drop(&mut self)", threePageJob.indexOf("impl Drop for Fem3dPageVisualLease"));
  const candidateDrop = rustBody(threePageJob, "fn drop(&mut self)", threePageJob.indexOf("impl Drop for Fem3dPageVisualJob"));
  const stateDrop = rustBody(fem3dSession, "fn drop(&mut self)", fem3dSession.indexOf("impl Drop for MountedState"));
  const mountedJobDrop = rustBody(fem3dSession, "fn drop(&mut self)", fem3dSession.indexOf("impl Drop for MountedJob"));
  const backingRecovery = rustBody(fem3dSession, "fn close_recovered_fem3d_backing(maximum_bytes: usize)");
  const stateRecoveryPublish = rustBody(fem3dSession, "fn publish_owner(&self, identity: Identity, state: MountedState)");
  const abandonedRecovery = rustBody(fem3dSession, "fn recover_abandoned_one(");
  const dropRecoveryLaw = rustBody(fem3dSession, "fn fem3d_queued_running_and_state_drop_publish_exact_identity_and_drain_one_owner()");
  const dropRecoveryDrainLaw = rustBody(fem3dSession, "fn drain_recovery_state(");
  const productionExact = (
    !!twoMountedRender &&
    twoMountedRender.body.includes("layers_json: String::new()") &&
    twoMountedRender.body.includes("snapshot: progress.map(Fem2dMountedVisualLease::snapshot)") &&
    !twoMountedRender.body.includes("materialize") &&
    !twoMountedRender.body.includes("serde_json") &&
    twoPacket.includes("pages: [Option<Canvas2dSnapshotPage>; FEM2D_MOUNTED_VISUAL_PAGE_COUNT]") &&
    twoPacket.includes("canvas2d_snapshot_admit_page") &&
    twoPacket.includes("canvas2d_snapshot_seal") &&
    !!twoPacketClose &&
    twoPacket.includes("canvas2d_snapshot_close_step") &&
    !!canvasClose &&
    canvasClose.body.includes("slot.admitted_pages -= 1") &&
    canvasClose.body.includes("slot.pages[usize::from(slot.admitted_pages)] = None") &&
    !!canvasRender &&
    ordered(canvasRender.body, ["if let Some(snapshot) = canvas.snapshot", "canvas2d_snapshot_with_page", "Canvas2dPacketItem<'_>", "return;", "canvas.layers_json"]) &&
    !canvasRender.body.slice(canvasRender.body.indexOf("if let Some(snapshot)"), canvasRender.body.indexOf("canvas.layers_json")).includes(".collect") &&
    !!numericalStep &&
    !numericalStep.body.includes("while ") &&
    fem3dSession.includes("scalars: [Option<Box<[std::mem::MaybeUninit<Fem3dSolverScalar>; FEM3D_SOLVER_FIELDS_PER_PAGE]>>; FEM3D_SOLVER_PAGE_COUNT]") &&
    fem3dSession.includes("initialized: [Option<Box<[bool; FEM3D_SOLVER_FIELDS_PER_PAGE]>>; FEM3D_SOLVER_PAGE_COUNT]") &&
    numerical.includes("ReserveSolverPages") &&
    numericalStep.body.includes("solver.admit_page(self.solver_page_cursor, self.solver_page_lane, backing)") &&
    !!solverAdmit &&
    ordered(solverAdmit.body, ["backing.claim(FEM3D_SOLVER_INITIALIZED_PAGE_BYTES)", "Box::new([false; FEM3D_SOLVER_FIELDS_PER_PAGE])"]) &&
    ordered(solverAdmit.body, ["backing.claim(FEM3D_SOLVER_SCALAR_PAGE_BYTES)", "Box::new([std::mem::MaybeUninit::uninit(); FEM3D_SOLVER_FIELDS_PER_PAGE])"]) &&
    !!orderAdmission &&
    ordered(orderAdmission.body, ["backing.claim(bytes)", "Box::new([None; N])"]) &&
    fem3dSession.includes("self.scalars[page].take().is_some()") &&
    fem3dSession.includes("self.initialized[page].take().is_some()") &&
    numericalStep.body.includes("MeshJob::new_mounted_bounded") &&
    numericalStep.body.includes("mesh.completed_point(self.point_cursor)") &&
    numericalStep.body.includes("mesh.completed_triangle(self.point_cursor)") &&
    ordered(numericalStep.body, ["context.consume_fuel(1)", "self.step_model(doc)?"]) &&
    ordered(numericalStep.body, ["context.consume_fuel(1)", "MeshJob::new_mounted_bounded"]) &&
    !numerical.includes("try_reserve_exact") &&
    !numerical.includes("close_vec_step") &&
    !numerical.includes("Vec::new()") &&
    numerical.includes("model: Option<MountedAnalysisModel>") &&
    numerical.includes("analysis_node_ids: FixedSlots<String, MAXIMUM_FIELDS>") &&
    numerical.includes("meshed_solids: FixedSlots<Fem3dMeshedSolid, MAXIMUM_REGIONS>") &&
    numerical.includes("solid_points: FixedSlots<[f64; 2], MAXIMUM_FIELDS>") &&
    numerical.includes("solid_tris: FixedSlots<[u32; 3], MAXIMUM_ELEMENTS>") &&
    numerical.includes("solid_node_ids: FixedSlots<String, MAXIMUM_FIELDS>") &&
    numerical.includes("solid_node_analysis_indices: FixedSlots<usize, MAXIMUM_FIELDS>") &&
    numerical.includes("rhs: MountedScalarSlots,") &&
    numerical.includes("modal_free_mass: MountedScalarSlots,") &&
    numerical.includes("SolidDomainOuterPoint") &&
    numerical.includes("SolidDomainHolePoint") &&
    numerical.includes("SolidNodeLookup") &&
    numerical.includes("SolidTet") &&
    numerical.includes("ElementMass") &&
    numerical.includes("SolidTetMass") &&
    numerical.includes("SolidIndicesRetire") &&
    numerical.includes("self.solid_node_analysis_indices.pop()") &&
    !numerical.includes("self.solid_node_analysis_indices = FixedSlots::new()") &&
    numerical.includes("Tet4 {") &&
    numerical.includes("self.tet_phase == 3") &&
    numerical.includes("ApplyAreaNode") &&
    numerical.includes("ApplySelfWeightSolid") &&
    numerical.includes("RecoverReaction") &&
    numerical.includes("self.reaction_accumulator - self.full_rhs[row]") &&
    numerical.includes("build.visual_full_entry(self.reaction_entry)") &&
    numerical.includes("build.visual_compact_index(entry_col)") &&
    numerical.includes("ModalInputConstruction::new_mounted(matrix, std::mem::take(&mut self.modal_free_mass))") &&
    numerical.includes("self.modal_lumped_mass[analysis * 6 + self.mass_update_cursor % 3] += self.pending_tet_mass") &&
    (numerical.match(/self\.pending_element = Some\(built\)/g) ?? []).length >= 2 &&
    numerical.includes("self.pending_tet = Some(tet)") &&
    numerical.includes("self.solid_node_ids = retained.node_ids") &&
    numerical.includes("self.rejected_pcg_matrix = Some(matrix)") &&
    !!numericalFixedAdmit &&
    ordered(numericalFixedAdmit.body, ["if target > N", "return Err(())", "self.admitted += 1"]) &&
    !!analysisFixedAdmit &&
    ordered(analysisFixedAdmit.body, ["if target > maximum", "return Err(())", "*admitted += 1"]) &&
    !!meshFixedAdmit &&
    ordered(meshFixedAdmit.body, ["if target > MOUNTED_DOMAIN_POINT_SLOTS", "return Err(())", "self.admitted += 1"]) &&
    !!scalarFixedAdmit &&
    ordered(scalarFixedAdmit.body, ["if target > MOUNTED_SCALAR_SLOTS", "return Err(())", "self.admitted += 1"]) &&
    !!fixedOwnerLaw &&
    ["ids.admit_one(2)", "ids.admitted, 0", "returned.as_ptr(), rejected_pointer", "ids.pop()", "ids.close_admission_one()", "model.admit_node_one(MAXIMUM_FIELDS + 1)", "returned.id.as_ptr(), node_pointer", "domain.admit_outer_one(MAXIMUM_FIELDS + 1)", "scalars.admit_one(MAXIMUM_FIELDS * 6 + 1)", "started.elapsed().as_micros() < 8_000"].every((token) => fixedOwnerLaw.body.includes(token)) &&
    femSparse.includes("ModalInputStage::ValidateMass") &&
    femSparse.includes("if !value.is_finite() || value <= 0.0") &&
    femSparse.includes("pub struct MountedScalarSlots") &&
    femSparse.includes("self.stage = ModalInputStage::CopyMountedMass") &&
    femSparse.includes("ModalInputStage::RetireMountedMass") &&
    femSparse.includes("PcgConstructionStage::RetireMountedB") &&
    numerical.includes("SubspaceIterationJob::new(") &&
    numerical.includes("subspace.visual_mode_scalar(0, equation)") &&
    numerical.includes("solver.publish_scalar(freshness") &&
    numerical.includes("solver.publish_progress(freshness") &&
    fem3dSession.includes("self.initialized_count != self.len || total != self.len") &&
    fem3dSession.includes("self.state == Fem3dVisualState::ValidatedFinal && self.initialized_count == self.len") &&
    !!numericalClose &&
    !numericalClose.body.includes("loop ") &&
    numericalClose.body.includes("self.analysis_node_ids.pop()") &&
    numericalClose.body.includes("self.solid_node_ids.pop()") &&
    numericalClose.body.includes("self.rhs.close_step()") &&
    numericalClose.body.includes("self.modal_free_mass.close_step()") &&
    ordered(numericalClose.body, ["self.mesh.as_mut()", "self.assembly_build.as_mut()", "self.csr_build.as_mut()", "self.pcg.as_mut()", "self.modal_build.as_mut()", "self.ldlt.as_mut()", "self.subspace.as_mut()"]) &&
    !!mountedClose &&
    ordered(mountedClose.body, ["self.numerical.as_mut()", "self.solver.as_mut()", "self.snapshot.take()"]) &&
    fem3dSession.includes("numerical_done: bool") &&
    fem3dSession.includes("self.numerical_done = true") &&
    !!threeMounted &&
    threeMounted.body.includes("numerical.step(snapshot, solver, &mut self.backing, freshness, operation, &mut cx)") &&
    fem3dSession.includes("registry.reserve_credit(shell)") &&
    ordered(threeReconcile?.body ?? "", ["registry.reserve_credit(shell)", "recovery.reserve(identity)", "MountedState::new(identity, snapshot, former, credit, recovery)"]) &&
    fem3dSession.includes("FEM3D_PROCESS_BACKING_ITEMS") &&
    fem3dSession.includes("FEM3D_PROCESS_BACKING_BYTES") &&
    fem3dSession.includes("self.credit_items[slot] = FEM3D_PROCESS_BACKING_ITEMS") &&
    fem3dSession.includes("self.credit_bytes[slot] = FEM3D_PROCESS_BACKING_BYTES") &&
    !!mountedFail &&
    !mountedFail.body.includes(".clone()") &&
    mountedFail.body.includes("JobStep::Failed(if detail.capacity() <= FAULT_BYTES { detail }") &&
    !!retainedFault &&
    retainedFault.body.includes("self.fault_payload = Some(payload)") &&
    !retainedFault.body.includes(".clone()") &&
    !retainedFault.body.includes("single_page") &&
    !retainedFault.body.includes("bytes.to_vec") &&
    fem3dSession.includes("impl Drop for Fem3dSolverView") &&
    !!solverDrop &&
    solverDrop.body.includes("self.scalars.iter_mut().filter_map(Option::take)") &&
    solverDrop.body.includes("self.initialized.iter_mut().filter_map(Option::take)") &&
    solverDrop.body.includes("recover_fem3d_backing") &&
    fem3dSession.includes("impl Drop for Fem3dPageVisualLease") &&
    !!leaseDrop &&
    leaseDrop.body.includes("world3d_snapshot_recover_lease(self.snapshot)") &&
    fem3dSession.includes("impl Drop for Fem3dPageVisualJob") &&
    !!candidateDrop &&
    candidateDrop.body.includes("self.pages.iter_mut().filter_map(Option::take)") &&
    candidateDrop.body.includes("world3d_snapshot_recover_page(page)") &&
    candidateDrop.body.includes("world3d_snapshot_recover_write(token)") &&
    candidateDrop.body.includes("Fem3dRecoveredBacking::RegionOrder(order.slots)") &&
    candidateDrop.body.includes("Fem3dRecoveredBacking::ElementOrder(order.slots)") &&
    fem3dSession.includes("impl Drop for MountedState") &&
    !!stateDrop &&
    ordered(stateDrop.body, ["self.cancel.cancel_now()", "std::mem::replace(self, placeholder)", "recovery.publish_owner(identity, owner)"]) &&
    !stateDrop.body.includes("external shell recovery owner reached terminal empty") &&
    fem3dSession.includes("impl Drop for MountedJob") &&
    !!mountedJobDrop &&
    ordered(mountedJobDrop.body, ["self.recovery.publish(self.identity, MountedRecoveryPublication::Recover)", "self.cancel()", "self.shell.try_borrow_mut()", "shell.take()", "self.recovery.publish_owner(self.identity, state)"]) &&
    mountedJobDrop.body.includes("*shell = Some(state)") &&
    !!stateRecoveryPublish &&
    ordered(stateRecoveryPublish.body, ["self.publish(identity, MountedRecoveryPublication::Recover)", "self.owner.lock()", "*owner = Some(state)"]) &&
    fem3dSession.includes("recoveries: [Rc<MountedRecoverySlot>; SHELL_CAPACITY]") &&
    fem3dSession.includes("recovery: Rc<MountedRecoverySlot>") &&
    fem3dSession.includes("recovery.reserved.get() != Some(identity)") &&
    !!abandonedRecovery &&
    ordered(abandonedRecovery.body, ["recovery.publication.get()?", "recovery.take_owner(identity)", "state.close_step(maximum_bytes)", "recovery.restore_owner(identity, state)"]) &&
    (abandonedRecovery.body.match(/recovery\.restore_owner\(identity, state\)/g) ?? []).length === 2 &&
    ordered(abandonedRecovery.body, ["state.terminal_is_empty()", "drop(state)", "recovery.release(identity)", "registry.release_credit(shell as u16)", "registry.release(shell as u16)"]) &&
    (fem3dSession.match(/recover_abandoned_one\(&mut registry\.borrow_mut\(\), app_instance_id, maximum_bytes\)/g) ?? []).length === 2 &&
    !!dropRecoveryLaw &&
    ["drop(queued_job)", "queued_borrow.as_ref().map(|state| state.identity)", "drop(running_job)", "running_recovery.owner.lock().unwrap().as_ref().map(|state| state.identity)", "drop(recoverable_state(state_identity, state_recovery.clone()))"].every((token) => dropRecoveryLaw.body.includes(token)) &&
    !!dropRecoveryDrainLaw &&
    ["released_items <= 1", "released_bytes <= WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY", "registry.credit_items[shell as usize], 0", "registry.credit_bytes[shell as usize], 0"].every((token) => dropRecoveryDrainLaw.body.includes(token)) &&
    fem3dSession.includes("recover_fem3d_backing") &&
    !!backingRecovery &&
    backingRecovery.body.includes("recovery.owners.iter_mut().find(|slot| slot.is_some())") &&
    backingRecovery.body.includes("let owner = owner.take()?") &&
    !backingRecovery.body.includes("recovery.owners = [const { None }") &&
    fem3dSession.includes("world3d_snapshot_recover_page") &&
    fem3dSession.includes("world3d_snapshot_recovery_close_step(maximum_bytes)") &&
    fem3dSession.includes("fem3d_process_permits_precede_solver_order_allocation_and_drop_handoff_closes_one_backing") &&
    !!snapshotBegin &&
    ordered(snapshotBegin.body, ["store.reserved_draw_bytes = reserved_draw_bytes", "store.slots[slot]", "Some(World3dSnapshotSlot"]) &&
    !!snapshotClaim &&
    snapshotClaim.body.includes("slot.descriptor.draw_count != draw_count") &&
    snapshotClaim.body.includes("slot.draw_claimed = true") &&
    !!snapshotRecovery &&
    snapshotRecovery.body.includes("store.recovered_pages.iter_mut().find(|page| page.is_some())?.take()?") &&
    snapshotRecovery.body.includes("slot.admitted_pages -= 1") &&
    !snapshotRecovery.body.includes("store.recovered_pages = [const { None }") &&
    worldSnapshot.includes("draw_permit_is_reserved_before_publication_and_orphan_close_is_one_page") &&
    ordered(statusConsumer, ["world3d_snapshot_claim_draw_permit(cursor.lease, 1, instance_count, draw_bytes)", "begin_world3d_draw_rebuild(state, descriptor)"]) &&
    frameworkWorld.includes("draw_permit: Option<World3dSnapshotDrawPermit>") &&
    threePageJob.includes("draw_count: 1") &&
    threePageJob.includes("draw_instance_count: self.draw_count") &&
    threePageJob.includes("draw_byte_count: self.draw_bytes") &&
    fem3dSession.includes("fem3d_production_numerical_child_solid_reaction_modal_and_close_are_cursorized") &&
    fem3dSession.includes("fem3d_production_field_correspondence_rejects_sparse_and_zero_aliases") &&
    femAnalyses.includes("pub fn visual_full_entry(&self, index: usize)") &&
    femAnalyses.includes("pub fn visual_compact_index(&self, new_index: usize)") &&
    femAnalyses.includes("pub struct MountedAnalysisModel") &&
    femAnalyses.includes("nodes: [Option<Node>; MOUNTED_ANALYSIS_NODE_SLOTS]") &&
    femAnalyses.includes("elements: [Option<Elements>; MOUNTED_ANALYSIS_ELEMENT_SLOTS]") &&
    femAnalyses.includes("supports: [Option<MountedAnalysisSupport>; MOUNTED_ANALYSIS_SUPPORT_SLOTS]") &&
    femAnalyses.includes("pub fn new_mounted(model: Arc<MountedAnalysisModel>") &&
    femMesh.includes("pub struct MountedPlanarDomain") &&
    femMesh.includes("points: [[f64; 2]; MOUNTED_DOMAIN_POINT_SLOTS]") &&
    femMesh.includes("holes: [MountedPlanarPolygon; MOUNTED_DOMAIN_HOLE_SLOTS]") &&
    femMesh.includes("pub fn new_mounted_bounded(domain: MountedPlanarDomain") &&
    femMesh.includes("pub fn completed_point(&self, index: usize)") &&
    femSparse.includes("pub fn visual_mode_scalar(&self, mode: usize, index: usize)") &&
    femSparse.includes("pub fn close_step(&mut self, maximum_bytes: usize)") &&
    statusConsumer.includes("World3dSnapshotPageKind::Status if (10..=14).contains(&item.flags)") &&
    statusConsumer.includes("item.numbers[11] as f32") &&
    statusConsumer.includes("World3dSnapshotPageKind::Status if item.flags == 20") &&
    statusConsumer.includes("world3d_draw_rebuild_admit_instance") &&
    frameworkWorld.includes("state.prepared_status = cursor.status") &&
    frameworkWorld.includes("for (index, status) in state.prepared_status.iter().flatten().enumerate()") &&
    fem3dSession.includes("fem3d_solver_nonzero_generation_corresponds_to_every_published_field_page") &&
    !!editorRender &&
    editorRender.body.includes("live_visual::with_live_visual(doc.render_operation(), |visual| window_model::render_with_progress(camera, visual))") &&
    editorRender.body.includes("live_visual::with_live_visual(doc.render_operation(), |visual| window_results::render_with_progress(camera, visual))") &&
    !!viewerDispatch &&
    viewerDispatch.body.includes("live_visual::with_live_visual") &&
    !!modelRender &&
    modelRender.body.includes("scene.snapshot = visual.map(") &&
    !!resultsRender &&
    resultsRender.body.includes("scene.snapshot = visual.map(") &&
    !!viewerRender &&
    viewerRender.body.includes("scene.snapshot = visual.map(")
  );
  const visualExact = (
    ordered(fem2dModel, stageTokens("Fem2dVisualJobStage")) &&
    ordered(fem3dSession, stageTokens("Fem3dVisualJobStage")) &&
    twoStorage.includes("pages: [Option<Canvas2dSnapshotPage>; FEM2D_MOUNTED_VISUAL_PAGE_COUNT]") &&
    twoStorage.includes("slots: [Option<usize>; N]") &&
    twoStorage.includes("self.output.admit_page(usize::from(self.reserve_lane) - 1)") &&
    twoStorage.includes("self.output.take_page(self.page_cursor)") &&
    !!twoWrite &&
    twoWrite.body.includes("push(value.as_bytes())") &&
    !!twoPageClose &&
    twoPageClose.body.includes("self.pages.iter_mut().find(|page| page.is_some())") &&
    twoPageClose.body.includes("*page = None") &&
    !!twoLeaseClose &&
    twoLeaseClose.body.includes("self.region_order.pop().is_some()") &&
    twoLeaseClose.body.includes("canvas2d_snapshot_close_step(self.snapshot)") &&
    !twoStorage.includes("bytes: String") &&
    !twoStorage.includes("Vec<usize>") &&
    !twoStorage.includes("try_reserve_exact") &&
    !twoStorage.includes(".truncate(") &&
    !twoJob.includes("while ") &&
    !twoJob.includes("for ") &&
    twoJob.includes("fem2d.visual-maximum-plus-one") &&
    twoJob.includes("self.region_order.swap(") &&
    twoJob.includes("self.element_order.swap(") &&
    twoJob.includes("self.field_order.swap(") &&
    twoJob.includes("freshness != self.freshness") &&
    twoJob.includes("reaction-field-") &&
    twoJob.includes("contour-field-") &&
    twoJob.includes("mode-field-") &&
    twoJob.includes("accessible-en") &&
    twoJob.includes("accessible-de") &&
    twoJob.includes("Cancel Retry Discard") &&
    twoJob.includes("Abbrechen Wiederholen Verwerfen") &&
    fem2dModel.includes("mounted_visual_output_exact_maximum_plus_one_and_page_handback") &&
    fem2dModel.includes("fem2d_visual_job_maximum_plus_one_rejects_before_owner_transfer") &&
    fem2dModel.includes("fem2d_visual_job_stale_cancel_fault_and_device_close_preserve_last_valid") &&
    fem2dModel.includes("fem2d_visual_job_replay_accessibility_and_each_step_are_bounded") &&
    !!twoDrive &&
    twoWork.includes("cx.consume_fuel(1);\n        match candidate.step_one(snapshot, &self.visual, freshness)") &&
    twoDrive.body.includes("self.visual_rejected = self.visual_job_candidate.take()") &&
    fem2dSession.includes("self.visual_displaced = self.visual_current.take()") &&
    fem2dSession.includes("lease.matches_freshness(freshness)") &&
    fem2dSession.includes("snapshot.commit_authority_matches(") &&
    fem2dSession.includes("MountedStage::SyncPcgVisual") &&
    fem2dSession.includes("fem2d.visual-load-maximum-plus-one") &&
    femSparse.includes("pub fn visual_scalar(&self, index: usize) -> Option<PcgVisualScalar>") &&
    femSparse.includes("reaction: -residual") &&
    femSparse.includes("pub fn visual_progress(&self)") &&
    threePageJob.includes("scalars: [Option<Box<[std::mem::MaybeUninit<Fem3dSolverScalar>; FEM3D_SOLVER_FIELDS_PER_PAGE]>>; FEM3D_SOLVER_PAGE_COUNT]") &&
    threePageJob.includes("initialized: [Option<Box<[bool; FEM3D_SOLVER_FIELDS_PER_PAGE]>>; FEM3D_SOLVER_PAGE_COUNT]") &&
    threePageJob.includes("freshness != self.freshness || index >= self.len") &&
    threePageJob.includes("pages: [Option<World3dSnapshotPage>; FEM3D_VISUAL_PAGES]") &&
    threePageJob.includes("slots: Box<[Option<usize>; N]>") &&
    threePageJob.includes("self.pages[page] = Some(World3dSnapshotPage::new(Self::page_kind(page)))") &&
    threePageJob.includes("world3d_snapshot_admit_page(token, page)") &&
    threePageJob.includes("world3d_snapshot_seal(token)") &&
    !threePageJob.includes("BoundedText") &&
    !threePageJob.includes("try_reserve_exact") &&
    !threePageJob.includes(".truncate(") &&
    !!threeStep &&
    !threeStep.body.includes("while ") &&
    !threeStep.body.includes("for ") &&
    threeStep.body.includes("fem3d.visual-maximum-plus-one") &&
    threeStep.body.includes("self.region_order") &&
    threeStep.body.includes("self.element_order") &&
    threeStep.body.includes('"tetrahedron"') &&
    threeStep.body.includes('"hexahedron"') &&
    threeStep.body.includes("solver.scalar(self.cursor)") &&
    threeStep.body.includes("(field.displacement, field.contour, 10)") &&
    threeStep.body.includes("(field.residual, solver.residual_norm, 11)") &&
    threeStep.body.includes("(field.reaction, field.contour, 12)") &&
    threeStep.body.includes("(field.mode_shape, field.eigen_estimate, 14)") &&
    threeStep.body.includes("freshness != self.freshness || solver.freshness != self.freshness") &&
    !!threeJobClose &&
    threeJobClose.body.includes("self.pages.iter_mut().find(|page| page.is_some())") &&
    threeJobClose.body.includes("*page = None") &&
    threeJobClose.body.includes("self.region_order.take().is_some()") &&
    threeJobClose.body.includes("self.element_order.take().is_some()") &&
    fem3dSession.includes("pub fn publish_solver_scalar(") &&
    fem3dSession.includes("pub fn publish_solver_progress(") &&
    fem3dSession.includes("fem3d_solver_nonzero_generation_corresponds_to_every_published_field_page") &&
    fem3dSession.includes("assert_eq!(&displacement[..3], &scalar.displacement)") &&
    fem3dSession.includes("assert_eq!(&residual[..3], &scalar.residual)") &&
    fem3dSession.includes("assert_eq!(&reaction[..3], &scalar.reaction)") &&
    fem3dSession.includes("assert_eq!(contour[3], scalar.contour)") &&
    fem3dSession.includes("assert_eq!(&mode[..3], &scalar.mode_shape)") &&
    fem3dSession.includes("assert_eq!(mode[3], scalar.eigen_estimate)") &&
    fem3dSession.includes("fem3d_visual_maximum_plus_one_rejects_before_owner_transfer") &&
    fem3dSession.includes("fem3d_snapshot_preflight_page_maximum_plus_one_returns_exact_producer") &&
    fem3dSession.includes("fem3d_visual_stale_cancel_fault_deadline_and_interrupted_device_close_preserve_last_valid") &&
    fem3dSession.includes("fem3d_visual_replay_accessibility_fixed_page_close_and_each_step_are_bounded") &&
    !!threeMounted &&
    threeWork.includes("cx.consume_fuel(1);\n        let step = self.candidate.as_mut().map(|candidate| candidate.step_one(snapshot, solver, &mut self.backing, freshness))") &&
    threeMounted.body.includes("self.displaced = self.current.replace(lease)") &&
    !!threeReconcile &&
    ordered(threeReconcile.body, ["doc.take_snapshot_read()", "Effect::CancelJob", "Effect::SpawnJob"]) &&
    threeReconcile.body.includes("current.take()") &&
    threeReconcile.body.includes("JobPlacement::Isolated") &&
    !!preflightCharge &&
    preflightGuard >= 0 &&
    preflightGuard < preflightCommit &&
    !!preflightStep &&
    !preflightStep.body.includes("while ") &&
    !preflightStep.body.includes("for ") &&
    preflightStep.body.includes("if next > MAXIMUM_LOADS") &&
    fem3dSession.includes("pending.preflight.step_one(snapshot)") &&
    fem3dSession.includes("pending.preflight.stage == SnapshotPreflightStage::Complete") &&
    fem3dSession.includes("return_to_registry_witness()") &&
    fem3dSession.includes("terminal_drop_is_shallow") &&
    femPluginRoot.includes("crate::artifacts::fem3d::live_visual::initialize();") &&
    femGlue.includes("pub mod live_visual;") &&
    !!editorRender &&
    editorRender.body.includes("crate::artifacts::fem3d::live_visual::with_live_visual(doc.render_operation()") &&
    editorRender.body.includes("window_model::render_with_progress(camera, visual)") &&
    editorRender.body.includes("window_results::render_with_progress(camera, visual)") &&
    !!modelRender &&
    modelRender.body.includes("scene.snapshot = visual.map(") &&
    cleanRender(modelRender.body) &&
    !!resultsRender &&
    resultsRender.body.includes("scene.snapshot = visual.map(") &&
    cleanRender(resultsRender.body) &&
    fem3dViewer.includes("fn mounted_job_prepare_snapshot_read(") &&
    fem3dViewer.includes("fn mounted_job_maintenance_step(") &&
    fem3dViewer.includes("fn mounted_job_close_step(") &&
    fem3dViewer.includes("fn mounted_jobs_terminal_is_empty(") &&
    fem3dViewer.includes("crate::artifacts::fem3d::live_visual::reconcile(doc)") &&
    !!viewerDispatch &&
    viewerDispatch.body.includes("crate::artifacts::fem3d::live_visual::with_live_visual(doc.render_operation(), model::render)") &&
    !!viewerRender &&
    viewerRender.body.includes("scene.snapshot = visual.map(") &&
    cleanRender(viewerRender.body) &&
    viewerApp.includes("V::mounted_job_maintenance_step(instance_id, maximum_items, maximum_bytes)") &&
    viewerApp.includes("V::mounted_job_close_step(instance_id, maximum_items, maximum_bytes)") &&
    viewerApp.includes("V::mounted_jobs_terminal_is_empty(instance_id)") &&
    viewerApp.includes("V::mounted_job_prepare_snapshot_read(operation, snapshot)") &&
    viewerApp.includes("V::pending_effects(doc, cfg)") &&
    !!prepared &&
    prepared.body.includes("world3d_snapshot_with_page(cursor.lease, cursor.page") &&
    prepared.body.includes("page.item(usize::from(cursor.item)).copied()") &&
    prepared.body.includes("begin_world3d_draw_rebuild(state, descriptor)") &&
    (prepared.body.match(/world3d_draw_rebuild_admit_instance\(state, 0, id, model, color, false, false\)/g) ?? []).length === 2 &&
    prepared.body.includes("world3d_draw_rebuild_seal(state)") &&
    prepared.body.includes("state.snapshot_lease = Some(cursor.lease)") &&
    prepared.body.includes("context.consume_fuel(1)")
  );
  return productionExact && visualExact;
}
