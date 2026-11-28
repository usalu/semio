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

  /** 🧩️ `component` names the deployable component of the module the actor runs (the package descriptor's `components`); `null` runs the module's only, or first, component. */
  async activate(actorId: string, moduleUrl: string, caps: readonly ShardCapabilityGrant[], budget: ShardBudget, assets: readonly ShardAsset[] = [], component: string | null = null): Promise<void> {
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
    await this.hub.transport.send<void>(slot, { kind: "activate", requestId, actorId, activationGeneration: generation, originalAdmission, moduleUrl, component, caps, budget, assets, coreModules: this.state.coreModules.seeds() }, requestId);
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
        // 🚦️ A reactor turn admits exactly ONE event, so a host batch (typed-operation acknowledgements, coalesced UI events) crosses as one crossing per event, in order, and the answers are merged into the one result this API returns.
        // ✂️ The cut is dynamic: a patch page and its receipt cannot be merged with another answer's, so the first answer that carries ui patches or a receipt ends this call, and the events not yet crossed go back to the FRONT of this actor's queue (`deferredEvents`) for the next crossing, in order. The call then answers `more-work` so every drain loop comes back for them. A command page rides only the last crossing. When a cut leaves events pending, the caller still owns the page and offers it again.
        const deferred = deferredEvents.get(activation) ?? [];
        deferredEvents.delete(activation);
        const batch: readonly ShardEventEnvelope[] = deferred.length > 0 ? [...deferred, ...events] : events;
        const slices: readonly (readonly ShardEventEnvelope[])[] = batch.length > 1 ? batch.map((event) => [event]) : [batch];
        const answers: unknown[] = [];
        let postponed = false;
        for (const [index, slice] of slices.entries()) {
          const requestId = this.hub.transport.nextRequestId();
          const answer = await this.hub.transport.send(slot, { kind: "turn", requestId, actorId, activationGeneration: activation.generation, events: slice, commandPage: index === slices.length - 1 ? commandPage : undefined, budget, batched: index < slices.length - 1 }, requestId);
          answers.push(answer);
          if (index < slices.length - 1) {
            assertActive();
            if (carriesUiPatches(answer)) {
              deferredEvents.set(activation, batch.slice(index + 1));
              postponed = true;
              break;
            }
          }
        }
        const merged = answers.length === 1 ? answers[0] : mergeSequentialTurnAnswers(answers);
        const left = postponed ? (deferredEvents.get(activation)?.length ?? 0) : 0;
        const result = postponed ? withMoreWork(merged, left) : deferred.length > 0 ? { ...(merged as Record<string, unknown>), deferredEvents: 0 } : merged;
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
    this.state.coreModules.release(String(activation.generation));
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

/** 🧾️ Events a batch had not crossed yet when an earlier answer carried ui patches; the next `turn` of the same activation crosses them first. */
const deferredEvents = new WeakMap<object, readonly ShardEventEnvelope[]>();

/** 🩹️ Whether one crossing's answer carries a patch page or a patch receipt (which cannot share a result with another answer's). */
function carriesUiPatches(answer: unknown): boolean {
  const turn = answer as Record<string, any>;
  return (Array.isArray(turn.uiPatches) && turn.uiPatches.length > 0) || (turn.uiPatchReceipt !== undefined && turn.uiPatchReceipt !== null);
}

/** 🚥️ The answer of a cut batch: the events left over (`deferredEvents`, kept by `coerceTurnResult`) are work the guest has not seen yet, so the status says `more-work` in the shape the answers already use. A later answer that crossed them says `deferredEvents: 0`. */
function withMoreWork(answer: unknown, left: number): unknown {
  const turn = answer as Record<string, any>;
  return { ...turn, deferredEvents: left, status: typeof turn.status === "string" ? "more-work" : { ...(turn.status ?? {}), tag: "more-work" } };
}

/** 🧩️ Folds the answers of one-event crossings into the single result of the batch: effects and presence concatenate, the last status and wake win, and at most ONE answer may carry ui patches/receipts (a patch page and its receipt are one unit that cannot be merged). */
function mergeSequentialTurnAnswers(answers: readonly unknown[]): unknown {
  const turns = answers as readonly Record<string, any>[];
  const last = turns[turns.length - 1]!;
  const carriers = turns.filter((turn) => (Array.isArray(turn.uiPatches) && turn.uiPatches.length > 0) || (turn.uiPatchReceipt !== undefined && turn.uiPatchReceipt !== null));
  if (carriers.length > 1) throw new Error("actor-poll.event-batch-carried: more than one event of a batch answered with ui patches; send those events as their own crossings");
  const carrier = carriers[0];
  const lifecycle = turns.find((turn) => turn.lifecycleReceipt !== undefined && turn.lifecycleReceipt !== null);
  const ingress = [...turns].reverse().find((turn) => turn.commandIngress && turn.commandIngress.tag !== "idle");
  return {
    ...last,
    uiPatches: carrier ? carrier.uiPatches : last.uiPatches,
    uiPatchReceipt: carrier ? carrier.uiPatchReceipt : last.uiPatchReceipt,
    effects: turns.flatMap((turn) => (Array.isArray(turn.effects) ? turn.effects : [])),
    presence: turns.flatMap((turn) => (Array.isArray(turn.presence) ? turn.presence : [])),
    nextWake: [...turns].reverse().find((turn) => turn.nextWake !== null && turn.nextWake !== undefined)?.nextWake ?? last.nextWake,
    lifecycleReceipt: lifecycle ? lifecycle.lifecycleReceipt : last.lifecycleReceipt,
    commandIngress: ingress ? ingress.commandIngress : last.commandIngress,
  };
}
