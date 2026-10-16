/** 📦️ `@semio-tech/framework-kernel` — the package entry of the `🎠️kernel` module: plugin runtime, leases, invocation responses, and playground boot. It lists the public names of the kernel children (each child owns one concern) and re-exports the modules the kernel has always fronted; domain code lives in the children, never here. */

//#region 🎠️Kernel
export {
  descriptorPackageAssets,
  fetchDescriptorManifest,
  fetchPackageDescriptor,
  PLUGIN_DESCRIPTOR_CODE_UNIT_CAPACITY,
  pluginDescriptorUrl
} from "../../🛂️descriptor-admission/🟦️.ts";
export type { PluginPackageDescriptor } from "../../🛂️descriptor-admission/🟦️.ts";
export { createTurnOutcomeBroadcast } from "../../📡️turn-outcome-broadcast/🟦️.ts";
export type { TurnOutcome } from "../../📡️turn-outcome-broadcast/🟦️.ts";
export type { PluginDispatchHintV1, PluginWasmHandle } from "../../🪝️plugin-wasm-handle/🟦️.ts";
export { buildContributionsJson, resolveLayoutForMode, scopeContributionsJson } from "../../🧾️contributions-json/🟦️.ts";
export { expandPluginRegistry } from "../../🗂️plugin-registry/🟦️.ts";
export type { PluginDependency, PluginRegistryEntry, VersionPin } from "../../🗂️plugin-registry/🟦️.ts";
export type {
  ArtifactContributionDescriptor,
  ContributedInferenceMetadata,
  ContributedMutationMetadata,
  ContributedMutationSemantics
} from "../../🔖️artifact-contribution/🟦️.ts";
export { AppRouter, SURFACE_FAULT_CODES } from "../../🧭️app-router/🟦️.ts";
export type { AppRouterManifest } from "../../🧭️app-router/🟦️.ts";
export { ioConfidenceFromRank, IoEntryGraph, ioIdentify, ioRun } from "../../🚪️io-router/🟦️.ts";
export type { IoConfidence, IoEntryGraphPlugin, IoHopRunner, IoSniffRunner } from "../../🚪️io-router/🟦️.ts";
export {
  decodeOpeningConfigMutation,
  decodeOpeningPreferences,
  EMPTY_OPENING_PREFERENCES,
  foldOpeningPreferences,
  resolveOpeningApp
} from "../../📖️opening-resolver/🟦️.ts";
export type { DefaultApp, OpeningConfigMutation, OpeningPreferences } from "../../📖️opening-resolver/🟦️.ts";
export { artifactKindActivationOwner, ON_ARTIFACT_KIND_ACTIVATION_PREFIX, ON_EXTENSION_REQUEST_ACTIVATION_PREFIX } from "../../📚️plugin-catalog/🟦️.ts";
export type { PlaygroundCatalogTarget, PluginCatalog, PluginCatalogTarget, PluginHostConfig } from "../../📚️plugin-catalog/🟦️.ts";
export type {
  AppEvent,
  ArtifactDiff,
  EditRef,
  HybridLogicalTimestamp,
  InverseMutation,
  KernelMutation,
  UndoGroup,
  UndoPolicy
} from "../../🧬️mutation-vocabulary/🟦️.ts";
export { SemioFaultError, severityAsU8, severityFromU8 } from "../../🚨️fault/🟦️.ts";
export type { Diagnostic, Fault, FaultCause, FaultOrigin, FaultScope, Severity, TextSpan } from "../../🚨️fault/🟦️.ts";
export type { Effect, ServiceOperationPayloadV1 } from "../../⚡️effect/🟦️.ts";
export { MEDIA_EXPORT_BASE64_ENCODING, MEDIA_EXPORT_UTF8_ENCODING, mediaExportBytes, MediaExportEncodingError } from "../../🎬️media-export/🟦️.ts";
export {
  MEDIA_VIDEO_RENDER_CAPABILITY,
  VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE,
  VIDEO_RENDER_PROGRAM_MAXIMUM_FPS,
  VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES,
  VIDEO_RENDER_PROGRAM_SCHEMA,
  videoRenderCropIsAdmitted,
  videoRenderDurationMilliseconds,
  videoRenderFrameCount,
  videoRenderImageUrlIsAdmitted,
  videoRenderPathIsWellFormed,
  videoRenderProgramFromWire,
  videoRenderProgramProblem
} from "../../🎥️video-render-program/🟦️.ts";
export type {
  VideoRenderColor,
  VideoRenderImage,
  VideoRenderOp,
  VideoRenderPath,
  VideoRenderProgram,
  VideoRenderProgramErrorCode,
  VideoRenderRun,
  VideoRenderScene,
  VideoRenderTransform
} from "../../🎥️video-render-program/🟦️.ts";
export { VideoRenderJobLedger } from "../../🧵️video-render-job/🟦️.ts";
export type {
  VideoRenderEncoderTier,
  VideoRenderJobEvent,
  VideoRenderJobEventErrorCode,
  VideoRenderJobOutcome,
  VideoRenderJobRow
} from "../../🧵️video-render-job/🟦️.ts";
export {
  IMPORT_ARGUMENT_CHUNK,
  IMPORT_ARGUMENT_CHUNK_COUNT,
  IMPORT_ARGUMENT_INDEX,
  IMPORT_ARGUMENT_NAME,
  IMPORT_ARGUMENT_PAYLOAD,
  IMPORT_ARGUMENT_TOTAL,
  IMPORT_CHUNK_BYTES,
  importChunkArguments,
  importPayloadChunks
} from "../../🗄️file-open-import/🟦️.ts";
export type { ImportChunk } from "../../🗄️file-open-import/🟦️.ts";
export {
  FRAMEWORK_RESERVED_JOB_KIND,
  JOB_PLACEMENTS,
  jobPlacementFromWireName,
  SPAWNED_JOB_DEADLINE_MS,
  SPAWNED_JOB_FUEL,
  SPAWNED_JOB_STEP_CEILING,
  spawnedJobCompletion,
  SpawnedJobDriveError
} from "../../🛠️spawned-job/🟦️.ts";
export type { JobPlacement, SpawnedJobCompletion, SpawnedJobStep } from "../../🛠️spawned-job/🟦️.ts";
export {
  mergeUiDirtyScopes,
  partitionRefreshWindowInstancesV1,
  resolveUiDirtyScope,
  uiDirtyScopeAsksForNothing,
  uiDirtyScopeWantsCatalogue,
  uiDirtyScopeWantsPanelBody,
  uiDirtyScopeWantsSection,
  uiDirtyScopeWantsWindowBody,
  windowLayoutWindowIdsV1
} from "../../🎯️ui-dirty-scope/🟦️.ts";
export type { UiDirtyScope, UiDirtySection } from "../../🎯️ui-dirty-scope/🟦️.ts";
export { HISTORY_REPROJECTION_LABELS, historyEntryKey, historyEntryLabelText, historyReprojectionStatus } from "../../🕰️history/🟦️.ts";
export type {
  HistoryEntry,
  HistoryMutationEntry,
  HistoryMutationMessage,
  HistoryPatch,
  HistoryReprojection,
  HistoryReprojectionKind,
  HistoryReprojectionStatus,
  HistoryTimeTravel,
  HistoryTimeTravelProblem,
  HistoryTimeTravelReview,
  HistoryTimeTravelStage,
  HistoryTransaction
} from "../../🕰️history/🟦️.ts";
export { faultNotice, FRAMEWORK_FAULT_NOTICE_LABELS, frameworkFaultNotice, HISTORY_NOTICE_LABELS, historyNotice } from "../../🏷️notice-labels/🟦️.ts";
export { parseInvocationResponse } from "../../🧮️invocation-response/🟦️.ts";
export type { InvocationResponse } from "../../🧮️invocation-response/🟦️.ts";
export {
  conflictResolutionAsU8,
  conflictResolutionFromU8,
  DEFAULT_MERGE_POLICY,
  mergePolicyAsU8,
  mergePolicyFromU8,
  MUTATION_APPLY_ERROR_SCHEMA,
  MUTATION_APPLY_ERROR_WIRE_PARITY_VECTOR
} from "../../🔄️merge-outcome/🟦️.ts";
export type {
  Conflict,
  ConflictId,
  ConflictKind,
  ConflictResolution,
  ConflictStatus,
  DispatchReport,
  EditMessages,
  MergePolicy,
  MergeReport,
  MutationApplyError,
  MutationMessage
} from "../../🔄️merge-outcome/🟦️.ts";
export { postPluginBackboneInbound, registerPluginBackboneRoute, relayPluginBackboneOutbound } from "../../🌉️plugin-backbone-routing/🟦️.ts";
export { activationEventEnvelope, activationReasonForAppId } from "../../🚦️activation-reason/🟦️.ts";
export type { ActivationReason } from "../../🚦️activation-reason/🟦️.ts";
export { DEFAULT_MAX_RESIDENT_ACTORS, defaultMemoryProbe, residentActorCapFromMemory } from "../../🧯️memory-pressure/🟦️.ts";
export type { MemoryProbe, MemoryProbeReading } from "../../🧯️memory-pressure/🟦️.ts";
export { RUNTIME_METRICS_PUBLISH_INTERVAL_MS, runtimeMetricsDue } from "../../⏱️runtime-metrics/🟦️.ts";
export type { RuntimeMetricsActorRow, RuntimeMetricsSnapshot } from "../../⏱️runtime-metrics/🟦️.ts";
export { ActivationRegistry, intersectCapabilityGrants } from "../../🎛️activation-registry/🟦️.ts";
export type { ActivationManifestEntry, ActivationRegistryOptions } from "../../🎛️activation-registry/🟦️.ts";
export {
  createBundledPluginSource,
  createDevPluginSource,
  createExtensionSource,
  extensionRegistryFromCatalog,
  extensionSourceEventToPluginSourceEvent,
  multiplexPluginSources,
  PluginModuleUnavailableError
} from "../../🔌️plugin-source/🟦️.ts";
export type {
  PluginModuleAcquired,
  PluginModuleAcquisition,
  PluginModuleAcquisitionProgress,
  PluginSource,
  PluginSourceEvent,
  PluginSourceWatch
} from "../../🔌️plugin-source/🟦️.ts";
export {
  PlaygroundBootPlanner,
  PLUGIN_GRAPH_CHUNK_ROWS,
  resolvePlaygroundBoot,
  resolvePlaygroundDefaultAppId,
  resolvePluginHostConfig,
  resolvePluginRegistryId
} from "../../🎮️playground-boot/🟦️.ts";
export type { PlaygroundBoot, PlaygroundBootSession } from "../../🎮️playground-boot/🟦️.ts";
export {
  orderPluginRegistryEntries,
  pluginDependents,
  PluginGraph,
  pluginGraphErrorMessage,
  resolvePluginLoadOrder,
  validatePluginDependencyGraph,
  versionSatisfies
} from "../../🕸️plugin-graph/🟦️.ts";
export type { PluginGraphError, PluginGraphNode } from "../../🕸️plugin-graph/🟦️.ts";
export { InstanceDirectory } from "../../📍️instance-directory/🟦️.ts";
export type { ArtifactInstanceRef } from "../../📍️instance-directory/🟦️.ts";
export { ArtifactContributionNotPermittedError, ArtifactInferenceRouter, ArtifactMutationRouter, ArtifactRouterConflictError } from "../../🧲️artifact-routers/🟦️.ts";
export type { ArtifactRouterOwnership } from "../../🧲️artifact-routers/🟦️.ts";

export {readDescriptorText,reportDescriptorProgress,DescriptorTextLimitError} from "../../📇️descriptor/📥️text/🟦️.ts";
export type {DescriptorTextPolicy,DescriptorProgress} from "../../📇️descriptor/📥️text/🟦️.ts";
export {readPackageReceiptIdentity} from "../../📇️descriptor/📦️package/🟦️.ts";
export type {PackageReceiptIdentity,PackageReceiptRequest,PackageReceiptSource} from "../../📇️descriptor/📦️package/🟦️.ts";
export {CLIPBOARD_TEXT_MAX_BYTES,CLIPBOARD_METADATA_MAX_WIRE_BYTES,CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES,CLIPBOARD_PASTE_MAX_WIRE_BYTES,clipboardJsonStringBytes} from "../../📋️clipboard/🟦️.ts";
export { PACKAGE_ASSET_DIRECTORIES, PACKAGE_ASSET_ROUTE, fetchPackageAssets, packageAssetMounts, packageAssetUrl, readPackageAssetDeclarations, sha256Hex } from "../../📦️package-assets/🟦️.ts";
export type { PackageAssetDeclaration } from "../../📦️package-assets/🟦️.ts";
export { KernelReturnContentFraming, type KernelReturnContentMetadata, type KernelReturnContentByte } from "../../📤️return/📦️content/🟦️.ts";
export { KernelReturnUiOperationHeader, type KernelReturnUiOperationFields, type KernelReturnUiFieldName } from "../../📤️return/📦️content/🟦️.ts";
export { resolveExternalSlots, hasExternalSlots, ensureContributorInstance, retireContributorInstances, type ExternalSlotResolverContext, type ExternalSlotContributor, type ContributorInstance } from "../../🧩️extensions/🟦️.ts";
//#endregion 🎠️Kernel
