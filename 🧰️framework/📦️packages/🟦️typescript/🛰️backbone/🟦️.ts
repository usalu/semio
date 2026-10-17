/** 🧵️ SDK entry `@semio-tech/framework/backbone`: explicit public surface of the framework module that plugins build on. */
export { BACKBONE_ENDPOINT_PATH, BLOB_ENDPOINT_PATH, DEV_STREAM_ROUTES, FRAMEWORK_SYNC_CONTROLLER_ID, STREAM_MUX_ANNOUNCEMENT, STREAM_MUX_PATH, artifactBackboneRef, backboneKindFromUri, buildFileBackboneUri, buildFolderBackboneUri, buildFrameworkSyncUtilities, buildRemoteBackboneUri, decodeDocumentPackBytes, decodeDocumentPackSnapshot, encodeDocumentPackBundle, encodeDocumentPackBytes, parseRemoteBackboneUri } from "../../../🔨️modules/🛰️backbone/🟦️.ts";
export { resolveArtifactOpeningRelay } from "../../../🔨️modules/🛰️backbone/🧭️opening-relay/🟦️.ts";
export { BACKBONE_ENVELOPE_RETRY_WINDOW_MS, readBackboneEnvelope, writeBackboneEnvelope } from "../../../🔨️modules/🛰️backbone/🌐️envelope-io/🟦️.ts";
export { decodeBackboneMessage, encodeBackboneMessage, parseDocumentBackboneMessage, parseInboundDocumentBackboneMessage } from "../../../🔨️modules/🛰️backbone/🔀️message/🟦️.ts";
export { DESKTOP_WINDOW_CONTROL_CHANNELS, desktopWindowControlsBridge, registerDesktopWindowControlHandlers } from "../../../🔨️modules/🛰️backbone/🪟️desktop-window/🟦️.ts";
export { bindingsDataClass, documentRuntimeKeyV1, ephemeralLocalPersistenceBinding, ephemeralSharedPersistenceBinding, folderPersistenceBinding, hubPersistenceBinding, wireLaneDataClass } from "../../../🔨️modules/🛰️backbone/🔑️persistence-binding/🟦️.ts";
export { BACKBONE_WORKER_WIRE_MAGIC, DIRECTORY_SPACE_EVENTS_MAX_PER_MESSAGE, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse } from "../../../🔨️modules/🛰️backbone/🧱️worker-wire/🟦️.ts";
export { parseSpaceArtifactCreationCatalogRefreshRequiredV1, parseSpaceArtifactCreationCatalogStatusV1, parseSpaceArtifactCreationCatalogV1, parseSpaceArtifactCreationStatusV1 } from "../../../🔨️modules/🛰️backbone/🔬️worker-parsers/🟦️.ts";
export { planWorkflow } from "../../../🔨️modules/🛰️backbone/🗺️workflow-planner/🟦️.ts";
export type { ArtifactBackboneRef, BackboneKind, FrameworkSyncUtilityLeaf } from "../../../🔨️modules/🛰️backbone/🟦️.ts";
export type { ResolvedArtifactOpeningRelay } from "../../../🔨️modules/🛰️backbone/🧭️opening-relay/🟦️.ts";
export type { BinaryBackboneMessage, DocumentBackboneMessage } from "../../../🔨️modules/🛰️backbone/🔀️message/🟦️.ts";
export type { DesktopWindowControls } from "../../../🔨️modules/🛰️backbone/🪟️desktop-window/🟦️.ts";
export type { ClassifiedWireLane, DocumentRuntimeScopeV1, PersistenceBinding, PersistenceDataClass } from "../../../🔨️modules/🛰️backbone/🔑️persistence-binding/🟦️.ts";
export type { ArtifactActorConfig, ArtifactActorMsg, ArtifactEvent, ArtifactSyncStatus, BrowserActorUiMountedV1, RemoteState, SyncConflict } from "../../../🔨️modules/🛰️backbone/🎭️artifact-actor/🟦️.ts";
export type { BackboneWorkerWireMessage } from "../../../🔨️modules/🛰️backbone/🧱️worker-wire/🟦️.ts";
export type { ArtifactBootstrapWorkerEvent, BackboneWorkerRequest, BackboneWorkerResponse, DirectoryAdministrationInviteCapabilityStatusV1, DirectoryAdministrationPhaseV1, DirectoryAdministrationSectionV1, SpaceArtifactCreationCatalogPhaseV1, SpaceArtifactCreationCatalogRefreshRequiredV1, SpaceArtifactCreationCatalogStatusV1, SpaceArtifactCreationCatalogV1, SpaceArtifactCreationKindV1, SpaceArtifactCreationPhaseV1, SpaceArtifactCreationReadyV1, SpaceArtifactCreationStatusV1 } from "../../../🔨️modules/🛰️backbone/📨️worker-messages/🟦️.ts";
export type { MediaContract, OsMediaPort, OsWorkflow, OsWorkflowEdge, OsWorkflowNode, WorkflowDelivery } from "../../../🔨️modules/🛰️backbone/🗺️workflow-planner/🟦️.ts";
