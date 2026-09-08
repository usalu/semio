// #region 🧲️Header
/** @emoji 🐚️ wgpu-web's plugin-loading + bridge-adapter pair — MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME
 * (`wgpu-web-shard`) replacement for the deleted `acquirePluginModule`/`pluginHandleForBridge` (both
 * removed from `@semio-tech/framework` when the kernel was ported — see `📓️terra-web-shard-*` reports)
 * and for `🟦️.ts`'s own retired `PluginWorkerClient` (one dedicated `Worker` per plugin, the
 * OLD synchronous-ish request/response ABI). `loadPluginModule` now drives a real actor through the
 * kernel's `ActivationRegistry` over `ShardClient` (bounded shard-worker pool, `actorId`-multiplexed) —
 * copying `PluginRuntime/🟦️.tsx`'s shape exactly, as this packet's brief requires, rather than
 * inventing a second worker-management scheme. `pluginHandleForBridge` then adapts the typed
 * {@link WgpuPluginHandle} down to the raw string-in/string-out JS surface
 * `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s `js_sys::Reflect::get(handle, "createApp"/"handleAction"/...)` still
 * expects on `wasm32` — that Rust file is outside this packet's lease (pure-TypeScript,
 * "do not wait on any Rust crate"), so this adapter preserves its existing contract rather than
 * changing it.
 *
 * Reuse decisions (see `📓️terra-wgpu-web-shard-report.md` for the full write-up):
 * - `ActivationRegistry`/`Effect`/`InvocationResponse`/`PluginManifest`/`SemioFaultError`/`TurnOutcome`/
 *   `createTurnOutcomeBroadcast` come from `@semio-tech/framework` (already a dependency — that
 *   package's `🟦️.ts` re-exports the whole kernel + manifest modules).
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
 *   wrapper (transactions/merge/conflicts/backbone/presence) — `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s
 *   `wasm32` branch only ever calls `manifest`/`createApp`/`destroyApp`/`handleAction`/`handleCommand`/
 *   `render`/`contextMenu`, so building the rest would be dead code for this target.
 * - Turn serialization: `PluginRuntime` runs a lane-prioritizing, coalescing `TurnScheduler` on top of
 *   `ShardClient.turn`. wgpu's own call pattern has no redraw-burst pressure (one winit-driven caller,
 *   not a pointer-move loop), so `submitTurn` below is a plain per-actor promise chain instead — enough
 *   to satisfy the shard worker's "never two turns in flight for one actor" rule without importing
 *   `TurnScheduler` for a guarantee this target doesn't need yet.
 * - `PluginWasmHandle.enqueue`/`.outcomes` (fire-and-forget + multicast reply stream) is
 *   `AppChannelClient`'s ONLY accepted handle shape as of channel v12/H1-react — its constructor takes
 *   `AppChannelHandle = Pick<PluginWasmHandle, "enqueue" | "outcomes">`, not the older synchronous
 *   request/response `exchange(instanceId, frames) -> Promise<frames>` this file was first ported
 *   against (that method no longer exists on `PluginWasmHandle` at all — `📌️important.md`'s "Replace,
 *   never wrap" list). `channelHandle` below builds exactly that `enqueue`/`outcomes` pair on top of
 *   `submitTurn`, one {@link createTurnOutcomeBroadcast} per `loadPluginModule` call, matching
 *   `PluginRuntime`'s own `handle`/`turnOutcomes` construction in its `loadPluginModule` line for line.
 *
 * Honest gap: `render` has no wire counterpart any more (channel v12 retired the per-verb
 * `render`/`renderWithDocument` command) — it is rebuilt here on top of a raw `"surface-visible"` turn
 * event + the retained-patch reconciliation `🖼️wire-turn.ts` provides, exactly mirroring
 * `PluginRuntime`'s own `refreshUi`. `windowEngagements`/`windowMeasures` are left unimplemented
 * (Rust's `🌉️ProgramBridge` already tolerates a missing function there with an empty-map fallback) —
 * `PluginRuntime` documents the identical gap ("no wire path yet, `dirty_render` only ever renders the
 * ONE window surface this wave").
 */
// #endregion 🧲️Header

// #region 🔌️Imports
import {
  ActivationRegistry,
  type ActivationReason,
  createTurnOutcomeBroadcast,
  fetchDescriptorManifest,
  type Effect,
  type InvocationResponse,
  type PluginManifest,
  type PluginWasmHandle as KernelPluginWasmHandle,
  SemioFaultError,
  type TurnOutcome,
} from "@semio-tech/framework";
import { AppChannelClient, AppChannelRequestSequence, decodeFaultFromWire, decodeInvocationResultPacks, decodePackValue, packValueToExactJson, decodePackWire, encodePackValue, faultDisplayMessage, packWireNatural } from "@semio-tech/framework-os";
import { createShardCommandIngressPages, ShardClient, type ShardCommandIngressPage, type ShardEventEnvelope } from "../../../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { createPooledActorRuntime, DEFAULT_SHARD_BUDGET, type PooledActorRuntime } from "../../../../../../../../../../🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts"
import { SHARD_WORKER_URL } from "../../../../../../../../../../🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts";
import type { ShardInstanceLifecycleLease, ShardWorkerLike } from "../../../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { rendererResidentLedger } from "../../../../../💾️resident/🟦️.ts";
import {
  applyUiPatchToRetained,
  coerceTurnResult,
  coerceWireBytes,
  decodeWirePatchOps,
  shellFrameBytes,
  wireEffectToFriendly,
  type RetainedSurface,
  type WireTurnResult,
  type WireUiPatch,
  type WireVariant,
} from "../../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts";
// #endregion 🔌️Imports

//#region 🧵️MainThreadShardWorkers
/** @emoji 🧵️ A shard worker that the UI ISOLATE owns, reached over a `MessagePort`.
 *
 * This frame worker cannot call `new Worker(...)` for a shard itself: a NESTED dedicated worker fails
 * to load outright in some embedded browsers — measured here, where even a one-line
 * `self.postMessage` worker failed with a message-less `error` event while the identical script loaded
 * fine from the page. That presented as all four shards dying at once and
 * `create_app promise failed: shard 0 terminated`, with no filename or line to point at.
 *
 * `ShardClient.createWorker` is synchronous, so this proxy is returned immediately and queues anything
 * sent before the port lands; the queue drains in order the moment it does. `terminate()` is relayed so
 * the UI isolate can drop the real worker, and a relayed `shard-worker-error` is re-raised on `onerror`
 * so `ShardClient`'s existing rebuild/failure ladder behaves exactly as with a direct `Worker`. */
class MainThreadShardWorker implements ShardWorkerLike {
  onmessage: ((event: { readonly data: unknown }) => void) | null = null;
  onerror: ((event: unknown) => void) | null = null;
  private port: MessagePort | null = null;
  private readonly queued: { readonly message: unknown; readonly transfer?: readonly Transferable[] }[] = [];
  private terminated = false;

  constructor(private readonly shardIndex: number) {
    shardPortWaiters.set(shardIndex, (port) => this.attach(port));
    (self as unknown as { postMessage(message: unknown): void }).postMessage({ kind: "shard-spawn", shardIndex, url: SHARD_WORKER_URL });
  }

  private attach(port: MessagePort): void {
    if (this.terminated) {
      port.close();
      return;
    }
    this.port = port;
    port.onmessage = (event: MessageEvent) => {
      const data = event.data as { readonly kind?: unknown; readonly message?: unknown } | null;
      if (data && typeof data === "object" && data.kind === "shard-worker-error") {
        this.onerror?.(data);
        return;
      }
      this.onmessage?.({ data: event.data });
    };
    port.start();
    for (const entry of this.queued.splice(0)) port.postMessage(entry.message, (entry.transfer ?? []) as Transferable[]);
  }

  postMessage(message: unknown, transfer?: readonly Transferable[]): void {
    if (this.terminated) return;
    if (!this.port) {
      this.queued.push({ message, transfer });
      return;
    }
    this.port.postMessage(message, (transfer ?? []) as Transferable[]);
  }

  terminate(): void {
    if (this.terminated) return;
    this.terminated = true;
    shardPortWaiters.delete(this.shardIndex);
    this.port?.close();
    this.port = null;
    this.queued.length = 0;
    (self as unknown as { postMessage(message: unknown): void }).postMessage({ kind: "shard-terminate", shardIndex: this.shardIndex });
  }
}

const shardPortWaiters = new Map<number, (port: MessagePort) => void>();
self.addEventListener("message", (event: MessageEvent) => {
  const data = event.data as { readonly kind?: unknown; readonly shardIndex?: unknown; readonly port?: unknown } | null;
  if (!data || typeof data !== "object" || data.kind !== "shard-port" || typeof data.shardIndex !== "number") return;
  shardPortWaiters.get(data.shardIndex)?.(data.port as MessagePort);
  shardPortWaiters.delete(data.shardIndex);
});
//#endregion 🧵️MainThreadShardWorkers

//#region 🔖️PooledSingletons
let pooledRuntime: PooledActorRuntime | null = null;
function getShardClient(): ShardClient {
  pooledRuntime ??= createPooledActorRuntime({
    createWorker: (shardIndex: number) => new MainThreadShardWorker(shardIndex),
    residentLedger: rendererResidentLedger(),
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
function submitTurn(actorId: string, events: readonly ShardEventEnvelope[], commandPage?: ShardCommandIngressPage): Promise<WireTurnResult> {
  getActivationRegistry().touch(actorId);
  const previousSettled = (actorTurnChains.get(actorId) ?? Promise.resolve()).catch(() => undefined);
  const next = previousSettled.then(() => getShardClient().turn(actorId, events, DEFAULT_SHARD_BUDGET, commandPage));
  actorTurnChains.set(actorId, next);
  return next.then(coerceTurnResult);
}
//#endregion 🔖️TurnSubmit

//#region 🚪️InstanceLifecycleSettle
/** @emoji 🚪️ Drives one instance's open handshake to LIVE. `open()` only POSTS the request — the guest
 * answers with a lifecycle receipt that the host must acknowledge before the instance is registered, and
 * `⚛️reactor/🔄️turn/🦀️.rs`'s dirty-surface loop `continue`s past any instance whose
 * `native_close_key` is not registered. Skipping the ACK therefore renders NOTHING, silently: measured as
 * 257 turns producing zero patches and zero effects for `puzzle3d.play.composite`, with no fault raised.
 * Bounded by the same opportunity ceiling as the document drain. */
async function settleInstanceLifecycle(lifecycle: ShardInstanceLifecycleLease): Promise<void> {
  for (let opportunity = 0; opportunity < RETAINED_DOCUMENT_OPPORTUNITIES; opportunity += 1) {
    const receipt = lifecycle.pendingReceipt;
    if (receipt !== null) {
      await lifecycle.acknowledge(receipt, DEFAULT_SHARD_BUDGET);
      continue;
    }
    const phase = lifecycle.progress().kind;
    if (phase !== "opening" && phase !== "captured") return;
    await lifecycle.poll(DEFAULT_SHARD_BUDGET);
  }
  console.warn(`[DEBUG] plugin-bridge: instance lifecycle never reached open within ${RETAINED_DOCUMENT_OPPORTUNITIES} opportunities`);
}

const lifecycleByInstance = new Map<number, ShardInstanceLifecycleLease>();
//#endregion 🚪️InstanceLifecycleSettle

//#region 🔖️RetainedWindow
const retainedWindowByActor = new Map<string, RetainedSurface>();

/** 🖼️ One wire `UiPatch` reconciled onto `previous`: every `pack`-typed op payload is projected through
 * {@link decodePackWire} and the two WIT `u64` revisions narrowed through {@link packWireNatural}, so a
 * lossless integer carrier never reaches the retained tree as a `{kind, value}` object and a `bigint`
 * revision never fails the `baseRevision` identity check as a permanent desync. */
export function reconcileRetainedWindowPatch(previous: RetainedSurface | null, patch: WireUiPatch): { readonly surface: RetainedSurface | null; readonly desynced: boolean } {
  const ops = decodeWirePatchOps(patch.ops ?? [], decodePackWire);
  return applyUiPatchToRetained(previous, { revision: packWireNatural(patch.revision, "uiPatch.revision"), baseRevision: packWireNatural(patch.baseRevision, "uiPatch.baseRevision"), ops });
}

function applyRetainedWindowPatches(actorId: string, uiPatches: readonly WireUiPatch[]): void {
  for (const patch of uiPatches) {
    const previous = retainedWindowByActor.get(actorId) ?? null;
    const { surface, desynced } = reconcileRetainedWindowPatch(previous, patch);
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
 * (`⚛️reactor/🦀️.rs`'s `dirty_render` loop hardcodes it) — `bodyKey`/`viewState` are accepted
 * for ABI compatibility but not yet threaded through, matching `PluginRuntime`'s own identical gap. */
async function performRender(actorId: string, instanceId: number, bodyKey: string): Promise<unknown> {
  const result = await submitTurn(actorId, [{ kind: "surface-visible", payload: { surface: { instance: instanceId, surface: bodyKey } } }]);
  if (result.uiPatches.length > 0) applyRetainedWindowPatches(actorId, result.uiPatches);
  return retainedWindowByActor.get(actorId)?.node ?? null;
}

/** @emoji 📃️ Publishes ONE surface's retained document as the `UiSnapshot`-shaped JSON the wgpu
 * renderer's `render_with_document_js` assembles into a `UiDocumentLease`.
 *
 * Reads the RAW `ui-patch` ops (`🧬️schema/📜️.wit`'s `patch-op`: `upsert` carries a pack-encoded
 * `contract-doc::UiNodeRecord`, `set-root` carries the root `node-id`) rather than the reconciled JS tree,
 * because the Rust side wants the flat node table `UiSnapshot` declares — and because `decodeWirePatchOps`
 * still speaks the older `replace`/`insert-child` vocabulary this variant no longer emits.
 *
 * Every decoded record is projected with `packValueToExactJson`: `decodePackValue` yields `{kind,value}`
 * integer carriers, which serialize as `[object Object]` where `UiNodeId`/`UiRevision` expect numbers. */
/** @emoji 🔁️ Bounded drain for one retained surface: a guest answers `surface-visible` with
 * `status: more-work` and publishes its `ui-patch` a turn or more later, so a single turn reads empty
 * (measured — `puzzle3d.play.composite` returns zero ops on the visibility turn itself). Mirrors the
 * native `render_with_document`'s own opportunity ceiling: keep stepping until the surface publishes,
 * and fail closed rather than spin. `WireTurnResult` carries no `status`, so the ceiling — not a guest
 * flag — is what bounds this. */
const RETAINED_DOCUMENT_OPPORTUNITIES = 256;

async function publishRetainedDocument(actorId: string, instanceId: number, bodyKey: string): Promise<string> {
  const collected: WireUiPatch[] = [];
  const seenSurfaces: string[] = [];
  const seenTags: string[] = [];
  let anyPatches = 0;
  let anyEffects = 0;
  const seenEffects: string[] = [];
  let turns = 1;
  let result = await submitTurn(actorId, [{ kind: "surface-visible", payload: { surface: { instance: instanceId, surface: bodyKey } } }]);
  for (let opportunity = 0; opportunity < RETAINED_DOCUMENT_OPPORTUNITIES; opportunity += 1) {
    anyPatches += result.uiPatches.length;
    anyEffects += result.effects.length;
    for (const effect of result.effects) if (typeof effect.tag === "string" && !seenEffects.includes(effect.tag)) seenEffects.push(effect.tag);
    for (const patch of result.uiPatches) {
      const name = patch.surface?.surface ?? "(none)";
      if (!seenSurfaces.includes(name)) seenSurfaces.push(name);
      for (const op of patch.ops ?? []) if (typeof op.tag === "string" && !seenTags.includes(op.tag)) seenTags.push(op.tag);
    }
    if (result.uiPatches.length > 0) {
      applyRetainedWindowPatches(actorId, result.uiPatches);
      collected.push(...result.uiPatches.filter((patch) => (patch.surface?.surface ?? bodyKey) === bodyKey));
    }
    if (collected.length > 0) break;
    result = await submitTurn(actorId, []);
    turns += 1;
  }
  console.log(`[DEBUG] publishRetainedDocument ${bodyKey}: turns=${turns} collected=${collected.length} anyPatches=${anyPatches} surfaces=${JSON.stringify(seenSurfaces)} tags=${JSON.stringify(seenTags.slice(0, 12))} effects=${anyEffects} effectTags=${JSON.stringify(seenEffects.slice(0, 12))}`);
  const nodes: unknown[] = [];
  let revision = 0;
  let root: number | null = null;
  for (const patch of collected) {
    if (typeof patch.revision === "number") revision = patch.revision;
    for (const op of patch.ops ?? []) {
      const value = (op.val ?? {}) as Record<string, unknown>;
      if (op.tag === "upsert") {
        nodes.push(packValueToExactJson(decodePackValue(coerceWireBytes(value.node)) as Parameters<typeof packValueToExactJson>[0]));
        continue;
      }
      if (op.tag === "set-root") root = Number(typeof op.val === "object" && op.val !== null ? (value.id ?? 0) : op.val);
    }
  }
  if (root === null && nodes.length > 0) root = Number((nodes[0] as { readonly id?: unknown }).id ?? 0);
  return JSON.stringify({ surface: bodyKey, revision, root, nodes, layoutEpoch: 0 });
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

/** 📥️ Projects the four `pack`-typed payloads of one `Invocation` reply frame onto exact JSON. The wire
 * codec returns lossless integer carriers, so a raw decode would hand the shell `{kind, value}` objects
 * wherever a plugin returned a `u64` — the `render`/`handleAction` output, the diagnostics list, the UI
 * scope and the history patch all cross this one boundary. */
export function decodeInvocationPayloads(frame: { readonly output: ArrayLike<number>; readonly diagnostics: ArrayLike<number>; readonly ui_scope: ArrayLike<number>; readonly history_patch: ArrayLike<number>; readonly mutations: ArrayLike<number>; readonly inverse_group: ArrayLike<number> }): Pick<InvocationResponse, "output" | "diagnostics" | "uiScope" | "historyPatch" | "mutations" | "inverseGroup"> {
  const diagnostics = decodePackWire(new Uint8Array(frame.diagnostics), "invocation.diagnostics");
  const historyPatch = decodePackWire(new Uint8Array(frame.history_patch), "invocation.historyPatch");
  return {
    ...decodeInvocationResultPacks(frame),
    output: decodePackWire(new Uint8Array(frame.output), "invocation.output"),
    diagnostics: Array.isArray(diagnostics) ? (diagnostics as InvocationResponse["diagnostics"]) : [],
    uiScope: decodePackWire(new Uint8Array(frame.ui_scope), "invocation.uiScope") as InvocationResponse["uiScope"],
    historyPatch: historyPatch && typeof historyPatch === "object" ? (historyPatch as InvocationResponse["historyPatch"]) : undefined,
  };
}

async function performInvocation(client: AppChannelClient, instanceId: number, invocation: unknown, viewState: unknown): Promise<InvocationResponse> {
  const frames = await client.command(encodePackValue(invocation), viewState);
  let output: unknown = null;
  let diagnostics: InvocationResponse["diagnostics"] = [];
  let uiScope: InvocationResponse["uiScope"];
  let historyPatch: InvocationResponse["historyPatch"];
  let mutations: InvocationResponse["mutations"] = [];
  let inverseGroup: InvocationResponse["inverseGroup"] = { invocationId: "", mutations: [], inverseMutations: [] };
  for (const frame of frames) {
    if ("Invocation" in frame) {
      ({ output, diagnostics, uiScope, historyPatch, mutations, inverseGroup } = decodeInvocationPayloads(frame.Invocation));
    } else if ("Error" in frame) {
      const fault = decodeFaultFromWire(frame.Error.fault, decodePackValue);
      if (fault) throw new SemioFaultError(fault);
      throw new Error(`invocation failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
    }
  }
  const leftover = pendingTurnEffects.get(instanceId) ?? [];
  pendingTurnEffects.delete(instanceId);
  const requestedEffects = leftover.map((effect) => wireEffectToFriendly(effect, decodePackWire)).filter((effect): effect is Effect => effect !== null);
  return { output, mutations, inverseGroup, diagnostics, requestedEffects, events: [], uiScope, historyPatch };
}
//#endregion 🔖️Invocation

//#region 🔖️WgpuPluginHandle
/** 🐚️ The typed handle this file hands to a `bootFrameworkOsWgpu`/`🟦️.ts` caller — narrower than
 * `PluginRuntime`'s wide `PluginWasmHandle` (no transactions/merge/conflicts/backbone/presence): only
 * the surface `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s `wasm32` branch actually calls. */
export interface WgpuPluginHandle {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly handleAction: (instanceId: number, actionJson: string, viewState: unknown) => Promise<InvocationResponse>;
  readonly handleCommand: (instanceId: number, commandJson: string, viewState: unknown) => Promise<InvocationResponse>;
  readonly render: (instanceId: number, bodyKey: string, viewState: unknown) => Promise<unknown>;
  readonly renderDocument: (instanceId: number, bodyKey: string) => Promise<string>;
  readonly contextMenu: (instanceId: number, request: unknown) => Promise<unknown>;
  readonly dispose: () => void;
}

/** 🐚️ Acquires a real actor through `ActivationRegistry`/`ShardClient` (replacing the deleted
 * `acquirePluginModule`/per-plugin `Worker` — design-runtime.md §3, copying `PluginRuntime`'s own
 * `loadPluginModule` shape). `dispose()` disposes every instance's worker-side actor entry via
 * `ShardClient.dispose` — no shared module lease to refcount any more, one actor belongs to exactly
 * one instance. */
export async function loadPluginModule(pluginId: string, moduleUrl: string, signal?: AbortSignal): Promise<WgpuPluginHandle> {
  const manifest = await fetchDescriptorManifest(pluginId, moduleUrl, signal);
  const registry = getActivationRegistry();
  registry.registerManifest({ pluginId, moduleUrl, caps: [] });
  const shardClient = getShardClient();
  const actorIdByInstance = new Map<number, string>();
  const channelByInstance = new Map<number, AppChannelClient>();
  const channelRequests = new AppChannelRequestSequence();
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

  /** 📤️📥️ Backs `channelHandle.enqueue`/`.outcomes` below — one broadcast per `loadPluginModule` call,
   * matching the handle's own lifetime: every instance this call's `createApp` ever opens shares it,
   * and each instance's `AppChannelClient` filters to its own `instanceId` (`pumpOutcomes`'s own doc in
   * `💻️os/🟦️.ts`). Mirrors `PluginRuntime`'s own `turnOutcomes`/`loadPluginModule` exactly. */
  const turnOutcomes = createTurnOutcomeBroadcast<TurnOutcome>();

  /** 🔀️ Frames every `AppCommand`/`AppFrame` `AppChannelClient` sends through — one `"app-command"`
   * shard event per batched frame, demuxing the resulting turn's `Effect::SendMessage{Shell}` replies
   * back into frames and stashing everything else as this instance's leftover effects
   * (`performInvocation` drains them) — pushed onto {@link turnOutcomes} instead of returned, since
   * `PluginWasmHandle.enqueue` is fire-and-forget (channel v12/H1-react retired the old synchronous
   * `exchange(instanceId, frames) -> Promise<frames>` RPC shape this file was first ported against). A
   * turn-submission failure becomes an `error`-shaped outcome rather than an uncaught rejection, since
   * nothing here awaits this function's own promise. Mirrors `PluginRuntime`'s own `runQueuedTurn`. */
  const runQueuedTurn = async (instanceId: number, events: readonly Uint8Array[]): Promise<void> => {
    try {
      const actorId = requireActorId(instanceId);
      const results: WireTurnResult[] = [];
      for (let commandIndex = 0; commandIndex < events.length; commandIndex += 1) {
        eventSeq += 1;
        const pages = createShardCommandIngressPages({
          owner: BigInt(instanceId),
          generation: 1n,
          commandIndex,
          commandCount: events.length,
          instance: instanceId,
          seq: BigInt(eventSeq),
          command: events[commandIndex]!,
        });
        for (const commandPage of pages) results.push(await submitTurn(actorId, [], commandPage));
        let terminal = results.at(-1)?.commandIngress?.tag;
        for (let continuation = 0; terminal !== "command-complete" && continuation < 1_024; continuation += 1) {
          if (terminal === "fault") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress fault`);
          if (terminal === "backpressure") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress backpressure after serialized submission`);
          const continued = await submitTurn(actorId, []);
          results.push(continued);
          terminal = continued.commandIngress?.tag;
        }
        if (terminal !== "command-complete") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress did not complete within 1024 continuations`);
      }
      const result: WireTurnResult = {
        uiPatches: results.flatMap((turn) => turn.uiPatches),
        effects: results.flatMap((turn) => turn.effects),
        nextWake: [...results].reverse().find((turn) => turn.nextWake !== null)?.nextWake ?? null,
        commandIngress: results.at(-1)?.commandIngress,
      };
      const outFrames: Uint8Array[] = [];
      const leftover: WireVariant[] = [];
      for (const effect of result.effects) {
        const frame = shellFrameBytes(effect, instanceId);
        if (frame) outFrames.push(frame);
        else leftover.push(effect);
      }
      pendingTurnEffects.set(instanceId, leftover);
      if (result.uiPatches.length > 0) applyRetainedWindowPatches(actorId, result.uiPatches);
      turnOutcomes.push({ instanceId, frames: outFrames });
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    }
  };

  const channelHandle: Pick<KernelPluginWasmHandle, "enqueue" | "outcomes"> = {
    enqueue: (instanceId, events) => {
      void runQueuedTurn(instanceId, events);
    },
    outcomes: turnOutcomes.stream,
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
      const lifecycle = getShardClient().captureInstanceLifecycle(actorId, instanceId);
      await lifecycle.open({ appId, actor: "local", config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})) }, DEFAULT_SHARD_BUDGET);
      await settleInstanceLifecycle(lifecycle);
      lifecycleByInstance.set(instanceId, lifecycle);
      channelByInstance.set(instanceId, new AppChannelClient(channelHandle, channelRequests, instanceId, appId, "local"));
      return instanceId;
    },
    destroyApp: async (instanceId) => {
      const actorId = actorIdByInstance.get(instanceId);
      if (!actorId) return;
      // 🔌️ Ends this instance's channel's own outcome subscription BEFORE dropping it — otherwise it
      // leaks a live subscriber against `turnOutcomes` for the rest of this `loadPluginModule` call's
      // lifetime (`AppChannelClient.dispose`'s own doc; mirrors `PluginRuntime`'s `adaptPluginHandle`).
      channelByInstance.get(instanceId)?.dispose();
      actorIdByInstance.delete(instanceId);
      channelByInstance.delete(instanceId);
      retainedWindowByActor.delete(actorId);
      pendingTurnEffects.delete(instanceId);
      shardClient.dispose(actorId);
    },
    handleAction: (instanceId, actionJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(actionJson), viewState),
    handleCommand: (instanceId, commandJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(commandJson), viewState),
    render: (instanceId, bodyKey) => performRender(requireActorId(instanceId), instanceId, bodyKey),
    renderDocument: (instanceId, bodyKey) => publishRetainedDocument(requireActorId(instanceId), instanceId, bodyKey),
    contextMenu: (instanceId, request) => requireChannel(instanceId).contextMenu(request),
    dispose: () => {
      for (const instanceId of channelByInstance.keys()) channelByInstance.get(instanceId)?.dispose();
      for (const actorId of actorIdByInstance.values()) {
        retainedWindowByActor.delete(actorId);
        shardClient.dispose(actorId);
      }
      actorIdByInstance.clear();
      channelByInstance.clear();
      turnOutcomes.complete();
    },
  };
}
//#endregion 🔖️WgpuPluginHandle

//#region 🔖️JsBridge
/** 🌉️ The raw string-in/string-out JS surface `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s `wasm32` branch reads
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
  readonly renderDocument: (instanceId: number, bodyKey: string) => Promise<string>;
  readonly contextMenu: (instanceId: number, requestJson: string) => Promise<string>;
}

/** 📥️ `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s `handle_action_js`/`handle_command_js` pass a THIRD argument
 * that is `{"viewState": ..., "actor": "local"}` JSON (its own `context_json`, not the bare view
 * state) — this unwraps `.viewState` from it. The pre-rewrite `🟦️.ts` fed that whole context
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
    renderDocument: (instanceId, bodyKey) => handle.renderDocument(instanceId, bodyKey),
    contextMenu: (instanceId, requestJson) => handle.contextMenu(instanceId, JSON.parse(requestJson)).then((items) => JSON.stringify(items)),
  };
}
//#endregion 🔖️JsBridge
