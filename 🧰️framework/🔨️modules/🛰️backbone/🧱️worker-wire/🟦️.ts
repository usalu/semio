/** 🧱️ Backbone worker wire: the framed request and response encoders and decoders of the backbone worker protocol. */

import { parseRetainedCloneGrant } from "@semio-tech/value-ts/retained-clone/wire";
import { canonicalDirectoryCommandV1, parseDirectoryEventV1 } from "../../📇️directory/🚪️io/📝️text/🟦️.ts";
import { parseInstalledServiceContributionsV1, parseInstalledServiceOperationV1, parseInstalledServiceStatusV1, parseInstalledServiceHistoryStatusV1 } from "../../💡️inference/🔌️service/🟦️.ts";
import { documentCheckInStatusFromValueV1 } from "../../📄️document/📌️check-in/🟦️.ts";
import { parseInferencePortClosedV1, parseInferencePortOpeningResultV1 } from "../../💡️inference/🚪️opening/🟦️.ts";
import { DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS } from "../../📇️directory/🧬️schema/📄️event-page/🟦️.ts";
import { parseBrowserActorUiPatchOfferV1, parseBrowserActorUiPatchResultV1 } from "../../🌐️browser-bundle/🩹️patch-handoff/🟦️.ts";
import { parseBrowserActorActionRequestV1, parseBrowserActorActionResultV1 } from "../../🌐️browser-bundle/🎯️action-handoff/🟦️.ts";
import { parseBrowserActorViewStateRequest } from "../../🌐️browser-bundle/🪟️view-context/🟦️.ts";
import { decodePackValue, encodePackValue } from "../../🎁️packs/🟦️.ts";
import { parseArtifactActorMsg, parseArtifactEvent, wireArtifactActorMsg, wireArtifactEvent } from "../🎭️artifact-actor/🟦️.ts";
import { parseDirectoryAdministrationWorkerRequestV1, parseSpaceArtifactCreationCatalogRefreshRequiredV1, parseSpaceArtifactCreationCatalogStatusV1, parseSpaceArtifactCreationCatalogV1, parseSpaceArtifactCreationStatusV1, parseSpaceArtifactCreationWorkerRequestV1, workerWireArtifactFrontierV1, workerWireClientInstanceIdV1, workerWireCreationIdentityV1, workerWireCreationRequestIdV1, workerWireIdV1, workerWireScopeV1, workerWireSha256V1 } from "../🔬️worker-parsers/🟦️.ts";
import type { BackboneWorkerRequest, BackboneWorkerResponse } from "../📨️worker-messages/🟦️.ts";

//#region 🔖️BackboneWorkerProtocol
//#region 🔖️EnvelopeCodec
/** 📤️ Main thread → `🏪️store/👷️worker/🟦️.ts` — `bytes` is a UTF-8 worker wire payload (see {@link encodeBackboneWorkerRequest}). */
export type BackboneWorkerWireMessage = { readonly wire: Uint8Array };

/** 🧵️ Worker wire magic — must match `store_sync::backbone_worker_wire::MAGIC`. */
export const BACKBONE_WORKER_WIRE_MAGIC = 0x01;

function parseBackboneWorkerWire<T>(wire: Uint8Array, decode: (value: unknown) => T): T {
  if (wire.length === 0 || wire[0] !== BACKBONE_WORKER_WIRE_MAGIC) {
    throw new Error("backbone worker wire: unknown or empty payload");
  }
  return decode(decodePackValue(wire.subarray(1)));
}

/** 📇️ The most events one `directory-space-events` message carries: one sealed directory page's raw rows. */
export const DIRECTORY_SPACE_EVENTS_MAX_PER_MESSAGE = DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS;

/** 🧵️ Encodes a {@link BackboneWorkerRequest} for the wasm `store_worker` (`handleRequestBytes`). */
export function encodeBackboneWorkerRequest(request: BackboneWorkerRequest): Uint8Array {
  const wire =
    request.kind === "send"
      ? { ...request, message: wireArtifactActorMsg(request.message) }
      : request;
  const packed = encodePackValue(wire);
  return new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...packed]);
}

/** 🧵️ Decodes a {@link BackboneWorkerRequest} from the wasm actor or structured-clone twin. */
export function decodeBackboneWorkerRequest(wire: Uint8Array): BackboneWorkerRequest {
  const parsed = parseBackboneWorkerWire(wire, (value) => value as Record<string, unknown>);
  if (parsed.kind === "service-contributions") return parseInstalledServiceContributionsV1(parsed);
  if (parsed.kind === "service-operation") return parseInstalledServiceOperationV1(parsed);
  if (typeof parsed.kind === "string" && parsed.kind.startsWith("directory-administration-")) return parseDirectoryAdministrationWorkerRequestV1(parsed);
  if (parsed.kind === "browser-actor-action") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker request: invalid client instance id");
    const { clientInstanceId: _client, ...request } = parsed;
    return { ...parseBrowserActorActionRequestV1(request), clientInstanceId };
  }
  if (parsed.kind === "browser-actor-view-state") return parseBrowserActorViewStateRequest(parsed);
  if (parsed.kind === "space-artifact-create" || parsed.kind === "space-artifact-create-cancel") {
    return parseSpaceArtifactCreationWorkerRequestV1(parsed);
  }
  if (parsed.kind === "document-check-in" || parsed.kind === "document-check-in-cancel") {
    const start = parsed.kind === "document-check-in";
    if (Object.keys(parsed).sort().join(",") !== (start ? "clientInstanceId,kind,requestId,scope" : "kind,requestId,scope")) throw new Error("backbone worker request: invalid check-in fields");
    const requestId = workerWireCreationRequestIdV1(parsed.requestId);
    const documentId = typeof parsed.scope === "object" && parsed.scope !== null ? workerWireIdV1((parsed.scope as Record<string, unknown>).documentId) : null;
    const scope = documentId === null ? null : workerWireScopeV1(parsed.scope, documentId);
    if (requestId === null || scope === null) throw new Error("backbone worker request: invalid check-in owner");
    if (!start) return { kind: "document-check-in-cancel", requestId, scope };
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker request: invalid check-in client instance id");
    return { kind: "document-check-in", requestId, clientInstanceId, scope };
  }
  if (parsed.kind === "directory-space-open" || parsed.kind === "directory-space-close") {
    const open = parsed.kind === "directory-space-open";
    if (Object.keys(parsed).sort().join(",") !== (open ? "baseUrl,kind,spaceId" : "kind,spaceId")) throw new Error("backbone worker request: invalid space directory fields");
    const spaceId = workerWireCreationIdentityV1(parsed.spaceId);
    if (spaceId === null || (open && (typeof parsed.baseUrl !== "string" || parsed.baseUrl.length === 0 || parsed.baseUrl.length > 2048))) throw new Error("backbone worker request: invalid space directory owner");
    return open ? { kind: "directory-space-open", baseUrl: parsed.baseUrl as string, spaceId } : { kind: "directory-space-close", spaceId };
  }
  if (parsed.kind === "space-artifact-creation-catalog-open") {
    if (Object.keys(parsed).sort().join(",") !== "clientInstanceId,kind,spaceId") throw new Error("backbone worker request: invalid creation catalog fields");
    const spaceId = workerWireCreationIdentityV1(parsed.spaceId),
      clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (spaceId === null || clientInstanceId === null) throw new Error("backbone worker request: invalid creation catalog scope");
    return { kind: "space-artifact-creation-catalog-open", clientInstanceId, spaceId };
  }
  if (parsed.kind === "browser-actor-ui-patch-result") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker request: invalid client instance id");
    const { clientInstanceId: _clientInstanceId, ...result } = parsed;
    return { ...parseBrowserActorUiPatchResultV1(result), clientInstanceId };
  }
  if (parsed.kind === "send" && typeof parsed.message === "object" && parsed.message !== null) {
    const clientInstanceId = parsed.clientInstanceId === undefined ? undefined : workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker request: invalid client instance id");
    const message = parseArtifactActorMsg(parsed.message as Record<string, unknown>);
    if (message.kind === "documentBackbone") {
      const fields = ["kind", "documentId", "clientInstanceId", "message", ...(parsed.spaceId === undefined ? [] : ["spaceId"])];
      const documentId = workerWireIdV1(parsed.documentId);
      const spaceId = parsed.spaceId === undefined ? undefined : workerWireIdV1(parsed.spaceId);
      if (Object.keys(parsed).sort().join(",") !== fields.sort().join(",") || documentId === null || clientInstanceId === undefined || spaceId === null) throw new Error("backbone worker request: invalid document backbone owner");
      return { kind: "send", documentId, clientInstanceId, ...(spaceId === undefined ? {} : { spaceId }), message };
    }
    return {
      kind: "send",
      documentId: String(parsed.documentId),
      ...(typeof parsed.spaceId === "string" ? { spaceId: parsed.spaceId } : {}),
      ...(clientInstanceId === undefined ? {} : { clientInstanceId }),
      message,
    };
  }
  if (parsed.kind === "open" || parsed.kind === "close") {
    if(parsed.kind === "open")parseRetainedCloneGrant(parsed.actorIdentityGrant);
    const clientInstanceId = parsed.clientInstanceId === undefined ? undefined : workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (parsed.clientInstanceId !== undefined && clientInstanceId === null) throw new Error("backbone worker request: invalid client instance id");
  }
  if (parsed.kind === "directory-command") {
    if (Object.keys(parsed).sort().join(",") !== "command,kind,requestId") throw new Error("backbone worker request: invalid directory command fields");
    const requestId = workerWireCreationRequestIdV1(parsed.requestId);
    if (requestId === null) throw new Error("backbone worker request: invalid directory command request id");
    return { kind: "directory-command", requestId, command: canonicalDirectoryCommandV1(parsed.command) };
  }

  return parsed as BackboneWorkerRequest;
}

/** 🧵️ Encodes a {@link BackboneWorkerResponse} from the wasm actor / TS fallback. */
export function encodeBackboneWorkerResponse(response: BackboneWorkerResponse): Uint8Array {
  const wire =
    response.kind === "event" ? { ...response, event: wireArtifactEvent(response.event) } : response;
  const packed = encodePackValue(wire);
  return new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...packed]);
}

/** 🧵️ Decodes a worker response/event wire payload from the wasm actor. */
export function decodeBackboneWorkerResponse(wire: Uint8Array): BackboneWorkerResponse {
  const parsed = parseBackboneWorkerWire(wire, (value) => value as Record<string, unknown>);
  if (parsed.kind === "browser-actor-action-result") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker response: invalid client instance id");
    const { clientInstanceId: _client, ...response } = parsed;
    return { ...parseBrowserActorActionResultV1(response), clientInstanceId };
  }
  if (parsed.kind === "space-artifact-creation-status") return parseSpaceArtifactCreationStatusV1(parsed);
  if (parsed.kind === "directory-space-events") {
    if (Object.keys(parsed).sort().join(",") !== "events,kind,spaceId") throw new Error("backbone worker response: invalid space directory fields");
    const spaceId = workerWireCreationIdentityV1(parsed.spaceId);
    if (spaceId === null || !Array.isArray(parsed.events) || parsed.events.length === 0 || parsed.events.length > DIRECTORY_SPACE_EVENTS_MAX_PER_MESSAGE) throw new Error("backbone worker response: invalid space directory owner");
    const events = parsed.events.map((value) => parseDirectoryEventV1(value));
    if (events.some((event, index) => event.spaceId !== spaceId || (index > 0 && events[index - 1]!.seq >= event.seq))) throw new Error("backbone worker response: invalid space directory events");
    return { kind: "directory-space-events", spaceId, events };
  }
  if (parsed.kind === "document-check-in-status") {
    if (Object.keys(parsed).sort().join(",") !== "clientInstanceId,kind,scope,status") throw new Error("backbone worker response: invalid check-in status fields");
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    const documentId = typeof parsed.scope === "object" && parsed.scope !== null ? workerWireIdV1((parsed.scope as Record<string, unknown>).documentId) : null;
    const scope = documentId === null ? null : workerWireScopeV1(parsed.scope, documentId);
    const status = documentCheckInStatusFromValueV1(parsed.status);
    if (clientInstanceId === null || scope === null || status === null) throw new Error("backbone worker response: invalid check-in status");
    return { kind: "document-check-in-status", clientInstanceId, scope, status };
  }
  if (parsed.kind === "space-artifact-creation-catalog-refresh-required") return parseSpaceArtifactCreationCatalogRefreshRequiredV1(parsed);
  if (parsed.kind === "space-artifact-creation-catalog") return parseSpaceArtifactCreationCatalogV1(parsed);
  if (parsed.kind === "space-artifact-creation-catalog-status") return parseSpaceArtifactCreationCatalogStatusV1(parsed);
  if (parsed.kind === "browser-actor-ui-patch") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker response: invalid client instance id");
    const { clientInstanceId: _clientInstanceId, ...offer } = parsed;
    return { ...parseBrowserActorUiPatchOfferV1(offer), clientInstanceId };
  }
  if (parsed.kind === "browser-actor-ui-mounted") {
    const fields = ["activationGeneration", "activeCheckpointId", "browserActorSha256", "catalogGenerationId", "clientInstanceId", "componentSha256", "descriptorDigestV1", "descriptorSha256", "frontier", "instanceId", "kind", "scope", "uiRevision", "verifiedSurfaceId", "windowKindId"];
    if (Object.keys(parsed).sort().join(",") !== fields.sort().join(",")) throw new Error("backbone worker response: invalid mounted UI fields");
    const scopeRow = parsed.scope as Record<string, unknown> | undefined;
    const documentId = scopeRow === undefined ? null : workerWireIdV1(scopeRow.documentId);
    const scope = documentId === null ? null : workerWireScopeV1(parsed.scope, documentId);
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    const verifiedSurfaceId = workerWireIdV1(parsed.verifiedSurfaceId);
    const windowKindId = workerWireIdV1(parsed.windowKindId);
    const activationGeneration = typeof parsed.activationGeneration === "string" && /^[1-9][0-9]{0,19}$/u.test(parsed.activationGeneration) && BigInt(parsed.activationGeneration) <= 0xffffffffffffffffn ? parsed.activationGeneration : null;
    if (scope === null || clientInstanceId === null || verifiedSurfaceId === null || windowKindId === null || activationGeneration === null || !Number.isSafeInteger(parsed.instanceId) || (parsed.instanceId as number) < 0 || !Number.isSafeInteger(parsed.uiRevision) || (parsed.uiRevision as number) < 1) throw new Error("backbone worker response: invalid mounted UI owner");
    const catalogGenerationId = workerWireSha256V1(parsed.catalogGenerationId),
      componentSha256 = workerWireSha256V1(parsed.componentSha256),
      descriptorSha256 = workerWireSha256V1(parsed.descriptorSha256),
      browserActorSha256 = workerWireSha256V1(parsed.browserActorSha256),
      activeCheckpointId = workerWireSha256V1(parsed.activeCheckpointId),
      descriptorDigestV1 = workerWireSha256V1(parsed.descriptorDigestV1),
      frontier = workerWireArtifactFrontierV1(parsed.frontier, scope);
    if (catalogGenerationId === null || componentSha256 === null || descriptorSha256 === null || browserActorSha256 === null || activeCheckpointId === null || descriptorDigestV1 === null || frontier === null) throw new Error("backbone worker response: invalid mounted UI identity");
    return { kind: "browser-actor-ui-mounted", scope, clientInstanceId, activationGeneration, instanceId: parsed.instanceId as number, verifiedSurfaceId, windowKindId, catalogGenerationId, componentSha256, descriptorSha256, browserActorSha256, activeCheckpointId, descriptorDigestV1, frontier, uiRevision: parsed.uiRevision as number };
  }
  if (parsed.kind === "event" && typeof parsed.event === "object" && parsed.event !== null) {
    const documentId = workerWireIdV1(parsed.documentId);
    if (documentId === null) throw new Error("backbone worker response: invalid document id");
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker response: invalid client instance id");
    const event = parseArtifactEvent(parsed.event as Record<string, unknown>);
    const scope = workerWireScopeV1(parsed.scope, documentId);
    if (event.kind === "documentBackbone") {
      const fields = ["clientInstanceId", "documentId", "event", "kind", ...(parsed.scope === undefined ? [] : ["scope"])];
      if (Object.keys(parsed).sort().join(",") !== fields.sort().join(",") || parsed.verifiedSurfaceId !== undefined || (parsed.scope !== undefined && scope === null)) throw new Error("backbone worker response: invalid document backbone owner");
      return { kind: "event", documentId, clientInstanceId, event, ...(scope === null ? {} : { scope }) };
    }
    if (event.kind !== "presence") return { kind: "event", documentId, clientInstanceId, event, ...(scope === null ? {} : { scope }) };
    const verifiedSurfaceId = workerWireIdV1(parsed.verifiedSurfaceId);
    if (scope === null || verifiedSurfaceId === null) return { kind: "event", documentId, clientInstanceId, event: { kind: "presence", peers: [] }, ...(scope === null ? {} : { scope }) };
    return { kind: "event", documentId, clientInstanceId, scope, verifiedSurfaceId, event };
  }
  if (
    parsed.kind === "socket-actor"
    || parsed.kind === "socket-actor-failed"
    || parsed.kind === "artifact-bootstrap-progress"
    || parsed.kind === "artifact-bootstrap-failed"
    || parsed.kind === "artifact-rebootstrap-required"
    || parsed.kind === "execution-target-status"
  ) {
    const documentId = workerWireIdV1(parsed.documentId);
    if (documentId === null) throw new Error("backbone worker response: invalid document id");
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker response: invalid client instance id");
    let scope = workerWireScopeV1(parsed.scope, documentId);
    if (parsed.kind === "execution-target-status" && scope !== null && parsed.spaceId !== scope.spaceId) scope = null;
    const { documentId: _documentId, clientInstanceId: _clientInstanceId, scope: _scope, ...rest } = parsed;
    return { ...rest, documentId, clientInstanceId, ...(scope === null ? {} : { scope }) } as BackboneWorkerResponse;
  }
  if (parsed.kind === "inference-port-status") {
    if (!Number.isSafeInteger(parsed.operationEpoch) || (parsed.operationEpoch as number) < 0 || typeof parsed.scope !== "object" || parsed.scope === null || Array.isArray(parsed.scope)) throw new Error("backbone worker response: invalid inference owner");
    const scopeRow = parsed.scope as Record<string, unknown>;
    const documentId = workerWireIdV1(scopeRow.documentId);
    if (documentId === null) throw new Error("backbone worker response: invalid inference document id");
    const scope = workerWireScopeV1(parsed.scope, documentId);
    if (scope === null) throw new Error("backbone worker response: invalid inference scope");
    return { kind: "inference-port-status", operationEpoch: parsed.operationEpoch as number, scope, status: parseInstalledServiceStatusV1(parsed.status) };
  }
  if (parsed.kind === "inference-port-opened") return parseInferencePortOpeningResultV1(parsed);
  if (parsed.kind === "inference-port-closed") return parseInferencePortClosedV1(parsed);
  if (parsed.kind === "inference-history-status") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null || !Number.isSafeInteger(parsed.historyEpoch) || (parsed.historyEpoch as number) < 1) throw new Error("backbone worker response: invalid inference history owner");
    if (typeof parsed.scope !== "object" || parsed.scope === null || Array.isArray(parsed.scope)) throw new Error("backbone worker response: invalid inference history scope");
    const documentId = workerWireIdV1((parsed.scope as Record<string, unknown>).documentId);
    const scope = documentId === null ? null : workerWireScopeV1(parsed.scope, documentId);
    if (scope === null) throw new Error("backbone worker response: invalid inference history scope");
    return { owner: typeof parsed.owner === "string" ? parsed.owner : (() => { throw new Error("service history: owner required"); })(), serviceId: typeof parsed.serviceId === "string" ? parsed.serviceId : (() => { throw new Error("service history: service required"); })(), kind: "inference-history-status", historyEpoch: parsed.historyEpoch as number, clientInstanceId, scope, status: parseInstalledServiceHistoryStatusV1(parsed.status) };
  }
  return parsed as BackboneWorkerResponse;
}
//#endregion 🔖️EnvelopeCodec
//#endregion 🔖️BackboneWorkerProtocol
