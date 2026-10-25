import type { ShardClientState } from "../🗃️state/🟦️.ts";
import type { ShardClientHubPort } from "../🧩️hub/🟦️.ts";
import { OwnedUiPatchAcknowledgement } from "../../../🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import { actorInstanceLifetimeEquals } from "../../🚪️lifetime/🟦️.ts";
import { actorUiPatchReceiptEquals, decodeActorUiPatchReceipt, validateActorUiPatchPairing } from "../../🚪️lifetime/🩹️patch/🟦️.ts";
import type { ShardInstanceOwner } from "../🔐️activation/🟦️.ts";
import { type NativeUiPatchState, OwnedNativeUiPatchAuthority, type OwnedNativeUiPatchSubmissionReceipt, type OwnedUiPatchAcknowledgementEntry, mintNativePatch, mintNativeSubmission, nativePatchState } from "../🩹️ui-patch/🟦️.ts";
import type { ShardBudget, ShardEventEnvelope } from "../📜️wire/🟦️.ts";

/** 📥️ Captures the native UI patches of an instance turn and acknowledges them in one lifecycle crossing. */
export class ShardUiAcknowledgements {
  private readonly state: ShardClientState;
  private readonly hub: ShardClientHubPort;
  constructor(hub: ShardClientHubPort) { this.hub = hub; this.state = hub.state; }

  recordInstanceTurn(owner: ShardInstanceOwner, result: unknown): void {
    if (result !== null && typeof result === "object") this.state.instanceTurns.set(result, { owner, patches: new WeakMap() });
  }

  captureInstanceUiPatch(owner: ShardInstanceOwner, turn: object, patchIndex: number): OwnedNativeUiPatchAuthority {
    const captured = this.state.instanceTurns.get(turn);
    if (!captured || captured.owner !== owner || owner.lifetime === null) throw new Error("actor-lifecycle.foreign-turn");
    const patches: unknown = Reflect.get(turn, "uiPatches");
    if (!Array.isArray(patches) || !Number.isSafeInteger(patchIndex) || patchIndex < 0 || patchIndex >= patches.length) throw new Error("actor-lifecycle.patch-index");
    const wire = Reflect.get(turn, "uiPatchReceipt");
    const decoded = wire == null ? null : decodeActorUiPatchReceipt(wire);
    validateActorUiPatchPairing(patches.length, decoded);
    if (new Set(patches).size !== patches.length) throw new Error("actor-ui-patch.pairing");
    if (!decoded || !actorInstanceLifetimeEquals(decoded.lifetime, owner.lifetime)) throw new Error("actor-ui-patch.lifetime-mismatch");
    const patch: unknown = patches[patchIndex];
    if (patch === null || typeof patch !== "object") throw new Error("actor-lifecycle.patch-envelope");
    const existing = captured.patches.get(patch);
    if (existing) {
      if (!actorUiPatchReceiptEquals(existing.value.receipt, decoded)) throw new Error("actor-ui-patch.receipt-mismatch");
      return existing;
    }
    if (decoded.patchSequence < owner.lastPatchSequence || (decoded.patchSequence === owner.lastPatchSequence && owner.lastPatchTurn !== turn)) throw new Error("actor-ui-patch.duplicate-sequence");
    const surface: unknown = Reflect.get(patch, "surface");
    const operations: unknown = Reflect.get(patch, "ops");
    const revision: unknown = Reflect.get(patch, "revision");
    const base: unknown = Reflect.get(patch, "baseRevision");
    const exactRevision = (value: unknown): number => {
      if (typeof value === "bigint" && value >= 0n && value <= BigInt(Number.MAX_SAFE_INTEGER)) return Number(value);
      if (typeof value === "number" && Number.isSafeInteger(value) && value >= 0) return value;
      throw new Error("actor-lifecycle.patch-revision");
    };
    if (!surface || typeof surface !== "object" || Reflect.get(surface, "instance") !== owner.lifetime.instanceId || !Array.isArray(operations)) throw new Error("actor-lifecycle.patch-envelope");
    const name: unknown = Reflect.get(surface, "surface");
    if (typeof name !== "string" || name.length > 512 || new TextEncoder().encode(name).length > 512) throw new Error("actor-lifecycle.patch-surface");
    const receipt = Object.freeze({ lifetime: Object.freeze(decoded.lifetime), patchSequence: decoded.patchSequence });
    const value = Object.freeze({ activation: owner.operation, lifetime: owner.lifetime, receipt, surface: name, revision: exactRevision(revision), baseRevision: exactRevision(base), operationCount: operations.length });
    const authority = mintNativePatch({ owner, turn, patch, operations, value, ordinal: 0, read: false, original: undefined, input: null, token: null, submission: null });
    captured.patches.set(patch, authority);
    owner.lastPatchSequence = receipt.patchSequence;
    owner.lastPatchTurn = turn;
    return authority;
  }

  /** 🧾️ Validates ONE (patch, acknowledgement) pair against the instance owner and returns its
   * private state, so the single and the batched submission agree on every clause by construction. */
  private admitInstanceUiAcknowledgement(owner: ShardInstanceOwner, source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement): NativeUiPatchState {
    if (!owner.lifetime || !OwnedNativeUiPatchAuthority.matches(source, owner.operation, owner.lifetime) || !OwnedUiPatchAcknowledgement.matches(token, source)) throw new Error("actor-lifecycle.ui-ack-mismatch");
    const state = nativePatchState(source);
    const value = token.value;
    if (state.owner !== owner || state.token !== null && state.token !== token || value.actor !== owner.activation.actorId || value.instance !== owner.lifetime.instanceId || value.surface !== state.value.surface || value.revision !== state.value.revision || !actorInstanceLifetimeEquals(value.lifetime, owner.lifetime) || !actorUiPatchReceiptEquals(value.receipt, state.value.receipt)) throw new Error("actor-lifecycle.ui-ack-mismatch");
    if (!source.inputRetired) throw new Error("actor-lifecycle.ui-input-pending");
    return state;
  }

  private patchAckEvent(owner: ShardInstanceOwner, state: NativeUiPatchState): ShardEventEnvelope {
    return { kind: "patch-ack", payload: { receipt: state.value.receipt, surface: { instance: owner.lifetime!.instanceId, surface: state.value.surface }, revision: BigInt(state.value.revision) } };
  }

  /** 💊️ Acknowledges a whole turn's patch BATCH in ONE lifecycle crossing.
   *
   * 🐛️ A turn result may now carry every surface that was ready, and acknowledging them one at a time
   * put the N host round trips the guest had just stopped paying straight back on the inbound side —
   * the reactor's `poll` takes a LIST of events, so N `patch-ack`s ride one crossing and the guest
   * retires all N in the same turn. */
  async submitInstanceUiAcknowledgements(owner: ShardInstanceOwner, entries: readonly OwnedUiPatchAcknowledgementEntry[], budget: ShardBudget): Promise<{ readonly receipts: readonly OwnedNativeUiPatchSubmissionReceipt[]; readonly result: unknown }> {
    if (entries.length === 0) throw new Error("actor-lifecycle.ui-ack-mismatch");
    const states = entries.map((entry) => this.admitInstanceUiAcknowledgement(owner, entry.source, entry.token));
    if (states.every((state) => state.submission !== null)) {
      const settled = await Promise.all(states.map((state) => state.submission!));
      return Object.freeze({ receipts: settled.map((value) => value.receipt), result: settled[0]!.result });
    }
    if (states.some((state) => state.submission !== null)) throw new Error("actor-lifecycle.ui-ack-mismatch");
    const crossing = (async () => {
      const result = await this.hub.lifecycles.sendInstanceLifecycle(owner, states.map((state) => this.patchAckEvent(owner, state)), budget);
      const status = result !== null && typeof result === "object" ? Reflect.get(result, "status") : undefined;
      if (!status || typeof status !== "object" || !["idle", "more-work"].includes(Reflect.get(status, "tag"))) throw new Error("actor-lifecycle.ui-ack-not-admitted");
      return result;
    })();
    entries.forEach((entry, index) => {
      const state = states[index]!;
      state.token = entry.token;
      state.submission = crossing.then((result) => Object.freeze({ receipt: mintNativeSubmission(entry.source, entry.token), result }));
    });
    try {
      const settled = await Promise.all(states.map((state) => state.submission!));
      return Object.freeze({ receipts: settled.map((value) => value.receipt), result: settled[0]!.result });
    } catch (error) {
      for (const state of states) state.submission = null;
      throw error;
    }
  }

  /** 🩹️ The one-patch case of [`submitInstanceUiAcknowledgements`] — a batch of one, so both paths
   * admit, cross and memoize through exactly the same code. */
  async submitInstanceUiAcknowledgement(owner: ShardInstanceOwner, source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement, budget: ShardBudget): Promise<{ readonly receipt: OwnedNativeUiPatchSubmissionReceipt; readonly result: unknown }> {
    const batch = await this.submitInstanceUiAcknowledgements(owner, [{ source, token }], budget);
    return Object.freeze({ receipt: batch.receipts[0]!, result: batch.result });
  }
}
