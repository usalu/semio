import { OwnedResidentLedger, type ResidentStep, type ResidentGrant, type OwnedResidentAdmission, type OwnedResidentRecord } from "@semio-tech/value-ts/resident";
import { uiResidentMetadataEnvelope } from "@semio-tech/framework/modules/ui/contract/retained/resident/metadata";
import { OwnedActorTurnOutput, OwnedActorTurnOutputs } from "../../🪪️activation/🚪️instance/📥️output/🟦️.ts";
import { type ActorInstanceOpenRequest, type ActorInstanceLifetime, type ActorInstanceLifecycleReceipt, type ActorInstanceCloseRequest, actorInstanceLifetimeEquals } from "../../🚪️lifetime/🟦️.ts";
import { OwnedUiInstanceRetirement, OwnedUiInstance, OwnedUiPatchAcknowledgement } from "@semio-tech/framework/modules/ui/contract/retained/instance";
import type { OriginalOperationAdmission } from "../../🎟️retained-turn/🟦️.ts";
import type { ActorReturnResult, ActorReturnPageReceipt, ActorReturnOrigin, ActorReturnIdentity } from "../../📤️return/🟦️.ts";
import type { ActorBytePage } from "../../📃️page/🟦️.ts";
import type { ShardWorkerLike } from "../🚨️quota/🟦️.ts";
import type { ShardHeartbeatState } from "../💓️heartbeat/🟦️.ts";
import type { ShardAsset, ShardBudget, ShardCapabilityGrant, ShardCommandIngressPage, ShardEventEnvelope } from "../📜️wire/🟦️.ts";
import type { OwnedNativeUiPatchAuthority, OwnedNativeUiPatchSubmissionReceipt, OwnedUiPatchAcknowledgementEntry } from "../🩹️ui-patch/🟦️.ts";
import { returnContentMatches } from "../../../🖱️ui/🧬️contract/🧵️retained/🔌️host/📦️return-content/🟦️.ts";
/** 📦️ The decoded content a consumer binds to a published return; the actor holds it opaquely and never calls into it. */
export type ShardReturnContent = object;

/** 🧵️ The opaque identity of the owning `ShardClient` shell: activations and captured returns compare it by reference and never call into it. */
export type ShardClientIdentity = object;

/** 🧬️ Brand-check accessor for {@link OwnedResidentLedger}, resolved LAZILY on first use.
 * `OwnedResidentLedger` arrives over an import cycle (`📮️shard-client` → `🎠️kernel/📥️input` →
 * `🖱️ui/…/💾️resident` → back here), and reading `.prototype` at module-evaluation time touches the
 * binding while that cycle is still initializing — which is a TDZ
 * (`ReferenceError: Cannot access 'X' before initialization`) in any bundled build, killing the whole
 * preview before it mounts. Deferring the lookup to first call moves it past module evaluation; the
 * check itself is unchanged. */
let residentCapacityGetter: (() => number) | undefined;
export const residentCapacity = (): (() => number) => (residentCapacityGetter ??= Object.getOwnPropertyDescriptor(OwnedResidentLedger.prototype, "capacity")!.get! as () => number);
export const NO_RESIDENT_FAULT = Symbol("actor-resident.no-fault");
export const poolUiEnvelope = uiResidentMetadataEnvelope("pool");
export const poolRecordEnvelope = poolUiEnvelope;
export const poolControllerEnvelope = Object.freeze({ bytes: 224, slots: 1, owners: 1 });
export const workerControllerEnvelope = Object.freeze({ bytes: 128, slots: 0, owners: 0 });
export const residentStep = (kind: ResidentStep["kind"], phase: string, bytes = 0): ResidentStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
export const residentGrant = (grant: ResidentGrant, bytes: number): boolean => Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= bytes;
export function residentChild(current: ResidentStep, grant: ResidentGrant): ResidentStep {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) return residentStep("rejected", "actor-resident.child-grant");
  return current.kind === "complete" || current.kind === "ready" ? { ...current, kind: "pending" } : current;
}

/** 🩺️ Rebuilds a worker-side failure as a main-thread `Error` that still carries the worker's own
 * stack. Without this the only frame a caller ever sees is `handleMessage`, because the structured
 * clone across `postMessage` cannot carry an `Error` — which is exactly why the collaboration e2e's
 * `Maximum call stack size exceeded` was undiagnosable. The `[TRACE] ` line is deliberate, permanent
 * diagnostic infrastructure the e2e log parses; it is not leftover scaffolding. */
export function graftWorkerStack(actorId: string, reason: string, stack: string | undefined, kind: string | undefined, framesBytes: number | undefined, retryableLifecycle?: boolean): Error {
  const error = new Error(reason);
  if (stack) error.stack = `${stack}\n    \u21b3 main: ${error.stack ?? ""}`;
  if (retryableLifecycle === true) Object.defineProperty(error, RETRYABLE_LIFECYCLE_TURN, { value: true });
  return error;
}

/** ⏱️ Marks a rejection the worker classified as a RETRYABLE guest lifecycle-turn deadline — the
 * browser twin of the host's `retryable_lifecycle_turn` verdict (`🔌plugin/🖥host/🦀.rs`). The guest
 * retained its receipt, so `ShardClient.sendInstanceLifecycle` replays the same events instead
 * of failing the open; a symbol keeps the marker off every structured-clone and JSON path. */
export const RETRYABLE_LIFECYCLE_TURN = Symbol("semio.actor.retryable-lifecycle-turn");

/** ⏱️ Whether a rejection carries the {@link RETRYABLE_LIFECYCLE_TURN} marker. */
export function isRetryableLifecycleTurn(error: unknown): boolean {
  return typeof error === "object" && error !== null && (error as Record<symbol, unknown>)[RETRYABLE_LIFECYCLE_TURN] === true;
}

/** ⏱️ How many times one lifecycle turn replays a retryable deadline before the open is failed.
 * Native `ShardLoop::pump` re-grants without a bound because the shard scheduler is the bound there;
 * in the browser the shard watchdog only sees a wedged worker, so the replay carries its own. */
export const RETRYABLE_LIFECYCLE_TURN_ATTEMPTS = 16;

export type PendingEntry = { readonly resolve: (value: unknown) => void; readonly reject: (error: unknown) => void; readonly slot: ShardSlot; readonly startedAtMs: number; readonly actorId: string; readonly kind: string; readonly firstTurn: boolean; readonly output: OwnedActorTurnOutput | null };

export type ShardSlot = {
  index: number;
  worker: ShardWorkerLike;
  available: boolean;
  readonly heartbeat: ShardHeartbeatState;
  readonly pendingRequestIds: Set<string>;
  readonly actorIds: Set<string>;
};

/** 🧾️ Supplies the authored open payload without permitting arbitrary event submission. */
export interface ShardInstanceOpenInput {
  readonly appId: string;
  readonly actor: unknown;
  readonly config: Uint8Array | readonly number[];
  readonly assets: readonly ShardAsset[];
  readonly capabilities: readonly ShardCapabilityGrant[];
  readonly quotas: Uint8Array | readonly number[];
}

/** 🚪️ Retains lifecycle authority independently of revocable operation admission. */
export interface ShardInstanceLifecycleLease {
  readonly activation: ShardActorActivationLease;
  readonly openRequest: ActorInstanceOpenRequest;
  readonly lifetime: ActorInstanceLifetime | null;
  readonly pendingReceipt: ActorInstanceLifecycleReceipt | null;
  readonly interruptedTurn: unknown;
  readonly pendingReturn: OwnedShardReturn | null;
  reserveReturn(maximumResponses: number, grant: ResidentGrant): ShardReturnAdmission;
  retireUnusedReturn(grant: ResidentGrant): ResidentStep;
  open(input: ShardInstanceOpenInput, budget: ShardBudget): Promise<unknown>;
  poll(budget: ShardBudget): Promise<unknown>;
  beginClose(): ActorInstanceCloseRequest;
  close(budget: ShardBudget): Promise<unknown>;
  acknowledge(receipt: ActorInstanceLifecycleReceipt, budget: ShardBudget, retirement?: OwnedUiInstanceRetirement): Promise<unknown>;
  bindHostRetirement(participant: OwnedUiInstance): void;
  captureUiPatchAuthority(originalTurn: object, patchIndex: number): OwnedNativeUiPatchAuthority;
  submitUiAcknowledgement(source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement, budget: ShardBudget): Promise<{ readonly receipt: OwnedNativeUiPatchSubmissionReceipt; readonly result: unknown }>;
  submitUiAcknowledgements(entries: readonly OwnedUiPatchAcknowledgementEntry[], budget: ShardBudget): Promise<{ readonly receipts: readonly OwnedNativeUiPatchSubmissionReceipt[]; readonly result: unknown }>;
  dispose(): void;
  progress(): { readonly kind: ShardInstancePhase | "blocked"; readonly failure: ShardInstanceFailure | null };
}

/** 🪪️ Pins operation admission to one activation and worker; revocation does not release its close owner. */
export interface ShardActorActivationLease {
  readonly actorId: string;
  readonly activationGeneration: bigint;
  assertActive(): void;
  turn(events: readonly ShardEventEnvelope[], budget: ShardBudget, commandPage?: ShardCommandIngressPage): Promise<unknown>;
}

const ACTIVATION_MINT = Symbol("actor-activation.private-lease");
export let mintCapturedActivation: (client: ShardClientIdentity, actorId: string, generation: bigint, assertActive: ShardActorActivationLease["assertActive"], turn: ShardActorActivationLease["turn"]) => ShardActorActivationLease;
export let capturedActivationMatches: (activation: unknown, client: ShardClientIdentity) => boolean;
/** 🧷️ Original composition identity survives revocation without granting new operation authority. */
class CapturedShardActivation implements ShardActorActivationLease {
  readonly #client: ShardClientIdentity;
  readonly actorId: string;
  readonly activationGeneration: bigint;
  readonly assertActive: ShardActorActivationLease["assertActive"];
  readonly turn: ShardActorActivationLease["turn"];
  private constructor(mint: symbol, client: ShardClientIdentity, actorId: string, generation: bigint, assertActive: ShardActorActivationLease["assertActive"], turn: ShardActorActivationLease["turn"]) {
    if (mint !== ACTIVATION_MINT) throw new Error("actor-activation.private-lease");
    this.#client = client; this.actorId = actorId; this.activationGeneration = generation; this.assertActive = assertActive; this.turn = turn; Object.freeze(this);
  }
  static {
    mintCapturedActivation = (client, actorId, generation, assertActive, turn) => new CapturedShardActivation(ACTIVATION_MINT, client, actorId, generation, assertActive, turn);
    capturedActivationMatches = (activation, client) => activation !== null && typeof activation === "object" && #client in activation && activation.#client === client;
  }
}

export type ShardActivation = { readonly originalAdmission: OriginalOperationAdmission; readonly slot: ShardSlot; readonly actorId: string; readonly generation: bigint; available: boolean; activated: boolean; teardownPosted: boolean; operationsAllowed: boolean; operationGeneration: bigint; lastGuestLifetime: bigint; lastReturnSequence: bigint; returned: CapturedReturn | null; instance: ShardInstanceOwner | null; close: ShardInstanceOwner | null };
export type PendingHostEffect = { readonly activation: ShardActivation; readonly controller: AbortController; readonly requestId: string; previous: PendingHostEffect | null; next: PendingHostEffect | null };
export type HostEffectLedger = { readonly activation: ShardActivation; readonly requests: Map<string, PendingHostEffect>; head: PendingHostEffect | null; tail: PendingHostEffect | null };
type ShardInstancePhase = "opening" | "captured" | "open" | "closing" | "accepted" | "retired" | "complete";
type ShardInstanceFailure = "transport-refused" | "worker-refused" | "worker-lost" | "invalid-receipt";
export type ShardInstanceOwner = {
  readonly activation: ShardActivation;
  readonly operation: ShardActorActivationLease;
  readonly open: ActorInstanceOpenRequest;
  phase: ShardInstancePhase;
  lifetime: ActorInstanceLifetime | null;
  receipt: ActorInstanceLifecycleReceipt | null;
  accepted: ActorInstanceLifecycleReceipt | null;
  close: ActorInstanceCloseRequest | null;
  host: OwnedUiInstance | null;
  inFlight: boolean;
  failure: ShardInstanceFailure | null;
  interruptedTurn: unknown;
  cancellation: HostEffectLedger | null;
  lastPatchSequence: bigint;
  lastPatchTurn: object | null;
  returnCell: OwnedResidentAdmission | null;
  returnRecord: OwnedResidentRecord | null;
  returnPhase: ReturnAdmissionPhase;
  returnFault: unknown;
  returnCapacity: number;
};

export type ShardReturnReport = Exclude<ActorReturnResult, { kind: "page" }> | { readonly kind: "page"; readonly receipt: ActorReturnPageReceipt };
export type ShardReturnAdmission = { readonly step: ResidentStep; readonly source: OwnedShardReturn | null };
type ReturnAdmissionPhase = "empty" | "preparing" | "cell-held" | "claiming" | "claimed" | "record-admitting" | "record-held" | "installing" | "installed" | "state-held" | "roster-held" | "facade-held" | "published" | "closing" | "rejected";
export type CapturedReturnWork = { readonly kind: "execute"; readonly events: readonly ShardEventEnvelope[] } | { readonly kind: "retry" | "poll" | "cancel" };
export type CapturedReturn = { instance: ShardInstanceOwner | null; outputs: OwnedActorTurnOutputs | null; client: ShardClientIdentity | null; facade: OwnedShardReturn | null; origin: ActorReturnOrigin | null; identity: ActorReturnIdentity | null; events: readonly ShardEventEnvelope[] | null; latest: OwnedActorTurnOutput | null; page: OwnedShardReturnPage | null; content: ShardReturnContent | null; inFlight: boolean; retry: boolean; failed: boolean; fault: unknown; cancelled: boolean; retired: boolean };
const RETURN_MINT = Object.freeze({});
export const NO_RETURN_FAULT = Object.freeze({});
export const returnDomainEnvelope = Object.freeze({ bytes: 800, slots: 4, owners: 4 });
export function returnAdmission(kind: ResidentStep["kind"], phase: string, bytes = 0): ShardReturnAdmission { return { step: residentStep(kind, phase, bytes), source: null }; }
export let mintCapturedReturn: (state: CapturedReturn) => OwnedShardReturn;
export let capturedReturnState: (owner: OwnedShardReturn) => CapturedReturn;
export let submitCapturedReturn: (client: ShardClientIdentity, state: CapturedReturn, work: CapturedReturnWork, budget: ShardBudget) => Promise<ShardReturnReport>;
export let reserveCapturedResponse: (client: ShardClientIdentity, state: CapturedReturn, grant: ResidentGrant) => ResidentStep;
/** 🔗️ Binds the owning client's dispatch for captured-return submission and response reservation; ES imports are read-only, so the client binds through the declaring module. */
export function bindCapturedReturnDispatch<Client extends ShardClientIdentity>(submit: (client: Client, state: CapturedReturn, work: CapturedReturnWork, budget: ShardBudget) => Promise<ShardReturnReport>, reserve: (client: Client, state: CapturedReturn, grant: ResidentGrant) => ResidentStep): void { submitCapturedReturn = submit as typeof submitCapturedReturn; reserveCapturedResponse = reserve as typeof reserveCapturedResponse; }
export let mintCapturedReturnPage: (state: CapturedReturn, output: OwnedActorTurnOutput, receipt: ActorReturnPageReceipt, page: ActorBytePage) => OwnedShardReturnPage;
export function sameReturnOrigin(left: ActorReturnOrigin, right: ActorReturnOrigin): boolean { return left.activationGeneration === right.activationGeneration && left.requestSequence === right.requestSequence; }
export function sameReturnIdentity(left: ActorReturnIdentity, right: ActorReturnIdentity): boolean { return sameReturnOrigin(left.origin, right.origin) && left.returnSequence === right.returnSequence; }

/** 📤️ A captured instance retains fixed responses before callers can observe them; controls carry no new semantic events. */
export class OwnedShardReturn {
  readonly #state: CapturedReturn;
  private constructor(mint: object, state: CapturedReturn) { if (mint !== RETURN_MINT) throw new Error("actor-return.private-owner"); this.#state = state; state.facade = this; Object.freeze(this); }
  static { mintCapturedReturn = state => new OwnedShardReturn(RETURN_MINT, state); capturedReturnState = owner => owner.#state; }
  static matchesOwner(source: unknown, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): source is OwnedShardReturn {
    if (source === null || typeof source !== "object" || !(#state in source)) return false;
    const instance = source.#state.instance;
    return instance !== null && instance.host === owner && instance.operation === activation && instance.lifetime !== null && actorInstanceLifetimeEquals(instance.lifetime, lifetime);
  }
  get origin(): ActorReturnOrigin | null { return this.#state.origin; }
  get page(): OwnedShardReturnPage | null { return this.#state.page; }
  get content(): ShardReturnContent | null { return this.#state.content; }
  bindContent(content: ShardReturnContent): boolean {
    const state = this.#state; const instance = state.instance;
    if (!instance || instance.returnPhase !== "published" || state.content !== null || !instance.host || !instance.lifetime || !returnContentMatches(content, this, instance.host, instance.operation, instance.lifetime)) return false;
    state.content = content; return true;
  }
  get retainedResponses(): number { return this.#state.outputs?.pending ?? 0; }
  reserveResponse(grant: ResidentGrant): ResidentStep { return this.#state.client ? reserveCapturedResponse(this.#state.client, this.#state, grant) : residentStep("rejected", "actor-return.closed"); }
  execute(events: readonly ShardEventEnvelope[], budget: ShardBudget): Promise<ShardReturnReport> { return this.#submit({ kind: "execute", events }, budget); }
  retry(budget: ShardBudget): Promise<ShardReturnReport> { return this.#submit({ kind: "retry" }, budget); }
  poll(budget: ShardBudget): Promise<ShardReturnReport> { return this.#submit({ kind: "poll" }, budget); }
  cancel(budget: ShardBudget): Promise<ShardReturnReport> { return this.#submit({ kind: "cancel" }, budget); }
  #submit(work: CapturedReturnWork, budget: ShardBudget): Promise<ShardReturnReport> { return this.#state.client ? submitCapturedReturn(this.#state.client, this.#state, work, budget) : Promise.reject(new Error("actor-return.closed")); }
}

/** 📄️ Only exact captured response settlement mints this page; its raw response remains strongly retained. */
export class OwnedShardReturnPage {
  readonly #state: CapturedReturn;
  readonly #output: OwnedActorTurnOutput;
  readonly #receipt: ActorReturnPageReceipt;
  readonly #page: ActorBytePage;
  private constructor(mint: object, state: CapturedReturn, output: OwnedActorTurnOutput, receipt: ActorReturnPageReceipt, page: ActorBytePage) { if (mint !== RETURN_MINT) throw new Error("actor-return.private-page"); this.#state = state; this.#output = output; this.#receipt = receipt; this.#page = page; Object.freeze(this); }
  static { mintCapturedReturnPage = (state, output, receipt, page) => new OwnedShardReturnPage(RETURN_MINT, state, output, receipt, page); }
  static matchesOwner(page: unknown, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): page is OwnedShardReturnPage {
    if (page === null || typeof page !== "object" || !(#state in page)) return false;
    const instance = page.#state.instance;
    return instance !== null && instance.host === owner && instance.operation === activation && instance.lifetime !== null && actorInstanceLifetimeEquals(instance.lifetime, lifetime) && page.#receipt.identity.origin.activationGeneration === lifetime.activationGeneration && page.#output.responseEnvelope !== null;
  }
  get receipt(): ActorReturnPageReceipt { return this.#receipt; }
  byteAt(index: number): number {
    if (this.#state.failed || this.#state.cancelled || !Number.isInteger(index) || index < 0 || index >= this.#receipt.length) throw new Error("actor-return.page-read");
    const block = this.#page[`block${Math.floor(index / 64).toString().padStart(2, "0")}` as keyof ActorBytePage] as ActorBytePage["block00"];
    const word = block[`word${Math.floor(index % 64 / 8)}` as keyof typeof block];
    return Number(word >> BigInt(index % 8 * 8) & 255n);
  }
}
