/** 🧮️ The FEM numerical micro-cursor contract of the live replay law. */
import { toolJobRustBlock } from "../../📋️inventory/🟦️.ts";

/** 🔥️ Phase-6h FEM numerical hot-loop, fixed-owner, identity and hostile-law contract. */
export function toolJobFemNumericalMicrocursorExact(sparse: string, mesh: string, analyses: string, model: string, elements: string, session: string, runtime: string): boolean {
  const namedBlock = (source: string, signature: string) => {
    const start = source.indexOf(signature);
    const open = start < 0 ? -1 : source.indexOf("{", start);
    return open < 0 ? undefined : toolJobRustBlock(source, open);
  };
  const ordered = (source: string, tokens: readonly string[]) => {
    let cursor = 0;
    for (const token of tokens) {
      const next = source.indexOf(token, cursor);
      if (next < 0) return false;
      cursor = next + token.length;
    }
    return true;
  };
  const loopFree = (block: { body: string } | undefined) => !!block && !/\b(?:while|for)\b/.test(block.body);
  const ldltStart = sparse.indexOf("impl InteractiveJob for LdltJob");
  const ldltOpen = ldltStart < 0 ? -1 : sparse.indexOf("{", ldltStart);
  const ldlt = ldltOpen < 0 ? undefined : toolJobRustBlock(sparse, ldltOpen);
  const subspaceStart = sparse.indexOf("impl InteractiveJob for SubspaceIterationJob");
  const subspaceOpen = subspaceStart < 0 ? -1 : sparse.indexOf("{", subspaceStart);
  const subspace = subspaceOpen < 0 ? undefined : toolJobRustBlock(sparse, subspaceOpen);
  const constraintStart = mesh.lastIndexOf("MeshJobStage::ConstrainBoundary =>");
  const constraintEnd = constraintStart < 0 ? -1 : mesh.indexOf("MeshJobStage::ReservePointIndex =>", constraintStart);
  const constraint = constraintEnd < 0 ? "" : mesh.slice(constraintStart, constraintEnd);
  const elementStart = analyses.indexOf("fn advance_element_build(&mut self)");
  const elementOpen = elementStart < 0 ? -1 : analyses.indexOf("{", elementStart);
  const element = elementOpen < 0 ? undefined : toolJobRustBlock(analyses, elementOpen);
  const ldltCheckpoint = namedBlock(sparse, "fn advance_checkpoint_entry(state: &LdltCheckpoint");
  const ldltColumn = namedBlock(sparse, "fn advance_column_microcursor(&mut self)");
  const subspaceCheckpoint = namedBlock(sparse, "fn advance_checkpoint_entry(state: &SubspaceCheckpoint");
  const ldltOutput = namedBlock(sparse, "fn advance_output_entry(");
  const subspacePreview = namedBlock(sparse, "fn advance_preview_entry(");
  const ldltRestore = namedBlock(sparse, "fn decode_page_entry(&mut self, bytes: &[u8]) -> Result<bool, NumericalCheckpointFault>");
  const secondRestoreStart = sparse.indexOf("fn decode_page_entry(&mut self, bytes: &[u8]) -> Result<bool, NumericalCheckpointFault>", (ldltRestore?.end ?? 0) + 1);
  const subspaceRestore = secondRestoreStart < 0 ? undefined : toolJobRustBlock(sparse, sparse.indexOf("{", secondRestoreStart));
  const ldltRestoreImpl = namedBlock(sparse, "impl LdltRestoreCursor");
  const subspaceRestoreImpl = namedBlock(sparse, "impl SubspaceRestoreCursor");
  const restoreHelpers = ["fn restore_u32_entry(", "fn restore_usize_entry(", "fn restore_f64_entry(", "fn restore_pair_entry(", "fn restore_matrix_entry("].map((signature) => namedBlock(sparse, signature));
  const ownerWriters = ["fn advance_u32_owner(", "fn advance_u64_owner(", "fn advance_u64_values(", "fn advance_f64_owner(", "fn advance_pair_owner(", "fn advance_matrix_owner("].map((signature) => namedBlock(sparse, signature));
  const meshStep = namedBlock(mesh, "impl InteractiveJob for MeshJob");
  const meshPreparation = namedBlock(mesh, "impl MeshInputPreparation");
  const meshMountedBegin = namedBlock(mesh, "fn begin_mounted(points: Vec<[f64; 2]>");
  const meshMountedInitialization = namedBlock(mesh, "fn advance_mounted_initialization(&mut self)");
  const meshFinishInsertion = namedBlock(mesh, "fn advance_finish_insertion(&mut self)");
  const meshPayload = namedBlock(mesh, "fn advance_mesh_payload(&mut self)");
  const meshPublication = namedBlock(mesh, "fn advance_publication(&mut self");
  const meshConstraint = namedBlock(mesh, "fn advance_constraint_recovery(&mut self)");
  const faceCursor = namedBlock(mesh, "fn advance_face_classification(&mut self)");
  const meshLaw = namedBlock(mesh, "fn p6h_constraint_flip_interrupts_after_every_edge_phase_and_updates_only_affected_adjacencies()");
  const meshCursorLaw = namedBlock(mesh, "fn p6h_mounted_mesh_preparation_initialization_finish_publication_interrupt_replay_timing_and_close()");
  const elementLaw = namedBlock(analyses, "fn p6h_element_stiffness_microcursor_deadline_stale_cancel_close_and_stage_laws()");
  const assemblyCursorLaw = namedBlock(analyses, "fn p6h_owned_assembly_lookup_partition_scan_transfer_interrupt_replay_and_timing()");
  const elementAdmissionLaw = namedBlock(analyses, "fn mounted_element_stiffness_observes_before_admit_and_retires_rejected_backing()");
  const partitionMerge = namedBlock(analyses, "fn advance_partition_merge(&mut self");
  const subspaceConstructor = namedBlock(sparse, "pub fn new(operation: Operation, k_factor: LdltFactor");
  const identityMatches = namedBlock(sparse, "fn matches(self, operation: Operation) -> bool");
  const stagedWriterStart = runtime.indexOf("impl RetainedJobPayloadWriter");
  const stagedCloseStart = runtime.indexOf("pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> JobPayloadCloseStep", stagedWriterStart);
  const stagedClose = stagedCloseStart < 0 ? undefined : toolJobRustBlock(runtime, runtime.indexOf("{", stagedCloseStart));
  const stagedBegin = namedBlock(runtime, "pub fn begin_staged_page(&mut self");
  const stagedCommit = namedBlock(runtime, "pub fn commit_staged_page(&mut self)");
  const ldltStages = ["ReserveColumn", "SourceEntry", "ContributorLookup", "ContributorEntry", "PivotRead", "DiagonalCommit", "EmitRow", "PublishColumn", "CompleteColumn"];
  const subspaceStages = [
    "ReserveIteration",
    "ApplyOperatorColumnRow",
    "FactorForwardEntry",
    "FactorDiagonalEntry",
    "FactorBackwardEntry",
    "OrthogonalizePairElement",
    "NormalizeColumnElement",
    "ProjectedMatrixCellEntry",
    "JacobiFindPairCell",
    "JacobiRotateCell",
    "JacobiConvergenceCell",
    "ModeSortCompare",
    "ModePermuteElement",
    "ResidualColumnRow",
    "ConvergenceMode",
    "PublishIteration",
  ];
  const constraintStages = [
    "ReserveConstraintWorkspace",
    "IndexTriangleEdge",
    "SearchConstraintEdge",
    "ClassifyIntersection",
    "SelectDeterministicFlip",
    "ValidateFlip",
    "ApplyFlip",
    "RetireFormerEdge",
    "PublishConstraintProgress",
    "ConstraintComplete",
  ];
  const elementStages = [
    "ReferenceQuadraturePoint",
    "ShapeFunctionDerivativeScalar",
    "JacobianCell",
    "DeterminantInverseCell",
    "StrainDisplacementCell",
    "ConstitutiveCell",
    "LocalStiffnessMultiplyCell",
    "BodyTractionLoadCell",
    "LocalToGlobalTripletCell",
  ];
  return (
    ldltStages.every((stage) => sparse.includes(stage)) &&
    subspaceStages.every((stage) => sparse.includes(stage)) &&
    constraintStages.every((stage) => mesh.includes(stage)) &&
    elementStages.every((stage) => analyses.includes(stage)) &&
    sparse.includes("const LDLT_MAXIMUM_ORDER: usize = 40") &&
    sparse.includes("const SUBSPACE_MAXIMUM_ORDER: usize = 40") &&
    sparse.includes("const SUBSPACE_MAXIMUM_COLUMNS: usize = 40") &&
    sparse.includes("struct NumericalCheckpointIdentity") &&
    sparse.includes("revision: operation.base_revision.0") &&
    sparse.includes("generation: operation.generation.0") &&
    sparse.includes("seed: operation.seed") &&
    sparse.includes('b"FEMLCP1\\0"') &&
    sparse.includes('b"FEMSCP1\\0"') &&
    sparse.includes("NUMERICAL_CHECKPOINT_VERSION") &&
    sparse.includes("struct NumericalPageCursor") &&
    sparse.includes("pub struct LdltRestoreCursor") &&
    sparse.includes("pub struct SubspaceRestoreCursor") &&
    sparse.includes("fn decode_page_entry(&mut self, bytes: &[u8])") &&
    sparse.includes("pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize)") &&
    sparse.includes("pub fn terminal_is_empty(&self) -> bool") &&
    sparse.includes("reserve_lane: u8") &&
    sparse.includes("fn advance_reservation(&mut self)") &&
    sparse.includes("initialization_cursor: usize") &&
    sparse.includes("retiring_work: Option<SubspaceWork>") &&
    sparse.includes("fn advance_preview_entry(state: &mut SubspaceCheckpoint") &&
    sparse.includes("fn advance_checkpoint_entry(state: &LdltCheckpoint") &&
    sparse.includes("fn advance_checkpoint_entry(state: &SubspaceCheckpoint") &&
    sparse.includes("begin_staged_page(context)") &&
    sparse.includes("JobPayloadStream::CheckpointState") &&
    sparse.includes("terminal_writer: Option<RetainedJobPayloadWriter>") &&
    sparse.includes("NumericalCheckpointFault::Truncated") &&
    sparse.includes("interrupted_restore.close_step(1, usize::MAX)") &&
    !sparse.includes("publication_bytes") &&
    !sparse.includes("preview_bytes") &&
    sparse.includes("p6h_ldlt_microcursor_max_plus_one_cancel_deadline_stale_replay_and_numerical_parity") &&
    sparse.includes("p6h_subspace_cancellation_is_observed_at_every_nested_stage_and_worker_replay_is_exact") &&
    mesh.includes("indexed_constraint_edges: Vec<IndexedConstraintEdge>") &&
    mesh.includes("p6h_constraint_flip_interrupts_after_every_edge_phase_and_updates_only_affected_adjacencies") &&
    analyses.includes("p6h_element_stiffness_microcursor_deadline_stale_cancel_close_and_stage_laws") &&
    elements.includes("p6h_mounted_element_fixed_schema_cells_match_batch_and_reject_maximum_plus_one") &&
    model.includes("fn mounted_stiffness_cell(&self, _ctx: &ElementContext, _row: usize, _column: usize) -> Option<f64>") &&
    (elements.match(/fn mounted_stiffness_cell\(/g) ?? []).length === 3 &&
    session.includes("MountedOwnerClaim { class: MountedOwnerClass::MeshEdgeIndexVectors, roots: 3") &&
    !!ldlt &&
    ordered(ldlt.body, ["context.should_yield()", "context.consume_fuel(1)", "writer.staged_page_len().is_none()", "return match writer.begin_staged_page(context)", "advance_checkpoint_entry"]) &&
    !ldlt.body.includes("ldlt_column(") &&
    !ldlt.body.includes("while ") &&
    !ldlt.body.includes("unwrap_or_default") &&
    !ldlt.body.includes("checkpoint_bytes()") &&
    !ldlt.body.includes("serde_json") &&
    !ldlt.body.includes("expect(") &&
    !!ldltColumn && loopFree(ldltColumn) && ldltColumn.body.includes("lookup_lower") && ldltColumn.body.includes("lookup_upper") && ldltColumn.body.includes("lookup_mid") && ldltColumn.body.includes("lookup_comparison") && !ldltColumn.body.includes("binary_search") &&
    !!subspace &&
    ordered(subspace.body, ["context.should_yield()", "context.consume_fuel(1)", "writer.staged_page_len().is_none()", "return match writer.begin_staged_page(context)", "advance_checkpoint_entry"]) &&
    !subspace.body.includes("self.iterate(") &&
    !subspace.body.includes("while ") &&
    !subspace.body.includes("serde_json::to_vec(&self.preview())") &&
    !subspace.body.includes("checkpoint_bytes()") &&
    !subspace.body.includes("expect(") &&
    !!subspaceConstructor && !subspaceConstructor.body.includes(".iter().all") && subspaceConstructor.body.includes("factor_validation_complete: admission_fault") && subspace.body.includes("fem.subspace.validate-factor-owner") && ordered(subspace.body, ["!self.state.factor_validation_complete", "context.should_yield()", "context.consume_fuel(1)", "self.state.k_factor.l_cols.get(self.state.factor_validation_cursor)", "self.state.factor_validation_cursor += 1"]) &&
    constraint.includes("advance_constraint_recovery()") &&
    !constraint.includes("recover_constraint(") &&
    !constraint.includes(".edges()") &&
    !!element &&
    element.body.includes("mounted_stiffness_cell") &&
    !element.body.includes("stiffness_global(") &&
    ownerWriters.every((block) => loopFree(block) && block?.body.includes("cursor.item += 1") && block.body.includes("return Ok(false)")) &&
    restoreHelpers.every(loopFree) &&
    loopFree(ldltCheckpoint) &&
    loopFree(subspaceCheckpoint) &&
    loopFree(ldltOutput) &&
    loopFree(subspacePreview) &&
    !!ldltCheckpoint && ordered(ldltCheckpoint.body, ["advance_numerical_page_header", "advance_", "writer.commit_staged_page()", "cursor.item = 0", "cursor.owner = 0"]) &&
    !!subspaceCheckpoint && ordered(subspaceCheckpoint.body, ["advance_numerical_page_header", "advance_", "writer.commit_staged_page()", "cursor.item = 0", "cursor.owner = 0"]) &&
    !!ldltRestore && !ldltRestore.body.includes("[0u8;") && !ldltRestore.body.includes("copy_from_slice(source)") && ldltRestore.body.includes("page_entry") &&
    !!subspaceRestore && !subspaceRestore.body.includes("[0u8;") && !subspaceRestore.body.includes("copy_from_slice(source)") && subspaceRestore.body.includes("page_entry") &&
    !!ldltRestoreImpl && !!subspaceRestoreImpl && !ldltRestoreImpl.body.includes("copy_from_slice(source)") && !subspaceRestoreImpl.body.includes("copy_from_slice(source)") && ldltRestoreImpl.body.includes("payload.take()") && subspaceRestoreImpl.body.includes("payload.take()") &&
    !!identityMatches && identityMatches.body.includes("self == Self::from_operation(operation)") &&
    !!meshStep && loopFree(meshStep) && ordered(meshStep.body, ["context.should_yield()", "context.consume_fuel(1)", "match self.stage"]) && meshStep.body.includes("MeshJobStage::CountInput") && meshStep.body.includes("triangulation.insert_next();") &&
    !!meshPreparation && loopFree(meshPreparation) && meshPreparation.body.includes("point_lookup_cursor") && meshPreparation.body.includes("grid_polygon") && meshPreparation.body.includes("grid_edge") && !meshPreparation.body.includes(".iter().find") && !meshPreparation.body.includes("point_in_polygon(") && !meshPreparation.body.includes("holes.iter()") &&
    !!meshMountedBegin && loopFree(meshMountedBegin) && !meshMountedBegin.body.includes(".iter()") && !meshMountedBegin.body.includes("sort_by") && !meshMountedBegin.body.includes("sort_unstable") && meshMountedBegin.body.includes("MountedTriangulationStage::BoundsPoint") &&
    !!meshMountedInitialization && loopFree(meshMountedInitialization) && !meshMountedInitialization.body.includes("sort_by") && !meshMountedInitialization.body.includes("extend(") && meshMountedInitialization.body.includes("sort_inner") && meshMountedInitialization.body.includes("insertion_order.swap") &&
    !!meshFinishInsertion && loopFree(meshFinishInsertion) && !meshFinishInsertion.body.includes(".retain(") && !meshFinishInsertion.body.includes("sort_unstable") && meshFinishInsertion.body.includes("finish.read") && meshFinishInsertion.body.includes("finish.write") && meshFinishInsertion.body.includes("triangles.swap") &&
    !!meshPayload && loopFree(meshPayload) && !meshPayload.body.includes("Vec::with_capacity") && !meshPayload.body.includes("extend_from_slice") && meshPayload.body.includes("write_staged") && meshPayload.body.includes("commit_staged_page") && meshPayload.body.includes("MeshPayloadStage::PointCoordinate") && meshPayload.body.includes("MeshPayloadStage::TriangleIndex") &&
    !!meshPublication && loopFree(meshPublication) && ordered(meshPublication.body, ["RetainedJobPayloadWriter::new", "writer.staged_page_len().is_none()", "writer.begin_staged_page(context)", "self.advance_mesh_payload()", "writer.finish()"]) &&
    !!meshConstraint && loopFree(meshConstraint) && meshConstraint.body.includes("constraint_retire_adjacency_cursor") && meshConstraint.body.includes("slot.adjacent[self.constraint_retire_adjacency_cursor]") && ordered(meshConstraint.body, ["self.constraint_retire_adjacency_cursor += 1", "slot.active =", "self.constraint_retire_cursor += 1"]) &&
    !!faceCursor && loopFree(faceCursor) && faceCursor.body.includes("FaceClassificationStage::HoleEdge") && faceCursor.body.includes("self.point_index.get(cursor.lookup)") &&
    !!meshLaw && meshLaw.body.includes("job.step(&mut deadline)") && meshLaw.body.includes("job.step(&mut stale)") && meshLaw.body.includes("job.step(&mut cancelled)") && meshLaw.body.includes("maximum_micros < 8_000") && !meshLaw.body.includes("job.advance_constraint_recovery()") &&
    !!meshCursorLaw && meshCursorLaw.body.includes("MeshJob::new_bounded") && meshCursorLaw.body.includes("preparation_lookup_seen") && meshCursorLaw.body.includes("MountedTriangulationStage::OrderInsertion") && meshCursorLaw.body.includes("TriangulationFinishStage::OrderTriangles") && meshCursorLaw.body.includes("MeshPayloadStage::CommitPage") && meshCursorLaw.body.includes("StepBudget::new(1, 0)") && meshCursorLaw.body.includes("InteractiveJob::close_step(&mut interrupted, 1, 0)") && meshCursorLaw.body.includes("first.1.max(second.1) < 8_000") &&
    !!elementLaw && elementLaw.body.includes("AssemblyJobConstruction::new_owned") && elementLaw.body.includes("Bar2") && elementLaw.body.includes("BeamEb2") && elementLaw.body.includes("Tri3Cst") && elementLaw.body.includes("mounted stiffness mismatch") && elementLaw.body.includes("maximum_micros < 8_000") &&
    element.body.includes("lookup_cursor") && element.body.includes("PendingElementBuildStage::PublishIndex") && element.body.includes("PendingElementBuildStage::PublishPosition") && !element.body.includes("dof_map.get(") && !element.body.includes("nodes.iter().find") &&
    !!partitionMerge && loopFree(partitionMerge) && partitionMerge.body.includes("merge_scan_partition") && partitionMerge.body.includes("merge_candidate") && !partitionMerge.body.includes(".iter()") && !partitionMerge.body.includes("filter_map") && !partitionMerge.body.includes("min_by_key") && !analyses.includes("fn next_partition_triplet") &&
    !!assemblyCursorLaw && assemblyCursorLaw.body.includes("AssemblyJobConstruction::new_owned") && assemblyCursorLaw.body.includes("merge_scan_partition") && assemblyCursorLaw.body.includes("lookup_cursor") && assemblyCursorLaw.body.includes("StepBudget::new(1, 0)") && assemblyCursorLaw.body.includes("first.2.max(second.2) < 8_000") &&
    !!elementAdmissionLaw && elementAdmissionLaw.body.includes("reserve_exact_owner_page") && elementAdmissionLaw.body.includes("observed - 1") && elementAdmissionLaw.body.includes("rejected.as_ptr()") && !elementAdmissionLaw.body.includes("include_str!") &&
    !!stagedBegin && stagedBegin.body.includes("self.admit_page(cx)?.stage()") &&
    !!stagedCommit && stagedCommit.body.includes("self.staged.take()") &&
    !!stagedClose && ordered(stagedClose.body, ["self.staged.as_ref()", "ledger.release(stream)", "self.rejected.is_some()", "payload.close_step"])
  );
}
