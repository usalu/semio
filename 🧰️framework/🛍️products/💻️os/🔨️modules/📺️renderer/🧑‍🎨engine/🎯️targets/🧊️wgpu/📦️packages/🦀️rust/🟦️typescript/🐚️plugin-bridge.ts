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
 * `PluginRuntime`'s own `refreshUi`. `windowEngagements`/`windowMeasures` are left unimplemented;
 * Rust's `🌉️ProgramBridge` already treats their absence as an empty-map result.
 */
// #endregion 🧲️Header

// #region 🔌️Imports
import {
  ActivationRegistry,
  type ActivationReason,
  createTurnOutcomeBroadcast,
  fetchDescriptorManifest,
  type BuiltNode,
  type Component,
  type Effect,
  type InvocationResponse,
  type PluginManifest,
  type PluginWasmHandle as KernelPluginWasmHandle,
  SemioFaultError,
  type TurnOutcome,
} from "@semio-tech/framework";
import { AppChannelClient, AppChannelRequestSequence, type WindowConfigPackEntry, decodeFaultFromWire, decodeInvocationResultPacks, decodePackValue, decodePackWire, encodePackValue, faultDisplayMessage, packWireNatural } from "@semio-tech/framework-os";
import { createShardCommandIngressPages, settleFailedInstanceOpen, ShardClient, type ShardCommandIngressPage, type ShardEventEnvelope } from "../../../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { createPooledActorRuntime, DEFAULT_SHARD_BUDGET, type PooledActorRuntime } from "../../../../../../../../../../🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts"
import { SHARD_WORKER_URL } from "../../../../../../../../../../🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts";
import type { ShardInstanceLifecycleLease, ShardWorkerLike } from "../../../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { rendererResidentLedger } from "../../../../../💾️resident/🟦️.ts";
import { DEFAULT_UI_DOCUMENT_LIMITS } from "../../../../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import { OwnedUiPatchIntake, RETAINED_UI_INTAKE_SLICE_STEPS, retainedUiIntakeStepCeiling } from "../../../../../🧱️elements/📃️UiDocumentStore/📥️intake/🟦️.ts";
import { OwnedUiInstance, type OwnedUiInstanceRetirement, type OwnedUiInstanceSurface, type OwnedUiPatchAcknowledgement } from "../../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import type { RetainedUiNodeRecord } from "../../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts";
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
    routeShardPorts();
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
let shardPortsRouted = false;

/** @emoji 📬️ Arms the `shard-port` handoff route the UI isolate answers a `shard-spawn` with, on the
 * FIRST shard this isolate spawns rather than at module scope. The worker global only exists in the
 * isolate that spawns shards, so a module-scope `self.addEventListener` made merely IMPORTING this
 * bridge throw `ReferenceError: self is not defined` in every other host — which took the package's own
 * node test suite down at import, before a single test ran. */
function routeShardPorts(): void {
  if (shardPortsRouted) return;
  shardPortsRouted = true;
  self.addEventListener("message", (event: MessageEvent) => {
    const data = event.data as { readonly kind?: unknown; readonly shardIndex?: unknown; readonly port?: unknown } | null;
    if (!data || typeof data !== "object" || data.kind !== "shard-port" || typeof data.shardIndex !== "number") return;
    shardPortWaiters.get(data.shardIndex)?.(data.port as MessagePort);
    shardPortWaiters.delete(data.shardIndex);
  });
}
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
function submitActorWork<T>(actorId: string, work: () => Promise<T>): Promise<T> {
  getActivationRegistry().touch(actorId);
  const previousSettled = (actorTurnChains.get(actorId) ?? Promise.resolve()).catch(() => undefined);
  const next = previousSettled.then(work);
  actorTurnChains.set(actorId, next);
  return next;
}

function submitTurn(actorId: string, events: readonly ShardEventEnvelope[], commandPage?: ShardCommandIngressPage): Promise<WireTurnResult> {
  return submitActorWork(actorId, () => getShardClient().turn(actorId, events, DEFAULT_SHARD_BUDGET, commandPage)).then(coerceTurnResult);
}
//#endregion 🔖️TurnSubmit

//#region 🔖️OwnedUiRoute
const RETAINED_DOCUMENT_OPPORTUNITIES = 256;
const WGPU_UI_CONTINUATION_BATCH_SIZE = 8;
const WGPU_UI_GRANT = Object.freeze({ maxItems: 1, maxBytes: 4_096 });

/** 📏️ The whole-document liveness backstop, shared with the React target and declared
 * language-agnostically in `🧵️retained/🧫️fixtures/📥️intake/🔣️.json` — NOT a per-drive cap. The fixed
 * `4_096` that stood here priced every intake as if it were a few KiB: the intake advances ONE phase
 * per `advance(grant)` (a LEB128 byte, a text body, an attach), so the grant never buys more phases,
 * and generation3d's first document (flow window scene + preview + catalogue pages + measures) blew
 * through it at `shell-boot` as `wgpu-ui.intake-budget-exhausted`. */
export const WGPU_UI_INTAKE_STEP_CEILING = retainedUiIntakeStepCeiling(DEFAULT_UI_DOCUMENT_LIMITS);
type WgpuActorExecutor = <T>(work: () => Promise<T>) => Promise<T>;
type WgpuOwnedUiProjection = Readonly<{ node: BuiltNode; document: Readonly<{ surface: string; revision: number; root: number; nodes: readonly object[]; layoutEpoch: number }> }>;

async function yieldWgpuUi(step: number): Promise<void> {
  if (step % WGPU_UI_CONTINUATION_BATCH_SIZE === 0) await new Promise<void>((resolve) => setTimeout(resolve, 0));
}

/** 🎞️ Hands the frame back. `requestAnimationFrame` where the target has one (the worker's
 * `OffscreenCanvas` context does), a macrotask otherwise, so a headless test resumes too. */
async function nextWgpuFrame(): Promise<void> {
  const frame = (globalThis as { requestAnimationFrame?: (callback: () => void) => unknown }).requestAnimationFrame;
  if (typeof frame === "function") await new Promise<void>((resolve) => frame.call(globalThis, () => resolve()));
  else await new Promise<void>((resolve) => setTimeout(resolve, 0));
}

/** 🎞️ One RESUMABLE drive cursor for a single retained intake. Exhausting a slice
 * ({@link RETAINED_UI_INTAKE_SLICE_STEPS}) is a yield, not a fault: the cursor is retained across the
 * frame boundary and the very same intake continues where it stopped, so a document larger than one
 * slice publishes over several frames. Only {@link WGPU_UI_INTAKE_STEP_CEILING} — the whole-document
 * backstop — is terminal, and a genuine stall is caught earlier and more precisely by the intake's own
 * 32-consecutive-zero-progress rejection. */
export class WgpuUiIntakeCursor {
  #steps = 0;
  readonly #ceiling: number;
  constructor(ceiling: number = WGPU_UI_INTAKE_STEP_CEILING) { this.#ceiling = ceiling; }
  get steps(): number { return this.#steps; }
  async next(phase: string): Promise<void> {
    this.#steps += 1;
    if (this.#steps > this.#ceiling) throw new Error(`wgpu-ui.intake-budget-exhausted:${phase}:${this.#steps}`);
    if (this.#steps % RETAINED_UI_INTAKE_SLICE_STEPS === 0) await nextWgpuFrame();
    else await yieldWgpuUi(this.#steps);
  }
}

function ownedUiComponentToBuilt(component: RetainedUiNodeRecord["component"]): Component {
  if (component.type !== "surface") return component;
  return { ...component, doc: { bytes: Array.from({ length: component.doc.bytes.length }, (_, index) => component.doc.bytes.byteAt(index)) } };
}

/** 🏠️ Exact WGPU host owner for one captured guest lifetime. Patch intake, render reads and
 * terminal retirement all traverse this same owner; no parallel retained tree can outlive its witness. */
export class WgpuOwnedUiInstanceRoute {
  readonly lifecycle: ShardInstanceLifecycleLease;
  readonly owner: OwnedUiInstance;
  readonly #surfaces = new Map<string, OwnedUiInstanceSurface>();
  readonly #intakes = new Set<OwnedUiPatchIntake>();
  readonly #reads = new Set<Promise<unknown>>();
  #retirement: OwnedUiInstanceRetirement | null = null;
  #closing = false;

  constructor(lifecycle: ShardInstanceLifecycleLease) {
    const lifetime = lifecycle.lifetime;
    if (!lifetime) throw new Error("wgpu-ui.native-lifetime-required");
    this.lifecycle = lifecycle;
    this.owner = new OwnedUiInstance(lifecycle.activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 32 });
    lifecycle.bindHostRetirement(this.owner);
  }

  get hasPendingReads(): boolean { return this.#reads.size > 0; }
  get terminalIsEmpty(): boolean { return this.owner.terminalIsEmpty() && this.#surfaces.size === 0 && this.#intakes.size === 0 && this.#reads.size === 0; }
  hasSurface(name: string): boolean { return this.#surfaces.has(name); }

  async #advanceMaintenance(phase: string, budget: { steps: number }): Promise<void> {
    while (this.owner.maintenancePending) {
      if (++budget.steps > DEFAULT_UI_DOCUMENT_LIMITS.maxNodes * 64) throw new Error(`wgpu-ui.${phase}-budget-exhausted`);
      const current = this.owner.advanceMaintenance(WGPU_UI_GRANT);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`wgpu-ui.${phase}-${current.kind}:${current.phase}`);
      await yieldWgpuUi(budget.steps);
    }
  }

  async #closeIntake(intake: OwnedUiPatchIntake): Promise<void> {
    intake.beginClose();
    const cursor = new WgpuUiIntakeCursor();
    while (!intake.terminalIsEmpty()) {
      const current = intake.closeStep(WGPU_UI_GRANT);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`wgpu-ui.intake-close-${current.kind}:${current.phase}`);
      await cursor.next("intake-close");
    }
    this.#intakes.delete(intake);
  }

  async accept(turn: WireTurnResult, execute: WgpuActorExecutor): Promise<readonly WireTurnResult[]> {
    if (this.#closing) throw new Error("wgpu-ui.owner-closing");
    if (turn.uiPatches.length === 0) {
      if (turn.uiPatchReceipt !== undefined) throw new Error("wgpu-ui.receipt-without-patch");
      return [];
    }
    if (!turn.original || !turn.uiPatchReceipt) throw new Error("wgpu-ui.native-owner-required");
    const supplemental: WireTurnResult[] = [];
    for (const [index, patch] of turn.uiPatches.entries()) {
      const surfaceId = patch.surface?.surface;
      if (!surfaceId) throw new Error("wgpu-ui.projection-surface-required");
      const source = this.lifecycle.captureUiPatchAuthority(turn.original, index);
      const intake = new OwnedUiPatchIntake(this.owner, source);
      this.#intakes.add(intake);
      const cursor = new WgpuUiIntakeCursor();
      let token: OwnedUiPatchAcknowledgement | null = null;
      while (token === null) {
        const current = intake.advance(WGPU_UI_GRANT);
        token = intake.peekAcknowledgement();
        if (current.kind === "rejected") throw new Error(`wgpu-ui.intake-rejected:${current.phase}:${intake.failure ?? "unknown"}`);
        if (current.kind === "blocked" && token === null) throw new Error(`wgpu-ui.intake-blocked:${current.phase}`);
        await cursor.next("intake");
      }
      const acknowledged = await execute(() => this.lifecycle.submitUiAcknowledgement(source, token, DEFAULT_SHARD_BUDGET));
      if (!intake.acceptAcknowledgement(acknowledged.receipt)) throw new Error("wgpu-ui.acknowledgement-refused");
      for (;;) {
        const current = intake.advance(WGPU_UI_GRANT);
        if (current.kind === "ready") break;
        if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`wgpu-ui.intake-${current.kind}:${current.phase}`);
        await cursor.next("publication-close");
      }
      const surface = intake.takeSurface();
      if (!surface) throw new Error("wgpu-ui.surface-missing");
      this.#surfaces.set(surfaceId, surface);
      await this.#closeIntake(intake);
      const next = coerceTurnResult(acknowledged.result);
      supplemental.push(next, ...await this.accept(next, execute));
    }
    return supplemental;
  }

  async project(surfaceId: string): Promise<WgpuOwnedUiProjection | null> {
    if (this.#closing) throw new Error("wgpu-ui.owner-closing");
    const surface = this.#surfaces.get(surfaceId);
    if (!surface) return null;
    const read = this.#project(surfaceId, surface);
    this.#reads.add(read);
    try { return await read; } finally { this.#reads.delete(read); }
  }

  async #project(surfaceId: string, surface: OwnedUiInstanceSurface): Promise<WgpuOwnedUiProjection | null> {
    const view = surface.view;
    if (view.root === null) return null;
    if (!view.hash) throw new Error("wgpu-ui.surface-hash-required");
    const visited = new Set<number>();
    const records: object[] = [];
    const budget = { steps: 0 };
    const build = async (id: number, depth: number): Promise<BuiltNode> => {
      if (this.#closing || this.#surfaces.get(surfaceId) !== surface) throw new Error("wgpu-ui.read-stale");
      if (depth > DEFAULT_UI_DOCUMENT_LIMITS.maxDepth || visited.size >= DEFAULT_UI_DOCUMENT_LIMITS.maxNodes || visited.has(id)) throw new Error("wgpu-ui.read-graph-invalid");
      visited.add(id);
      const subscription = surface.subscribeNode(id, () => {});
      let record: RetainedUiNodeRecord | null = null;
      try {
        await this.#advanceMaintenance("read", budget);
        const snapshot = subscription.snapshot;
        if (!snapshot || snapshot.version !== view.revision || !snapshot.record) throw new Error("wgpu-ui.read-snapshot-missing");
        record = snapshot.record;
        surface.acknowledgeRead(subscription, snapshot);
      } finally {
        surface.unsubscribeNode(subscription);
        await this.#advanceMaintenance("read-retirement", budget);
      }
      if (!record) throw new Error("wgpu-ui.read-snapshot-missing");
      const component = ownedUiComponentToBuilt(record.component);
      records.push({ ...record, component });
      const children: BuiltNode[] = [];
      for (const child of record.children) children.push(await build(child, depth + 1));
      return { key: record.key, component, layout: record.layout, style: record.style, activity: record.activity, disabled: record.disabled, accessibility: record.accessibility, bindings: record.bindings, menu: record.menu, children };
    };
    const node = await build(view.root, 1);
    if (this.#closing || this.#surfaces.get(surfaceId) !== surface || surface.view !== view) throw new Error("wgpu-ui.read-stale");
    return { node, document: { surface: surfaceId, revision: view.revision, root: view.root, nodes: records, layoutEpoch: 0 } };
  }

  async retire(): Promise<OwnedUiInstanceRetirement> {
    if (this.#retirement) return this.#retirement;
    this.#closing = true;
    await Promise.allSettled([...this.#reads]);
    for (const intake of [...this.#intakes]) await this.#closeIntake(intake);
    this.#surfaces.clear();
    this.owner.beginClose();
    const cursor = new WgpuUiIntakeCursor();
    while (!this.owner.terminalIsEmpty()) {
      const current = this.owner.closeStep(WGPU_UI_GRANT);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`wgpu-ui.owner-close-${current.kind}:${current.phase}`);
      await cursor.next("owner-close");
    }
    const witness = this.owner.takeRetirementWitness();
    if (!witness) throw new Error("wgpu-ui.retirement-witness-missing");
    this.#retirement = witness;
    return witness;
  }
}

/** 🧯 Re-exported from its one owner (`📮️shard-client/🟦️.ts`) so this target and the React runtime
 * settle a failed open the same way: run the cleanup, report a cleanup fault on its own line, reject
 * with the ORIGINAL cause. */
export { settleFailedInstanceOpen };

/** 🚪️ Opens one captured lifecycle only after binding its exact host UI retirement owner. */
async function settleInstanceLifecycle(lifecycle: ShardInstanceLifecycleLease, route: WgpuOwnedUiInstanceRoute, initial: WireTurnResult, execute: WgpuActorExecutor): Promise<void> {
  let current = initial;
  for (let opportunity = 0; opportunity < RETAINED_DOCUMENT_OPPORTUNITIES; opportunity += 1) {
    await route.accept(current, execute);
    const receipt = lifecycle.pendingReceipt;
    if (receipt !== null) {
      if (receipt.kind !== "captured") throw new Error("wgpu-ui.open-receipt-kind");
      current = coerceTurnResult(await execute(() => lifecycle.acknowledge(receipt, DEFAULT_SHARD_BUDGET)));
      continue;
    }
    const phase = lifecycle.progress().kind;
    if (phase === "open") return;
    if (phase !== "opening" && phase !== "captured") throw new Error(`wgpu-ui.open-${phase}`);
    current = coerceTurnResult(await execute(() => lifecycle.poll(DEFAULT_SHARD_BUDGET)));
  }
  throw new Error(`wgpu-ui.open-budget-exhausted:${RETAINED_DOCUMENT_OPPORTUNITIES}`);
}

/** 🧹 Retires one captured WGPU lifetime without asking `beginClose` to cross an unacknowledged
 * Captured receipt. Cancellation at that exact stage first admits the original lifetime, then closes
 * the same bound UI owner through Accepted and Retired. */
export async function retireWgpuOwnedUiInstanceLifecycle(lifecycle: ShardInstanceLifecycleLease, route: WgpuOwnedUiInstanceRoute, execute: WgpuActorExecutor): Promise<void> {
  const captured = lifecycle.pendingReceipt;
  if (captured?.kind === "captured") {
    const acknowledged = coerceTurnResult(await execute(() => lifecycle.acknowledge(captured, DEFAULT_SHARD_BUDGET)));
    if (acknowledged.uiPatches.length > 0 || acknowledged.uiPatchReceipt !== undefined) throw new Error("wgpu-ui.patch-during-cancelled-open");
  }
  lifecycle.beginClose();
  const cursor = new WgpuUiIntakeCursor();
  while (lifecycle.progress().kind !== "complete") {
    const receipt = lifecycle.pendingReceipt;
    let current: WireTurnResult;
    if (receipt?.kind === "accepted") current = coerceTurnResult(await execute(() => lifecycle.acknowledge(receipt, DEFAULT_SHARD_BUDGET)));
    else if (receipt?.kind === "retired") {
      const retirement = await route.retire();
      current = coerceTurnResult(await execute(() => lifecycle.acknowledge(receipt, DEFAULT_SHARD_BUDGET, retirement)));
    } else {
      const progress = lifecycle.progress();
      if (progress.kind === "blocked") throw new Error(`wgpu-ui.lifecycle-close-blocked:${progress.failure ?? "unknown"}`);
      current = coerceTurnResult(await execute(() => progress.kind === "closing" ? lifecycle.close(DEFAULT_SHARD_BUDGET) : lifecycle.poll(DEFAULT_SHARD_BUDGET)));
    }
    if (current.uiPatches.length > 0 || current.uiPatchReceipt !== undefined) throw new Error("wgpu-ui.patch-after-lifecycle-close");
    await cursor.next("lifecycle-close");
  }
  lifecycle.dispose();
}

//#region 🧪️RetainedPatchOracle

/** 🖼️ One wire `UiPatch` reconciled onto `previous`: every `pack`-typed op payload is projected through
 * {@link decodePackWire} and the two WIT `u64` revisions narrowed through {@link packWireNatural}, so a
 * lossless integer carrier never reaches the retained tree as a `{kind, value}` object and a `bigint`
 * revision never fails the `baseRevision` identity check as a permanent desync. */
export function reconcileRetainedWindowPatch(previous: RetainedSurface | null, patch: WireUiPatch): { readonly surface: RetainedSurface | null; readonly desynced: boolean } {
  const ops = decodeWirePatchOps(patch.ops ?? [], decodePackWire);
  return applyUiPatchToRetained(previous, { revision: packWireNatural(patch.revision, "uiPatch.revision"), baseRevision: packWireNatural(patch.baseRevision, "uiPatch.baseRevision"), ops });
}
//#endregion 🧪️RetainedPatchOracle

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
  readonly render: (instanceId: number, surfaceId: string, bodyKey: string, viewState: unknown) => Promise<unknown>;
  readonly renderDocument: (instanceId: number, surfaceId: string, bodyKey: string, viewState: unknown) => Promise<string>;
  readonly contextMenu: (instanceId: number, request: unknown) => Promise<unknown>;
  readonly readWindowConfigPacks: (instanceId: number) => Promise<readonly WindowConfigPackEntry[]>;
  readonly loadWindowConfigPack: (instanceId: number, entry: WindowConfigPackEntry) => Promise<void>;
  readonly dispose: () => Promise<void>;
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
  const lifecycleByInstance = new Map<number, ShardInstanceLifecycleLease>();
  const uiRouteByInstance = new Map<number, WgpuOwnedUiInstanceRoute>();
  const openingInstances = new Map<number, Promise<number>>();
  const retiringInstances = new Map<number, Promise<void>>();
  const closingInstances = new Set<number>();
  const channelRequests = new AppChannelRequestSequence();
  let disposing = false;
  let disposal: Promise<void> | null = null;
  let eventSeq = 0;

  const requireActorId = (instanceId: number): string => {
    const actorId = actorIdByInstance.get(instanceId);
    if (disposing) throw new Error("wgpu-plugin-handle.closed");
    if (!actorId || closingInstances.has(instanceId)) throw new Error(`[DEBUG] program ${pluginId}: no actor for instance ${instanceId} (createApp not called, or already destroyed)`);
    return actorId;
  };
  const requireChannel = (instanceId: number): AppChannelClient => {
    const client = channelByInstance.get(instanceId);
    if (!client) throw new Error(`[DEBUG] program ${pluginId}: no channel for instance ${instanceId} (createApp not called, or already destroyed)`);
    return client;
  };
  const requireUiRoute = (instanceId: number): WgpuOwnedUiInstanceRoute => {
    const route = uiRouteByInstance.get(instanceId);
    if (!route) throw new Error("wgpu-ui.native-owner-required");
    return route;
  };
  const executeFor = (actorId: string): WgpuActorExecutor => <T>(work: () => Promise<T>) => submitActorWork(actorId, work);
  const releaseInstance = (instanceId: number, actorId: string): void => {
    actorIdByInstance.delete(instanceId);
    channelByInstance.delete(instanceId);
    lifecycleByInstance.delete(instanceId);
    uiRouteByInstance.delete(instanceId);
    pendingTurnEffects.delete(instanceId);
    actorTurnChains.delete(actorId);
    closingInstances.delete(instanceId);
  };

  const renderSurface = async (instanceId: number, surfaceId: string, bodyKey: string, viewState: unknown): Promise<WgpuOwnedUiProjection> => {
    const actorId = requireActorId(instanceId);
    const route = requireUiRoute(instanceId);
    const execute = executeFor(actorId);
    let current = await submitTurn(actorId, [{ kind: "surface-visible", payload: { surface: { instance: instanceId, surface: surfaceId }, bodyKey, viewState: encodePackValue(viewState) } }]);
    for (let opportunity = 0; opportunity < RETAINED_DOCUMENT_OPPORTUNITIES; opportunity += 1) {
      await route.accept(current, execute);
      requireActorId(instanceId);
      if (uiRouteByInstance.get(instanceId) !== route) throw new Error("wgpu-ui.owner-replaced");
      const projected = await route.project(surfaceId);
      if (projected) return projected;
      const status = typeof current.status === "string" ? current.status : current.status && typeof current.status === "object" && "tag" in current.status ? String((current.status as { readonly tag?: unknown }).tag ?? "") : "";
      if (status.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase() !== "more-work") throw new Error(`wgpu-ui.surface-not-published:${surfaceId}`);
      current = await submitTurn(actorId, []);
    }
    throw new Error(`wgpu-ui.render-budget-exhausted:${surfaceId}`);
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
      const route = requireUiRoute(instanceId);
      const execute = executeFor(actorId);
      const results: WireTurnResult[] = [];
      const accept = async (turn: WireTurnResult): Promise<void> => {
        results.push(turn, ...await route.accept(turn, execute));
      };
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
        let terminal: string | undefined;
        for (const commandPage of pages) {
          const submitted = await submitTurn(actorId, [], commandPage);
          terminal = submitted.commandIngress?.tag;
          await accept(submitted);
        }
        for (let continuation = 0; terminal !== "command-complete" && continuation < 1_024; continuation += 1) {
          if (terminal === "fault") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress fault`);
          if (terminal === "backpressure") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress backpressure after serialized submission`);
          const continued = await submitTurn(actorId, []);
          await accept(continued);
          terminal = continued.commandIngress?.tag;
        }
        if (terminal !== "command-complete") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress did not complete within 1024 continuations`);
      }
      const outFrames: Uint8Array[] = [];
      const leftover: WireVariant[] = [];
      for (const effect of results.flatMap((turn) => turn.effects)) {
        const frame = shellFrameBytes(effect, instanceId);
        if (frame) outFrames.push(frame);
        else leftover.push(effect);
      }
      pendingTurnEffects.set(instanceId, leftover);
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

  const handle: WgpuPluginHandle = {
    pluginId,
    manifest,
    createApp: (appId) => {
      if (disposing) return Promise.reject(new Error("wgpu-plugin-handle.closed"));
      const instanceId = nextGlobalInstanceId;
      nextGlobalInstanceId += 1;
      const actorId = `${pluginId}#${instanceId}`;
      actorIdByInstance.set(instanceId, actorId);
      const requireOpening = (): void => {
        if (disposing || closingInstances.has(instanceId) || actorIdByInstance.get(instanceId) !== actorId) throw new Error("wgpu-plugin-handle.closed");
      };
      const opening = Promise.resolve().then(async () => {
        requireOpening();
        await registry.activate(pluginId, actorId, "manual" satisfies ActivationReason);
        requireOpening();
        eventSeq += 1;
        const lifecycle = shardClient.captureInstanceLifecycle(actorId, instanceId);
        lifecycleByInstance.set(instanceId, lifecycle);
        const execute = executeFor(actorId);
        const opened = coerceTurnResult(await execute(() => lifecycle.open({ appId, actor: "local", config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})) }, DEFAULT_SHARD_BUDGET)));
        const captured = lifecycle.pendingReceipt;
        if (!captured || captured.kind !== "captured" || !lifecycle.lifetime) throw new Error("wgpu-ui.native-lifetime-required");
        const route = new WgpuOwnedUiInstanceRoute(lifecycle);
        uiRouteByInstance.set(instanceId, route);
        requireOpening();
        await settleInstanceLifecycle(lifecycle, route, opened, execute);
        requireOpening();
        channelByInstance.set(instanceId, new AppChannelClient(channelHandle, channelRequests, instanceId, appId, "local"));
        return instanceId;
      });
      openingInstances.set(instanceId, opening);
      const forget = (): void => { if (openingInstances.get(instanceId) === opening) openingInstances.delete(instanceId); };
      return opening.then((value) => { forget(); return value; }, (error) => {
        forget();
        return settleFailedInstanceOpen(error, () => (retiringInstances.has(instanceId) ? Promise.resolve() : handle.destroyApp(instanceId)));
      });
    },
    /** 🧹 Retires one instance. An instance whose `open` never produced a lifetime owns NO retained UI
     * — there is nothing to retire, only an actor to cancel — so a missing route joins the missing
     * lifecycle branch instead of throwing. Throwing here is what turned every guest first-step trap
     * into `create_app promise failed: wgpu-ui.native-owner-required` at the shell boundary: the
     * failed-open cleanup below awaits this call, and its rejection replaced the real cause. */
    destroyApp: (instanceId) => {
      const previous = retiringInstances.get(instanceId);
      if (previous) return previous;
      const actorId = actorIdByInstance.get(instanceId);
      if (!actorId) return Promise.resolve();
      closingInstances.add(instanceId);
      const opening = openingInstances.get(instanceId);
      const retirement = Promise.resolve().then(async () => {
        await opening?.then(() => {}, () => {});
        if (actorIdByInstance.get(instanceId) !== actorId) return;
        channelByInstance.get(instanceId)?.dispose();
        const lifecycle = lifecycleByInstance.get(instanceId);
        const route = uiRouteByInstance.get(instanceId);
        if (!lifecycle || !route) {
          registry.cancel(actorId);
          releaseInstance(instanceId, actorId);
          return;
        }
        await retireWgpuOwnedUiInstanceLifecycle(lifecycle, route, executeFor(actorId));
        registry.cancel(actorId);
        releaseInstance(instanceId, actorId);
      });
      retiringInstances.set(instanceId, retirement);
      const forget = (): void => { if (retiringInstances.get(instanceId) === retirement) retiringInstances.delete(instanceId); };
      void retirement.then(forget, forget);
      return retirement;
    },
    handleAction: (instanceId, actionJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(actionJson), viewState),
    handleCommand: (instanceId, commandJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(commandJson), viewState),
    render: (instanceId, surfaceId, bodyKey, viewState) => renderSurface(instanceId, surfaceId, bodyKey, viewState).then((result) => result.node),
    renderDocument: (instanceId, surfaceId, bodyKey, viewState) => renderSurface(instanceId, surfaceId, bodyKey, viewState).then((result) => JSON.stringify(result.document)),
    contextMenu: (instanceId, request) => requireChannel(instanceId).contextMenu(request),
    readWindowConfigPacks: (instanceId) => requireChannel(instanceId).readWindowConfigs(),
    loadWindowConfigPack: (instanceId, entry) => requireChannel(instanceId).loadWindowConfig(entry),
    dispose: () => {
      if (disposal) return disposal;
      disposing = true;
      const retirements = [...actorIdByInstance.keys()].map((instanceId) => handle.destroyApp(instanceId));
      disposal = Promise.allSettled(retirements).then((results) => {
        const failures = results.filter((result): result is PromiseRejectedResult => result.status === "rejected");
        if (failures.length) throw new AggregateError(failures.map((result) => result.reason), "wgpu-plugin-handle.retirement-failed");
        turnOutcomes.complete();
      });
      return disposal;
    },
  };
  return handle;
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
  readonly render: (instanceId: number, surfaceId: string, bodyKey: string, viewStateJson: string) => Promise<string>;
  readonly renderDocument: (instanceId: number, surfaceId: string, bodyKey: string, viewStateJson: string) => Promise<string>;
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
    render: (instanceId, surfaceId, bodyKey, viewStateJson) => handle.render(instanceId, surfaceId, bodyKey, JSON.parse(viewStateJson)).then((node) => JSON.stringify(node)),
    renderDocument: (instanceId, surfaceId, bodyKey, viewStateJson) => handle.renderDocument(instanceId, surfaceId, bodyKey, JSON.parse(viewStateJson)),
    contextMenu: (instanceId, requestJson) => handle.contextMenu(instanceId, JSON.parse(requestJson)).then((items) => JSON.stringify(items)),
  };
}
//#endregion 🔖️JsBridge
