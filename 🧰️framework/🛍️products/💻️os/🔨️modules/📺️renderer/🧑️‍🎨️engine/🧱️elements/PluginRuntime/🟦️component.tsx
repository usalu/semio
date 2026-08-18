/// <reference types="vitest/importMeta" />
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/PluginRuntime/component.tsx
/** @emoji 🐚️ `PluginRuntime` — the `PluginWasmHandle` binary-channel adapter (`loadPluginModule`/
 * `adaptPluginHandle`) that wraps a leased `framework-core` plugin wasm module's 5-function
 * `exchange` ABI behind the wider action/command/refreshUi/contextMenu/document-sync surface the
 * rest of the shell calls, plus the `AppChannelClient` frame-reassembly helpers
 * (`🔖️ChannelAdapter`) that back it.
 *
 * MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H1-react, design-runtime.md §1/§3): `loadPluginModule`
 * no longer leases one Worker per plugin (`acquirePluginModule`/`PluginModuleLease`, both deleted
 * in packet H2 — `📓️terra-H2-web-shard-report.md`). It drives a real actor through the kernel's
 * `ActivationRegistry` (manifest-only activation, LRU suspend/resume) over `ShardClient` (bounded
 * shard-worker pool, `actorId`-multiplexed) — see `🔖️ActorAdapter` below. `exchange()` on the raw
 * handle this file constructs submits one `app-command` event per frame through `ShardClient.turn`
 * and demuxes the resulting `TurnResult.effects` for the `SendMessage{Shell{instance}}` entries
 * `⚛️reactor/🦀️component.rs`'s `route_app_frame` wraps every non-`UiPatch` `AppFrame` reply in —
 * everything else in this file (`AppChannelClient`, `adaptPluginHandle`'s command/transaction/merge
 * methods) is unchanged, since it only ever spoke `AppCommand`/`AppFrame` bytes through that one
 * `exchange` seam and does not care what backs it.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import {
  type ArtifactInstanceRef,
  ArtifactMutationRouter,
  type Conflict,
  type ConflictId,
  type ConflictResolution,
  type ContextMenuItemSpec,
  type Effect,
  type HistoryPatch,
  InstanceDirectory,
  type InvocationResponse,
  type MergePolicy,
  type MergeReport,
  type PluginContextMenuRequest,
  type PluginGraphError,
  type PluginRegistryEntry,
  type PluginUiRefreshRequest,
  type PluginUiRefreshResponse,
  SemioFaultError,
  orderPluginRegistryEntries,
} from "@semio-tech/framework";
import {
  AppChannelClient,
  type AppFrameValue,
  decodeConflictsFromWire,
  decodeFaultFromWire,
  decodeMergeReportFromWire,
  decodeMutationEnvelopesPack,
  decodePackValue,
  encodePackValue,
  faultDisplayMessage,
} from "@semio-tech/framework-os";
import { type UiNode } from "@semio-tech/framework";
import {
  ActivationRegistry,
  type ActivationReason,
  type PluginWasmHandle as KernelPluginWasmHandle,
} from "../../../../../../../🔨️modules/🎠️kernel/🟦️component.ts";
import {
  ShardClient,
  type ShardBudget,
  type ShardEventEnvelope,
  type ShardWorkerLike,
} from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts";
import { type PluginManifest, type ViewModel } from "../Shell/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️plugin-runtime

export type PluginWasmHandle = {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly handleAction: (instanceId: number, actionJson: string, viewState: ViewModel) => Promise<InvocationResponse>;
  /** 🎛️ Dispatches a scoped command (os/plugin/app/mode) — optional since not every program declares commands. */
  readonly handleCommand?: (instanceId: number, commandJson: string, viewState: ViewModel) => Promise<InvocationResponse>;
  readonly refreshUi: (instanceId: number, request: PluginUiRefreshRequest) => Promise<PluginUiRefreshResponse>;
  readonly contextMenu: (instanceId: number, request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
  /** 🧾️ Complete projection used to seed or resynchronize host-owned history state. */
  readonly readHistory: (instanceId: number) => Promise<HistoryPatch>;
  /** 🔗️ The `DocumentApp` document-sync surface (WS-D) — optional since not every program has migrated onto it yet (WS-F).
   * 🚧️ Wave 1 gap (documented, not silently dropped): `protocol_channel::AppCommand` only carries
   * binary `pack`/`spr` document-container bytes (`LoadDocument`/`ReadDocument`, backed by
   * `store::print_document_pack`/`parse_document_pack`'s deflate+BLAKE3 `.spk` container) — there is
   * no JSON-text document command on the new channel, and no TS-side encoder for that container
   * format (deliberately out of scope for `🔖️PackValueCodec`, see its header doc). The OLD
   * `applyMutations`/`readAppDocument`/`loadAppDocument` all carried plain JSON text
   * (`MutationEnvelope[]` / a VCS envelope string), so they cannot be rebuilt on top of the binary
   * channel without a real pack encoder in TS (a separate, much larger work package). Every call
   * site already feature-detects these (`if (plugin.loadAppDocument) ...`), so leaving them
   * `undefined` here fails loud-but-inert (a `console.error`/no-op at the call site) rather than
   * silently miscoding a `.spk` container. */
  /** ⚖️ `AppCommand::ApplyEnvelopes`'s reply batches `MergeReport`/`Conflicts` frames alongside the
   * ingest itself (contract freeze §C6/§C9 "pushed unsolicited after every ingest") — decoded here,
   * same shape as {@link resolveConflict}'s reply, so a REMOTE peer's quarantined/degraded merge
   * reaches the caller instead of being dropped after the `Error` check. */
  readonly applyMutations?: (
    instanceId: number,
    mutationsPack: string,
  ) => Promise<{ readonly mergeReport: MergeReport | null; readonly conflicts: readonly Conflict[] | null }>;
  readonly readAppDocument?: (instanceId: number) => Promise<string>;
  readonly loadAppDocument?: (instanceId: number, documentJson: string) => Promise<void>;
  /** 📂️ Binary pack+spr document load (`AppCommand::LoadDocument`) — the Wave-1 channel-native path. */
  readonly loadAppDocumentPack?: (instanceId: number, pack: Uint8Array, spr: Uint8Array) => Promise<void>;
  readonly attachBackbone?: (instanceId: number, uri: string) => Promise<void>;
  readonly detachBackbone?: (instanceId: number) => Promise<void>;
  readonly ephemeralSnapshot?: (instanceId: number) => Promise<{ readonly presence: readonly number[]; readonly presenceGeneration: number; readonly transientGeneration: number } | null>;
  /** 🔁️ H1-react (design-abi.md §2) — delivers the result of an `Effect::InvokeExtension` back to
   * the ORIGINATING instance's actor as `Event::Completed{req, outcome}`, resuming the guest SDK
   * future `RequestRegistry` parked on `req`. Replaces the old `responseAction` redispatch —
   * `ShellHost/🟦️component.tsx`'s `applyHostEffects` `invokeExtension` branch is this method's one
   * real caller. Optional: only the `ActivationRegistry`/`ShardClient`-backed handle this file
   * constructs can submit a turn at all; a bare `adaptPluginHandle` (every inline test) has no actor
   * to submit one to. */
  readonly completeExtensionInvoke?: (instanceId: number, req: number, outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array }) => Promise<void>;
  /** 📦️ The instance's cached document pack (ticket
   * 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS, scout-1 §4) — `null`
   * before any document has been loaded/read on this instance. `TransactionCoordinator` reads this to
   * hand a contributor plugin the target's current snapshot for `artifact-mutation-plan`. */
  readonly documentPack: (instanceId: number) => { readonly pack: Uint8Array; readonly spr: Uint8Array } | null;
  /** 🎫️ `AppCommand::TransactionPrepare`, either wire form (contract freeze §2/§5.3) — see
   * {@link TransactionPrepareRequest}. */
  readonly transactionPrepare: (instanceId: number, txnId: string, request: TransactionPrepareRequest) => Promise<TransactionPrepareOutcome>;
  readonly transactionCommit: (instanceId: number, txnId: string) => Promise<TransactionCommitOutcome>;
  readonly transactionRollback: (instanceId: number, txnId: string) => Promise<void>;
  readonly transactionUndo: (instanceId: number, groupId: string) => Promise<void>;
  readonly transactionRedo: (instanceId: number, groupId: string) => Promise<void>;
  //#region 🔖️Merge
  /** ⚖️ Sets this instance's local merge-policy authority (`os.set-merge-policy`, contract freeze
   * `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6/§C8/§C9) — mirrors
   * `AppChannelClient.setMergePolicy` (`💻️os/🟦️component.ts`); throws on an `AppFrame::Error` reply
   * rather than silently no-opping. */
  readonly setMergePolicy: (instanceId: number, policy: MergePolicy) => Promise<void>;
  /** ⚔️ Accepts/discards an `Open` {@link Conflict} (`os.resolve-conflict`) — mirrors
   * `AppChannelClient.resolveConflict`; the reply batches `MergeReport` and (when the roster
   * changed) `Conflicts` frames together (contract freeze §C6 `resolve_conflict`), both decoded
   * here so the caller never re-parses wire frames itself. */
  readonly resolveConflict: (
    instanceId: number,
    conflictId: ConflictId,
    resolution: ConflictResolution,
  ) => Promise<{ readonly mergeReport: MergeReport | null; readonly conflicts: readonly Conflict[] | null }>;
  /** 📖️ Reads the open-conflict projection (`os.read-conflicts`) — mirrors
   * `AppChannelClient.readConflicts`. */
  readonly readConflicts: (instanceId: number) => Promise<readonly Conflict[]>;
  //#endregion 🔖️Merge
  readonly dispose: () => void;
};

export type { PluginRegistryEntry };

//#region 🔖️ActorAdapter
/**
 * @emoji 🧵️ H1-react — replaces the deleted `acquirePluginModule`/`PluginModuleLease` (one Worker per
 * plugin, `📓️terra-H2-web-shard-report.md`'s "must not exist" list) with the pooled `ShardClient` +
 * `ActivationRegistry` design-runtime.md §1/§3 specifies. ONE `ShardClient` (bounded worker pool,
 * `min(hardwareConcurrency-1,4)` shards) and ONE `ActivationRegistry` (manifest-only activation, LRU
 * suspend/resume) for the whole tab — every `loadPluginModule` call shares them, matching the design's
 * "ShardClient... replaces PluginWorkerClient" framing (one pool, not one per caller). Lazily
 * constructed on first use so a pure SSR/test import of this module never touches `Worker`/
 * `navigator`. */
const SHARD_WORKER_URL = "/plugin-modules/_shard/🟨️shard-worker.js";

/** ⛽️ Provisional constant turn budget — same honestly-flagged gap `ProgramBridge/🧊️component.rs`'s
 * native `TURN_BUDGET` documents ("until the DRR scheduler threads a real per-lane one through");
 * this is that same budget's web twin, field-for-field against `ShardBudget`. */
const DEFAULT_SHARD_BUDGET: ShardBudget = { fuel: 50_000_000, wallMs: 100, memoryBytes: 256 * 1024 * 1024, uiNodes: 20_000, mailboxLen: 64, maxEffects: 64, maxPatchBytes: 1 << 20 };

let sharedShardClient: ShardClient | null = null;
function getShardClient(): ShardClient {
  if (sharedShardClient) return sharedShardClient;
  const hardwareConcurrency = typeof navigator !== "undefined" && typeof navigator.hardwareConcurrency === "number" ? navigator.hardwareConcurrency : 5;
  const shardCount = Math.max(1, Math.min(hardwareConcurrency - 1, 4));
  sharedShardClient = new ShardClient({
    shardCount,
    // 🎭️ A real DOM `Worker` satisfies `ShardWorkerLike` structurally at runtime (same claim
    // `🌐plugin-web-materialize.ts`'s own doc makes) — the cast only bridges `onmessage`/`onerror`'s
    // wider native `MessageEvent`/`ErrorEvent` handler types down to the interface's minimal
    // `{data: unknown}`/`unknown` shape, which a `MessageEvent`/`ErrorEvent` handler always satisfies.
    createWorker: () => new Worker(SHARD_WORKER_URL, { type: "module" }) as unknown as ShardWorkerLike,
    onActorTrap: (actorId, message) => console.error(`[DEBUG] PluginRuntime: actor ${actorId} trapped: ${message}`),
    onShardLost: (shardIndex, actorIds) => console.error(`[DEBUG] PluginRuntime: shard ${shardIndex} lost, actors needing restore: ${actorIds.join(", ")}`),
  });
  return sharedShardClient;
}

let sharedActivationRegistry: ActivationRegistry | null = null;
function getActivationRegistry(): ActivationRegistry {
  sharedActivationRegistry ??= new ActivationRegistry({ shardClient: getShardClient(), defaultBudget: DEFAULT_SHARD_BUDGET });
  return sharedActivationRegistry;
}

/** 🚧️ Best-effort JS representation of one raw WIT `effect`/`patch-op` variant crossing the wasm
 * boundary — UNVERIFIED against a real compiled artifact (no plugin has migrated onto `world actor`
 * yet; W3 hasn't started — same gap `🧵️shard-client.ts`'s and `🌐plugin-web-materialize.ts`'s own
 * header docs flag). Assumed shape: jco's standard variant binding, `tag` the WIT case name
 * (kebab-case) and `val` its payload record (fields camelCased from kebab), matching the SAME
 * convention this ticket's other packets already documented for this exact boundary. */
type WireVariant<T = unknown> = { readonly tag?: string; readonly val?: T };

type WireUiPatch = {
  readonly surface?: { readonly instance?: number; readonly surface?: number };
  readonly kind?: string;
  readonly revision?: number;
  readonly baseRevision?: number;
  readonly ops?: readonly WireVariant[];
};

type WireTurnResult = {
  readonly uiPatches: readonly WireUiPatch[];
  readonly effects: readonly WireVariant[];
  readonly nextWake: number | null;
};

/** 📥️ Defensive parse of `ShardClient.turn()`'s opaque `unknown` return (typed opaque at that
 * module's own public boundary — see its header doc) into the fields this file needs, tolerating a
 * missing/differently-shaped field rather than throwing mid-turn. */
function coerceTurnResult(raw: unknown): WireTurnResult {
  const record = (raw && typeof raw === "object" ? raw : {}) as Record<string, unknown>;
  const uiPatches = Array.isArray(record.uiPatches) ? (record.uiPatches as WireUiPatch[]) : [];
  const effects = Array.isArray(record.effects) ? (record.effects as WireVariant[]) : [];
  const nextWake = typeof record.nextWake === "number" ? record.nextWake : null;
  return { uiPatches, effects, nextWake };
}

/** 🔀️ `Effect::SendMessage{target: Shell{instance}}` → the raw `AppFrame` bytes it wraps —
 * `⚛️reactor/🦀️component.rs`'s `route_app_frame` puts EVERY non-`UiPatch` `AppFrame` reply here
 * (design-abi.md §2). Mirrors `📦️glue.rs`'s native `apply_turn_result` (H3-wgpu-native) — same
 * demux, TS twin. */
function shellFrameBytes(effect: WireVariant, instanceId: number): Uint8Array | null {
  if (effect.tag !== "send-message") return null;
  const val = (effect.val ?? {}) as { readonly target?: WireVariant<number>; readonly payload?: unknown };
  if (!val.target || val.target.tag !== "shell") return null;
  if (Number(val.target.val) !== instanceId) return null;
  if (val.payload === undefined) return null;
  return coerceWireBytes(val.payload);
}

//#region 🔖️RetainedUiPatch
/** 🩹️ `kernel::PatchOp`, TS twin restricted to what `⚛️reactor/🩹️patches/🦀️component.rs`'s
 * `PatchTracker` actually emits this wave (its own doc: "full-body only — every dirty surface emits
 * one `PatchOp::Replace` at the root path"). `path` is `list<u32>` at the WIT boundary (an empty
 * array for the root). */
type PatchOp =
  | { readonly kind: "Replace"; readonly path: readonly number[]; readonly node: UiNode }
  | { readonly kind: "InsertChild"; readonly path: readonly number[]; readonly index: number; readonly node: UiNode }
  | { readonly kind: "RemoveChild"; readonly path: readonly number[]; readonly index: number }
  | { readonly kind: "SetProps"; readonly path: readonly number[]; readonly props: unknown };

function decodeWirePatchOps(ops: readonly WireVariant[]): readonly PatchOp[] {
  const decoded: PatchOp[] = [];
  for (const op of ops) {
    const val = (op.val ?? {}) as Record<string, unknown>;
    const path = Array.isArray(val.path) ? (val.path as number[]) : [];
    switch (op.tag) {
      case "replace":
        decoded.push({ kind: "Replace", path, node: decodePackValue(coerceWireBytes(val.node)) as UiNode });
        break;
      case "insert-child":
        decoded.push({ kind: "InsertChild", path, index: Number(val.index ?? 0), node: decodePackValue(coerceWireBytes(val.node)) as UiNode });
        break;
      case "remove-child":
        decoded.push({ kind: "RemoveChild", path, index: Number(val.index ?? 0) });
        break;
      case "set-props":
        decoded.push({ kind: "SetProps", path, props: val.props !== undefined ? decodePackValue(coerceWireBytes(val.props)) : undefined });
        break;
      default:
        break;
    }
  }
  return decoded;
}

export type RetainedSurface = { readonly revision: number; readonly node: UiNode };

/**
 * @emoji 🖼️ H1-react (design-runtime.md §1 `SceneStore` / packet brief item 2) — reconciles one
 * `UiPatch`'s ops onto `previous` (the last body this file retained for the surface), so the UI
 * thread reads an already-reconciled tree instead of awaiting a plugin turn. Only a root
 * `PatchOp::Replace` (path `[]`) is applied — the only shape any guest emits this wave (see
 * `PatchOp`'s doc above); anything else, or a `baseRevision` that doesn't match `previous.revision`
 * on a NON-full-replace patch, is an honest desync — `previous` is kept rather than an unverified
 * partial walk applied, mirroring `📦️glue.rs`'s native `KernelThreadState.retained` exactly (H3).
 */
export function applyUiPatchToRetained(previous: RetainedSurface | null, patch: { readonly revision: number; readonly baseRevision: number; readonly ops: readonly PatchOp[] }): { readonly surface: RetainedSurface | null; readonly desynced: boolean } {
  let node: UiNode | null = previous?.node ?? null;
  let sawFullReplace = false;
  for (const op of patch.ops) {
    if (op.kind === "Replace" && op.path.length === 0) {
      node = op.node;
      sawFullReplace = true;
    } else {
      return { surface: previous, desynced: true };
    }
  }
  if (!sawFullReplace && previous && patch.baseRevision !== previous.revision) return { surface: previous, desynced: true };
  return { surface: node !== null ? { revision: patch.revision, node } : previous, desynced: false };
}
//#endregion 🔖️RetainedUiPatch

/** 🚧️ Best-effort conversion of a raw WIT `effect` variant (`{tag, val}`, see `WireVariant`'s doc for
 * the unverified-boundary caveat) into the friendly `Effect` union `🎠️kernel/🟦️component.ts` already
 * declares — Rust `kernel::Effect`'s externally-tagged serde shape (`{effectName: {...fields}}` /
 * `"requestSync"`), which is what every downstream consumer (`applyHostEffects` and friends) already
 * expects. Covers the effect kinds this renderer actually branches on; an effect kind with no case
 * here degrades to an honest `[DEBUG]`-logged drop rather than guessing an unverified shape. */
function wireEffectToFriendly(effect: WireVariant): Effect | null {
  const val = (effect.val ?? {}) as Record<string, unknown>;
  const str = (key: string): string => String(val[key] ?? "");
  const num = (key: string): number => Number(val[key] ?? 0);
  const packField = (key: string): unknown => (val[key] !== undefined ? decodePackValue(coerceWireBytes(val[key])) : undefined);
  switch (effect.tag) {
    case "request-sync":
      return "requestSync";
    case "notify":
      return { notify: { message: str("message") } };
    case "navigate":
      return { navigate: { uri: str("uri") } };
    case "open-external-url":
      return { openExternalUrl: { url: str("url") } };
    case "set-panel":
      return { setPanel: { panelJson: str("panelJson") } };
    case "clipboard-write":
      return { clipboardWrite: { fragment: packField("fragment") } };
    case "replay-shell-command":
      return { replayShellCommand: { actionId: str("actionId"), args: packField("args") } };
    case "set-active-utility":
      return { setActiveUtility: { windowId: str("windowId"), utilityId: str("utilityId") } };
    case "set-active-tool":
      return { setActiveTool: { toolId: str("toolId") } };
    case "open-window":
      return { openWindow: { req: num("req"), kind: str("kind"), params: packField("params") } };
    case "close-window":
      return { closeWindow: { window: num("window") } };
    case "dispatch-action":
      return { dispatchAction: { req: num("req"), action: str("action"), args: packField("args"), delayMs: num("delayMs") } };
    case "open-dialog":
      return { openDialog: { req: num("req"), dialogId: str("dialogId"), args: packField("args") as Record<string, unknown> | undefined } };
    case "invoke-extension":
      return { invokeExtension: { req: num("req"), extensionId: str("extensionId"), capability: str("capability"), requestJson: JSON.stringify(packField("payload") ?? {}) } };
    case "spawn-plugin-instance":
      return { spawnPluginInstance: { req: num("req"), pluginId: str("pluginId"), appId: str("appId"), osInstanceId: val.osInstanceId as string | undefined, label: val.label as string | undefined, documentJson: val.documentJson as string | undefined } };
    case "open-plugin-instance":
      return { openPluginInstance: { pluginId: str("pluginId"), appId: str("appId"), osInstanceId: val.osInstanceId as string | undefined } };
    default:
      console.warn(`[DEBUG] wireEffectToFriendly: unmapped effect "${effect.tag}" dropped — unverified wasm-boundary conversion (this file's 🔖️ActorAdapter doc)`);
      return null;
  }
}

/** 🎯️ Per-instance "leftover" `TurnResult.effects` — everything a turn produced that was NOT a
 * `SendMessage{Shell}` reply frame (the old `AppFrame::Effects` wrapper's replacement, design-abi.md
 * §2). `exchange()` fills this on every turn; `performInvocation` drains it right after its own
 * `client.command()` call resolves — both operations share one turn, so this is never stale by more
 * than the caller's own await. */
const pendingTurnEffects = new Map<number, WireVariant[]>();

/** 🪪️ H1-react — instance ids must be unique across EVERY plugin, not just within one
 * `loadPluginModule` call: `pendingTurnEffects` above is keyed by `instanceId` alone and is shared
 * module-wide (mirrors `📦️glue.rs`'s native `KernelClient` — `next_instance_id` lives on the ONE
 * global `KernelThreadState`, not per-plugin). A per-plugin-scoped counter would let two different
 * plugins both mint instance `1` and silently cross-read each other's leftover turn effects. */
let nextGlobalInstanceId = 1;

/** 🚦 H1-react — `🟨️shard-worker.js` rejects (not queues) a SECOND in-flight `turn` for the same
 * `actorId` ("shard worker: actor … already has a turn in flight", `🌐plugin-web-materialize.ts`'s
 * `inFlightTurnActors` guard: "two turn requests for the SAME actorId overlapping is a caller bug —
 * the scheduler's job to prevent, not this worker's"). The OLD adapter's `withSerializedPluginWasmHandle`
 * (deleted alongside `PluginWorkerClient`, `🎠️kernel/🟦️component.ts`'s own doc comment names it)
 * queued concurrent `exchange()` calls transparently — this is that same guarantee's replacement, one
 * promise chain per actor, so two overlapping `handleAction`/`refreshUi`/etc. calls on the same
 * instance run turn-after-turn instead of the second one throwing. */
const actorTurnQueue = new Map<string, Promise<unknown>>();
export function serializePerActor<T>(actorId: string, run: () => Promise<T>): Promise<T> {
  const previous = actorTurnQueue.get(actorId) ?? Promise.resolve();
  const next = previous.then(run, run);
  actorTurnQueue.set(
    actorId,
    next.catch(() => {}),
  );
  return next;
}

/** 🖼️ Last reconciled "window" body per actor — the ONE surface `⚛️reactor/🦀️component.rs`'s
 * `dirty_render` loop renders this wave (hardcoded `plugin_render(instance, "window", "{}")`
 * regardless of which surface key a `surface-visible` event names — a real, upstream limitation of
 * this wave's reactor, not invented here). Keyed by `actorId` so a suspend+resume (fresh checkpoint
 * restore) naturally starts a new entry. */
const retainedWindowByActor = new Map<string, RetainedSurface>();

//#endregion 🔖️ActorAdapter

/** 🐚️ Acquires a real actor through `ActivationRegistry`/`ShardClient` (replacing the deleted
 * `acquirePluginModule`/`PluginModuleLease` per-plugin Worker lease — design-runtime.md §3) and
 * adapts it exactly like the old wasm-Worker handle: `dispose()` disposes this instance's worker-side
 * actor entry via `ShardClient.dispose` (not a `LeasePool` release — there is no shared module lease
 * to refcount anymore, one actor belongs to exactly one instance). */
export async function loadPluginModule(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  const registry = getActivationRegistry();
  registry.registerManifest({ pluginId, moduleUrl, caps: [] });
  const manifest = await fetchDescriptorManifest(pluginId, moduleUrl);
  const shardClient = getShardClient();
  const actorIdByInstance = new Map<number, string>();
  let eventSeq = 0;
  const requireActorId = (instanceId: number): string => {
    const actorId = actorIdByInstance.get(instanceId);
    if (!actorId) throw new Error(`[DEBUG] program ${pluginId}: no actor for instance ${instanceId} (createApp not called, or already destroyed)`);
    return actorId;
  };
  const submitTurn = async (actorId: string, events: readonly ShardEventEnvelope[]): Promise<WireTurnResult> =>
    serializePerActor(actorId, async () => coerceTurnResult(await shardClient.turn(actorId, events, DEFAULT_SHARD_BUDGET)));

  const handle: KernelPluginWasmHandle = {
    manifest: async () => encodePackValue(manifest),
    createApp: async (appId) => {
      const instanceId = nextGlobalInstanceId;
      nextGlobalInstanceId += 1;
      const actorId = `${pluginId}#${instanceId}`;
      actorIdByInstance.set(instanceId, actorId);
      await registry.activate(pluginId, actorId, "manual" satisfies ActivationReason);
      eventSeq += 1;
      await submitTurn(actorId, [
        {
          kind: "instance-open",
          payload: { instance: instanceId, appId, actor: currentPluginRuntimeActor, config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})) },
        },
      ]);
      return instanceId;
    },
    destroyApp: async (instanceId) => {
      const actorId = actorIdByInstance.get(instanceId);
      if (!actorId) return;
      actorIdByInstance.delete(instanceId);
      retainedWindowByActor.delete(actorId);
      pendingTurnEffects.delete(instanceId);
      shardClient.dispose(actorId);
    },
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
    dispose: () => {
      for (const actorId of actorIdByInstance.values()) {
        retainedWindowByActor.delete(actorId);
        shardClient.dispose(actorId);
      }
      actorIdByInstance.clear();
    },
  };

  const richHandle = await adaptPluginHandle(pluginId, { handle, release: handle.dispose });

  /** 🔁️ H1-react item 2 — window-body refresh no longer goes through `AppCommand::RefreshUi`
   * (deleted, channel v12): it submits `Event::SurfaceVisible` directly and reads back whatever this
   * SAME turn's `TurnResult.uiPatches` produced (or the retained tree if nothing changed — the
   * `PatchTracker` on the guest side emits nothing for an unchanged body). Panels/engagements/
   * measures/tools/labels have no wire path yet — `⚛️reactor/🦀️component.rs`'s `dirty_render` loop
   * only ever renders the ONE "window" surface this wave, an upstream limitation reported honestly
   * (matching `ProgramBridge/🧊️component.rs`'s native `window_engagements`/`window_measures` stubs,
   * H3-wgpu-native) rather than guessed at here. */
  const refreshUi = async (instanceId: number, request: PluginUiRefreshRequest): Promise<PluginUiRefreshResponse> => {
    const windowTargets = request.windows ?? [];
    if (windowTargets.length === 0) return {};
    const actorId = requireActorId(instanceId);
    eventSeq += 1;
    const result = await submitTurn(actorId, [{ kind: "surface-visible", payload: { surface: { instance: instanceId, surface: 0 } } }]);
    if (result.uiPatches.length > 0) applyRetainedWindowPatches(actorId, result.uiPatches);
    const retained = retainedWindowByActor.get(actorId);
    if (!retained) return {};
    const hash = fnv1aHex(new TextEncoder().encode(JSON.stringify(retained.node))) + ":" + String(retained.revision);
    const windows = windowTargets.map((target) => ({ key: target.key, hash, value: retained.node }));
    return { windows };
  };

  /** 🔁️ H1-react item 2 ("finish the invokeExtension branch") — the real `req`-correlated completion
   * `ShellHost/🟦️component.tsx`'s `applyHostEffects` used to only log loudly about. Submits
   * `Event::Completed{req, outcome}` on the ORIGINATING instance's own actor so its `RequestRegistry`
   * resumes the parked future (design-abi.md §2's "the SDK resumes the awaiting future on
   * `event.completed`"). */
  const completeExtensionInvoke = async (instanceId: number, req: number, outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array }): Promise<void> => {
    const actorId = requireActorId(instanceId);
    await submitTurn(actorId, [
      {
        kind: "completed",
        payload: { req, outcome: "ok" in outcome ? { tag: "ok", val: Array.from(outcome.ok) } : { tag: "fault", val: Array.from(outcome.fault) } },
      },
    ]);
  };

  return { ...richHandle, refreshUi, completeExtensionInvoke };
}

function applyRetainedWindowPatches(actorId: string, uiPatches: readonly WireUiPatch[]): void {
  for (const patch of uiPatches) {
    const ops = decodeWirePatchOps(patch.ops ?? []);
    const previous = retainedWindowByActor.get(actorId) ?? null;
    const { surface, desynced } = applyUiPatchToRetained(previous, { revision: patch.revision ?? 0, baseRevision: patch.baseRevision ?? 0, ops });
    if (desynced) {
      console.warn(`[DEBUG] applyRetainedWindowPatches: actor ${actorId} desynced (unrecognized op shape or stale baseRevision) — keeping the previously retained body`);
      continue;
    }
    if (surface) retainedWindowByActor.set(actorId, surface);
  }
}

/** 📇️ H1-react — reads the build-time `🔣️descriptor.json` (design-abi.md §3, packet E1-describe's
 * emitter) siblinged next to `moduleUrl`'s directory, matching `ProgramBridge/🧊️component.rs`'s
 * native `read_descriptor_manifest` (H3-wgpu-native) exactly: an honest EMPTY manifest (zero apps),
 * not a fabricated one, whenever no descriptor exists yet — as of this packet, only `🗒️note` has a
 * real committed one (`📓️status.md`'s "E2-builder-descriptor" entry); every other plugin hits this
 * fallback until W3 migrates it. Never instantiates wasm to ask — that's exactly the "no eager
 * loading" property `loadPluginModule` used to break (per H2's lease-request naming this file). */
export async function fetchDescriptorManifest(pluginId: string, moduleUrl: string): Promise<PluginManifest> {
  const descriptorUrl = moduleUrl.replace(/\/[^/]+$/, "/🔣️descriptor.json");
  try {
    const response = await fetch(descriptorUrl);
    if (response.ok) {
      const descriptor = (await response.json()) as { readonly manifest?: PluginManifest };
      if (descriptor.manifest) return descriptor.manifest;
    }
  } catch (error) {
    console.warn(`[DEBUG] fetchDescriptorManifest: ${descriptorUrl} unreachable — using an empty manifest`, error);
  }
  console.warn(`[DEBUG] fetchDescriptorManifest: no descriptor for ${pluginId} yet (E1-describe/W3 seam) — loading with an empty manifest, no eager instantiation`);
  return { pluginId, label: pluginId, version: "", apps: [], examples: [], capabilities: [], topicContributions: [], commands: [], artifactKinds: [], dependencies: [], contributions: [] } as unknown as PluginManifest;
}

//#region 🔖️ChannelAdapter
/** 🎯️ DslValue may ship `Vec<u8>` as a number array, a Uint8Array, or a `{ kind:"bytes", value }`
 * object — used both for the old DSL-pack byte fields AND (H1-react) for `pack`-typed fields inside a
 * raw WIT `effect`/`patch-op` variant, which jco represents as a plain byte array or `Uint8Array`. */
export function coerceWireBytes(raw: unknown): Uint8Array {
  if (raw instanceof Uint8Array) return raw;
  if (Array.isArray(raw)) return Uint8Array.from(raw as number[]);
  if (raw && typeof raw === "object") {
    const record = raw as Record<string, unknown>;
    if (record.kind === "bytes" && Array.isArray(record.value)) return Uint8Array.from(record.value as number[]);
    if (Array.isArray(record.data)) return Uint8Array.from(record.data as number[]);
  }
  if (typeof raw === "string") {
    // base64 fallback
    const binary = atob(raw);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
    return bytes;
  }
  throw new Error(`[DEBUG] coerceWireBytes: unsupported payload ${JSON.stringify(raw)?.slice(0, 120)}`);
}

/** 🎯️ Shared by `handleAction`/`handleCommand`: encodes `envelope` + `viewState`, sends one
 * `AppCommand::Command` frame, and reassembles the `Invocation` frame plus this SAME turn's leftover
 * `TurnResult.effects` (`pendingTurnEffects`, H1-react — replaces the deleted `AppFrame::Effects`/
 * `Events` frames) back into the `InvocationResponse` shape the rest of this file already consumes.
 * `events` has no wire counterpart in this wave (an honest gap `ProgramBridge/🧊️component.rs`'s
 * native `invocation_from_frames` already flags identically). */
async function performInvocation(client: AppChannelClient, instanceId: number, invocation: unknown, invocationKind: "action" | "command", viewState: unknown): Promise<InvocationResponse> {
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
      throw new Error(`${invocationKind} failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
    }
  }
  const leftover = pendingTurnEffects.get(instanceId) ?? [];
  pendingTurnEffects.delete(instanceId);
  const requestedEffects = leftover.map(wireEffectToFriendly).filter((effect): effect is Effect => effect !== null);
  return {
    output,
    mutations: [],
    inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] },
    diagnostics,
    requestedEffects,
    events: [],
    uiScope,
    historyPatch,
  };
}

async function performContextMenu(client: AppChannelClient, request: PluginContextMenuRequest): Promise<readonly ContextMenuItemSpec[]> {
  const items = await client.contextMenu(request);
  return Array.isArray(items) ? (items as ContextMenuItemSpec[]) : [];
}

//#region 🔖️ActorIdentity
/** 🪪️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§C3 — the shell's
 * current actor id (`user:{userId}#{shellSessionId}` once identity resolves, `client-<random>` before
 * that), stamped onto every `AppChannelClient` created from here on. Module-level rather than an
 * `adaptPluginHandle` parameter because `PluginWasmHandle` is the kernel's frozen shape (no setter
 * method to add) and every real call site (`loadPluginModuleResilient`, `ShellHelpers/🟦️component.tsx`)
 * lives outside this ticket's lease — this is the smallest surface that reaches every future
 * `createApp` call without touching a foreign-leased signature. Known limitation: a single JS realm
 * hosting more than one `ShellHost` (e.g. the multi-pane demonstrator) shares one actor id across
 * panes — out of scope for this lane, flagged in `📓️w2-c-report.md`. */
let currentPluginRuntimeActor = "local";

/** 🪪️ Sets the actor id every subsequently-created `AppChannelClient` is stamped with — call once the
 * shell mints/resolves its `user:{userId}#{shellSessionId}` identity (or reverts it on sign-out). */
export function setPluginRuntimeActor(actor: string): void {
  currentPluginRuntimeActor = actor;
}
//#endregion 🔖️ActorIdentity

/** 📡️ Wraps the framework-core `PluginWasmHandle` (the 5-function binary `exchange` ABI) behind the
 * SAME method surface the rest of this file already calls — the compatibility adapter for
 * `HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS`'s ABI flip. One `AppChannelClient` per
 * live instance id (created in `createApp`, dropped in `destroyApp`) frames every call through
 * `AppCommand`/`AppFrame`; no `AppCommand::Hello` handshake is sent — `plugin_exchange` already
 * defaults an un-`Hello`'d instance's actor to `"local"` (see `instance_actor`'s doc), so skipping it
 * avoids the alternative (sending a real `Hello.config`, which would run every migrated app's
 * `apply_config_bytes` against an arbitrary empty/placeholder config — wrong for an app like shooting
 * whose `ShootingConfig` fields have no `#[serde(default)]` and would reject `{}`). */
export async function adaptPluginHandle(pluginId: string, lease: { readonly handle: KernelPluginWasmHandle; readonly release: () => void }): Promise<PluginWasmHandle> {
  const handle = lease.handle;
  const manifest = decodePackValue(await handle.manifest()) as unknown as PluginManifest;
  const channels = new Map<number, AppChannelClient>();
  const requireChannel = (instanceId: number): AppChannelClient => {
    const client = channels.get(instanceId);
    if (!client) throw new Error(`[DEBUG] program ${pluginId}: no channel for instance ${instanceId} (createApp not called, or already destroyed)`);
    return client;
  };
  return {
    pluginId,
    manifest,
    createApp: async (appId) => {
      const instanceId = await handle.createApp(appId);
      channels.set(instanceId, new AppChannelClient(handle, instanceId, appId, currentPluginRuntimeActor));
      return instanceId;
    },
    destroyApp: async (instanceId) => {
      channels.delete(instanceId);
      await handle.destroyApp(instanceId);
    },
    handleAction: (instanceId, actionJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(actionJson), "action", viewState),
    handleCommand: (instanceId, commandJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(commandJson), "command", viewState),
    // 🚧️ H1-react — window-body refresh needs the ActivationRegistry/ShardClient `Event::SurfaceVisible`
    // path this bare adapter has no access to (only the raw `exchange`-shaped `handle`, no actorId);
    // `loadPluginModule` overrides this field with the real implementation right after calling this
    // function. A caller that constructs `adaptPluginHandle` directly (every inline test in this file)
    // gets an honest empty result rather than a throw — `AppCommand::RefreshUi`/`SectionProbe` no
    // longer exist on the wire regardless (channel v12), so there is no fallback command to send here.
    refreshUi: async () => ({}),
    contextMenu: (instanceId, request) => performContextMenu(requireChannel(instanceId), request),
    readHistory: async (instanceId) => {
      const frames = await requireChannel(instanceId).readHistory();
      const frame = frames.find((candidate): candidate is Extract<AppFrameValue, { readonly HistorySnapshot: unknown }> => "HistorySnapshot" in candidate);
      if (!frame) throw new Error("[DEBUG] readHistory: missing HistorySnapshot frame");
      return decodePackValue(new Uint8Array(frame.HistorySnapshot.history_patch)) as HistoryPatch;
    },
    applyMutations: async (instanceId, mutationsPack) => {
      const envelopes = decodeMutationEnvelopesPack(mutationsPack);
      const frames = await requireChannel(instanceId).applyEnvelopes(envelopes);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`[DEBUG] applyMutations failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
      const mergeFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly MergeReport: unknown }> => "MergeReport" in frame);
      const conflictsFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Conflicts: unknown }> => "Conflicts" in frame);
      return {
        mergeReport: mergeFrame ? decodeMergeReportFromWire(mergeFrame.MergeReport.report, decodePackValue) : null,
        conflicts: conflictsFrame ? decodeConflictsFromWire(conflictsFrame.Conflicts.conflicts, decodePackValue) : null,
      };
    },
    readAppDocument: undefined,
    loadAppDocument: undefined,
    loadAppDocumentPack: async (instanceId, pack, spr) => {
      const frames = await requireChannel(instanceId).loadDocument(pack, spr);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`[DEBUG] loadAppDocumentPack failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    },
    // 🚧️ Channel v12 (A4-channel) retired `AppChannelClient.attachBackbone`/`detachBackbone`/`drain` —
    // backbone attach/detach collapses into event-driven `Event::Message`/`subscribe` (design-abi.md
    // §2/§4), and `exchange(id, [])`'s drain has no replacement (guests are woken by events/timers/
    // `next-wake` now). `EffectBackbone` (the per-instance replacement) has not landed — flagged as a
    // still-open critical-path gap in `📓️status.md`'s "A2-abi-sdk — honest partial" entry, confirmed
    // still open as of `ProgramBridge/🧊️component.rs`'s native twin (H3-wgpu-native), which stubs the
    // identical three methods with explicit errors rather than guessing a wire format. Left `undefined`
    // here — every real call site in `ShellHost/🟦️component.tsx` already optional-chains these.
    attachBackbone: undefined,
    detachBackbone: undefined,
    ephemeralSnapshot: undefined,
    documentPack: (instanceId) => requireChannel(instanceId).documentPack(),
    transactionPrepare: async (instanceId, txnId, request) => {
      const frames =
        request.form === "owner"
          ? await requireChannel(instanceId).transactionPrepareOwner(txnId, request.mutationId, request.payload)
          : await requireChannel(instanceId).transactionPreparePlanned(txnId, request.preparedOps, request.label, request.origin);
      const frame = frames.find((candidate): candidate is Extract<AppFrameValue, { readonly transactionPrepared: unknown }> => "transactionPrepared" in candidate);
      if (!frame) throw new Error(`[DEBUG] program ${pluginId}: transactionPrepare(${instanceId}): missing transactionPrepared frame`);
      return {
        foreign: frame.transactionPrepared.foreign.map((bytes) => new Uint8Array(bytes)),
        rejection: frame.transactionPrepared.rejection.length > 0 ? new Uint8Array(frame.transactionPrepared.rejection) : null,
      };
    },
    transactionCommit: async (instanceId, txnId) => {
      const frames = await requireChannel(instanceId).transactionCommit(txnId);
      const committed = frames.find((candidate): candidate is Extract<AppFrameValue, { readonly transactionCommitted: unknown }> => "transactionCommitted" in candidate);
      if (committed) return { editId: committed.transactionCommitted.edit_id };
      const errorFrame = frames.find((candidate): candidate is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in candidate);
      if (errorFrame) return { rejection: new Uint8Array(errorFrame.Error.fault) };
      throw new Error(`[DEBUG] program ${pluginId}: transactionCommit(${instanceId}): missing transactionCommitted/Error frame`);
    },
    transactionRollback: async (instanceId, txnId) => {
      await requireChannel(instanceId).transactionRollback(txnId);
    },
    transactionUndo: async (instanceId, groupId) => {
      await requireChannel(instanceId).transactionUndo(groupId);
    },
    transactionRedo: async (instanceId, groupId) => {
      await requireChannel(instanceId).transactionRedo(groupId);
    },
    //#region 🔖️Merge
    setMergePolicy: async (instanceId, policy) => {
      const frames = await requireChannel(instanceId).setMergePolicy(policy);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`[DEBUG] program ${pluginId}: setMergePolicy failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    },
    resolveConflict: async (instanceId, conflictId, resolution) => {
      const frames = await requireChannel(instanceId).resolveConflict(conflictId, resolution);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`[DEBUG] program ${pluginId}: resolveConflict failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
      const mergeFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly MergeReport: unknown }> => "MergeReport" in frame);
      const conflictsFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Conflicts: unknown }> => "Conflicts" in frame);
      return {
        mergeReport: mergeFrame ? decodeMergeReportFromWire(mergeFrame.MergeReport.report, decodePackValue) : null,
        conflicts: conflictsFrame ? decodeConflictsFromWire(conflictsFrame.Conflicts.conflicts, decodePackValue) : null,
      };
    },
    readConflicts: async (instanceId) => {
      const frames = await requireChannel(instanceId).readConflicts();
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`[DEBUG] program ${pluginId}: readConflicts failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
      const conflictsFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Conflicts: unknown }> => "Conflicts" in frame);
      return conflictsFrame ? decodeConflictsFromWire(conflictsFrame.Conflicts.conflicts, decodePackValue) : [];
    },
    //#endregion 🔖️Merge
    dispose: () => lease.release(),
  };
}
//#endregion 🔖️ChannelAdapter

//#region 🔖️Transaction
/** 🎫️ One member of a resolved transaction — mirrors the Rust host's transaction member shape
 * (contract freeze §5). */
export type TransactionMember = {
  readonly pluginId: string;
  readonly instanceId: number;
  readonly artifactId: string;
};

/** 🎫️ `AppCommand::TransactionPrepare`'s two frozen wire forms (contract freeze §2): owner-mutation
 * (`mutationId`+`payload`, single op) or pre-planned (`preparedOps`+`label`+`origin`, an op list). */
export type TransactionPrepareRequest =
  | { readonly form: "owner"; readonly mutationId: string; readonly payload: Uint8Array }
  | { readonly form: "planned"; readonly preparedOps: readonly Uint8Array[]; readonly label: string; readonly origin: Uint8Array };

export type TransactionPrepareOutcome = { readonly foreign: readonly Uint8Array[]; readonly rejection: Uint8Array | null };

export type TransactionCommitOutcome = { readonly editId: string } | { readonly rejection: Uint8Array };

/** 🧩️ Host-side call into a CONTRIBUTOR plugin's `contributor.artifact-mutation-plan` WIT export
 * (contract freeze §5.3/§6) — a plugin-level component-model export, not an app-instance `exchange`
 * call, so it is injected rather than assumed available on every {@link PluginWasmHandle}. No browser
 * WIT bindgen for the `contributor` interface exists yet (0-D's Wave-0 scope was stub exports only,
 * contract freeze §6) — a caller without a real implementation should omit this constructor argument;
 * {@link TransactionCoordinator} then rejects any CONTRIBUTED step with
 * `transaction.contribution-not-permitted` rather than silently skip it. */
export type ArtifactMutationPlanner = (
  contributorPluginId: string,
  request: { readonly targetPack: Uint8Array; readonly targetSpr: Uint8Array; readonly mutationId: string; readonly payload: Uint8Array },
) => Promise<{ readonly ops: readonly Uint8Array[]; readonly label: string }>;

export type TransactionProposal = {
  readonly initiatorPluginId: string;
  readonly initiatorInstanceId: number;
  readonly initiatorArtifactId: string;
  readonly initiatorArtifactKind: string;
  readonly localOps: readonly Uint8Array[];
  readonly description: string;
  readonly foreign: readonly Uint8Array[];
};

export type TransactionOutcome = { readonly ok: true; readonly txnId: string; readonly editIds: ReadonlyMap<string, string> } | { readonly ok: false; readonly code: string };

/** 🔢️ FNV-1a — used only for the transaction cycle-detection key and this coordinator's own
 * best-effort `MutationOrigin.contributed.payloadHash` (see {@link encodeMutationOrigin}'s doc); never
 * asserted byte-identical against Rust's `PayloadHash`. */
function fnv1aHex(bytes: Uint8Array): string {
  let hash = 0x811c9dc5;
  for (let index = 0; index < bytes.length; index += 1) {
    hash ^= bytes[index]!;
    hash = Math.imul(hash, 0x01000193);
  }
  return (hash >>> 0).toString(16);
}

type WireForeignStep = {
  readonly target: { readonly artifactId: string; readonly artifactKind: string; readonly dialect?: string };
  readonly mutationId: string;
  readonly payload: Uint8Array;
  readonly label: string;
};

/** 📥️ Decodes one wire `foreign`/`TransactionProposal.foreign` element — a `store::pack_rt::encode_wire_value`-encoded
 * `ForeignStep` (contract freeze §1/§2), i.e. exactly what {@link decodePackValue} already mirrors.
 * W0-B's channel codec deliberately keeps these opaque bytes at the framing layer ("this lease never
 * imports or decodes W0-A's ForeignStep type") — the coordinator is exactly the layer that DOES need
 * the decoded shape to route a step. */
function decodeForeignStep(bytes: Uint8Array): WireForeignStep {
  const raw = decodePackValue(bytes) as {
    readonly target: { readonly artifactId: string; readonly artifactKind: string; readonly dialect?: string };
    readonly mutationId: string;
    readonly payload: unknown;
    readonly label: string;
  };
  return {
    target: { artifactId: raw.target.artifactId, artifactKind: raw.target.artifactKind, dialect: raw.target.dialect },
    mutationId: raw.mutationId,
    payload: coerceWireBytes(raw.payload),
    label: raw.label,
  };
}

/** 🧾️ `MutationOrigin` JSON shape (contract freeze §1's `#[serde(rename_all = "camelCase", tag =
 * "kind")]` enum) — encoded through {@link encodePackValue}, the same "any JSON-shaped value" wire
 * mechanism the frozen `origin: Vec<u8>` field uses (contract freeze §2: "origin is the wire-encoded
 * MutationOrigin"). */
type MutationOriginWire =
  | { readonly kind: "owner" }
  | { readonly kind: "contributed"; readonly pluginId: string; readonly mutationId: string; readonly payloadHash: string }
  | { readonly kind: "transaction"; readonly initiator: { readonly artifactId: string; readonly artifactKind: string } };

function encodeMutationOrigin(origin: MutationOriginWire): Uint8Array {
  return encodePackValue(origin);
}

/** 🧯️ Recovers a frozen rejection code from a wire `rejection`/`Error.fault` byte blob — reuses
 * {@link decodeFaultFromWire} since `TransactionPrepared.rejection` and an `AppFrame::Error.fault`
 * share the same `encode_wire_serialized(&fault)` encoding. */
function rejectionCodeFromBytes(bytes: Uint8Array): string {
  const fault = decodeFaultFromWire(Array.from(bytes), decodePackValue);
  return fault?.code ?? "transaction.member-rejected";
}

/** 🔢️ Mirrors Rust `MAX_PLAN_DEPTH`/`MAX_TXN_DEPTH` (contract freeze §1/§5.4). */
export const MAX_TRANSACTION_DEPTH = 8;

/**
 * 🧭️ Browser mirror of the Rust host's `TransactionCoordinator` (contract freeze §5 steps 1-7) —
 * proposal → resolve foreign steps → owner prepare / contributed plan-then-prepare → recurse with
 * depth+cycle guards → all-prepared → commit in reverse discovery order → compensation → group
 * undo/redo, over {@link PluginWasmHandle}'s transaction methods (which frame everything through
 * `AppChannelClient`).
 *
 * Known simplification vs. the Rust host (documented, not silently mishandled): "a second visit
 * appends ops to a member" (contract freeze §5.4) is only supported for steps discovered at the SAME
 * depth (the same parent `foreign` batch) — they're grouped into one `TransactionPrepare` call per
 * member. A step at a LATER depth targeting an ALREADY-prepared member can't be merged into that
 * member's one-and-only prepare call (the guest's §5.9 "one pending transaction per instance" rule
 * would reject a second prepare with `transaction.instance-busy` before this coordinator could even
 * try), so it is treated the same as a cycle (`transaction.cycle`) — fails loud instead of dropping
 * ops silently.
 */
export class TransactionCoordinator {
  private readonly completedGroups = new Map<string, readonly TransactionMember[]>();

  constructor(
    private readonly instances: InstanceDirectory,
    private readonly mutationRouter: ArtifactMutationRouter,
    private readonly plugins: ReadonlyMap<string, PluginWasmHandle>,
    private readonly planContributedMutation?: ArtifactMutationPlanner,
  ) {}

  async run(proposal: TransactionProposal): Promise<TransactionOutcome> {
    const txnId = crypto.randomUUID();
    const initiator: TransactionMember = { pluginId: proposal.initiatorPluginId, instanceId: proposal.initiatorInstanceId, artifactId: proposal.initiatorArtifactId };
    const discoveryOrder: TransactionMember[] = [initiator];
    const preparedInstances = new Set<string>([initiator.artifactId]);
    const seenCycleKeys = new Set<string>();

    const initiatorHandle = this.plugins.get(initiator.pluginId);
    if (!initiatorHandle) return { ok: false, code: "transaction.unknown-target" };

    const initiatorOutcome = await initiatorHandle.transactionPrepare(initiator.instanceId, txnId, {
      form: "planned",
      preparedOps: proposal.localOps,
      label: proposal.description,
      origin: encodeMutationOrigin({ kind: "owner" }),
    });
    if (initiatorOutcome.rejection) return { ok: false, code: rejectionCodeFromBytes(initiatorOutcome.rejection) };

    let frontier: readonly Uint8Array[] = [...proposal.foreign, ...initiatorOutcome.foreign];
    let depth = 1;

    while (frontier.length > 0) {
      if (depth > MAX_TRANSACTION_DEPTH) {
        await this.rollback(txnId, discoveryOrder);
        return { ok: false, code: "transaction.depth-exceeded" };
      }

      type PendingGroup = { readonly member: TransactionMember; readonly ops: Uint8Array[]; contributedFrom: string | null };
      const groups = new Map<string, PendingGroup>();
      const groupOrder: string[] = [];

      for (const stepBytes of frontier) {
        const step = decodeForeignStep(stepBytes);
        const cycleKey = `${step.target.artifactId} ${step.mutationId} ${fnv1aHex(step.payload)}`;
        if (seenCycleKeys.has(cycleKey) || preparedInstances.has(step.target.artifactId)) {
          await this.rollback(txnId, discoveryOrder);
          return { ok: false, code: "transaction.cycle" };
        }
        seenCycleKeys.add(cycleKey);

        const ref: ArtifactInstanceRef | undefined = this.instances.resolve(step.target.artifactId);
        if (!ref) {
          await this.rollback(txnId, discoveryOrder);
          return { ok: false, code: "transaction.unknown-target" };
        }
        const ownership = this.mutationRouter.resolve(step.target.artifactKind, step.mutationId);
        if (!ownership) {
          await this.rollback(txnId, discoveryOrder);
          return { ok: false, code: "transaction.unknown-mutation" };
        }

        let opsToAppend: Uint8Array[];
        let contributedFrom: string | null = null;
        if (ownership.kind === "owner") {
          opsToAppend = [step.payload];
        } else {
          if (!this.planContributedMutation) {
            await this.rollback(txnId, discoveryOrder);
            return { ok: false, code: "transaction.contribution-not-permitted" };
          }
          const targetHandle = this.plugins.get(ref.pluginId);
          const pack = targetHandle?.documentPack(ref.instanceId);
          if (!targetHandle || !pack) {
            await this.rollback(txnId, discoveryOrder);
            return { ok: false, code: "transaction.unknown-target" };
          }
          const planned = await this.planContributedMutation(ownership.pluginId, { targetPack: pack.pack, targetSpr: pack.spr, mutationId: step.mutationId, payload: step.payload });
          opsToAppend = [...planned.ops];
          contributedFrom = ownership.pluginId;
        }

        let group = groups.get(step.target.artifactId);
        if (!group) {
          group = { member: { pluginId: ref.pluginId, instanceId: ref.instanceId, artifactId: step.target.artifactId }, ops: [], contributedFrom: null };
          groups.set(step.target.artifactId, group);
          groupOrder.push(step.target.artifactId);
        }
        group.ops.push(...opsToAppend);
        if (contributedFrom && !group.contributedFrom) group.contributedFrom = contributedFrom;
      }

      const nextFrontier: Uint8Array[] = [];
      for (const artifactId of groupOrder) {
        const group = groups.get(artifactId)!;
        const handle = this.plugins.get(group.member.pluginId);
        if (!handle) {
          await this.rollback(txnId, discoveryOrder);
          return { ok: false, code: "transaction.unknown-target" };
        }
        const origin = group.contributedFrom
          ? encodeMutationOrigin({ kind: "contributed", pluginId: group.contributedFrom, mutationId: "", payloadHash: fnv1aHex(group.ops[0] ?? new Uint8Array()) })
          : encodeMutationOrigin({ kind: "transaction", initiator: { artifactId: initiator.artifactId, artifactKind: proposal.initiatorArtifactKind } });
        const outcome = await handle.transactionPrepare(group.member.instanceId, txnId, { form: "planned", preparedOps: group.ops, label: proposal.description, origin });
        if (outcome.rejection) {
          await this.rollback(txnId, discoveryOrder);
          return { ok: false, code: rejectionCodeFromBytes(outcome.rejection) };
        }
        discoveryOrder.push(group.member);
        preparedInstances.add(group.member.artifactId);
        nextFrontier.push(...outcome.foreign);
      }
      frontier = nextFrontier;
      depth += 1;
    }

    // 🎯️ Phase 2 — commit in reverse discovery order (contract freeze §5.6).
    const editIds = new Map<string, string>();
    for (let index = discoveryOrder.length - 1; index >= 0; index -= 1) {
      const member = discoveryOrder[index]!;
      const handle = this.plugins.get(member.pluginId)!;
      const commitOutcome = await handle.transactionCommit(member.instanceId, txnId);
      if ("rejection" in commitOutcome) {
        // 🎯️ Members strictly deeper in discovery order already committed — undo them. The failing
        // member itself is included in the rollback batch (not just the ones before it): a commit
        // failure other than the guest's own generation-mismatch restore may still leave that
        // member's `pending_transaction` set, and `transactionRollback` on an instance with nothing
        // pending is a safe no-op on the guest side.
        await this.undoMembers(txnId, discoveryOrder.slice(index + 1));
        await this.rollback(txnId, discoveryOrder.slice(0, index + 1));
        return { ok: false, code: "transaction.commit-failed" };
      }
      editIds.set(member.artifactId, commitOutcome.editId);
    }

    this.completedGroups.set(txnId, discoveryOrder);
    return { ok: true, txnId, editIds };
  }

  private async rollback(txnId: string, members: readonly TransactionMember[]): Promise<void> {
    await Promise.all(
      members.map(async (member) => {
        const handle = this.plugins.get(member.pluginId);
        if (!handle) return;
        try {
          await handle.transactionRollback(member.instanceId, txnId);
        } catch (error) {
          console.warn(`[DEBUG] TransactionCoordinator rollback(${member.pluginId}#${member.instanceId}) failed`, error);
        }
      }),
    );
  }

  private async undoMembers(groupId: string, members: readonly TransactionMember[]): Promise<void> {
    await Promise.all(
      members.map(async (member) => {
        const handle = this.plugins.get(member.pluginId);
        if (!handle) return;
        try {
          await handle.transactionUndo(member.instanceId, groupId);
        } catch (error) {
          console.warn(`[DEBUG] TransactionCoordinator undo(${member.pluginId}#${member.instanceId}) failed`, error);
        }
      }),
    );
  }

  /** 🎁️ Group undo — fans `TransactionUndo{groupId}` out to every member of a COMPLETED transaction
   * (contract freeze §5.7). `groupId === txnId` for a transaction this coordinator itself ran
   * (§5.6's "group_id = txn_id"); returns `{ok: false}` for an unknown group instead of throwing,
   * since a caller often can't tell in advance whether a given id was ever a transaction group. */
  async undoGroup(groupId: string): Promise<{ readonly ok: boolean }> {
    const members = this.completedGroups.get(groupId);
    if (!members) return { ok: false };
    await this.undoMembers(groupId, members);
    return { ok: true };
  }

  async redoGroup(groupId: string): Promise<{ readonly ok: boolean }> {
    const members = this.completedGroups.get(groupId);
    if (!members) return { ok: false };
    await Promise.all(
      members.map(async (member) => {
        const handle = this.plugins.get(member.pluginId);
        if (!handle) return;
        try {
          await handle.transactionRedo(member.instanceId, groupId);
        } catch (error) {
          console.warn(`[DEBUG] TransactionCoordinator redo(${member.pluginId}#${member.instanceId}) failed`, error);
        }
      }),
    );
    return { ok: true };
  }
}
//#endregion 🔖️Transaction

//#region 🔖️DependencyOrderedBoot
/** 🎯️ Loads several plugin modules SEQUENTIALLY in dependency order (scout-2 §4: "boot must walk the
 * dependency order from `PluginGraph` instead of relying on array order") — loading a dependency and
 * letting it finish before its dependent starts is what "activate in dependency order" means at this
 * adapter layer; concurrent `Promise.all` loading gives no such guarantee. Entries a dependency-graph
 * fault blocks are reported in `errors` alongside the successfully loaded handles rather than
 * aborting the whole boot (the same fail-soft posture {@link orderPluginRegistryEntries} already
 * documents) — a caller wanting the localized dependency-fault UI turns each error into text via
 * `pluginGraphErrorMessage` (`@semio-tech/framework`). */
export async function loadPluginModulesInDependencyOrder(entries: readonly PluginRegistryEntry[]): Promise<{ readonly handles: readonly PluginWasmHandle[]; readonly errors: readonly PluginGraphError[] }> {
  const { order, errors } = orderPluginRegistryEntries(entries);
  const handles: PluginWasmHandle[] = [];
  for (const entry of order) {
    handles.push(await loadPluginModule(entry.pluginId, entry.moduleUrl));
  }
  return { handles, errors };
}
//#endregion 🔖️DependencyOrderedBoot
//#endregion 🔖️plugin-runtime

//#region 🧪️Tests
// 🧪️ This file has no vitest project of its own (see `📓️w2-b-report.md` for why — the two `.tsx`
// packages that could plausibly own it either don't `includeSource` this path or would create an
// import cycle through `@semio-tech/framework-os`'s own alias). These tests were verified with a
// throwaway `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/🧪️w2-b-plugin-runtime-vitest.config.ts`
// scratch config (kept in the ticket folder per CLAUDE.md) — see the report for the real `bunx
// vitest run` output. Written in the same inline `import.meta.vitest` style every other file in this
// tree uses, so wiring a real project target later is a config change, not a test rewrite.
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  function encodeForeignStepBytes(step: {
    readonly target: { readonly artifactId: string; readonly artifactKind: string };
    readonly mutationId: string;
    readonly payload: readonly number[];
    readonly label: string;
  }): Uint8Array {
    return encodePackValue(step);
  }

  function encodeFaultBytes(code: string): Uint8Array {
    return encodePackValue({ origin: "os", code, severity: "error", message: code, scope: {}, retryable: false });
  }

  type FakeHandleOptions = {
    readonly prepareForeign?: (instanceId: number) => readonly Uint8Array[];
    readonly prepareRejection?: Uint8Array;
    readonly commitRejects?: boolean;
    readonly pack?: { readonly pack: Uint8Array; readonly spr: Uint8Array } | null;
  };

  function fakeHandle(pluginId: string, calls: string[], commitOrder: string[], options: FakeHandleOptions = {}): PluginWasmHandle {
    return {
      pluginId,
      manifest: {} as unknown as PluginManifest,
      createApp: async () => 0,
      destroyApp: async () => {},
      handleAction: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }),
      refreshUi: async () => ({}),
      contextMenu: async () => [],
      readHistory: async () => ({ cursor: 0 }) as unknown as HistoryPatch,
      documentPack: (instanceId) => (options.pack !== undefined ? options.pack : { pack: new Uint8Array([1]), spr: new Uint8Array([instanceId]) }),
      transactionPrepare: async (instanceId, _txnId, _request) => {
        calls.push(`${pluginId}:${instanceId}:prepare`);
        if (options.prepareRejection) return { foreign: [], rejection: options.prepareRejection };
        return { foreign: options.prepareForeign?.(instanceId) ?? [], rejection: null };
      },
      transactionCommit: async (instanceId, _txnId) => {
        if (options.commitRejects) return { rejection: encodeFaultBytes("transaction.commit-failed") };
        commitOrder.push(`${pluginId}:${instanceId}`);
        return { editId: `edit-${pluginId}-${instanceId}` };
      },
      transactionRollback: async (instanceId) => {
        calls.push(`${pluginId}:${instanceId}:rollback`);
      },
      transactionUndo: async (instanceId) => {
        calls.push(`${pluginId}:${instanceId}:undo`);
      },
      transactionRedo: async (instanceId) => {
        calls.push(`${pluginId}:${instanceId}:redo`);
      },
      setMergePolicy: async () => {},
      resolveConflict: async () => ({ mergeReport: null, conflicts: null }),
      readConflicts: async () => [],
      dispose: () => {},
    };
  }

  describe("PluginRuntime TransactionCoordinator", () => {
    it("runs proposal -> prepare x2 -> commit, committing in reverse discovery order", async () => {
      const calls: string[] = [];
      const commitOrder: string[] = [];
      const directory = new InstanceDirectory();
      directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
      const router = new ArtifactMutationRouter();
      router.registerOwner("s.b.doc", "s.b#mutate");
      const plugins = new Map<string, PluginWasmHandle>([
        ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
        ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
      ]);
      const coordinator = new TransactionCoordinator(directory, router, plugins);

      const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [7], label: "duplicate" });
      const outcome = await coordinator.run({
        initiatorPluginId: "a-plugin",
        initiatorInstanceId: 10,
        initiatorArtifactId: "artifact-a",
        initiatorArtifactKind: "s.a.doc",
        localOps: [new Uint8Array([1])],
        description: "duplicate widget",
        foreign: [foreign],
      });

      expect(outcome.ok).toBe(true);
      if (!outcome.ok) return;
      expect(calls).toEqual(["a-plugin:10:prepare", "b-plugin:20:prepare"]);
      // 🎯️ Reverse discovery order: initiator (a) discovered first, foreign target (b) discovered
      // second — commit visits b before a (contract freeze §5.6).
      expect(commitOrder).toEqual(["b-plugin:20", "a-plugin:10"]);
      expect(outcome.editIds.get("artifact-a")).toBe("edit-a-plugin-10");
      expect(outcome.editIds.get("artifact-b")).toBe("edit-b-plugin-20");
    });

    it("undoGroup fans TransactionUndo out to every member of a completed transaction", async () => {
      const calls: string[] = [];
      const commitOrder: string[] = [];
      const directory = new InstanceDirectory();
      directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
      const router = new ArtifactMutationRouter();
      router.registerOwner("s.b.doc", "s.b#mutate");
      const plugins = new Map<string, PluginWasmHandle>([
        ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
        ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
      ]);
      const coordinator = new TransactionCoordinator(directory, router, plugins);
      const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [7], label: "x" });
      const outcome = await coordinator.run({
        initiatorPluginId: "a-plugin",
        initiatorInstanceId: 10,
        initiatorArtifactId: "artifact-a",
        initiatorArtifactKind: "s.a.doc",
        localOps: [new Uint8Array([1])],
        description: "x",
        foreign: [foreign],
      });
      expect(outcome.ok).toBe(true);
      if (!outcome.ok) return;

      calls.length = 0;
      const undoResult = await coordinator.undoGroup(outcome.txnId);
      expect(undoResult.ok).toBe(true);
      expect(new Set(calls)).toEqual(new Set(["a-plugin:10:undo", "b-plugin:20:undo"]));

      const unknownUndo = await coordinator.undoGroup("not-a-real-group");
      expect(unknownUndo.ok).toBe(false);
    });

    it("rolls back a commit-failed transaction: undoes what already committed, rolls back the rest", async () => {
      const calls: string[] = [];
      const commitOrder: string[] = [];
      const directory = new InstanceDirectory();
      directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.doc" });
      directory.register("artifact-c", { pluginId: "c-plugin", instanceId: 30, artifactKind: "s.doc" });
      const router = new ArtifactMutationRouter();
      router.registerOwner("s.doc", "s#mutate");
      const plugins = new Map<string, PluginWasmHandle>([
        ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
        ["b-plugin", fakeHandle("b-plugin", calls, commitOrder, { commitRejects: true })],
        ["c-plugin", fakeHandle("c-plugin", calls, commitOrder)],
      ]);
      const coordinator = new TransactionCoordinator(directory, router, plugins);
      const foreignB = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.doc" }, mutationId: "s#mutate", payload: [1], label: "x" });
      const foreignC = encodeForeignStepBytes({ target: { artifactId: "artifact-c", artifactKind: "s.doc" }, mutationId: "s#mutate", payload: [2], label: "x" });
      const outcome = await coordinator.run({
        initiatorPluginId: "a-plugin",
        initiatorInstanceId: 10,
        initiatorArtifactId: "artifact-a",
        initiatorArtifactKind: "s.doc",
        localOps: [new Uint8Array([1])],
        description: "x",
        foreign: [foreignB, foreignC],
      });
      expect(outcome).toEqual({ ok: false, code: "transaction.commit-failed" });
      // discovery order was [a, b, c]; commit visits c (succeeds) then b (fails) — c is already
      // committed so it gets undone, a (never reached) gets rolled back alongside the failing b.
      expect(commitOrder).toEqual(["c-plugin:30"]);
      expect(calls).toContain("c-plugin:30:undo");
      expect(calls).toContain("a-plugin:10:rollback");
      expect(calls).toContain("b-plugin:20:rollback");
    });

    it("reaches transaction.unknown-target when the initiator plugin has no registered handle", async () => {
      const coordinator = new TransactionCoordinator(new InstanceDirectory(), new ArtifactMutationRouter(), new Map());
      const outcome = await coordinator.run({
        initiatorPluginId: "missing-plugin",
        initiatorInstanceId: 1,
        initiatorArtifactId: "artifact-a",
        initiatorArtifactKind: "s.a.doc",
        localOps: [],
        description: "x",
        foreign: [],
      });
      expect(outcome).toEqual({ ok: false, code: "transaction.unknown-target" });
    });

    it("reaches transaction.unknown-target when InstanceDirectory has no entry for the foreign target", async () => {
      const calls: string[] = [];
      const commitOrder: string[] = [];
      const plugins = new Map<string, PluginWasmHandle>([["a-plugin", fakeHandle("a-plugin", calls, commitOrder)]]);
      const coordinator = new TransactionCoordinator(new InstanceDirectory(), new ArtifactMutationRouter(), plugins);
      const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-unknown", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [1], label: "x" });
      const outcome = await coordinator.run({
        initiatorPluginId: "a-plugin",
        initiatorInstanceId: 10,
        initiatorArtifactId: "artifact-a",
        initiatorArtifactKind: "s.a.doc",
        localOps: [],
        description: "x",
        foreign: [foreign],
      });
      expect(outcome).toEqual({ ok: false, code: "transaction.unknown-target" });
    });

    it("reaches transaction.unknown-mutation when the router has no entry for a foreign step", async () => {
      const calls: string[] = [];
      const commitOrder: string[] = [];
      const directory = new InstanceDirectory();
      directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
      const plugins = new Map<string, PluginWasmHandle>([
        ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
        ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
      ]);
      const coordinator = new TransactionCoordinator(directory, new ArtifactMutationRouter(), plugins);
      const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#nope", payload: [1], label: "x" });
      const outcome = await coordinator.run({
        initiatorPluginId: "a-plugin",
        initiatorInstanceId: 10,
        initiatorArtifactId: "artifact-a",
        initiatorArtifactKind: "s.a.doc",
        localOps: [],
        description: "x",
        foreign: [foreign],
      });
      expect(outcome).toEqual({ ok: false, code: "transaction.unknown-mutation" });
    });

    it("reaches transaction.contribution-not-permitted when a contributed mutation has no planner wired", async () => {
      const calls: string[] = [];
      const commitOrder: string[] = [];
      const directory = new InstanceDirectory();
      directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
      const router = new ArtifactMutationRouter();
      router.registerContributed(
        "s.b.doc",
        "aec-building",
        "b-plugin",
        { mutationId: "s.b#aec-building:add-room", semantics: { verb: "add", entity: "room", kind: "add-room", record: "Room" }, schemaVersion: 1, algorithmVersion: 1 },
        true,
      );
      const plugins = new Map<string, PluginWasmHandle>([
        ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
        ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
      ]);
      const coordinator = new TransactionCoordinator(directory, router, plugins); // no planner injected
      const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#aec-building:add-room", payload: [1], label: "x" });
      const outcome = await coordinator.run({
        initiatorPluginId: "a-plugin",
        initiatorInstanceId: 10,
        initiatorArtifactId: "artifact-a",
        initiatorArtifactKind: "s.a.doc",
        localOps: [],
        description: "x",
        foreign: [foreign],
      });
      expect(outcome).toEqual({ ok: false, code: "transaction.contribution-not-permitted" });
    });

    it("plans and prepares a contributed mutation using the target's cached document pack", async () => {
      const calls: string[] = [];
      const commitOrder: string[] = [];
      const directory = new InstanceDirectory();
      directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
      const router = new ArtifactMutationRouter();
      router.registerContributed(
        "s.b.doc",
        "aec-building",
        "b-plugin",
        { mutationId: "s.b#aec-building:add-room", semantics: { verb: "add", entity: "room", kind: "add-room", record: "Room" }, schemaVersion: 1, algorithmVersion: 1 },
        true,
      );
      const seenPlanRequests: { readonly targetPack: Uint8Array; readonly targetSpr: Uint8Array }[] = [];
      const targetPack = { pack: new Uint8Array([9, 9]), spr: new Uint8Array([8]) };
      const plugins = new Map<string, PluginWasmHandle>([
        ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
        ["b-plugin", fakeHandle("b-plugin", calls, commitOrder, { pack: targetPack })],
      ]);
      const coordinator = new TransactionCoordinator(directory, router, plugins, async (contributorPluginId, request) => {
        seenPlanRequests.push({ targetPack: request.targetPack, targetSpr: request.targetSpr });
        expect(contributorPluginId).toBe("aec-building");
        return { ops: [new Uint8Array([42])], label: "aec-building add-room" };
      });
      const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#aec-building:add-room", payload: [1], label: "x" });
      const outcome = await coordinator.run({
        initiatorPluginId: "a-plugin",
        initiatorInstanceId: 10,
        initiatorArtifactId: "artifact-a",
        initiatorArtifactKind: "s.a.doc",
        localOps: [],
        description: "x",
        foreign: [foreign],
      });
      expect(outcome.ok).toBe(true);
      expect(seenPlanRequests).toEqual([{ targetPack: targetPack.pack, targetSpr: targetPack.spr }]);
    });

    it("reaches transaction.cycle when the same (artifact, mutation, payload) step repeats", async () => {
      const calls: string[] = [];
      const commitOrder: string[] = [];
      const directory = new InstanceDirectory();
      directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
      const router = new ArtifactMutationRouter();
      router.registerOwner("s.b.doc", "s.b#mutate");
      const plugins = new Map<string, PluginWasmHandle>([
        ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
        ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
      ]);
      const coordinator = new TransactionCoordinator(directory, router, plugins);
      const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [1], label: "x" });
      const outcome = await coordinator.run({
        initiatorPluginId: "a-plugin",
        initiatorInstanceId: 10,
        initiatorArtifactId: "artifact-a",
        initiatorArtifactKind: "s.a.doc",
        localOps: [],
        description: "x",
        foreign: [foreign, foreign],
      });
      expect(outcome).toEqual({ ok: false, code: "transaction.cycle" });
    });

    it("reaches transaction.depth-exceeded when a foreign-step chain runs past MAX_TRANSACTION_DEPTH", async () => {
      const calls: string[] = [];
      const commitOrder: string[] = [];
      const directory = new InstanceDirectory();
      const chainLength = MAX_TRANSACTION_DEPTH + 4;
      for (let index = 1; index <= chainLength; index += 1) {
        directory.register(`artifact-${index}`, { pluginId: "chain-plugin", instanceId: index, artifactKind: "s.chain.doc" });
      }
      const router = new ArtifactMutationRouter();
      router.registerOwner("s.chain.doc", "s.chain#step");
      const chainHandle: PluginWasmHandle = {
        pluginId: "chain-plugin",
        manifest: {} as unknown as PluginManifest,
        createApp: async () => 0,
        destroyApp: async () => {},
        handleAction: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }),
        refreshUi: async () => ({}),
        contextMenu: async () => [],
        readHistory: async () => ({ cursor: 0 }) as unknown as HistoryPatch,
        documentPack: () => ({ pack: new Uint8Array([1]), spr: new Uint8Array([2]) }),
        transactionPrepare: async (instanceId) => {
          calls.push(`chain:${instanceId}:prepare`);
          const next = instanceId + 1;
          if (next > chainLength) return { foreign: [], rejection: null };
          return {
            foreign: [encodeForeignStepBytes({ target: { artifactId: `artifact-${next}`, artifactKind: "s.chain.doc" }, mutationId: "s.chain#step", payload: [next], label: "x" })],
            rejection: null,
          };
        },
        transactionCommit: async (instanceId) => {
          commitOrder.push(`chain:${instanceId}`);
          return { editId: `edit-${instanceId}` };
        },
        transactionRollback: async (instanceId) => {
          calls.push(`chain:${instanceId}:rollback`);
        },
        transactionUndo: async () => {},
        transactionRedo: async () => {},
        setMergePolicy: async () => {},
        resolveConflict: async () => ({ mergeReport: null, conflicts: null }),
        readConflicts: async () => [],
        dispose: () => {},
      };
      const plugins = new Map<string, PluginWasmHandle>([
        ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
        ["chain-plugin", chainHandle],
      ]);
      const coordinator = new TransactionCoordinator(directory, router, plugins);
      const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-1", artifactKind: "s.chain.doc" }, mutationId: "s.chain#step", payload: [1], label: "x" });
      const outcome = await coordinator.run({
        initiatorPluginId: "a-plugin",
        initiatorInstanceId: 10,
        initiatorArtifactId: "artifact-a",
        initiatorArtifactKind: "s.a.doc",
        localOps: [],
        description: "x",
        foreign: [foreign],
      });
      expect(outcome).toEqual({ ok: false, code: "transaction.depth-exceeded" });
    });

    it("passes a member's TransactionPrepared.rejection code straight through — instance-busy, generation-mismatch, and the member-rejected default all reachable", async () => {
      const directory = new InstanceDirectory();
      const router = new ArtifactMutationRouter();
      for (const code of ["transaction.instance-busy", "transaction.generation-mismatch", "not-a-real-fault-code"]) {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const plugins = new Map<string, PluginWasmHandle>([["a-plugin", fakeHandle("a-plugin", calls, commitOrder, { prepareRejection: code === "not-a-real-fault-code" ? new Uint8Array([255, 255, 255]) : encodeFaultBytes(code) })]]);
        const coordinator = new TransactionCoordinator(directory, router, plugins);
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          description: "x",
          foreign: [],
        });
        const expectedCode = code === "not-a-real-fault-code" ? "transaction.member-rejected" : code;
        expect(outcome).toEqual({ ok: false, code: expectedCode });
      }
    });
  });

  describe("PluginRuntime documentPack/transaction wire adapter", () => {
    it("adaptPluginHandle's documentPack/transactionPrepare/transactionCommit/transactionRollback/transactionUndo/transactionRedo frame through AppChannelClient", async () => {
      const { decodeAppCommand, encodeAppFrame } = await import("@semio-tech/framework-os");
      const seenCommands: unknown[] = [];
      const fakeLease = {
        handle: {
          manifest: async () => encodePackValue({ pluginId: "b-plugin", label: "B", version: "1.0.0", apps: [], workflows: [], examples: [] }),
          createApp: async () => 20,
          destroyApp: async () => {},
          exchange: async (_instanceId: number, frames: Uint8Array[]) => {
            const commands = frames.map((frame) => decodeAppCommand(frame));
            seenCommands.push(...commands);
            return commands.map((command) => {
              if ("transactionPrepare" in command) {
                return encodeAppFrame({ transactionPrepared: { txn_id: command.transactionPrepare.txn_id, foreign: [], rejection: [] } });
              }
              if ("transactionCommit" in command) {
                return encodeAppFrame({ transactionCommitted: { txn_id: command.transactionCommit.txn_id, edit_id: "edit-1" } });
              }
              if ("ReadDocument" in command) {
                return encodeAppFrame({ Document: { in_reply_to: command.ReadDocument.seq, pack: [5, 5], spr: [6], ops: "" } });
              }
              return encodeAppFrame({ Done: { in_reply_to: 0 } });
            });
          },
          dispose: () => {},
        },
        release: () => {},
      };
      const handle = await adaptPluginHandle("b-plugin", fakeLease);
      const instanceId = await handle.createApp("app-b");

      // 📦️ documentPack() is null until the underlying AppChannelClient has observed a document —
      // this adapter only ever surfaces the CACHE, it never issues a document round trip itself.
      expect(handle.documentPack(instanceId)).toBeNull();

      const prepareOutcome = await handle.transactionPrepare(instanceId, "txn-1", { form: "owner", mutationId: "s.b#mutate", payload: new Uint8Array([1]) });
      expect(prepareOutcome).toEqual({ foreign: [], rejection: null });

      const commitOutcome = await handle.transactionCommit(instanceId, "txn-1");
      expect(commitOutcome).toEqual({ editId: "edit-1" });

      await handle.transactionRollback(instanceId, "txn-2");
      await handle.transactionUndo(instanceId, "grp-1");
      await handle.transactionRedo(instanceId, "grp-1");

      expect(seenCommands).toEqual([
        { transactionPrepare: { seq: 1, txn_id: "txn-1", mutation_id: "s.b#mutate", payload: [1], prepared_ops: [], label: "", origin: [] } },
        { transactionCommit: { seq: 2, txn_id: "txn-1" } },
        { transactionRollback: { seq: 3, txn_id: "txn-2" } },
        { transactionUndo: { seq: 4, group_id: "grp-1" } },
        { transactionRedo: { seq: 5, group_id: "grp-1" } },
      ]);
    });

    it("documentPack() reflects the cache after loadAppDocumentPack() — the adapter reads the SAME live channel it just loaded through", async () => {
      const { decodeAppCommand, encodeAppFrame } = await import("@semio-tech/framework-os");
      const fakeLease = {
        handle: {
          manifest: async () => encodePackValue({ pluginId: "b-plugin", label: "B", version: "1.0.0", apps: [], workflows: [], examples: [] }),
          createApp: async () => 20,
          destroyApp: async () => {},
          exchange: async (_instanceId: number, frames: Uint8Array[]) => {
            const commands = frames.map((frame) => decodeAppCommand(frame));
            return commands.map((command) => ("LoadDocument" in command ? encodeAppFrame({ Done: { in_reply_to: command.LoadDocument.seq } }) : encodeAppFrame({ Done: { in_reply_to: 0 } })));
          },
          dispose: () => {},
        },
        release: () => {},
      };
      const handle = await adaptPluginHandle("b-plugin", fakeLease);
      const instanceId = await handle.createApp("app-b");
      expect(handle.documentPack(instanceId)).toBeNull();
      await handle.loadAppDocumentPack?.(instanceId, new Uint8Array([1, 2]), new Uint8Array([3]));
      expect(handle.documentPack(instanceId)).toEqual({ pack: new Uint8Array([1, 2]), spr: new Uint8Array([3]) });
    });
  });
}
//#endregion 🧪️Tests
