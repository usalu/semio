
import { directoryCommandSha256, sealDirectoryCommandReceiptV1, sealDirectoryCommandRequestV1 } from "@semio-tech/framework/directory/io/text";

import { StreamMuxPortEndpointV1 } from "@semio-tech/framework/io/stream-mux";
import type { BackboneWorkerRequest, BackboneWorkerWireMessage, DocumentScope } from "../../../🟦️";
import { ArtifactBootstrapAssembler, DOCUMENT_BACKBONE_RETENTION_LIMITS, decodeClientFrame, decodeServerFrame, encodeDocumentBackboneEnvelopeBatchExact, encodeServerFrame, extractServerCommandsDocumentBackboneBatchExact } from "@semio-tech/framework-replication";
import { DirectoryClient, HUB_RECONNECT_MAX_MS, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, decodePackValue, documentRuntimeKeyV1, encodeBackboneMessage, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse, encodePackValue, parseDocumentBackboneMessage } from "../../../🟦️";
import { browserActorChildCapacity } from "@semio-tech/framework-plugin/browser-bundle/child";
import { verifyBrowserActorDescribeV1 } from "@semio-tech/framework-plugin/browser-bundle/describe";
import { encodeActorUiPatchReceipt } from "../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts";
import { parseBrowserActorActionRequestV1 } from "@semio-tech/framework/browser-bundle/action-handoff";
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
  const { registerColdPairLoadingTests } = await import("./🧪️tests/🧪️cold-pair-loading/🟦️.ts");
  await registerColdPairLoadingTests(import.meta.vitest, { settleColdPairLoading, retryColdPairBackpressure, coldPairWaitDelay, COLD_PAIR_WAIT_POLICY });

}
//#endregion 🧪️Tests
