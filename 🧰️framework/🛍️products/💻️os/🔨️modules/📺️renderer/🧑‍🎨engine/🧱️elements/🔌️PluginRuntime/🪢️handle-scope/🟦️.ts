/** 🪢️ Phases 2, 3 and 5 of `loadPluginModule`: the per-handle scope with its instance tables and counters, the instance table and the lazily activated request actor.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { OwnedUiPatchIntake } from "../../📃️UiDocumentStore/📥️intake/🟦️.ts";
// #region 🔌️Adapters
import { ActivationRegistry } from "../../../../../../../../🔨️modules/🎠️kernel/🎛️activation-registry/🟦️.ts";
import { type ActivationReason } from "../../../../../../../../🔨️modules/🎠️kernel/🚦️activation-reason/🟦️.ts";
import { createTurnOutcomeBroadcast, type TurnOutcome } from "../../../../../../../../🔨️modules/🎠️kernel/📡️turn-outcome-broadcast/🟦️.ts";
import { type PluginWasmHandle as KernelPluginWasmHandle } from "../../../../../../../../🔨️modules/🎠️kernel/🪝️plugin-wasm-handle/🟦️.ts";
import {
  ShardClient,
  type ShardActorActivationLease,
  type ShardCommandIngressPage,
  type ShardEventEnvelope,
  type ShardInstanceLifecycleLease
} from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
// #endregion 🔌️Adapters
import { OwnedUiInstance, type OwnedUiInstanceRetirement, type OwnedUiInstanceSurface } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import { type Lane } from "../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts";
import { ActorDocumentBindingV1 } from "../../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts";
import { type WireTurnResult } from "../🪝️wire-codec/🟦️.ts";
import { forgetInstanceForRecovery, getShardClient } from "../🛰️runtime-singletons/🟦️.ts";
import { submitPluginTurn, teardownPluginActor } from "../🧵️turn-dispatch/🟦️.ts";
import { completionFanouts, pendingCompletionEffects, pendingTurnEffects } from "../🏁️operation-completion/🟦️.ts";
import { markPluginInstanceRetiredV1 } from "../🪦️instance-retirement/🟦️.ts";
import { guestIngressGenerationByInstance } from "../📥️invocation-frames/🟦️.ts";

//#region 🔖️plugin-runtime
//#region 🔖️ActorAdapter
/** 🪪️ H1-react — instance ids must be unique across EVERY plugin, not just within one
 * `loadPluginModule` call: `pendingTurnEffects` above is keyed by `instanceId` alone and is shared
 * module-wide (mirrors `🦀️.rs`'s native `KernelClient` — `next_instance_id` lives on the ONE
 * global `KernelThreadState`, not per-plugin). A per-plugin-scoped counter would let two different
 * plugins both mint instance `1` and silently cross-read each other's leftover turn effects. */
export let nextGlobalInstanceId = 1;
//#endregion 🔖️ActorAdapter

/** 🪪️ Mints the next process-wide instance id — the one writer of {@link nextGlobalInstanceId}, so a module that
 * only holds an import of it never has to assign it. */
export function takeNextGlobalInstanceId(): number {
  const instanceId = nextGlobalInstanceId;
  nextGlobalInstanceId += 1;
  return instanceId;
}

/** 🪢️ Everything one `loadPluginModule` call shares across its phases: the per-handle instance
 * tables, the retained-UI ledgers, the outcome broadcast and the handful of mutable counters
 * (`state`). Built once by {@link createPluginHandleScope}; every later phase receives it explicitly. */
export type PluginHandleScope = Readonly<{
  pluginId: string;
  manifest: KernelPluginWasmHandle["manifest"];
  registry: ActivationRegistry;
  shardClient: ShardClient;
  turnOutcomes: ReturnType<typeof createTurnOutcomeBroadcast<TurnOutcome>>;
  state: { disposing: boolean; disposal: Promise<void> | null; eventSeq: number; requestActor: Promise<string> | null; extensionRequestSeq: bigint };
  actorIdByInstance: Map<number, string>;
  closingInstances: Set<number>;
  openingInstances: Map<number, Promise<number>>;
  retiringInstances: Map<number, Promise<void>>;
  documentBindings: Map<number, ActorDocumentBindingV1>;
  documentBindingGenerations: Map<number, bigint>;
  lifecycleByInstance: Map<number, ShardInstanceLifecycleLease>;
  uiOwnerByInstance: Map<number, OwnedUiInstance>;
  uiRetirementByInstance: Map<number, OwnedUiInstanceRetirement>;
  uiSurfaceByInstance: Map<number, Map<string, OwnedUiInstanceSurface>>;
  uiIntakesByInstance: Map<number, Set<OwnedUiPatchIntake>>;
  uiReadsByInstance: Map<number, Set<Promise<unknown>>>;
}>;

/** 🪢️ Phase 2 of {@link loadPluginModule}: acquires the shared `ShardClient` and creates the empty per-handle
 * instance tables, the mutable counters and the turn-outcome broadcast. */
export function createPluginHandleScope(pluginId: string, manifest: KernelPluginWasmHandle["manifest"], registry: ActivationRegistry): PluginHandleScope {
  const shardClient = getShardClient();
  const actorIdByInstance = new Map<number, string>();
  const closingInstances = new Set<number>();
  const openingInstances = new Map<number, Promise<number>>();
  const retiringInstances = new Map<number, Promise<void>>();
  const state = { disposing: false, disposal: null as Promise<void> | null, eventSeq: 0, requestActor: null as Promise<string> | null, extensionRequestSeq: 0n };
  const documentBindings = new Map<number, ActorDocumentBindingV1>();
  const documentBindingGenerations = new Map<number, bigint>();
  /** 🚪️ One captured lifecycle owner per live instance — `createApp` opens through it so the guest
   * receives the `activation-generation`/`request-sequence` authority its own wire decoder demands. */
  const lifecycleByInstance = new Map<number, ShardInstanceLifecycleLease>();
  const uiOwnerByInstance = new Map<number, OwnedUiInstance>();
  const uiRetirementByInstance = new Map<number, OwnedUiInstanceRetirement>();
  const uiSurfaceByInstance = new Map<number, Map<string, OwnedUiInstanceSurface>>();
  const uiIntakesByInstance = new Map<number, Set<OwnedUiPatchIntake>>();
  const uiReadsByInstance = new Map<number, Set<Promise<unknown>>>();
  /** 📤️📥️ Backs {@link KernelPluginWasmHandle.enqueue}/`.outcomes` (see this file's own header doc).
   * One broadcast per `loadPluginModule` call, matching the handle's own
   * lifetime: every instance this handle ever `createApp`s shares it, and each instance's
   * `AppChannelClient` (`💻️os/🟦️.ts`) filters to its own `instanceId`. */
  const turnOutcomes = createTurnOutcomeBroadcast<TurnOutcome>();
  return {
    pluginId,
    manifest,
    registry,
    shardClient,
    turnOutcomes,
    state,
    actorIdByInstance,
    closingInstances,
    openingInstances,
    retiringInstances,
    documentBindings,
    documentBindingGenerations,
    lifecycleByInstance,
    uiOwnerByInstance,
    uiRetirementByInstance,
    uiSurfaceByInstance,
    uiIntakesByInstance,
    uiReadsByInstance,
  };
}

/** 🗂️ Phase 3 of {@link loadPluginModule}: the instance table — resolving an instance id to its actor, releasing every
 * per-instance map when it goes, and submitting a turn on its behalf. */
export function createInstanceTable(scope: PluginHandleScope) {
  const { pluginId, registry, state, actorIdByInstance, closingInstances, documentBindings, documentBindingGenerations, lifecycleByInstance, uiOwnerByInstance, uiRetirementByInstance, uiSurfaceByInstance, uiIntakesByInstance, uiReadsByInstance } = scope;
  const requireActorId = (instanceId: number): string => {
    const actorId = actorIdByInstance.get(instanceId);
    if (state.disposing) throw new Error("plugin-handle.closed");
    if (!actorId || closingInstances.has(instanceId)) throw markPluginInstanceRetiredV1(new Error(`program ${pluginId}: no actor for instance ${instanceId} (createApp not called, or already destroyed)`));
    return actorId;
  };
  const releaseInstanceMaps = (instanceId: number, actorId: string): void => {
    forgetInstanceForRecovery(actorId);
    guestIngressGenerationByInstance.delete(instanceId);
    documentBindings.delete(instanceId);
    documentBindingGenerations.delete(instanceId);
    actorIdByInstance.delete(instanceId);
    lifecycleByInstance.delete(instanceId);
    uiOwnerByInstance.delete(instanceId);
    uiRetirementByInstance.delete(instanceId);
    uiSurfaceByInstance.delete(instanceId);
    uiIntakesByInstance.delete(instanceId);
    uiReadsByInstance.delete(instanceId);
    pendingTurnEffects.delete(instanceId);
    pendingCompletionEffects.delete(instanceId);
    completionFanouts.delete(instanceId);
    teardownPluginActor(actorId);
    closingInstances.delete(instanceId);
  };
  /** 🚦 `lane`/`turnKey` forward to {@link submitPluginTurn} — see that function's own doc for the
   * lane-assignment reasoning. `registry.touch(actorId)` refreshes this actor's LRU position on every
   * turn (its own doc: "call on every turn, not just activation"); turns dispatch through this file's
   * own {@link submitPluginTurn} rather than `ActivationRegistry.enqueueTurn` (see that decision's
   * write-up above `serializePerActor`), so nothing else would ever call it. */
  const submitTurn = (actorId: string, events: readonly ShardEventEnvelope[], options?: { readonly lane?: Lane; readonly turnKey?: string; readonly commandPage?: ShardCommandIngressPage; readonly activation?: ShardActorActivationLease }): Promise<WireTurnResult> => {
    registry.touch(actorId);
    return submitPluginTurn(actorId, events, options?.lane ?? "Interactive", options?.turnKey, options?.commandPage, options?.activation);
  };
  return { requireActorId, releaseInstanceMaps, submitTurn };
}

/** 📥️ Phase 5 of {@link loadPluginModule}: the lazily activated inbound-request actor of this handle. */
export function createRequestActor(scope: PluginHandleScope) {
  const { pluginId, registry, state } = scope;
  /** 📥️ This handle's inbound-request actor — see {@link PluginWasmHandle.invoke}. Activated on the
   * first call so a program nobody ever calls into costs no worker residency at all. */
  const ensureRequestActor = (): Promise<string> => {
    if (state.disposing) return Promise.reject(new Error("plugin-handle.closed"));
    state.requestActor ??= (async () => {
      const actorId = `${pluginId}#request`;
      await registry.activate(pluginId, actorId, "manual" satisfies ActivationReason);
      return actorId;
    })().catch((error: unknown) => {
      state.requestActor = null;
      throw error;
    });
    return state.requestActor;
  };
  const retireRequestActor = async (): Promise<void> => {
    const pending = state.requestActor;
    state.requestActor = null;
    if (!pending) return;
    const actorId = await pending.catch(() => null);
    if (!actorId) return;
    registry.cancel(actorId);
    teardownPluginActor(actorId);
  };
  return { ensureRequestActor, retireRequestActor };
}
//#endregion 🔖️plugin-runtime
