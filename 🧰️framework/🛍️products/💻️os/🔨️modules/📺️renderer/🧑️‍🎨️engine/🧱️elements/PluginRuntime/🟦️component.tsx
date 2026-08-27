/// <reference types="vitest/importMeta" />
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/PluginRuntime/component.tsx
/** @emoji 🐚️ `PluginRuntime` — the `PluginWasmHandle` binary-channel adapter (`loadPluginModule`/
 * `adaptPluginHandle`) that wraps a leased `framework-core` plugin wasm module's `enqueue`/`outcomes`
 * turn ABI behind the wider action/command/refreshUi/contextMenu/document-sync surface the rest of the
 * shell calls, plus the `AppChannelClient` frame-reassembly helpers (`🔖️ChannelAdapter`) that back it.
 *
 * MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H1-react, design-runtime.md §1/§3): `loadPluginModule`
 * no longer leases one Worker per plugin (`acquirePluginModule`/`PluginModuleLease`, both deleted
 * in packet H2 — `📓️terra-H2-web-shard-report.md`). It drives a real actor through the kernel's
 * `ActivationRegistry` (manifest-only activation, LRU suspend/resume) over `ShardClient` (bounded
 * shard-worker pool, `actorId`-multiplexed) — see `🔖️ActorAdapter` below. `enqueue()` on the raw
 * handle this file constructs submits one `app-command` event per queued frame through
 * `ShardClient.turn` and demuxes the resulting `TurnResult.effects` for the `SendMessage{Shell{
 * instance}}` entries `⚛️reactor/🦀️component.rs`'s `route_app_frame` wraps every non-`UiPatch`
 * `AppFrame` reply in, pushing them onto the handle's `outcomes` broadcast (`📌️important.md`'s "Replace, never wrap"
 * list — the old handle exposed one synchronous per-call method instead) — everything else in this file (`AppChannelClient`, `adaptPluginHandle`'s
 * command/transaction/merge methods) is unchanged, since it only ever spoke `AppCommand`/`AppFrame`
 * bytes through that one channel seam and does not care what backs it.
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
  AppChannelRequestSequence,
  type AppFrameValue,
  decodeAppFrame,
  decodeConflictsFromWire,
  decodeFaultFromWire,
  decodeMergeReportFromWire,
  decodeMutationEnvelopesPack,
  decodePackValue,
  encodePackValue,
  faultDisplayMessage,
} from "@semio-tech/framework-os";
import type { ArtifactPresencePeer } from "@semio-tech/framework-replication";
import { type BuiltNode, type UiNodeRecord, type UiPatchOp, type UiSnapshot } from "@semio-tech/framework";
import { applyUiPatch, emptyUiDocumentState, type UiDocumentState } from "../UiDocumentStore/🟦️component.tsx";
import {
  ActivationRegistry,
  type ActivationReason,
  createTurnOutcomeBroadcast,
  type PluginWasmHandle as KernelPluginWasmHandle,
  type TurnOutcome,
} from "../../../../../../../🔨️modules/🎠️kernel/🟦️component.ts";
import {
  createShardCommandIngressPages,
  ShardClient,
  type ShardBudget,
  type ShardCommandIngressPage,
  type ShardEventEnvelope,
  type ShardWorkerLike,
} from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts";
import { TurnScheduler, type Lane } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️turn-scheduler.ts";
import { type PluginManifest, type ViewModel } from "../Shell/🟦️component.tsx";
import { SEGMENTED_DOWNLOAD_MARKER_PREFIX } from "../SegmentedDownload/🟦️component.ts";
// #endregion 🔌️Adapters

//#region 🔖️plugin-runtime

export type PluginWasmHandle = {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  /** 🧵 Drains one operation-owned export chunk; `undefined` is the sealed end marker. */
  readonly takeSegmentedDownloadChunk: (instanceId: number, operationId: bigint) => Promise<Uint8Array | undefined>;
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
  /** 👥️ `interaction` (contract-freeze §C7.6) is the app's own declared-broadcast selection/hover
   * slice — `encode_presence_interaction` output, empty when no domain is declared or broadcasting
   * right now. */
  readonly ephemeralSnapshot?: (
    instanceId: number,
  ) => Promise<{ readonly presence: readonly number[]; readonly presenceGeneration: number; readonly transientGeneration: number; readonly interaction: readonly number[] } | null>;
  /** 👥️ Pushes the document-wide presence roster into this instance's plugin app — the ONLY plugin
   * ingress for peers (contract-freeze §C7.6). `ownColor` is this actor's own hub-assigned palette
   * index (`null` for a folder-only session with no hub); `peers` is the whole roster with the
   * caller's own actor already dropped. */
  readonly pushPresence?: (instanceId: number, ownColor: number | null, peers: readonly ArtifactPresencePeer[]) => Promise<void>;
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

/** 🧮️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): `min(hardwareConcurrency-1,
 * 4)` — the SAME physical bound `getShardClient`'s own worker-pool `shardCount` uses (design-runtime.md
 * §1 `ShardTable`), factored out once so it never drifts between the two call sites that both mean
 * "how many wasm-boundary hops can genuinely run at once on this device": the shard pool itself, and
 * {@link loadPluginModulesInDependencyOrder}'s per-level boot concurrency (that function's own doc has
 * the "why reuse this exact number" reasoning). Falls back to `5` (so the clamp lands on `4`) when
 * `navigator.hardwareConcurrency` is unavailable (SSR/test), matching `getShardClient`'s own fallback. */
function poolConcurrency(): number {
  const hardwareConcurrency = typeof navigator !== "undefined" && typeof navigator.hardwareConcurrency === "number" ? navigator.hardwareConcurrency : 5;
  return Math.max(1, Math.min(hardwareConcurrency - 1, 4));
}

/** 🚑️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): `ShardClientOptions.onShardLost`'s
 * real production wiring — before this packet, the callback here only `console.error`'d (a shard's
 * whole actor roster silently stopped receiving turns, undetectable short of reading the console).
 * `ActivationRegistry.handleShardLost` (`🎠️kernel/🟦️component.ts`, already coordinator-verified) is
 * the REAL restore path: it bumps each affected actor's generation, cancels its stale queue, and
 * `resume()`s it from its last checkpoint on a freshly rebuilt shard. Split out from
 * {@link buildShardClientOptions} (rather than inlined as an arrow there) purely so a test can call it
 * directly against a monkey-patched {@link sharedActivationRegistry} without constructing a real
 * `ShardClient`/`Worker`. */
function handlePluginShardLost(shardIndex: number, actorIds: readonly string[]): void {
  console.error(`[DEBUG] PluginRuntime: shard ${shardIndex} lost, restoring actors: ${actorIds.join(", ")}`);
  getActivationRegistry().handleShardLost(shardIndex, actorIds);
}

/** 🎭️ Split out from {@link getShardClient} so a test can construct a REAL `ShardClient` (exercising
 * its actual lane/heartbeat/dispose machinery) against a FAKE `createWorker` — `getShardClient` itself
 * is untestable in isolation since it hardcodes a real DOM `Worker`, which this suite's `jsdom`
 * environment doesn't provide. `createWorker` defaults to the real `Worker` constructor for every
 * production call (`getShardClient` never passes an override). */
function buildShardClientOptions(createWorker: () => ShardWorkerLike = () => new Worker(SHARD_WORKER_URL, { type: "module" }) as unknown as ShardWorkerLike): {
  readonly shardCount: number;
  readonly createWorker: () => ShardWorkerLike;
  readonly onActorTrap: (actorId: string, message: string) => void;
  readonly onShardLost: (shardIndex: number, actorIds: readonly string[]) => void;
} {
  return {
    shardCount: poolConcurrency(),
    // 🎭️ A real DOM `Worker` satisfies `ShardWorkerLike` structurally at runtime (same claim
    // `🌐plugin-web-materialize.ts`'s own doc makes) — the cast only bridges `onmessage`/`onerror`'s
    // wider native `MessageEvent`/`ErrorEvent` handler types down to the interface's minimal
    // `{data: unknown}`/`unknown` shape, which a `MessageEvent`/`ErrorEvent` handler always satisfies.
    createWorker,
    onActorTrap: (actorId, message) => console.error(`[DEBUG] PluginRuntime: actor ${actorId} trapped: ${message}`),
    onShardLost: handlePluginShardLost,
  };
}

let sharedShardClient: ShardClient | null = null;
function getShardClient(): ShardClient {
  if (sharedShardClient) return sharedShardClient;
  sharedShardClient = new ShardClient(buildShardClientOptions());
  // 🚑️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): before this packet, neither
  // `checkHeartbeats` nor `pollHeartbeatSab` had a production caller anywhere in the repo (`ShardClient`'s
  // own doc on `startWatchdog` — "the watchdog's whole failure ladder was wired but nothing in
  // production ever turned the crank"), so a wedged shard went undetected forever in the real app.
  // Self-ticks at `startWatchdog`'s own default cadence (`heartbeatTimeoutMs`) — see that method's doc.
  sharedShardClient.startWatchdog();
  return sharedShardClient;
}

/** 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime) — metrics-publisher ownership
 * decision: `autoStartMetricsPublisher` is left at its default (`false`) here, DELIBERATELY. This
 * function is the constructor call site, yes, but it is not the DELIBERATE-choice construction site
 * `ActivationRegistryOptions.autoStartMetricsPublisher`'s own doc asks for — `getActivationRegistry` is
 * a lazy, shared, module-wide singleton created on the FIRST `loadPluginModule` call from anywhere (a
 * plugin boot, not a user action), with no relationship to whether anyone is actually watching
 * `metricsBus` right now. Turning it on here would start a real 2 Hz `setInterval` the moment the
 * first plugin loads, for the lifetime of the tab, regardless of whether the task-manager window
 * (`TaskManager/🟦️component.tsx`'s own header doc: still registrar-only, unmounted work) — the
 * registry's own ONLY real subscriber — is even open. The deliberate choice belongs to whichever
 * construction site mounts that consumer (`ShellHost`, registrar-only per this packet's lease) —
 * turning this on here would silently change a shared default for every other current/future
 * consumer of this singleton, exactly what `autoStartMetricsPublisher` exists to prevent. */
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
  readonly surface?: { readonly instance?: number; readonly surface?: string };
  readonly kind?: string;
  readonly revision?: number | bigint;
  readonly baseRevision?: number | bigint;
  readonly ops?: readonly WireVariant[];
};

type WireTurnResult = {
  readonly uiPatches: readonly WireUiPatch[];
  readonly effects: readonly WireVariant[];
  readonly nextWake: number | null;
  readonly status?: unknown;
  readonly commandIngress?: WireVariant;
};

/** 📥️ Defensive parse of `ShardClient.turn()`'s opaque `unknown` return (typed opaque at that
 * module's own public boundary — see its header doc) into the fields this file needs, tolerating a
 * missing/differently-shaped field rather than throwing mid-turn. */
function coerceTurnResult(raw: unknown): WireTurnResult {
  const record = (raw && typeof raw === "object" ? raw : {}) as Record<string, unknown>;
  const uiPatches = Array.isArray(record.uiPatches) ? (record.uiPatches as WireUiPatch[]) : [];
  const effects = Array.isArray(record.effects) ? (record.effects as WireVariant[]) : [];
  const nextWake = typeof record.nextWake === "number" ? record.nextWake : null;
  const status = record.status;
  const commandIngress = record.commandIngress && typeof record.commandIngress === "object" ? (record.commandIngress as WireVariant) : undefined;
  return { uiPatches, effects, nextWake, status, commandIngress };
}

/** 🧯️ Decodes the scalar WIT command-ingress fault envelope while retaining readable kernel
 * rejection codes used by pre-decode validation paths. */
function commandIngressFaultDisplay(status: WireVariant | undefined): string {
  if (status?.tag !== "fault" || !status.val || typeof status.val !== "object") return "unknown fault";
  const fault = (status.val as { readonly fault?: unknown }).fault;
  const raw = fault && typeof fault === "object" && "val" in fault ? (fault as { readonly val?: unknown }).val : fault;
  const bytes = coerceWireBytes(raw);
  const decoded = faultDisplayMessage(Array.from(bytes), decodePackValue);
  if (decoded !== "unknown fault") return decoded;
  const text = new TextDecoder().decode(bytes).trim();
  return text.length > 0 ? text : decoded;
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

//#region 📬️TypedOperationResult
const TYPED_OPERATION_PAGE_MAGIC = new TextEncoder().encode("semio.typed-operation-page.v1\0");
const TYPED_OPERATION_ACK_MAGIC = new TextEncoder().encode("semio.typed-operation-ack.v1\0");

function typedOperationResult(effect: WireVariant): { readonly acknowledgement: ShardEventEnvelope; readonly lane: number; readonly payload: Uint8Array; readonly operation: bigint } | null {
  if (effect.tag !== "send-message") return null;
  const value = effect.val as { readonly target?: WireVariant; readonly payload?: unknown } | undefined;
  if (value?.target?.tag !== "shell" || value.payload === undefined) return null;
  const bytes = coerceWireBytes(value.payload);
  if (!TYPED_OPERATION_PAGE_MAGIC.every((byte, index) => bytes[index] === byte)) return null;
  const body = bytes.subarray(TYPED_OPERATION_PAGE_MAGIC.length);
  if (body.length < 30) throw new Error("typed-operation result header is truncated");
  const view = new DataView(body.buffer, body.byteOffset, body.byteLength);
  const receiver = view.getUint32(0, true);
  const lane = body[25]!;
  const length = view.getUint32(26, true);
  if (Number(value.target.val) !== receiver || lane > 11 || length > 4_096 || body.length !== 30 + length) throw new Error("typed-operation result violates its receiver, lane, or page authority");
  const ack = new Uint8Array(TYPED_OPERATION_ACK_MAGIC.length + 25);
  ack.set(TYPED_OPERATION_ACK_MAGIC);
  ack.set(body.subarray(0, 25), TYPED_OPERATION_ACK_MAGIC.length);
  return { acknowledgement: { kind: "message", payload: { source: { tag: "shell", val: String(receiver) }, payload: Array.from(ack) } }, lane, payload: body.subarray(30), operation: view.getBigUint64(4, true) };
}

function typedOperationAcknowledgements(result: WireTurnResult): ShardEventEnvelope[] {
  return result.effects.flatMap((effect) => {
    const page = typedOperationResult(effect);
    return page ? [page.acknowledgement] : [];
  });
}

function consumeTypedOperationEffects(effects: readonly WireVariant[]): WireVariant[] {
  return effects.flatMap((effect) => {
    const page = typedOperationResult(effect);
    if (!page) return [effect];
    if (page.lane === 11) throw new Error(`typed-operation failed: ${new TextDecoder().decode(page.payload)}`);
    if (page.lane !== 9) return [];
    const metadata: unknown = JSON.parse(new TextDecoder().decode(page.payload));
    if (!Array.isArray(metadata) || typeof metadata[0] !== "string" || typeof metadata[1] !== "string" || (metadata[2] !== null && metadata[2] !== "base64" && metadata[2] !== "identity")) throw new Error("typed-operation download metadata is invalid");
    return [{ tag: "download-media-export", val: { filename: metadata[0], mimeType: metadata[1], data: String(page.operation), encoding: `${SEGMENTED_DOWNLOAD_MARKER_PREFIX}${metadata[2] ?? "identity"}` } }];
  });
}
//#endregion 📬️TypedOperationResult

//#region 🔖️RetainedUiPatch
/** 🩹️ `kernel::PatchOp`, TS twin restricted to what `⚛️reactor/🩹️patches/🦀️component.rs`'s
 * `PatchTracker` actually emits this wave (its own doc: "full-body only — every dirty surface emits
 * one `PatchOp::Replace` at the root path"). `path` is `list<u32>` at the WIT boundary (an empty
 * array for the root).
 *
 * MIGRATION (semantic UI contract, ticket 26/08/20): the old recursive `UiNode` this file's `node`
 * payload used to carry no longer exists — `🛂️manifest/🟦️component.ts`'s hand-written mirror was
 * deleted in favor of `semio-framework-ui-contract`'s flat, id-keyed `UiSnapshot`/`UiNodeRecord`
 * (`UiDocumentStore`'s header doc). A "whole-body replace" is now a whole `UiSnapshot` (root pointer +
 * flat node table), not a single recursive node; an "insert one child" is a single `UiNodeRecord` row.
 * The kernel/actor WIT boundary this file bridges has not flipped onto the new wire tags yet (owned by
 * the MICROKERNEL program's `sdk-flip`/`wit-flip` packets, forbidden here) — this is therefore a
 * type-level migration only, matching this whole boundary's own doc ("UNVERIFIED against a real
 * compiled artifact... no plugin has migrated onto `world actor` yet"). */
export function decodeWirePatchOps(ops: readonly WireVariant[]): readonly UiPatchOp[] {
  const decoded: UiPatchOp[] = [];
  for (const op of ops) {
    const val = (op.val ?? {}) as Record<string, unknown>;
    switch (op.tag) {
      case "upsert":
        decoded.push({ type: "upsert", ...normalizeWireUiNodeRecord(decodePackValue(coerceWireBytes(val.node))) });
        break;
      case "set-component":
        decoded.push({ type: "setComponent", id: wireNatural(val.node), component: decodePackValue(coerceWireBytes(val.component)) as Extract<UiPatchOp, { type: "setComponent" }>["component"] });
        break;
      case "set-layout":
        decoded.push({ type: "setLayout", id: wireNatural(val.node), layout: decodePackValue(coerceWireBytes(val.layout)) as Extract<UiPatchOp, { type: "setLayout" }>["layout"] });
        break;
      case "set-activity": {
        const activity = decodePackValue(coerceWireBytes(val.activity)) as Pick<Extract<UiPatchOp, { type: "setActivity" }>, "activity" | "disabled">;
        decoded.push({ type: "setActivity", id: wireNatural(val.node), activity: activity.activity, disabled: activity.disabled });
        break;
      }
      case "set-children":
        decoded.push({ type: "setChildren", id: wireNatural(val.node), children: Array.isArray(val.children) ? val.children.map(wireNatural) : [] });
        break;
      case "set-style":
        decoded.push({ type: "setStyle", id: wireNatural(val.node), style: decodePackValue(coerceWireBytes(val.style)) as Extract<UiPatchOp, { type: "setStyle" }>["style"] });
        break;
      case "set-accessibility":
        decoded.push({ type: "setAccessibility", id: wireNatural(val.node), accessibility: decodePackValue(coerceWireBytes(val.accessibility)) as Extract<UiPatchOp, { type: "setAccessibility" }>["accessibility"] });
        break;
      case "set-bindings":
        decoded.push({ type: "setBindings", id: wireNatural(val.node), bindings: decodePackValue(coerceWireBytes(val.bindings)) as Extract<UiPatchOp, { type: "setBindings" }>["bindings"] });
        break;
      case "set-menu":
        decoded.push({ type: "setMenu", id: wireNatural(val.node), menu: decodePackValue(coerceWireBytes(val.menu)) as Extract<UiPatchOp, { type: "setMenu" }>["menu"] });
        break;
      case "remove":
        decoded.push({ type: "remove", id: wireNatural(op.val) });
        break;
      case "set-root":
        decoded.push({ type: "setRoot", id: wireNatural(op.val) });
        break;
      default:
        break;
    }
  }
  return decoded;
}

function wireNatural(raw: unknown): number {
  const value = Number(raw ?? 0);
  if (!Number.isSafeInteger(value) || value < 0) throw new Error(`[DEBUG] actor WIT integer is outside the JavaScript safe range: ${String(raw)}`);
  return value;
}

function normalizeWireUiNodeRecord(raw: unknown): UiNodeRecord {
  const record = raw as Partial<UiNodeRecord>;
  return {
    ...(record as UiNodeRecord),
    id: wireNatural(record.id),
    disabled: record.disabled ?? false,
    transition: record.transition ?? null,
    bindings: Array.isArray(record.bindings) ? record.bindings : [],
    menu: record.menu ?? null,
    children: Array.isArray(record.children) ? record.children.map(wireNatural) : [],
  };
}

/** 🗄️ The retained per-actor document — a `UiDocumentStore`-shaped state, not the store class itself
 * (this file has no per-node React subscribers to serve; it only needs the flat table to hash/forward
 * to `refreshUi` callers). Reuses `UiDocumentStore`'s own `uiDocumentStateFromSnapshot` so this file
 * and the React tree apply the identical algorithm to the identical wire shape — never a second,
 * drifting reimplementation. */
export type RetainedSurface = UiDocumentState;

/**
 * @emoji 🖼️ H1-react (design-runtime.md §1 `SceneStore` / packet brief item 2) — reconciles one
 * `UiPatch`'s ops onto `previous` (the last body this file retained for the surface), so the UI
 * thread reads an already-reconciled tree instead of awaiting a plugin turn. Reuses the transactional
 * `UiDocumentStore` patch applicator so revision checks, graph validation, quotas, and every semantic
 * operation stay identical to the renderer's subscribed document store.
 */
export function applyUiPatchToRetained(
  previous: RetainedSurface | null,
  patch: { readonly surface?: string; readonly revision: number | bigint; readonly baseRevision: number | bigint; readonly ops: readonly UiPatchOp[] },
): { readonly surface: RetainedSurface | null; readonly desynced: boolean } {
  const surfaceId = patch.surface ?? previous?.surface ?? "window";
  if (previous && previous.surface !== surfaceId) return { surface: previous, desynced: true };
  const state = previous ?? emptyUiDocumentState(surfaceId);
  const applied = applyUiPatch(state, {
    surface: surfaceId,
    revision: wireNatural(patch.revision),
    baseRevision: wireNatural(patch.baseRevision),
    ops: [...patch.ops],
  });
  return applied.ok ? { surface: applied.state, desynced: false } : { surface: previous, desynced: true };
}

/** 🔁️ Rebuilds a plain `UiSnapshot` (nodes flattened back to an array) from a retained
 * `UiDocumentState` — needed only where a `Map` must cross a `JSON.stringify`/hash boundary; the
 * store's own consumers read `.nodes` directly and never need this round trip. */
function retainedSurfaceToSnapshot(surface: RetainedSurface): UiSnapshot {
  return { surface: surface.surface, revision: surface.revision, root: surface.root ?? 0, nodes: [...surface.nodes.values()], layoutEpoch: 0n };
}

function retainedSurfaceHash(snapshot: UiSnapshot): string {
  const json = JSON.stringify(snapshot, (_key, value) => (typeof value === "bigint" ? value.toString() : value));
  return fnv1aHex(new TextEncoder().encode(json)) + ":" + String(snapshot.revision);
}

function retainedSurfaceToBuiltNode(surface: RetainedSurface): BuiltNode | null {
  if (surface.root === null) return null;
  const build = (id: number): BuiltNode => {
    const record = surface.nodes.get(id);
    if (!record) throw new Error(`[DEBUG] retained UI surface ${surface.surface} references missing node ${id}`);
    return {
      key: record.key,
      component: record.component,
      layout: record.layout,
      style: record.style,
      activity: record.activity,
      disabled: record.disabled,
      accessibility: record.accessibility,
      bindings: record.bindings,
      menu: record.menu,
      children: record.children.map(build),
    };
  };
  return build(surface.root);
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
    case "load-document":
      return { loadDocument: { pack: Array.from(coerceWireBytes(val.pack)), spr: Array.from(coerceWireBytes(val.spr)) } };
    case "download-media-export":
      return { downloadMediaExport: { filename: str("filename"), mimeType: str("mimeType"), data: str("data"), encoding: val.encoding as string | undefined } };
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
 * §2). `runQueuedTurn` fills this on every turn; `performInvocation` drains it right after its own
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
 * queued concurrent per-call requests transparently — this is that same guarantee's replacement.
 *
 * 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): the ORIGINAL implementation
 * here was a raw `Map<actorId, Promise>` FIFO chain — unbounded (a pointer-move burst queued 200 deep
 * rather than coalescing), lane-blind (an urgent action waited behind a stale background probe), and
 * un-cancellable. Both `serializePerActor` (below, a generic per-key run-this-thunk-serially utility —
 * KEPT with its exact existing signature/contract since `@semio-tech/framework-renderer-react`'s
 * `📦️index.tsx` re-exports it and `🧪️index.test.ts` asserts that generic contract directly, outside
 * this packet's lease) and {@link submitPluginTurn} (this file's own internal turn-dispatch seam, used
 * by every real `submitTurn` call site below) now both sit on top of the landed `TurnScheduler` —
 * bounded mailbox, lane priority, latest-wins coalescing, `cancelQueued`/`teardownActor` — instead of
 * hand-rolling a second queue. Two SEPARATE `TurnScheduler` instances (not one shared): `TurnScheduler`
 * is transport/payload-agnostic by design (its own header doc), and `serializePerActor`'s payload
 * (an arbitrary thunk) and `submitPluginTurn`'s (wire events + a request/response waiter list, needed
 * because a mailbox's `coalesced` collapse silently drops the SUPERSEDED envelope's own callback
 * otherwise) are genuinely different shapes — the same "one dedicated `TurnScheduler` per consumer"
 * pattern `ActivationRegistry` already uses for its own internal `enqueueTurn`, not a reinvention.
 *
 * `ActivationRegistry.enqueueTurn` was considered and REJECTED for `submitPluginTurn`'s job: its
 * `onTurnResult(actorId, result)` fires once per REGISTRY, not once per caller, so it has no way to
 * hand a coalesced-away caller (or any specific caller, once more than one turn can be pending per
 * actor) back its own turn's result — exactly what `handleAction`/`refreshUi`/etc.'s `Promise<...>`
 * return contracts require. A dedicated scheduler whose `runTurn` seam we control ourselves is what
 * makes per-caller correlation (via `PluginTurnPayload.waiters`, see below) possible at all. */

//#region 🔖️GenericThunkQueue
interface ThunkTurnPayload {
  readonly run: () => Promise<unknown>;
  readonly resolve: (value: unknown) => void;
  readonly reject: (error: unknown) => void;
}

/** 🧮️ A generous but FINITE cap — large enough that no known real `serializePerActor` caller (this
 * file's own `submitTurn` no longer uses it; the react-target package re-export is its only consumer)
 * ever approaches it, small enough that a caller who genuinely never lets an actor's queue drain gets
 * an honest rejection instead of the unbounded heap growth this file existed to fix. */
const SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY = 256;

let sharedThunkScheduler: TurnScheduler<ThunkTurnPayload, undefined> | null = null;
function getThunkScheduler(): TurnScheduler<ThunkTurnPayload, undefined> {
  sharedThunkScheduler ??= new TurnScheduler<ThunkTurnPayload, undefined>({
    mailboxCapacity: SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY,
    budgetFor: () => undefined,
    runTurn: async (_actorId, payload) => {
      try {
        payload.resolve(await payload.run());
      } catch (error) {
        payload.reject(error);
      }
    },
  });
  return sharedThunkScheduler;
}

/** 🚦 Generic per-`actorId` serializer: `run` never starts for a given `actorId` before the previous
 * `run` for that SAME id has settled (resolved OR rejected — a fault never wedges the queue, matching
 * this file's pre-existing contract), while independent `actorId`s run fully concurrently. Backed by
 * {@link getThunkScheduler} — bounded, so a caller that floods one `actorId` gets a rejected promise
 * once {@link SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY} is exceeded rather than growing memory forever. */
export function serializePerActor<T>(actorId: string, run: () => Promise<T>): Promise<T> {
  return new Promise<T>((resolve, reject) => {
    const backpressure = getThunkScheduler().enqueue(actorId, { lane: "Interactive", payload: { run, resolve: resolve as (value: unknown) => void, reject } });
    if (backpressure.kind === "rejected") reject(new Error(`[DEBUG] serializePerActor: actor ${actorId}'s queue is full (>${SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY} pending turns) — rejected rather than growing unbounded`));
  });
}

/** 📥️ Holds the actor's complete paged command-ingress sequence as one serialized unit. Every
 * direct poll operation uses the same key so redraw/completion turns cannot consume the command's
 * retained pending/terminal status and response effects before its channel caller observes them. */
export function serializeCommandIngressForActor<T>(actorId: string, run: () => Promise<T>): Promise<T> {
  return serializePerActor(`command-ingress:${actorId}`, run);
}
//#endregion 🔖️GenericThunkQueue

//#region 🔖️PluginTurnScheduler
interface PluginTurnWaiter {
  readonly resolve: (result: WireTurnResult) => void;
  readonly reject: (error: unknown) => void;
}

/** ✉️ `events` is mutable ON PURPOSE: a coalesced call replaces it in place (see {@link submitPluginTurn})
 * rather than asking the mailbox to juggle a second envelope object, which is what would otherwise
 * silently discard the superseded call's own `waiters`. `coalesceMapKey`, when set, is cleared from
 * {@link pendingCoalescedTurns} the instant this payload's turn actually dispatches (in `runTurn`,
 * before the `await`) — a call arriving AFTER that point must start a fresh coalescing cycle, not
 * append to one that's already running or already finished. */
interface PluginTurnPayload {
  events: readonly ShardEventEnvelope[];
  readonly commandPage?: ShardCommandIngressPage;
  readonly waiters: PluginTurnWaiter[];
  readonly coalesceMapKey?: string;
}

/** 🧮️ Matches `ActivationRegistry`'s own `DEFAULT_TURN_MAILBOX_CAPACITY` — no reason for this file's
 * per-actor turn queue to be shaped differently from the kernel's own default. */
const PLUGIN_TURN_MAILBOX_CAPACITY = 32;

const pendingCoalescedTurns = new Map<string, PluginTurnPayload>();
const tearingDownPluginActors = new Set<string>();

let sharedPluginTurnScheduler: TurnScheduler<PluginTurnPayload, ShardBudget> | null = null;
/** 🧵️ Reads `getShardClient()` INSIDE `runTurn` (not once at construction) purely so a test can swap
 * the module-private `sharedShardClient` for a fake between calls without this scheduler ever pinning
 * itself to whichever shard client happened to exist first — in production there is only ever one. */
function getPluginTurnScheduler(): TurnScheduler<PluginTurnPayload, ShardBudget> {
  sharedPluginTurnScheduler ??= new TurnScheduler<PluginTurnPayload, ShardBudget>({
    mailboxCapacity: PLUGIN_TURN_MAILBOX_CAPACITY,
    budgetFor: () => DEFAULT_SHARD_BUDGET,
    runTurn: async (actorId, payload, budget) => {
      if (payload.coalesceMapKey) pendingCoalescedTurns.delete(payload.coalesceMapKey);
      try {
        const result = coerceTurnResult(await getShardClient().turn(actorId, payload.events, budget, payload.commandPage));
        for (const waiter of payload.waiters) waiter.resolve(result);
      } catch (error) {
        for (const waiter of payload.waiters) waiter.reject(error);
        if (!tearingDownPluginActors.has(actorId)) throw error;
      }
    },
    onTurnError: (actorId, error) => console.error(`[DEBUG] PluginRuntime: turn failed for actor ${actorId}`, error),
  });
  return sharedPluginTurnScheduler;
}

/**
 * 🚦 This file's own internal turn-dispatch seam — replaces the old unbounded `actorTurnQueue` chain.
 * `lane` prioritizes across an actor's own pending turns (`"Interactive"` for anything a caller awaits
 * a specific reply from — `runQueuedTurn`/`createApp`/`completeExtensionInvoke` all use it below —
 * `"UserVisible"` for {@link loadPluginModule}'s opportunistic `refreshUi` probe, so a real command
 * always preempts a mere redraw poll). `coalesceKey`, when passed, collapses a burst of same-key calls
 * for the SAME actor into the single latest one — every caller in the burst (not just the winner) still
 * gets the SAME final result, via {@link PluginTurnPayload.waiters} rather than the mailbox's own
 * envelope-replacement (which has no callback for the superseded call).
 *
 * 🚧️ Honest gap: `BoundedMailbox.enqueue`'s `dropped` backpressure (this actor's mailbox at capacity,
 * evicting the lowest-priority NONEMPTY lane below the incoming one) has no callback for the evicted
 * envelope — a caller whose turn was silently evicted this way would never see its promise settle.
 * Mitigated, not eliminated, by lane discipline: every call site below uses only two lanes
 * (`"Interactive"`/`"UserVisible"`), and `"UserVisible"` traffic is deduplicated to at most one pending
 * envelope per actor by `pendingCoalescedTurns` before it ever reaches the mailbox — so an eviction can
 * only happen if a single actor accumulates more than `PLUGIN_TURN_MAILBOX_CAPACITY` (32) genuinely
 * distinct `"Interactive"` turns, which no real call site here does. Flagged rather than silently
 * risked, per this repo's own "must not assume" rule — see `📓️terra-web-plugin-runtime-report.md`
 * `## honest gaps`.
 */
function submitPluginTurn(actorId: string, events: readonly ShardEventEnvelope[], lane: Lane, coalesceKey?: string, commandPage?: ShardCommandIngressPage): Promise<WireTurnResult> {
  const scheduler = getPluginTurnScheduler();
  return new Promise<WireTurnResult>((resolve, reject) => {
    const waiter: PluginTurnWaiter = { resolve, reject };
    if (coalesceKey !== undefined) {
      const mapKey = `${actorId} ${coalesceKey}`;
      const pending = pendingCoalescedTurns.get(mapKey);
      if (pending) {
        pending.events = events; // 🎯️ latest-wins: the mailbox still holds THIS SAME payload object.
        pending.waiters.push(waiter);
        return;
      }
      const payload: PluginTurnPayload = { events, waiters: [waiter], coalesceMapKey: mapKey, commandPage };
      pendingCoalescedTurns.set(mapKey, payload);
      const backpressure = scheduler.enqueue(actorId, { lane, coalesce: coalesceKey, payload });
      if (backpressure.kind === "rejected") {
        pendingCoalescedTurns.delete(mapKey);
        reject(new Error(`[DEBUG] PluginRuntime: actor ${actorId}'s turn queue is full — rejected rather than growing unbounded`));
      }
      return;
    }
    const payload: PluginTurnPayload = { events, waiters: [waiter], commandPage };
    const backpressure = scheduler.enqueue(actorId, { lane, payload });
    if (backpressure.kind === "rejected") reject(new Error(`[DEBUG] PluginRuntime: actor ${actorId}'s turn queue is full — rejected rather than growing unbounded`));
  });
}

function teardownPluginActor(actorId: string): void {
  const fault = new Error(`plugin actor ${actorId} disposed`);
  tearingDownPluginActors.add(actorId);
  getPluginTurnScheduler().teardownActor(actorId, (payload) => {
    if (payload.coalesceMapKey) pendingCoalescedTurns.delete(payload.coalesceMapKey);
    for (const waiter of payload.waiters) waiter.reject(fault);
  });
  getThunkScheduler().teardownActor(`command-ingress:${actorId}`, (payload) => payload.reject(fault));
  setTimeout(() => tearingDownPluginActors.delete(actorId), 0);
}

function wireTurnStatusTag(status: unknown): string {
  const raw =
    typeof status === "string"
      ? status
      : status && typeof status === "object" && "tag" in status
        ? String((status as { readonly tag?: unknown }).tag ?? "")
        : "";
  return raw.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase();
}

function wirePatchSurfaceId(patch: WireUiPatch): string | null {
  return patch.surface ? retainedSurfaceId(wireNatural(patch.surface.instance), patch.surface.surface ?? "window") : null;
}

function hasRequiredUiPatches(results: readonly WireTurnResult[], requiredSurfaceIds?: ReadonlySet<string>): boolean {
  if (!requiredSurfaceIds) return results.some((result) => result.uiPatches.length > 0);
  if (requiredSurfaceIds.size === 0) return true;
  const published = new Set(results.flatMap((result) => result.uiPatches.map(wirePatchSurfaceId).filter((surface): surface is string => surface !== null)));
  return [...requiredSurfaceIds].every((surface) => published.has(surface));
}

const PLUGIN_UI_CONTINUATION_LIMIT = 4_096;
const PLUGIN_UI_CONTINUATION_BATCH_SIZE = 8;

async function yieldPluginUiContinuation(): Promise<void> {
  await new Promise<void>((resolve) => setTimeout(resolve, 0));
}

/** 🔄️ Drives a reactor-owned continuation until its requested patch set is published or it
 * quiesces. UI reconciliation is deliberately incremental: the turn that marks surfaces dirty may
 * publish them across multiple MoreWork frames. A supplied empty `requiredSurfaceIds` means every
 * requested surface is already retained, so an unchanged refresh needs no continuation at all.
 * Accepted patches are acknowledged between turns to release bounded publication capacity. */
async function settlePluginTurn(actorId: string, initial: WireTurnResult, lane: Lane, requiredSurfaceIds?: ReadonlySet<string>, acceptPatches?: (result: WireTurnResult) => readonly ShardEventEnvelope[], drainOperations = false): Promise<WireTurnResult> {
  const results: WireTurnResult[] = [initial];
  const acknowledge = (result: WireTurnResult) => [...(acceptPatches?.(result) ?? []), ...typedOperationAcknowledgements(result)];
  const hasWork = () => (drainOperations || !hasRequiredUiPatches(results, requiredSurfaceIds)) && wireTurnStatusTag(results.at(-1)?.status) === "more-work";
  let acknowledgements = acknowledge(initial);
  for (let continuation = 0; (acknowledgements.length > 0 || hasWork()) && continuation < PLUGIN_UI_CONTINUATION_LIMIT; continuation += 1) {
    const continued = await submitPluginTurn(actorId, acknowledgements, lane);
    results.push(continued);
    acknowledgements = acknowledge(continued);
    if ((continuation + 1) % PLUGIN_UI_CONTINUATION_BATCH_SIZE === 0 && hasWork()) {
      await yieldPluginUiContinuation();
    }
  }
  if (acknowledgements.length > 0 || hasWork()) {
    const published = results.flatMap((result) => result.uiPatches.map(wirePatchSurfaceId).filter((surface): surface is string => surface !== null));
    throw new Error(
      `[DEBUG] PluginRuntime: actor ${actorId} did not publish its requested UI surfaces within ${PLUGIN_UI_CONTINUATION_LIMIT} continuations ` +
        `(required=${JSON.stringify([...(requiredSurfaceIds ?? [])])}, published=${JSON.stringify(published)}, ` +
        `effects=${results.reduce((count, result) => count + result.effects.length, 0)}, status=${wireTurnStatusTag(results.at(-1)?.status)})`,
    );
  }
  if (requiredSurfaceIds?.size && !hasRequiredUiPatches(results, requiredSurfaceIds)) {
    const published = new Set(results.flatMap((result) => result.uiPatches.map(wirePatchSurfaceId).filter((surface): surface is string => surface !== null)));
    const missing = [...requiredSurfaceIds].filter((surface) => !published.has(surface));
    throw new Error(`[DEBUG] PluginRuntime: actor ${actorId} stopped without publishing requested UI surfaces (missing=${JSON.stringify(missing)}, status=${wireTurnStatusTag(results.at(-1)?.status)})`);
  }
  return {
    uiPatches: results.flatMap((result) => result.uiPatches),
    effects: consumeTypedOperationEffects(results.flatMap((result) => result.effects)),
    nextWake: [...results].reverse().find((result) => result.nextWake !== null)?.nextWake ?? null,
    status: results.at(-1)?.status,
    commandIngress: results.at(-1)?.commandIngress,
  };
}
async function settleAcknowledgedPluginTurns(actorId: string, results: readonly WireTurnResult[], acknowledgements: readonly ShardEventEnvelope[]): Promise<WireTurnResult> {
  const initial: WireTurnResult = {
    uiPatches: [],
    effects: [],
    nextWake: null,
    commandIngress: results.at(-1)?.commandIngress,
    status: results.at(-1)?.status,
  };
  const continued = await settlePluginTurn(actorId, initial, "Interactive", new Set(), (turn) => turn === initial ? acknowledgements : patchAckEvents(retainTurnUiPatches(actorId, turn)), true);
  return {
    ...continued,
    uiPatches: [...results.flatMap((turn) => turn.uiPatches), ...continued.uiPatches],
    effects: consumeTypedOperationEffects([...results.flatMap((turn) => turn.effects), ...continued.effects]),
    nextWake: continued.nextWake ?? [...results].reverse().find((turn) => turn.nextWake !== null)?.nextWake ?? null,
  };
}
//#endregion 🔖️PluginTurnScheduler

/** 🖼️ Last reconciled UI document per actor and exact `(instance, body-key)` surface. Keyed by
 * `actorId` so a suspend+resume (fresh checkpoint restore) naturally starts a new entry. */
const retainedWindowByActor = new Map<string, Map<string, RetainedSurface>>();

function pluginSurfaceRef(instance: number, bodyKey: string): { readonly instance: number; readonly surface: string } {
  return { instance, surface: bodyKey };
}

function retainedSurfaceId(instance: number, bodyKey: string): string {
  return `${instance}:${bodyKey}`;
}

/** 🪟️ Requests each authored window or panel body once through the shared surface lifecycle. */
function uiRefreshBodyKeys(request: PluginUiRefreshRequest): string[] {
  return [...new Set([...(request.windows ?? []), ...(request.panels ?? [])].map((target) => target.bodyKey ?? target.key))];
}

/** 📬️ Projects retained bodies back to their requested window and panel keys. */
function retainedUiRefreshResponse(instanceId: number, request: PluginUiRefreshRequest, retained: ReadonlyMap<string, RetainedSurface>, effects: readonly WireVariant[] = []): PluginUiRefreshResponse {
  const project = (targets: PluginUiRefreshRequest["windows"]) => (targets ?? []).flatMap((target) => {
    const surface = retained.get(retainedSurfaceId(instanceId, target.bodyKey ?? target.key));
    const value = surface && retainedSurfaceToBuiltNode(surface);
    return surface && value ? [{ key: target.key, hash: retainedSurfaceHash(retainedSurfaceToSnapshot(surface)), value }] : [];
  });
  const requestedEffects: Effect[] = [];
  for (const effect of effects) {
    const bytes = shellFrameBytes(effect, instanceId);
    if (bytes) {
      const frame = decodeAppFrame(bytes);
      if ("Error" in frame) {
        const fault = decodeFaultFromWire(frame.Error.fault, decodePackValue);
        if (fault) throw new SemioFaultError(fault);
        throw new Error(`refresh failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
      }
    } else {
      const requested = wireEffectToFriendly(effect);
      if (requested) requestedEffects.push(requested);
    }
  }
  return { windows: project(request.windows), panels: project(request.panels), requestedEffects };
}

function retainedSurfacesForActor(actorId: string): Map<string, RetainedSurface> {
  const existing = retainedWindowByActor.get(actorId);
  if (existing) return existing;
  const created = new Map<string, RetainedSurface>();
  retainedWindowByActor.set(actorId, created);
  return created;
}

//#endregion 🔖️ActorAdapter

/** 🚦️ Resolves the small descriptor request before starting the shard-worker module graph, keeping
 * cold browser connection capacity available for the request that lets plugin loading complete. */
export async function resolveDescriptorBeforeRuntime<TManifest, TRuntime>(loadDescriptor: () => Promise<TManifest>, initializeRuntime: () => TRuntime): Promise<{ readonly manifest: TManifest; readonly runtime: TRuntime }> {
  const manifest = await loadDescriptor();
  return { manifest, runtime: initializeRuntime() };
}

/** 🐚️ Acquires a real actor through `ActivationRegistry`/`ShardClient` (replacing the deleted
 * `acquirePluginModule`/`PluginModuleLease` per-plugin Worker lease — design-runtime.md §3) and
 * adapts it exactly like the old wasm-Worker handle: `dispose()` disposes this instance's worker-side
 * actor entry via `ShardClient.dispose` (not a `LeasePool` release — there is no shared module lease
 * to refcount anymore, one actor belongs to exactly one instance). */
export async function loadPluginModule(pluginId: string, moduleUrl: string, signal?: AbortSignal): Promise<PluginWasmHandle> {
  const { manifest, runtime: registry } = await resolveDescriptorBeforeRuntime(() => fetchDescriptorManifest(pluginId, moduleUrl, signal), getActivationRegistry);
  registry.registerManifest({ pluginId, moduleUrl, caps: [] });
  const shardClient = getShardClient();
  const actorIdByInstance = new Map<number, string>();
  let eventSeq = 0;
  const requireActorId = (instanceId: number): string => {
    const actorId = actorIdByInstance.get(instanceId);
    if (!actorId) throw new Error(`[DEBUG] program ${pluginId}: no actor for instance ${instanceId} (createApp not called, or already destroyed)`);
    return actorId;
  };
  /** 🚦 `lane`/`coalesceKey` forward to {@link submitPluginTurn} — see that function's own doc for the
   * lane-assignment reasoning. `registry.touch(actorId)` refreshes this actor's LRU position on every
   * turn (its own doc: "call on every turn, not just activation"); turns dispatch through this file's
   * own {@link submitPluginTurn} rather than `ActivationRegistry.enqueueTurn` (see that decision's
   * write-up above `serializePerActor`), so nothing else would ever call it. */
  const submitTurn = (actorId: string, events: readonly ShardEventEnvelope[], options?: { readonly lane?: Lane; readonly coalesceKey?: string; readonly commandPage?: ShardCommandIngressPage }): Promise<WireTurnResult> => {
    registry.touch(actorId);
    return submitPluginTurn(actorId, events, options?.lane ?? "Interactive", options?.coalesceKey, options?.commandPage);
  };

  /** 📤️📥️ Backs {@link KernelPluginWasmHandle.enqueue}/`.outcomes` (see this file's own header doc).
   * One broadcast per `loadPluginModule` call, matching the handle's own
   * lifetime: every instance this handle ever `createApp`s shares it, and each instance's
   * `AppChannelClient` (`💻️os/🟦️component.ts`) filters to its own `instanceId`. */
  const turnOutcomes = createTurnOutcomeBroadcast<TurnOutcome>();

  /** 🔀️ The real turn-submission body `enqueue` used to run synchronously inline and return — now
   * run fire-and-forget from `enqueue`, pushing its settlement onto {@link turnOutcomes} instead of
   * resolving a caller's promise directly (a caller's own promise now lives one layer up, in
   * `AppChannelClient.sendCommand`, correlated against this broadcast). A turn-submission failure
   * (NOT an `AppFrame::Error` — that is still an ordinary decoded frame) becomes an `error`-shaped
   * outcome rather than an uncaught rejection, since nothing here awaits this function's own promise. */
  const runQueuedTurn = async (instanceId: number, events: readonly Uint8Array[]): Promise<void> => {
    try {
      const actorId = requireActorId(instanceId);
      const result = await serializeCommandIngressForActor(actorId, async (): Promise<WireTurnResult> => {
        const results: WireTurnResult[] = [];
        let acknowledgements: readonly ShardEventEnvelope[] = [];
        const acceptTurn = (turn: WireTurnResult) => {
          results.push(turn);
          acknowledgements = [...patchAckEvents(retainTurnUiPatches(actorId, turn)), ...typedOperationAcknowledgements(turn)];
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
          for (const commandPage of pages) acceptTurn(await submitTurn(actorId, acknowledgements, { commandPage }));
          let terminal = results.at(-1)?.commandIngress?.tag;
          const observedStatuses = new Set([terminal ?? "missing"]);
          for (let continuation = 0; terminal !== "command-complete" && continuation < 1_024; continuation += 1) {
            if (terminal === "fault") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress fault: ${commandIngressFaultDisplay(results.at(-1)?.commandIngress)}`);
            if (terminal === "backpressure") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress backpressure after serialized submission`);
            const continued = await submitTurn(actorId, acknowledgements);
            acceptTurn(continued);
            terminal = continued.commandIngress?.tag;
            observedStatuses.add(terminal ?? "missing");
          }
          if (terminal !== "command-complete") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress did not complete within 1024 continuations (observed statuses: ${[...observedStatuses].join(", ")})`);
        }
        return settleAcknowledgedPluginTurns(actorId, results, acknowledgements);
      });
      const outFrames: Uint8Array[] = [];
      const leftover: WireVariant[] = [];
      for (const effect of result.effects) {
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

  const handle: KernelPluginWasmHandle = {
    manifest: async () => encodePackValue(manifest),
    createApp: async (appId) => {
      const instanceId = nextGlobalInstanceId;
      nextGlobalInstanceId += 1;
      const actorId = `${pluginId}#${instanceId}`;
      actorIdByInstance.set(instanceId, actorId);
      await registry.activate(pluginId, actorId, "manual" satisfies ActivationReason);
      eventSeq += 1;
      await settlePluginTurn(
        actorId,
        await submitTurn(actorId, [
          {
            kind: "instance-open",
            payload: { instance: instanceId, appId, actor: currentPluginRuntimeActor, config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})) },
          },
        ]),
        "Interactive",
        new Set(),
        (turn) => patchAckEvents(retainTurnUiPatches(actorId, turn)),
      );
      return instanceId;
    },
    destroyApp: async (instanceId) => {
      const actorId = actorIdByInstance.get(instanceId);
      if (!actorId) return;
      actorIdByInstance.delete(instanceId);
      retainedWindowByActor.delete(actorId);
      pendingTurnEffects.delete(instanceId);
      teardownPluginActor(actorId);
      shardClient.dispose(actorId);
    },
    takeSegmentedDownloadChunk: (instanceId, operationId) => shardClient.takeSegmentedDownloadChunk(requireActorId(instanceId), instanceId, operationId),
    enqueue: (instanceId, events) => {
      void runQueuedTurn(instanceId, events);
    },
    outcomes: turnOutcomes.stream,
    dispose: () => {
      for (const actorId of actorIdByInstance.values()) {
        retainedWindowByActor.delete(actorId);
        teardownPluginActor(actorId);
        shardClient.dispose(actorId);
      }
      actorIdByInstance.clear();
      turnOutcomes.complete();
    },
  };

  const richHandle = await adaptPluginHandle(pluginId, { handle, release: handle.dispose });

  /** 🔁️ H1-react item 2 — window-body refresh no longer goes through `AppCommand::RefreshUi`
   * (deleted, channel v12): it submits `Event::SurfaceVisible` directly and reads back whatever this
   * SAME turn's `TurnResult.uiPatches` produced (or the retained tree if nothing changed — the
   * `PatchTracker` on the guest side emits nothing for an unchanged body). Every requested window is
   * identified by its schema-owned body key; continuations drain until all surfaces missing from the
   * retained actor state publish their first patch. Panels use the same authored-body surface path. */
  const refreshUi = async (instanceId: number, request: PluginUiRefreshRequest): Promise<PluginUiRefreshResponse> => {
    const bodyKeys = uiRefreshBodyKeys(request);
    if (bodyKeys.length === 0) return {};
    const actorId = requireActorId(instanceId);
    eventSeq += 1;
    const retainedBeforeRefresh = retainedWindowByActor.get(actorId);
    const missingSurfaceIds = new Set(bodyKeys.map((bodyKey) => retainedSurfaceId(instanceId, bodyKey)).filter((surfaceId) => !retainedBeforeRefresh?.has(surfaceId)));
    // 🎯️ H1-react (terra-web-plugin-runtime) — a pointer-move-driven redraw burst hits this call
    // repeatedly for the SAME actor; "UserVisible" (below "Interactive", above "Background") lets a
    // real command preempt it, and the `"surface-visible"` coalesce key collapses the burst to the
    // single latest probe rather than queuing every intermediate one (see `submitPluginTurn`'s doc).
    const result = await serializeCommandIngressForActor(actorId, async () => {
      const settled = await settlePluginTurn(
        actorId,
        await submitTurn(
          actorId,
          bodyKeys.map((bodyKey) => ({ kind: "surface-visible", payload: { surface: pluginSurfaceRef(instanceId, bodyKey) } })),
          { lane: "UserVisible", coalesceKey: "surface-visible" },
        ),
        "UserVisible",
        missingSurfaceIds,
        (turn) => patchAckEvents(retainTurnUiPatches(actorId, turn)),
        true,
      );
      return settled;
    });
    const retained = retainedWindowByActor.get(actorId);
    return retainedUiRefreshResponse(instanceId, request, retained ?? new Map(), result.effects);
  };

  /** 🔁️ H1-react item 2 ("finish the invokeExtension branch") — the real `req`-correlated completion
   * `ShellHost/🟦️component.tsx`'s `applyHostEffects` used to only log loudly about. Submits
   * `Event::Completed{req, outcome}` on the ORIGINATING instance's own actor so its `RequestRegistry`
   * resumes the parked future (design-abi.md §2's "the SDK resumes the awaiting future on
   * `event.completed`"). */
  const completeExtensionInvoke = async (instanceId: number, req: number, outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array }): Promise<void> => {
    const actorId = requireActorId(instanceId);
    await serializeCommandIngressForActor(actorId, () =>
      submitTurn(actorId, [
        {
          kind: "completed",
          payload: { req, outcome: "ok" in outcome ? { tag: "ok", val: Array.from(outcome.ok) } : { tag: "fault", val: Array.from(outcome.fault) } },
        },
      ]),
    );
  };

  return { ...richHandle, refreshUi, completeExtensionInvoke };
}

function patchAckEvents(uiPatches: readonly WireUiPatch[]): ShardEventEnvelope[] {
  return uiPatches.flatMap((patch) => (patch.surface ? [{ kind: "patch-ack", payload: { surface: patch.surface, revision: patch.revision ?? 0 } }] : []));
}

function applyRetainedWindowPatches(actorId: string, uiPatches: readonly WireUiPatch[]): WireUiPatch[] {
  const retained = retainedSurfacesForActor(actorId);
  const accepted: WireUiPatch[] = [];
  for (const patch of uiPatches) {
    const ops = decodeWirePatchOps(patch.ops ?? []);
    const surfaceId = wirePatchSurfaceId(patch) ?? "window";
    const previous = retained.get(surfaceId) ?? null;
    const { surface, desynced } = applyUiPatchToRetained(previous, { surface: surfaceId, revision: patch.revision ?? 0, baseRevision: patch.baseRevision ?? 0, ops });
    if (desynced) {
      console.warn(`[DEBUG] applyRetainedWindowPatches: actor ${actorId} desynced (unrecognized op shape or stale baseRevision) — keeping the previously retained body`);
      continue;
    }
    if (surface) {
      retained.set(surfaceId, surface);
      accepted.push(patch);
    }
  }
  return accepted;
}

/** 🪟️ Captures every render-producing turn, including instance-open: a later
 * surface-visible probe may legitimately emit no patch when the guest tree is unchanged, so
 * dropping the first turn's patch would leave the shell on its loading placeholder forever. */
function retainTurnUiPatches(actorId: string, result: Pick<WireTurnResult, "uiPatches">): WireUiPatch[] {
  return result.uiPatches.length > 0 ? applyRetainedWindowPatches(actorId, result.uiPatches) : [];
}

/** 📇️ H1-react — reads the build-time `🔣️descriptor.json` (design-abi.md §3, packet E1-describe's
 * emitter) siblinged next to `moduleUrl`'s directory, matching `ProgramBridge/🧊️component.rs`'s
 * native `read_descriptor_manifest` (H3-wgpu-native) exactly: an honest EMPTY manifest (zero apps),
 * not a fabricated one, whenever no descriptor exists yet — as of this packet, only `🗒️note` has a
 * real committed one (`📓️status.md`'s "E2-builder-descriptor" entry); every other plugin hits this
 * fallback until W3 migrates it. Never instantiates wasm to ask — that's exactly the "no eager
 * loading" property `loadPluginModule` used to break (per H2's lease-request naming this file).
 *
 * 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): `signal`, when given, is
 * forwarded straight to `fetch` — a caller (a component unmount mid-boot, or
 * {@link loadPluginModulesInDependencyOrder}'s own `signal` option) that wants to give up on a slow
 * descriptor fetch can now do so. An abort PROPAGATES rather than falling back to the empty-manifest
 * default: silently continuing to load a plugin the caller explicitly gave up on would be a worse
 * surprise than a rejected promise. A genuine network failure (not an abort) keeps the existing
 * fallback — no descriptor existing yet is this wave's honest, expected default (see above). */
export async function fetchDescriptorManifest(pluginId: string, moduleUrl: string, signal?: AbortSignal): Promise<PluginManifest> {
  const descriptorUrl = moduleUrl.replace(/\/[^/]+$/, "/🔣️descriptor.json");
  try {
    const response = await fetch(descriptorUrl, signal ? { signal } : undefined);
    const contentType = response.headers?.get?.("content-type") ?? "";
    if (response.ok && !contentType.includes("text/html")) {
      const descriptor = (await response.json()) as { readonly manifest?: PluginManifest };
      if (descriptor.manifest) return descriptor.manifest;
    }
  } catch (error) {
    if (signal?.aborted) throw error;
  }
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
      if (frame.Invocation.output.length) output = decodePackValue(new Uint8Array(frame.Invocation.output));
      if (frame.Invocation.diagnostics.length) {
        const decodedDiagnostics = decodePackValue(new Uint8Array(frame.Invocation.diagnostics));
        diagnostics = Array.isArray(decodedDiagnostics) ? (decodedDiagnostics as InvocationResponse["diagnostics"]) : [];
      }
      if (frame.Invocation.ui_scope.length) uiScope = decodePackValue(new Uint8Array(frame.Invocation.ui_scope)) as InvocationResponse["uiScope"];
      if (frame.Invocation.history_patch.length) {
        const decodedHistoryPatch = decodePackValue(new Uint8Array(frame.Invocation.history_patch));
        historyPatch = decodedHistoryPatch && typeof decodedHistoryPatch === "object" ? (decodedHistoryPatch as InvocationResponse["historyPatch"]) : undefined;
      }
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

/** 📡️ Wraps the framework-core `PluginWasmHandle` (the `enqueue`/`outcomes` turn ABI) behind the
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
  const channelRequests = new AppChannelRequestSequence();
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
      channels.set(instanceId, new AppChannelClient(handle, channelRequests, instanceId, appId, currentPluginRuntimeActor));
      return instanceId;
    },
    destroyApp: async (instanceId) => {
      // 🔌️ Ends this instance's channel's own outcome subscription BEFORE dropping it from `channels`
      // — otherwise it leaks a live subscriber against `handle.outcomes` for the rest of the handle's
      // lifetime (see `AppChannelClient.dispose`'s own doc).
      channels.get(instanceId)?.dispose();
      channels.delete(instanceId);
      await handle.destroyApp(instanceId);
    },
    takeSegmentedDownloadChunk: (instanceId, operationId) => handle.takeSegmentedDownloadChunk(instanceId, operationId),
    handleAction: (instanceId, actionJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(actionJson), "action", viewState),
    handleCommand: (instanceId, commandJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(commandJson), "command", viewState),
    // 🚧️ H1-react — window-body refresh needs the ActivationRegistry/ShardClient `Event::SurfaceVisible`
    // path this bare adapter has no access to (only the raw `enqueue`/`outcomes` `handle`, no actorId);
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
    // §2/§4), and the old empty-batch drain call has no replacement (guests are woken by events/timers/
    // `next-wake` now). `EffectBackbone` (the per-instance replacement) has not landed — flagged as a
    // still-open critical-path gap in `📓️status.md`'s "A2-abi-sdk — honest partial" entry, confirmed
    // still open as of `ProgramBridge/🧊️component.rs`'s native twin (H3-wgpu-native), which stubs the
    // identical three methods with explicit errors rather than guessing a wire format. Left `undefined`
    // here — every real call site in `ShellHost/🟦️component.tsx` already optional-chains these.
    attachBackbone: undefined,
    detachBackbone: undefined,
    // 🚧️ Same channel-v12 retirement as `attachBackbone`/`detachBackbone` above: the old
    // `AppFrame::Ephemeral` poll was the literal empty-batch drain design-abi.md §4 names as
    // retired outright — `ProgramBridge/🧊️component.rs`'s native twin (`ephemeral_snapshot`) stubs the
    // identical call with an explicit error for the same reason. `Ephemeral` frames still arrive
    // unsolicited on every real turn outcome (`plugin_exchange` appends one to every batch,
    // contract-freeze §C7.6) — a future packet that wants an on-demand snapshot here should cache the
    // most recently observed `Ephemeral` frame per instance rather than resurrecting the retired poll.
    ephemeralSnapshot: undefined,
    // 👥️ Contract-freeze §C7.6 — the ONLY plugin ingress for peers. `AppChannelClient.pushPresence`
    // encodes each `ArtifactPresencePeer` and sends the `AppCommand::Presence` frame; a plain `Done`
    // reply, nothing further decoded here.
    pushPresence: async (instanceId, ownColor, peers) => {
      await requireChannel(instanceId).pushPresence(ownColor, peers);
    },
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
 * (contract freeze §5.3/§6) — a plugin-level component-model export, not an app-instance turn-channel
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
/** 🎯️ One plugin `loadPluginModule` itself rejected on (network/activation failure) — kept SEPARATE
 * from {@link PluginGraphError} (a static graph fault: missing dependency, version mismatch, cycle)
 * since the two are different failure classes with different callers-facing meaning; conflating a
 * runtime load failure into a `PluginGraphError` code would mislead `pluginGraphErrorMessage`
 * (`@semio-tech/framework`) into describing a graph problem that never happened. */
export interface PluginLoadFailure {
  readonly pluginId: string;
  readonly error: unknown;
}

/** 🧮️ Groups `order`'s already-topologically-sorted entries into dependency LEVELS: level 0 has no
 * dependency inside `order` at all, level N+1's members depend on at least one level-N member (and on
 * nothing deeper). Every entry in the SAME level is independent of every other entry in that level —
 * that's exactly the "siblings may run in parallel" property {@link loadPluginModulesInDependencyOrder}
 * needs. Walking `order` in its own sequence (rather than re-deriving a topological order here) is
 * sufficient: Kahn's-algorithm output (`orderPluginRegistryEntries`'s own implementation) guarantees
 * every dependency of an entry appears strictly before it, so `levelOf` is always populated for a
 * dependency by the time its dependent is visited. A dependency id that ISN'T in `order` (already
 * dropped as blocked by a graph fault, or simply not part of this call's own entry set) contributes no
 * edge — same fail-soft posture as `orderPluginRegistryEntries` itself. */
function computeDependencyLevels(order: readonly PluginRegistryEntry[]): readonly (readonly PluginRegistryEntry[])[] {
  const levelOf = new Map<string, number>();
  const levels: PluginRegistryEntry[][] = [];
  for (const entry of order) {
    let level = 0;
    for (const dependency of entry.dependencies ?? []) {
      const dependencyLevel = levelOf.get(dependency.pluginId);
      if (dependencyLevel !== undefined) level = Math.max(level, dependencyLevel + 1);
    }
    levelOf.set(entry.pluginId, level);
    (levels[level] ??= []).push(entry);
  }
  return levels;
}

/** 🧵️ Runs `run` over every item in `items`, at most `limit` concurrently — a plain worker-pool loop
 * (each of `limit` workers pulls the next unclaimed index until the list is exhausted), not a batched
 * `Promise.all` chunking, so a fast item's slot is reused immediately rather than waiting for its whole
 * batch to finish. */
async function runBounded<T>(items: readonly T[], limit: number, run: (item: T) => Promise<void>): Promise<void> {
  let cursor = 0;
  const workerCount = Math.max(1, Math.min(limit, items.length));
  await Promise.all(
    Array.from({ length: workerCount }, async () => {
      while (cursor < items.length) {
        const item = items[cursor]!;
        cursor += 1;
        await run(item);
      }
    }),
  );
}

/**
 * 🎯️ Loads several plugin modules in dependency order (scout-2 §4: "boot must walk the dependency
 * order from `PluginGraph` instead of relying on array order") — a dependency's WHOLE level finishes
 * before any of its dependents starts, but independent siblings within one level load CONCURRENTLY,
 * bounded to `options.concurrency` (default {@link poolConcurrency}, the exact `min(hardwareConcurrency
 * -1, 4)` bound `getShardClient`'s own worker pool uses).
 *
 * 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): before this packet, this
 * function was a strict serial `for` loop — dependencies WERE respected, but so were completely
 * independent siblings (~20 plugins boot strictly one-after-another on the real app's real cold boot,
 * `📌️important.md`'s own "Flaky OS Dev Preview" note). The concurrency bound is deliberately the SAME
 * number as the shard pool's own worker count: `activate()`'s real cost (worker-side wasm
 * instantiation) is bounded by how many shard workers exist to run it, not by how many `activate()`
 * calls are in flight — requesting more concurrent activations than there are shards to service them
 * would only add memory pressure (`ActivationRegistry.evictForMemoryPressure`'s LRU thrashing) for zero
 * extra real parallelism, so `poolConcurrency()` (not, say, `entries.length`) is the bound that
 * actually reflects the hardware.
 *
 * Two independent failure classes, BOTH fail-soft (never abort the whole boot): a static graph fault
 * (missing dependency/version mismatch/cycle, `errors`, `orderPluginRegistryEntries`'s own existing
 * posture) drops the entry before it is ever attempted; a RUNTIME `loadPluginModule` rejection
 * (`loadFailures`, new in this packet) drops that entry AND cascades to skip every not-yet-attempted
 * descendant that (transitively) depends on it — a dependent can't sensibly boot on a dependency that
 * never loaded. `options.signal`, when aborted, stops STARTING new loads (in-flight ones still settle
 * naturally) — forwarded to `loadPluginModule`'s own descriptor-fetch abort for whichever entries
 * haven't started yet. `handles` stays in the same topological sequence `order` always had, regardless
 * of the level-parallel loading order underneath, for caller stability. */
export async function loadPluginModulesInDependencyOrder(
  entries: readonly PluginRegistryEntry[],
  options?: {
    readonly loadModule?: (pluginId: string, moduleUrl: string, signal?: AbortSignal) => Promise<PluginWasmHandle>;
    readonly concurrency?: number;
    readonly signal?: AbortSignal;
  },
): Promise<{ readonly handles: readonly PluginWasmHandle[]; readonly errors: readonly PluginGraphError[]; readonly loadFailures: readonly PluginLoadFailure[] }> {
  const loadModule = options?.loadModule ?? loadPluginModule;
  const limit = options?.concurrency ?? poolConcurrency();
  const signal = options?.signal;
  const { order, errors } = orderPluginRegistryEntries(entries);
  const levels = computeDependencyLevels(order);
  const handleByPluginId = new Map<string, PluginWasmHandle>();
  const failedPluginIds = new Set<string>();
  const loadFailures: PluginLoadFailure[] = [];

  for (const level of levels) {
    const loadable = level.filter((entry) => {
      const blockedDependency = (entry.dependencies ?? []).find((dependency) => failedPluginIds.has(dependency.pluginId));
      if (!blockedDependency) return true;
      failedPluginIds.add(entry.pluginId);
      loadFailures.push({ pluginId: entry.pluginId, error: new Error(`[DEBUG] loadPluginModulesInDependencyOrder: ${entry.pluginId} skipped — dependency ${blockedDependency.pluginId} failed to load`) });
      return false;
    });
    await runBounded(loadable, limit, async (entry) => {
      if (signal?.aborted) {
        failedPluginIds.add(entry.pluginId);
        loadFailures.push({ pluginId: entry.pluginId, error: new Error(`[DEBUG] loadPluginModulesInDependencyOrder: ${entry.pluginId} skipped — boot aborted`) });
        return;
      }
      try {
        handleByPluginId.set(entry.pluginId, await loadModule(entry.pluginId, entry.moduleUrl, signal));
      } catch (error) {
        failedPluginIds.add(entry.pluginId);
        loadFailures.push({ pluginId: entry.pluginId, error });
      }
    });
  }

  const handles = order.map((entry) => handleByPluginId.get(entry.pluginId)).filter((handle): handle is PluginWasmHandle => handle !== undefined);
  return { handles, errors, loadFailures };
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

  describe("command-ingress fault diagnostics", () => {
    it("decodes the normalized scalar-wire fault payload instead of hiding the terminal cause", () => {
      const bytes = encodeFaultBytes("plugin.command-rejected");
      expect(commandIngressFaultDisplay({ tag: "fault", val: { cursor: {}, fault: { tag: "fault", val: Array.from(bytes) } } })).toBe("plugin.command-rejected: plugin.command-rejected");
    });
  });

  describe("instance-open retained UI lifecycle", () => {
    it("preserves document effects returned by a refresh even before any surface is retained", async () => {
      const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧪️fixtures/📬️operation-continuation.json");
      const response = retainedUiRefreshResponse(7, { viewState: {} }, new Map(), [{ tag: "load-document", val: fixture.effects.loadDocument }]);
      expect(response.requestedEffects).toEqual([{ loadDocument: fixture.effects.loadDocument }]);
    });

    it("reports a refresh fault frame instead of returning an unchanged surface", async () => {
      const { encodeAppFrame } = await import("@semio-tech/framework-os");
      const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧪️fixtures/📬️operation-continuation.json");
      const payload = encodeAppFrame({ Error: { in_reply_to: null, fault: Array.from(encodeFaultBytes(fixture.wire.fault)), report: [] } });
      expect(() => retainedUiRefreshResponse(7, { viewState: {} }, new Map(), [{ tag: "send-message", val: { target: { tag: "shell", val: "7" }, payload } }])).toThrow(fixture.wire.fault);
    });

    it("refreshes window and panel surfaces from the language-agnostic ownership cases", async () => {
      const { default: fixture } = await import("./🧪️fixtures/🔣️surface-refresh.json");
      for (const testCase of fixture.cases) {
        const bodyKeys = uiRefreshBodyKeys(testCase.request);
        expect(bodyKeys, testCase.name).toEqual(testCase.bodyKeys);
        const retained = new Map<string, RetainedSurface>();
        for (const bodyKey of bodyKeys) {
          const root: UiNodeRecord = {
            id: 0, key: bodyKey, component: { type: "text", value: bodyKey, emphasize: null, dataAttributes: null },
            layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false,
            transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [],
          };
          const { surface, desynced } = applyUiPatchToRetained(null, {
            surface: bodyKey, revision: 1, baseRevision: 0,
            ops: [{ type: "upsert", ...root }, { type: "setRoot", id: 0 }],
          });
          expect(desynced).toBe(false);
          retained.set(retainedSurfaceId(7, bodyKey), surface!);
        }
        const response = retainedUiRefreshResponse(7, testCase.request, retained);
        expect(response.windows?.map((entry) => entry.key)).toEqual(testCase.windowKeys);
        expect(response.panels?.map((entry) => entry.key)).toEqual(testCase.panelKeys);
        for (const entry of [...(response.windows ?? []), ...(response.panels ?? [])]) {
          expect(entry.value).toMatchObject({ component: { type: "text" } });
          expect(entry.hash).not.toBe("");
        }
      }
    });

    it("acknowledges only patches that identify the exact retained surface", () => {
      expect(
        patchAckEvents([
          { surface: pluginSurfaceRef(4, "workflow"), revision: 9n, baseRevision: 8n, ops: [] },
          { revision: 1n, baseRevision: 0n, ops: [] },
        ]),
      ).toEqual([{ kind: "patch-ack", payload: { surface: pluginSurfaceRef(4, "workflow"), revision: 9n } }]);
    });

    it("retains the first render patch so an unchanged surface-visible probe can reuse it", () => {
      const actorId = "initial-render-retention-test#1";
      expect(pluginSurfaceRef(1, "workflow")).toEqual({ instance: 1, surface: "workflow" });
      const root: UiNodeRecord = {
        id: 0,
        key: "root",
        component: { type: "text", value: "ready", emphasize: null, dataAttributes: null },
        layout: { kind: "leaf", width: "hug", height: "hug" },
        style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
        activity: "idle",
        disabled: false,
        transition: null,
        accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
        bindings: [],
        menu: null,
        children: [],
      };
      retainedWindowByActor.delete(actorId);
      try {
        retainTurnUiPatches(actorId, {
          uiPatches: [{ revision: 1n, baseRevision: 0n, ops: [{ tag: "upsert", val: { node: Array.from(encodePackValue(root)) } }, { tag: "set-root", val: 0n }] }],
        });
        const retained = retainedWindowByActor.get(actorId)?.get("window");
        expect(retained).toMatchObject({ surface: "window", revision: 1, root: 0 });
        expect(() => retainedSurfaceHash(retainedSurfaceToSnapshot(retained!))).not.toThrow();
        expect(retainedSurfaceToBuiltNode(retained!)).toMatchObject({ key: "root", component: { type: "text", value: "ready" }, children: [] });
      } finally {
        retainedWindowByActor.delete(actorId);
      }
    });
  });

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
      takeSegmentedDownloadChunk: async () => undefined,
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
        takeSegmentedDownloadChunk: async () => undefined,
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
      const turnBroadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const fakeLease = {
        handle: {
          manifest: async () => encodePackValue({ pluginId: "b-plugin", label: "B", version: "1.0.0", apps: [], workflows: [], examples: [] }),
          createApp: async () => 20,
          destroyApp: async () => {},
          takeSegmentedDownloadChunk: async () => undefined,
          enqueue: (instanceId: number, events: readonly Uint8Array[]) => {
            const commands = events.map((frame) => decodeAppCommand(frame));
            seenCommands.push(...commands);
            const frames = commands.flatMap((command) => {
              if ("transactionPrepare" in command) {
                return [encodeAppFrame({ transactionPrepared: { txn_id: command.transactionPrepare.txn_id, foreign: [], rejection: [] } }), encodeAppFrame({ Done: { in_reply_to: command.transactionPrepare.seq } })];
              }
              if ("transactionCommit" in command) {
                return [encodeAppFrame({ transactionCommitted: { txn_id: command.transactionCommit.txn_id, edit_id: "edit-1" } }), encodeAppFrame({ Done: { in_reply_to: command.transactionCommit.seq } })];
              }
              if ("transactionRollback" in command) return [encodeAppFrame({ transactionRolledBack: { txn_id: command.transactionRollback.txn_id } }), encodeAppFrame({ Done: { in_reply_to: command.transactionRollback.seq } })];
              if ("transactionUndo" in command) return [encodeAppFrame({ Done: { in_reply_to: command.transactionUndo.seq } })];
              if ("transactionRedo" in command) return [encodeAppFrame({ Done: { in_reply_to: command.transactionRedo.seq } })];
              if ("ReadDocument" in command) {
                return [encodeAppFrame({ Document: { in_reply_to: command.ReadDocument.seq, pack: [5, 5], spr: [6], ops: "" } })];
              }
              throw new Error(`unexpected command ${JSON.stringify(command)}`);
            });
            turnBroadcast.push({ instanceId, frames });
          },
          outcomes: turnBroadcast.stream,
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
      const turnBroadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const fakeLease = {
        handle: {
          manifest: async () => encodePackValue({ pluginId: "b-plugin", label: "B", version: "1.0.0", apps: [], workflows: [], examples: [] }),
          createApp: async () => 20,
          destroyApp: async () => {},
          takeSegmentedDownloadChunk: async () => undefined,
          enqueue: (instanceId: number, events: readonly Uint8Array[]) => {
            const commands = events.map((frame) => decodeAppCommand(frame));
            const frames = commands.map((command) => {
              if (!("LoadDocument" in command)) throw new Error(`unexpected command ${JSON.stringify(command)}`);
              return encodeAppFrame({ Done: { in_reply_to: command.LoadDocument.seq } });
            });
            turnBroadcast.push({ instanceId, frames });
          },
          outcomes: turnBroadcast.stream,
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

  //#region 🧪️terra-web-plugin-runtime
  /** 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime). No real sleeps anywhere
   * below — every wait is a `queueMicrotask` flush or a deferred promise the test itself settles. */
  const flushMicrotasks = async (times = 4): Promise<void> => {
    for (let index = 0; index < times; index += 1) await new Promise<void>((resolve) => queueMicrotask(resolve));
  };

  describe("submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue)", () => {
    function withFakeShardClient<T>(turnImpl: (actorId: string, events: readonly ShardEventEnvelope[], budget: ShardBudget) => Promise<unknown>, run: () => Promise<T>): Promise<T> {
      const previous = sharedShardClient;
      sharedShardClient = { turn: turnImpl } as unknown as ShardClient;
      return run().finally(() => {
        sharedShardClient = previous;
      });
    }

    it("continues admitted operations after surfaces are retained and ACKs each exact result", async () => {
      const { Buffer } = await import("node:buffer");
      const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧪️fixtures/📬️operation-continuation.json");
      const token = Buffer.alloc(25);
      token.writeUInt32LE(fixture.wire.receiver, 0);
      token.writeBigUInt64LE(BigInt(fixture.wire.operation), 4);
      token.writeBigUInt64LE(BigInt(fixture.wire.generation), 12);
      token.writeUInt32LE(fixture.wire.sequence, 20);
      token[24] = fixture.wire.attempt;
      const expectedAck = Buffer.concat([Buffer.from("semio.typed-operation-ack.v1\0"), token]);
      const payload = Buffer.from(fixture.wire.payload);
      const length = Buffer.alloc(4);
      length.writeUInt32LE(payload.length);
      let turns = 0;
      await withFakeShardClient(async (_actor, events) => {
        if (turns > 0) expect(events).toEqual([{ kind: "message", payload: { source: { tag: "shell", val: String(fixture.wire.receiver) }, payload: Array.from(expectedAck) } }]);
        const lane = fixture.wire.lanes[turns++];
        const effects = lane === undefined ? [] : [{ tag: "send-message", val: { target: { tag: "shell", val: String(fixture.wire.receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), token, Buffer.from([lane]), length, payload])) } }];
        return { uiPatches: [], effects, nextWake: null, status: { tag: lane === undefined ? "idle" : "more-work" } };
      }, async () => {
        const result = await settlePluginTurn("retained-operation#7", { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } }, "Interactive", new Set(), undefined, true);
        expect(turns).toBe(fixture.wire.lanes.length + 1);
        expect(result.effects).toEqual([]);
        expect(result.status).toEqual({ tag: "idle" });
      });
    });

    it("does not replay already acknowledged ingress publications during settlement", async () => {
      const { Buffer } = await import("node:buffer");
      const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧪️fixtures/📬️operation-continuation.json");
      const results: WireTurnResult[] = fixture.wire.lanes.map((lane, sequence) => {
        const header = Buffer.alloc(30);
        header.writeUInt32LE(fixture.wire.receiver, 0);
        header.writeBigUInt64LE(BigInt(fixture.wire.operation), 4);
        header.writeBigUInt64LE(BigInt(fixture.wire.generation), 12);
        header.writeUInt32LE(sequence, 20);
        header[24] = fixture.wire.attempt;
        header[25] = lane;
        return { uiPatches: [], effects: [{ tag: "send-message", val: { target: { tag: "shell", val: String(fixture.wire.receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), header])) } }], nextWake: null, status: { tag: "more-work" } };
      });
      const pending = typedOperationAcknowledgements(results.at(-1)!);
      await withFakeShardClient(async (_actor, events) => {
        expect(events).toEqual(pending);
        return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
      }, async () => {
        const result = await settleAcknowledgedPluginTurns("retained-ingress#7", results, pending);
        expect(result.effects).toEqual([]);
        expect(result.status).toEqual({ tag: "idle" });
      });
    });

    it("validates fixed result page authority and preserves document and download effects", async () => {
      const { Buffer } = await import("node:buffer");
      const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧪️fixtures/📬️operation-continuation.json");
      const wire = (lane: number, text: string, receiver = fixture.wire.receiver): WireVariant => {
        const body = Buffer.alloc(30);
        body.writeUInt32LE(fixture.wire.receiver, 0);
        body.writeBigUInt64LE(BigInt(fixture.wire.operation), 4);
        body.writeBigUInt64LE(BigInt(fixture.wire.generation), 12);
        body.writeUInt32LE(fixture.wire.sequence, 20);
        body[24] = fixture.wire.attempt;
        body[25] = lane;
        const payload = Buffer.from(text);
        body.writeUInt32LE(payload.length, 26);
        return { tag: "send-message", val: { target: { tag: "shell", val: String(receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), body, payload])) } };
      };
      expect(typedOperationResult(wire(10, "x".repeat(4_096)))?.payload.length).toBe(4_096);
      for (const invalid of [wire(12, ""), wire(10, "x".repeat(4_097)), wire(10, "", 8)]) expect(() => typedOperationResult(invalid)).toThrow("authority");
      const truncated = wire(10, "x");
      (truncated.val as { payload: number[] }).payload.pop();
      expect(() => typedOperationResult(truncated)).toThrow("authority");
      expect(wireEffectToFriendly({ tag: "load-document", val: fixture.effects.loadDocument })).toEqual({ loadDocument: fixture.effects.loadDocument });
      const download = consumeTypedOperationEffects([wire(9, JSON.stringify(fixture.effects.download))]);
      expect(download.map(wireEffectToFriendly)).toEqual([{ downloadMediaExport: { filename: fixture.effects.download[0], mimeType: fixture.effects.download[1], data: fixture.wire.operation, encoding: "semio-segmented-handle-v1:identity" } }]);
      let acknowledged = false;
      await withFakeShardClient(async (_actor, events) => {
        acknowledged = events.length === 1 && events[0]?.kind === "message";
        return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
      }, async () => {
        await expect(settlePluginTurn("faulted-operation#7", { uiPatches: [], effects: [wire(11, fixture.wire.fault)], nextWake: null, status: { tag: "idle" } }, "Interactive", new Set(), undefined, true)).rejects.toThrow(fixture.wire.fault);
        expect(acknowledged).toBe(true);
      });
    });

    it("keeps the command reply when publication supplies only an unsolicited UI scope", async () => {
      const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧪️fixtures/📬️operation-continuation.json");
      const bytes = (value: unknown) => Array.from(encodePackValue(value));
      const frames = [
        { Invocation: { in_reply_to: 1, output: bytes({ operationId: fixture.wire.operation }), diagnostics: bytes([]), ui_scope: bytes({ kind: "none" }), history_patch: [], messages: [] } },
        { Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: bytes({ kind: "full" }), history_patch: [], messages: [] } },
      ];
      const client = { command: async () => frames } as unknown as AppChannelClient;
      const result = await performInvocation(client, 7, {}, "action", {});
      expect(result.output).toEqual({ operationId: fixture.wire.operation });
      expect(result.uiScope).toEqual({ kind: "full" });
    });

    it("drains an actor's more-work turns until the reconciled UI patch is publishable", async () => {
      let continuationCount = 0;
      await withFakeShardClient(
        async () => {
          continuationCount += 1;
          return { uiPatches: [{ revision: 1, baseRevision: 0, ops: [] }], effects: [], nextWake: null, status: { tag: "idle" } };
        },
        async () => {
          const actorId = "turn-test-more-work-actor";
          const result = await settlePluginTurn(actorId, { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult, "UserVisible");
          expect(continuationCount).toBe(1);
          expect(result.uiPatches).toHaveLength(1);
        },
      );
    });

    it("acknowledges each retained surface before requesting the next bounded publication", async () => {
      const { default: fixture } = await import("./🧪️fixtures/🔣️surface-refresh.json");
      const surfaces = fixture.acknowledgement.surfaces;
      const submitted: string[][] = [];
      await withFakeShardClient(
        async (_actor, events) => {
          const acknowledgements = events.filter((event) => event.kind === "patch-ack").map((event) => (event.payload as { surface: { surface: string } }).surface.surface);
          submitted.push(acknowledgements);
          return {
            uiPatches: acknowledgements.includes(surfaces[0]!) ? [{ surface: pluginSurfaceRef(7, surfaces[1]!), revision: 1, baseRevision: 0, ops: [] }] : [],
            effects: [], nextWake: null, status: { tag: "idle" },
          };
        },
        async () => {
          const result = await settlePluginTurn(
            "bounded-panel-publication#1",
            { uiPatches: [{ surface: pluginSurfaceRef(7, surfaces[0]!), revision: 1, baseRevision: 0, ops: [] }], effects: [], nextWake: null, status: { tag: "more-work" } },
            "UserVisible",
            new Set(surfaces.map((surface) => retainedSurfaceId(7, surface))),
            (turn) => patchAckEvents(turn.uiPatches),
          );
          expect(submitted).toEqual(surfaces.map((surface) => [surface]));
          expect(submitted.every((batch) => batch.length <= fixture.acknowledgement.maxUnacknowledged)).toBe(true);
          expect(result.uiPatches.map((patch) => patch.surface?.surface)).toEqual(surfaces);
        },
      );
    });

    it("rejects an idle actor that did not publish a requested surface instead of retaining a loading placeholder", async () => {
      const { default: fixture } = await import("./🧪️fixtures/🔣️surface-refresh.json");
      const { instance, requested, published, missing } = fixture.quiescent;
      await expect(settlePluginTurn(
        "idle-missing-surface#1",
        { uiPatches: published.map((surface) => ({ surface: pluginSurfaceRef(instance, surface), revision: 1, baseRevision: 0, ops: [] })), effects: [], nextWake: null, status: { tag: "idle" } },
        "UserVisible",
        new Set(requested.map((surface) => retainedSurfaceId(instance, surface))),
      )).rejects.toThrow(`missing=${JSON.stringify(missing)}`);
    });

    it("does not chase background work during instance-open before a UI surface is requested", async () => {
      let continuationCount = 0;
      await withFakeShardClient(
        async () => {
          continuationCount += 1;
          return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
        },
        async () => {
          const result = await settlePluginTurn(
            "instance-open-with-background-work#1",
            { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
            "Interactive",
            new Set(),
          );
          expect(continuationCount).toBe(0);
          expect(result.uiPatches).toHaveLength(0);
        },
      );
    });

    it("drains until every missing requested surface has published its first patch", async () => {
      let continuationCount = 0;
      await withFakeShardClient(
        async () => {
          continuationCount += 1;
          const surface = continuationCount === 1 ? "workflow" : "preview";
          return {
            uiPatches: [{ surface: pluginSurfaceRef(7, surface), revision: 1, baseRevision: 0, ops: [] }],
            effects: [],
            nextWake: null,
            status: { tag: continuationCount === 1 ? "more-work" : "idle" },
          };
        },
        async () => {
          const actorId = "turn-test-multiple-surfaces-actor";
          const result = await settlePluginTurn(
            actorId,
            { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
            "UserVisible",
            new Set([retainedSurfaceId(7, "workflow"), retainedSurfaceId(7, "preview")]),
          );
          expect(continuationCount).toBe(2);
          expect(result.uiPatches.map((patch) => patch.surface?.surface)).toEqual(["workflow", "preview"]);
        },
      );
    });

    it("allows a large retained surface to reconcile beyond the former continuation ceiling", async () => {
      let continuationCount = 0;
      await withFakeShardClient(
        async () => {
          continuationCount += 1;
          return {
            uiPatches: continuationCount === 1_025 ? [{ surface: pluginSurfaceRef(9, "large"), revision: 1, baseRevision: 0, ops: [] }] : [],
            effects: [],
            nextWake: null,
            status: { tag: continuationCount === 1_025 ? "idle" : "more-work" },
          };
        },
        async () => {
          const result = await settlePluginTurn(
            "turn-test-large-surface-actor",
            { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
            "UserVisible",
            new Set([retainedSurfaceId(9, "large")]),
          );
          expect(continuationCount).toBe(1_025);
          expect(result.uiPatches).toHaveLength(1);
        },
      );
    });

    it("yields the browser event loop while a retained surface needs several continuation batches", async () => {
      let continuationCount = 0;
      let browserTaskObserved = false;
      const browserTask = new Promise<void>((resolve) =>
        setTimeout(() => {
          browserTaskObserved = true;
          resolve();
        }, 0),
      );
      await withFakeShardClient(
        async () => {
          continuationCount += 1;
          return {
            uiPatches: continuationCount === PLUGIN_UI_CONTINUATION_BATCH_SIZE + 1 ? [{ surface: pluginSurfaceRef(9, "cooperative"), revision: 1, baseRevision: 0, ops: [] }] : [],
            effects: [],
            nextWake: null,
            status: { tag: continuationCount === PLUGIN_UI_CONTINUATION_BATCH_SIZE + 1 ? "idle" : "more-work" },
          };
        },
        async () => {
          const result = await settlePluginTurn(
            "turn-test-cooperative-surface-actor",
            { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
            "UserVisible",
            new Set([retainedSurfaceId(9, "cooperative")]),
          );
          expect(browserTaskObserved).toBe(true);
          expect(result.uiPatches).toHaveLength(1);
        },
      );
      await browserTask;
    });

    it("does not poll for an unchanged refresh when every requested surface is already retained", async () => {
      let continuationCount = 0;
      await withFakeShardClient(
        async () => {
          continuationCount += 1;
          return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
        },
        async () => {
          const result = await settlePluginTurn(
            "turn-test-retained-surfaces-actor",
            { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
            "UserVisible",
            new Set(),
          );
          expect(continuationCount).toBe(0);
          expect(result.uiPatches).toHaveLength(0);
        },
      );
    });

    it("dispatches a queued Interactive-lane turn before an already-queued UserVisible-lane turn for the SAME actor, regardless of arrival order", async () => {
      const dispatchOrder: string[] = [];
      await withFakeShardClient(
        async (_actorId, events) => {
          dispatchOrder.push((events[0]!.payload as { readonly marker: string }).marker);
          return { uiPatches: [], effects: [], nextWake: null };
        },
        async () => {
          const actorId = "turn-test-lane-actor";
          // 🎯️ UserVisible enqueued FIRST, Interactive second — both land before the scheduler's first
          // microtask pump, so dispatch order must reflect lane priority, not arrival order.
          const userVisible = submitPluginTurn(actorId, [{ kind: "surface-visible", payload: { marker: "user-visible-1" } }], "UserVisible", "surface-visible");
          const interactive = submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: "interactive-1" } }], "Interactive");
          await Promise.all([userVisible, interactive]);
          expect(dispatchOrder).toEqual(["interactive-1", "user-visible-1"]);
        },
      );
    });

    it("collapses a 200-call coalescing burst to a single dispatched turn, resolving EVERY waiter (not just the last) with the winning result", async () => {
      let dispatchCount = 0;
      await withFakeShardClient(
        async (_actorId, events) => {
          dispatchCount += 1;
          return { uiPatches: [], effects: [{ tag: "notify", val: { message: String((events[0]!.payload as { readonly marker: number }).marker) } }], nextWake: null };
        },
        async () => {
          const actorId = "turn-test-coalesce-actor";
          const results = await Promise.all(
            Array.from({ length: 200 }, (_, index) => submitPluginTurn(actorId, [{ kind: "surface-visible", payload: { marker: index } }], "UserVisible", "surface-visible")),
          );
          expect(dispatchCount).toBe(1);
          for (const result of results) expect(result.effects).toEqual(results[0]!.effects);
        },
      );
    });

    it("teardown rejects queued turn waiters before disposing the actor transport", async () => {
      let release!: () => void;
      const gate = new Promise<unknown>((resolve) => {
        release = () => resolve({ uiPatches: [], effects: [], nextWake: null });
      });
      await withFakeShardClient(
        async () => gate,
        async () => {
          const actorId = "turn-test-teardown-actor";
          const inFlight = submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: "in-flight" } }], "Interactive");
          await flushMicrotasks();
          const queued = submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: "queued" } }], "Interactive");
          teardownPluginActor(actorId);
          await expect(queued).rejects.toThrow(`plugin actor ${actorId} disposed`);
          release();
          await expect(inFlight).resolves.toMatchObject({ uiPatches: [], effects: [] });
        },
      );
    });

    it("surfaces Rejected once an actor's mailbox is genuinely full of distinct turns, instead of growing without bound", async () => {
      await withFakeShardClient(
        () => new Promise(() => {}), // 🎯️ never settles — every submitted turn for this actor stays queued behind the first, in flight forever.
        async () => {
          const actorId = "turn-test-rejected-actor";
          const settled: Array<"accepted" | "rejected"> = [];
          // 🎯️ One turn dispatches immediately (goes "in flight"); the rest queue behind it in the same
          // lane (no coalescing, no lower lane to evict) until PLUGIN_TURN_MAILBOX_CAPACITY (32) is hit.
          for (let index = 0; index < 41; index += 1) {
            submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: `t-${index}` } }], "Interactive").then(
              () => settled.push("accepted"),
              () => settled.push("rejected"),
            );
          }
          await flushMicrotasks();
          expect(settled.filter((outcome) => outcome === "rejected").length).toBeGreaterThan(0);
        },
      );
    });

    it("never dispatches a second turn for an actor before the first settles, even while a DIFFERENT actor's turns run concurrently", async () => {
      const inFlightByActor = new Map<string, number>();
      const maxInFlightByActor = new Map<string, number>();
      await withFakeShardClient(
        async (actorId) => {
          inFlightByActor.set(actorId, (inFlightByActor.get(actorId) ?? 0) + 1);
          maxInFlightByActor.set(actorId, Math.max(maxInFlightByActor.get(actorId) ?? 0, inFlightByActor.get(actorId)!));
          await flushMicrotasks(2);
          inFlightByActor.set(actorId, inFlightByActor.get(actorId)! - 1);
          return { uiPatches: [], effects: [], nextWake: null };
        },
        async () => {
          const actorA = "turn-test-interleave-a";
          const actorB = "turn-test-interleave-b";
          await Promise.all([
            submitPluginTurn(actorA, [{ kind: "app-command", payload: {} }], "Interactive"),
            submitPluginTurn(actorA, [{ kind: "app-command", payload: {} }], "Interactive"),
            submitPluginTurn(actorA, [{ kind: "app-command", payload: {} }], "Interactive"),
            submitPluginTurn(actorB, [{ kind: "app-command", payload: {} }], "Interactive"),
          ]);
          expect(maxInFlightByActor.get(actorA)).toBe(1);
          expect(maxInFlightByActor.get(actorB)).toBe(1);
        },
      );
    });
  });

  describe("PluginRuntime shard-loss wiring (real restore, not just a console.error)", () => {
    it("handlePluginShardLost delegates to ActivationRegistry.handleShardLost for EXACTLY the affected actorIds", () => {
      const restoreCalls: Array<{ readonly shardIndex: number; readonly actorIds: readonly string[] }> = [];
      const fakeRegistry = { handleShardLost: (shardIndex: number, actorIds: readonly string[]) => restoreCalls.push({ shardIndex, actorIds }) } as unknown as ActivationRegistry;
      const previous = sharedActivationRegistry;
      sharedActivationRegistry = fakeRegistry;
      try {
        handlePluginShardLost(2, ["plugin-a#1", "plugin-b#7"]);
        expect(restoreCalls).toEqual([{ shardIndex: 2, actorIds: ["plugin-a#1", "plugin-b#7"] }]);
      } finally {
        sharedActivationRegistry = previous;
      }
    });

    it("buildShardClientOptions wires onShardLost to handlePluginShardLost (not a bare console.error) and sizes shardCount via poolConcurrency", () => {
      const fakeCreateWorker = () => ({ postMessage: () => {}, terminate: () => {}, onmessage: null, onerror: null }) as unknown as ShardWorkerLike;
      const options = buildShardClientOptions(fakeCreateWorker);
      expect(options.onShardLost).toBe(handlePluginShardLost);
      expect(options.shardCount).toBe(poolConcurrency());
      expect(options.createWorker).toBe(fakeCreateWorker);
    });
  });

  describe("fetchDescriptorManifest AbortSignal", () => {
    it("propagates an aborted signal's fetch rejection instead of silently falling back to an empty manifest", async () => {
      const controller = new AbortController();
      controller.abort();
      const originalFetch = globalThis.fetch;
      globalThis.fetch = (async () => {
        throw new DOMException("aborted", "AbortError");
      }) as typeof fetch;
      try {
        await expect(fetchDescriptorManifest("p", "https://x/p.js", controller.signal)).rejects.toThrow();
      } finally {
        globalThis.fetch = originalFetch;
      }
    });

    it("still falls back to an empty manifest on a genuine (non-abort) fetch failure — the existing E1-describe/W3 gap, unchanged", async () => {
      const originalFetch = globalThis.fetch;
      globalThis.fetch = (async () => {
        throw new Error("network down");
      }) as typeof fetch;
      try {
        const manifest = await fetchDescriptorManifest("p", "https://x/p.js");
        expect(manifest.pluginId).toBe("p");
      } finally {
        globalThis.fetch = originalFetch;
      }
    });

    it("treats the dev server's HTML SPA fallback as an absent descriptor without a parse warning", async () => {
      const originalFetch = globalThis.fetch;
      const originalWarn = console.warn;
      let warningCount = 0;
      console.warn = () => {
        warningCount += 1;
      };
      globalThis.fetch = (async () => new Response("<!doctype html>", { status: 200, headers: { "content-type": "text/html" } })) as typeof fetch;
      try {
        const manifest = await fetchDescriptorManifest("p", "https://x/p.js");
        expect(manifest.pluginId).toBe("p");
        expect(warningCount).toBe(0);
      } finally {
        console.warn = originalWarn;
        globalThis.fetch = originalFetch;
      }
    });
  });

  describe("loadPluginModulesInDependencyOrder — level-parallel boot", () => {
    it("runs independent siblings in parallel within a level, holding a dependent until its dependency's WHOLE level finishes", async () => {
      const started: string[] = [];
      const releaseFns = new Map<string, () => void>();
      const loadModule = (pluginId: string): Promise<PluginWasmHandle> =>
        new Promise<PluginWasmHandle>((resolve) => {
          started.push(pluginId);
          releaseFns.set(pluginId, () => resolve(fakeHandle(pluginId, [], [])));
        });
      const entries: PluginRegistryEntry[] = [
        { pluginId: "a", moduleUrl: "https://x/a.js", dependencies: [] },
        { pluginId: "b", moduleUrl: "https://x/b.js", dependencies: [] },
        { pluginId: "c", moduleUrl: "https://x/c.js", dependencies: [{ pluginId: "a", version: "*" }] },
      ];
      const resultPromise = loadPluginModulesInDependencyOrder(entries, { loadModule, concurrency: 4 });
      await flushMicrotasks();
      expect(new Set(started)).toEqual(new Set(["a", "b"]));
      expect(started).not.toContain("c");
      releaseFns.get("a")!();
      releaseFns.get("b")!();
      await flushMicrotasks();
      expect(started).toContain("c");
      releaseFns.get("c")!();
      const result = await resultPromise;
      expect(result.handles.map((handle) => handle.pluginId)).toEqual(["a", "b", "c"]);
      expect(result.errors).toEqual([]);
      expect(result.loadFailures).toEqual([]);
    });

    it("bounds within-level concurrency to the given limit — a third independent sibling waits for a free slot", async () => {
      const started: string[] = [];
      const releaseFns: Array<() => void> = [];
      const loadModule = (pluginId: string): Promise<PluginWasmHandle> =>
        new Promise<PluginWasmHandle>((resolve) => {
          started.push(pluginId);
          releaseFns.push(() => resolve(fakeHandle(pluginId, [], [])));
        });
      const entries: PluginRegistryEntry[] = ["a", "b", "c"].map((id) => ({ pluginId: id, moduleUrl: `https://x/${id}.js`, dependencies: [] }));
      const resultPromise = loadPluginModulesInDependencyOrder(entries, { loadModule, concurrency: 2 });
      await flushMicrotasks();
      expect(started).toHaveLength(2);
      releaseFns[0]!();
      await flushMicrotasks();
      expect(started).toHaveLength(3);
      releaseFns[1]!();
      releaseFns[2]!();
      await resultPromise;
    });

    it("cascades a runtime load failure to skip dependents, while unrelated siblings still load — a distinct PluginLoadFailure, not a PluginGraphError", async () => {
      const loadModule = (pluginId: string): Promise<PluginWasmHandle> => (pluginId === "a" ? Promise.reject(new Error("boom")) : Promise.resolve(fakeHandle(pluginId, [], [])));
      const entries: PluginRegistryEntry[] = [
        { pluginId: "a", moduleUrl: "https://x/a.js", dependencies: [] },
        { pluginId: "b", moduleUrl: "https://x/b.js", dependencies: [] },
        { pluginId: "c", moduleUrl: "https://x/c.js", dependencies: [{ pluginId: "a", version: "*" }] },
      ];
      const result = await loadPluginModulesInDependencyOrder(entries, { loadModule, concurrency: 4 });
      expect(result.handles.map((handle) => handle.pluginId)).toEqual(["b"]);
      expect(result.errors).toEqual([]);
      expect(result.loadFailures.map((failure) => failure.pluginId).sort()).toEqual(["a", "c"]);
    });

    it("defaults its concurrency bound to poolConcurrency() when the caller doesn't override it", async () => {
      const cap = poolConcurrency();
      let inFlight = 0;
      let maxInFlight = 0;
      const loadModule = (pluginId: string): Promise<PluginWasmHandle> =>
        new Promise<PluginWasmHandle>((resolve) => {
          inFlight += 1;
          maxInFlight = Math.max(maxInFlight, inFlight);
          queueMicrotask(() => {
            inFlight -= 1;
            resolve(fakeHandle(pluginId, [], []));
          });
        });
      const entries: PluginRegistryEntry[] = Array.from({ length: cap + 6 }, (_, index) => ({ pluginId: `p${index}`, moduleUrl: `https://x/p${index}.js`, dependencies: [] }));
      await loadPluginModulesInDependencyOrder(entries, { loadModule });
      expect(maxInFlight).toBeLessThanOrEqual(cap);
      expect(maxInFlight).toBeGreaterThan(0);
    });

    it("aborts cleanly: aborting mid-boot stops starting new loads without throwing, while an already-started load still settles normally", async () => {
      const controller = new AbortController();
      const started: string[] = [];
      const loadModule = (pluginId: string): Promise<PluginWasmHandle> => {
        started.push(pluginId);
        if (pluginId === "a") controller.abort(); // 🎯️ abort while level 0 ("a") is already in flight.
        return Promise.resolve(fakeHandle(pluginId, [], []));
      };
      const entries: PluginRegistryEntry[] = [
        { pluginId: "a", moduleUrl: "https://x/a.js", dependencies: [] },
        { pluginId: "b", moduleUrl: "https://x/b.js", dependencies: [{ pluginId: "a", version: "*" }] },
      ];
      const result = await loadPluginModulesInDependencyOrder(entries, { loadModule, signal: controller.signal });
      // "a" (already in flight when abort fired) settles normally; "b" (level 1, not yet started) never starts.
      expect(started).toEqual(["a"]);
      expect(result.handles.map((handle) => handle.pluginId)).toEqual(["a"]);
      expect(result.loadFailures.some((failure) => failure.pluginId === "b")).toBe(true);
    });
  });
  //#endregion 🧪️terra-web-plugin-runtime
}
//#endregion 🧪️Tests
