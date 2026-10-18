import { ShardClientHub } from "./🧩️wiring/🟦️.ts";
import { ShardClientState } from "./🗃️state/🟦️.ts";
import { SHARD_LIVENESS_POLICY } from "./🫀️liveness/🟦️.ts";
import { OwnedResidentLedger, OwnedResidentRetirement, type ResidentGrant, type ResidentStep } from "@semio-tech/value-ts/resident";
import { OwnedKernelReturnContent } from "@semio-tech/framework-kernel/return/content/input";
import { OwnedUiInstance, OwnedUiInstanceRetirement, OwnedUiPatchAcknowledgement, OwnedUiPatchInputRetirement } from "../../🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import { OwnedUiResidentPool } from "../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts";
import { uiResidentMetadataEnvelope } from "../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️.ts";
import { ACTOR_BYTE_PAGE_BYTES, createActorBytePage } from "../📃️page/🟦️.ts";
import { encodeActorInstanceLifecycle } from "../🚪️lifetime/🟦️.ts";
import { encodeActorUiPatchReceipt } from "../🚪️lifetime/🩹️patch/🟦️.ts";
import { OwnedActorTurnOutput, OwnedActorTurnOutputs } from "../🪪️activation/🚪️instance/📥️output/🟦️.ts";
import { admitActorCodecRequestV1, admitActorCodecAnswerV1, type ActorCodecAnswer, type ActorCodecRequest } from "../🧬️codec/🟦️.ts";
import { admitSegmentedDownloadChunk, admitSegmentedDownloadOperationId, SEGMENTED_DOWNLOAD_CONTRACT, SEGMENTED_DOWNLOAD_REFUSAL } from "./📤️segmented-download/🟦️.ts";
import { NO_RESIDENT_FAULT, OwnedShardReturn, OwnedShardReturnPage, RETRYABLE_LIFECYCLE_TURN, RETRYABLE_LIFECYCLE_TURN_ATTEMPTS, type ShardActorActivationLease, type ShardInstanceLifecycleLease, type ShardSlot, capturedActivationMatches, capturedReturnState, graftWorkerStack, isRetryableLifecycleTurn, bindCapturedReturnDispatch, poolControllerEnvelope, poolUiEnvelope } from "./🔐️activation/🟦️.ts";
import { OwnedNativeUiPatchAuthority, OwnedNativeUiPatchSubmissionReceipt } from "./🩹️ui-patch/🟦️.ts";
import type { ShardClientOptions } from "./⚙️options/🟦️.ts";
import { SHARD_JSPI_FAULT_CODE, ShardJspiUnavailableError, assertShardJspiAvailable, describeShardMessageError, describeShardSilence, describeShardWorkerError, evaluateShardLiveness, isShardLostError, settleFailedInstanceOpen, shardJspiAvailable } from "./💓️heartbeat/🟦️.ts";
import type { OutboundMessage } from "./📨️messages/🟦️.ts";
import { MAINTENANCE_LANE_DEFAULT_BUDGET, SHARD_COMMAND_MAXIMUM_PAGES, SHARD_FRAME_VARIANT_FIELDS, type ShardAsset, type ShardBudget, type ShardCapabilityGrant, type ShardCommandIngressPage, type ShardEnvelope, type ShardEventEnvelope, type ShardJobBudget, type ShardJobStep, type ShardMetricsSample, createGrantedBudgetTracker, createShardCommandIngressPages, interpretShardFrame, orderEnvelopesByLane } from "./📜️wire/🟦️.ts";

export { ACTOR_RETURN_RESULT_MAXIMUM_BYTES, decodeActorReturnDrive, decodeActorReturnResult, encodeActorReturnDrive, encodeActorReturnResult, type ActorReturnControl, type ActorReturnDrive, type ActorReturnFault, type ActorReturnIdentity, type ActorReturnOrigin, type ActorReturnPageReceipt, type ActorReturnResult } from "../📤️return/🟦️.ts";
export { admitSegmentedDownloadChunk, admitSegmentedDownloadOperationId, SEGMENTED_DOWNLOAD_CONTRACT, SEGMENTED_DOWNLOAD_REFUSAL } from "./📤️segmented-download/🟦️.ts";
export { SHARD_LIVENESS_POLICY } from "./🫀️liveness/🟦️.ts";

/** 🧵️ `ShardClient` — the web `ShardTransport` (design-runtime.md §1 `ShardTransport` /
 * §3 "Web shard"): a bounded pool of `🟨️shard-worker.js` Web Workers multiplexed by `actorId`,
 * replacing one-Worker-per-plugin (`PluginWorkerClient`, deleted from `🎠️kernel` in
 * the same packet). V8 reserves a 4 GiB guard region per wasm module per worker, so
 * one-worker-per-plugin capped the browser at ~20 plugins; this pools K = `min(hardwareConcurrency-1,
 * 4)` (design's `ShardTable`) workers and pins actors onto them instead.
 *
 * 🚧 UPDATE (terra-web-shardframe): A1's `🤖️generated/🎭️actor/🟦️.ts` has now landed clean (no more
 * un-typeable `object & string` intersections — `Lane`/`Envelope`/`Payload`/`Origin` are real types),
 * which is what makes the region below possible. `turn()`/`activate()` stay EXACTLY as they were —
 * opaque-to-this-module `ShardEventEnvelope[]` JSON in, `unknown` out, nothing broken — while
 * `📨️ShardFrame` below adds the Rust `ShardFrame` enum's SHAPE (`Register`/`Unregister`/`Grant`/
 * `Envelope`) as a NEW, additive wire alongside them: {@link ShardClient.grant}/{@link
 * ShardClient.envelope}. Deliberately shape-only, not byte-for-byte — see that region's own header
 * doc for the encoding decision (structured clone, no hand-rolled pack codec, on purpose, for now).
 */

//#region 🧵️ShardClient
/** 🧵️ One `ShardClient` instance owns the WHOLE bounded pool (design: "ShardClient... replaces both
 * `PluginWorkerClient`s [both former per-plugin copies] and `pluginHandleForBridge`") — internally one
 * `ShardWorkerLike` ("MessagePort") per shard, `actorId`-tagged request/reply multiplexing so several
 * actors share a shard's single message channel without cross-talk. */
export class ShardClient {
  readonly #residentLedger: OwnedResidentLedger;
  private readonly hub: ShardClientHub;
  constructor(options: ShardClientOptions) {
    const state = new ShardClientState(options);
    this.#residentLedger = state.residentLedger;
    this.hub = new ShardClientHub(this, state);
    for (let index = 0; index < options.shardCount; index += 1) state.shards.push(this.hub.transport.spawnShard(index));
  }

  static { bindCapturedReturnDispatch((client, state, work, budget) => client.hub.returns.sendCapturedReturn(state, work, budget), (client, state, grant) => client.hub.returns.reserveReturnResponse(state, grant)); }
  static matchesResidentLedger(client: unknown, ledger: unknown): client is ShardClient { return client !== null && typeof client === "object" && #residentLedger in client && client.#residentLedger === ledger; }

  /** 🧭️ Checks the privately captured original client; operation liveness is independently asserted. */
  static matchesActivation(client: unknown, activation: unknown): activation is ShardActorActivationLease {
    return client !== null && typeof client === "object" && #residentLedger in client && capturedActivationMatches(activation, client);
  }

  prepareWorkerBootstrap(grant: ResidentGrant): ResidentStep { return this.hub.residents.prepareWorkerBootstrap(grant); }
  closeWorkerBootstrapStep(grant: ResidentGrant): ResidentStep { return this.hub.residents.closeWorkerBootstrapStep(grant); }
  prepareUiResidentPool(ledger: OwnedResidentLedger, grant: ResidentGrant): ResidentStep { return this.hub.residents.prepareUiResidentPool(ledger, grant); }
  captureUiResidentPoolFault(error: unknown): void { return this.hub.residents.captureUiResidentPoolFault(error); }
  ownsUiResidentPool(pool: unknown): boolean { return this.hub.residents.ownsUiResidentPool(pool); }
  closeUiResidentPoolStep(grant: ResidentGrant): ResidentStep { return this.hub.residents.closeUiResidentPoolStep(grant); }
  installUiResidentPool(pool: OwnedUiResidentPool, grant: ResidentGrant): ResidentStep { return this.hub.residents.installUiResidentPool(pool, grant); }
  releaseUiResidentPool(pool: OwnedUiResidentPool, witness: unknown, grant: ResidentGrant): ResidentStep { return this.hub.residents.releaseUiResidentPool(pool, witness, grant); }
  leaseExclusive(actorId: string, options?: { readonly force?: boolean }): number { return this.hub.routing.leaseExclusive(actorId, options); }
  releaseExclusive(actorId: string): void { return this.hub.routing.releaseExclusive(actorId); }
  shardIndexFor(actorId: string): number | undefined { return this.hub.routing.shardIndexFor(actorId); }
  activate(actorId: string, moduleUrl: string, caps: readonly ShardCapabilityGrant[], budget: ShardBudget, assets?: readonly ShardAsset[]): Promise<void> { return this.hub.activations.activate(actorId, moduleUrl, caps, budget, assets); }
  captureActorActivation(actorId: string): ShardActorActivationLease { return this.hub.activations.captureActorActivation(actorId); }
  dispose(actorId: string): void { return this.hub.activations.dispose(actorId); }
  captureInstanceLifecycle(actorId: string, instanceId: number): ShardInstanceLifecycleLease { return this.hub.lifecycles.captureInstanceLifecycle(actorId, instanceId); }
  pollHeartbeatSab(nowMs?: number): void { return this.hub.watchdog.pollHeartbeatSab(nowMs); }
  checkHeartbeats(nowMs?: number): void { return this.hub.watchdog.checkHeartbeats(nowMs); }
  startWatchdog(intervalMs?: number): void { return this.hub.watchdog.startWatchdog(intervalMs); }
  stopWatchdog(): void { return this.hub.watchdog.stopWatchdog(); }
  shardMetricsSamples(nowMs?: number): readonly ShardMetricsSample[] { return this.hub.watchdog.shardMetricsSamples(nowMs); }
  terminate(index: number, detail?: string): readonly string[] { return this.hub.routing.terminate(index, detail); }
  rebuild(index: number): void { return this.hub.routing.rebuild(index); }
  disposeAll(): void { return this.hub.routing.disposeAll(); }
  private requireShard(actorId: string): ShardSlot { return this.hub.routing.requireShard(actorId); }
  private nextRequestId(): string { return this.hub.transport.nextRequestId(); }
  private send<T>(slot: ShardSlot, message: OutboundMessage, requestId: string | null, posted?: () => void, output?: OwnedActorTurnOutput | null): Promise<T> { return this.hub.transport.send(slot, message, requestId, posted, output); }

  /** ▶️ One turn (`reactor::poll`), never more than one in flight per `actorId` at a time — a second
   * `turn()` call for the same actor before the first resolves is a caller bug (the scheduler's own
   * per-actor serialization, not this transport's job, per design's "runs one turn at a time per
   * actor"), so it is rejected rather than silently queued. */
  async turn(actorId: string, events: readonly ShardEventEnvelope[], budget: ShardBudget, commandPage?: ShardCommandIngressPage): Promise<unknown> {
    if (!this.hub.state.actorShard.has(actorId)) throw new Error(`ShardClient.turn(${actorId}): not activated on any shard`);
    return this.captureActorActivation(actorId).turn(events, budget, commandPage);
  }

  /** 📨️ terra-web-shardframe: `ShardFrame::Envelope` passthrough — wraps ONE envelope's worth of work
   * in the Rust-mirrored shape, deliberately budget-LESS on the wire (the worker resolves
   * `envelope.to`'s LAST {@link ShardFrame.Grant} via a {@link GrantedBudgetTracker}, falling back to
   * {@link MAINTENANCE_LANE_DEFAULT_BUDGET} for an actor never granted one) — exactly the incremental
   * adoption step the Rust `ShardFrame::Envelope` doc calls for ("kept so the web `ShardClient`... can
   * adopt this wire incrementally... without both ends changing atomically"). {@link turn} is left
   * completely untouched by this method; both wire shapes coexist on purpose. */
  async envelope(shardEnvelope: ShardEnvelope): Promise<unknown> {
    const slot = this.requireShard(shardEnvelope.to);
    const activation = this.captureActorActivation(shardEnvelope.to);
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "frame", requestId, actorId: shardEnvelope.to, activationGeneration: activation.activationGeneration, frame: { kind: "Envelope", envelope: shardEnvelope } }, requestId);
  }

  /** ⚖️ terra-web-shardframe: `ShardFrame::Grant` — `budget` travels WITH `envelopes` in ONE wire
   * message (design-runtime.md's DRR promise), sent to the worker in LANE-PRIORITY order via
   * {@link orderEnvelopesByLane} rather than push/arrival order. The worker remembers `budget` as
   * `actorId`'s new granted budget for any later {@link envelope} passthrough. */
  async grant(actorId: string, budget: ShardBudget, envelopes: readonly ShardEnvelope[]): Promise<unknown> {
    const slot = this.requireShard(actorId);
    const activation = this.captureActorActivation(actorId);
    const requestId = this.nextRequestId();
    const ordered = orderEnvelopesByLane(envelopes);
    return this.send(slot, { kind: "frame", requestId, actorId, activationGeneration: activation.activationGeneration, frame: { kind: "Grant", actor: actorId, budget, envelopes: ordered } }, requestId);
  }

  async startJob(actorId: string, job: bigint, jobKind: string, input: Uint8Array): Promise<void> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send<void>(slot, { kind: "startJob", requestId, actorId, job, jobKind, input }, requestId);
  }

  async stepJob(actorId: string, job: bigint, budget: ShardJobBudget): Promise<ShardJobStep> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send<ShardJobStep>(slot, { kind: "stepJob", requestId, actorId, job, budget }, requestId);
  }

  async cancelJob(actorId: string, job: bigint): Promise<void> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send<void>(slot, { kind: "cancelJob", requestId, actorId, job }, requestId);
  }

  /** 🧵 Requests exactly one operation-owned item and admits it against {@link SEGMENTED_DOWNLOAD_CONTRACT}. */
  async takeSegmentedDownloadChunk(actorId: string, instanceId: number, operationId: bigint): Promise<Uint8Array | undefined> {
    admitSegmentedDownloadOperationId(operationId, instanceId);
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return admitSegmentedDownloadChunk(await this.send<unknown>(slot, { kind: "takeSegmentedDownloadChunk", requestId, actorId, instanceId, operationId }, requestId));
  }

  async checkpoint(actorId: string): Promise<Uint8Array> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send<Uint8Array>(slot, { kind: "checkpoint", requestId, actorId }, requestId);
  }

  async restore(actorId: string, state: Uint8Array): Promise<void> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send<void>(slot, { kind: "restore", requestId, actorId, state }, requestId);
  }

  /** 🧬️ Calls the actor's component `codec` interface once. The caller serializes it with the actor's
   * turns (the worker refuses a codec call while a turn of the actor is in flight). */
  async codec<Request extends ActorCodecRequest>(actorId: string, request: Request): Promise<ActorCodecAnswer<Request["operation"]>> {
    admitActorCodecRequestV1(request);
    const operation=request.operation;
    const slot = this.requireShard(actorId);
    const activation = this.captureActorActivation(actorId);
    const requestId = this.nextRequestId();
    return this.send<unknown>(slot, { kind: "codec", requestId, actorId, activationGeneration: activation.activationGeneration, request }, requestId).then(value => admitActorCodecAnswerV1<Request["operation"]>(operation, value));
  }
}
//#endregion 🧵️ShardClient

export type ShardClientTestDependenciesV1 = ReturnType<typeof shardClientTestDependenciesV1>;
export const shardClientTestDependenciesV1 = () => ({ ACTOR_BYTE_PAGE_BYTES, SHARD_COMMAND_MAXIMUM_PAGES, MAINTENANCE_LANE_DEFAULT_BUDGET, NO_RESIDENT_FAULT, SEGMENTED_DOWNLOAD_CONTRACT, SEGMENTED_DOWNLOAD_REFUSAL, admitSegmentedDownloadChunk, OwnedActorTurnOutput, OwnedActorTurnOutputs, OwnedKernelReturnContent, OwnedNativeUiPatchAuthority, OwnedNativeUiPatchSubmissionReceipt, OwnedResidentLedger, OwnedResidentRetirement, OwnedShardReturn, OwnedShardReturnPage, OwnedUiInstance, OwnedUiInstanceRetirement, OwnedUiPatchAcknowledgement, OwnedUiPatchInputRetirement, OwnedUiResidentPool, SHARD_FRAME_VARIANT_FIELDS, SHARD_JSPI_FAULT_CODE, SHARD_LIVENESS_POLICY, ShardClient, ShardJspiUnavailableError, assertShardJspiAvailable, capturedReturnState, createActorBytePage, createGrantedBudgetTracker, createShardCommandIngressPages, describeShardMessageError, describeShardSilence, describeShardWorkerError, encodeActorInstanceLifecycle, encodeActorUiPatchReceipt, evaluateShardLiveness, interpretShardFrame, isShardLostError, orderEnvelopesByLane, poolControllerEnvelope, poolUiEnvelope, settleFailedInstanceOpen, shardJspiAvailable, uiResidentMetadataEnvelope });
export type ShardClientRetryableLifecycleTestDependenciesV1 = ReturnType<typeof shardClientRetryableLifecycleTestDependenciesV1>;
export const shardClientRetryableLifecycleTestDependenciesV1 = () => ({ RETRYABLE_LIFECYCLE_TURN, RETRYABLE_LIFECYCLE_TURN_ATTEMPTS, graftWorkerStack, isRetryableLifecycleTurn });
if (import.meta.vitest) {
  const testSource = { directory: (await import("node:url")).fileURLToPath(new URL(".", import.meta.url)), url: import.meta.url };
  const { registerCommandIngressPageTests } = await import("./🧪️tests/📥️command-ingress-pages/🟦️.ts");
  await registerCommandIngressPageTests(import.meta.vitest, shardClientTestDependenciesV1(), testSource);
}
