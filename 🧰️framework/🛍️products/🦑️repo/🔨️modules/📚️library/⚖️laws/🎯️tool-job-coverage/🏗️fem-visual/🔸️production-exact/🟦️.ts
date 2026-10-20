/** 🔸️ The production exact phase of toolJobFemLiveVisualPublicationExact. */
import type { buildRustBodyState } from "../🔶️rust-body/🟦️.ts";

/** 🔹 Builds productionExact of toolJobFemLiveVisualPublicationExact. */
export function buildProductionExact({ twoMountedRender, twoPacket, twoPacketClose, canvasClose, canvasRender, ordered, numericalStep, fem3dSession, numerical, solverAdmit, orderAdmission, numericalFixedAdmit, analysisFixedAdmit, meshFixedAdmit, scalarFixedAdmit, fixedOwnerLaw, femSparse, numericalClose, mountedClose, threeMounted, threeReconcile, mountedFail, retainedFault, solverDrop, leaseDrop, candidateDrop, stateDrop, mountedJobDrop, stateRecoveryPublish, abandonedRecovery, dropRecoveryLaw, dropRecoveryDrainLaw, backingRecovery, snapshotBegin, snapshotClaim, snapshotRecovery, worldSnapshot, statusConsumer, frameworkWorld, threePageJob, femAnalyses, femMesh, editorRender, viewerDispatch, modelRender, resultsRender, viewerRender }: Pick<ReturnType<typeof buildRustBodyState>, "twoMountedRender" | "twoPacket" | "twoPacketClose" | "canvasClose" | "canvasRender" | "ordered" | "numericalStep" | "numerical" | "solverAdmit" | "orderAdmission" | "numericalFixedAdmit" | "analysisFixedAdmit" | "meshFixedAdmit" | "scalarFixedAdmit" | "fixedOwnerLaw" | "numericalClose" | "mountedClose" | "threeMounted" | "threeReconcile" | "mountedFail" | "retainedFault" | "solverDrop" | "leaseDrop" | "candidateDrop" | "stateDrop" | "mountedJobDrop" | "stateRecoveryPublish" | "abandonedRecovery" | "dropRecoveryLaw" | "dropRecoveryDrainLaw" | "backingRecovery" | "snapshotBegin" | "snapshotClaim" | "snapshotRecovery" | "statusConsumer" | "threePageJob" | "editorRender" | "viewerDispatch" | "modelRender" | "resultsRender" | "viewerRender"> & { readonly fem3dSession: string; readonly femSparse: string; readonly worldSnapshot: string; readonly frameworkWorld: string; readonly femAnalyses: string; readonly femMesh: string }) {
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
  return { productionExact };
}
