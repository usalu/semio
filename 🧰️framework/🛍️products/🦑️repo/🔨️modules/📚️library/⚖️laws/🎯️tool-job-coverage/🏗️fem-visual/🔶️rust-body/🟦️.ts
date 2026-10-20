/** 🔶️ The rust body phase of toolJobFemLiveVisualPublicationExact. */
import { toolJobRustBlock } from "../../📋️inventory/🟦️.ts";

/** 🔷 Builds rustBody, region, ordered, stages of toolJobFemLiveVisualPublicationExact. */
export function buildRustBodyState({ fem2dModel, fem2dSession, fem3dSession, fem3dModel, fem3dResults, fem3dViewerModel, fem3dEditor, fem3dViewer, frameworkPlugin, frameworkWorld, femAnalyses, femMesh, femSparse, canvasRenderer, canvasSnapshot, worldSnapshot }: { readonly fem2dModel: string; readonly fem2dSession: string; readonly fem3dSession: string; readonly fem3dModel: string; readonly fem3dResults: string; readonly fem3dViewerModel: string; readonly fem3dEditor: string; readonly fem3dViewer: string; readonly frameworkPlugin: string; readonly frameworkWorld: string; readonly femAnalyses: string; readonly femMesh: string; readonly femSparse: string; readonly canvasRenderer: string; readonly canvasSnapshot: string; readonly worldSnapshot: string }) {
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
  return { twoMountedRender, twoPacket, twoPacketClose, canvasClose, canvasRender, ordered, numericalStep, numerical, solverAdmit, orderAdmission, numericalFixedAdmit, analysisFixedAdmit, meshFixedAdmit, scalarFixedAdmit, fixedOwnerLaw, numericalClose, mountedClose, threeMounted, threeReconcile, mountedFail, retainedFault, solverDrop, leaseDrop, candidateDrop, stateDrop, mountedJobDrop, stateRecoveryPublish, abandonedRecovery, dropRecoveryLaw, dropRecoveryDrainLaw, backingRecovery, snapshotBegin, snapshotClaim, snapshotRecovery, statusConsumer, threePageJob, editorRender, viewerDispatch, modelRender, resultsRender, viewerRender, stageTokens, twoStorage, twoWrite, twoPageClose, twoLeaseClose, twoJob, twoDrive, twoWork, threeStep, threeJobClose, threeWork, preflightCharge, preflightGuard, preflightCommit, preflightStep, cleanRender, viewerApp, prepared };
}
