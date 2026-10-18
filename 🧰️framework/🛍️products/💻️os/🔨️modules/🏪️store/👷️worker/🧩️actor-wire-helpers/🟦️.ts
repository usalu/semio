import type { ArtifactActorConfig, PersistenceBinding } from "../../../../🟦️";
import { documentRuntimeKeyV1, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, isPackInteger, decodePackValue, encodePackValue, decodeAppCommand, encodeAppCommand } from "../../../../🟦️";
import { reserveBrowserActorChild, type BrowserActorChildValue } from "../../../🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts";
import type { DocumentOpenIntentV1, DocumentExecutionTargetLeaseFieldsV1 } from "@semio-tech/framework/directory/schema";
import type { ActorUiPatchReceipt } from "../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts";
import { parseColdDocumentPairLifetime, type ColdPairIngressStatus, type ColdDocumentPairCursor, coldDocumentPairCursorEquals, parseWitColdPairIngressStatus } from "@semio-tech/framework-actor/cold-pair";
import { type ActorInstanceOpenRequest, type ActorInstanceLifecycleReceipt, actorInstanceCapturedReceiptMatches, type ActorInstanceCloseRequest, actorInstanceCloseReceiptMatches, type ActorInstanceLifetime, actorInstanceLifetimeEquals } from "../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import { boundChildText, measureChildValue, BROWSER_ACTOR_CHILD_LIMITS, type CommandIngressKindV1, COMMAND_INGRESS_KINDS } from "@semio-tech/framework/browser-bundle/child/schema";
import type { PackValue } from "../../../../🟦️";
import type { BrowserActorActionRequestV1, BrowserActorActionCommitReceiptV1, BrowserActorActionResultV1 } from "@semio-tech/framework/browser-bundle/action-handoff";
import { EXECUTION_TARGET_DIAGNOSTIC_MAX_BYTES } from "../⚙️constants/🟦️.ts";

export function documentRuntimeKeyForConfig(config: ArtifactActorConfig): string {
  const hub = hubBinding(config);
  return hub === null ? documentRuntimeKeyV1({ kind: "local", documentId: config.documentId }) : documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: hub.spaceId, documentId: config.documentId });
}

export type DocumentBrowserActorChild = Awaited<ReturnType<typeof reserveBrowserActorChild>>;
export type DocumentBrowserActorOpen = Readonly<{ binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>; intent: DocumentOpenIntentV1; assertCurrent(): void }>;
export type DocumentBrowserActorGrant = Readonly<{ actorId: string; reserveBeforeMs: number; retireAtMs: number }>;
export const DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT = 256;

export function browserActorRecord(value: BrowserActorChildValue|undefined, code: string): Record<string, BrowserActorChildValue> {
  if (!value || typeof value !== "object" || Array.isArray(value) || value instanceof Uint8Array || value instanceof ArrayBuffer) throw new Error(code);
  return value;
}

function unwrapBrowserActorOption(value: BrowserActorChildValue | undefined): BrowserActorChildValue | undefined {
  if (value === null || value === undefined) return undefined;
  const record = browserActorRecord(value, "document browser actor: invalid option");
  if (record.tag === "none" && Object.keys(record).length === 1) return undefined;
  if (record.tag === "some" && Object.keys(record).length === 2 && Object.hasOwn(record, "val")) return record.val;
  return value;
}

/** 🩹️ Reads the guest's own `option<ui-patch-receipt>` as the WIT RECORD it is. The shard wire
 * carries this receipt as varint bytes and the browser-bundle child does not: it answers the WIT
 * `turn-result` directly, so a byte reader refuses every receipt a patch turn issues. */
export function browserActorUiPatchReceipt(value: BrowserActorChildValue | undefined): ActorUiPatchReceipt | null {
  const raw = unwrapBrowserActorOption(value);
  if (raw === undefined) return null;
  const record = browserActorRecord(raw, "document browser actor: invalid patch receipt");
  const sequence = record.patchSequence;
  if (typeof sequence !== "bigint" || sequence < 1n || sequence > 0xffffffffffffffffn) throw new Error("document browser actor: invalid patch receipt sequence");
  return { lifetime: parseColdDocumentPairLifetime(record.lifetime), patchSequence: sequence };
}

export function browserActorTurnResult(value: BrowserActorChildValue): Record<string, BrowserActorChildValue> {
  const result = browserActorRecord(value, "document browser actor: invalid turn result");
  const status = browserActorRecord(result.status, "document browser actor: invalid turn status");
  if (status.tag === "faulted") throw new Error("document browser actor: guest turn fault");
  if (status.tag !== "idle" && status.tag !== "more-work" && status.tag !== "checkpoint-ready") throw new Error("document browser actor: invalid turn status");
  return result;
}

export function browserActorCapturedReceipt(value: BrowserActorChildValue, request: ActorInstanceOpenRequest): ActorInstanceLifecycleReceipt {
  const result = browserActorTurnResult(value);
  const raw = unwrapBrowserActorOption(result.lifecycleReceipt);
  const tagged = browserActorRecord(raw ?? null, "document browser actor: missing captured receipt");
  const body = browserActorRecord(tagged.val, "document browser actor: invalid captured receipt");
  const sequence = body.requestSequence;
  if (tagged.tag !== "captured" || typeof sequence !== "bigint" || sequence < 1n || sequence > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("document browser actor: invalid captured receipt");
  const receipt: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: parseColdDocumentPairLifetime(body.lifetime), requestSequence: Number(sequence) };
  if (!actorInstanceCapturedReceiptMatches(request, receipt)) throw new Error("document browser actor: captured receipt mismatch");
  return receipt;
}

export function browserActorCloseReceipt(value: BrowserActorChildValue, request: ActorInstanceCloseRequest, accepted: ActorInstanceLifecycleReceipt | null): ActorInstanceLifecycleReceipt | null {
  const raw = unwrapBrowserActorOption(browserActorTurnResult(value).lifecycleReceipt);
  if (raw === undefined) return null;
  const tagged = browserActorRecord(raw, "document browser actor: invalid close receipt");
  const body = browserActorRecord(tagged.val, "document browser actor: invalid close receipt body");
  if (tagged.tag !== "accepted" && tagged.tag !== "retired") throw new Error("document browser actor: invalid close receipt phase");
  const sequence = body.requestSequence,
    closeGeneration = body.closeGeneration;
  if (typeof sequence !== "bigint" || sequence < 1n || sequence > BigInt(Number.MAX_SAFE_INTEGER) || typeof closeGeneration !== "bigint" || closeGeneration < 1n || closeGeneration > 0xffffffffffffffffn) throw new Error("document browser actor: invalid close receipt authority");
  const receipt: ActorInstanceLifecycleReceipt = { kind: tagged.tag, lifetime: parseColdDocumentPairLifetime(body.lifetime), requestSequence: Number(sequence), closeGeneration };
  if (!actorInstanceCloseReceiptMatches(request, accepted, receipt)) throw new Error("document browser actor: close receipt mismatch");
  return receipt;
}

/** 🩻️ Names what a cold-pair turn actually answered. A page receipt that is merely refused as
 * "invalid" hides the difference between a guest that faulted, one that asked for backpressure and
 * one that answered for a different page — the three things an operator needs to tell apart. */
export function coldPairIngressStatusDiagnostic(status: ColdPairIngressStatus): string {
  if (status.kind === "idle" || status.kind === "applied") return status.kind;
  if (status.kind === "fault") return `fault page ${status.cursor.pageIndex + 1}/${status.cursor.pageCount}: ${boundChildText(new TextDecoder().decode(status.fault), EXECUTION_TARGET_DIAGNOSTIC_MAX_BYTES)}`;
  return `${status.kind} page ${status.cursor.pageIndex + 1}/${status.cursor.pageCount} generation ${status.cursor.transferGeneration}`;
}

/** ⏱️ How long and how hard the host waits on a cold pair whose guest keeps answering `loading` (its whole-document load still
 * folding the history, `📓️api-stepped-document-load.md` §8) or `backpressure` (an older pair still settling). The bound is
 * wall time (design §20.14): `deadlineMs` from the first answer, `maximumTurns` only a secondary cap. The first `eagerTurns`
 * turns follow each other at once — every one gives the guest a full turn budget, so a progressing load is never slowed —
 * and later turns back off exponentially from `backoffMs` to `maximumBackoffMs`, so a guest stuck in `loading` stops
 * spinning the worker until the deadline refuses it. `now`/`sleep` are the clock (a fake one in the law). */
export type ColdPairWaitPolicy = Readonly<{
  deadlineMs: number;
  maximumTurns: number;
  eagerTurns: number;
  backoffMs: number;
  maximumBackoffMs: number;
  now: () => number;
  sleep: (ms: number) => Promise<void>;
}>;

/** ⏱️ The host's cold-pair wait: 5 minutes of wall time, back to back for the first 4096 turns (≈ 20 s of continuous
 * folding at a 4–5 ms turn), then backing off 1 → 64 ms. */
export const COLD_PAIR_WAIT_POLICY: ColdPairWaitPolicy = Object.freeze({
  deadlineMs: 300_000,
  maximumTurns: 1 << 20,
  eagerTurns: 4096,
  backoffMs: 1,
  maximumBackoffMs: 64,
  now: () => performance.now(),
  sleep: (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms)),
});

/** 📈️ The pause the host takes before wait turn `turn` (0-based): none during the eager turns, then `backoffMs` doubling
 * per turn up to `maximumBackoffMs`. */
export function coldPairWaitDelay(policy: ColdPairWaitPolicy, turn: number): number {
  if (turn < policy.eagerTurns) return 0;
  return Math.min(policy.maximumBackoffMs, policy.backoffMs * 2 ** Math.min(turn - policy.eagerTurns, 30));
}

/** ⏳️ Waits before the next turn of a cold-pair wait that began at `started`, refusing with `exhausted` once the turn cap or the
 * wall deadline is reached; a backoff never sleeps past the deadline. */
async function coldPairWaitTurn(policy: ColdPairWaitPolicy, started: number, turn: number, unit: string, exhausted: (why: string) => Error): Promise<void> {
  if (turn >= policy.maximumTurns) throw exhausted(`${turn} ${unit}`);
  const remaining = policy.deadlineMs - (policy.now() - started);
  if (remaining <= 0) throw exhausted(`${policy.deadlineMs} ms (${turn} ${unit})`);
  const delay = Math.min(coldPairWaitDelay(policy, turn), remaining);
  if (delay > 0) await policy.sleep(delay);
}

/** ⏳️ Settles the last page of a cold-pair transfer whose guest answers `loading` while its whole-document load runs across
 * turns: polls empty turns until the guest answers anything else, holds every `loading` to exactly the last page of this
 * transfer, and releases every answer it moves past. Answers the settling turn (the caller requires `applied`). Cancelling
 * is `poll` throwing — the owner stopped being current — and leaves the guest to restore its previous document. The wait is
 * bounded and paced by `policy` ([`ColdPairWaitPolicy`]). */
export async function settleColdPairLoading<T>(first: T, status: (value: T) => ColdPairIngressStatus, expected: ColdDocumentPairCursor, poll: () => Promise<T>, release: (value: T) => void, policy: ColdPairWaitPolicy = COLD_PAIR_WAIT_POLICY): Promise<T> {
  const started = policy.now();
  let value = first;
  for (let turn = 0; ; turn += 1) {
    const current = status(value);
    if (current.kind !== "loading") return value;
    release(value);
    if (!coldDocumentPairCursorEquals(current.cursor, expected)) throw new Error(`document browser actor: invalid loading receipt (${coldPairIngressStatusDiagnostic(current)})`);
    await coldPairWaitTurn(policy, started, turn, "turns", (why) => new Error(`document browser actor: cold pair still loading after ${why}`));
    value = await poll();
  }
}

/** 🚦️ Sends one cold-pair page until the guest takes it: a `backpressure` answer names the pair the same guest lifetime
 * still settles (an older transfer whose whole-document load the guest is cancelling, `📓️api-stepped-document-load.md` §8),
 * so the host releases it, gives the guest one empty turn to finish that pair, and sends a fresh copy of the page. A
 * backpressure for another lifetime is refused; every answer moved past is released. Answers the first other answer. The
 * resends are bounded and paced by `policy` ([`ColdPairWaitPolicy`]). */
export async function retryColdPairBackpressure<T>(send: () => Promise<T>, status: (value: T) => ColdPairIngressStatus, lifetime: ActorInstanceLifetime, poll: () => Promise<T>, release: (value: T) => void, policy: ColdPairWaitPolicy = COLD_PAIR_WAIT_POLICY): Promise<T> {
  const started = policy.now();
  for (let attempt = 0; ; attempt += 1) {
    const value = await send();
    const current = status(value);
    if (current.kind !== "backpressure") return value;
    release(value);
    if (!actorInstanceLifetimeEquals(current.cursor.lifetime, lifetime)) throw new Error(`document browser actor: invalid backpressure receipt (${coldPairIngressStatusDiagnostic(current)})`);
    release(await poll());
    await coldPairWaitTurn(policy, started, attempt + 1, "attempts", (why) => new Error(`document browser actor: cold page still refused after ${why}`));
  }
}

export function browserActorColdStatus(value: BrowserActorChildValue, allowLifecycleReceipt = false): ColdPairIngressStatus {
  const result = browserActorTurnResult(value);
  if (!allowLifecycleReceipt && unwrapBrowserActorOption(result.lifecycleReceipt) !== undefined) throw new Error("document browser actor: unexpected lifecycle receipt");
  return parseWitColdPairIngressStatus(result.coldPairIngress);
}

export function wipeBrowserActorValue(value: BrowserActorChildValue): void {
  for (const buffer of measureChildValue(value, BROWSER_ACTOR_CHILD_LIMITS.outputBytes).transfers) if (buffer.byteLength) new Uint8Array(buffer).fill(0);
}

export function browserActorTurnBudget(): BrowserActorChildValue {
  return { fuel: 80_000_000n, deadlineMs: BROWSER_ACTOR_CHILD_LIMITS.invokeMs, maxEffects: 512, maxPatchBytes: 2_097_152, maxFrames: 8 };
}

export function browserActorBytes(value: BrowserActorChildValue | undefined, code: string): Uint8Array {
  if (value instanceof Uint8Array) return value.slice();
  if (!Array.isArray(value) || value.length === 0 || value.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || value.some((byte) => typeof byte !== "number" || !Number.isInteger(byte) || byte < 0 || byte > 255)) throw new Error(code);
  return Uint8Array.from(value);
}

function validateBrowserActorUiValues(values: readonly (PackValue|undefined)[]): void {
  const encoder = new TextEncoder(),
    pending = values.map(value => ({ value, depth: 0 }));
  let items = 0;
  while (pending.length > 0) {
    const entry = pending.pop();if(!entry)break;
    if(entry.value===undefined)throw Error("document browser actor: invalid intent value");
    items++;
    if (items > 256 || entry.depth > 256) throw new Error("document browser actor: intent value capacity");
    if (entry.value === null || typeof entry.value === "boolean") continue;
    if (typeof entry.value === "number") {
      if (!Number.isFinite(entry.value)) throw new Error("document browser actor: invalid intent number");
      continue;
    }
    if (typeof entry.value === "string") {
      if (encoder.encode(entry.value).byteLength > 512) throw new Error("document browser actor: intent text capacity");
      continue;
    }
    if (isPackInteger(entry.value)) continue;
    if (Array.isArray(entry.value)) {
      if (entry.value.length > 256) throw new Error("document browser actor: intent value capacity");
      for (let index = entry.value.length - 1; index >= 0; index--) pending.push({ value: entry.value[index], depth: entry.depth + 1 });
      continue;
    }
    if (entry.value === null || typeof entry.value !== "object" || entry.value instanceof Uint8Array) throw new Error("document browser actor: invalid intent value");
    const fields = Object.entries(entry.value);
    if (fields.length > 256) throw new Error("document browser actor: intent value capacity");
    for (let index = fields.length - 1; index >= 0; index--) {
      const field=fields[index];if(!field)throw Error("document browser actor: intent field required");const [key,value]=field;
      if (encoder.encode(key).byteLength > 512) throw new Error("document browser actor: intent text capacity");
      pending.push({ value, depth: entry.depth + 1 });
    }
  }
}

/** 🧭️ Admits one canonical UI intent and names the rendered surface (the window or a panel body) it was made on;
 * whether that surface was painted at the intent's revision is the reservation's check. */
export function browserActorUiIntentV1(request: BrowserActorActionRequestV1): Readonly<{ surfaceKey: string; bytes: Uint8Array }> {
  if (request.payload.kind !== "ui-intent") throw new Error("document browser actor: invalid intent payload");
  const bytes = Uint8Array.from(request.payload.bytes),
    decoded = decodePackValue(bytes),
    encoder = new TextEncoder();
  if (decoded === null || typeof decoded !== "object" || Array.isArray(decoded)) throw new Error("document browser actor: invalid intent record");
  const value = decoded as Readonly<Record<string, PackValue>>,
    keys = ["action", "args", "input", "node", "nodeKey", "revision", "seq", "surface", "trigger"];
  if (Object.keys(value).sort().join(",") !== keys.sort().join(",")) throw new Error("document browser actor: invalid intent fields");
  const action = value.action;
  if (action === null || typeof action !== "object" || Array.isArray(action)) throw new Error("document browser actor: invalid intent action");
  const actionRecord = action as Readonly<Record<string, PackValue>>;
  if (Object.keys(actionRecord).sort().join(",") !== "name,scope,version" || typeof actionRecord.scope !== "string" || typeof actionRecord.name !== "string" || actionRecord.scope.length === 0 || actionRecord.name.length === 0 || encoder.encode(actionRecord.scope).byteLength > 512 || encoder.encode(actionRecord.name).byteLength > 512) throw new Error("document browser actor: invalid intent action");
  const uint = (field: PackValue | undefined, maximum: bigint, code: string): bigint => {
    if (!isPackInteger(field) || field.kind !== "uint" || field.value > maximum) throw new Error(code);
    return field.value;
  };
  const prefix = `${request.instanceId}:`,
    surface = value.surface;
  if (
    typeof surface !== "string" ||
    !surface.startsWith(prefix) ||
    surface.length === prefix.length ||
    encoder.encode(surface).byteLength > 512 ||
    uint(value.revision, BigInt(Number.MAX_SAFE_INTEGER), "document browser actor: invalid intent revision") !== BigInt(request.surfaceRevision) ||
    uint(value.node, BigInt(Number.MAX_SAFE_INTEGER), "document browser actor: invalid intent node") > BigInt(Number.MAX_SAFE_INTEGER) ||
    uint(actionRecord.version, 0xffffn, "document browser actor: invalid intent action version") > 0xffffn ||
    uint(value.seq, 0xffffffffffffffffn, "document browser actor: invalid intent sequence") > 0xffffffffffffffffn ||
    typeof value.nodeKey !== "string" || encoder.encode(value.nodeKey).byteLength > 512 ||
    typeof value.trigger !== "string" || !["abort", "activate", "change", "commit", "delta", "drop", "hoverPreview", "repeatLast", "submit"].includes(value.trigger)
  ) throw new Error("document browser actor: intent owner mismatch");
  validateBrowserActorUiValues([value.args, value.input]);
  const canonical = encodePackValue(value);
  if (canonical.byteLength !== bytes.byteLength || canonical.some((byte, index) => byte !== bytes[index])) throw new Error("document browser actor: noncanonical intent");
  return { surfaceKey: surface.slice(prefix.length), bytes };
}

function browserActorExactRecord(value: PackValue | undefined, fields: readonly string[], code: string): Readonly<Record<string, PackValue>> {
  if (value === null || typeof value !== "object" || Array.isArray(value) || value instanceof Uint8Array) throw new Error(code);
  const record = value as Readonly<Record<string, PackValue>>;
  if (Object.keys(record).sort().join(",") !== [...fields].sort().join(",")) throw new Error(code);
  return record;
}

function browserActorPackRecord(value: PackValue | undefined, code: string): Readonly<Record<string, PackValue>> {
  if (value === null || typeof value !== "object" || Array.isArray(value) || value instanceof Uint8Array) throw new Error(code);
  return value as Readonly<Record<string, PackValue>>;
}

function browserActorOwnedText(value: PackValue | undefined, expected: string | null, code: string): string {
  if (typeof value !== "string" || value.length === 0 || new TextEncoder().encode(value).byteLength > 256 || (expected !== null && value !== expected)) throw new Error(code);
  return value;
}

/** 🎛️ Admits one canonical app command against the verified target and names the window it was issued in: any window
 * kind of the verified app, addressed at its base instance, with a view state bound to that same window. A command
 * addressed by id (no window in its address) is issued in the view state's active window, or the lease window. */
export function browserActorAppCommandV1(request: BrowserActorActionRequestV1, fields: DocumentExecutionTargetLeaseFieldsV1, windowKeys: ReadonlySet<string>): Readonly<{ bytes: Uint8Array; surfaceKey: string }> {
  if (request.payload.kind !== "app-command") throw new Error("document browser actor: invalid command payload");
  const bytes = Uint8Array.from(request.payload.bytes),
    command = decodeAppCommand(bytes);
  if (!("Command" in command) || command.Command.seq !== request.actionSequence) throw new Error("document browser actor: command sequence mismatch");
  const canonical = encodeAppCommand(command);
  if (canonical.byteLength !== bytes.byteLength || canonical.some((byte, index) => byte !== bytes[index])) throw new Error("document browser actor: noncanonical command");
  const invocationBytes = Uint8Array.from(command.Command.command),
    viewStateBytes = Uint8Array.from(command.Command.view_state),
    invocation = browserActorExactRecord(decodePackValue(invocationBytes), ["address", "arguments"], "document browser actor: invalid command invocation"),
    rawAddress = browserActorPackRecord(invocation.address, "document browser actor: invalid command address"),
    address = browserActorExactRecord(rawAddress, Object.hasOwn(rawAddress, "pluginId") ? ["actionId", "appId", "modeId", "pluginId", "windowInstanceId", "windowKindId"] : ["commandId", "owner"], "document browser actor: invalid command address"),
    viewState = browserActorPackRecord(decodePackValue(viewStateBytes), "document browser actor: invalid command view state"),
    invocationCanonical = encodePackValue(invocation),
    viewStateCanonical = encodePackValue(viewState);
  validateBrowserActorUiValues([invocation.arguments, viewState]);
  if (invocationCanonical.byteLength !== invocationBytes.byteLength || invocationCanonical.some((byte, index) => byte !== invocationBytes[index]) || viewStateCanonical.byteLength !== viewStateBytes.byteLength || viewStateCanonical.some((byte, index) => byte !== viewStateBytes[index])) throw new Error("document browser actor: noncanonical command values");
  const pluginId = fields.package.pluginId,
    appId = fields.surface.appId,
    activeWindowKindId = viewState.activeWindowKindId;
  let windowKindId = typeof activeWindowKindId === "string" ? activeWindowKindId : fields.surface.windowKindId;
  if ("pluginId" in address) {
    browserActorOwnedText(address.pluginId, pluginId, "document browser actor: command owner mismatch");
    browserActorOwnedText(address.appId, appId, "document browser actor: command owner mismatch");
    browserActorOwnedText(address.modeId, null, "document browser actor: invalid command mode");
    windowKindId = browserActorOwnedText(address.windowKindId, null, "document browser actor: command owner mismatch");
    browserActorOwnedText(address.windowInstanceId, windowKindId, "document browser actor: command owner mismatch");
    browserActorOwnedText(address.actionId, null, "document browser actor: invalid command action");
  } else {
    browserActorOwnedText(address.commandId, null, "document browser actor: invalid command id");
    if (address.owner === "os") throw new Error("document browser actor: command owner mismatch");
    const rawOwner = browserActorPackRecord(address.owner, "document browser actor: invalid command owner"),
      ownerKeys = Object.keys(rawOwner);
    if (ownerKeys.length !== 1) throw new Error("document browser actor: invalid command owner");
    const ownerKind = ownerKeys[0],
      ownerRecord = browserActorExactRecord(rawOwner, [ownerKind!], "document browser actor: invalid command owner");
    if (ownerKind !== "plugin" && ownerKind !== "app" && ownerKind !== "mode") throw new Error("document browser actor: invalid command owner");
    const ownerFields = browserActorExactRecord(ownerRecord[ownerKind], ownerKind === "plugin" ? ["pluginId"] : ownerKind === "app" ? ["appId", "pluginId"] : ["appId", "modeId", "pluginId"], "document browser actor: invalid command owner");
    browserActorOwnedText(ownerFields.pluginId, pluginId, "document browser actor: command owner mismatch");
    if (ownerKind !== "plugin") browserActorOwnedText(ownerFields.appId, appId, "document browser actor: command owner mismatch");
    if (ownerKind === "mode") browserActorOwnedText(ownerFields.modeId, typeof viewState.activeModeId === "string" ? viewState.activeModeId : null, "document browser actor: command owner mismatch");
  }
  if (!windowKeys.has(windowKindId)) throw new Error("document browser actor: command owner mismatch");
  if (activeWindowKindId !== undefined && activeWindowKindId !== windowKindId) throw new Error("document browser actor: command view owner mismatch");
  if (viewState.windowId !== undefined && viewState.windowId !== windowKindId) throw new Error("document browser actor: command view owner mismatch");
  return { bytes, surfaceKey: windowKindId };
}

export function browserActorActionDisposition(request: BrowserActorActionRequestV1, outcome: "guest-applied" | "rejected", mutationCount: number, commit: BrowserActorActionCommitReceiptV1 | null, hostEffects: readonly (readonly number[])[] = [], reason?: string, historyPatches: readonly (readonly number[])[] = []): BrowserActorActionResultV1 {
  return {
    kind: "browser-actor-action-result",
    scope: { ...request.scope },
    verifiedSurfaceId: request.verifiedSurfaceId,
    appChannelVersion: request.appChannelVersion,
    activationGeneration: request.activationGeneration,
    instanceId: request.instanceId,
    surfaceRevision: request.surfaceRevision,
    actionSequence: request.actionSequence,
    outcome,
    mutationCount,
    commit,
    hostEffects,
    historyPatches,
    ...(reason === undefined ? {} : { reason }),
  };
}

export type BrowserActorActionPublication = { readonly kind: "ui-intent" | "app-command"; readonly sequence: number; frames: number; commit: BrowserActorActionCommitReceiptV1 | null; readonly refusals: string[]; readonly hostEffects: (readonly number[])[]; readonly historyPatches: (readonly number[])[] };

/** 🚫️ The guest refused the action (a typed-operation fault page or its `AppFrame::Error` reply) — raised only after the
 * whole turn was answered, so the actor stays open; `detail` is the guest's first fault as the shell displays it. */
export class BrowserActorGuestRefusalV1 extends Error {
  constructor(readonly detail: string | null) {
    super("action-guest-refused");
  }
}

export function browserActorCommandIngressStatus(value: BrowserActorChildValue): CommandIngressKindV1 {
  const ingress = browserActorRecord(browserActorTurnResult(value).commandIngress, "document browser actor: invalid command ingress"),
    tag = typeof ingress.kind === "number" ? COMMAND_INGRESS_KINDS[ingress.kind] : undefined;
  if (tag === undefined) throw new Error("document browser actor: invalid command ingress");
  return tag;
}

/** 🧵️ One `spawn-job` a guest turn handed back; every framework reserved tool verb (`clearSelection`, `interactionSelect`, …) arrives as one. */
export type BrowserActorSpawnedJob = Readonly<{ job: bigint; kind: string; input: Uint8Array }>;

export function hubBinding(config: ArtifactActorConfig): Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }> | null {
  const binding = config.bindings.find((entry): entry is Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }> => entry.kind === "hub");
  return binding ?? null;
}
