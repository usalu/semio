/** 🧰️ The shared shard client test kit: the destructured dependencies, the fake shard worker, the client harness and the resident, lifecycle and output fixtures of the settlement tests. */
import { fixtureOriginalWorkStock } from "../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🧫️fixtures/🟦️.ts";
import { loadActorCompositionFixture } from "../../../../../../../🔨️modules/🎭️actor/🧪️tests/🧫️fixtures/🟦️.ts";
import { loadRetainedResidentSlotFixture, loadRetainedResidentSlotSchema, loadRetainedResidentPayloadFixture, loadRetainedResidentPayloadSchema, loadRetainedResidentBuilderFixture, loadRetainedResidentBuilderSchema } from "../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts";

/** 🕵️ A getter spy for bun, whose `spyOn` refuses accessor properties: counts reads, optionally replaces the read, restores the original descriptor. */
export const spyGetter = (prototype: object, name: string) => {
  const descriptor = Object.getOwnPropertyDescriptor(prototype, name)!, read = descriptor.get!, calls: unknown[][] = [];
  let implementation: ((this: unknown) => unknown) | undefined;
  const spy = { mock: { calls }, mockClear: () => { calls.length = 0; }, mockRestore: () => { Object.defineProperty(prototype, name, descriptor); }, mockImplementation: (replacement: (this: any) => unknown) => { implementation = replacement; return spy; } };
  Object.defineProperty(prototype, name, { ...descriptor, get(this: unknown) { calls.push([]); return implementation ? implementation.call(this) : Reflect.apply(read, this, []); } });
  return spy;
};
/** ⏳️ Runs a rejection assertion once its promise has settled: vitest's `rejects` returns at once while bun's blocks the test until the promise settles, so a promise that only settles after a later statement (terminate, revoke, release) must be asserted after that statement under both runners. */
export const afterSettle = (promise: Promise<unknown>, assertion: () => Promise<void>): Promise<void> => promise.then(() => undefined, () => undefined).then(assertion);
export type TestSource = { readonly directory: string; readonly url: string };
export type ShardClientInternals = { readonly state: import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🗃️state/🟦️.ts").ShardClientState; readonly hub: import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🧩️wiring/🟦️.ts").ShardClientHub };
/** 🔬 Reads the private hub and the single private state object of a shard client, which the settlement tests observe instead of the former private fields. */
export const shardClientInternals = (client: unknown): ShardClientInternals => { const hub = Reflect.get(client as object, "hub") as ShardClientInternals["hub"]; return { hub, state: hub.state }; };

export type ActorInstanceLifecycleReceipt = import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts").ActorInstanceLifecycleReceipt;
export type ActorInstanceLifetime = import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts").ActorInstanceLifetime;
export type InboundMessage = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/📨️messages/🟦️.ts").InboundMessage;
export type Lane = import("../../../../../../../🔨️modules/🎭️actor/🤖️generated/🎭️actor/🟦️.ts").Lane;
export type OutboundMessage = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/📨️messages/🟦️.ts").OutboundMessage;
export type OwnedResidentAdmission = import("@semio-tech/value-ts/resident").OwnedResidentAdmission;
export type OwnedResidentRecord = import("@semio-tech/value-ts/resident").OwnedResidentRecord;
export type OwnedUiResidentInstance = import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts").OwnedUiResidentInstance;
export type OwnedUiResidentPayload = import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts").OwnedUiResidentPayload;
export type PendingEntry = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🔐️activation/🟦️.ts").PendingEntry;
export type ResidentStep = import("@semio-tech/value-ts/resident").ResidentStep;
export type ShardBudget = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts").ShardBudget;
export type ShardClientOptions = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/⚙️options/🟦️.ts").ShardClientOptions;
export type ShardEnvelope = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts").ShardEnvelope;
export type ShardEventEnvelope = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts").ShardEventEnvelope;
export type ShardFrame = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts").ShardFrame;
export type ShardInstanceLifecycleLease = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🔐️activation/🟦️.ts").ShardInstanceLifecycleLease;
export type ShardInstanceOwner = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🔐️activation/🟦️.ts").ShardInstanceOwner;
export type ShardJspiScope = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/💓️heartbeat/🟦️.ts").ShardJspiScope;
export type ShardReturnAdmission = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🔐️activation/🟦️.ts").ShardReturnAdmission;
export type ShardSlot = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🔐️activation/🟦️.ts").ShardSlot;
export type ShardWorkerLike = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🚨️quota/🟦️.ts").ShardWorkerLike;
export type UiResidentSourceProof = import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts").OwnedUiResidentPayloadSourceRelease;
export type ShardClient = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts").ShardClient;
export type OwnedNativeUiPatchAuthority = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🩹️ui-patch/🟦️.ts").OwnedNativeUiPatchAuthority;
export type OwnedNativeUiPatchSubmissionReceipt = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🩹️ui-patch/🟦️.ts").OwnedNativeUiPatchSubmissionReceipt;
export type OwnedShardReturn = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🔐️activation/🟦️.ts").OwnedShardReturn;
export type OwnedShardReturnPage = import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🔐️activation/🟦️.ts").OwnedShardReturnPage;
export type OwnedActorTurnOutput = import("../../../../../../../🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts").OwnedActorTurnOutput;
export type OwnedActorTurnOutputs = import("../../../../../../../🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts").OwnedActorTurnOutputs;
export type OwnedKernelReturnContent = import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts").OwnedKernelReturnContent;
export type OwnedResidentLedger = import("@semio-tech/value-ts/resident").OwnedResidentLedger;
export type OwnedResidentRetirement = import("@semio-tech/value-ts/resident").OwnedResidentRetirement;
export type OwnedUiInstance = import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts").OwnedUiInstance;
export type OwnedUiInstanceRetirement = import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts").OwnedUiInstanceRetirement;
export type OwnedUiPatchAcknowledgement = import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts").OwnedUiPatchAcknowledgement;
export type OwnedUiPatchInputRetirement = import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts").OwnedUiPatchInputRetirement;
export type OwnedUiResidentPool = import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts").OwnedUiResidentPool;
export type EffectReplyPayload = { readonly requestId: string; readonly value?: unknown; readonly message?: string };
export type EffectReplyMessage = { readonly kind: "frame"; readonly frame: { readonly kind: "Envelope"; readonly envelope: { readonly payload: { readonly kind: "effect-complete" | "effect-error"; readonly payload: EffectReplyPayload } } } };

export class FakeShardWorker implements ShardWorkerLike {
  readonly index: number;
  onmessage: ((event: { readonly data: unknown }) => void) | null = null;
  onerror: ((event: unknown) => void) | null = null;
  onmessageerror: ((event: unknown) => void) | null = null;
  readonly sent: unknown[] = [];
  terminated = false;
  constructor(index: number) {
    this.index = index;
  }
  postMessage(message: unknown): void {
    this.sent.push(message);
  }
  terminate(): void {
    this.terminated = true;
  }
  deliver(message: InboundMessage): void {
    this.onmessage?.({ data: message });
  }
}

export async function flushAdmission(): Promise<void> {
  for (let hop = 0; hop < 32; hop += 1) await Promise.resolve();
}

export function makeEffectRequestFrame(actorId: string, effect: string, requestId: string, params: unknown, activationGeneration = 1n): InboundMessage {
  return { kind: "frame", actorId, activationGeneration, frame: { kind: "Envelope", envelope: { to: "kernel", from: { kind: "actor", id: actorId }, lane: "Background", seq: 1, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind: "effect-request", payload: { effect, requestId, params } } } } };
}

export function findEffectReply(sent: readonly unknown[], requestId: string, kind: "effect-complete" | "effect-error"): EffectReplyMessage | undefined {
  return (sent as readonly { readonly kind: string; readonly frame?: { readonly kind: string; readonly envelope?: { readonly payload?: { readonly kind?: string; readonly payload?: { readonly requestId?: string } } } } }[]).find(
    (message) => message.kind === "frame" && message.frame?.kind === "Envelope" && message.frame.envelope?.payload?.kind === kind && message.frame.envelope.payload?.payload?.requestId === requestId,
  ) as EffectReplyMessage | undefined;
}

export function flushMicrotasks(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

/** 🏭 Creates the shared shard client test kit of one registration from the injected dependencies. */
export function createShardClientSettlementTestKit(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts").ShardClientTestDependenciesV1, testSource: TestSource) {
  const { ACTOR_BYTE_PAGE_BYTES, MAINTENANCE_LANE_DEFAULT_BUDGET, NO_RESIDENT_FAULT, SEGMENTED_DOWNLOAD_CONTRACT, SEGMENTED_DOWNLOAD_REFUSAL, admitSegmentedDownloadChunk, OwnedActorTurnOutput, OwnedActorTurnOutputs, OwnedKernelReturnContent, OwnedNativeUiPatchAuthority, OwnedNativeUiPatchSubmissionReceipt, OwnedResidentLedger, OwnedResidentRetirement, OwnedShardReturn, OwnedShardReturnPage, OwnedUiInstance, OwnedUiInstanceRetirement, OwnedUiPatchAcknowledgement, OwnedUiPatchInputRetirement, OwnedUiResidentPool, SHARD_FRAME_VARIANT_FIELDS, SHARD_JSPI_FAULT_CODE, SHARD_LIVENESS_POLICY, ShardClient, ShardJspiUnavailableError, assertShardJspiAvailable, capturedReturnState, createActorBytePage, createGrantedBudgetTracker, createShardCommandIngressPages, describeShardMessageError, describeShardSilence, describeShardWorkerError, encodeActorInstanceLifecycle, encodeActorUiPatchReceipt, evaluateShardLiveness, interpretShardFrame, isShardLostError, orderEnvelopesByLane, poolControllerEnvelope, poolUiEnvelope, settleFailedInstanceOpen, shardJspiAvailable, uiResidentMetadataEnvelope } = dependencies;
  const { describe, expect, it, vi } = vitest;
  function prepareResidentFixture(client: ShardClient, ledger: OwnedResidentLedger, bytes: readonly number[]): void {
    for (const amount of bytes) expect(client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: amount })).toMatchObject({ kind: "pending", items: 1, bytes: amount });
  }
  function harness(shardCount = 2, extra?: Partial<ShardClientOptions>) {
    const workers: FakeShardWorker[] = [];
    let nowMs = 0;
    const residentLedger = extra?.residentLedger ?? new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } });
    const client = new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(),
      residentLedger,
      shardCount,
      createWorker: (index) => {
        const worker = new FakeShardWorker(index);
        workers.push(worker);
        return worker;
      },
      now: () => nowMs,
      ...extra,
    });
    return { client, residentLedger, workers, advance: (ms: number) => (nowMs += ms), setNow: (ms: number) => (nowMs = ms) };
  }
  async function fixtureResidentPool(client: ShardClient, ledger: OwnedResidentLedger): Promise<OwnedUiResidentPool> {
    const fixture = await loadActorCompositionFixture();
    const { produce } = await import("immer"); const before = ledger.usage.data;
    expect(ShardClient.matchesResidentLedger(client, ledger)).toBe(true);
    for (const bytes of fixture.poolPreparation.prepareBytes) {
      const result = OwnedUiResidentPool.begin(client, ledger, { maxItems: 1, maxBytes: bytes });
      expect(result.pool).toBeNull(); expect(result.step.kind).toBe("pending"); expect(result.step.items).toBe(1); expect(result.step.bytes).toBe(bytes);
    }
    const result = OwnedUiResidentPool.begin(client, ledger, { maxItems: 1, maxBytes: uiResidentMetadataEnvelope("pool").bytes + 64 });
    expect(result.step.kind).toBe("ready"); expect(result.pool).not.toBeNull();
    const pool = result.pool; if (!pool) throw new Error("Exact fixture resident pool was not admitted");
    expect(OwnedUiResidentPool.matchesComposition(pool, client, ledger)).toBe(true); expect(client.ownsUiResidentPool(pool)).toBe(true);
    expect(ledger.usage.data).toEqual(produce(before, value => { value.bytes += fixture.poolPreparation.total.bytes; value.slots += fixture.poolPreparation.total.slots; value.owners += fixture.poolPreparation.total.owners; }));
    return pool;
  }
  async function fixtureResidentScope(pool: OwnedUiResidentPool, ledger: OwnedResidentLedger, lease: ShardInstanceLifecycleLease): Promise<OwnedUiResidentInstance> {
    const { OwnedUiResidentInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts");
    const fixture = await loadRetainedResidentSlotFixture();
    const schema = await loadRetainedResidentSlotSchema(); const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
    
    const owner = fixtureHosts.get(lease); const lifetime = lease.lifetime; if (!owner || !lifetime) throw new Error("Original fixture host has not been captured");
    const before = ledger.usage.data; let scope: OwnedUiResidentInstance | null = null;
    for (let index = 0; index < fixture.firstChild.prepareBytes.length; index++) {
      const bytes = fixture.firstChild.prepareBytes[index]!; const last = index === fixture.firstChild.prepareBytes.length - 1;
      const result = pool.bindInstance(owner, lease.activation, lifetime, { maxItems: 1, maxBytes: bytes });
      expect(result.step).toMatchObject({ kind: last ? "ready" : "pending", items: 1, bytes });
      if (last) { expect(result.scope).not.toBeNull(); scope = result.scope; } else expect(result.scope).toBeNull();
    }
    if (!scope) throw new Error("Exact fixture resident scope was not admitted");
    expect(OwnedUiResidentInstance.matches(scope, owner, lease.activation, lifetime)).toBe(true);
    const repeated = pool.bindInstance(owner, lease.activation, lifetime, { maxItems: 1, maxBytes: 64 }); expect(repeated.scope).toBe(scope); expect(repeated.step).toMatchObject({ kind: "ready", items: 0, bytes: 0 });
    const expected = produce({ ...before }, usage => { usage.bytes += fixture.firstChild.total.bytes; usage.slots += fixture.firstChild.total.slots; usage.owners += fixture.firstChild.total.owners; }); expect(ledger.usage.data).toEqual(expected);
    return scope;
  }
  async function fixtureResidentPayload(scope: OwnedUiResidentInstance, ledger: OwnedResidentLedger, field: NonNullable<OwnedKernelReturnContent["field"]>): Promise<OwnedUiResidentPayload> {
    const { OwnedUiResidentPayload } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts");
    const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
    const fixture = await loadRetainedResidentPayloadFixture(); const schema = await loadRetainedResidentPayloadSchema();
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); 
    const before = ledger.usage.data; let payload: OwnedUiResidentPayload | null = null;
    for (let index = 0; index < fixture.admissionBytes.length; index++) {
      const bytes = fixture.admissionBytes[index]!; const last = index === fixture.admissionBytes.length - 1;
      const short = scope.beginPayload(field, { maxItems: 0, maxBytes: bytes }); expect(short.payload).toBeNull(); expect(short.step).toMatchObject({ kind: "blocked", items: 0, bytes: 0 });
      const result = scope.beginPayload(field, { maxItems: 1, maxBytes: bytes }); expect(result.step, fixture.admissionPhases[index]).toMatchObject({ kind: last ? "ready" : "pending", items: 1, bytes });
      if (last) { expect(result.payload).not.toBeNull(); payload = result.payload; } else expect(result.payload).toBeNull();
    }
    if (!payload) throw new Error("Exact fixture resident payload was not admitted");
    expect(OwnedUiResidentPayload.matchesScope(payload, scope)).toBe(true); expect(OwnedUiResidentPayload.matchesField(payload, field)).toBe(true); expect(OwnedKernelReturnInputField.matchesResidentPayload(field, payload)).toBe(true);
    expect(field.residentPayload(scope)).toBe(payload); const repeated = scope.beginPayload(field, { maxItems: 1, maxBytes: 64 }); expect(repeated.payload).toBe(payload); expect(repeated.step).toMatchObject({ kind: "ready", items: 0, bytes: 0 });
    const expected = produce({ ...before }, usage => { usage.bytes += fixture.expectedPayloadTotal.bytes; usage.slots += fixture.expectedPayloadTotal.slots; usage.owners += fixture.expectedPayloadTotal.owners; }); expect(ledger.usage.data).toEqual(expected);
    return payload;
  }
  async function fixtureResidentBuilder(ledger: OwnedResidentLedger, field: NonNullable<OwnedKernelReturnContent["field"]>, resident: OwnedUiResidentPayload) {
    const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts");
    const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
    const fixture = await loadRetainedResidentBuilderFixture(); const schema = await loadRetainedResidentBuilderSchema();
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); 
    const before = ledger.usage.data;
    for (let index = 0; index < fixture.grants.length; index++) {
      const bytes = fixture.grants[index]!; const last = index === fixture.grants.length - 1;
      const short = OwnedUiOperationPayloadBuilder.begin(field.owner, field.activation, field.lifetime, field, resident, { maxItems: 0, maxBytes: bytes }); expect(short.builder).toBeNull(); expect(short.step).toMatchObject({ kind: "blocked", items: 0, bytes: 0 });
      const result = OwnedUiOperationPayloadBuilder.begin(field.owner, field.activation, field.lifetime, field, resident, { maxItems: 1, maxBytes: bytes });
      if (result.step.kind === "rejected") { expect(result.builder).toBeNull(); return result; }
      expect(result.step, fixture.phases[index]).toMatchObject({ kind: last ? "ready" : "pending", items: 1, bytes });
      if (!last) { expect(result.builder).toBeNull(); continue; }
      const builder = result.builder; if (!builder) throw new Error("Exact fixture resident builder was not admitted");
      expect(OwnedUiOperationPayloadBuilder.matchesField(builder, field)).toBe(true); expect(OwnedKernelReturnInputField.matchesBuilder(field, builder)).toBe(true);
      const repeated = resident.beginBuilder(field, { maxItems: 1, maxBytes: 64 }); expect(repeated.builder).toBe(builder); expect(repeated.step).toMatchObject({ kind: "ready", items: 0, bytes: 0 });
      const expected = produce({ ...before }, usage => { usage.bytes += fixture.total.bytes; usage.slots += fixture.total.slots; usage.owners += fixture.total.owners; }); expect(ledger.usage.data).toEqual(expected);
      return result;
    }
    throw new Error("Exact fixture builder phase sequence did not publish");
  }
  const BUDGET: ShardBudget = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16, maxFrames: 8 };
  const fixtureHosts = new WeakMap<ShardInstanceLifecycleLease, OwnedUiInstance>();
  function bindFixtureHost(lease: ShardInstanceLifecycleLease): OwnedUiInstance {
    const host = new OwnedUiInstance(lease.activation, lease.lifetime!, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 });
    lease.bindHostRetirement(host);
    fixtureHosts.set(lease, host);
    return host;
  }
  function fixtureRetirement(lease: ShardInstanceLifecycleLease): OwnedUiInstanceRetirement {
    const host = fixtureHosts.get(lease)!;
    host.beginClose();
    for (let step = 0; step < 32 && !host.terminalIsEmpty(); step += 1) host.closeStep({ maxItems: 1, maxBytes: 4096 });
    const witness = host.takeRetirementWitness();
    if (!witness) throw new Error("fixture host retirement did not complete");
    return witness;
  }
  async function answerLifecycle(worker: FakeShardWorker, pending: Promise<unknown>, receipt?: ActorInstanceLifecycleReceipt): Promise<unknown> {
    const message = worker.sent.at(-1) as { requestId: string };
    worker.deliver({ kind: "result", requestId: message.requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: receipt ? encodeActorInstanceLifecycle(receipt) : undefined } });
    return pending;
  }
  async function captureFixtureInstance(client: ShardClient, worker: FakeShardWorker, actorId: string, instanceId = 7, guestLifetime = 13n): Promise<ShardInstanceLifecycleLease> {
    const lease = client.captureInstanceLifecycle(actorId, instanceId);
    const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: lease.activation.activationGeneration, instanceId, guestLifetime }, requestSequence: lease.openRequest.requestSequence };
    await answerLifecycle(worker, lease.open({ appId: "fixture", actor: "fixture", config: [], assets: [], capabilities: [], quotas: [] }, BUDGET), captured);
    await answerLifecycle(worker, lease.acknowledge(captured, BUDGET));
    bindFixtureHost(lease);
    return lease;
  }
  async function retireFixtureInstance(worker: FakeShardWorker, lease: ShardInstanceLifecycleLease): Promise<void> {
    const request = lease.beginClose();
    const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: request.requestSequence, closeGeneration: 31n };
    if (!lease.pendingReceipt) await answerLifecycle(worker, lease.close(BUDGET), accepted);
    const retired: ActorInstanceLifecycleReceipt = { ...accepted, kind: "retired" };
    await answerLifecycle(worker, lease.acknowledge(lease.pendingReceipt!, BUDGET), retired);
    await answerLifecycle(worker, lease.acknowledge(retired, BUDGET, fixtureRetirement(lease)));
  }
  async function activateActor(client: ShardClient, workers: readonly FakeShardWorker[], actorId: string, shardIndex = 0): Promise<void> {
    const promise = client.activate(actorId, `https://x/${actorId}.js`, [], BUDGET);
    await flushAdmission();
    const message = workers[shardIndex]!.sent.at(-1) as { readonly requestId: string };
    workers[shardIndex]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
    await promise;
  }
  return { ACTOR_BYTE_PAGE_BYTES, BUDGET, MAINTENANCE_LANE_DEFAULT_BUDGET, NO_RESIDENT_FAULT, OwnedActorTurnOutputs, OwnedKernelReturnContent, OwnedNativeUiPatchAuthority, OwnedNativeUiPatchSubmissionReceipt, OwnedResidentLedger, OwnedResidentRetirement, OwnedShardReturn, OwnedShardReturnPage, OwnedUiInstance, OwnedUiInstanceRetirement, OwnedUiResidentPool, SEGMENTED_DOWNLOAD_CONTRACT, SEGMENTED_DOWNLOAD_REFUSAL, SHARD_FRAME_VARIANT_FIELDS, SHARD_JSPI_FAULT_CODE, SHARD_LIVENESS_POLICY, ShardClient, ShardJspiUnavailableError, activateActor, admitSegmentedDownloadChunk, answerLifecycle, assertShardJspiAvailable, captureFixtureInstance, capturedReturnState, createActorBytePage, createGrantedBudgetTracker, createShardCommandIngressPages, describe, describeShardMessageError, describeShardSilence, describeShardWorkerError, encodeActorInstanceLifecycle, encodeActorUiPatchReceipt, evaluateShardLiveness, expect, fixtureHosts, fixtureResidentBuilder, fixtureResidentPayload, fixtureResidentPool, fixtureResidentScope, fixtureRetirement, harness, interpretShardFrame, isShardLostError, it, orderEnvelopesByLane, poolControllerEnvelope, poolUiEnvelope, prepareResidentFixture, retireFixtureInstance, settleFailedInstanceOpen, shardJspiAvailable, testSource, uiResidentMetadataEnvelope, vi };
}
export type ShardClientSettlementTestKit = ReturnType<typeof createShardClientSettlementTestKit>;
