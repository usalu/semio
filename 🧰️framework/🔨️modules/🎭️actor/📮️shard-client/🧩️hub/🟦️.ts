import type { ResidentGrant, ResidentStep } from "@semio-tech/value-ts/resident";
import type { OwnedUiPatchAcknowledgement } from "../../../🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import type { OwnedActorTurnOutput } from "../../🪪️activation/🚪️instance/📥️output/🟦️.ts";
import type { ActorInstanceLifecycleReceipt } from "../../🚪️lifetime/🟦️.ts";
import type { HostEffectLedger, ShardActivation, ShardActorActivationLease, ShardClientIdentity, ShardInstanceOwner, ShardReturnAdmission, ShardSlot, CapturedReturn, CapturedReturnWork } from "../🔐️activation/🟦️.ts";
import type { OutboundMessage } from "../📨️messages/🟦️.ts";
import type { ShardClientState } from "../🗃️state/🟦️.ts";
import type { OwnedNativeUiPatchAuthority, OwnedNativeUiPatchSubmissionReceipt, OwnedUiPatchAcknowledgementEntry } from "../🩹️ui-patch/🟦️.ts";
import type { ShardBudget, ShardEventEnvelope, ShardFrame } from "../📜️wire/🟦️.ts";

/** 🧩️ Collaboration ports of the shard client: each collaborator names the narrow roles it needs from its siblings, so no collaborator imports the shard client that wires them. */

/** 📬️ What siblings need from the request transport. */
export interface ShardTransportRole {
  send<T>(slot: ShardSlot, message: OutboundMessage, requestId: string | null, posted?: () => void, output?: OwnedActorTurnOutput | null): Promise<T>;
  nextRequestId(): string;
  failShard(slot: ShardSlot, error: Error): void;
  spawnShard(index: number): ShardSlot;
  rejectActorPending(slot: ShardSlot, actorId: string, error: Error): void;
}

/** 🧭️ What siblings need from the shard routing. */
export interface ShardRoutingRole {
  terminate(index: number, detail?: string): readonly string[];
  rebuild(index: number): void;
  assignShard(actorId: string): ShardSlot;
}

/** 🪪️ What siblings need from the exact activations. */
export interface ShardActivationsRole {
  activationIsActive(activation: ShardActivation): boolean;
  inboundActivation(slot: ShardSlot, actorId: string, generation: bigint | null): ShardActivation | undefined;
  disposeActivation(activation: ShardActivation): void;
  captureActorActivation(actorId: string): ShardActorActivationLease;
}

/** 🚪️ What siblings need from the instance lifecycles. */
export interface ShardLifecyclesRole {
  sendInstanceLifecycle(owner: ShardInstanceOwner, events: readonly ShardEventEnvelope[], budget: ShardBudget, acknowledged?: ActorInstanceLifecycleReceipt): Promise<unknown>;
}

/** ↩️ What siblings need from the captured returns. */
export interface ShardReturnsRole {
  retireUnusedInstanceReturn(instance: ShardInstanceOwner, grant: ResidentGrant): ResidentStep;
  reserveInstanceReturn(instance: ShardInstanceOwner, maximumResponses: number, grant: ResidentGrant): ShardReturnAdmission;
}

/** 📥️ What siblings need from the UI acknowledgements. */
export interface ShardUiAcknowledgementsRole {
  submitInstanceUiAcknowledgements(owner: ShardInstanceOwner, entries: readonly OwnedUiPatchAcknowledgementEntry[], budget: ShardBudget): Promise<{ readonly receipts: readonly OwnedNativeUiPatchSubmissionReceipt[]; readonly result: unknown }>;
  submitInstanceUiAcknowledgement(owner: ShardInstanceOwner, source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement, budget: ShardBudget): Promise<{ readonly receipt: OwnedNativeUiPatchSubmissionReceipt; readonly result: unknown }>;
  recordInstanceTurn(owner: ShardInstanceOwner, result: unknown): void;
  captureInstanceUiPatch(owner: ShardInstanceOwner, turn: object, patchIndex: number): OwnedNativeUiPatchAuthority;
}

/** 🌉️ What siblings need from the host effect bridge. */
export interface ShardEffectsRole {
  handleInboundFrame(activation: ShardActivation, frame: ShardFrame): void;
  abortOutstandingEffects(actorId: string): void;
  cancelOneEffect(ledger: HostEffectLedger): void;
}

/** ⏱️ What siblings need from the watchdog. */
export interface ShardWatchdogRole {
  recordHeartbeat(slot: ShardSlot, turnSeq: number, atMs: number, phase?: string | null): void;
  noteLiveness(slot: ShardSlot, atMs: number): void;
  stopWatchdog(): void;
}

/** 🧩️ The hub every collaborator receives: the shell identity, the one private state and the sibling roles, reached only at call time. */
export interface ShardClientHubPort<Shell extends ShardClientIdentity = ShardClientIdentity> {
  readonly shell: Shell;
  readonly state: ShardClientState;
  readonly transport: ShardTransportRole;
  readonly routing: ShardRoutingRole;
  readonly activations: ShardActivationsRole;
  readonly lifecycles: ShardLifecyclesRole;
  readonly returns: ShardReturnsRole;
  readonly uiAcknowledgements: ShardUiAcknowledgementsRole;
  readonly effects: ShardEffectsRole;
  readonly watchdog: ShardWatchdogRole;
}
