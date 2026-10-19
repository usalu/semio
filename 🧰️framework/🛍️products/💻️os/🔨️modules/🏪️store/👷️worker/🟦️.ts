

import { LOCAL_CATALOG_CONFIG_SCHEMA, applyIdentityDiff, applyLocalCatalogDiff, decodeIdentityDiff, decodeIdentityValue, decodeLocalCatalogDiff, decodeLocalCatalogValue } from "@semio-tech/framework/config/schema/mutations";
import {admitCheckpointSelectionV1} from "../../📇️directory/🚪️io/🧱️binary/🪢️checkpoint-pair/🟦️.ts";
import { parseDirectorySessionAuthorityJsonV1 } from "@semio-tech/framework/directory/io/text/session-authority-v1";
import { parseDirectoryEventPageV1 } from "@semio-tech/framework/directory/io/text";
import { parseSpaceArtifactCreationCatalogJsonV1, parseSpaceArtifactCreationStatusJsonV1 } from "../../📇️directory/🚪️io/📝️text/🌱️space-artifact-creation-v1/🟦️.ts";
import { documentCheckInCanonicalJson, parseDocumentCheckInStatusV1 } from "../../📇️directory/🚪️io/📝️text/📌️document-check-in-v1/🟦️.ts";
import { directoryAdministrationCommandAllowedV1, directoryCommandRequestJson, directoryCommandSha256, sealDirectoryCommandReceiptV1, sealDirectoryCommandRequestV1 } from "@semio-tech/framework/directory/io/text";
import { CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1, decodeCanonicalCheckpointPairV1 } from "../../📇️directory/🚪️io/🧱️binary/🪢️checkpoint-pair/🟦️.ts";
import { DOCUMENT_SERVICE_TOPIC_V1, InstalledServiceRegistryV1, type InstalledServiceDriverV1, type AdmittedDocumentServiceDeclarationV1 } from "@semio-tech/framework/inference/service";
import {admitDocumentServiceWireDeclarationV1, documentServiceRequestV1, admitDocumentServiceResponseV1} from "@semio-tech/framework/inference/service/io";
import type { DocumentServiceDriverContextV1, DocumentServiceWorkerHostV1 } from "@semio-tech/framework/inference/service/worker-host";
// #region Header
/**
 * 🧵️ `🏪️store/👷️worker/🟦️.ts` — browser backbone loader. Authenticated hub
 * document lifecycles are owned here so D1 issue/exchange/WebSocket authority cannot be bypassed
 * when the Rust WASM worker resolves; other lanes use Rust when available and the TypeScript twin
 * otherwise.
 */
// #endregion Header

import { DIRECTORY_SESSION_AUTHORITY_MAX_BYTES, type DirectorySessionAuthorityV1 } from "@semio-tech/framework/directory/schema/session-authority-v1";
import { DIRECTORY_PREFERENCE_PAGE_PATH_V1, type DirectoryEvent } from "@semio-tech/framework/directory/schema";
import { directoryStreamWakeV1 } from "@semio-tech/framework/directory";
import { StreamMuxPortEndpointV1, type StreamMuxEndpointV1 } from "@semio-tech/framework/io/stream-mux";

import type {
  ArtifactBootstrapControl,
  ArtifactBootstrapProgress,
  ArtifactPresencePeer,
  ClientFrame,
  ExactWireMutationEnvelope,
  MutationEnvelope,
  ServerFrame,
  WireAckStage,
  WireArtifactBootstrap,
  WireFrontierSummary,
  WireLane,
  WireMutationEnvelope,
} from "@semio-tech/framework-replication";
import type {
  ArtifactActorConfig,
  ArtifactActorMsg,
  ArtifactBootstrapWorkerEvent,
  ArtifactEvent,
  ArtifactSyncStatus,
  BackboneWorkerRequest,
  BackboneWorkerResponse,
  BackboneWorkerWireMessage,
  HubSessionPortResponseV1,
  CanonicalDirectoryEventPageV1,
  CommandAckOutcome,
  DirectoryAcknowledgedStream,
  DirectoryAdministrationPhaseV1,
  DirectoryCommand,
  DirectoryEventPageAckV1,
  DirectoryStreamMessage,
  DocumentScope,
  DocumentLink,
  DocumentLinkStatusCode,
  PersistenceBinding,
  RemoteState,
  DocumentSocketGrantReceiptV1,
  SocketGrantReceiptV1,
} from "../../../🟦️";
import { ArtifactBootstrapAssembler, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, DOCUMENT_BACKBONE_BATCH_LIMITS, DOCUMENT_BACKBONE_RETENTION_LIMITS, HISTORY_TRANSITION_DIFF_SCHEMA, decodeClientFrame, encodeClientCommandsFrameExact, decodePresenceHistoryEdit, decodePresenceInteraction, decodePresencePeer, decodePresenceToolRun, decodeServerFrame, decodeDocumentBackboneEnvelopeBatchExact, encodeClientFrame, encodeDocumentBackboneEnvelopeBatchExact, encodePresencePeer, encodeServerFrame, extractServerCommandsDocumentBackboneBatchExact } from "@semio-tech/framework-replication";
import { DEV_STREAM_ROUTES, DirectoryClient, DirectoryCommandError, DirectoryHttpError, DOCUMENT_LINK_ACCESS_REFUSED_STATUSES, admitRemoteEnvelopes, noteAuthoredEnvelopeIds, hubCommandRejectionV1, hubTransientApplyRefusalV1, localCommandRejectionV1, HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS, createSocketGrantIssuerV1, documentLinkExpiresAtMs, documentLinkStatus, documentLinkTransition, decodeBackboneMessage, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, decodeDocumentArchiveBytes, decodeAppCommand, decodePackWire, decodePackValue, packValueToExactJson, faultDisplayMessage, documentRuntimeKeyV1, encodeBackboneMessage, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse, encodeDocumentArchiveBytes, encodeAppCommand, encodePackValue, isPackInteger, packUIntSafeOrNull, packWireNatural, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, DOCUMENT_ARCHIVE_MAXIMUM_BYTES, HUB_SESSION_CAPABILITY_PATTERN_V1, parseHubSessionPortRequestV1, parseDocumentBackboneMessage, parseInboundDocumentBackboneMessage, parseDocumentSocketGrantReceiptV1, parseSocketGrantReceiptV1 } from "../../../🟦️";
import type { PackValue } from "../../../🟦️";
import { SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES, SPACE_ARTIFACT_CREATION_MAX_BYTES, sealSpaceArtifactCreateV1, type SpaceArtifactCreationCatalogV1 as HubSpaceArtifactCreationCatalogV1, type SpaceArtifactCreationStatusV1 as HubSpaceArtifactCreationStatusV1 } from "../../📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
import { browserActorChildCapacity, reserveBrowserActorChild, type BrowserActorChildValue } from "../../🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts";
import { DOCUMENT_ACTOR_RECOVERY_FRESH_V1, DOCUMENT_ACTOR_RECOVERY_V1, documentActorRecoveryStepV1, type DocumentActorLossCauseV1, type DocumentActorRecoveryMemoryV1 } from "./🚑️actor-recovery/🟦️.ts";
import { assertBrowserActorDescribeCapacityV1, verifyBrowserActorDescribeV1 } from "../../🔌️plugin/🌐️browser-bundle/🧾️describe/🟦️.ts";
import { BROWSER_ACTOR_CHILD_LIMITS, COMMAND_INGRESS_KINDS, boundChildText, measureChildValue, type CommandIngressKindV1 } from "@semio-tech/framework/browser-bundle/child/schema";
import { coldDocumentPairCursorEquals, coldDocumentPairFrontierEquals, parseColdDocumentPairLifetime, parseWitColdPairIngressStatus, type ColdDocumentPairCursor, type ColdDocumentPairFrontier, type ColdPairIngressStatus } from "../../../../../🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts";
import { createShardCommandIngressPages, type ShardCommandIngressPage } from "../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { driveSpawnedJob, spawnedJobCompletedEvent, typedOperationPageAnswerV1, typedOperationResult, wireSpawnJob } from "../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import { actorInstanceCapturedReceiptMatches, actorInstanceCloseReceiptMatches, actorInstanceLifetimeEquals, type ActorInstanceCloseRequest, type ActorInstanceLifecycleReceipt, type ActorInstanceLifetime, type ActorInstanceOpenRequest } from "../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import { encodeActorUiPatchReceipt, type ActorUiPatchReceipt } from "../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts";
import { BROWSER_ACTOR_UI_PATCH_SURFACE_MAXIMUM, browserActorUiPatchOwnerMatchesV1, captureBrowserActorUiPatchV1, type BrowserActorUiPatchOfferV1, type BrowserActorUiPatchResultV1 } from "@semio-tech/framework/browser-bundle/patch-handoff";
import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION, BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM, browserActorAdmissionRefusalReasonV1, browserActorGuestRefusalReasonV1, parseBrowserActorActionRequestV1, parseBrowserActorHistoryPatchBytesV1, parseBrowserActorHostEffectBytesV1, type BrowserActorActionCommitReceiptV1, type BrowserActorActionRequestV1, type BrowserActorActionResultV1 } from "@semio-tech/framework/browser-bundle/action-handoff";
import { decodeBrowserActorCommandPublicationV1, decodeBrowserActorIntentPublicationV1, decodeBrowserActorUnsolicitedPublicationV1, encodeBrowserActorHostEffectV1, requireBrowserActorCommandBackboneProjectionV1, type BrowserActorCommandBackboneEnvelopeV1, type BrowserActorCommandPublicationV1, type BrowserActorEphemeralSnapshotV1 } from "@semio-tech/framework/browser-bundle/action-handoff/publication";
import { ActorDocumentBindingV1, documentBackboneEffectV1, encodeDocumentBackboneControlV1 } from "@semio-tech/framework/backbone/binding";
import { parseBrowserActorViewStateRequest } from "@semio-tech/framework/browser-bundle/view-context";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "@semio-tech/framework/directory/schema";
import { parseDocumentExecutionTargetLeaseFieldsV1, sameLeaseFieldsV1 } from "../../📇️directory/🧬️schema/🔏️document-open/🟦️.ts";
import { DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1, documentExecutionTargetStatusRoleV1 } from "@semio-tech/framework/directory/schema";
/** 🪪️ Identity and local-catalog config lanes (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS
 * §C3) — the neutral backbone module owns the actor configs and the identity fold; never redefined here. */
import { IDENTITY_CONFIG_SCHEMA, foldIdentityEvent, identityActorConfig } from "@semio-tech/framework/backbone/config-lane";
import { COLD_PAIR_WAIT_POLICY, browserActorActionDisposition, browserActorAppCommandV1, coldPairWaitDelay, documentRuntimeKeyForConfig, hubBinding, retryColdPairBackpressure, settleColdPairLoading } from "./🧩️actor-wire-helpers/🟦️.ts";
import { ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES, PENDING_MUTATIONS_QUEUE_LIMIT, SANITY_POLL_MIN_MS, SUSTAINED_HEALTHY_MS, documentExecutionTargetLeaseMintToken, workerScope } from "./⚙️constants/🟦️.ts";
import { artifactState, artifacts, documentServices, dropVerifiedColdDocumentPair, emitEvent, post, setWorkerPostTestSink, workerPostTestSink } from "./🧾️document-state/🟦️.ts";
import { DirectoryEventPageBootstrapV1, acceptBrowserSessionAuthority, attachHubSessionPort, browserDirectoryRequest, browserSessionAuthority, browserSessionOperationFence, bytesHex, captureBrowserSessionOperationFence, clearHubSessionCapability, closeDirectory, detachHubSessionPort, directoryAdministration, directoryBootstrap, directoryClient, directorySessionEpoch, directoryWorkerEpoch, documentSocketGrantTestIssue, flushDirectoryQueue, hubSessionCapability, hubSessionFetch, hubSessionQueued, installHubSessionCapability, loadDirectoryAdministrationPage, openDirectory, openDirectoryAdministration, openDirectoryBootstrap, setDirectoryAdministration, setDirectoryClient, setDirectorySessionEpoch, setDocumentSocketGrantTestIssue, setHubSessionQueued, setSocketGrantTestIssue, settleDirectoryCommand, socketGrantTestIssue } from "./🔐️hub-session/🟦️.ts";
import { VerifiedColdDocumentPair, verifiedColdDocumentPairMintToken } from "./🧊️cold-pair/🟦️.ts";
import { DocumentExecutionTargetLease, browserExecutionTargetAssetRequest } from "./🪪️execution-lease/🟦️.ts";
import { dropDocumentExecutionTargetLease, executionTargetHex, executionTargetSha256Hex, executionTargetStatusObserver, ownedArrayBuffer, readExecutionTargetBody, requestDocumentActorRecoveryV1, setExecutionTargetStatusObserver } from "./📥️execution-target-read/🟦️.ts";
import "./🧷️actor-reservation/🟦️.ts";
import { documentOpenPlanAuthority, requestDocumentSocketAuthority } from "./🎭️actor-lifecycle/🟦️.ts";
import { reserveDocumentBrowserActorChild } from "./🎬️actor-activation/🟦️.ts";
import { reconcileDocumentServiceContributionsV1 } from "./🗂️config-lane/🟦️.ts";
import { decodePackPayload, fromWireEnvelope, rollbackEnvelope, stampSession, toWireEnvelope } from "./🌉️wire-bridge/🟦️.ts";
import { installStreamMuxEndpoint } from "./📁️folder/🟦️.ts";
import { connectHubOnce } from "./🔌️hub-connection/🟦️.ts";
import { documentCatchingUpV1, relayMutationsToHub } from "./🛰️hub-relay/🟦️.ts";
import { queueOutbox } from "./📮️hub-outbox/🟦️.ts";
import { handleAck } from "./🧮️hub-ack/🟦️.ts";
import "./🚀️artifact-bootstrap/🟦️.ts";
import { abortArtifactBootstrap, artifactBootstrapFailure, closeArtifactRuntime, directoryCommandOperations, directoryCommandQueue, scopedDirectoryStreams, spaceArtifactCreationCatalogOperations, spaceArtifactCreationOperations } from "./🛑️session-teardown/🟦️.ts";
import { handleHubFrame } from "./📡️hub-frames/🟦️.ts";
import { DIRECTORY_COMMAND_TRANSPORT_CAPACITY, cancelSpaceArtifactCreation, openSpaceArtifactCreationCatalog, setSpaceArtifactCreationTestFetch, spaceArtifactCreationTestFetch, submitSpaceArtifactCreation } from "./📇️space-artifact-creation/🟦️.ts";
import { cancelDocumentCheckIn, submitDocumentCheckIn } from "./📌️document-check-in/🟦️.ts";
import { acknowledgeDirectoryBootstrap, cancelDirectoryCommand, closeScopedDirectory, closeSpaceDirectory, openScopedDirectory, openSpaceDirectory, rejectDirectoryBootstrap, submitDirectoryCommand } from "./🧭️directory-bootstrap/🟦️.ts";
import { closePreferenceLane, openPreferenceLane, recordPreferenceLaneChange } from "./🌐️preference-lane/🟦️.ts";
import { closeDirectoryAdministration, requestDirectoryAdministrationCapability, revokeDirectoryAdministrationForScope, settleDirectoryAdministrationCapability, submitDirectoryAdministrationCommand } from "./🛡️directory-administration/🟦️.ts";
import "./🧱️blob-cache/🟦️.ts";
import { closeArtifact, handleLocalMsg, newArtifactState } from "./♻️artifact-lifecycle/🟦️.ts";
import { openArtifact } from "./🚪️artifact-open/🟦️.ts";

export type RustWorkerHost = {
  handleRequestBytes(bytes: Uint8Array): void;
  postReady(): void;
};

type DocumentExecutionOwner = "typescript" | "rust";
type DocumentExecutionOwnerEntry = Readonly<{ owner: DocumentExecutionOwner; documentId: string; clientInstanceId: string; spaceId?: string }>;

const documentExecutionOwners = new Map<string, DocumentExecutionOwnerEntry>();

function validDocumentOpeningAttemptId(value: unknown): value is string {
  return typeof value === "string" && /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u.test(value);
}

function ownedDocumentRuntimeKey(documentId: string, spaceId?: string): string | null {
  if (spaceId !== undefined) return documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId, documentId });
  const localKey = documentRuntimeKeyV1({ kind: "local", documentId });
  if (documentExecutionOwners.has(localKey)) return localKey;
  const matches = [...documentExecutionOwners].filter(([, entry]) => entry.documentId === documentId);
  return matches.length === 1 ? matches[0]![0] : matches.length === 0 ? localKey : null;
}

/** 🛡️ Keeps one document's open/send/close lifecycle on a single execution owner and
 * reserves every hub-bound document for the authenticated browser D1 transport. */
function dispatchBackboneWorkerRequest(request: BackboneWorkerRequest, host: RustWorkerHost | null, typescriptDispatch: (request: BackboneWorkerRequest) => void = handleTsRequest): void {
  const rustDispatch = (value: BackboneWorkerRequest): void => host?.handleRequestBytes(encodeBackboneWorkerRequest(value));
  if (request.kind === "directory-bootstrap-open" || request.kind === "directory-bootstrap-ack" || request.kind === "directory-bootstrap-reject" || request.kind === "directory-bootstrap-close" || request.kind === "directory-space-open" || request.kind === "directory-space-close" || request.kind === "preference-lane-open" || request.kind === "preference-lane-record" || request.kind === "preference-lane-close") {
    typescriptDispatch(request);
    return;
  }
  if (request.kind === "browser-actor-action" || request.kind === "browser-actor-ui-patch-result" || request.kind === "browser-actor-view-state") {
    typescriptDispatch(request);
    return;
  }
  if (request.kind === "service-operation" || request.kind === "service-contributions") { typescriptDispatch(request); return; }
  if (request.kind === "open") {
    const clientInstanceId = request.clientInstanceId ?? crypto.randomUUID();
    if (!validDocumentOpeningAttemptId(clientInstanceId)) return;
    const next: DocumentExecutionOwner = hubBinding(request) === null && host !== null ? "rust" : "typescript";
    const runtimeKey = documentRuntimeKeyForConfig(request);
    const previous = documentExecutionOwners.get(runtimeKey);
    if (previous?.clientInstanceId === clientInstanceId) return;
    if (previous !== undefined) {
      const close: BackboneWorkerRequest = { kind: "close", documentId: request.documentId, clientInstanceId: previous.clientInstanceId, ...(previous.spaceId === undefined ? {} : { spaceId: previous.spaceId }) };
      if (previous.owner === "typescript") typescriptDispatch(close);
      else rustDispatch(close);
    }
    const hub = hubBinding(request);
    const openedRequest: BackboneWorkerRequest = { ...request, clientInstanceId };
    documentExecutionOwners.set(runtimeKey, { owner: next, documentId: request.documentId, clientInstanceId, ...(hub === null ? {} : { spaceId: hub.spaceId }) });
    if (next === "typescript") typescriptDispatch(openedRequest);
    else rustDispatch(openedRequest);
    return;
  }
  if (request.kind === "send" || request.kind === "close") {
    const runtimeKey = ownedDocumentRuntimeKey(request.documentId, request.spaceId);
    if (runtimeKey === null) return;
    const current = documentExecutionOwners.get(runtimeKey);
    if (current !== undefined && request.clientInstanceId !== current.clientInstanceId) return;
    const owner = current?.owner ?? (host === null ? "typescript" : "rust");
    if (owner === "typescript") typescriptDispatch(request);
    else rustDispatch(request);
    if (request.kind === "close" && current !== undefined && request.clientInstanceId === current.clientInstanceId) documentExecutionOwners.delete(runtimeKey);
    return;
  }
  if (host === null) typescriptDispatch(request);
  else rustDispatch(request);
}

function isBackboneWorkerWireMessage(message: unknown): message is BackboneWorkerWireMessage {
  return typeof message === "object" && message !== null && "wire" in message && (message as BackboneWorkerWireMessage).wire instanceof Uint8Array;
}

function decodeWorkerRequest(message: BackboneWorkerWireMessage): BackboneWorkerRequest {
  return decodeBackboneWorkerRequest(message.wire);
}

if (workerScope) {
  workerScope.onmessage = (messageEvent: MessageEvent<unknown>) => {
    if (typeof messageEvent.data === "object" && messageEvent.data !== null && Reflect.get(messageEvent.data, "kind") === "semio-hub-session-port" && Reflect.get(messageEvent.data, "port") instanceof MessagePort) {
      attachHubSessionPort(Reflect.get(messageEvent.data, "port") as MessagePort);
      return;
    }
    if (typeof messageEvent.data === "object" && messageEvent.data !== null && Reflect.get(messageEvent.data, "kind") === "semio-stream-mux-port" && Reflect.get(messageEvent.data, "port") instanceof MessagePort) {
      installStreamMuxEndpoint(new StreamMuxPortEndpointV1(Reflect.get(messageEvent.data, "port") as MessagePort));
      return;
    }
    // 🛡️ React DevTools and other injectors postMessage into every Worker; ignore non-wire traffic.
    if (!isBackboneWorkerWireMessage(messageEvent.data)) return;
    const request = decodeWorkerRequest(messageEvent.data);
    queueMicrotask(() => dispatchBackboneWorkerRequest(request, null));
  };
}

queueMicrotask(() => post({ kind: "ready" }));

/** 🧵️ Exact shape of the mutable seam bag the worker hands its extracted test module; every member is a `typeof` of the live binding, so it cannot drift. */
export type BackboneWorkerTestSeams = {
  directoryAdministration: typeof directoryAdministration;
  directoryClient: typeof directoryClient;
  directorySessionEpoch: typeof directorySessionEpoch;
  executionTargetStatusObserver: typeof executionTargetStatusObserver;
  hubSessionQueued: typeof hubSessionQueued;
  socketGrantTestIssue: typeof socketGrantTestIssue;
  documentSocketGrantTestIssue: typeof documentSocketGrantTestIssue;
  spaceArtifactCreationTestFetch: typeof spaceArtifactCreationTestFetch;
  workerPostTestSink: typeof workerPostTestSink;
  readonly browserSessionAuthority: typeof browserSessionAuthority;
  readonly hubSessionCapabilityHeld: boolean;
  readonly browserSessionOperationFence: typeof browserSessionOperationFence;
  readonly acceptBrowserSessionAuthority: typeof acceptBrowserSessionAuthority;
  readonly captureBrowserSessionOperationFence: typeof captureBrowserSessionOperationFence;
  readonly attachHubSessionPort: typeof attachHubSessionPort;
  readonly detachHubSessionPort: typeof detachHubSessionPort;
  readonly browserActorAppCommandV1: typeof browserActorAppCommandV1;
};

/** 🧪️ Exact shape of the dependency bag `🧪️tests/🧪️space-artifact-creation-owner` receives from this module. */
export type BackboneWorkerTestDependencies = {
  readonly testSeams: BackboneWorkerTestSeams;
  readonly DOCUMENT_BACKBONE_RETENTION_LIMITS: typeof DOCUMENT_BACKBONE_RETENTION_LIMITS;
  readonly handleAck: typeof handleAck;
  readonly ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES: typeof ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES;
  readonly ArtifactBootstrapAssembler: typeof ArtifactBootstrapAssembler;
  readonly DIRECTORY_COMMAND_TRANSPORT_CAPACITY: typeof DIRECTORY_COMMAND_TRANSPORT_CAPACITY;
  readonly DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1: typeof DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1;
  readonly DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1: typeof DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1;
  readonly DirectoryClient: typeof DirectoryClient;
  readonly DirectoryEventPageBootstrapV1: typeof DirectoryEventPageBootstrapV1;
  readonly DocumentExecutionTargetLease: typeof DocumentExecutionTargetLease;
  readonly HUB_RECONNECT_MAX_MS: typeof HUB_RECONNECT_MAX_MS;
  readonly IDENTITY_CONFIG_SCHEMA: typeof IDENTITY_CONFIG_SCHEMA;
  readonly PENDING_MUTATIONS_QUEUE_LIMIT: typeof PENDING_MUTATIONS_QUEUE_LIMIT;
  readonly SANITY_POLL_MIN_MS: typeof SANITY_POLL_MIN_MS;
  readonly SUSTAINED_HEALTHY_MS: typeof SUSTAINED_HEALTHY_MS;
  readonly VerifiedColdDocumentPair: typeof VerifiedColdDocumentPair;
  readonly abortArtifactBootstrap: typeof abortArtifactBootstrap;
  readonly installStreamMuxEndpoint: typeof installStreamMuxEndpoint;
  readonly artifactBootstrapFailure: typeof artifactBootstrapFailure;
  readonly artifactState: typeof artifactState;
  readonly artifacts: typeof artifacts;
  readonly browserActorChildCapacity: typeof browserActorChildCapacity;
  readonly hubSessionFetch: typeof hubSessionFetch;
  readonly browserDirectoryRequest: typeof browserDirectoryRequest;
  readonly browserExecutionTargetAssetRequest: typeof browserExecutionTargetAssetRequest;
  readonly bytesHex: typeof bytesHex;
  readonly clearHubSessionCapability: typeof clearHubSessionCapability;
  readonly closeArtifact: typeof closeArtifact;
  readonly closeArtifactRuntime: typeof closeArtifactRuntime;
  readonly closeDirectory: typeof closeDirectory;
  readonly connectHubOnce: typeof connectHubOnce;
  readonly decodeBackboneWorkerRequest: typeof decodeBackboneWorkerRequest;
  readonly decodeBackboneWorkerResponse: typeof decodeBackboneWorkerResponse;
  readonly decodeClientFrame: typeof decodeClientFrame;
  readonly decodePackPayload: typeof decodePackPayload;
  readonly decodePackValue: typeof decodePackValue;
  readonly decodeServerFrame: typeof decodeServerFrame;
  readonly directoryAdministration: typeof directoryAdministration;
  readonly directoryClient: typeof directoryClient;
  readonly directoryCommandOperations: typeof directoryCommandOperations;
  readonly directoryCommandQueue: typeof directoryCommandQueue;
  readonly directoryCommandSha256: typeof directoryCommandSha256;
  readonly directorySessionEpoch: typeof directorySessionEpoch;
  readonly directoryWorkerEpoch: typeof directoryWorkerEpoch;
  readonly dispatchBackboneWorkerRequest: typeof dispatchBackboneWorkerRequest;
  readonly documentExecutionOwners: typeof documentExecutionOwners;
  readonly documentExecutionTargetLeaseMintToken: typeof documentExecutionTargetLeaseMintToken;
  readonly documentExecutionTargetStatusRoleV1: typeof documentExecutionTargetStatusRoleV1;
  readonly documentOpenPlanAuthority: typeof documentOpenPlanAuthority;
  readonly documentRuntimeKeyForConfig: typeof documentRuntimeKeyForConfig;
  readonly newArtifactState: typeof newArtifactState;
  readonly documentCatchingUpV1: typeof documentCatchingUpV1;
  readonly requestDocumentActorRecoveryV1: typeof requestDocumentActorRecoveryV1;
  readonly documentRuntimeKeyV1: typeof documentRuntimeKeyV1;
  readonly dropDocumentExecutionTargetLease: typeof dropDocumentExecutionTargetLease;
  readonly dropVerifiedColdDocumentPair: typeof dropVerifiedColdDocumentPair;
  readonly emitEvent: typeof emitEvent;
  readonly encodeActorUiPatchReceipt: typeof encodeActorUiPatchReceipt;
  readonly encodeBackboneMessage: typeof encodeBackboneMessage;
  readonly encodeBackboneWorkerRequest: typeof encodeBackboneWorkerRequest;
  readonly encodeBackboneWorkerResponse: typeof encodeBackboneWorkerResponse;
  readonly encodeDocumentBackboneEnvelopeBatchExact: typeof encodeDocumentBackboneEnvelopeBatchExact;
  readonly encodePackValue: typeof encodePackValue;
  readonly encodeServerFrame: typeof encodeServerFrame;
  readonly executionTargetHex: typeof executionTargetHex;
  readonly executionTargetSha256Hex: typeof executionTargetSha256Hex;
  readonly executionTargetStatusObserver: typeof executionTargetStatusObserver;
  readonly extractServerCommandsDocumentBackboneBatchExact: typeof extractServerCommandsDocumentBackboneBatchExact;
  readonly flushDirectoryQueue: typeof flushDirectoryQueue;
  readonly foldIdentityEvent: typeof foldIdentityEvent;
  readonly fromWireEnvelope: typeof fromWireEnvelope;
  readonly handleHubFrame: typeof handleHubFrame;
  readonly handleTsRequest: typeof handleTsRequest;
  readonly hubBinding: typeof hubBinding;
  readonly identityActorConfig: typeof identityActorConfig;
  readonly installHubSessionCapability: typeof installHubSessionCapability;
  readonly hubSessionQueued: typeof hubSessionQueued;
  readonly openArtifact: typeof openArtifact;
  readonly ownedArrayBuffer: typeof ownedArrayBuffer;
  readonly parseDocumentBackboneMessage: typeof parseDocumentBackboneMessage;
  readonly parseDocumentExecutionTargetLeaseFieldsV1: typeof parseDocumentExecutionTargetLeaseFieldsV1;
  readonly queueOutbox: typeof queueOutbox;
  readonly readExecutionTargetBody: typeof readExecutionTargetBody;
  readonly relayMutationsToHub: typeof relayMutationsToHub;
  readonly requestDocumentSocketAuthority: typeof requestDocumentSocketAuthority;
  readonly reserveDocumentBrowserActorChild: typeof reserveDocumentBrowserActorChild;
  readonly revokeDirectoryAdministrationForScope: typeof revokeDirectoryAdministrationForScope;
  readonly rollbackEnvelope: typeof rollbackEnvelope;
  readonly sameLeaseFieldsV1: typeof sameLeaseFieldsV1;
  readonly scopedDirectoryStreams: typeof scopedDirectoryStreams;
  readonly sealDirectoryCommandReceiptV1: typeof sealDirectoryCommandReceiptV1;
  readonly sealDirectoryCommandRequestV1: typeof sealDirectoryCommandRequestV1;
  readonly settleDirectoryCommand: typeof settleDirectoryCommand;
  readonly socketGrantTestIssue: typeof socketGrantTestIssue;
  readonly spaceArtifactCreationCatalogOperations: typeof spaceArtifactCreationCatalogOperations;
  readonly spaceArtifactCreationOperations: typeof spaceArtifactCreationOperations;
  readonly spaceArtifactCreationTestFetch: typeof spaceArtifactCreationTestFetch;
  readonly stampSession: typeof stampSession;
  readonly toWireEnvelope: typeof toWireEnvelope;
  readonly verifiedColdDocumentPairMintToken: typeof verifiedColdDocumentPairMintToken;
  readonly verifyBrowserActorDescribeV1: typeof verifyBrowserActorDescribeV1;
  readonly workerPostTestSink: typeof workerPostTestSink;
};

//#region 🔖️MessageBridge
function handleTsRequest(request: BackboneWorkerRequest): void {
  switch (request.kind) {
    case "open":
      openArtifact(request);
      break;
    case "close":
      closeArtifact(request.documentId, request.spaceId, request.clientInstanceId);
      break;
    case "send": {
      const state = artifactState(request.documentId, request.spaceId);
      if (state && request.clientInstanceId === state.openClientInstanceId) void handleLocalMsg(state, request.message);
      break;
    }
    case "directory-open":
      openDirectory(request.baseUrl, request.since);
      break;
    case "directory-bootstrap-open":
      openDirectoryBootstrap(request.baseUrl, request.after, request.bootstrapEpoch);
      break;
    case "preference-lane-open":
      openPreferenceLane(request.baseUrl, request.userId, request.after);
      break;
    case "preference-lane-record":
      void recordPreferenceLaneChange(request.requestId, request.schema, request.mutation);
      break;
    case "preference-lane-close":
      closePreferenceLane();
      break;
    case "directory-bootstrap-ack":
      acknowledgeDirectoryBootstrap(request);
      break;
    case "directory-bootstrap-reject":
      rejectDirectoryBootstrap(request.bootstrapEpoch, request.receiptSha256);
      break;
    case "directory-bootstrap-close":
      if (directoryBootstrap?.machine.bootstrapEpoch === request.bootstrapEpoch) closeDirectory();
      break;
    case "directory-scope-open":
      openScopedDirectory(request.baseUrl, request.scope, request.since);
      break;
    case "directory-scope-close":
      closeScopedDirectory(request.scope);
      break;
    case "directory-space-open":
      openSpaceDirectory(request.baseUrl, request.spaceId);
      break;
    case "directory-space-close":
      closeSpaceDirectory(request.spaceId);
      break;
    case "directory-command":
      void submitDirectoryCommand(request.requestId, request.command);
      break;
    case "directory-command-cancel":
      cancelDirectoryCommand(request.requestId);
      break;
    case "space-artifact-creation-catalog-open":
      void openSpaceArtifactCreationCatalog(request.spaceId, request.clientInstanceId);
      break;
    case "space-artifact-create":
      submitSpaceArtifactCreation(request);
      break;
    case "space-artifact-create-cancel":
      cancelSpaceArtifactCreation(request.requestId, request.spaceId);
      break;
    case "document-check-in":
      submitDocumentCheckIn(request);
      break;
    case "document-check-in-cancel":
      cancelDocumentCheckIn(request.requestId, request.scope);
      break;
    case "directory-administration-open":
      openDirectoryAdministration(request.operationEpoch, request.spaceId);
      break;
    case "directory-administration-refresh":
      void loadDirectoryAdministrationPage(request.operationEpoch, request.cursor, "refreshing");
      break;
    case "directory-administration-submit":
      void submitDirectoryAdministrationCommand(request.operationEpoch, request.requestId, request.command);
      break;
    case "directory-administration-capability-request":
      requestDirectoryAdministrationCapability(request.operationEpoch);
      break;
    case "directory-administration-capability-result":
      settleDirectoryAdministrationCapability(request.operationEpoch, request.transferEpoch, request.copied);
      break;
    case "directory-administration-close":
      closeDirectoryAdministration(request.operationEpoch);
      break;
    case "directory-close":
      closeDirectory();
      break;
    case "service-contributions": { reconcileDocumentServiceContributionsV1(request.services); break; }
    case "service-operation": {
      if (!documentServices.dispatch(request) && request.action === "open") {
        const scope = (request.payload as { scope: DocumentScope }).scope;
        post({ kind: "inference-port-opened", owner: request.owner, serviceId: request.serviceId, operationEpoch: request.operationEpoch, scope, outcome: "refused", code: "inference.unavailable" });
      }
      break;
    }
    case "browser-actor-action": {
      const state = artifactState(request.scope.documentId, request.scope.spaceId),
        { clientInstanceId, ...raw } = request,
        action = parseBrowserActorActionRequestV1(raw);
      void (async () => {
        const result = state?.openClientInstanceId === clientInstanceId && state.browserActorReservation !== null
          ? await state.browserActorReservation.dispatchAction(action)
          : browserActorActionDisposition(action, "rejected", 0, null, [], "action-owner-mismatch");
        post({ ...result, clientInstanceId });
      })();
      break;
    }
    case "browser-actor-view-state": {
      const context = parseBrowserActorViewStateRequest(request);
      const state = artifactState(context.scope.documentId, context.scope.spaceId);
      if (state?.openClientInstanceId === context.clientInstanceId && JSON.stringify(state.browserActorViewState) !== JSON.stringify(context.viewState)) {
        state.browserActorViewState = context.viewState;
        state.browserActorReservation?.refreshHostView();
      }
      break;
    }
    case "browser-actor-ui-patch-result": {
      const state = artifactState(request.scope.documentId, request.scope.spaceId);
      if (state?.openClientInstanceId === request.clientInstanceId) state.browserActorReservation?.settleUiPatch(request);
      break;
    }
  }
}
//#endregion 🔖️MessageBridge

/** 🧪 Neutral worker authority seams available only to test-owned assembly registrars. */
export function documentWorkerTestDependenciesV1(): BackboneWorkerTestDependencies {
  if (typeof process === "undefined" || process.env.VITEST !== "true") throw new Error("worker test authority unavailable");
  const testSeams = {
    get directoryAdministration() { return directoryAdministration; },
    set directoryAdministration(value: typeof directoryAdministration) { setDirectoryAdministration(value); },
    get directoryClient() { return directoryClient; },
    set directoryClient(value: typeof directoryClient) { setDirectoryClient(value); },
    get directorySessionEpoch() { return directorySessionEpoch; },
    set directorySessionEpoch(value: typeof directorySessionEpoch) { setDirectorySessionEpoch(value); },
    get executionTargetStatusObserver() { return executionTargetStatusObserver; },
    set executionTargetStatusObserver(value: typeof executionTargetStatusObserver) { setExecutionTargetStatusObserver(value); },
    get hubSessionQueued() { return hubSessionQueued; },
    set hubSessionQueued(value: typeof hubSessionQueued) { setHubSessionQueued(value); },
    get browserSessionAuthority() { return browserSessionAuthority; },
    get hubSessionCapabilityHeld() { return hubSessionCapability !== undefined; },
    get browserSessionOperationFence() { return browserSessionOperationFence; },
    acceptBrowserSessionAuthority,
    captureBrowserSessionOperationFence,
    attachHubSessionPort,
    detachHubSessionPort,
    browserActorAppCommandV1,
    get socketGrantTestIssue() { return socketGrantTestIssue; },
    set socketGrantTestIssue(value: typeof socketGrantTestIssue) { setSocketGrantTestIssue(value); },
    get documentSocketGrantTestIssue() { return documentSocketGrantTestIssue; },
    set documentSocketGrantTestIssue(value: typeof documentSocketGrantTestIssue) { setDocumentSocketGrantTestIssue(value); },
    get spaceArtifactCreationTestFetch() { return spaceArtifactCreationTestFetch; },
    set spaceArtifactCreationTestFetch(value: typeof spaceArtifactCreationTestFetch) { setSpaceArtifactCreationTestFetch(value); },
    get workerPostTestSink() { return workerPostTestSink; },
    set workerPostTestSink(value: typeof workerPostTestSink) { setWorkerPostTestSink(value); },
  };
  return { testSeams, DOCUMENT_BACKBONE_RETENTION_LIMITS, handleAck, ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES, ArtifactBootstrapAssembler, DIRECTORY_COMMAND_TRANSPORT_CAPACITY, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1, DirectoryClient, DirectoryEventPageBootstrapV1, DocumentExecutionTargetLease, HUB_RECONNECT_MAX_MS, IDENTITY_CONFIG_SCHEMA, PENDING_MUTATIONS_QUEUE_LIMIT, SANITY_POLL_MIN_MS, SUSTAINED_HEALTHY_MS, VerifiedColdDocumentPair, abortArtifactBootstrap, installStreamMuxEndpoint, artifactBootstrapFailure, artifactState, artifacts, browserActorChildCapacity, browserDirectoryRequest, browserExecutionTargetAssetRequest, bytesHex, clearHubSessionCapability, closeArtifact, closeArtifactRuntime, closeDirectory, connectHubOnce, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, decodeClientFrame, decodePackPayload, decodePackValue, decodeServerFrame, directoryAdministration, directoryClient, directoryCommandOperations, directoryCommandQueue, directoryCommandSha256, directorySessionEpoch, directoryWorkerEpoch, dispatchBackboneWorkerRequest, documentExecutionOwners, documentExecutionTargetLeaseMintToken, documentExecutionTargetStatusRoleV1, documentOpenPlanAuthority, documentRuntimeKeyForConfig, documentRuntimeKeyV1, documentCatchingUpV1, newArtifactState, requestDocumentActorRecoveryV1, dropDocumentExecutionTargetLease, dropVerifiedColdDocumentPair, emitEvent, encodeActorUiPatchReceipt, encodeBackboneMessage, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse, encodeDocumentBackboneEnvelopeBatchExact, encodePackValue, encodeServerFrame, executionTargetHex, executionTargetSha256Hex, executionTargetStatusObserver, extractServerCommandsDocumentBackboneBatchExact, flushDirectoryQueue, foldIdentityEvent, fromWireEnvelope, handleHubFrame, handleTsRequest, hubBinding, identityActorConfig, installHubSessionCapability, hubSessionFetch, hubSessionQueued, openArtifact, ownedArrayBuffer, parseDocumentBackboneMessage, parseDocumentExecutionTargetLeaseFieldsV1, queueOutbox, readExecutionTargetBody, relayMutationsToHub, requestDocumentSocketAuthority, reserveDocumentBrowserActorChild, revokeDirectoryAdministrationForScope, rollbackEnvelope, sameLeaseFieldsV1, scopedDirectoryStreams, sealDirectoryCommandReceiptV1, sealDirectoryCommandRequestV1, settleDirectoryCommand, socketGrantTestIssue, spaceArtifactCreationCatalogOperations, spaceArtifactCreationOperations, spaceArtifactCreationTestFetch, stampSession, toWireEnvelope, verifiedColdDocumentPairMintToken, verifyBrowserActorDescribeV1, workerPostTestSink };
}

export const documentWorkerTestSourceV1 = { directory: import.meta.dir, url: import.meta.url };

//#region 🧪️Tests
// 🧵️ Whole block stripped from production builds (see this file's header doc) — `node:*` imports
// below are dynamic specifically so they never get bundled into the actual browser Worker script.
if (import.meta.vitest) {
  const dependencies = documentWorkerTestDependenciesV1();
  const testSeams = dependencies.testSeams;
  const { registerFolderArchiveRestoreTests } = await import("../../../🧪️tests/🧪️folder-archive-restore/🟦️.ts");
  await registerFolderArchiveRestoreTests(import.meta.vitest, { testSeams, artifactState, closeArtifact, handleTsRequest, installStreamMuxEndpoint });
  const { registerConfigFacetFoldTests } = await import("../../../🧪️tests/🎚️config-facet-fold/🟦️.ts");
  await registerConfigFacetFoldTests(import.meta.vitest, { testSeams, artifactState, closeArtifact, handleTsRequest, installStreamMuxEndpoint, configFacetArchiveV1 });
  const { registerColdPairLoadingTests } = await import("./🧪️tests/🧪️cold-pair-loading/🟦️.ts");
  await registerColdPairLoadingTests(import.meta.vitest, { settleColdPairLoading, retryColdPairBackpressure, coldPairWaitDelay, COLD_PAIR_WAIT_POLICY });

}
//#endregion 🧪️Tests

export type ConfigFacetV1 = Readonly<{
  schema: string;
  initial: unknown;
  decodeValue: (payload: unknown) => unknown;
  decodeDiff: (payload: unknown) => unknown;
  apply: (base: unknown, diff: unknown) => unknown;
}>;

/** 🎚️ The facets the store folds, keyed by their schema (which is also their document id). */
export const CONFIG_FACETS: ReadonlyMap<string, ConfigFacetV1> = new Map<string, ConfigFacetV1>([
  [IDENTITY_CONFIG_SCHEMA, { schema: IDENTITY_CONFIG_SCHEMA, initial: null, decodeValue: decodeIdentityValue, decodeDiff: decodeIdentityDiff, apply: (base, diff) => applyIdentityDiff(base as Parameters<typeof applyIdentityDiff>[0], diff as Parameters<typeof applyIdentityDiff>[1]) }],
  [LOCAL_CATALOG_CONFIG_SCHEMA, { schema: LOCAL_CATALOG_CONFIG_SCHEMA, initial: { documents: [] }, decodeValue: decodeLocalCatalogValue, decodeDiff: decodeLocalCatalogDiff, apply: (base, diff) => applyLocalCatalogDiff(base as Parameters<typeof applyLocalCatalogDiff>[0], diff as Parameters<typeof applyLocalCatalogDiff>[1]) }],
]);

/** 🗃️ A facet's persisted form: its whole value as the archive's pack value, no history log. */
export function configFacetArchiveV1(value: unknown): Uint8Array {
  return encodeDocumentArchiveBytes({ parent_pack: Array.from(encodePackValue(value as PackValue)), parent_spr: [], members: [] });
}

/** 🗃️ The value a facet archive holds, or `undefined` when the archive is not one of this facet. */
function decodeConfigFacetArchive(facet: ConfigFacetV1, archive: Uint8Array): unknown {
  try {
    return facet.decodeValue(decodePackValue(new Uint8Array(decodeDocumentArchiveBytes(archive).parent_pack)));
  } catch {
    return undefined;
  }
}

/** 🧮️ Folds the diffs of `envelopes` onto `base`, in order, through the facet's applier. `null` when one is not a diff of this facet. */
function foldConfigFacetEnvelopes(facet: ConfigFacetV1, base: unknown, envelopes: readonly MutationEnvelope[]): { readonly value: unknown } | null {
  let value = base;
  for (const envelope of envelopes) {
    const diff = facet.decodeDiff(envelope.diff.payload);
    if (diff === undefined) return null;
    value = facet.apply(value, diff);
  }
  return { value };
}

/** 🎚️ Seeds a facet document from what its folder holds (`archive`), or from the facet's empty value when the folder holds none,
 * then folds the envelopes that waited for that first answer. */
function seedConfigFacet(state: ArtifactState, archive: Uint8Array | null): void {
  const facet = CONFIG_FACETS.get(state.config.schema);
  if (facet === undefined) return;
  const held = archive === null ? undefined : decodeConfigFacetArchive(facet, archive);
  if (held !== undefined) state.facetValue = held;
  else if (state.facetValue === undefined) state.facetValue = facet.initial;
  const waiting = state.facetBacklog;
  state.facetBacklog = [];
  if (waiting.length > 0) foldConfigFacet(state, waiting);
}

/** 🎚️ The store's fold of locally admitted facet envelopes: each diff applies through the facet's central twin onto the held
 * value, the resulting archive is persisted to the facet's folder, and every client of the document is handed that archive as
 * the new projection. Envelopes that arrive before the folder's first answer wait for it. */
function foldConfigFacet(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  const facet = CONFIG_FACETS.get(state.config.schema);
  if (facet === undefined) return;
  const folder = folderBinding(state.config);
  if (folder !== null && !state.folderBootstrapped) {
    state.facetBacklog.push(...envelopes);
    return;
  }
  const folded = foldConfigFacetEnvelopes(facet, state.facetValue === undefined ? facet.initial : state.facetValue, envelopes);
  if (folded === null) {
    rejectLocally(state, localCommandRejectionV1("local.backbone-malformed", "a config facet envelope must carry the facet's sparse diff", { envelopes: envelopes.length }));
    return;
  }
  state.facetValue = folded.value;
  const archive = configFacetArchiveV1(folded.value);
  if (folder !== null) {
    void writeFolder(state, folder, Array.from(archive)).then(() => {
      releaseDocumentBackboneOwnership(state, state.pendingMutations);
      state.pendingMutations = [];
      setStatus(state, { pendingMutations: 0 });
    }, (error) => console.error("[backbone-worker] folder write failed", state.config.documentId, error));
  }
  emitEvent(state, { kind: "documentArchiveReplaced", archive: Array.from(archive) });
}

/** 🎚️ Folds the envelopes another tab's store admitted (they arrive on the document's channel) into the held value. That tab
 * persisted them; this one only keeps its own projection current and tells its clients. */
function foldRemoteConfigFacet(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  const facet = CONFIG_FACETS.get(state.config.schema);
  if (facet === undefined) return;
  const folded = foldConfigFacetEnvelopes(facet, state.facetValue === undefined ? facet.initial : state.facetValue, envelopes);
  if (folded !== null) state.facetValue = folded.value;
}
