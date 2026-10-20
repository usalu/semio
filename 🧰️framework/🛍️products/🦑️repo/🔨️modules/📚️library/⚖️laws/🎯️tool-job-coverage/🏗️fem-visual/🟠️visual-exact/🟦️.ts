/** 🟠️ The visual exact phase of toolJobFemLiveVisualPublicationExact. */
import type { buildRustBodyState } from "../🔶️rust-body/🟦️.ts";

/** 🟡 Builds visualExact of toolJobFemLiveVisualPublicationExact. */
export function buildVisualExact({ ordered, fem2dModel, stageTokens, fem3dSession, twoStorage, twoWrite, twoPageClose, twoLeaseClose, twoJob, twoDrive, twoWork, fem2dSession, femSparse, threePageJob, threeStep, threeJobClose, threeMounted, threeWork, threeReconcile, preflightCharge, preflightGuard, preflightCommit, preflightStep, femPluginRoot, femGlue, editorRender, modelRender, cleanRender, resultsRender, fem3dViewer, viewerDispatch, viewerRender, viewerApp, prepared }: Pick<ReturnType<typeof buildRustBodyState>, "ordered" | "stageTokens" | "twoStorage" | "twoWrite" | "twoPageClose" | "twoLeaseClose" | "twoJob" | "twoDrive" | "twoWork" | "threePageJob" | "threeStep" | "threeJobClose" | "threeMounted" | "threeWork" | "threeReconcile" | "preflightCharge" | "preflightGuard" | "preflightCommit" | "preflightStep" | "editorRender" | "modelRender" | "cleanRender" | "resultsRender" | "viewerDispatch" | "viewerRender" | "viewerApp" | "prepared"> & { readonly fem2dModel: string; readonly fem3dSession: string; readonly fem2dSession: string; readonly femSparse: string; readonly femPluginRoot: string; readonly femGlue: string; readonly fem3dViewer: string }) {
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
  return { visualExact };
}
