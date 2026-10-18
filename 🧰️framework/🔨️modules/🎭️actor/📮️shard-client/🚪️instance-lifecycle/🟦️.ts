import type { ShardClientState } from "../🗃️state/🟦️.ts";
import type { ShardClientHub } from "../🧩️wiring/🟦️.ts";
import type { ResidentGrant } from "@semio-tech/value-ts/resident";
import { OwnedUiInstance, OwnedUiInstanceRetirement, OwnedUiPatchAcknowledgement } from "../../../🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import { actorInstanceCapturedReceiptMatches, actorInstanceCloseReceiptMatches, actorInstanceLifecycleReceiptEquals, actorInstanceLifetimeEquals, decodeActorInstanceLifecycle, type ActorInstanceCloseRequest, type ActorInstanceLifecycleReceipt, type ActorInstanceOpenRequest } from "../../🚪️lifetime/🟦️.ts";
import { NO_RETURN_FAULT, RETRYABLE_LIFECYCLE_TURN_ATTEMPTS, type ShardInstanceLifecycleLease, type ShardInstanceOpenInput, type ShardInstanceOwner, isRetryableLifecycleTurn } from "../🔐️activation/🟦️.ts";
import { OwnedNativeUiPatchAuthority, type OwnedUiPatchAcknowledgementEntry } from "../🩹️ui-patch/🟦️.ts";
import type { ShardBudget, ShardEventEnvelope } from "../📜️wire/🟦️.ts";

/** 🚪️ Owns the exact instance lifecycle of an activation: open, poll, close and acknowledgement crossings with their receipts. */
export class ShardInstanceLifecycles {
  private readonly state: ShardClientState;
  constructor(private readonly hub: ShardClientHub) { this.state = hub.state; }

  /** 🪪️ Reserves an exact pre-open owner; only a guest receipt can populate its lifetime. */
  captureInstanceLifecycle(actorId: string, instanceId: number): ShardInstanceLifecycleLease {
    if (!Number.isInteger(instanceId) || instanceId < 0 || instanceId > 0xffffffff) throw new Error("actor-lifecycle.invalid-instance");
    const activation = this.state.actorActivations.get(actorId);
    if (!activation || !this.hub.activations.activationIsActive(activation)) throw new Error("actor-lifecycle.activation-not-ready");
    if (activation.instance !== null) throw new Error("actor-lifecycle.instance-already-owned");
    const operation = this.hub.activations.captureActorActivation(actorId);
    this.hub.transport.nextRequestId();
    const open: ActorInstanceOpenRequest = Object.freeze({ kind: "open", activationGeneration: activation.generation, instanceId, requestSequence: this.state.requestSeq });
    const owner: ShardInstanceOwner = { activation, operation, open, phase: "opening", lifetime: null, receipt: null, accepted: null, close: null, host: null, inFlight: false, failure: null, interruptedTurn: null, cancellation: null, lastPatchSequence: 0n, lastPatchTurn: null, returnCell: null, returnRecord: null, returnPhase: "empty", returnFault: NO_RETURN_FAULT, returnCapacity: 0 };
    activation.instance = owner;
    this.state.instanceLifecycles.set(open.requestSequence, owner);
    return Object.freeze({
      activation: operation,
      openRequest: open,
      get lifetime() { return owner.lifetime; },
      get pendingReceipt() { return owner.receipt; },
      get interruptedTurn() { return owner.interruptedTurn; },
      get pendingReturn() { return owner.activation.returned?.instance === owner ? owner.activation.returned.facade : null; },
      reserveReturn: (maximumResponses: number, grant: ResidentGrant) => this.hub.returns.reserveInstanceReturn(owner, maximumResponses, grant),
      retireUnusedReturn: (grant: ResidentGrant) => this.hub.returns.retireUnusedInstanceReturn(owner, grant),
      open: async (input: ShardInstanceOpenInput, budget: ShardBudget): Promise<unknown> => {
        if (owner.phase !== "opening") throw new Error("actor-lifecycle.open-already-captured");
        operation.assertActive();
        return this.sendInstanceLifecycle(owner, [{ kind: "instance-open", payload: { instance: instanceId, activationGeneration: open.activationGeneration, requestSequence: open.requestSequence, appId: input.appId, actor: input.actor, config: input.config, assets: input.assets, capabilities: input.capabilities, quotas: input.quotas } }], budget);
      },
      poll: (budget: ShardBudget) => this.sendInstanceLifecycle(owner, [], budget),
      beginClose: () => this.beginInstanceLifecycleClose(owner),
      close: async (budget: ShardBudget): Promise<unknown> => {
        const request = this.beginInstanceLifecycleClose(owner);
        if (owner.receipt !== null) throw new Error("actor-lifecycle.receipt-ack-required");
        return this.sendInstanceLifecycle(owner, [{ kind: "instance-close", payload: request }], budget);
      },
      acknowledge: async (receipt: ActorInstanceLifecycleReceipt, budget: ShardBudget, retirement?: OwnedUiInstanceRetirement): Promise<unknown> => {
        if (!owner.receipt || !actorInstanceLifecycleReceiptEquals(owner.receipt, receipt)) throw new Error("actor-lifecycle.ack-mismatch");
        if (receipt.kind === "retired" && (owner.cancellation !== null || !owner.host || !owner.lifetime || !OwnedUiInstanceRetirement.matches(retirement, owner.host, operation, owner.lifetime))) throw new Error("actor-lifecycle.host-retirement-pending");
        return this.sendInstanceLifecycle(owner, [{ kind: "instance-lifecycle-ack", payload: { kind: "ack", receipt: owner.receipt } }], budget, owner.receipt);
      },
      bindHostRetirement: (participant: OwnedUiInstance): void => {
        if (owner.host !== null || owner.lifetime === null || !OwnedUiInstance.matches(participant, operation, owner.lifetime)) throw new Error("actor-lifecycle.host-owner-mismatch");
        owner.host = participant;
      },
      captureUiPatchAuthority: (originalTurn: object, patchIndex: number) => this.hub.uiAcknowledgements.captureInstanceUiPatch(owner, originalTurn, patchIndex),
      submitUiAcknowledgement: (source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement, budget: ShardBudget) => this.hub.uiAcknowledgements.submitInstanceUiAcknowledgement(owner, source, token, budget),
      submitUiAcknowledgements: (entries: readonly OwnedUiPatchAcknowledgementEntry[], budget: ShardBudget) => this.hub.uiAcknowledgements.submitInstanceUiAcknowledgements(owner, entries, budget),
      dispose: () => {
        if (owner.phase !== "complete") throw new Error("actor-close.native-retirement-pending");
        this.hub.activations.disposeActivation(owner.activation);
      },
      progress: (): ReturnType<ShardInstanceLifecycleLease["progress"]> => ({ kind: owner.failure === null ? owner.phase : "blocked", failure: owner.failure }),
    });
  }

  private beginInstanceLifecycleClose(owner: ShardInstanceOwner): ActorInstanceCloseRequest {
    if (owner.close) return owner.close;
    if (!owner.activation.available || !owner.activation.slot.available || this.state.shards[owner.activation.slot.index] !== owner.activation.slot) { owner.failure = "worker-lost"; throw new Error("actor-lifecycle.worker-lost"); }
    if (owner.lifetime === null || owner.phase !== "open") throw new Error("actor-lifecycle.capture-pending");
    if (owner.activation.operationGeneration >= 0xffffffffffffffffn) throw new Error("actor-lifecycle.operation-generation-exhausted");
    this.hub.transport.nextRequestId();
    owner.close = Object.freeze({ kind: "close", lifetime: owner.lifetime, requestSequence: this.state.requestSeq });
    owner.phase = "closing";
    owner.activation.close = owner;
    owner.activation.operationGeneration += 1n;
    const ledger = this.state.outstandingEffectsByActor.get(owner.activation.actorId);
    if (ledger?.activation === owner.activation) {
      this.state.outstandingEffectsByActor.delete(owner.activation.actorId);
      owner.cancellation = ledger;
    }
    return owner.close;
  }

  async sendInstanceLifecycle(owner: ShardInstanceOwner, events: readonly ShardEventEnvelope[], budget: ShardBudget, acknowledged?: ActorInstanceLifecycleReceipt): Promise<unknown> {
    const { activation } = owner;
    const { slot } = activation;
    if (!activation.available || !slot.available || this.state.shards[slot.index] !== slot) { owner.failure = "worker-lost"; throw new Error("actor-lifecycle.worker-lost"); }
    if (activation.returned !== null) throw new Error("actor-return.retirement-pending");
    if (owner.phase === "complete") throw new Error("actor-lifecycle.already-complete");
    if (owner.inFlight) throw new Error("actor-lifecycle.turn-already-pending");
    const requestId = this.hub.transport.nextRequestId();
    owner.inFlight = true;
    let posted = false;
    try {
      if (owner.cancellation) {
        this.hub.effects.cancelOneEffect(owner.cancellation);
        if (owner.cancellation.head === null) owner.cancellation = null;
      }
      let result: unknown;
      for (let attempt = 0; ; attempt += 1) {
        try {
          const attemptId = attempt === 0 ? requestId : this.hub.transport.nextRequestId();
          result = await this.hub.transport.send<unknown>(slot, { kind: "turn", requestId: attemptId, actorId: activation.actorId, activationGeneration: activation.generation, events, budget }, attemptId, () => { posted = true; });
          break;
        } catch (error) {
          if (!isRetryableLifecycleTurn(error) || attempt + 1 >= RETRYABLE_LIFECYCLE_TURN_ATTEMPTS) throw error;
          posted = false;
          if (!activation.available || !slot.available || this.state.shards[slot.index] !== slot) { owner.failure = "worker-lost"; throw new Error("actor-lifecycle.worker-lost"); }
        }
      }
      this.hub.uiAcknowledgements.recordInstanceTurn(owner, result);
      if (!activation.available || !slot.available || this.state.shards[slot.index] !== slot) { owner.failure = "worker-lost"; throw new Error("actor-lifecycle.worker-lost"); }
      const status = result !== null && typeof result === "object" ? Reflect.get(result, "status") : undefined;
      const admitted = status !== null && typeof status === "object" && ["idle", "more-work", "checkpoint-ready"].includes(Reflect.get(status, "tag"));
      try { this.acceptInstanceLifecycleResult(owner, result, admitted ? acknowledged : undefined); }
      catch (error) { owner.failure = "invalid-receipt"; throw error; }
      if (!admitted) {
        owner.interruptedTurn = result;
        owner.failure = "worker-refused";
        throw new Error(acknowledged ? "actor-lifecycle.ack-not-admitted" : "actor-lifecycle.turn-not-admitted");
      }
      owner.failure = null;
      return result;
    } catch (error) {
      owner.failure ??= !activation.available || !slot.available || this.state.shards[slot.index] !== slot ? "worker-lost" : posted ? "worker-refused" : "transport-refused";
      throw error;
    } finally {
      owner.inFlight = false;
    }
  }

  private acceptInstanceLifecycleResult(owner: ShardInstanceOwner, result: unknown, acknowledged?: ActorInstanceLifecycleReceipt): void {
    const wire = result && typeof result === "object" ? Reflect.get(result, "lifecycleReceipt") : undefined;
    let incoming: ActorInstanceLifecycleReceipt | null = null;
    if (wire !== undefined && wire !== null) {
      const decoded = decodeActorInstanceLifecycle(wire);
      if (decoded.kind !== "captured" && decoded.kind !== "accepted" && decoded.kind !== "retired") throw new Error("actor-lifecycle.receipt-required");
      incoming = Object.freeze({ ...decoded, lifetime: Object.freeze(decoded.lifetime) });
    }
    let phase = owner.phase;
    let pending = owner.receipt;
    if (acknowledged && (!incoming || !actorInstanceLifecycleReceiptEquals(acknowledged, incoming))) {
      pending = null;
      phase = acknowledged.kind === "captured" ? "open" : acknowledged.kind === "retired" ? "complete" : "accepted";
    }
    if (incoming) {
      if (phase === "complete" || pending !== null && !actorInstanceLifecycleReceiptEquals(pending, incoming)) throw new Error("actor-lifecycle.receipt-mismatch");
      if (incoming.kind === "captured") {
        if (!actorInstanceCapturedReceiptMatches(owner.open, incoming) || owner.lifetime !== null && !actorInstanceLifetimeEquals(owner.lifetime, incoming.lifetime) || owner.lifetime === null && incoming.lifetime.guestLifetime <= owner.activation.lastGuestLifetime) throw new Error("actor-lifecycle.receipt-mismatch");
        phase = "captured";
      } else {
        if (!owner.close || !actorInstanceCloseReceiptMatches(owner.close, owner.accepted, incoming)) throw new Error("actor-lifecycle.receipt-mismatch");
        phase = incoming.kind;
      }
      pending = incoming;
    }
    if (incoming?.kind === "captured") {
      owner.lifetime = incoming.lifetime;
      owner.activation.lastGuestLifetime = incoming.lifetime.guestLifetime;
    }
    if (incoming?.kind === "accepted") owner.accepted = incoming;
    owner.receipt = pending;
    owner.phase = phase;
    if (phase === "complete") {
      if (owner.activation.instance === owner) owner.activation.instance = null;
      if (owner.activation.close === owner) owner.activation.close = null;
      this.state.instanceLifecycles.delete(owner.open.requestSequence);
    }
  }
}
