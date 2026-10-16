/** 🛂️ The narrow authority port a document worker lends to an installed owner service. The os worker implements it; a plugin driver builds on it through the SDK entry `@semio-tech/framework/inference/service/worker-host` and never imports the worker.
 * @see ../🟦️.ts installed service contract the driver implements */
import type { ColdDocumentPairFrontier } from "../../../🎭️actor/📥️cold-pair/🟦️.ts";
import type { FetchTimeoutResponse } from "../../../🚪️io/🌐️fetch-timeout/🟦️.ts";
import type { DocumentScope } from "../../../📄️document/🪪️scope/🟦️.ts";

/** 🚨️ Thrown for a non-2xx hub answer; `status` tells "the hub answered and refused" from a network failure that has no status. */
export class DirectoryHttpError extends Error {
  readonly status: number;
  constructor(status: number, message: string) {
    super(message);
    this.status = status;
  }
}

/** 🧾️ The public fields of one verified execution-target lease that a driver reads; receipts, grants and origins never appear here. */
export type DocumentServiceLeaseFieldsV1 = Readonly<{
  scope: DocumentScope;
  catalog: Readonly<{ generationId: string }>;
  package: Readonly<{ pluginId: string; componentSha256: string; descriptorByteSha256: string }>;
  browserActor: Readonly<{ kind: "none" }> | Readonly<{ kind: "closed-browser-actor"; sha256: string }>;
  grant: Readonly<{ write: boolean }>;
  revalidation: Readonly<{ directoryRevision: number; membershipGeneration: number; sessionGeneration?: number; shareGeneration?: number }>;
}>;

export type DocumentServiceLeaseV1 = Readonly<{ live: boolean; fields(): DocumentServiceLeaseFieldsV1 }>;
export type DocumentServiceBrowserActorReservationV1 = Readonly<{ generation: bigint; bindApprovalUndoIfMounted(): void }>;
export type DocumentServiceVerifiedColdPairV1 = Readonly<{ frontier: ColdDocumentPairFrontier }>;

/** 📄️ The document state a driver may observe. */
export type DocumentServiceArtifactStateV1 = Readonly<{
  runtimeKey: string;
  closed: boolean;
  docAbort: AbortController;
  openClientInstanceId: string;
  config: Readonly<{ documentId: string }>;
  executionTargetLease: DocumentServiceLeaseV1 | null;
  browserActorReservation: DocumentServiceBrowserActorReservationV1 | null;
  verifiedColdPair: DocumentServiceVerifiedColdPairV1 | null;
}>;

/** 🧩️ What the worker hands to a driver on rebootstrap and mount. */
export type DocumentServiceDriverContextV1 = Readonly<{
  state: DocumentServiceArtifactStateV1;
  reservation?: DocumentServiceBrowserActorReservationV1;
  pair?: DocumentServiceVerifiedColdPairV1;
}>;

export type DocumentServiceSessionFenceV1 = Readonly<{ token: object; sessionBindingSha256: string; authorizationGeneration: number }>;
export type DocumentServiceRuntimeScopeV1 = Readonly<{ kind: "hub"; dataClass: "persistedShared"; spaceId: string; documentId: string }> | Readonly<{ kind: "local"; documentId: string }>;
export type DocumentServiceCallOptionsV1 = Readonly<{
  timeoutMs: number;
  signal?: AbortSignal;
  admit?: () => boolean;
  retain?: () => boolean;
  accept?: (response: FetchTimeoutResponse, admission: object) => Promise<FetchTimeoutResponse>;
}>;
export type DocumentServiceTransportV1 = Readonly<{ call(action: string, payload: unknown, options: DocumentServiceCallOptionsV1): Promise<FetchTimeoutResponse> }>;

/** 📮️ One message a driver posts to the shell; the worker stamps `owner` and `serviceId` and validates the wire shape on encode. */
export type DocumentServiceWorkerMessageV1 = Readonly<{ kind: string }> & Readonly<Record<string, unknown>>;

/** 🔌️ The authority a worker lends to one installed service for exactly as long as the service stays installed. */
export type DocumentServiceWorkerHostV1 = Readonly<{
  artifactScope(state: DocumentServiceArtifactStateV1): DocumentScope | undefined;
  artifactState(documentId: string, spaceId?: string): DocumentServiceArtifactStateV1 | undefined;
  bytesHex(value: Uint8Array): string;
  captureBrowserSessionOperationFence(): DocumentServiceSessionFenceV1 | null;
  sameBrowserSessionOperationFence(fence: DocumentServiceSessionFenceV1): boolean;
  post(message: DocumentServiceWorkerMessageV1): void;
  documentRuntimeKeyV1(scope: DocumentServiceRuntimeScopeV1): string;
  SOCKET_GRANT_REQUEST_TIMEOUT_MS: number;
  directorySessionEpoch: number;
  bindDocumentService(owner: string, serviceId: string, scope: DocumentScope): DocumentServiceTransportV1;
}>;
