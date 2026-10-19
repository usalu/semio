import type { ShardClientState } from "../🗃️state/🟦️.ts";
import type { ShardClientHubPort } from "../🧩️hub/🟦️.ts";
import { type ShardActivation, type ShardActorActivationLease, type ShardSlot, mintCapturedActivation } from "../🔐️activation/🟦️.ts";
import type { OutboundMessage } from "../📨️messages/🟦️.ts";
import type { ShardAsset, ShardBudget, ShardCapabilityGrant, ShardCommandIngressPage, ShardEventEnvelope } from "../📜️wire/🟦️.ts";

/** 🪪️ Owns the exact activation of every actor: activation, revocable operation leases and teardown. */
export class ShardExactActivations {
  private readonly state: ShardClientState;
  private readonly hub: ShardClientHubPort;
  constructor(hub: ShardClientHubPort) { this.hub = hub; this.state = hub.state; }

  async activate(actorId: string, moduleUrl: string, caps: readonly ShardCapabilityGrant[], budget: ShardBudget, assets: readonly ShardAsset[] = []): Promise<void> {
    if (this.state.actorActivations.get(actorId)?.available) throw new Error("actor-close.activation-already-owned");
    if (this.state.activationGeneration >= 0xffffffffffffffffn) throw new Error("actor-close.activation-generation-exhausted");
    const requestId = this.hub.transport.nextRequestId();
    const generation = this.state.activationGeneration + 1n;
    const slot = this.hub.routing.assignShard(actorId);
    const originalAdmission = await this.state.originalWorkStock.admit();
    const activation: ShardActivation = { originalAdmission, slot, actorId, generation, available: true, activated: false, teardownPosted: false, operationsAllowed: true, operationGeneration: 0n, lastGuestLifetime: 0n, lastReturnSequence: 0n, returned: null, instance: null, close: null };
    this.state.activationGeneration = generation;
    this.state.actorActivations.set(actorId, activation);
    this.state.actorsPastFirstTurn.delete(actorId);
    await this.hub.transport.send<void>(slot, { kind: "activate", requestId, actorId, activationGeneration: generation, originalAdmission, moduleUrl, caps, budget, assets }, requestId);
    activation.activated = true;
  }

  activationIsActive(activation: ShardActivation): boolean {
    return activation.available && activation.slot.available && activation.activated && activation.operationsAllowed && activation.close === null && this.state.actorActivations.get(activation.actorId) === activation && this.state.actorShard.get(activation.actorId) === activation.slot.index && this.state.shards[activation.slot.index] === activation.slot;
  }

  inboundActivation(slot: ShardSlot, actorId: string, generation: bigint | null): ShardActivation | undefined {
    const activation = this.state.actorActivations.get(actorId);
    return activation && activation.slot === slot && activation.generation === generation && this.activationIsActive(activation) ? activation : undefined;
  }

  /** 🎯️ Captures once before asynchronous work; every turn checks before dispatch and after settlement. */
  captureActorActivation(actorId: string): ShardActorActivationLease {
    const activation = this.state.actorActivations.get(actorId);
    if (!activation?.activated) throw new Error("actor-activation.not-ready");
    const slot = activation.slot;
    const worker = slot.worker;
    const operationGeneration = activation.operationGeneration;
    const assertActive = (): void => {
      if (!activation.available || !slot.available || !activation.operationsAllowed || activation.operationGeneration !== operationGeneration || activation.close !== null || this.state.actorActivations.get(actorId) !== activation || this.state.actorShard.get(actorId) !== slot.index || this.state.shards[slot.index] !== slot || slot.worker !== worker) throw new Error("actor-activation.revoked");
    };
    assertActive();
    return mintCapturedActivation(this.hub.shell, actorId, activation.generation, assertActive,
      async (events: readonly ShardEventEnvelope[], budget: ShardBudget, commandPage?: ShardCommandIngressPage): Promise<unknown> => {
        assertActive();
        if (activation.returned !== null) throw new Error("actor-return.already-owned");
        const owner = activation.instance;
        const requestId = this.hub.transport.nextRequestId();
        const result = await this.hub.transport.send(slot, { kind: "turn", requestId, actorId, activationGeneration: activation.generation, events, commandPage, budget }, requestId);
        if (owner) this.hub.uiAcknowledgements.recordInstanceTurn(owner, result);
        try { assertActive(); } catch (error) { if (owner) owner.interruptedTurn = result; throw error; }
        return result;
      });
  }

  /** ⏏️ Frees the worker-side actor entry — does not touch this shard's routing entry so a caller that
   * immediately re-`activate()`s the same `actorId` (hot reload) can still target the same shard;
   * pair with a routing-level `actorShard.delete` only when the actor is gone for good (unusual — most
   * callers instead let `activate` overwrite in place). */
  dispose(actorId: string): void {
    const activation = this.state.actorActivations.get(actorId);
    if (activation) { this.disposeActivation(activation); return; }
    const shardIndex = this.state.actorShard.get(actorId);
    if (shardIndex === undefined) return;
    this.state.shards[shardIndex]!.actorIds.delete(actorId);
    this.state.actorShard.delete(actorId);
  }

  disposeActivation(activation: ShardActivation): void {
    if (activation.teardownPosted) return;
    if (activation.instance !== null || activation.close !== null) throw new Error("actor-close.native-retirement-pending");
    if (activation.returned !== null) throw new Error("actor-return.retirement-pending");
    const { actorId, slot } = activation;
    if (!activation.available || !slot.available || this.state.shards[slot.index] !== slot) throw new Error("actor-close.worker-lost");
    activation.operationsAllowed = false;
    this.state.originalClosures.set(activation.originalAdmission.operation, activation);
    slot.worker.postMessage({ kind: "dispose", actorId, activationGeneration: activation.generation } satisfies OutboundMessage);
    activation.teardownPosted = true;
    activation.available = false;
    if (this.state.outstandingEffectsByActor.get(actorId)?.activation === activation) this.hub.effects.abortOutstandingEffects(actorId);
    this.hub.transport.rejectActorPending(slot, actorId, new Error(`ShardClient actor disposed: ${actorId}`));
    slot.actorIds.delete(actorId);
    if (this.state.actorActivations.get(actorId) === activation) {
      const route = this.state.actorShard.get(actorId);
      if (route !== undefined) this.state.shards[route]!.actorIds.delete(actorId);
      this.state.actorShard.delete(actorId);
      this.state.actorActivations.delete(actorId);
    }
  }
}
