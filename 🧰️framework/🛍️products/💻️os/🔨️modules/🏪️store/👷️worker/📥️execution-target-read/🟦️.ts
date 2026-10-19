import type { DocumentOpenPlanV1, DocumentExecutionTargetLeaseFieldsV1, DocumentExecutionTargetStatusCodeV1, DocumentExecutionTargetProgressV1 } from "@semio-tech/framework/directory/schema";
import { leaseFieldsFromPlanV1 } from "../../../📇️directory/🧬️schema/🔏️document-open/🟦️.ts";
import type { BackboneWorkerResponse, PersistenceBinding } from "../../../../🟦️";
import { boundChildText } from "@semio-tech/framework/browser-bundle/child/schema";
import type { FetchTimeoutResponse } from "@semio-tech/framework";
import type { PackValue } from "../../../../🟦️";
import { BROWSER_ACTOR_UI_PATCH_SURFACE_MAXIMUM } from "@semio-tech/framework/browser-bundle/patch-handoff";
import { isPackInteger, decodePackValue, encodePackValue, packUIntSafeOrNull, noteAuthoredEnvelopeIds } from "../../../../🟦️";
import { panelTabKindId, type PanelTabKind } from "@semio-tech/framework/manifest";
import { type SurfaceArtifactKindV1, type SurfaceKindAppV1, surfaceOpensArtifactKindV1 } from "@semio-tech/framework/directory/schema";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "@semio-tech/framework/directory/schema";
import { type DocumentActorLossCauseV1, documentActorRecoveryStepV1, DOCUMENT_ACTOR_RECOVERY_V1 } from "../🚑️actor-recovery/🟦️.ts";
import { closeHubSocketV1 } from "@semio-tech/framework/directory/client/socket-close";
import { type ArtifactState, artifactScope, dropVerifiedColdDocumentPair, post } from "../🧾️document-state/🟦️.ts";
import { EXECUTION_TARGET_DIAGNOSTIC_MAX_BYTES, EXECUTION_TARGET_PROGRESS_UNIT_BYTES } from "../⚙️constants/🟦️.ts";
import { retireFolderCanonicalBootstrapMirror } from "../🌉️wire-bridge/🟦️.ts";
import { requeuePendingBatches } from "../📮️hub-outbox/🟦️.ts";

/** 🧾️ Receipt-free lease projection of one parsed plan, at the byte lengths under comparison. */
export function receiptFreeFields(plan: DocumentOpenPlanV1, byteLengths: { readonly component: number; readonly descriptor: number; readonly browserActor?: number }): DocumentExecutionTargetLeaseFieldsV1 {
  return leaseFieldsFromPlanV1(plan, byteLengths);
}

export function executionTargetHex(bytes: Uint8Array): string {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

export async function executionTargetSha256Hex(bytes: Uint8Array): Promise<string> {
  return executionTargetHex(new Uint8Array(await crypto.subtle.digest("SHA-256", ownedArrayBuffer(bytes))));
}

export function ownedArrayBuffer(bytes: Uint8Array): ArrayBuffer {
  const owned = new Uint8Array(bytes.byteLength);
  owned.set(bytes);
  return owned.buffer;
}

export type DocumentExecutionTargetAssetV1 = "manifest" | "component" | "descriptor" | "browser-actor";

export function executionTargetAssetPath(spaceId: string, documentId: string, asset: DocumentExecutionTargetAssetV1): string {
  return `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/execution-target/${asset}`;
}

/** 🔭️ Observation seam for the execution-target live region, mirroring {@link socketGrantTestIssue}:
 * a worker scope posts, and a harness without one still sees the exact bounded payload. */
export let executionTargetStatusObserver: ((status: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>) => void) | null = null;
/** 🧪️ Assigns `executionTargetStatusObserver` for the test seam, because an imported binding cannot be assigned outside its declaring module. */
export function setExecutionTargetStatusObserver(value: typeof executionTargetStatusObserver): void {
  executionTargetStatusObserver = value;
}

export function emitExecutionTargetStatus(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>, code: DocumentExecutionTargetStatusCodeV1, progress?: DocumentExecutionTargetProgressV1, diagnostic?: string): void {
  const scope = { spaceId: binding.spaceId, documentId: state.config.documentId };
  const status: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }> = { kind: "execution-target-status", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, spaceId: binding.spaceId, scope, code, ...(progress ? { progress } : {}), ...(diagnostic ? { diagnostic: boundChildText(diagnostic, EXECUTION_TARGET_DIAGNOSTIC_MAX_BYTES) } : {}) };
  executionTargetStatusObserver?.(status);
  post(status);
}

export type ExecutionTargetReadControl = Readonly<{ signal: AbortSignal; deadlineAtMs: number; assertCurrent(): void }>;
type ExecutionTargetBodyResponse = FetchTimeoutResponse & { readonly body?: ReadableStream<Uint8Array> | null };

type ExecutionTargetReadResult = Awaited<ReturnType<ReturnType<NonNullable<ExecutionTargetBodyResponse["body"]>["getReader"]>["read"]>>;

function isExecutionTargetByteChunk(value: unknown): value is Uint8Array {
  return ArrayBuffer.isView(value) && Object.prototype.toString.call(value) === "[object Uint8Array]";
}

export function assertExecutionTargetRead(control: ExecutionTargetReadControl): void {
  if (control.signal.aborted || !Number.isSafeInteger(control.deadlineAtMs) || Date.now() >= control.deadlineAtMs) throw new Error("document execution target: cancelled or expired");
  control.assertCurrent();
}

/** 🧹️ Owns one bounded body through EOF, cancellation and stale-read retirement. */
export async function readBoundedExecutionTargetBody(response: FetchTimeoutResponse, expected: number | null, maximum: number, control: ExecutionTargetReadControl, report: (received: number, total: number) => void): Promise<Uint8Array> {
  const body = (response as ExecutionTargetBodyResponse).body;
  if (!(body instanceof ReadableStream)) throw new Error("document execution target: invalid body");
  const reader = body.getReader();
  let bytes: Uint8Array | null = null,
    cancelled = false,
    timer: ReturnType<typeof setTimeout> | undefined;
  let rejectStop: (error: Error) => void = () => {};
  const stop = new Promise<never>((_resolve, reject) => {
    rejectStop = reject;
  });
  void stop.catch(() => {});
  const abort = (): void => rejectStop(new Error("document execution target: cancelled or expired"));
  const cancel = (): void => {
    if (cancelled) return;
    cancelled = true;
    void reader.cancel().catch(() => {});
  };
  try {
    const header = response.headers.get("content-length");
    const declared = header === null ? null : Number(header);
    if (
      !response.ok ||
      !Number.isSafeInteger(maximum) ||
      maximum < 1 ||
      (declared !== null && (!Number.isSafeInteger(declared) || declared < 1 || declared > maximum)) ||
      (expected !== null && (declared !== expected || !Number.isSafeInteger(expected) || expected < 1 || expected > maximum))
    )
      throw new Error("document execution target: invalid body");
    assertExecutionTargetRead(control);
    control.signal.addEventListener("abort", abort, { once: true });
    timer = setTimeout(abort, Math.min(0x7fffffff, Math.max(0, control.deadlineAtMs - Date.now())));
    assertExecutionTargetRead(control);
    bytes = new Uint8Array(declared ?? maximum);
    let received = 0,
      announced = 0;
    for (;;) {
      assertExecutionTargetRead(control);
      const slot: { state: "pending" | "claimed" | "abandoned"; result?: ExecutionTargetReadResult } = { state: "pending" };
      const pending = reader.read().then((result) => {
        slot.result = result;
        if (slot.state === "abandoned" && isExecutionTargetByteChunk(result.value)) result.value.fill(0);
        return result;
      });
      let result: ExecutionTargetReadResult;
      try {
        result = await Promise.race([pending, stop]);
        slot.state = "claimed";
      } catch (error) {
        slot.state = "abandoned";
        if (isExecutionTargetByteChunk(slot.result?.value)) slot.result.value.fill(0);
        throw error;
      }
      try {
        assertExecutionTargetRead(control);
        if (result.done) break;
        if (!isExecutionTargetByteChunk(result.value) || result.value.byteLength > bytes.length - received) throw new Error("document execution target: invalid body");
        bytes.set(result.value, received);
        received += result.value.byteLength;
        if (received - announced >= EXECUTION_TARGET_PROGRESS_UNIT_BYTES) {
          announced = received;
          report(received, declared ?? maximum);
        }
      } finally {
        if (isExecutionTargetByteChunk(result.value)) result.value.fill(0);
      }
    }
    assertExecutionTargetRead(control);
    if (received === 0 || (declared !== null && received !== declared)) throw new Error("document execution target: invalid body");
    report(received, declared ?? received);
    assertExecutionTargetRead(control);
    if (declared === null) return bytes.slice(0, received);
    const owned = bytes;
    bytes = null;
    return owned;
  } catch (error) {
    cancel();
    throw error;
  } finally {
    clearTimeout(timer);
    control.signal.removeEventListener("abort", abort);
    bytes?.fill(0);
    reader.releaseLock();
  }
}

/** 📥️ Decodes the bounded manifest and wipes its temporary body owner. */
export async function readExecutionTargetJson(response: FetchTimeoutResponse, maximum: number, control: ExecutionTargetReadControl): Promise<unknown> {
  const bytes = await readBoundedExecutionTargetBody(response, null, maximum, control, () => {});
  try {
    assertExecutionTargetRead(control);
    return JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
  } finally {
    bytes.fill(0);
  }
}

/** 📥️ Acquires exactly one declared asset body with bounded progress. */
export async function readExecutionTargetBody(
  response: FetchTimeoutResponse,
  expectedByteLength: number,
  maxBytes: number,
  stage: "component" | "descriptor" | "browser-actor",
  control: ExecutionTargetReadControl,
  report: (progress: DocumentExecutionTargetProgressV1) => void,
): Promise<Uint8Array> {
  return readBoundedExecutionTargetBody(response, expectedByteLength, maxBytes, control, (completedBytes, totalBytes) => report({ stage, completedBytes, totalBytes }));
}

/** 🪟️ One body a verified actor child renders, keyed as the shell keys its store and the guest its surface ref. */
type DocumentRenderSurfaceV1 = Readonly<{ key: string; bodyKey: string }>;

/** 🪟️ The bodies one verified actor child renders: every window kind of the verified app (the lease's window among
 * them, the one its mount identity names) and every panel-tab leaf, each panel keyed exactly as the shell keys its tab
 * (`panelTabKindId`). A document the shell shows in several windows (note's `note-composite` + `note-navigator`) is one
 * actor, so every one of its windows renders from — and commands into — that actor; a window left to the shell's local
 * instance showed a document the hub never sent it and refused every command as `command owner mismatch` (ticket
 * 26/09/23 C10, S15 finding b). The shell's own manifest never selects them. */
export type DocumentRenderSurfacesV1 = Readonly<{ windows: readonly DocumentRenderSurfaceV1[]; panels: readonly DocumentRenderSurfaceV1[] }>;

function renderSurfaceTextV1(value: PackValue | undefined): value is string {
  return typeof value === "string" && value.length > 0 && new TextEncoder().encode(value).byteLength <= 256 && !/[\u0000-\u001f\u007f]/u.test(value);
}

/** 🪟️ Every window kind of the verified app, in declaration order. A malformed window, a key that repeats and more
 * surfaces than one patch offer carries fail the descriptor. */
function verifiedWindowSurfacesV1(windowKinds: readonly Readonly<Record<string, PackValue>>[]): readonly DocumentRenderSurfaceV1[] {
  const keys = new Set<string>();
  return Object.freeze(windowKinds.map((window) => {
    if (!renderSurfaceTextV1(window.id) || !renderSurfaceTextV1(window.bodyKey) || keys.has(window.id) || keys.size === BROWSER_ACTOR_UI_PATCH_SURFACE_MAXIMUM) throw new Error("document execution target: descriptor mismatch");
    keys.add(window.id);
    return Object.freeze({ key: window.id, bodyKey: window.bodyKey });
  }));
}

/** 🗂️ Every panel-tab leaf that carries a body, depth-first like the shell's `flattenPanelTabLeaves`. A malformed tab,
 * a key that repeats (or names a window) and more surfaces than one patch offer carries fail the descriptor. */
function verifiedPanelSurfacesV1(tabs: PackValue | undefined, windows: readonly DocumentRenderSurfaceV1[]): readonly DocumentRenderSurfaceV1[] {
  const panels: DocumentRenderSurfaceV1[] = [],
    keys = new Set(windows.map(({ key }) => key));
  const visit = (value: PackValue | undefined, depth: number): void => {
    if (value === undefined) return;
    if (!Array.isArray(value) || depth > 8) throw new Error("document execution target: descriptor mismatch");
    for (const raw of value as readonly PackValue[]) {
      if (raw === null || typeof raw !== "object" || Array.isArray(raw) || raw instanceof Uint8Array || isPackInteger(raw)) throw new Error("document execution target: descriptor mismatch");
      const tab = raw as Readonly<Record<string, PackValue>>;
      if (Array.isArray(tab.children) && tab.children.length > 0) {
        visit(tab.children, depth + 1);
        continue;
      }
      if (tab.bodyKey === undefined) continue;
      const kind = tab.kind;
      const kindTag = kind !== null && typeof kind === "object" && !Array.isArray(kind) ? (kind as Readonly<Record<string, PackValue>>).kind : undefined;
      const key = typeof kindTag !== "string" ? undefined : kindTag === "app" ? (kind as Readonly<Record<string, PackValue>>).id : (panelTabKindId({ kind: kindTag } as unknown as PanelTabKind) as string | undefined);
      if (!renderSurfaceTextV1(key) || !renderSurfaceTextV1(tab.bodyKey) || keys.has(key) || keys.size === BROWSER_ACTOR_UI_PATCH_SURFACE_MAXIMUM) throw new Error("document execution target: descriptor mismatch");
      keys.add(key);
      panels.push(Object.freeze({ key, bodyKey: tab.bodyKey }));
    }
  };
  visit(tabs, 0);
  return Object.freeze(panels);
}

/** 📜️ Strictly admits the raw semantic descriptor bytes: canonical re-encoding equality first, then exact equality between
 * the decoded package descriptor and the verified lease fields. The descriptor names no bytes — the lease package's shipped
 * `ComponentReceiptV1` binds it to the component, and a descriptor carrying any byte binding of its own is refused. A sibling
 * JSON manifest or a caller-selected URL is never descriptor authority, and the render surfaces it answers come from those
 * verified bytes alone. */
export function parseVerifiedPackageDescriptorV1(bytes: Uint8Array, fields: DocumentExecutionTargetLeaseFieldsV1): DocumentRenderSurfacesV1 {
  const decoded = decodePackValue(bytes);
  const canonical = encodePackValue(decoded);
  if (canonical.length !== bytes.length || canonical.some((byte, index) => byte !== bytes[index])) throw new Error("document execution target: descriptor is not canonical");
  const record = (value: PackValue|undefined): Record<string, PackValue> => {
    if (value === null || typeof value !== "object" || Array.isArray(value) || isPackInteger(value)) throw new Error("document execution target: descriptor invalid");
    return value as Record<string, PackValue>;
  };
  const descriptor = record(decoded);
  const executionProtocol = record(descriptor.executionProtocol);
  const manifest = record(descriptor.manifest);
  const apps = Array.isArray(manifest.apps) ? (manifest.apps as readonly PackValue[]).map(record) : [];
  const artifactKinds = Array.isArray(manifest.artifactKinds) ? (manifest.artifactKinds as readonly PackValue[]).map(record) : [];
  const hostedArtifactKinds = Array.isArray(manifest.hostedArtifactKinds) ? (manifest.hostedArtifactKinds as readonly PackValue[]).map(record) : [];
  const app = apps.find((entry) => entry.id === fields.surface.appId);
  const dialect = app === undefined ? undefined : record(app.dialect);
  const windowKinds = app !== undefined && Array.isArray(app.windowKinds) ? (app.windowKinds as readonly PackValue[]).map(record) : [];
  const window = windowKinds.find((window) => window.id === fields.surface.windowKindId);
  const kindPairs = (kinds: readonly Record<string, PackValue>[]): SurfaceArtifactKindV1[] => kinds.flatMap((kind) => (typeof kind.id === "string" && typeof kind.schema === "string" ? [{ id: kind.id, schema: kind.schema }] : []));
  const surfaceApp = (entry: Record<string, PackValue>): SurfaceKindAppV1 | null => {
    const coordinate = entry.dialect;
    if (coordinate === null || typeof coordinate !== "object" || Array.isArray(coordinate) || coordinate instanceof Uint8Array || isPackInteger(coordinate)) return null;
    const { artifactKind, standard, subset } = coordinate as Readonly<Record<string, PackValue>>;
    if ((entry.role !== "editor" && entry.role !== "viewer") || typeof artifactKind !== "string" || typeof standard !== "string" || typeof subset !== "string") return null;
    const packRecord = (value: PackValue | undefined): Readonly<Record<string, PackValue>> | null => (value === undefined || value === null || typeof value !== "object" || Array.isArray(value) || value instanceof Uint8Array || isPackInteger(value) ? null : (value as Readonly<Record<string, PackValue>>));
    const io = packRecord(entry.io);
    const presentedId = packRecord(io?.artifact)?.id;
    const presentedSchema = io?.artifactSchema;
    const presents = typeof presentedId === "string" && presentedId !== "" && typeof presentedSchema === "string" ? { id: presentedId, schema: presentedSchema } : null;
    return { role: entry.role, dialect: { artifactKind, standard, subset }, artifactKinds: kindPairs(Array.isArray(entry.artifactKinds) ? (entry.artifactKinds as readonly PackValue[]).map(record) : []), presents };
  };
  const openingApp = app === undefined ? null : surfaceApp(app);
  if (
    packUIntSafeOrNull(descriptor.descriptorVersion??null) !== 1 ||
    descriptor.packageId !== fields.package.packageId ||
    descriptor.execution !== "isolated" ||
    Object.keys(executionProtocol).length !== 1 ||
    packUIntSafeOrNull(executionProtocol.appChannelVersion??null) !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 ||
    packUIntSafeOrNull(executionProtocol.appChannelVersion??null) !== fields.package.executionProtocol.appChannelVersion ||
    manifest.pluginId !== fields.package.pluginId ||
    manifest.version !== fields.package.version ||
    descriptor.hashes !== undefined ||
    app === undefined ||
    dialect === undefined ||
    app.id !== fields.surface.surfaceId ||
    app.role !== fields.surface.role ||
    dialect.artifactKind !== fields.parentDialect.artifactKind ||
    dialect.standard !== fields.parentDialect.standard ||
    dialect.subset !== fields.parentDialect.subset ||
    window === undefined ||
    openingApp === null ||
    !surfaceOpensArtifactKindV1(kindPairs(artifactKinds), kindPairs(hostedArtifactKinds), apps.flatMap((entry) => surfaceApp(entry) ?? []), openingApp, fields.artifact)
  )
    throw new Error("document execution target: descriptor mismatch");
  const windows = verifiedWindowSurfacesV1(windowKinds);
  return Object.freeze({ windows, panels: verifiedPanelSurfacesV1(app.panelTabs, windows) });
}

/** 🗑️ Drops the execution-target lease with its actor child, and with them the document that child held
 * ({@link forgetMountedDocument}). */
export function dropDocumentExecutionTargetLease(state: ArtifactState): void {
  const mounted = state.verifiedColdPair !== null;
  dropVerifiedColdDocumentPair(state);
  const mirror = state.canonicalFolderMirror;
  state.canonicalFolderMirror = null;
  if (mirror) void retireFolderCanonicalBootstrapMirror(mirror);
  state.browserActorReservation?.close();
  state.executionTargetLease?.drop();
  state.executionTargetLease = null;
  state.browserActorBackboneBeforeReservation = [];
  if (mounted) forgetMountedDocument(state);
}

/** 🧹️ Forgets the document an actor child held once its cold pair is gone: the pack and spr published from that pair, the frontier
 * and resume token the child reached, and the ids it ingested — the operations still unacknowledged stay noted, since their echo
 * must never apply twice. The next connection then opens like a first one: its `SocketHelloV1` names no frontier, the hub answers
 * with the tail from its active checkpoint, and the next child is seeded from that checkpoint's canonical pair. A kept pack made
 * {@link seedColdPairFromCanonicalCheckpoint} skip the seed and left the next child without a document — "verifying" forever, every
 * edit refused `owner-mismatch` (ticket 26/09/23 C12, run `c12short5`: a 5 s link cut whose reconnect named a newer plan). */
export function forgetMountedDocument(state: ArtifactState): void {
  state.currentPack = null;
  state.currentSpr = null;
  state.frontier = null;
  state.resumeToken = null;
  state.artifactBootstrapProgress = [];
  state.ingestedMutationIds.clear();
  const unacknowledged = [...[...state.pendingBatches.values()].flat(), ...state.outbox];
  noteAuthoredEnvelopeIds(state.ingestedMutationIds, unacknowledged.map((envelope) => state.exactLocalEnvelopes.get(envelope)?.envelope.mutation_id ?? envelope.id));
}

/** 🔁️ Retires the document's actor child while the document stays open — its socket closed and the child could not be suspended, or
 * a reconnect's plan named another execution target than the suspended child's — and, when a child was mounted, tells the Shell to
 * discard its stale surface exactly as a rebuild does (`artifact-rebootstrap-required`): the connection that follows seeds a fresh
 * child from the hub's active checkpoint and replays the unsent operations on top. */
export function retireMountedDocumentChild(state: ArtifactState, cause: "rebootstrap-required" | "actor-lost" = "rebootstrap-required"): void {
  const mounted = state.verifiedColdPair !== null || state.browserActorReservation !== null;
  dropDocumentExecutionTargetLease(state);
  if (!mounted || state.closed || state.docAbort.signal.aborted || state.artifactRebootstrapRequired) return;
  const scope = artifactScope(state);
  post({ kind: "artifact-rebootstrap-required", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), message: cause, retryable: true });
}

/** 🚑️ A mounted browser actor was lost without anyone asking — its action's outcome unconfirmed, its child faulted on an inbound
 * hub frame, a render turn failed ({@link DOCUMENT_ACTOR_RECOVERY_V1}). The in-flight batches go back to the outbox front (their
 * outcome is unknown; the hub settles what it already holds), the mounted child retires as `actor-lost`, and the socket ends
 * with `actorLost` so the reconnect loop reopens AT ONCE — a new plan, lease and actor seeded from the hub's checkpoint pair and
 * tail, the outbox resent in order. Past the policy's bound the document gets the typed fault instead and no further actor. */
export function requestDocumentActorRecoveryV1(state: ArtifactState, cause: DocumentActorLossCauseV1): void {
  if (state.closed || state.docAbort.signal.aborted) return;
  const step = documentActorRecoveryStepV1(state.actorRecovery, { kind: "lost", cause, atMs: Date.now() });
  state.actorRecovery = step.memory;
  requeuePendingBatches(state);
  if (step.decision === "exhausted") {
    dropDocumentExecutionTargetLease(state);
    const scope = artifactScope(state);
    post({ kind: "artifact-bootstrap-failed", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), code: "recovery-exhausted", message: `${DOCUMENT_ACTOR_RECOVERY_V1.exhaustedFault} (${cause})`, retryable: false });
    return;
  }
  state.actorRecoveryRequested = true;
  retireMountedDocumentChild(state, "actor-lost");
  const socket = state.socket;
  if (socket !== null && socket.readyState === WebSocket.OPEN) closeHubSocketV1(socket, "actorLost");
}
