// #region 🧲️Header
/** @emoji 🐚️ wgpu-web's plugin-loading + bridge-adapter pair — MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME
 * (`wgpu-web-shard`) replacement for the deleted `acquirePluginModule`/`pluginHandleForBridge` (both
 * removed from `@semio-tech/framework` when the kernel was ported — see `📓️terra-web-shard-*` reports)
 * and for `🟦️boot.ts`'s own retired `PluginWorkerClient` (one dedicated `Worker` per plugin, the
 * OLD synchronous-ish request/response ABI). `loadPluginModule` now drives a real actor through the
 * kernel's `ActivationRegistry` over `ShardClient` (bounded shard-worker pool, `actorId`-multiplexed) —
 * copying `PluginRuntime/🟦️component.tsx`'s shape exactly, as this packet's brief requires, rather than
 * inventing a second worker-management scheme. `pluginHandleForBridge` then adapts the typed
 * {@link WgpuPluginHandle} down to the raw string-in/string-out JS surface
 * `ProgramBridge/🧊️component.rs`'s `js_sys::Reflect::get(handle, "createApp"/"handleAction"/...)` still
 * expects on `wasm32` — that Rust file is outside this packet's lease (pure-TypeScript,
 * "do not wait on any Rust crate"), so this adapter preserves its existing contract rather than
 * changing it.
 *
 * Reuse decisions (see `📓️terra-wgpu-web-shard-report.md` for the full write-up):
 * - `ActivationRegistry`/`Effect`/`InvocationResponse`/`PluginManifest`/`SemioFaultError` come from
 *   `@semio-tech/framework` (already a dependency — that package's `🟦️glue.ts` re-exports the whole
 *   kernel + manifest modules).
 * - `AppChannelClient` + the pack/fault codec come from `@semio-tech/framework-os` (NEW dependency
 *   added to this package's `package.json` — a pure sync/protocol package, no React in its import
 *   graph, the same package `PluginRuntime` itself depends on for this exact class).
 * - `ShardClient` and the pool-bootstrap/wire-turn-interpretation helpers are NOT re-exported by any
 *   package, so they're imported by relative path — `🧵️shard-runtime.ts`/`🖼️wire-turn.ts` (both NEW,
 *   `🎭️actor/📦️packages/🟦️typescript/`) are this packet's lift of the generic (non-React) half of
 *   `PluginRuntime`'s `🔖️ActorAdapter`/`🔖️RetainedUiPatch` regions, so this file does not reimplement
 *   worker-pool bootstrap or UI-patch reconciliation a second time. `PluginRuntime` itself still carries
 *   its own inline copy (outside this packet's lease to edit) — a follow-up should point it at the same
 *   two modules.
 * - Deliberately NOT reused: `PluginRuntime`'s own `adaptPluginHandle`/`AppChannelClient`-wide-surface
 *   wrapper (transactions/merge/conflicts/backbone/presence) — `ProgramBridge/🧊️component.rs`'s
 *   `wasm32` branch only ever calls `manifest`/`createApp`/`destroyApp`/`handleAction`/`handleCommand`/
 *   `render`/`contextMenu`, so building the rest would be dead code for this target.
 * - Turn serialization: `PluginRuntime` runs a lane-prioritizing, coalescing `TurnScheduler` on top of
 *   `ShardClient.turn`. wgpu's own call pattern has no redraw-burst pressure (one winit-driven caller,
 *   not a pointer-move loop), so `submitTurn` below is a plain per-actor promise chain instead — enough
 *   to satisfy the shard worker's "never two turns in flight for one actor" rule without importing
 *   `TurnScheduler` for a guarantee this target doesn't need yet.
 *
 * Honest gap: `render` has no wire counterpart any more (channel v12 retired the per-verb
 * `render`/`renderWithDocument` command) — it is rebuilt here on top of a raw `"surface-visible"` turn
 * event + the retained-patch reconciliation `🖼️wire-turn.ts` provides, exactly mirroring
 * `PluginRuntime`'s own `refreshUi`. `windowEngagements`/`windowMeasures` are left unimplemented
 * (Rust's `ProgramBridge` already tolerates a missing function there with an empty-map fallback) —
 * `PluginRuntime` documents the identical gap ("no wire path yet, `dirty_render` only ever renders the
 * ONE window surface this wave").
 */
// #endregion 🧲️Header

// #region 🔌️Imports
import {
  ActivationRegistry,
  type ActivationReason,
  type Effect,
  type InvocationResponse,
  type PluginManifest,
  type PluginWasmHandle as KernelPluginWasmHandle,
  SemioFaultError,
} from "@semio-tech/framework";
import { AppChannelClient, decodeFaultFromWire, decodePackValue, encodePackValue, faultDisplayMessage } from "@semio-tech/framework-os";
import { ShardClient, type ShardEventEnvelope } from "../../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts";
import { createPooledActorRuntime, DEFAULT_SHARD_BUDGET, type PooledActorRuntime } from "../../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-runtime.ts";
import {
  applyUiPatchToRetained,
  coerceTurnResult,
  decodeWirePatchOps,
  shellFrameBytes,
  wireEffectToFriendly,
  type RetainedSurface,
  type WireTurnResult,
  type WireUiPatch,
  type WireVariant,
} from "../../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts";
// #endregion 🔌️Imports

//#region 🔖️PooledSingletons
let pooledRuntime: PooledActorRuntime | null = null;
function getShardClient(): ShardClient {
  pooledRuntime ??= createPooledActorRuntime({
    onActorTrap: (actorId, message) => console.error(`[DEBUG] wgpu plugin-bridge: actor ${actorId} trapped: ${message}`),
    onShardLost: (shardIndex, actorIds) => {
      console.error(`[DEBUG] wgpu plugin-bridge: shard ${shardIndex} lost, restoring actors: ${actorIds.join(", ")}`);
      getActivationRegistry().handleShardLost(shardIndex, actorIds);
    },
  });
  return pooledRuntime.shardClient;
}

let sharedActivationRegistry: ActivationRegistry | null = null;
function getActivationRegistry(): ActivationRegistry {
  sharedActivationRegistry ??= new ActivationRegistry({ shardClient: getShardClient(), defaultBudget: DEFAULT_SHARD_BUDGET });
  return sharedActivationRegistry;
}
//#endregion 🔖️PooledSingletons

//#region 🔖️TurnSubmit
/** 🚦 Plain per-actor promise chain — never lets a second `turn()` start for the same `actorId` before
 * the previous one settles (the shard worker rejects, not queues, an overlapping turn). See this
 * file's header doc for why this is deliberately simpler than `PluginRuntime`'s lane/coalescing
 * `TurnScheduler`. */
const actorTurnChains = new Map<string, Promise<unknown>>();
function submitTurn(actorId: string, events: readonly ShardEventEnvelope[]): Promise<WireTurnResult> {
  getActivationRegistry().touch(actorId);
  const previousSettled = (actorTurnChains.get(actorId) ?? Promise.resolve()).catch(() => undefined);
  const next = previousSettled.then(() => getShardClient().turn(actorId, events, DEFAULT_SHARD_BUDGET));
  actorTurnChains.set(actorId, next);
  return next.then(coerceTurnResult);
}
//#endregion 🔖️TurnSubmit

//#region 🔖️RetainedWindow
const retainedWindowByActor = new Map<string, RetainedSurface>();

function applyRetainedWindowPatches(actorId: string, uiPatches: readonly WireUiPatch[]): void {
  for (const patch of uiPatches) {
    const ops = decodeWirePatchOps(patch.ops ?? [], decodePackValue);
    const previous = retainedWindowByActor.get(actorId) ?? null;
    const { surface, desynced } = applyUiPatchToRetained(previous, { revision: patch.revision ?? 0, baseRevision: patch.baseRevision ?? 0, ops });
    if (desynced) {
      console.warn(`[DEBUG] plugin-bridge: actor ${actorId} desynced (unrecognized op shape or stale baseRevision) — keeping the previously retained body`);
      continue;
    }
    if (surface) retainedWindowByActor.set(actorId, surface);
  }
}

/** 🖼️ Channel v12 retired the per-verb `render`/`renderWithDocument` `AppCommand` — rebuilt here as a
 * raw `"surface-visible"` turn event, reading back whatever this SAME turn's `TurnResult.uiPatches`
 * produced (or the retained tree if nothing changed). Only the ONE "window" surface renders this wave
 * (`⚛️reactor/🦀️component.rs`'s `dirty_render` loop hardcodes it) — `bodyKey`/`viewState` are accepted
 * for ABI compatibility but not yet threaded through, matching `PluginRuntime`'s own identical gap. */
async function performRender(actorId: string, instanceId: number): Promise<unknown> {
  const result = await submitTurn(actorId, [{ kind: "surface-visible", payload: { surface: { instance: instanceId, surface: 0 } } }]);
  if (result.uiPatches.length > 0) applyRetainedWindowPatches(actorId, result.uiPatches);
  return retainedWindowByActor.get(actorId)?.node ?? null;
}
//#endregion 🔖️RetainedWindow

//#region 🔖️Invocation
/** 🎯️ Per-instance "leftover" `TurnResult.effects` — everything a turn produced that was NOT a
 * `SendMessage{Shell}` reply frame. Filled by `exchange` on every turn, drained by `performInvocation`
 * right after its own `client.command()` call resolves. */
const pendingTurnEffects = new Map<number, WireVariant[]>();

/** 🪪️ Instance ids are unique across EVERY plugin `loadPluginModule` loads, not just within one call —
 * `pendingTurnEffects` is keyed by `instanceId` alone and shared module-wide, mirroring the kernel's own
 * single global `next_instance_id`. */
let nextGlobalInstanceId = 1;

async function performInvocation(client: AppChannelClient, instanceId: number, invocation: unknown, viewState: unknown): Promise<InvocationResponse> {
  const frames = await client.command(encodePackValue(invocation), viewState);
  let output: unknown = null;
  let diagnostics: InvocationResponse["diagnostics"] = [];
  let uiScope: InvocationResponse["uiScope"];
  let historyPatch: InvocationResponse["historyPatch"];
  for (const frame of frames) {
    if ("Invocation" in frame) {
      output = decodePackValue(new Uint8Array(frame.Invocation.output));
      const decodedDiagnostics = decodePackValue(new Uint8Array(frame.Invocation.diagnostics));
      diagnostics = Array.isArray(decodedDiagnostics) ? (decodedDiagnostics as InvocationResponse["diagnostics"]) : [];
      uiScope = decodePackValue(new Uint8Array(frame.Invocation.ui_scope)) as InvocationResponse["uiScope"];
      const decodedHistoryPatch = decodePackValue(new Uint8Array(frame.Invocation.history_patch));
      historyPatch = decodedHistoryPatch && typeof decodedHistoryPatch === "object" ? (decodedHistoryPatch as InvocationResponse["historyPatch"]) : undefined;
    } else if ("Error" in frame) {
      const fault = decodeFaultFromWire(frame.Error.fault, decodePackValue);
      if (fault) throw new SemioFaultError(fault);
      throw new Error(`invocation failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
    }
  }
  const leftover = pendingTurnEffects.get(instanceId) ?? [];
  pendingTurnEffects.delete(instanceId);
  const requestedEffects = leftover.map((effect) => wireEffectToFriendly(effect, decodePackValue)).filter((effect): effect is Effect => effect !== null);
  return { output, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] }, diagnostics, requestedEffects, events: [], uiScope, historyPatch };
}
//#endregion 🔖️Invocation

//#region 🔖️DescriptorManifest
/** 📇️ Reads the build-time `🔣️descriptor.json` siblinged next to `moduleUrl`'s directory — an honest
 * EMPTY manifest (zero apps), not a fabricated one, whenever no descriptor exists yet. Never
 * instantiates wasm to ask. Mirrors `PluginRuntime/🟦️component.tsx`'s `fetchDescriptorManifest`
 * (small enough — one `fetch` + fallback — that lifting it into a shared module wasn't worth a new
 * cross-target dependency for two nearly-identical ~15-line copies). */
async function fetchDescriptorManifest(pluginId: string, moduleUrl: string, signal?: AbortSignal): Promise<PluginManifest> {
  const descriptorUrl = moduleUrl.replace(/\/[^/]+$/, "/🔣️descriptor.json");
  try {
    const response = await fetch(descriptorUrl, signal ? { signal } : undefined);
    if (response.ok) {
      const descriptor = (await response.json()) as { readonly manifest?: PluginManifest };
      if (descriptor.manifest) return descriptor.manifest;
    }
  } catch (error) {
    if (signal?.aborted) throw error;
    console.warn(`[DEBUG] fetchDescriptorManifest: ${descriptorUrl} unreachable — using an empty manifest`, error);
  }
  console.warn(`[DEBUG] fetchDescriptorManifest: no descriptor for ${pluginId} yet — loading with an empty manifest, no eager instantiation`);
  return { pluginId, label: pluginId, version: "", apps: [], workflows: [], examples: [] };
}
//#endregion 🔖️DescriptorManifest

//#region 🔖️WgpuPluginHandle
/** 🐚️ The typed handle this file hands to a `bootFrameworkOsWgpu`/`🟦️boot.ts` caller — narrower than
 * `PluginRuntime`'s wide `PluginWasmHandle` (no transactions/merge/conflicts/backbone/presence): only
 * the surface `ProgramBridge/🧊️component.rs`'s `wasm32` branch actually calls. */
export interface WgpuPluginHandle {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly handleAction: (instanceId: number, actionJson: string, viewState: unknown) => Promise<InvocationResponse>;
  readonly handleCommand: (instanceId: number, commandJson: string, viewState: unknown) => Promise<InvocationResponse>;
  readonly render: (instanceId: number, bodyKey: string, viewState: unknown) => Promise<unknown>;
  readonly contextMenu: (instanceId: number, request: unknown) => Promise<unknown>;
  readonly dispose: () => void;
}

/** 🐚️ Acquires a real actor through `ActivationRegistry`/`ShardClient` (replacing the deleted
 * `acquirePluginModule`/per-plugin `Worker` — design-runtime.md §3, copying `PluginRuntime`'s own
 * `loadPluginModule` shape). `dispose()` disposes every instance's worker-side actor entry via
 * `ShardClient.dispose` — no shared module lease to refcount any more, one actor belongs to exactly
 * one instance. */
export async function loadPluginModule(pluginId: string, moduleUrl: string, signal?: AbortSignal): Promise<WgpuPluginHandle> {
  const registry = getActivationRegistry();
  registry.registerManifest({ pluginId, moduleUrl, caps: [] });
  const manifest = await fetchDescriptorManifest(pluginId, moduleUrl, signal);
  const shardClient = getShardClient();
  const actorIdByInstance = new Map<number, string>();
  const channelByInstance = new Map<number, AppChannelClient>();
  let eventSeq = 0;

  const requireActorId = (instanceId: number): string => {
    const actorId = actorIdByInstance.get(instanceId);
    if (!actorId) throw new Error(`[DEBUG] program ${pluginId}: no actor for instance ${instanceId} (createApp not called, or already destroyed)`);
    return actorId;
  };
  const requireChannel = (instanceId: number): AppChannelClient => {
    const client = channelByInstance.get(instanceId);
    if (!client) throw new Error(`[DEBUG] program ${pluginId}: no channel for instance ${instanceId} (createApp not called, or already destroyed)`);
    return client;
  };

  // 📡️ The raw `exchange`-shaped primitive `AppChannelClient` frames every `AppCommand`/`AppFrame`
  // through — one `"app-command"` shard event per batched frame, demuxing the resulting turn's
  // `Effect::SendMessage{Shell}` replies back into frames and stashing everything else as this
  // instance's leftover effects (`performInvocation` drains them). Mirrors `PluginRuntime`'s own
  // `KernelPluginWasmHandle.exchange` exactly.
  const exchangeHandle: Pick<KernelPluginWasmHandle, "exchange"> = {
    exchange: async (instanceId, frames) => {
      const actorId = requireActorId(instanceId);
      const events: ShardEventEnvelope[] = frames.map((frame) => {
        eventSeq += 1;
        return { kind: "app-command", payload: { instance: instanceId, seq: eventSeq, command: Array.from(frame) } };
      });
      const result = await submitTurn(actorId, events);
      const outFrames: Uint8Array[] = [];
      const leftover: WireVariant[] = [];
      for (const effect of result.effects) {
        const frame = shellFrameBytes(effect, instanceId);
        if (frame) outFrames.push(frame);
        else leftover.push(effect);
      }
      pendingTurnEffects.set(instanceId, leftover);
      if (result.uiPatches.length > 0) applyRetainedWindowPatches(actorId, result.uiPatches);
      return outFrames;
    },
  };

  return {
    pluginId,
    manifest,
    createApp: async (appId) => {
      const instanceId = nextGlobalInstanceId;
      nextGlobalInstanceId += 1;
      const actorId = `${pluginId}#${instanceId}`;
      actorIdByInstance.set(instanceId, actorId);
      await registry.activate(pluginId, actorId, "manual" satisfies ActivationReason);
      eventSeq += 1;
      await submitTurn(actorId, [{ kind: "instance-open", payload: { instance: instanceId, appId, actor: "local", config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})) } }]);
      channelByInstance.set(instanceId, new AppChannelClient(exchangeHandle, instanceId, appId, "local"));
      return instanceId;
    },
    destroyApp: async (instanceId) => {
      const actorId = actorIdByInstance.get(instanceId);
      if (!actorId) return;
      actorIdByInstance.delete(instanceId);
      channelByInstance.delete(instanceId);
      retainedWindowByActor.delete(actorId);
      pendingTurnEffects.delete(instanceId);
      shardClient.dispose(actorId);
    },
    handleAction: (instanceId, actionJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(actionJson), viewState),
    handleCommand: (instanceId, commandJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(commandJson), viewState),
    render: (instanceId) => performRender(requireActorId(instanceId), instanceId),
    contextMenu: (instanceId, request) => requireChannel(instanceId).contextMenu(request),
    dispose: () => {
      for (const actorId of actorIdByInstance.values()) {
        retainedWindowByActor.delete(actorId);
        shardClient.dispose(actorId);
      }
      actorIdByInstance.clear();
      channelByInstance.clear();
    },
  };
}
//#endregion 🔖️WgpuPluginHandle

//#region 🔖️JsBridge
/** 🌉️ The raw string-in/string-out JS surface `ProgramBridge/🧊️component.rs`'s `wasm32` branch reads
 * via `Reflect::get` — `manifest`/`createApp`/`render` are HARD requirements at
 * `ProgramBridgeEntry::from_js` construction time; the rest are looked up lazily per call
 * (`destroyApp`/`handleAction` degrade to a harmless no-op/empty result if absent, `handleCommand`
 * errors loudly if actually invoked while absent — this adapter always provides all of them, so none
 * of those fallbacks trigger). */
export interface WgpuJsBridge {
  readonly manifest: () => string;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly handleAction: (instanceId: number, actionJson: string, contextJson: string) => Promise<string>;
  readonly handleCommand: (instanceId: number, commandJson: string, contextJson: string) => Promise<string>;
  readonly render: (instanceId: number, bodyKey: string, viewStateJson: string) => Promise<string>;
  readonly contextMenu: (instanceId: number, requestJson: string) => Promise<string>;
}

/** 📥️ `ProgramBridge/🧊️component.rs`'s `handle_action_js`/`handle_command_js` pass a THIRD argument
 * that is `{"viewState": ..., "actor": "local"}` JSON (its own `context_json`, not the bare view
 * state) — this unwraps `.viewState` from it. The pre-rewrite `🟦️boot.ts` fed that whole context
 * object straight through as "viewState" without unwrapping it first (a latent double-wrap bug this
 * rewrite fixes in passing, not something this packet was asked to hunt for). */
function viewStateFromContextJson(contextJson: string): unknown {
  try {
    const parsed = JSON.parse(contextJson) as { readonly viewState?: unknown } | null;
    return parsed && typeof parsed === "object" && "viewState" in parsed ? parsed.viewState : parsed;
  } catch {
    return undefined;
  }
}

export function pluginHandleForBridge(handle: WgpuPluginHandle): WgpuJsBridge {
  return {
    manifest: () => JSON.stringify(handle.manifest),
    createApp: (appId) => handle.createApp(appId),
    destroyApp: (instanceId) => handle.destroyApp(instanceId),
    handleAction: (instanceId, actionJson, contextJson) => handle.handleAction(instanceId, actionJson, viewStateFromContextJson(contextJson)).then((result) => JSON.stringify(result)),
    handleCommand: (instanceId, commandJson, contextJson) => handle.handleCommand(instanceId, commandJson, viewStateFromContextJson(contextJson)).then((result) => JSON.stringify(result)),
    render: (instanceId, bodyKey, viewStateJson) => handle.render(instanceId, bodyKey, JSON.parse(viewStateJson)).then((node) => JSON.stringify(node)),
    contextMenu: (instanceId, requestJson) => handle.contextMenu(instanceId, JSON.parse(requestJson)).then((items) => JSON.stringify(items)),
  };
}
//#endregion 🔖️JsBridge
