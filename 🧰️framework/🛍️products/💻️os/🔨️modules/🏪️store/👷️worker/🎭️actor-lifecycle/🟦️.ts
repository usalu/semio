import { sameLeaseFieldsV1, sameExecutionTargetV1, parseDocumentPlanSocketGrantIntentV1, parseDocumentOpenIntentV1, parseDocumentOpenPlanV1 } from "../../../📇️directory/🧬️schema/🔏️document-open/🟦️.ts";
import { DOCUMENT_BACKBONE_RETENTION_LIMITS } from "@semio-tech/framework-replication";
import type { DocumentOpenPlanV1, DocumentOpenIntentV1, DocumentExecutionTargetLeaseFieldsV1 } from "@semio-tech/framework/directory/schema";
import type { ArtifactActorConfig, PersistenceBinding, DocumentLinkStatusCode, DocumentSocketGrantReceiptV1, ArtifactSyncStatus, RemoteState } from "../../../../🟦️";
import { documentLinkTransition, documentLinkExpiresAtMs, documentLinkStatus, parseDocumentSocketGrantReceiptV1, DOCUMENT_LINK_ACCESS_REFUSED_STATUSES } from "../../../../🟦️";
import { type ArtifactState, artifacts, emitEvent } from "../🧾️document-state/🟦️.ts";
import { type DocumentExecutionTargetLease, installDocumentExecutionTargetLease } from "../🪪️execution-lease/🟦️.ts";
import { documentRuntimeKeyForConfig, hubBinding } from "../🧩️actor-wire-helpers/🟦️.ts";
import { type BrowserDocumentSocketAuthorityV1, clearHubSessionCapability, documentSocketGrantTestIssue, hubSessionFetch, readDocumentOpenJson } from "../🔐️hub-session/🟦️.ts";
import { type ExecutionTargetReadControl, assertExecutionTargetRead, dropDocumentExecutionTargetLease, emitExecutionTargetStatus, receiptFreeFields, retireMountedDocumentChild } from "../📥️execution-target-read/🟦️.ts";
import { SOCKET_GRANT_REQUEST_TIMEOUT_MS, documentExecutionTargetLeaseMintToken } from "../⚙️constants/🟦️.ts";

/** 🔐️ Resolves authority only from the current document owner, never caller-supplied actor metadata. */
export function documentBrowserActorLease(state: ArtifactState): DocumentExecutionTargetLease {
  const lease = state.executionTargetLease,
    binding = hubBinding(state.config);
  if (state.closed || state.docAbort.signal.aborted || artifacts.get(state.runtimeKey) !== state || !binding || !lease?.live) throw new Error("document browser actor: unavailable");
  const fields = lease.fields();
  if (
    state.runtimeKey !== documentRuntimeKeyForConfig(state.config) ||
    fields.scope.spaceId !== binding.spaceId ||
    fields.scope.documentId !== state.config.documentId ||
    fields.artifact.schema !== state.config.schema ||
    lease.hubOrigin !== binding.baseUrl.replace(/\/+$/u, "")
  )
    throw new Error("document browser actor: scope mismatch");
  if ((binding.requestedSurfaceId !== undefined && binding.requestedSurfaceId !== fields.surface.surfaceId) || (binding.installedTarget !== undefined && !sameLeaseFieldsV1(binding.installedTarget, fields)))
    throw new Error("document browser actor: selection mismatch");
  return lease;
}

/** 🎭️ Whether this document's live lease runs it in a verified browser actor — its remote backbone then belongs to that
 * actor, never to the Shell's local instance, even before the child exists. */
export function documentAwaitsBrowserActor(state: ArtifactState): boolean {
  const lease = state.executionTargetLease;
  return lease !== null && lease.live && lease.fields().browserActor.kind === "closed-browser-actor";
}

/** 📥️ Keeps one remote backbone message for the actor child that is not reserved yet (the `Welcome` tail precedes the
 * admitting `Session`): before, it went to the Shell's local instance, so a document opened after edits showed only its
 * checkpoint — every edit since the last check-in was missing on screen although the hub held it (ticket 26/09/23 C10). */
export function retainBackboneBeforeBrowserActor(state: ArtifactState, message: Uint8Array): void {
  const pending = state.browserActorBackboneBeforeReservation;
  const bytes = pending.reduce((total, entry) => total + entry.byteLength, 0);
  if (pending.length >= DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages || message.byteLength > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - bytes) throw new Error("document browser actor: pre-reservation backbone capacity");
  pending.push(message);
}

/** ⚖️ Compares one server plan against the complete locally installed execution-target fields through
 * the single shared {@link sameLeaseFieldsV1} relation. A non-`react` renderer target is admitted
 * only when the caller owns a live private lease whose own verified fields are the comparison input;
 * the verified target is then routed to an explicit renderer state, never to a module loader. */
export function documentOpenPlanAuthority(
  plan: DocumentOpenPlanV1,
  intent: DocumentOpenIntentV1,
  config: ArtifactActorConfig,
  installed: NonNullable<Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>["installedTarget"]>,
  lease?: DocumentExecutionTargetLease,
): Omit<BrowserDocumentSocketAuthorityV1, "receipt"> {
  const leaseFields = lease !== undefined && lease.live ? lease.fields() : undefined;
  const projected = ((): DocumentExecutionTargetLeaseFieldsV1 | null => {
    try {
      return receiptFreeFields(plan, { component: installed.component.byteLength, descriptor: installed.descriptor.byteLength, browserActor: installed.browserActor.kind === "closed-browser-actor" ? installed.browserActor.byteLength : undefined });
    } catch {
      return null;
    }
  })();
  if (
    projected === null ||
    plan.artifact.schema !== config.schema ||
    plan.scope.spaceId !== intent.scope.spaceId ||
    plan.scope.documentId !== intent.scope.documentId ||
    intent.requestedSurfaceId !== installed.surface.surfaceId ||
    !sameLeaseFieldsV1(projected, installed) ||
    (plan.surface.rendererTarget !== "react" && (leaseFields === undefined || !sameLeaseFieldsV1(leaseFields, installed)))
  )
    throw new Error("document open: authority mismatch");
  const packSchemaHash = Array.from({ length: 32 }, (_unused, index) => Number.parseInt(plan.artifact.packSchemaHash.slice(index * 2, index * 2 + 2), 16));
  const configured = config.packSchemaHash;
  if (configured && configured.some((byte) => byte !== 0) && (configured.length !== 32 || configured.some((byte, index) => byte !== packSchemaHash[index]))) throw new Error("document open: authority mismatch");
  return { schema: plan.artifact.schema, packSchemaHash, parentDialect: plan.parentDialect, surfaceId: plan.surface.surfaceId };
}

type SuspendedDocumentBrowserActorV1 = Readonly<{ reservation: NonNullable<ArtifactState["browserActorReservation"]>; lease: DocumentExecutionTargetLease; pair: NonNullable<ArtifactState["verifiedColdPair"]> }>;

/** 🔌️ The mounted child a short link loss suspended, with its live lease and applied pair — or `null`. */
export function suspendedDocumentBrowserActor(state: ArtifactState): SuspendedDocumentBrowserActorV1 | null {
  const reservation = state.browserActorReservation,
    lease = state.executionTargetLease,
    pair = state.verifiedColdPair;
  return reservation !== null && reservation.suspended && lease !== null && lease.live && pair !== null ? { reservation, lease, pair } : null;
}

/** 🔌️ Suspends a MOUNTED child (and its applied cold pair) when its socket closes, so a short link loss neither refuses
 * the human's next edit nor turns the reconnect into a reopen: the link fails (`documentLinkTransition`) and a timer
 * ticks it at its expiry ({@link DOCUMENT_LINK_SHORTAGE_POLICY}'s bound). `false` when nothing mounted is resumable —
 * the caller then retires the lease as before. */
export function suspendDocumentBrowserActorLink(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>): boolean {
  const reservation = state.browserActorReservation,
    lease = state.executionTargetLease,
    pair = state.verifiedColdPair;
  if (state.docAbort.signal.aborted || reservation === null || lease === null || !lease.live || pair === null || !reservation.suspendLink()) return false;
  if (!pair.suspendLink()) {
    reservation.close();
    return false;
  }
  const nowMs = Date.now();
  state.link = documentLinkTransition(state.link, { kind: "failed", nowMs });
  const expiresAtMs = documentLinkExpiresAtMs(state.link);
  if (state.linkShortageTimer !== null) clearTimeout(state.linkShortageTimer);
  state.linkShortageTimer = null;
  if (expiresAtMs === undefined) {
    retireSuspendedDocumentBrowserActor(state, binding);
    return true;
  }
  state.linkShortageTimer = setTimeout(() => {
    state.linkShortageTimer = null;
    state.link = documentLinkTransition(state.link, { kind: "tick", nowMs: Date.now() });
    if (state.browserActorReservation === reservation && reservation.suspended && state.link.kind === "expired") retireSuspendedDocumentBrowserActor(state, binding);
  }, Math.max(0, expiresAtMs - nowMs));
  return true;
}

/** 🛑️ Retires a suspended child with its terminal link's localized reason: the link stayed down too long
 * (`link-expired`), or the hub withdrew access (`access-revoked`). */
function retireSuspendedDocumentBrowserActor(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>): void {
  if (state.linkShortageTimer !== null) clearTimeout(state.linkShortageTimer);
  state.linkShortageTimer = null;
  dropDocumentExecutionTargetLease(state);
  const code: DocumentLinkStatusCode = documentLinkStatus(state.link);
  if (!state.closed && (code === "link-expired" || code === "access-revoked")) emitExecutionTargetStatus(state, binding, code);
}

/** 🔁️ Re-admits the suspended child's actor on the fresh plan when the plan names the SAME verified target
 * ({@link sameExecutionTargetV1}: the lease adopts the plan's newer checkpoint and revalidation witness), exchanging the plan for a
 * socket grant without reinstalling or reloading anything. `null` when the target changed, in which case the caller reopens. */
async function resumeDocumentSocketAuthority(
  state: ArtifactState,
  binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>,
  plan: DocumentOpenPlanV1,
  intent: DocumentOpenIntentV1,
  assertOwner: DocumentOpenOwnerV1,
  suspended: SuspendedDocumentBrowserActorV1,
  grantPath: string,
): Promise<BrowserDocumentSocketAuthorityV1 | null> {
  let authority: Omit<BrowserDocumentSocketAuthorityV1, "receipt">;
  try {
    const current = suspended.lease.fields();
    const next = receiptFreeFields(plan, { component: current.component.byteLength, descriptor: current.descriptor.byteLength, browserActor: current.browserActor.kind === "closed-browser-actor" ? current.browserActor.byteLength : undefined });
    if (!sameExecutionTargetV1(current, next)) return null;
    suspended.lease.refresh(documentExecutionTargetLeaseMintToken, next);
    authority = documentOpenPlanAuthority(plan, intent, state.config, suspended.lease.fields(), suspended.lease);
  } catch {
    return null;
  }
  const grantControl: ExecutionTargetReadControl = { signal: state.docAbort.signal, deadlineAtMs: Math.min(plan.expiresAtUnixMs, Date.now() + SOCKET_GRANT_REQUEST_TIMEOUT_MS), assertCurrent: assertOwner };
  assertExecutionTargetRead(grantControl);
  const exchange = parseDocumentPlanSocketGrantIntentV1({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt });
  const grantResponse = await hubSessionFetch(grantPath, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(exchange) }, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: state.docAbort.signal });
  if (!grantResponse.ok) throw new Error("document open: unavailable");
  let receipt: DocumentSocketGrantReceiptV1;
  try {
    receipt = parseDocumentSocketGrantReceiptV1(await readDocumentOpenJson(grantResponse, grantControl));
    assertExecutionTargetRead(grantControl);
    if (receipt.expiresAtMs <= Date.now() || receipt.expiresAtMs > plan.expiresAtUnixMs) throw new Error("document open: invalid grant");
  } catch {
    clearHubSessionCapability();
    throw new Error("document open: invalid grant");
  }
  if (suspendedDocumentBrowserActor(state)?.reservation !== suspended.reservation) throw new Error("document open: cancelled");
  assertOwner.adopt();
  suspended.reservation.adoptGrant(suspended.lease.readmitBrowserActor(documentExecutionTargetLeaseMintToken, receipt, plan.expiresAtUnixMs, { binding, intent, assertCurrent: assertOwner }));
  return { receipt, ...authority };
}

/** 🧭️ One document-open attempt's owner check; `adopt` makes the attempt the document's current one. */
type DocumentOpenOwnerV1 = (() => void) & { readonly adopt: () => void };

/** 🧭️ Captures a single document-open attempt and rejects every later owner or selection change. A `published` attempt
 * owns the document at once; an unpublished one (a reconnect beside a suspended live child) leaves the current owner in
 * place until {@link DocumentOpenOwnerV1.adopt}, so failed reconnect attempts during a link loss never retire the child. */
function captureDocumentOpenOwner(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>, intent: DocumentOpenIntentV1, published = true): DocumentOpenOwnerV1 {
  const attempt = Symbol("document-open"),
    runtimeKey = state.runtimeKey,
    schema = state.config.schema;
  const origin = binding.baseUrl.replace(/\/+$/u, ""),
    installed = binding.installedTarget === undefined ? undefined : structuredClone(binding.installedTarget);
  let expected = published ? attempt : state.executionTargetOpen;
  if (published) state.executionTargetOpen = attempt;
  const assertCurrent = () => {
    const current = hubBinding(state.config);
    if (
      state.closed ||
      state.docAbort.signal.aborted ||
      state.executionTargetOpen !== expected ||
      state.runtimeKey !== runtimeKey ||
      artifacts.get(runtimeKey) !== state ||
      runtimeKey !== documentRuntimeKeyForConfig(state.config) ||
      state.config.schema !== schema ||
      state.config.documentId !== intent.scope.documentId ||
      state.openClientInstanceId !== intent.clientInstanceId ||
      !current ||
      current.spaceId !== intent.scope.spaceId ||
      current.baseUrl.replace(/\/+$/u, "") !== origin ||
      (current.requestedSurfaceId ?? current.installedTarget?.surface.surfaceId) !== intent.requestedSurfaceId ||
      (installed === undefined ? current.installedTarget !== undefined : current.installedTarget === undefined || !sameLeaseFieldsV1(current.installedTarget, installed))
    )
      throw new Error("document execution target: stale owner");
  };
  return Object.assign(assertCurrent, {
    adopt: () => {
      assertCurrent();
      state.executionTargetOpen = attempt;
      expected = attempt;
    },
  });
}

export async function requestDocumentSocketAuthority(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>): Promise<BrowserDocumentSocketAuthorityV1> {
  const grantPath = `/spaces/${encodeURIComponent(binding.spaceId)}/documents/${encodeURIComponent(state.config.documentId)}/socket-grants`;
  if (documentSocketGrantTestIssue) {
    const receipt = await documentSocketGrantTestIssue(binding.baseUrl, grantPath, state.docAbort.signal);
    return {
      receipt,
      schema: state.config.schema,
      packSchemaHash: state.config.packSchemaHash ?? new Array(32).fill(0),
      ...(binding.installedTarget ? { surfaceId: binding.installedTarget.surface.surfaceId } : binding.requestedSurfaceId ? { surfaceId: binding.requestedSurfaceId } : {}),
    };
  }
  const requestedSurfaceId = binding.requestedSurfaceId ?? binding.installedTarget?.surface.surfaceId;
  if (requestedSurfaceId === undefined) throw new Error("document open: installed target unavailable");
  const intent = parseDocumentOpenIntentV1({
    schema: "semio.hub.document-open-intent/v1",
    version: 1,
    scope: { spaceId: binding.spaceId, documentId: state.config.documentId },
    requestedSurfaceId,
    clientInstanceId: state.openClientInstanceId,
  });
  const suspended = suspendedDocumentBrowserActor(state);
  const assertOwner = captureDocumentOpenOwner(state, binding, intent, suspended === null);
  const openControl: ExecutionTargetReadControl = { signal: state.docAbort.signal, deadlineAtMs: Date.now() + SOCKET_GRANT_REQUEST_TIMEOUT_MS, assertCurrent: assertOwner };
  assertExecutionTargetRead(openControl);
  const openPath = `/spaces/${encodeURIComponent(binding.spaceId)}/documents/${encodeURIComponent(state.config.documentId)}/open-plan`;
  const openResponse = await hubSessionFetch(
    `${openPath}`,
    { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(intent) },
    { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: state.docAbort.signal },
  );
  if (!openResponse.ok) {
    if (suspended !== null && DOCUMENT_LINK_ACCESS_REFUSED_STATUSES.has(openResponse.status)) {
      state.link = documentLinkTransition(state.link, { kind: "refused", nowMs: Date.now() });
      retireSuspendedDocumentBrowserActor(state, binding);
    }
    throw new Error("document open: unavailable");
  }
  let plan: DocumentOpenPlanV1;
  try {
    plan = parseDocumentOpenPlanV1(await readDocumentOpenJson(openResponse, openControl), Date.now());
  } catch {
    const cancelled = state.docAbort.signal.aborted;
    if (!cancelled) clearHubSessionCapability();
    throw new Error(cancelled ? "document open: cancelled" : "document open: invalid plan");
  }
  assertOwner();
  if (suspended !== null) {
    const resumed = await resumeDocumentSocketAuthority(state, binding, plan, intent, assertOwner, suspended, grantPath);
    if (resumed !== null) return resumed;
    assertOwner.adopt();
  }
  retireMountedDocumentChild(state);
  let lease: DocumentExecutionTargetLease | undefined;
  let published = false;
  try {
    if (plan.surface.rendererTarget !== "react" || binding.installedTarget === undefined) {
      try {
        lease = await installDocumentExecutionTargetLease(state, binding, plan, intent, assertOwner);
      } catch (error) {
        const cancelled = state.docAbort.signal.aborted || String((error as Error).message).includes("cancelled");
        emitExecutionTargetStatus(state, binding, cancelled ? "cancelled" : "integrity-failed");
        if (!cancelled) clearHubSessionCapability();
        throw new Error(cancelled ? "document open: cancelled" : "document open: invalid execution target");
      }
    }
    let authority: Omit<BrowserDocumentSocketAuthorityV1, "receipt">;
    try {
      const installed = lease === undefined ? binding.installedTarget : lease.fields();
      if (installed === undefined) throw new Error("document open: installed target unavailable");
      authority = documentOpenPlanAuthority(plan, intent, state.config, installed, lease);
    } catch {
      lease?.drop();
      emitExecutionTargetStatus(state, binding, "stale");
      clearHubSessionCapability();
      throw new Error("document open: invalid plan");
    }
    if (state.docAbort.signal.aborted || (lease !== undefined && !lease.live)) {
      lease?.drop();
      emitExecutionTargetStatus(state, binding, "cancelled");
      throw new Error("document open: cancelled");
    }
    const grantControl: ExecutionTargetReadControl = { signal: state.docAbort.signal, deadlineAtMs: Math.min(plan.expiresAtUnixMs, Date.now() + SOCKET_GRANT_REQUEST_TIMEOUT_MS), assertCurrent: assertOwner };
    assertExecutionTargetRead(grantControl);
    const exchange = parseDocumentPlanSocketGrantIntentV1({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt });
    const grantResponse = await hubSessionFetch(
      `${grantPath}`,
      { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(exchange) },
      { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: state.docAbort.signal },
    );
    if (!grantResponse.ok) {
      lease?.drop();
      if (lease !== undefined) emitExecutionTargetStatus(state, binding, "stale");
      throw new Error("document open: unavailable");
    }
    try {
      const receipt = parseDocumentSocketGrantReceiptV1(await readDocumentOpenJson(grantResponse, grantControl));
      assertExecutionTargetRead(grantControl);
      if (receipt.expiresAtMs <= Date.now() || receipt.expiresAtMs > plan.expiresAtUnixMs || (lease !== undefined && !lease.live)) throw new Error("document open: invalid grant");
      if (lease !== undefined) {
        lease.admitBrowserActor(documentExecutionTargetLeaseMintToken, receipt, plan.expiresAtUnixMs, { binding, intent, assertCurrent: assertOwner });
        state.executionTargetLease = lease;
        emitExecutionTargetStatus(state, binding, "renderer-unavailable");
      }
      assertExecutionTargetRead(grantControl);
      published = true;
      return { receipt, ...authority };
    } catch {
      lease?.drop();
      clearHubSessionCapability();
      throw new Error("document open: invalid grant");
    }
  } finally {
    if (!published && lease !== undefined) {
      if (state.executionTargetLease === lease) dropDocumentExecutionTargetLease(state);
      lease.drop();
    }
  }
}

export function setStatus(state: ArtifactState, patch: Partial<ArtifactSyncStatus>): void {
  state.status = { ...state.status, ...patch };
  emitEvent(state, { kind: "status", ...state.status });
}

export function setRemote(state: ArtifactState, remote: RemoteState): void {
  setStatus(state, { remote });
}

export function folderBinding(config: ArtifactActorConfig): Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }> | null {
  const binding = config.bindings.find((entry): entry is Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }> => entry.kind === "folder");
  return binding ?? null;
}
