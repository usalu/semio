/** 🚪️ The hub ingress, frozen binary and process source checks of the GIS Map proposal approval fixture proof. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { moduleRustSource, readHubBinSource } from "../../../🧪️testing/📖️source-readers/🟦️.ts";

/** 🛰 Checks the hub approval ingress, handler, teardown and registration sources. */
export function checkHubIngressSources({ repoRoot, fixture, ledger }: {
  readonly repoRoot: string;
  readonly fixture: any;
  readonly ledger: any;
}) {
  const hubBin = readHubBinSource(repoRoot);
  const hubBinLaws = moduleRustSource(join(repoRoot, "🌎️hub", "🧪️tests"));
  for (const symbol of ["HubGisMapApprovalIngressAuthorityV1", "acquire_gis_map_approval_ingress", "revalidate_gis_map_approval_authority", "InferenceApprovalRouteContextV1", "inference_runtime.close().await", "publish_gis_map_checkpoint_change"])
    if (!hubBin.includes(symbol)) throw new Error(`Hub GIS Map approval ingress source is missing ${symbol}`);
  for (const law of ["gis_map_approval_ingress_holds_sorted_hub_authority_without_outer_document_write", "gis_map_applied_checkpoint_notifies_two_peers_with_one_exact_rebootstrap_pair"])
    if (!hubBinLaws.includes(law)) throw new Error(`Hub GIS Map approval ingress laws are missing ${law}`);
  const currentMatchesBase = hubBin.slice(hubBin.indexOf("fn current_matches_base("), hubBin.indexOf("impl GisMapApprovalCheckpointPublisherV1 for GisMapApprovalCheckpointPublisherV1Impl"));
  if (
    !hubBin.includes("Self::current_matches_base(&request.scope") ||
    !currentMatchesBase.includes("Some(checkpoint) => checkpoint.scope == *scope && checkpoint.baseline_frontier == *base") ||
    !currentMatchesBase.includes("None => false")
  ) {
    throw new Error("Hub GIS Map checkpoint publication does not require the exact active Directory genesis or edited parent");
  }
  const ingressSource = hubBin.slice(hubBin.indexOf("async fn acquire_gis_map_approval_ingress"), hubBin.indexOf("fn inference_context"));
  const ingressBindings = ["SocketBindingKeyV1::User", "SocketBindingKeyV1::Session", "SocketBindingKeyV1::DirectorySpaceAuthority", "SocketBindingKeyV1::Membership"];
  let ingressCursor = -1;
  for (const binding of ingressBindings) {
    const index = ingressSource.indexOf(binding);
    if (index <= ingressCursor) throw new Error(`Hub approval ingress does not acquire ${binding} in the shared total order`);
    ingressCursor = index;
  }
  if (ingressSource.includes("SocketBindingKeyV1::DocumentWrite")) throw new Error("Hub approval ingress outer-locks DocumentWrite before the retained runtime");
  const approvalHandler = hubBin.slice(hubBin.indexOf("async fn post_inference_gis_map_job_approval"), hubBin.indexOf("fn inference_routes"));
  const acquired = approvalHandler.indexOf("acquire_gis_map_approval_ingress(");
  if (!approvalHandler.includes("InferenceApprovalRouteContextV1 { route, ingress }") || acquired < 0 || acquired > approvalHandler.indexOf("approve_gis_map_job") || ingressSource.indexOf("revalidate_gis_map_approval_authority(") <= ingressSource.indexOf("share_bindings("))
    throw new Error("Hub approval route does not retain ingress through the lower call or revalidate under its retained guards");
  const terminalClose = hubBin.lastIndexOf("inference_runtime.close().await");
  const artifactClose = hubBin.lastIndexOf("artifact_maintenance.shutdown().await");
  if (terminalClose < 0 || artifactClose <= terminalClose) throw new Error("Hub process teardown does not close inference before artifact/process owners");
  const inferenceRegistration = hubBin.slice(hubBin.indexOf("let inference_runtime = match gis_map_binding.as_ref()"), hubBin.indexOf("let inference_ready = inference_runtime.is_some()"));
  if (!inferenceRegistration.includes("RetainedGisMapApprovalCommitterV1::new") || !inferenceRegistration.includes("GisMapApprovalCheckpointPublisherV1Impl") || inferenceRegistration.includes("UnavailableGisMapApprovalCommitterV1"))
    throw new Error("production GIS Map approval did not register its retained three-Store checkpoint committer");
  const frozen = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🧊️gis-map-frozen-binding-v1", "🔣️.json"), "utf8"));
  if (
    fixture.binding.digest !== frozen.expectedDigest ||
    fixture.binding.componentBlake3 !== frozen.binding.package.componentBlake3 ||
    fixture.binding.packageVersion !== frozen.binding.package.version ||
    JSON.stringify(fixture.binding) !== JSON.stringify(ledger.identity.binding)
  )
    throw new Error("proposal, frozen-binding and ledger corpora disagree on the frozen executable identity");
}

/** 🏃 Checks the compiled process runner source and its hostile mutations. */
export function checkProcessSources({ repoRoot, committerLaws, fixture, checkpointControlCases, retainedRuntimeCases, durableUndoCases }: {
  readonly repoRoot: string;
  readonly committerLaws: number;
  readonly fixture: any;
  readonly checkpointControlCases: number;
  readonly retainedRuntimeCases: number;
  readonly durableUndoCases: number;
}) {
  const runner = readFileSync(join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust", "📜️script.ts"), "utf8");
  const processStart = runner.indexOf("\nasync function proveGisMapProposalProcess");
  const processEnd = runner.indexOf("\n/** 🗺️ The exact GIS Map proposal/approval gate", processStart);
  const cancellationStart = runner.indexOf("\nasync function cancelGisMapProcessMcpOwner(");
  const cancellationEnd = runner.indexOf("\n}\n", cancellationStart);
  const processSource = processStart >= 0 && processEnd > processStart && cancellationStart >= 0 && cancellationEnd > cancellationStart
    ? runner.slice(processStart, processEnd).replace("const jobIdB = await cancelGisMapProcessMcpOwner(run, mcpB, fixture.documentId);", runner.slice(cancellationStart, cancellationEnd)) : "";
  const processConforms = (source: string): boolean => {
    const materialize = source.indexOf("await withMaterializedTrustedCatalogBundleV1(");
    const validate = source.indexOf("await validateAndPublishTrustedStdioGisCandidate(");
    const current = source.indexOf("const current = trustedBootstrapCurrent(dataRoot)");
    const publication = source.indexOf("await checkInProcessHeadV1(");
    const peerSubmit = source.indexOf('name: "inference_submit"');
    const checkpointEntered = source.indexOf("await waitForGisInferenceCheckpointControl(");
    const persistedProgress = source.indexOf("const observedB = await callGisMapProcessMcp(", checkpointEntered);
    const peerCancel = source.indexOf('name: "inference_cancel"');
    const checkpointRelease = source.indexOf("await releaseGisInferenceCheckpointControl(");
    const peerCancelled = source.indexOf('page.state === "cancelled"');
    const ownerSubmit = source.indexOf("const submittedA = await gisMapProcessHttp(");
    const approval = source.indexOf('"POST", `${inferenceRoot}/${jobId}/approval`');
    const approvalRebootstrap = source.indexOf('"Author A RebootstrapRequired"');
    const refreshed = source.indexOf("const refreshedA = await readGisMapProcessCheckpoint(");
    const undo = source.indexOf('"POST", undoRoot, undoBody');
    const undoRebootstrap = source.indexOf('"Author A undo RebootstrapRequired"');
    const undone = source.indexOf("const undoneA = await readGisMapProcessCheckpoint(");
    const undoReplay = source.indexOf("const replay = await gisMapProcessHttp(");
    return (
      materialize >= 0 &&
      validate > materialize &&
      current > validate &&
      publication > current &&
      peerSubmit > publication &&
      checkpointEntered > peerSubmit &&
      persistedProgress > checkpointEntered &&
      peerCancel > persistedProgress &&
      checkpointRelease > peerCancel &&
      peerCancelled > checkpointRelease &&
      ownerSubmit > peerCancelled &&
      approval > ownerSubmit &&
      approvalRebootstrap > approval &&
      refreshed > approvalRebootstrap &&
      undo > refreshed &&
      undoRebootstrap > undo &&
      undone > undoRebootstrap &&
      undoReplay > undone &&
      (source.match(/await startGisMapProcessMcpClient\(/gu)?.length ?? 0) === 2 &&
      (source.match(/await openGisMapProcessDocumentSocket\(/gu)?.length ?? 0) === 2 &&
      source.includes("inferenceCheckpointControl: true") &&
      source.includes('selected.executionProtocol?.appChannelVersion !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1') &&
      source.includes("cancelledBPage?.jobId !== jobIdB") &&
      source.includes("observedProgress.completed !== persistedProgress.completed") &&
      source.includes("observedProgress.total !== persistedProgress.total") &&
      source.includes("observedBPage.progress.length > 16") &&
      source.includes('!cancelledKinds.includes("cancel-requested")') &&
      source.includes('!cancelledKinds.includes("cancelled")') &&
      source.includes('terminalB?.proposalHash !== null') &&
      source.includes('denied.value?.code !== "inference.denied"') &&
      source.includes("JSON.stringify(refreshedA) !== JSON.stringify(refreshedB)") &&
      source.includes("JSON.stringify(undoHandle.expectedCurrent) !== JSON.stringify(refreshedA.frontier)") &&
      source.includes("peerUndo.status !== 403") &&
      source.includes("staleUndo.status !== 409") &&
      source.includes("undone.value?.replayed !== false") &&
      source.includes("JSON.stringify(undoneA) !== JSON.stringify(undoneB)") &&
      source.includes("undoneA.packSha256 !== initialA.packSha256") &&
      source.includes("replay.value?.replayed !== true") &&
      source.includes('"no-shell-scene"') &&
      !source.includes('"no-durable-collaborative-redo"')
    );
  };
  if (!processConforms(processSource)) throw new Error("GIS Map proposal process source lost its compiled-generation, two-Author, private-owner, or durable-pair fence");
  const processHostiles = [
    processSource.replace("await validateAndPublishTrustedStdioGisCandidate(", "void validateAndPublishTrustedStdioGisCandidate("),
    processSource.replace("selected.executionProtocol?.appChannelVersion !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1", "false"),
    processSource.replace(/await startGisMapProcessMcpClient\(/u, "await Promise.resolve("),
    processSource.replace(/await openGisMapProcessDocumentSocket\(/u, "await Promise.resolve("),
    processSource.replace("inferenceCheckpointControl: true", "inferenceCheckpointControl: false"),
    processSource.replace('name: "inference_submit"', 'name: "inference_events"'),
    processSource.replace("await waitForGisInferenceCheckpointControl(", "void waitForGisInferenceCheckpointControl("),
    processSource.replace("observedProgress.completed !== persistedProgress.completed", "false"),
    processSource.replace('name: "inference_cancel"', 'name: "inference_events"'),
    processSource.replace("await releaseGisInferenceCheckpointControl(", "void releaseGisInferenceCheckpointControl("),
    processSource.replace('page.state === "cancelled"', 'page.state === "succeeded"'),
    processSource.replace("cancelledBPage?.jobId !== jobIdB", "false"),
    processSource.replace('!cancelledKinds.includes("cancel-requested")', "false"),
    processSource.replace('!cancelledKinds.includes("cancelled")', "false"),
    processSource.replace('terminalB?.proposalHash !== null', "false"),
    processSource.replace('denied.value?.code !== "inference.denied"', "false"),
    processSource.replace('"Author A RebootstrapRequired"', '"substituted"'),
    processSource.replace("JSON.stringify(refreshedA) !== JSON.stringify(refreshedB)", "false"),
    processSource.replace("JSON.stringify(undoHandle.expectedCurrent) !== JSON.stringify(refreshedA.frontier)", "false"),
    processSource.replace("peerUndo.status !== 403", "false"),
    processSource.replace("staleUndo.status !== 409", "false"),
    processSource.replace("undone.value?.replayed !== false", "false"),
    processSource.replace('"Author A undo RebootstrapRequired"', '"substituted"'),
    processSource.replace("JSON.stringify(undoneA) !== JSON.stringify(undoneB)", "false"),
    processSource.replace("undoneA.packSha256 !== initialA.packSha256", "false"),
    processSource.replace("replay.value?.replayed !== true", "false"),
  ];
  processHostiles.forEach((source, index) => {
    if (source === processSource || processConforms(source)) throw new Error(`GIS Map proposal process source oracle admitted hostile ${index}`);
  });
  console.log(
    `gis-map-proposal-oracle: scope-exports=5 committer-laws=${committerLaws} node-sha256=2 independent-bounds=2 preview=1 lifecycle=${fixture.lifecycle.length + fixture.cancelLifecycle.length} checkpoint-control=${checkpointControlCases} retained-runtime=${retainedRuntimeCases} durable-undo=${durableUndoCases} committer=7 visibility=${fixture.visibility.length} errors=${fixture.errors.length} approval-rejections=${fixture.approvalRejections.length} cross-fixture=1 process-source=${processHostiles.length + 1}; no external model provider, no WGPU rendering`,
  );
}
