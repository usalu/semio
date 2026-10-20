import { TOOL_JOB_FRAMEWORK_RESERVED_IDS, type ToolJobCoverageReport } from "../📋️inventory/🟦️.ts";
import type { SelfTestLanes } from "../../🧪️self-test-lanes/🟦️.ts";
import { buildMacroFilesState } from "./🔶️macro-files/🟦️.ts";
import { buildFrameworkReservedExactState } from "./🔸️framework-reserved-exact/🟦️.ts";

/** 🎯️ Phase-8 source/runtime contract census used by `verify interactivity tool-jobs`. */
export async function toolJobCoverageRun(root: string, lanes: SelfTestLanes): Promise<ToolJobCoverageReport> {
  const { platform, failures, plugin, actionBus, puzzle2d, store, semio, causal, vcs, allRustFiles, presentationEnvelopeCodec, presentationWasm, writerEnvelopeCodec, writerEditor, writerWasm, jackEnvelopeCodec, jackEditor, jackWasm, trinityRewrite, gisMapEnvelopeCodec, gisMapEditor, gisMapWasm, rasterEvidence, drawingEnvelopeCodec, drawingEditor, drawingEditorTestLaws, channel, presenceRetirement, kernel, reactor, pluginHost, componentWit, mcpWorkspace, runHost, wgpuHost, actor, shardHost, jobRuntime, nativeIo, fem2dSession, fem2dEditor, fem2dModel, femMesh, femPluginRoot, femGlue, reactorJobs, femAnalyses, reactorFamily, valueSnapshotRead, femSparse, femEngineModel, femElements2d, reactorRequests, reactorExecutor, productionFiles, staticRows, dispositions, proofs, proofIdentities, manifest, factoryContracts, globalPayloadStores, scanThenMonolithRows, appOwnedRows, appOwnedKeys, scanThenMonolithKeys, registeredAppOwnedKeys, macroFiles, macroInvocations, commandRows, seenRows, duplicateRows, fixtureMacroFiles, fixtureMacroInvocations, fixtureCommandRows, batchOnlyRows, forbiddenRows, deletedRows, globalPayloadStoreExemptions, selfTests } = await buildMacroFilesState({ root, lanes });
  const { remainingCommands, frameworkReservedRoutes, reservedImporters, acceptedCommandRows, frameworkReservedExact, fullToolOperationJobBounded, storeOneItemPublicationBounded, ephemeralOneItemPublicationBounded, fullToolOperationBounded } = buildFrameworkReservedExactState({ platform, failures, plugin, actionBus, root, puzzle2d, store, semio, causal, vcs, allRustFiles, presentationEnvelopeCodec, presentationWasm, writerEnvelopeCodec, writerEditor, writerWasm, jackEnvelopeCodec, jackEditor, jackWasm, trinityRewrite, gisMapEnvelopeCodec, gisMapEditor, gisMapWasm, rasterEvidence, drawingEnvelopeCodec, drawingEditor, drawingEditorTestLaws, channel, presenceRetirement, kernel, reactor, pluginHost, componentWit, mcpWorkspace, runHost, wgpuHost, actor, shardHost, jobRuntime, nativeIo, fem2dSession, fem2dEditor, fem2dModel, femMesh, femPluginRoot, femGlue, reactorJobs, femAnalyses, reactorFamily, valueSnapshotRead, femSparse, femEngineModel, femElements2d, reactorRequests, reactorExecutor, productionFiles, staticRows, dispositions, proofs, proofIdentities, manifest, factoryContracts, globalPayloadStores, scanThenMonolithRows, appOwnedRows, appOwnedKeys, scanThenMonolithKeys, registeredAppOwnedKeys });
  return {
    macroHostFiles: macroFiles.size,
    macroInvocations,
    commandRows: commandRows.length,
    uniqueCommandRows: seenRows.size,
    duplicateRows,
    fixtureMacroHostFiles: fixtureMacroFiles.size,
    fixtureMacroInvocations,
    fixtureCommandRows: fixtureCommandRows.length,
    boundedRows: staticRows.length - remainingCommands.length - batchOnlyRows - forbiddenRows - deletedRows,
    batchOnlyRows,
    forbiddenRows,
    deletedRows,
    productionFactories: factoryContracts.length,
    productionRegistrations: appOwnedRows.length,
    productionDispatches: (plugin.match(/\.tool_jobs\s*\.dispatch\(/g) ?? []).length,
    aliases: (actionBus.match(/register_alias\(/g) ?? []).length,
    factoryContracts,
    frameworkReservedRoutes,
    reservedImporterOwners: reservedImporters.pending,
    globalPayloadStores,
    globalPayloadStoreExemptions,
    scanThenMonolithRows,
    literalRegistrations: staticRows.filter((row) => row.source === "literal").length,
    acceptedCommandRows,
    acceptedReservedRoutes: [
      ...(frameworkReservedExact ? TOOL_JOB_FRAMEWORK_RESERVED_IDS : []),
    ],
    remainingCommands,
    pipelineGates: {
      fullToolOperationJobBounded,
      durableOneItemPublicationBounded: storeOneItemPublicationBounded,
      ephemeralOneItemPublicationBounded,
      fullToolOperationBounded,
    },
    selfTests,
    failures,
  };
}
