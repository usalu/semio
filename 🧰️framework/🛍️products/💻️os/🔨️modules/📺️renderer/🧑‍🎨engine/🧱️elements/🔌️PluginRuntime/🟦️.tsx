/// <reference types="vitest/importMeta" />
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🔌️PluginRuntime/component.tsx
/** @emoji 🔌️ `🔌️PluginRuntime` — the `PluginWasmHandle` binary-channel adapter (`loadPluginModule`/
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
 * instance}}` entries `⚛️reactor/🦀️.rs`'s `route_app_frame` wraps every non-`UiPatch`
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
  type PluginUiRefreshSectionResponse,
  SemioFaultError,
  GUEST_HOST_ANSWER_CEILING_BYTES,
  guestAnswerPages,
  type UiRefreshSection,
  type UiRefreshSectionKey,
  UI_REFRESH_SECTIONS,
  orderPluginRegistryEntries,
  panelViewContext,
  sectionViewContext,
  windowViewContext,
} from "@semio-tech/framework";
import { packedTextLeaf } from "./packed-text.ts";
import { AppChannelClient, AppChannelRequestSequence, type AppFrameValue, type WindowConfigPackEntry, decodeAppCommand, decodeAppFrame, decodeConflictsFromWire, decodeFaultFromWire, decodeInvocationResultPacks, decodeMergeReportFromWire, decodeMutationEnvelopesPack, decodePackValue, decodePackWire, encodeAppFrame, encodePackValue, faultDisplayMessage, packWireNatural } from "@semio-tech/framework-os";
import {
  DOCUMENT_BACKBONE_RETENTION_LIMITS,
  decodeLocalInteractionCaptureJson,
  LOCAL_INTERACTION_CAPTURE_MAX_BYTES,
  localInteractionIdentityEquals,
  type ArtifactPresencePeer,
  type LocalInteractionCapture,
} from "@semio-tech/framework-replication";
import { type BuiltNode, type Component, type UiNodeRecord, type UiPatchOp, type UiSnapshot } from "@semio-tech/framework";
import { applyUiPatch, DEFAULT_UI_DOCUMENT_LIMITS, emptyUiDocumentState, type UiDocumentState } from "../📃️UiDocumentStore/🟦️.tsx";
import { OwnedUiPatchIntake, retainedUiIntakeStepCeiling } from "../📃️UiDocumentStore/📥️intake/🟦️.ts";
import {
  ActivationRegistry,
  type ActivationReason,
  createTurnOutcomeBroadcast,
  fetchDescriptorManifest,
  type PluginWasmHandle as KernelPluginWasmHandle,
  type TurnOutcome,
} from "../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
export { fetchDescriptorManifest };
import {
  assertShardJspiAvailable,
  createShardCommandIngressPages,
  isShardLostError,
  SHARD_LIVENESS_POLICY,
  ShardClient,
  settleFailedInstanceOpen,
  type OwnedNativeUiPatchAuthority,
  type OwnedNativeUiPatchSubmissionReceipt,
  type ShardActorActivationLease,
  type ShardBudget,
  type ShardCommandIngressPage,
  type ShardEventEnvelope,
  type ShardInstanceLifecycleLease,
  type ShardInstanceOpenInput,
  type ShardJobStep,
  type ShardWorkerLike,
} from "../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { SHARD_WORKER_URL } from "../../../../../../../🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts";
import type { ActorInstanceLifecycleReceipt } from "../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts";
import { decodeActorUiPatchReceipt, encodeActorUiPatchReceipt } from "../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts";
import { OwnedResidentLedger } from "../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️.ts";
import { rendererResidentLedger } from "../../💾️resident/🟦️.ts";
import { OwnedUiInstance, type OwnedUiInstanceRetirement, type OwnedUiInstanceSurface, type OwnedUiPatchAcknowledgement } from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import type { RetainedUiNodeRecord } from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts";
import { TurnScheduler, type Lane } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts";
import { wireExtensionInvocation } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts";
import { type PluginManifest, type ViewModel } from "../🐚️Shell/🟦️.tsx";
import { SEGMENTED_DOWNLOAD_MARKER_PREFIX } from "../📤️SegmentedDownload/🟦️.ts";
import { BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, decodeBackboneMessage } from "@semio-tech/framework-os";
import { ActorDocumentBindingV1, type ActorDocumentMessagePortV1, type ActorDocumentSourceV1, encodeDocumentBackboneControlV1 } from "../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️plugin-runtime

/** 🎟️ Captures requester activation and request identity before extension work or queue admission. */
export interface PluginExtensionCompletion {
  readonly instanceId: number;
  readonly req: bigint;
  assertActive(): void;
  complete(outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array }): Promise<InvocationResponse>;
}

export type PluginDocumentBindingV1 = ActorDocumentSourceV1 & Readonly<{
  current(): boolean;
  send(payload: Uint8Array): void;
  prepared?(port: ActorDocumentMessagePortV1): void;
  merge?(conflicts: readonly Conflict[] | null, report: MergeReport | null): void;
}>;

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
  readonly contextMenu: (instanceId: number, request: PluginContextMenuRequest, viewState: ViewModel) => Promise<readonly ContextMenuItemSpec[]>;
  /** 🧾️ Complete projection used to seed or resynchronize host-owned history state. */
  readonly readHistory: (instanceId: number) => Promise<HistoryPatch>;
  /** 🔗️ The `DocumentApp` document-sync surface (WS-D) — optional since not every program has migrated onto it yet (WS-F).
   * `protocol_channel::AppCommand` carries binary `pack`/`spr` document-container bytes only
   * (`LoadDocument`/`ReadDocument`, backed by `store::print_document_pack`/`parse_document_pack`'s
   * deflate+BLAKE3 `.spk` container) — there is no JSON-text document command on the channel. The OLD
   * `readAppDocument`/`loadAppDocument` pair (plain JSON text — `MutationEnvelope[]` / a VCS envelope
   * string) has been retired along with every call site that used to feature-detect it;
   * {@link readAppDocumentPack} and {@link loadAppDocumentPack} are the channel-native replacement,
   * both round-tripping the same `.spk` container `documentPack` caches. */
  /** ⚖️ `AppCommand::ApplyEnvelopes`'s reply batches `MergeReport`/`Conflicts` frames alongside the
   * ingest itself (contract freeze §C6/§C9 "pushed unsolicited after every ingest") — decoded here,
   * same shape as {@link resolveConflict}'s reply, so a REMOTE peer's quarantined/degraded merge
   * reaches the caller instead of being dropped after the `Error` check. */
  readonly applyMutations?: (
    instanceId: number,
    mutationsPack: string,
  ) => Promise<{ readonly mergeReport: MergeReport | null; readonly conflicts: readonly Conflict[] | null }>;
  /** 📖️ Binary pack+spr document read (`AppCommand::ReadDocument`) — the channel-native counterpart
   * to {@link loadAppDocumentPack}; `null` when the reply carries no `AppFrame::Document` frame. */
  readonly readAppDocumentPack?: (instanceId: number) => Promise<{ readonly pack: Uint8Array; readonly spr: Uint8Array; readonly ops?: string } | null>;
  /** 📂️ Binary pack+spr document load (`AppCommand::LoadDocument`) — the Wave-1 channel-native path. */
  readonly loadAppDocumentPack?: (instanceId: number, pack: Uint8Array, spr: Uint8Array) => Promise<void>;
  /** 🪟️ Reads every concrete window's persisted-local config envelope. */
  readonly readWindowConfigPacks: (instanceId: number) => Promise<readonly WindowConfigPackEntry[]>;
  /** 🪟️ Restores one concrete window config envelope before its first render. */
  readonly loadWindowConfigPack: (instanceId: number, entry: WindowConfigPackEntry) => Promise<void>;
  readonly bindDocumentPort?: (instanceId: number, binding: PluginDocumentBindingV1) => Promise<ActorDocumentMessagePortV1>;
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
  /** 🔁️ Binds one canonical Completed submission to the originating activation before evaluation. */
  readonly captureExtensionCompletion?: (instanceId: number, req: bigint) => PluginExtensionCompletion;
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
   * `AppChannelClient.setMergePolicy` (`💻️os/🟦️.ts`); throws on an `AppFrame::Error` reply
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
  /** 🏠️ Reads one exact retained local-interaction capture and ACKs every native-owned page. */
  readonly readLocalInteraction: (instanceId: number, signal?: AbortSignal) => Promise<LocalInteractionCapture>;
  /** 🏁️ Subscribes to this instance's typed-operation completions (`AppFrame::OperationCompleted`).
   * A retained operation reaches its terminal result on a continuation turn no host call is awaiting,
   * so its final UI scope, history delta and host effects reach the shell HERE and nowhere else.
   * Returns the unsubscribe. */
  readonly subscribeOperationCompletions: (instanceId: number, listener: (completion: PluginOperationCompletion) => void) => () => void;
  readonly dispose: () => Promise<void>;
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
/** 🛣️ 🩺️ The shard worker URL is now the ONE `SHARD_WORKER_URL` `🎭️actor/🧵️shard-runtime/🟦️.ts`
 * already publishes (and the wgpu bridge already uses), asserted equal to the schema-owned
 * distribution route authority (`🔌️plugin/📇️registry/📦️deployment/🛣️routes.json` +
 * `MODULE_SHARD_DIRECTORY` + `SHARD_WORKER_FILE`) by that module's own suite.
 *
 * This file used to declare its own worker path: an ASCII transliteration of the route (`_shard`
 * beneath an ASCII `plugin-modules`) that exists NOWHERE else in the repository. Vite answered it with the
 * SPA fallback (`200 text/html`, measured against the live served shell on 2026-09-05, versus
 * `200 text/javascript` for the canonical route), a module `Worker` refuses an HTML body, and that
 * failure reaches the page as a parent-side `error` event carrying no message at all. Every shard in
 * the pool therefore died at spawn, the boot logged four anonymous `shard N worker error Event` lines,
 * and the ~60 downstream `timeout loading <plugin>` faults all descended from that one dead path. */

/** ⛽️ Provisional constant turn budget — same honestly-flagged gap `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s
 * native `TURN_BUDGET` documents ("until the DRR scheduler threads a real per-lane one through");
 * this is that same budget's web twin, field-for-field against `ShardBudget`. */
const DEFAULT_SHARD_BUDGET: ShardBudget = { fuel: 50_000_000, wallMs: 100, memoryBytes: 256 * 1024 * 1024, uiNodes: 20_000, mailboxLen: 64, maxEffects: 64, maxPatchBytes: 1 << 20 };

/** 💼️ Isolated `step-job` slices that share one serialized actor admission.
 * One slice per admission lets a 120ms `fillBuildTick` poll starve a fill plan
 * (browser-measured 2026-09-10: 5 slices / 30s, still in token-admit, Count=0).
 * Native `ShardLoop::pump` grants many job slices per turn; the browser driver
 * must batch the same way. */
export function isolatedJobStepsPerSerializedAdmission(stepsPerYield: number): number {
  return Math.max(1, stepsPerYield);
}

/** 🗳️ True on every `stride` Isolated step so fillBuildTick can poll Count without grabbing every admission. */
export function isolatedJobUiPollEverySteps(step: number, stride = 128): boolean {
  return step > 0 && step % Math.max(1, stride) === 0;
}

let isolatedJobDriveDepth = 0;
let isolatedJobUiPollDue = 0;
const isolatedJobDriveListeners = new Set<() => void>();

function notifyIsolatedJobDrive(): void {
  for (const listener of isolatedJobDriveListeners) listener();
}

/** 🏗️ Marks an Isolated job pump as holding the actor so fillBuildTick can yield the serialize lock. */
export function beginIsolatedJobDrive(): void {
  isolatedJobDriveDepth += 1;
  notifyIsolatedJobDrive();
}

/** 🏗️ Ends one Isolated job pump; clears leftover UI-poll tokens when the last drive drops. */
export function endIsolatedJobDrive(): void {
  isolatedJobDriveDepth = Math.max(0, isolatedJobDriveDepth - 1);
  if (isolatedJobDriveDepth === 0) isolatedJobUiPollDue = 0;
  notifyIsolatedJobDrive();
}

/** 🏗️ Queues one fillBuildTick while a drive is active so the guest can poll Count without starving step-job. */
export function requestIsolatedJobUiPoll(): void {
  isolatedJobUiPollDue += 1;
  notifyIsolatedJobDrive();
}

/** 🏗️ Consumes one queued Isolated-job UI poll. */
export function takeIsolatedJobUiPoll(): boolean {
  if (isolatedJobUiPollDue <= 0) return false;
  isolatedJobUiPollDue -= 1;
  notifyIsolatedJobDrive();
  return true;
}

/** 🏗️ True while PluginRuntime is pumping an Isolated job. */
export function isolatedJobDriveIsActive(): boolean {
  return isolatedJobDriveDepth > 0;
}

/** 🏗️ Subscribe to Isolated-job drive / poll-token changes. */
export function subscribeIsolatedJobDrive(listener: () => void): () => void {
  isolatedJobDriveListeners.add(listener);
  return () => {
    isolatedJobDriveListeners.delete(listener);
  };
}

/** 🏗️ Snapshot of the Isolated-job drive store for `useSyncExternalStore`. */
export function isolatedJobDriveSnapshot(): { readonly driving: boolean; readonly pollDue: number } {
  return { driving: isolatedJobDriveDepth > 0, pollDue: isolatedJobUiPollDue };
}


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
 * `ActivationRegistry.handleShardLost` (`🎠️kernel/🟦️.ts`, already coordinator-verified) is
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
  readonly residentLedger: OwnedResidentLedger;
  readonly shardCount: number;
  readonly createWorker: () => ShardWorkerLike;
  readonly onActorTrap: (actorId: string, message: string) => void;
  readonly onShardLost: (shardIndex: number, actorIds: readonly string[]) => void;
} {
  return {
    residentLedger: rendererResidentLedger(),
    shardCount: poolConcurrency(),
    // 🎭️ A real DOM `Worker` satisfies `ShardWorkerLike` structurally at runtime (same claim
    // `🟦️.ts`'s own doc makes) — the cast only bridges `onmessage`/`onerror`'s
    // wider native `MessageEvent`/`ErrorEvent` handler types down to the interface's minimal
    // `{data: unknown}`/`unknown` shape, which a `MessageEvent`/`ErrorEvent` handler always satisfies.
    createWorker,
    onActorTrap: (actorId, message) => console.error(`[DEBUG] PluginRuntime: actor ${actorId} trapped: ${message}`),
    onShardLost: handlePluginShardLost,
  };
}

export { SHARD_LIVENESS_POLICY };

/** 🩺️ Typed boot fault for the ONE failure the shell retries rather than surfaces: the shard hosting
 * the primary plugin's first turn was taken down (watchdog kill under load, or a worker crash) and
 * `ShardClient.rebuild` already replaced it. Carries `code` so `windowFaultFromError` names the cause
 * on the error surface instead of leaking `shard 0 terminated` as the whole story. */
export const PLUGIN_BOOT_SHARD_LOST_FAULT = "plugin.boot.shard-lost";

export class PluginBootShardLostError extends Error {
  readonly code = PLUGIN_BOOT_SHARD_LOST_FAULT;
  constructor(pluginId: string, cause: unknown) {
    super(`${PLUGIN_BOOT_SHARD_LOST_FAULT}: primary program ${pluginId} lost its shard twice while booting (${cause instanceof Error ? cause.message : String(cause)})`);
  }
}

/** 🫀️ Per-plugin proof-of-progress clock backing `loadPluginModuleResilient`'s IDLE deadline: a load
 * that is still moving (descriptor fetched, manifest registered, module handed over) must not be
 * killed just because the whole pipeline is slow under load — only one that has stopped moving. */
const pluginLoadProgress = new Map<string, number>();

export function notePluginLoadProgress(pluginId: string, atMs: number = Date.now()): void {
  pluginLoadProgress.set(pluginId, atMs);
}

export function pluginLoadProgressAt(pluginId: string): number | undefined {
  return pluginLoadProgress.get(pluginId);
}

let sharedShardClient: ShardClient | null = null;
function getShardClient(): ShardClient {
  if (sharedShardClient) return sharedShardClient;
  // 🧪️ Probed ONCE on the main thread before the first worker exists. Without JSPI every shard's
  // script throws at module top level, `worker.onerror` fires for all of them, and the boot degrades
  // into dozens of unrelated-looking `timeout loading <plugin>` lines with the real cause nowhere in
  // the log — observed verbatim on 2026-09-05 (four `shard N worker error Event` lines, no beacon).
  assertShardJspiAvailable();
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
 * (`🧵️TaskManager/🟦️.tsx`'s own header doc: still registrar-only, unmounted work) — the
 * registry's own ONLY real subscriber — is even open. The deliberate choice belongs to whichever
 * construction site mounts that consumer (`🏛️ShellHost`, registrar-only per this packet's lease) —
 * turning this on here would silently change a shared default for every other current/future
 * consumer of this singleton, exactly what `autoStartMetricsPublisher` exists to prevent. */
let sharedActivationRegistry: ActivationRegistry | null = null;
function getActivationRegistry(): ActivationRegistry {
  sharedActivationRegistry ??= new ActivationRegistry({ shardClient: getShardClient(), defaultBudget: DEFAULT_SHARD_BUDGET });
  return sharedActivationRegistry;
}

/** 🚧️ Best-effort JS representation of one raw WIT `effect`/`patch-op` variant crossing the wasm
 * boundary — UNVERIFIED against a real compiled artifact (no plugin has migrated onto `world actor`
 * yet; W3 hasn't started — same gap `🧵️shard-client.ts`'s and `🟦️.ts`'s own
 * header docs flag). Assumed shape: jco's standard variant binding, `tag` the WIT case name
 * (kebab-case) and `val` its payload record (fields camelCased from kebab), matching the SAME
 * convention this ticket's other packets already documented for this exact boundary. */
export type WireVariant<T = unknown> = { readonly tag?: string; readonly val?: T };

type WireUiPatch = {
  readonly surface?: { readonly instance?: number; readonly surface?: string };
  readonly kind?: string;
  readonly revision?: number | bigint;
  readonly baseRevision?: number | bigint;
  readonly ops?: readonly WireVariant[];
};

export type WireTurnResult = {
  readonly original?: object;
  readonly lifecycleReceipt?: Uint8Array;
  readonly uiPatchReceipt?: Uint8Array;
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
  const lifecycleReceipt = record.lifecycleReceipt;
  if (lifecycleReceipt !== undefined && lifecycleReceipt !== null && !(lifecycleReceipt instanceof Uint8Array)) throw new Error("actor-lifecycle.receipt-bytes");
  const uiPatchReceipt = record.uiPatchReceipt;
  if (uiPatchReceipt !== undefined && uiPatchReceipt !== null && !(uiPatchReceipt instanceof Uint8Array)) throw new Error("actor-ui-patch.receipt-bytes");
  return { original: raw !== null && typeof raw === "object" ? raw : undefined, lifecycleReceipt: lifecycleReceipt ?? undefined, uiPatchReceipt: uiPatchReceipt ?? undefined, uiPatches, effects, nextWake, status, commandIngress };
}

/** 🧯️ Decodes the scalar WIT command-ingress fault envelope while retaining readable kernel
 * rejection codes used by pre-decode validation paths. */
function commandIngressFaultDisplay(status: WireVariant | undefined): string {
  if (status?.tag !== "fault" || !status.val || typeof status.val !== "object") return "unknown fault";
  const fault = (status.val as { readonly fault?: unknown }).fault;
  const option = fault && typeof fault === "object" && "tag" in fault ? (fault as { readonly tag?: unknown; readonly val?: unknown }) : null;
  if (option && option.tag === "none") return "unknown fault";
  const raw = option && option.tag === "some" ? option.val : fault && typeof fault === "object" && "val" in fault ? (fault as { readonly val?: unknown }).val : fault;
  if (raw === undefined || raw === null) return "unknown fault";
  const bytes = coerceWireBytes(raw);
  const decoded = faultDisplayMessage(Array.from(bytes), decodePackValue);
  if (decoded !== "unknown fault") return decoded;
  const text = new TextDecoder().decode(bytes).trim();
  return text.length > 0 ? text : decoded;
}

/** 🔀️ `Effect::SendMessage{target: Shell{instance}}` → the raw `AppFrame` bytes it wraps —
 * `⚛️reactor/🦀️.rs`'s `route_app_frame` puts EVERY non-`UiPatch` `AppFrame` reply here
 * (design-abi.md §2). Mirrors `🦀️.rs`'s native `apply_turn_result` (H3-wgpu-native) — same
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
const DIRECTORY_PROJECTION_RECEIPT_SCHEMA = "semio.space.home.directory-projection-receipt.v1";
const TYPED_OPERATION_TERMINAL_OUTPUT = "typed-operation-terminal-output";
const TYPED_OPERATION_PENDING_OUTPUT = "typed-operation-pending-output";
const TYPED_OPERATION_TERMINAL_SEEN = "typed-operation-terminal-seen";

/** 🛤️ Highest `TypedOperationResultLane` discriminant the Rust host emits (`🔌️plugin/🦀️.rs` `TypedOperationResultLane` → byte 25 of a result page: Artifact 0 … Fault 11, Interaction 12, WindowTransient 13, WindowConfig 14). */
const TYPED_OPERATION_RESULT_LANE_MAX = 14;

function typedOperationResult(effect: WireVariant): { readonly acknowledgement: ShardEventEnvelope; readonly lane: number; readonly payload: Uint8Array; readonly operation: bigint; readonly sequence: number } | null {
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
  if (Number(value.target.val) !== receiver || lane > TYPED_OPERATION_RESULT_LANE_MAX || length > 4_096 || body.length !== 30 + length) throw new Error("typed-operation result violates its receiver, lane, or page authority");
  const ack = new Uint8Array(TYPED_OPERATION_ACK_MAGIC.length + 25);
  ack.set(TYPED_OPERATION_ACK_MAGIC);
  ack.set(body.subarray(0, 25), TYPED_OPERATION_ACK_MAGIC.length);
  return { acknowledgement: { kind: "message", payload: { source: { tag: "shell", val: String(receiver) }, payload: Array.from(ack) } }, lane, payload: body.subarray(30), operation: view.getBigUint64(4, true), sequence: view.getUint32(20, true) };
}

function typedOperationAcknowledgements(result: WireTurnResult): ShardEventEnvelope[] {
  return result.effects.flatMap((effect) => {
    const page = typedOperationResult(effect);
    return page ? [page.acknowledgement] : [];
  });
}

/** 🚨️ Real fault code a typed-operation fault page is surfaced under when no host call may be
 * rejected with it — its operation never revealed a first result page to a live call, or the call that
 * owned it has already returned. Rejecting an arbitrary in-flight caller would misattribute someone
 * else's failure, which is exactly the defect this router exists to remove. */
const TYPED_OPERATION_UNATTRIBUTED_FAULT = "interactive-job.unattributed-result-fault";

/** 🚨️ Real fault code the parked-result registry evicts its oldest entry under once its fixed
 * capacity is exhausted — an evicted fault is always surfaced, never silently dropped. */
const TYPED_OPERATION_PARK_EVICTION_FAULT = "interactive-job.parked-result-capacity";

/** 📏️ Foreign fault pages one instance's router parks for their owning host call before the oldest is
 * evicted through {@link TYPED_OPERATION_PARK_EVICTION_FAULT}. */
const TYPED_OPERATION_PARK_CAPACITY = 32;

/** 🚨️ Shell fault path for a typed-operation fault page no host call may be rejected with. */
function reportTypedOperationFault(code: string, message: string): void {
  console.error(`${code}: ${message}`);
}

/** 🧭️ One host call's typed-operation ownership scope — one queued command turn, one `refreshUi`, one
 * instance-open settle, one extension completion, one document-port exchange. Opened by
 * {@link TypedOperationRouter.open}, closed exactly once by {@link withTypedOperationCall}. */
class TypedOperationCall {
  constructor(readonly router: TypedOperationRouter, readonly label: string) {}
  observe(operation: bigint, sequence: number): boolean { return this.router.observe(this, operation, sequence); }
  fault(operation: bigint, sequence: number, message: string): boolean { return this.router.fault(this, operation, sequence, message); }
  takeFault(): string | null { return this.router.takeFault(this); }
  close(): void { this.router.close(this); }
}

/** 🧭️ Per-instance owner ledger routing typed-operation result pages to the host call that owns their
 * operation. Ownership is read off the wire the Rust host already publishes: `🔌️plugin/🦀️.rs`'s
 * `MountedTypedCommandFullOperation` mounts every operation at `result_sequence: 0` and bumps it per
 * acknowledged page (`acknowledge_result_page`), so the page carrying `sequence === 0` is that
 * operation's first reveal and names the call that dispatched it. A fault page observed inside a
 * foreign call's settle loop is parked for its owner — still acknowledged, since the guest blocks on
 * the ACK — instead of rejecting the observer; a fault whose owner already returned, or whose
 * operation never revealed a first page here, goes to the shell fault path exactly once. */
class TypedOperationRouter {
  private readonly owners = new Map<bigint, TypedOperationCall>();
  private readonly live = new Set<TypedOperationCall>();
  private readonly parked: { readonly call: TypedOperationCall; readonly operation: bigint; readonly message: string }[] = [];
  constructor(private readonly report: (code: string, message: string) => void = reportTypedOperationFault) {}
  open(label: string): TypedOperationCall {
    const call = new TypedOperationCall(this, label);
    this.live.add(call);
    return call;
  }
  observe(call: TypedOperationCall, operation: bigint, sequence: number): boolean {
    const owner = this.owners.get(operation);
    if (owner) return owner === call;
    if (sequence !== 0 || !this.live.has(call)) return false;
    this.owners.set(operation, call);
    return true;
  }
  fault(call: TypedOperationCall, operation: bigint, sequence: number, message: string): boolean {
    if (this.observe(call, operation, sequence)) return true;
    const owner = this.owners.get(operation);
    if (!owner || !this.live.has(owner)) {
      this.report(TYPED_OPERATION_UNATTRIBUTED_FAULT, `${message} (operation ${operation} sequence ${sequence} observed by ${call.label})`);
      return false;
    }
    if (this.parked.length >= TYPED_OPERATION_PARK_CAPACITY) {
      const evicted = this.parked.shift()!;
      this.report(TYPED_OPERATION_PARK_EVICTION_FAULT, `${evicted.message} (operation ${evicted.operation} parked for ${evicted.call.label} evicted at capacity ${TYPED_OPERATION_PARK_CAPACITY})`);
    }
    this.parked.push({ call: owner, operation, message });
    return false;
  }
  takeFault(call: TypedOperationCall): string | null {
    const index = this.parked.findIndex((entry) => entry.call === call);
    return index === -1 ? null : this.parked.splice(index, 1)[0]!.message;
  }
  close(call: TypedOperationCall): void {
    this.live.delete(call);
    for (const [operation, owner] of [...this.owners]) if (owner === call) this.owners.delete(operation);
    for (let index = this.parked.length - 1; index >= 0; index -= 1) {
      if (this.parked[index]!.call !== call) continue;
      const dropped = this.parked.splice(index, 1)[0]!;
      this.report(TYPED_OPERATION_UNATTRIBUTED_FAULT, `${dropped.message} (operation ${dropped.operation} outlived ${call.label})`);
    }
  }
}

/** 🧭️ One {@link TypedOperationRouter} per live plugin instance, keyed by its actor id (1:1 with the
 * instance, the same keying {@link retainedWindowByActor} uses) and dropped by
 * {@link teardownPluginActor}. */
const typedOperationRoutersByActor = new Map<string, TypedOperationRouter>();

/** 🧭️ Runs one host call under `actorId`'s typed-operation ownership scope: the call rejects with the
 * fault page parked for this exact call and never with a foreign operation's. */
async function withTypedOperationCall<T>(actorId: string, label: string, body: (call: TypedOperationCall) => Promise<T>): Promise<T> {
  let router = typedOperationRoutersByActor.get(actorId);
  if (!router) {
    router = new TypedOperationRouter();
    typedOperationRoutersByActor.set(actorId, router);
  }
  const call = router.open(`${actorId}:${label}`);
  try {
    const value = await body(call);
    const fault = call.takeFault();
    if (fault !== null) throw new Error(`typed-operation failed: ${fault}`);
    return value;
  } finally {
    call.close();
  }
}

function consumeTypedOperationEffects(effects: readonly WireVariant[], call?: TypedOperationCall): WireVariant[] {
  const consumed: WireVariant[] = [];
  let terminal = false;
  let terminalOutput: unknown = undefined;
  for (const effect of effects) {
    if (effect.tag === TYPED_OPERATION_PENDING_OUTPUT) {
      if (terminalOutput !== undefined) throw new Error("typed-operation emitted more than one directory projection receipt");
      terminalOutput = effect.val;
      continue;
    }
    if (effect.tag === TYPED_OPERATION_TERMINAL_SEEN) {
      terminal = true;
      continue;
    }
    const page = typedOperationResult(effect);
    if (!page) {
      consumed.push(effect);
      continue;
    }
    if (page.lane === 11) {
      const message = new TextDecoder().decode(page.payload);
      if (!call || call.fault(page.operation, page.sequence, message)) throw new Error(`typed-operation failed: ${message}`);
      continue;
    }
    call?.observe(page.operation, page.sequence);
    if (page.lane === 10) {
      terminal = true;
      continue;
    }
    if (page.lane === 7) {
      const event: unknown = JSON.parse(new TextDecoder().decode(page.payload));
      if (event && typeof event === "object" && !Array.isArray(event) && (event as { readonly kind?: unknown }).kind === DIRECTORY_PROJECTION_RECEIPT_SCHEMA) {
        if (terminalOutput !== undefined) throw new Error("typed-operation emitted more than one directory projection receipt");
        terminalOutput = (event as { readonly payload?: unknown }).payload;
      }
      continue;
    }
    if (page.lane !== 9) continue;
    const metadata: unknown = JSON.parse(new TextDecoder().decode(page.payload));
    if (!Array.isArray(metadata) || typeof metadata[0] !== "string" || typeof metadata[1] !== "string" || (metadata[2] !== null && metadata[2] !== "base64" && metadata[2] !== "identity")) throw new Error("typed-operation download metadata is invalid");
    consumed.push({ tag: "download-media-export", val: { filename: metadata[0], mimeType: metadata[1], data: String(page.operation), encoding: `${SEGMENTED_DOWNLOAD_MARKER_PREFIX}${metadata[2] ?? "identity"}` } });
  }
  if (terminalOutput !== undefined) {
    consumed.push({ tag: terminal ? TYPED_OPERATION_TERMINAL_OUTPUT : TYPED_OPERATION_PENDING_OUTPUT, val: terminalOutput });
  } else if (terminal) {
    consumed.push({ tag: TYPED_OPERATION_TERMINAL_SEEN, val: null });
  }
  return consumed;
}
//#endregion 📬️TypedOperationResult

//#region 🔖️RetainedUiPatch
/** 🩹️ `kernel::PatchOp`, TS twin restricted to what `⚛️reactor/🩹️patches/🦀️.rs`'s
 * `PatchTracker` actually emits this wave (its own doc: "full-body only — every dirty surface emits
 * one `PatchOp::Replace` at the root path"). `path` is `list<u32>` at the WIT boundary (an empty
 * array for the root).
 *
 * MIGRATION (semantic UI contract, ticket 26/08/20): the old recursive `UiNode` this file's `node`
 * payload used to carry no longer exists — `🛂️manifest/🟦️.ts`'s hand-written mirror was
 * deleted in favor of `semio-framework-ui-contract`'s flat, id-keyed `UiSnapshot`/`UiNodeRecord`
 * (`📃️UiDocumentStore`'s header doc). A "whole-body replace" is now a whole `UiSnapshot` (root pointer +
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
        decoded.push({ type: "upsert", ...normalizeWireUiNodeRecord(decodeWirePack(val.node, "upsert.node")) });
        break;
      case "set-component":
        decoded.push({ type: "setComponent", id: wireNatural(val.node, "op.node"), component: decodeWirePack(val.component, "set-component.component") as Extract<UiPatchOp, { type: "setComponent" }>["component"] });
        break;
      case "set-layout":
        decoded.push({ type: "setLayout", id: wireNatural(val.node, "op.node"), layout: decodeWirePack(val.layout, "set-layout.layout") as Extract<UiPatchOp, { type: "setLayout" }>["layout"] });
        break;
      case "set-activity": {
        const activity = decodeWirePack(val.activity, "set-activity.activity") as Pick<Extract<UiPatchOp, { type: "setActivity" }>, "activity" | "disabled">;
        decoded.push({ type: "setActivity", id: wireNatural(val.node, "op.node"), activity: activity.activity, disabled: activity.disabled });
        break;
      }
      case "set-children":
        decoded.push({ type: "setChildren", id: wireNatural(val.node, "op.node"), children: Array.isArray(val.children) ? val.children.map((child) => wireNatural(child, "set-children.children[]")) : [] });
        break;
      case "set-style":
        decoded.push({ type: "setStyle", id: wireNatural(val.node, "op.node"), style: decodeWirePack(val.style, "set-style.style") as Extract<UiPatchOp, { type: "setStyle" }>["style"] });
        break;
      case "set-accessibility":
        decoded.push({ type: "setAccessibility", id: wireNatural(val.node, "op.node"), accessibility: decodeWirePack(val.accessibility, "set-accessibility.accessibility") as Extract<UiPatchOp, { type: "setAccessibility" }>["accessibility"] });
        break;
      case "set-bindings":
        decoded.push({ type: "setBindings", id: wireNatural(val.node, "op.node"), bindings: decodeWirePack(val.bindings, "set-bindings.bindings") as Extract<UiPatchOp, { type: "setBindings" }>["bindings"] });
        break;
      case "set-menu":
        decoded.push({ type: "setMenu", id: wireNatural(val.node, "op.node"), menu: decodeWirePack(val.menu, "set-menu.menu") as Extract<UiPatchOp, { type: "setMenu" }>["menu"] });
        break;
      case "remove":
        decoded.push({ type: "remove", id: wireNatural(op.val, "remove.val") });
        break;
      case "set-root":
        decoded.push({ type: "setRoot", id: wireNatural(op.val, "set-root.val") });
        break;
      default:
        break;
    }
  }
  return decoded;
}

/** 🔢️ One natural number off the actor WIT/pack boundary — {@link packWireNatural}, the single
 * implementation the wgpu bridge shares, applied to whatever jco handed this shell. */
const wireNatural = packWireNatural;

/** 📦️ Decodes one pack-encoded WIT byte payload and projects its lossless integer carriers onto exact
 * JSON numbers — the only shape the UI contract twins (`UiNodeRecord`, components, layouts, styles)
 * declare. Wraps {@link decodePackWire} with this boundary's own `list<u8>` coercion. */
function decodeWirePack(raw: unknown, path: string): unknown {
  return decodePackWire(coerceWireBytes(raw), path);
}


function normalizeWireUiNodeRecord(raw: unknown): UiNodeRecord {
  const record = raw as Partial<UiNodeRecord>;
  return {
    ...(record as UiNodeRecord),
    id: wireNatural(record.id, "record.id"),
    disabled: record.disabled ?? false,
    transition: record.transition ?? null,
    bindings: Array.isArray(record.bindings) ? record.bindings : [],
    menu: record.menu ?? null,
    children: Array.isArray(record.children) ? record.children.map((child) => wireNatural(child, "record.children[]")) : [],
  };
}

/** 🗄️ The retained per-actor document — a `📃️UiDocumentStore`-shaped state, not the store class itself
 * (this file has no per-node React subscribers to serve; it only needs the flat table to hash/forward
 * to `refreshUi` callers). Reuses `📃️UiDocumentStore`'s own `uiDocumentStateFromSnapshot` so this file
 * and the React tree apply the identical algorithm to the identical wire shape — never a second,
 * drifting reimplementation. */
export type RetainedSurface = UiDocumentState;

/**
 * @emoji 🖼️ H1-react (design-runtime.md §1 `SceneStore` / packet brief item 2) — reconciles one
 * `UiPatch`'s ops onto `previous` (the last body this file retained for the surface), so the UI
 * thread reads an already-reconciled tree instead of awaiting a plugin turn. Reuses the transactional
 * `📃️UiDocumentStore` patch applicator so revision checks, graph validation, quotas, and every semantic
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
    revision: wireNatural(patch.revision, "patch.revision"),
    baseRevision: wireNatural(patch.baseRevision, "patch.baseRevision"),
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

/** 🪟️ Copies only an opaque surface payload whose native byte view cannot cross the host response boundary. */
function ownedUiComponentToBuilt(component: RetainedUiNodeRecord["component"]): Component {
  if (component.type !== "surface") return component;
  return { ...component, doc: { bytes: Array.from({ length: component.doc.bytes.length }, (_, index) => component.doc.bytes.byteAt(index)) } };
}
//#endregion 🔖️RetainedUiPatch

/** 🚧️ Best-effort conversion of a raw WIT `effect` variant (`{tag, val}`, see `WireVariant`'s doc for
 * the unverified-boundary caveat) into the friendly `Effect` union `🎠️kernel/🟦️.ts` already
 * declares — Rust `kernel::Effect`'s externally-tagged serde shape (`{effectName: {...fields}}` /
 * `"requestSync"`), which is what every downstream consumer (`applyHostEffects` and friends) already
 * expects. Covers the effect kinds this renderer actually branches on; an effect kind with no case
 * here degrades to an honest `[DEBUG]`-logged drop rather than guessing an unverified shape. */
function wireEffectToFriendly(effect: WireVariant): Effect | null {
  const val = (effect.val ?? {}) as Record<string, unknown>;
  // 🧬️ Every `request-id`-carrying effect nests its payload in a `*-params` record (`🔌️plugin/🧬️schema/📜️.wit`,
  // lowered by `⚛️reactor/🦀️.rs`'s `wit::Effect::…({ req, params })`); the request-less effects keep their
  // fields at the top level. `wireExtensionInvocation` already reads the nested shape — the cases below
  // used to read those same records flatly, which silently produced empty ids: a `dispatch-action` with
  // `action: ""` reached the plugin as `window kind procedural-main does not own action ""`.
  const params = (val.params && typeof val.params === "object" ? val.params : {}) as Record<string, unknown>;
  const str = (key: string): string => String(val[key] ?? "");
  const num = (key: string): number => Number(val[key] ?? 0);
  const optionValue = (raw: unknown): unknown => {
    if (raw && typeof raw === "object" && "tag" in raw) {
      const variant = raw as { readonly tag?: unknown; readonly val?: unknown };
      if (variant.tag === "none") return undefined;
      if (variant.tag === "some") return variant.val;
    }
    return raw;
  };
  const packField = (key: string): unknown => {
    const raw = optionValue(val[key]);
    return raw !== undefined ? decodeWirePack(raw, `wire.${key}`) : undefined;
  };
  const paramStr = (key: string): string => String(params[key] ?? "");
  const paramNum = (key: string): number => Number(params[key] ?? 0);
  const paramText = (key: string): string | undefined => (typeof params[key] === "string" ? (params[key] as string) : undefined);
  const paramPack = (key: string): unknown => {
    const raw = optionValue(params[key]);
    return raw !== undefined ? decodeWirePack(raw, `wire.params.${key}`) : undefined;
  };
  switch (effect.tag) {
    case "request-sync":
      return "requestSync";
    case "load-document":
      return { loadDocument: { pack: Array.from(coerceWireBytes(val.pack)), spr: Array.from(coerceWireBytes(val.spr)) } };
    case "download-media-export":
      return { downloadMediaExport: { filename: str("filename"), mimeType: str("mimeType"), data: str("data"), encoding: typeof val.encoding === "string" ? val.encoding : undefined } };
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
    case "request-file-open": {
      const readAsRaw = optionValue(params["read-as"] ?? params.readAs);
      const mapped = {
        requestFileOpen: {
          req: num("req"),
          accept: paramStr("accept"),
          readAs: typeof readAsRaw === "string" ? readAsRaw : undefined,
          importAction: paramStr("import-action") || paramStr("importAction"),
          multiple: Boolean(optionValue(params.multiple) ?? params.multiple),
        },
      };
      console.warn(`[DEBUG] request-file-open mapped ${JSON.stringify({ keys: Object.keys(params), mapped: mapped.requestFileOpen })}`);
      return mapped;
    }
    case "set-active-utility":
      return { setActiveUtility: { windowId: str("windowId"), utilityId: str("utilityId") } };
    case "set-active-tool":
      return { setActiveTool: { toolId: str("toolId") } };
    case "open-window":
      return { openWindow: { req: num("req"), kind: paramStr("kind"), params: paramPack("params") } };
    case "close-window":
      return { closeWindow: { window: num("window") } };
    case "dispatch-action": {
      const action = paramStr("action");
      // 🚫️ An id-less self re-dispatch can only ever be rejected by the owning window kind; drop it at
      // the boundary that decoded it rather than letting `makeEffectDispatchOne` turn it into an
      // unhandled `plugin.internal` rejection with an empty action id in its message.
      if (!action) {
        console.warn(`[DEBUG] wireEffectToFriendly: dispatch-action req=${num("req")} dropped — empty action id (${JSON.stringify(Object.keys(params))})`);
        return null;
      }
      return { dispatchAction: { req: num("req"), action, args: paramPack("args"), delayMs: paramNum("delayMs") } };
    }
    case "open-dialog":
      return { openDialog: { req: num("req"), dialogId: paramStr("dialogId"), args: paramPack("args") as Record<string, unknown> | undefined } };
    case "invoke-extension":
      return wireExtensionInvocation(effect);
    case "spawn-plugin-instance":
      return { spawnPluginInstance: { req: num("req"), pluginId: paramStr("pluginId"), appId: paramStr("appId"), osInstanceId: paramText("osInstanceId"), label: paramText("label"), documentJson: paramText("documentJson") } };
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

/** 🏁️ One typed operation's terminal publication, delivered to every subscriber of
 * {@link PluginWasmHandle.subscribeOperationCompletions}. A mounted operation finishes turns long
 * after the command that started it resolved, so NOTHING in the invocation response describes its
 * outcome — `uiScope`, `historyPatch` and `requestedEffects` are that outcome, and this is their only
 * delivery path. */
export type PluginOperationCompletion = Readonly<{
  instanceId: number;
  operation: number;
  revision: number;
  uiScope: InvocationResponse["uiScope"];
  historyPatch: HistoryPatch | undefined;
  requestedEffects: readonly Effect[];
}>;

/** 🎯️ Per-instance leftover effects produced by the CONTINUATION turns
 * {@link drainTypedOperations} drives, kept separate from {@link pendingTurnEffects}: that map is
 * owned by `performInvocation`, which drains it the microtask after its own command reply resolves.
 * A completion frame can ride the very outcome that resolves a command, so a shared map would let the
 * completion subscriber steal an in-flight invocation's own effects. Two owners, two carriers. */
const pendingCompletionEffects = new Map<number, WireVariant[]>();

/** 📏️ Fixed retained authority for {@link pendingCompletionEffects}: an operation publishes at most
 * one host effect per continuation turn, so a completion that never arrives would otherwise
 * accumulate without bound. Exceeding it fails closed rather than growing. */
const PLUGIN_OPERATION_EFFECT_CAPACITY = 4_096;

/** 📏️ Continuation turns {@link drainTypedOperations} submits before it gives up on one instance —
 * the drain twin of {@link PLUGIN_UI_CONTINUATION_LIMIT}, which bounds ONE settle rather than the
 * poll chain across settles. */
const PLUGIN_OPERATION_DRAIN_BUDGET = 4_096;

/** ⏱️ Upper bound on the re-poll delay a `TurnResult.next-wake` may request. `next-wake` has no
 * settled host-side unit yet (nothing else in this file consumes it), so it is honoured only as
 * "wake again, no later than this" — clamping keeps a stale or absolute value from parking a
 * finished operation's publication for minutes. */
const PLUGIN_OPERATION_WAKE_MAX_MS = 1_000;

/** 🔁️ The bounded poll loop behind {@link PluginWasmHandle.subscribeOperationCompletions}: submit
 * one continuation settle, stop the moment the actor reports anything other than `more-work`, and
 * yield one macrotask between polls so a long drain never starves rendering. Injected `live`/`settle`
 * (rather than a closure over one `loadPluginModule` scope) so the stop conditions this whole feature
 * rests on — stops at idle, stops when the instance closes, never exceeds its budget — are assertable
 * without a shard worker. */
async function drainTypedOperationTurns(
  budget: number,
  live: () => boolean,
  settle: () => Promise<{ readonly status: unknown; readonly nextWake: number | null }>,
  yieldTurn: () => Promise<void> = yieldPluginUiContinuation,
): Promise<{ readonly polls: number; readonly stopped: "idle" | "closed" | "budget"; readonly nextWake: number | null }> {
  for (let poll = 0; poll < budget; poll += 1) {
    if (!live()) return { polls: poll, stopped: "closed", nextWake: null };
    const settled = await settle();
    if (wireTurnStatusTag(settled.status) !== "more-work") return { polls: poll + 1, stopped: "idle", nextWake: settled.nextWake };
    await yieldTurn();
  }
  return { polls: budget, stopped: "budget", nextWake: null };
}

/** 🪪️ H1-react — instance ids must be unique across EVERY plugin, not just within one
 * `loadPluginModule` call: `pendingTurnEffects` above is keyed by `instanceId` alone and is shared
 * module-wide (mirrors `🦀️.rs`'s native `KernelClient` — `next_instance_id` lives on the ONE
 * global `KernelThreadState`, not per-plugin). A per-plugin-scoped counter would let two different
 * plugins both mint instance `1` and silently cross-read each other's leftover turn effects. */
let nextGlobalInstanceId = 1;

/** 🚦 H1-react — `🟨️shard-worker.js` rejects (not queues) a SECOND in-flight `turn` for the same
 * `actorId` ("shard worker: actor … already has a turn in flight", `🟦️.ts`'s
 * `inFlightTurnActors` guard: "two turn requests for the SAME actorId overlapping is a caller bug —
 * the scheduler's job to prevent, not this worker's"). The OLD adapter's `withSerializedPluginWasmHandle`
 * (deleted alongside `PluginWorkerClient`, `🎠️kernel/🟦️.ts`'s own doc comment names it)
 * queued concurrent per-call requests transparently — this is that same guarantee's replacement.
 *
 * 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): the ORIGINAL implementation
 * here was a raw `Map<actorId, Promise>` FIFO chain — unbounded (a pointer-move burst queued 200 deep
 * rather than coalescing), lane-blind (an urgent action waited behind a stale background probe), and
 * un-cancellable. Both `serializePerActor` (below, a generic per-key run-this-thunk-serially utility —
 * KEPT with its exact existing signature/contract since `@semio-tech/framework-renderer-react`'s
 * `../🔌️PluginRuntime/🟦️.tsx` re-exports it and `🔬️index.test.ts` asserts that generic contract directly, outside
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
export function serializePerActor<T>(actorId: string, run: () => Promise<T>, lane: "Interactive" | "UserVisible" | "Background" | "Maintenance" = "Interactive"): Promise<T> {
  return new Promise<T>((resolve, reject) => {
    const backpressure = getThunkScheduler().enqueue(actorId, { lane, payload: { run, resolve: resolve as (value: unknown) => void, reject } });
    if (backpressure.kind === "rejected") reject(new Error(`[DEBUG] serializePerActor: actor ${actorId}'s queue is full (>${SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY} pending turns) — rejected rather than growing unbounded`));
  });
}

/** 📥️ Holds the actor's complete paged command-ingress sequence as one serialized unit. Every
 * direct poll operation uses the same key so redraw/completion turns cannot consume the command's
 * retained pending/terminal status and response effects before its channel caller observes them. */
export function serializeCommandIngressForActor<T>(actorId: string, run: () => Promise<T>, lane: "Interactive" | "UserVisible" | "Background" | "Maintenance" = "Interactive"): Promise<T> {
  return serializePerActor(`command-ingress:${actorId}`, run, lane);
}

/** 🎚 Catalog mesh registration is Background so reserved/user verbs stay Interactive and are not starved. */
export function commandIngressLaneForActionV1(actionId: string): "Interactive" | "Background" {
  return actionId === "registerBrushMesh" ? "Background" : "Interactive";
}

/** 🏷️ A command waiter hangs forever when the outcome has no in_reply_to matching its seq. */
export function commandIngressNeedsReplyStampV1(replySequences: readonly (number | null)[], seq: number | null): boolean {
  return seq !== null && !replySequences.some((reply) => reply === seq);
}

function inspectEncodedAppCommand(events: readonly Uint8Array[]): { actionId: string | null; seq: number | null; lane: "Interactive" | "Background" } {
  try {
    const command = decodeAppCommand(events[0]!);
    const seq = "Command" in command ? command.Command.seq : "ConfigCommand" in command ? command.ConfigCommand.seq : null;
    if (!("Command" in command)) return { actionId: null, seq, lane: "Interactive" };
    const invocation = decodePackValue(new Uint8Array(command.Command.command)) as { readonly address?: { readonly actionId?: unknown } } | null;
    const actionId = typeof invocation?.address?.actionId === "string" ? invocation.address.actionId : null;
    return { actionId, seq, lane: actionId === null ? "Interactive" : commandIngressLaneForActionV1(actionId) };
  } catch {
    return { actionId: null, seq: null, lane: "Interactive" };
  }
}

function encodedFrameReplySequence(bytes: Uint8Array): number | null {
  try {
    const value = Object.values(decodeAppFrame(bytes))[0] as { readonly in_reply_to?: unknown } | undefined;
    return value && typeof value.in_reply_to === "number" ? value.in_reply_to : null;
  } catch {
    return null;
  }
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
  readonly kind: "operation";
  events: readonly ShardEventEnvelope[];
  readonly commandPage?: ShardCommandIngressPage;
  readonly activation?: ShardActorActivationLease;
  readonly waiters: PluginTurnWaiter[];
  readonly coalesceMapKey?: string;
}

type PluginLifecycleWork =
  | { readonly kind: "open"; readonly input: ShardInstanceOpenInput }
  | { readonly kind: "poll" }
  | { readonly kind: "close" }
  | { readonly kind: "receipt-ack"; readonly receipt: ActorInstanceLifecycleReceipt; readonly retirement?: OwnedUiInstanceRetirement }
  | { readonly kind: "issued-ui-ack"; readonly source: OwnedNativeUiPatchAuthority; readonly token: OwnedUiPatchAcknowledgement };
type PluginLifecycleTurnResult = { readonly owner: ShardInstanceLifecycleLease; readonly raw: unknown; readonly turn: WireTurnResult; readonly submission: OwnedNativeUiPatchSubmissionReceipt | null };
type PluginLifecycleTurnPayload = { readonly kind: "lifecycle"; readonly owner: ShardInstanceLifecycleLease; readonly work: PluginLifecycleWork; readonly resolve: (result: PluginLifecycleTurnResult) => void; readonly reject: (error: unknown) => void };
type PendingPluginTurn = PluginTurnPayload | PluginLifecycleTurnPayload;

/** 🚪️ Only the captured lifecycle owner can dispatch retirement-authorized work. */
async function runPluginLifecycleTurn(owner: ShardInstanceLifecycleLease, work: PluginLifecycleWork, budget: ShardBudget): Promise<PluginLifecycleTurnResult> {
  let raw: unknown;
  let submission: OwnedNativeUiPatchSubmissionReceipt | null = null;
  switch (work.kind) {
    case "open": raw = await owner.open(work.input, budget); break;
    case "poll": raw = await owner.poll(budget); break;
    case "close": raw = await owner.close(budget); break;
    case "receipt-ack": raw = await owner.acknowledge(work.receipt, budget, work.retirement); break;
    case "issued-ui-ack": { const result = await owner.submitUiAcknowledgement(work.source, work.token, budget); raw = result.result; submission = result.receipt; break; }
    default: throw new Error("actor-lifecycle.work-kind");
  }
  return Object.freeze({ owner, raw, turn: coerceTurnResult(raw), submission });
}

/** 🧮️ Matches `ActivationRegistry`'s own `DEFAULT_TURN_MAILBOX_CAPACITY` — no reason for this file's
 * per-actor turn queue to be shaped differently from the kernel's own default. */
const PLUGIN_TURN_MAILBOX_CAPACITY = 32;

const pendingCoalescedTurns = new Map<string, PluginTurnPayload>();
const pendingLifecycleTurns = new Map<string, number>();
const tearingDownPluginActors = new Set<string>();

let sharedPluginTurnScheduler: TurnScheduler<PendingPluginTurn, ShardBudget> | null = null;
/** 🧵️ Reads `getShardClient()` INSIDE `runTurn` (not once at construction) purely so a test can swap
 * the module-private `sharedShardClient` for a fake between calls without this scheduler ever pinning
 * itself to whichever shard client happened to exist first — in production there is only ever one. */
function getPluginTurnScheduler(): TurnScheduler<PendingPluginTurn, ShardBudget> {
  sharedPluginTurnScheduler ??= new TurnScheduler<PendingPluginTurn, ShardBudget>({
    mailboxCapacity: PLUGIN_TURN_MAILBOX_CAPACITY,
    budgetFor: () => DEFAULT_SHARD_BUDGET,
    runTurn: async (actorId, payload, budget) => {
      if (payload.kind === "lifecycle") releasePendingLifecycleTurn(actorId);
      if (payload.kind === "operation" && payload.coalesceMapKey) pendingCoalescedTurns.delete(payload.coalesceMapKey);
      try {
        if (payload.kind === "lifecycle") { payload.resolve(await runPluginLifecycleTurn(payload.owner, payload.work, budget)); return; }
        payload.activation?.assertActive();
        const raw = await (payload.activation ? payload.activation.turn(payload.events, budget, payload.commandPage) : getShardClient().turn(actorId, payload.events, budget, payload.commandPage));
        payload.activation?.assertActive();
        const result = coerceTurnResult(raw);
        for (const waiter of payload.waiters) waiter.resolve(result);
      } catch (error) {
        if (payload.kind === "lifecycle") payload.reject(error);
        else for (const waiter of payload.waiters) waiter.reject(error);
        if (!tearingDownPluginActors.has(actorId)) throw error;
      }
    },
    onTurnError: (actorId, error) => console.error(`[DEBUG] PluginRuntime: turn failed for actor ${actorId}`, error),
  });
  return sharedPluginTurnScheduler;
}

function releasePendingLifecycleTurn(actorId: string): void {
  const remaining = (pendingLifecycleTurns.get(actorId) ?? 1) - 1;
  if (remaining === 0) pendingLifecycleTurns.delete(actorId); else pendingLifecycleTurns.set(actorId, remaining);
}

function enqueuePluginTurn(actorId: string, payload: PendingPluginTurn, lane: Lane, coalesce?: string): ReturnType<TurnScheduler<PendingPluginTurn, ShardBudget>["enqueue"]> {
  const scheduler = getPluginTurnScheduler();
  if ((payload.kind === "lifecycle" || pendingLifecycleTurns.has(actorId)) && scheduler.pendingCount(actorId) >= PLUGIN_TURN_MAILBOX_CAPACITY) return { kind: "rejected" };
  const backpressure = scheduler.enqueue(actorId, { lane, coalesce, payload });
  if (payload.kind === "lifecycle" && backpressure.kind !== "rejected") pendingLifecycleTurns.set(actorId, (pendingLifecycleTurns.get(actorId) ?? 0) + 1);
  return backpressure;
}

/**
 * 🚦 This file's own internal turn-dispatch seam — replaces the old unbounded `actorTurnQueue` chain.
 * `lane` prioritizes across an actor's own pending turns (`"Interactive"` for anything a caller awaits
 * a specific reply from — `runQueuedTurn`/`createApp`/`captureExtensionCompletion` all use it below —
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
function submitPluginTurn(actorId: string, events: readonly ShardEventEnvelope[], lane: Lane, coalesceKey?: string, commandPage?: ShardCommandIngressPage, activation?: ShardActorActivationLease): Promise<WireTurnResult> {
  return new Promise<WireTurnResult>((resolve, reject) => {
    if (activation && (activation.actorId !== actorId || coalesceKey !== undefined)) throw new Error("actor-activation.turn-owner-mismatch");
    activation?.assertActive();
    const waiter: PluginTurnWaiter = { resolve, reject };
    if (coalesceKey !== undefined) {
      const mapKey = `${actorId} ${coalesceKey}`;
      const pending = pendingCoalescedTurns.get(mapKey);
      if (pending) {
        pending.events = events; // 🎯️ latest-wins: the mailbox still holds THIS SAME payload object.
        pending.waiters.push(waiter);
        return;
      }
      const payload: PluginTurnPayload = { kind: "operation", events, waiters: [waiter], coalesceMapKey: mapKey, commandPage };
      pendingCoalescedTurns.set(mapKey, payload);
      const backpressure = enqueuePluginTurn(actorId, payload, lane, coalesceKey);
      if (backpressure.kind === "rejected") {
        pendingCoalescedTurns.delete(mapKey);
        reject(new Error(`[DEBUG] PluginRuntime: actor ${actorId}'s turn queue is full — rejected rather than growing unbounded`));
      }
      return;
    }
    const payload: PluginTurnPayload = { kind: "operation", events, waiters: [waiter], commandPage, activation };
    const backpressure = enqueuePluginTurn(actorId, payload, lane);
    if (backpressure.kind === "rejected") reject(new Error(`[DEBUG] PluginRuntime: actor ${actorId}'s turn queue is full — rejected rather than growing unbounded`));
  });
}

/** 📨️ Lifecycle work shares actor serialization without borrowing revoked command authority. */
function submitPluginLifecycleTurn(owner: ShardInstanceLifecycleLease, work: PluginLifecycleWork, lane: Lane): Promise<PluginLifecycleTurnResult> {
  return new Promise((resolve, reject) => {
    let captured: PluginLifecycleWork;
    switch (work.kind) {
      case "open": captured = Object.freeze({ kind: work.kind, input: work.input }); break;
      case "poll": case "close": captured = Object.freeze({ kind: work.kind }); break;
      case "receipt-ack": captured = Object.freeze({ kind: work.kind, receipt: work.receipt, retirement: work.retirement }); break;
      case "issued-ui-ack": captured = Object.freeze({ kind: work.kind, source: work.source, token: work.token }); break;
      default: throw new Error("actor-lifecycle.work-kind");
    }
    const actorId = owner.activation.actorId;
    const payload: PluginLifecycleTurnPayload = { kind: "lifecycle", owner, work: captured, resolve, reject };
    const backpressure = enqueuePluginTurn(actorId, payload, lane);
    if (backpressure.kind === "rejected") reject(new Error("actor-lifecycle.queue-full"));
  });
}

function teardownPluginActor(actorId: string): void {
  const fault = new Error(`plugin actor ${actorId} disposed`);
  tearingDownPluginActors.add(actorId);
  getPluginTurnScheduler().teardownActor(actorId, (payload) => {
    if (payload.kind === "lifecycle") { releasePendingLifecycleTurn(actorId); payload.reject(fault); return; }
    if (payload.coalesceMapKey) pendingCoalescedTurns.delete(payload.coalesceMapKey);
    for (const waiter of payload.waiters) waiter.reject(fault);
  });
  getThunkScheduler().teardownActor(`command-ingress:${actorId}`, (payload) => payload.reject(fault));
  typedOperationRoutersByActor.delete(actorId);
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
/** ⛔️ Consecutive continuations one settle may spend making NO progress — publishing no patch,
 * emitting no effect and producing no acknowledgement — before it is declared stalled and named.
 *
 * A continuation is a host round trip, and {@link PLUGIN_UI_CONTINUATION_LIMIT} counts every one of
 * them the same way, so a guest that answers `MoreWork` while publishing nothing spins 4 096 round
 * trips and then throws a timeout that names neither the stalled surface nor the reason. That is the
 * shape the Nakagin `interactionSelect`/`interactionHover` fault has (ticket 26/09/02, evidence
 * `w-ab-41-nakagin-brush4.txt`: `acks=0` for over a thousand consecutive continuations).
 *
 * The bound is derived from the retained document contract this renderer admits, not chosen: one
 * patch may carry at most `maxPatchBytes`, and the smallest owner a single guest retirement turn is
 * guaranteed to release is one maximal text (`maxTextBytes`), so a HEALTHY guest needs at most
 * `maxPatchBytes / maxTextBytes` publication-free turns per admitted patch; one whole continuation
 * batch of slack per such turn covers this loop's own acknowledgement cadence. */
const PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT = Math.ceil(DEFAULT_UI_DOCUMENT_LIMITS.maxPatchBytes / DEFAULT_UI_DOCUMENT_LIMITS.maxTextBytes) * PLUGIN_UI_CONTINUATION_BATCH_SIZE;
/** 😴️ The same bound read the other way: consecutive publication-free continuations a settle that has
 * NOTHING outstanding spends before it calls the actor quiesced FOR THIS TURN and returns what it
 * collected. One bound, two outcomes — a settle waiting on named surfaces that never come is a fault
 * ({@link pluginTurnStalledError}); a settle that asked for nothing has simply collected everything
 * this turn produces, and the poll behind `subscribeOperationCompletions` owns what comes later.
 *
 * It replaces ticket 26/09/02's `empty-required stop`, which ended such a turn at the FIRST
 * acknowledgement-free continuation. A guest keeps running its typed operation across continuations
 * and publishes nothing until it has something to publish, so that stop cut every mutation's turn
 * before its own publication: `addObjectKind`/`addTargetVolume` settled with
 * `frameKinds:["Invocation","Ephemeral"]` and `historyUpserts:0` and the edit never reached the world
 * lane (`📓️2026-09-11-wave-B11-editor-actions.md` §6, reproduced in §2 of W-B14's report). */
const PLUGIN_UI_QUIESCENT_CONTINUATIONS = PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT;
/** 📏️ Liveness backstop for one surface patch, priced off the retained document contract this renderer
 * admits — see {@link retainedUiIntakeStepCeiling} for why a patch-scaled budget is structurally wrong. */
const PLUGIN_UI_INTAKE_STEP_CEILING = retainedUiIntakeStepCeiling(DEFAULT_UI_DOCUMENT_LIMITS);
/** 🪃️ Intake steps per macrotask yield. One yield per 8 steps is the continuation cadence for TURNS,
 * where a step is a whole guest turn; an intake step is one wire phase costing ~3.4 µs, and a
 * Nakagin-scale world-3d publication takes 669 403 of them (`OwnedIntake drives a Nakagin-scale paged
 * scene-lane surface patch`), so that cadence spent 83 675 macrotasks — dwarfing the work itself. A
 * 1024-step stride is a ~3.5 ms slice, still well inside one frame, and 653 macrotasks for that patch. */
const PLUGIN_UI_INTAKE_YIELD_STRIDE = 1_024;
type PluginPatchAcceptance = Readonly<{
  acknowledgements: readonly ShardEventEnvelope[];
  turns: readonly WireTurnResult[];
}>;
function isPluginPatchAcceptance(value: readonly ShardEventEnvelope[] | PluginPatchAcceptance): value is PluginPatchAcceptance { return !Array.isArray(value); }

/** 🪃️ Macrotask yield for the UI continuation loops. A `setTimeout(0)` chain is throttled to one tick
 * per second in a hidden tab (once per minute under Chrome's intensive throttling), which turned a few
 * thousand intake steps into minutes of silence; a `MessageChannel` message is a macrotask that is never
 * throttled by page visibility. */
const pluginUiContinuationChannel = new MessageChannel();
const pluginUiContinuationWaiters: (() => void)[] = [];
pluginUiContinuationChannel.port1.onmessage = () => {
  const waiters = pluginUiContinuationWaiters.splice(0);
  for (const resume of waiters) resume();
};
async function yieldPluginUiContinuation(): Promise<void> {
  await new Promise<void>((resolve) => {
    pluginUiContinuationWaiters.push(resolve);
    pluginUiContinuationChannel.port2.postMessage(null);
  });
}

/** 🛑️ The attributable twin of the continuation timeout: it names the surfaces the settle is still
 * waiting on instead of only counting the round trips it spent. A settle waiting on named surfaces
 * reports the ones that never published; a drain reports the surfaces it DID publish, because those
 * are the owners whose retirement is holding the guest in `MoreWork`. */
function pluginTurnStalledError(actorId: string, results: readonly WireTurnResult[], requiredSurfaceIds: ReadonlySet<string> | undefined, zeroProgress: number, continuations: number, call?: TypedOperationCall): Error {
  const published = new Set(results.flatMap((result) => result.uiPatches.map(wirePatchSurfaceId).filter((surface): surface is string => surface !== null)));
  const missing = [...(requiredSurfaceIds ?? [])].filter((surface) => !published.has(surface));
  const pending = missing.length > 0 ? missing : [...published];
  return new Error(
    `[DEBUG] PluginRuntime: actor ${actorId} published, acknowledged and emitted nothing for ${zeroProgress} consecutive continuations ` +
      `(operation=${call?.label ?? "none"}, pending=${JSON.stringify(pending)}, required=${JSON.stringify([...(requiredSurfaceIds ?? [])])}, ` +
      `continuations=${continuations}, status=${wireTurnStatusTag(results.at(-1)?.status)}, zeroProgressLimit=${PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT})`,
  );
}

/** 🔄️ Drives a reactor-owned continuation until its requested patch set is published or the actor
 * quiesces. UI reconciliation is deliberately incremental: the turn that marks surfaces dirty may
 * publish them across multiple MoreWork frames, and a guest running a typed operation publishes
 * nothing at all until that operation has something to publish — so a turn ends on PROGRESS, never on
 * one acknowledgement-free round trip. A supplied empty `requiredSurfaceIds` means every requested
 * surface is already retained, so an unchanged refresh needs no continuation at all; it does not mean
 * the actor has nothing left to hand this turn. Accepted patches are acknowledged between turns to
 * release bounded publication capacity. See {@link PLUGIN_UI_QUIESCENT_CONTINUATIONS} for the two
 * outcomes of a publication-free streak. */
async function settlePluginTurn(actorId: string, initial: WireTurnResult, lane: Lane, requiredSurfaceIds?: ReadonlySet<string>, acceptPatches?: (result: WireTurnResult) => readonly ShardEventEnvelope[] | PluginPatchAcceptance | Promise<readonly ShardEventEnvelope[] | PluginPatchAcceptance>, drainOperations = false, activation?: ShardActorActivationLease, call?: TypedOperationCall): Promise<WireTurnResult> {
  const results: WireTurnResult[] = [initial];
  const acknowledge = async (result: WireTurnResult): Promise<readonly ShardEventEnvelope[]> => {
    activation?.assertActive();
    const accepted = await acceptPatches?.(result) ?? [];
    if (!isPluginPatchAcceptance(accepted)) return [...accepted, ...typedOperationAcknowledgements(result)];
    results.push(...accepted.turns);
    return [...accepted.acknowledgements, ...typedOperationAcknowledgements(result), ...accepted.turns.flatMap(typedOperationAcknowledgements)];
  };
  const hasWork = () => (drainOperations || !hasRequiredUiPatches(results, requiredSurfaceIds)) && wireTurnStatusTag(results.at(-1)?.status) === "more-work";
  const outstanding = () => !hasRequiredUiPatches(results, requiredSurfaceIds);
  let acknowledgements = await acknowledge(initial);
  let quiesced = false;
  let zeroProgress = 0;
  for (let continuation = 0; !quiesced && (acknowledgements.length > 0 || hasWork()) && continuation < PLUGIN_UI_CONTINUATION_LIMIT; continuation += 1) {
    const collected = results.length;
    const continued = await submitPluginTurn(actorId, acknowledgements, lane, undefined, undefined, activation);
    results.push(continued);
    acknowledgements = await acknowledge(continued);
    const progressed = acknowledgements.length > 0 || results.slice(collected).some((turn) => turn.uiPatches.length > 0 || turn.effects.length > 0);
    zeroProgress = progressed ? 0 : zeroProgress + 1;
    if (zeroProgress >= PLUGIN_UI_QUIESCENT_CONTINUATIONS && !outstanding()) quiesced = true;
    else if (zeroProgress >= PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT) throw pluginTurnStalledError(actorId, results, requiredSurfaceIds, zeroProgress, continuation + 1, call);
    if (!quiesced && (continuation + 1) % PLUGIN_UI_CONTINUATION_BATCH_SIZE === 0 && hasWork()) {
      await yieldPluginUiContinuation();
    }
  }
  if (!quiesced && (acknowledgements.length > 0 || hasWork())) {
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
  activation?.assertActive();
  return {
    uiPatches: results.flatMap((result) => result.uiPatches),
    effects: consumeTypedOperationEffects(results.flatMap((result) => result.effects), call),
    nextWake: [...results].reverse().find((result) => result.nextWake !== null)?.nextWake ?? null,
    status: results.at(-1)?.status,
    commandIngress: results.at(-1)?.commandIngress,
  };
}
async function settleAcknowledgedPluginTurns(actorId: string, results: readonly WireTurnResult[], acknowledgements: readonly ShardEventEnvelope[], acceptPatches?: (result: WireTurnResult) => PluginPatchAcceptance | Promise<PluginPatchAcceptance>, activation?: ShardActorActivationLease, call?: TypedOperationCall): Promise<WireTurnResult> {
  const initial: WireTurnResult = {
    uiPatches: [],
    effects: [],
    nextWake: null,
    commandIngress: results.at(-1)?.commandIngress,
    status: results.at(-1)?.status,
  };
  const continued = await settlePluginTurn(actorId, initial, "Interactive", new Set(), (turn) => turn === initial ? acknowledgements : acceptPatches?.(turn) ?? [], true, activation, call);
  return {
    ...continued,
    uiPatches: [...results.flatMap((turn) => turn.uiPatches), ...continued.uiPatches],
    effects: consumeTypedOperationEffects([...results.flatMap((turn) => turn.effects), ...continued.effects], call),
    nextWake: continued.nextWake ?? [...results].reverse().find((turn) => turn.nextWake !== null)?.nextWake ?? null,
  };
}
//#endregion 🔖️PluginTurnScheduler

/** 🧪️ Language-neutral retained-patch oracle used by the in-source contract tests. Production
 * rendering is owned exclusively by `OwnedUiInstanceSurface` inside `loadPluginModule`. */
const retainedWindowByActor = new Map<string, Map<string, RetainedSurface>>();

function pluginSurfaceRef(instance: number, bodyKey: string): { readonly instance: number; readonly surface: string } {
  return { instance, surface: bodyKey };
}

/** 🪟 Leftover Viewport patches omit a window instance and default the guest dirty surface to `window`. */
export const DEFAULT_LEFTOVER_WINDOW_SURFACE = "window";

/** 📌️ Full chrome refresh once leftover InteractionView carries a selection the Inspection body must re-render. */
export function leftoverInspectionRefreshScope(selectedIds: readonly string[]): { readonly kind: "full" } | null {
  return selectedIds.length > 0 ? { kind: "full" } : null;
}

/** 📌️ First leftover pick must omit the cached Inspection hash — B4's `uiRefreshSectionUnchanged` skip kept the empty summary while leftover already published `selectedIds`. */
export function leftoverInspectionPanelHash(selectedIds: readonly string[], cachedHash: string | undefined): string | undefined {
  return selectedIds.length > 0 ? undefined : cachedHash;
}

/** 🖌️ Leftover vortex hover under an armed brush must omit the cached world-body hash — B4's
 * hash-conditional skip keeps the boot tree (`preview=null`) while the guest already published
 * a 250–313 byte preview on the same suggestionsTick. */
export function leftoverBrushPreviewWindowHash(activeUtility: string | null | undefined, hoveredVortexFullId: string | null | undefined, cachedHash: string | undefined): string | undefined {
  return activeUtility === "brush" && Boolean(hoveredVortexFullId?.includes(":")) ? undefined : cachedHash;
}

export function leftoverBrushPreviewRefreshScope(activeUtility: string | null | undefined, hoveredVortexFullId: string | null | undefined): { readonly kind: "full" } | null {
  return leftoverBrushPreviewWindowHash(activeUtility, hoveredVortexFullId, "cached") === undefined ? { kind: "full" } : null;
}

/** 🖌️ Leftover world refresh waits for suggestionsTick (or leftover-published preview JSON), not the first empty hover leftover. */
export function leftoverBrushPreviewRefreshReady(actionId: string | null | undefined, activeUtility: string | null | undefined, hoveredVortexFullId: string | null | undefined, leftoverPreviewJson?: string | null): boolean {
  if (leftoverBrushPreviewWindowHash(activeUtility, hoveredVortexFullId, "cached") !== undefined) return false;
  if (typeof leftoverPreviewJson === "string" && leftoverPreviewJson.length > 0) return true;
  return actionId === "suggestionsTick";
}

/** 🪟 Binds each authored window plus the leftover default `window` surface to a host view. */
export function windowHostContextBindings(
  instanceId: number,
  windows: readonly { readonly key: string; readonly bodyKey?: string }[],
  viewState: Parameters<typeof windowViewContext>[0],
): ReadonlyArray<{ readonly surface: { readonly instance: number; readonly surface: string }; readonly bodyKey: string; readonly windowKey: string }> {
  const bindings: Array<{ readonly surface: { readonly instance: number; readonly surface: string }; readonly bodyKey: string; readonly windowKey: string }> = [];
  let alias: (typeof bindings)[number] | undefined;
  for (const target of windows) {
    if (!target.bodyKey) continue;
    if (!windowViewContext(viewState, target.key)) continue;
    bindings.push({ surface: pluginSurfaceRef(instanceId, target.key), bodyKey: target.bodyKey, windowKey: target.key });
    alias = { surface: pluginSurfaceRef(instanceId, DEFAULT_LEFTOVER_WINDOW_SURFACE), bodyKey: target.bodyKey, windowKey: target.key };
  }
  if (alias) bindings.push(alias);
  return bindings;
}

function retainedSurfaceId(instance: number, surface: string): string {
  return `${instance}:${surface}`;
}

function uiRefreshBodyKeys(request: PluginUiRefreshRequest): string[] {
  return [...new Set([...(request.windows ?? []), ...(request.panels ?? [])].flatMap((target) => target.bodyKey ? [target.bodyKey] : []))];
}

/** 🧩️ The reserved section surfaces this request asks for, in contract order. Unlike a window or a
 * panel the shell never names their body: the section's identity IS its reserved body key, so it can
 * never collide with an app-authored one. */
function uiRefreshSectionTargets(request: PluginUiRefreshRequest): readonly UiRefreshSection[] {
  return UI_REFRESH_SECTIONS.filter((section) => request[section.key] !== undefined);
}

/** 🪞️ Whether the host already holds this section's CURRENT tree, i.e. whether the whole projection
 * may be skipped.
 *
 * `buildUiRefreshRequest` has always stamped every requested entry with the host's own cached hash
 * precisely so "the plugin can omit payloads for sections that didn't change" — and no projector ever
 * read it: both {@link retainedUiRefreshResponse} and the owned projector rebuilt every requested tree
 * on every refresh. That cost twice:
 *
 * 1. the retained walk itself (measured on the Nakagin document switch, ticket 26/09/02 wave B4:
 *    51 projections, 1 275 nodes, 132 ms of main-thread time for ONE switch), and
 * 2. — the larger term — a FRESH `BuiltNode` object for an unchanged body, which defeats
 *    `applyUiRefreshResponseToCache`/`mergeRecordPreservingIdentity` and therefore
 *    `InterpretedUiNode`'s `React.memo`: every refresh re-reconciled the entire shell subtree of a
 *    window nothing had touched.
 *
 * A response entry carrying only `{ key, hash }` is the contract's own shape for "unchanged"
 * (`PluginUiRefreshSectionResponse.value` is optional); the shell then keeps the exact object
 * reference it already dispatched. The hash on both sides is the same surface view hash, so equality
 * is exact, never heuristic. */
export function uiRefreshSectionUnchanged(cached: string | undefined, view: { readonly root: number | null; readonly hash: string | null }): boolean {
  return cached !== undefined && cached.length > 0 && view.root !== null && view.hash === cached;
}

/** 🧩️ Rebuilds one reserved section's value from its retained surface tree: the depth-first
 * concatenation of every text leaf is exactly the canonical JSON the plugin serialized (the tree is
 * an `UI_BUILT_CHILDREN_MAX`-ary carrier of packed `UI_TEXT_MAX_BYTES` leaves (`value` + `dataAttributes`), see `section_component_tree`
 * in `🔌️plugin/🦀️.rs`). No `packValueToExactJson` is involved: the payload crosses as text, so
 * `JSON.parse` reproduces the plugin's own numbers without an integer-carrier projection.
 *
 * 🧯️ A payload that does not parse is a TYPED fault naming the reserved section and the producing
 * actor, never a bare `SyntaxError`. Reading only `value` and dropping the 32 packed
 * `dataAttributes` slices per leaf handed `JSON.parse` an exact 512-byte prefix, and the shell
 * reported it as `Unterminated string in JSON at position 512` from two unrelated-looking call sites
 * (`ShellHost`'s session-refresh effect and its typed-operation completion pass) with nothing naming
 * the carrier — see ticket 26/09/09/PROCEDURAL-3D-END-TO-END `📓️json-512-truncation-2026-09-10.md`. */
function sectionValueFromBuiltNode(bodyKey: string, node: BuiltNode, producer: string): unknown {
  if (node.key !== bodyKey) throw new Error(`plugin-ui.section-root-mismatch:${node.key}`);
  let payload = "";
  const walk = (current: BuiltNode): void => {
    if (current.component.type === "text") {
      if (typeof current.component.value !== "string") throw new Error(`plugin-ui.section-chunk-not-text:${bodyKey}`);
      payload += packedTextLeaf(current.component.value, current.component.dataAttributes);
    }
    for (const child of current.children) walk(child);
  };
  walk(node);
  try {
    return JSON.parse(payload) as unknown;
  } catch (error) {
    throw new SemioFaultError({
      origin: "renderer",
      code: "plugin-ui.section-payload-not-json",
      severity: "error",
      message: `reserved refresh section ${bodyKey} published by ${producer} carried ${payload.length} bytes that are not valid JSON`,
      scope: { bodyKey, instanceId: producer },
      causes: [{ message: error instanceof Error ? error.message : String(error) }],
      retryable: false,
    });
  }
}

/** 🪟️ Binds each concrete host surface to its authored body and projected ViewModel. */
function uiRefreshSurfaceEvents(instanceId: number, request: PluginUiRefreshRequest) {
  const windows = windowHostContextBindings(instanceId, request.windows ?? [], request.viewState).flatMap((binding) => {
    const viewState = windowViewContext(request.viewState, binding.windowKey);
    if (!viewState) return [];
    return [{
      kind: "surface-visible",
      payload: { surface: binding.surface, bodyKey: binding.bodyKey, viewState: encodePackValue(viewState) },
    } satisfies ShardEventEnvelope];
  });
  const panels = (request.panels ?? []).flatMap((target) => target.bodyKey ? [{
      kind: "surface-visible",
      payload: {
        surface: pluginSurfaceRef(instanceId, target.key),
        bodyKey: target.bodyKey,
        viewState: encodePackValue(panelViewContext(request.viewState)),
      },
    } satisfies ShardEventEnvelope] : []);
  const sections = uiRefreshSectionTargets(request).map((section) => ({
    kind: "surface-visible",
    payload: {
      surface: pluginSurfaceRef(instanceId, section.bodyKey),
      bodyKey: section.bodyKey,
      viewState: encodePackValue(sectionViewContext(request.viewState)),
    },
  } satisfies ShardEventEnvelope));
  return [...windows, ...panels, ...sections];
}

/** 🏛️ Projects one turn's host effects without borrowing any retained UI owner. */
function retainedUiRefreshEffects(instanceId: number, effects: readonly WireVariant[]): Effect[] {
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
  return requestedEffects;
}

/** 📬️ Projects retained bodies back to their requested window and panel keys. */
function retainedUiRefreshResponse(instanceId: number, request: PluginUiRefreshRequest, retained: ReadonlyMap<string, RetainedSurface>, effects: readonly WireVariant[] = []): PluginUiRefreshResponse {
  const project = (targets: PluginUiRefreshRequest["windows"]): PluginUiRefreshSectionResponse[] => (targets ?? []).flatMap((target) => {
    const surface = retained.get(retainedSurfaceId(instanceId, target.key));
    if (!surface) return [];
    const hash = retainedSurfaceHash(retainedSurfaceToSnapshot(surface));
    if (uiRefreshSectionUnchanged(target.hash, { root: surface.root, hash })) return [{ key: target.key, hash }];
    const value = retainedSurfaceToBuiltNode(surface);
    return value ? [{ key: target.key, hash, value }] : [];
  });
  const sections: { [key in UiRefreshSectionKey]?: PluginUiRefreshSectionResponse } = {};
  for (const section of uiRefreshSectionTargets(request)) {
    const surface = retained.get(retainedSurfaceId(instanceId, section.bodyKey));
    if (!surface) continue;
    const hash = retainedSurfaceHash(retainedSurfaceToSnapshot(surface));
    if (uiRefreshSectionUnchanged(request[section.key]?.hash, { root: surface.root, hash })) {
      sections[section.key] = { key: section.key, hash };
      continue;
    }
    const built = retainedSurfaceToBuiltNode(surface);
    if (built) sections[section.key] = { key: section.key, hash, value: sectionValueFromBuiltNode(section.bodyKey, built, `instance ${instanceId}`) };
  }
  return { windows: project(request.windows), panels: project(request.panels), ...sections, requestedEffects: retainedUiRefreshEffects(instanceId, effects) };
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
  notePluginLoadProgress(pluginId);
  const { manifest, runtime: registry } = await resolveDescriptorBeforeRuntime(() => fetchDescriptorManifest(pluginId, moduleUrl, signal), getActivationRegistry);
  notePluginLoadProgress(pluginId);
  registry.registerManifest({ pluginId, moduleUrl, caps: [] });
  notePluginLoadProgress(pluginId);
  const shardClient = getShardClient();
  const actorIdByInstance = new Map<number, string>();
  const closingInstances = new Set<number>();
  const openingInstances = new Map<number, Promise<number>>();
  const retiringInstances = new Map<number, Promise<void>>();
  let disposing = false;
  let disposal: Promise<void> | null = null;
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
  let eventSeq = 0;
  /** 🎟️ Per-step intake slice. One item per step made the 4 096-step continuation budget a hard
   * 4 096-item ceiling on a single surface patch (a 180-object world scene exhausts it); 256 items /
   * 64 KiB per step keeps each slice sub-millisecond while the budget bounds a patch at ~1 M items. */
  const uiGrant = Object.freeze({ maxItems: 256, maxBytes: 65_536 });
  const yieldUi = async (step: number): Promise<void> => { if (step % PLUGIN_UI_INTAKE_YIELD_STRIDE === 0) await yieldPluginUiContinuation(); };
  const closeIntake = async (instanceId: number, intake: OwnedUiPatchIntake): Promise<void> => {
    intake.beginClose();
    for (let step = 1; !intake.terminalIsEmpty(); step += 1) {
      if (step > PLUGIN_UI_CONTINUATION_LIMIT) throw new Error("plugin-ui.intake-close-budget-exhausted");
      const current = intake.closeStep(uiGrant);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`plugin-ui.intake-close-${current.kind}:${current.phase}`);
      await yieldUi(step);
    }
    uiIntakesByInstance.get(instanceId)?.delete(intake);
  };
  const acceptUiPatches = async (instanceId: number, turn: WireTurnResult): Promise<PluginPatchAcceptance> => {
    if (turn.uiPatches.length === 0) return { acknowledgements: [], turns: [] };
    requireActorId(instanceId);
    const lease = lifecycleByInstance.get(instanceId);
    const owner = uiOwnerByInstance.get(instanceId);
    if (!lease || !owner || !turn.original || !turn.uiPatchReceipt) throw new Error("plugin-ui.native-owner-required");
    const supplemental: WireTurnResult[] = [];
    for (const [index, patch] of turn.uiPatches.entries()) {
      const source = lease.captureUiPatchAuthority(turn.original, index);
      const surfaceId = wirePatchSurfaceId(patch);
      if (!surfaceId) throw new Error("plugin-ui.projection-surface-required");
      const intake = new OwnedUiPatchIntake(owner, source);
      const intakes = uiIntakesByInstance.get(instanceId) ?? new Set<OwnedUiPatchIntake>();
      intakes.add(intake); uiIntakesByInstance.set(instanceId, intakes);
      let token: OwnedUiPatchAcknowledgement | null = null;
      for (let step = 1; token === null; step += 1) {
        if (step > PLUGIN_UI_INTAKE_STEP_CEILING) throw new Error(`plugin-ui.intake-budget-exhausted:${surfaceId}:${PLUGIN_UI_INTAKE_STEP_CEILING}`);
        const current = intake.advance(uiGrant);
        token = intake.peekAcknowledgement();
        if (current.kind === "rejected") throw new Error(`plugin-ui.intake-rejected:${current.phase}:${intake.failure ?? "unknown"}`);
        if (current.kind === "blocked" && token === null) throw new Error(`plugin-ui.intake-blocked:${current.phase}`);
        await yieldUi(step);
      }
      const acknowledged = await submitPluginLifecycleTurn(lease, { kind: "issued-ui-ack", source, token }, "Interactive");
      if (!intake.acceptAcknowledgement(acknowledged.submission)) throw new Error("plugin-ui.acknowledgement-refused");
      for (let step = 1; ; step += 1) {
        if (step > PLUGIN_UI_INTAKE_STEP_CEILING) throw new Error(`plugin-ui.publication-close-budget-exhausted:${surfaceId}:${PLUGIN_UI_INTAKE_STEP_CEILING}`);
        const current = intake.advance(uiGrant);
        if (current.kind === "ready") break;
        if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`plugin-ui.intake-${current.kind}:${current.phase}`);
        await yieldUi(step);
      }
      const surface = intake.takeSurface();
      if (!surface) throw new Error("plugin-ui.surface-missing");
      const surfaces = uiSurfaceByInstance.get(instanceId) ?? new Map<string, OwnedUiInstanceSurface>();
      surfaces.set(surfaceId, surface); uiSurfaceByInstance.set(instanceId, surfaces);
      await closeIntake(instanceId, intake);
      supplemental.push(acknowledged.turn);
      const nested = await acceptUiPatches(instanceId, acknowledged.turn);
      supplemental.push(...nested.turns);
      if (nested.acknowledgements.length > 0) throw new Error("plugin-ui.unexpected-generic-acknowledgement");
    }
    return { acknowledgements: [], turns: supplemental };
  };
  const advanceUiMaintenance = async (owner: OwnedUiInstance, budget: { steps: number }, phase: string): Promise<void> => {
    while (owner.maintenancePending) {
      if (++budget.steps > DEFAULT_UI_DOCUMENT_LIMITS.maxNodes * 64) throw new Error(`plugin-ui.${phase}-budget-exhausted`);
      const current = owner.advanceMaintenance(uiGrant);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`plugin-ui.${phase}-${current.kind}:${current.phase}`);
      await yieldUi(budget.steps);
    }
  };
  const projectOwnedUiSurface = async (instanceId: number, actorId: string, lease: ShardInstanceLifecycleLease, owner: OwnedUiInstance, surface: OwnedUiInstanceSurface): Promise<{ readonly hash: string; readonly value: BuiltNode } | null> => {
    const current = () => !disposing && !closingInstances.has(instanceId) && actorIdByInstance.get(instanceId) === actorId && lifecycleByInstance.get(instanceId) === lease && uiOwnerByInstance.get(instanceId) === owner;
    const view = surface.view;
    if (view.root === null) return null;
    if (!view.hash) throw new Error("plugin-ui.surface-hash-required");
    const visited = new Set<number>();
    const budget = { steps: 0 };
    const build = async (id: number, depth: number): Promise<BuiltNode> => {
      if (!current()) throw new Error("plugin-ui.read-stale");
      if (depth > DEFAULT_UI_DOCUMENT_LIMITS.maxDepth || visited.size >= DEFAULT_UI_DOCUMENT_LIMITS.maxNodes || visited.has(id)) throw new Error("plugin-ui.read-graph-invalid");
      visited.add(id);
      const subscription = surface.subscribeNode(id, () => {});
      let record: RetainedUiNodeRecord | null = null;
      try {
        await advanceUiMaintenance(owner, budget, "read");
        const snapshot = subscription.snapshot;
        if (!snapshot || snapshot.version !== view.revision || !snapshot.record) throw new Error("plugin-ui.read-snapshot-missing");
        record = snapshot.record;
        surface.acknowledgeRead(subscription, snapshot);
      } finally {
        surface.unsubscribeNode(subscription);
        await advanceUiMaintenance(owner, budget, "read-retirement");
      }
      if (!record) throw new Error("plugin-ui.read-snapshot-missing");
      const children: BuiltNode[] = [];
      for (const child of record.children) children.push(await build(child, depth + 1));
      return { key: record.key, component: ownedUiComponentToBuilt(record.component), layout: record.layout, style: record.style, activity: record.activity, disabled: record.disabled, accessibility: record.accessibility, bindings: record.bindings, menu: record.menu, children };
    };
    const value = await build(view.root, 1);
    if (!current() || surface.view !== view) throw new Error("plugin-ui.read-stale");
    return { hash: view.hash, value };
  };
  const ownedUiRefreshResponse = async (instanceId: number, actorId: string, request: PluginUiRefreshRequest, effects: readonly WireVariant[]): Promise<PluginUiRefreshResponse> => {
    const lease = lifecycleByInstance.get(instanceId);
    const owner = uiOwnerByInstance.get(instanceId);
    const surfaces = uiSurfaceByInstance.get(instanceId);
    if (!lease || !owner || !surfaces) throw new Error("plugin-ui.native-owner-required");
    const project = async (targets: PluginUiRefreshRequest["windows"]): Promise<PluginUiRefreshSectionResponse[]> => {
      const result: PluginUiRefreshSectionResponse[] = [];
      for (const target of targets ?? []) {
        const surface = surfaces.get(retainedSurfaceId(instanceId, target.key));
        if (surface && uiRefreshSectionUnchanged(target.hash, surface.view)) {
          result.push({ key: target.key, hash: target.hash! });
          continue;
        }
        // 🧯️ A requested body that has no retained surface at all, or one the guest has not rooted yet,
        // is omitted from the response — and the shell's own merge keeps whatever it had (its loading
        // placeholder, on a first refresh). That is indistinguishable from a healthy refresh, so it is
        // recorded: one console record per dropped body, permanent (not a `[DEBUG]` trace), the same
        // way `AppRouter` records a plugin it excluded. A panel that renders forever empty in the
        // browser (measured 2026-09-09 21:05 on `framework.panel.inspection`) is either named here or
        // is a real guest render — no third possibility.
        if (!surface) {
          console.error(`refreshUi dropped requested body ${JSON.stringify(target.key)}: no retained surface on instance ${instanceId}`);
          continue;
        }
        const projected = await projectOwnedUiSurface(instanceId, actorId, lease, owner, surface);
        if (projected) result.push({ key: target.key, ...projected });
        else console.error(`refreshUi dropped requested body ${JSON.stringify(target.key)}: retained surface has no root on instance ${instanceId}`);
      }
      return result;
    };
    const projectSections = async (): Promise<{ [key in UiRefreshSectionKey]?: PluginUiRefreshSectionResponse }> => {
      const sections: { [key in UiRefreshSectionKey]?: PluginUiRefreshSectionResponse } = {};
      for (const section of uiRefreshSectionTargets(request)) {
        const surface = surfaces.get(retainedSurfaceId(instanceId, section.bodyKey));
        if (!surface) continue;
        const cached = request[section.key]?.hash;
        if (uiRefreshSectionUnchanged(cached, surface.view)) {
          sections[section.key] = { key: section.key, hash: cached! };
          continue;
        }
        const projected = await projectOwnedUiSurface(instanceId, actorId, lease, owner, surface);
        if (projected) sections[section.key] = { key: section.key, hash: projected.hash, value: sectionValueFromBuiltNode(section.bodyKey, projected.value, `${actorId} instance ${instanceId}`) };
      }
      return sections;
    };
    const read = (async () => ({ windows: await project(request.windows), panels: await project(request.panels), ...(await projectSections()) }))();
    const reads = uiReadsByInstance.get(instanceId) ?? new Set<Promise<unknown>>();
    reads.add(read); uiReadsByInstance.set(instanceId, reads);
    try {
      const projected = await read;
      return { ...projected, requestedEffects: retainedUiRefreshEffects(instanceId, effects) };
    } finally {
      reads.delete(read);
    }
  };
  const closeUiOwner = async (instanceId: number, owner: OwnedUiInstance): Promise<OwnedUiInstanceRetirement> => {
    const retained = uiRetirementByInstance.get(instanceId);
    if (retained) return retained;
    if ((uiReadsByInstance.get(instanceId)?.size ?? 0) !== 0) throw new Error("plugin-ui.read-retirement-pending");
    for (const intake of uiIntakesByInstance.get(instanceId) ?? []) await closeIntake(instanceId, intake);
    uiSurfaceByInstance.get(instanceId)?.clear();
    owner.beginClose();
    for (let step = 1; !owner.terminalIsEmpty(); step += 1) {
      if (step > PLUGIN_UI_CONTINUATION_LIMIT) throw new Error("plugin-ui.owner-close-budget-exhausted");
      const current = owner.closeStep(uiGrant);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`plugin-ui.owner-close-${current.kind}:${current.phase}`);
      await yieldUi(step);
    }
    const witness = owner.takeRetirementWitness();
    if (!witness) throw new Error("plugin-ui.retirement-witness-missing");
    uiRetirementByInstance.set(instanceId, witness);
    return witness;
  };
  const assertClosingTurn = (turn: WireTurnResult): void => {
    if (turn.uiPatches.length > 0 || turn.uiPatchReceipt !== undefined) throw new Error("plugin-ui.patch-after-lifecycle-close");
  };
  const retireInstanceLifecycle = async (instanceId: number, lease: ShardInstanceLifecycleLease, owner: OwnedUiInstance): Promise<void> => {
    const captured = lease.pendingReceipt;
    if (captured?.kind === "captured") {
      const acknowledged = await submitPluginLifecycleTurn(lease, { kind: "receipt-ack", receipt: captured }, "Interactive");
      assertClosingTurn(acknowledged.turn);
    }
    lease.beginClose();
    for (let step = 1; lease.progress().kind !== "complete"; step += 1) {
      if (step > PLUGIN_UI_CONTINUATION_LIMIT) throw new Error("plugin-ui.lifecycle-close-budget-exhausted");
      const receipt = lease.pendingReceipt;
      if (receipt?.kind === "accepted") {
        const acknowledged = await submitPluginLifecycleTurn(lease, { kind: "receipt-ack", receipt }, "Interactive");
        assertClosingTurn(acknowledged.turn);
      } else if (receipt?.kind === "retired") {
        const retirement = await closeUiOwner(instanceId, owner);
        const acknowledged = await submitPluginLifecycleTurn(lease, { kind: "receipt-ack", receipt, retirement }, "Interactive");
        assertClosingTurn(acknowledged.turn);
      } else {
        const progress = lease.progress();
        if (progress.kind === "blocked") throw new Error(`plugin-ui.lifecycle-close-blocked:${progress.failure ?? "unknown"}`);
        const current = progress.kind === "closing"
          ? await submitPluginLifecycleTurn(lease, { kind: "close" }, "Interactive")
          : await submitPluginLifecycleTurn(lease, { kind: "poll" }, "Interactive");
        assertClosingTurn(current.turn);
      }
      await yieldUi(step);
    }
    lease.dispose();
  };
  const requireActorId = (instanceId: number): string => {
    const actorId = actorIdByInstance.get(instanceId);
    if (disposing) throw new Error("plugin-handle.closed");
    if (!actorId || closingInstances.has(instanceId)) throw new Error(`[DEBUG] program ${pluginId}: no actor for instance ${instanceId} (createApp not called, or already destroyed)`);
    return actorId;
  };
  const releaseInstanceMaps = (instanceId: number, actorId: string): void => {
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
    teardownPluginActor(actorId);
    closingInstances.delete(instanceId);
  };
  /** 🚦 `lane`/`coalesceKey` forward to {@link submitPluginTurn} — see that function's own doc for the
   * lane-assignment reasoning. `registry.touch(actorId)` refreshes this actor's LRU position on every
   * turn (its own doc: "call on every turn, not just activation"); turns dispatch through this file's
   * own {@link submitPluginTurn} rather than `ActivationRegistry.enqueueTurn` (see that decision's
   * write-up above `serializePerActor`), so nothing else would ever call it. */
  const submitTurn = (actorId: string, events: readonly ShardEventEnvelope[], options?: { readonly lane?: Lane; readonly coalesceKey?: string; readonly commandPage?: ShardCommandIngressPage; readonly activation?: ShardActorActivationLease }): Promise<WireTurnResult> => {
    registry.touch(actorId);
    return submitPluginTurn(actorId, events, options?.lane ?? "Interactive", options?.coalesceKey, options?.commandPage, options?.activation);
  };

  /** 📤️📥️ Backs {@link KernelPluginWasmHandle.enqueue}/`.outcomes` (see this file's own header doc).
   * One broadcast per `loadPluginModule` call, matching the handle's own
   * lifetime: every instance this handle ever `createApp`s shares it, and each instance's
   * `AppChannelClient` (`💻️os/🟦️.ts`) filters to its own `instanceId`. */
  const turnOutcomes = createTurnOutcomeBroadcast<TurnOutcome>();

  const hotBytes = (raw: unknown): Uint8Array => {
    const length = raw instanceof Uint8Array || Array.isArray(raw) ? raw.length : 0;
    if (!length || length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || (Array.isArray(raw) && raw.some(value => !Number.isInteger(value) || value < 0 || value > 255))) throw new Error("actor-document-port.message-size");
    const bytes = raw instanceof Uint8Array ? raw : Uint8Array.from(raw as number[]);
    if (decodeBackboneMessage(bytes).kind === "snapshot") throw new Error("actor-document-port.snapshot-requires-cold-pair");
    return bytes;
  };
  /** 💼️ In-flight `spawn-job` drives, keyed `actorId#job`. A guest re-emitting the same `job` id
   * (a replayed turn, a checkpoint restore) must never open a second `start-job` for one owner. */
  const drivingJobs = new Set<string>();
  const finishedReservedJobs = new Set<string>();
  const pendingReservedJobDrives: Promise<void>[] = [];
  const flushReservedJobDrives = async (): Promise<void> => {
    const batch = pendingReservedJobDrives.splice(0);
    if (batch.length === 0) return;
    await Promise.all(batch);
  };

  /** ⛽️ The budget one `step-job` is granted. `fuel` is a WIT `u64`, hence a `bigint` — the same
   * reshaping `ShardLoop::pump` does natively (`🖥️host/🧵️shard/🦀️.rs:146-147`,
   * `job_budget_from_grant`), so a job slice on this target costs what it costs on the native one. */
  const jobStepBudget = { fuel: BigInt(DEFAULT_SHARD_BUDGET.fuel), deadlineMs: DEFAULT_SHARD_BUDGET.wallMs } as const;

  /** 📏️ Steps driven back to back before yielding the main thread, so a long plan never blocks a
   * frame. `yieldPluginUiContinuation` is deliberately not a `setTimeout` chain (its own doc: a
   * hidden tab throttles those to one tick a second). */
  const PLUGIN_JOB_STEPS_PER_YIELD = 32;

  /** 🏁️ Feeds one job's terminal back into the guest as `events::job-completed` — the event
   * `⚛️reactor/🔄️turn/🦀️.rs:327-344` resolves the guest's own parked `spawn_job` request with
   * (`RequestId(job)` IS the job id). Without this the guest waits forever on a job the host already
   * finished. `job-progress` is deliberately NOT delivered per slice: the guest ignores its payload
   * (`⚛️reactor/🔄️turn/🦀️.rs:317` only marks the surface dirty) and one whole actor turn per slice
   * would cost more than the repaint is worth. */
  const deliverJobCompletionTurn = async (instanceId: number, actorId: string, job: bigint, outcome: ShardJobStep): Promise<void> => {
    if (outcome.status === "running") throw new Error("plugin.job-completion-not-terminal");
    const val = Array.from(outcome.value);
    const payload = { job, outcome: outcome.status === "done" ? { tag: "ok", val } : { tag: "fault", val } };
    const activation = shardClient.captureActorActivation(actorId);
    const settled = await withTypedOperationCall(actorId, `job-completion#${instanceId}`, async (call) => {
      activation.assertActive();
      return settlePluginTurn(actorId, await submitTurn(actorId, [{ kind: "job-completed", payload }], { lane: "Background", activation }), "Background", new Set(), (turn) => acceptUiPatches(instanceId, turn), true, activation, call);
    });
    const leftover = pendingTurnEffects.get(instanceId) ?? [];
    for (const effect of routeHostEffects(instanceId, settled.effects, documentBindings.get(instanceId)?.port)) leftover.push(effect);
    pendingTurnEffects.set(instanceId, leftover);
    console.warn(`[DEBUG] job-completed leftover job=${job} effects=${leftover.map((effect) => effect.tag).join(",") || "none"}`);
  };
  const deliverJobCompletion = async (instanceId: number, actorId: string, job: bigint, outcome: ShardJobStep): Promise<void> => {
    await serializeCommandIngressForActor(actorId, () => deliverJobCompletionTurn(instanceId, actorId, job, outcome));
    await flushReservedJobDrives();
  };

  /** 💼️ The host half of `Effect::SpawnJob` on this target: `start-job` once, then one `step-job`
   * per turn of this loop until the guest answers `done`/`failed`, then one `job-completed` event
   * back into the actor. This is the browser counterpart of `ShardLoop::pump`'s `running_jobs` walk
   * (`🔌️plugin/🖥️host/🧵️shard/🦀️.rs:359`, :1714-1765). Before ticket 26/09/02 W-J nothing on this
   * target started or stepped an isolated job at all — the effect was dropped unmapped in
   * `wireEffectToFriendly`, so every plugin-authored job silently never ran.
   *
   * ⛽️ A job is bounded PER SLICE and never over its lifetime, exactly as `ShardLoop::pump` bounds it:
   * every `step-job` carries {@link jobStepBudget} (`job_budget_from_grant`'s web twin) and the loop
   * ends only when the guest's own `BoundedJob::step` answers `done`/`failed`, or when the instance
   * this drive belongs to goes away. Ticket 26/09/02/PUZZLE-3D-END-TO-END W-J shipped a fixed
   * 65 536-step LIFETIME cap here instead: a real `semio.puzzle3d.fill` plan (up to
   * `PUZZLE3D_FILL_COUNT_MAX = 1000` placements over a whole document census) reaches that in ~40 s of
   * ticking, and the cancel it then issued tore the guest's live envelope down mid-run — browser-
   * measured 2026-09-10 as `unreachable` → `shard 0 lost` → `actor-activation.revoked` on every later
   * call. A host may not decide a plan is over; only its owner may. */
  const driveSpawnedJob = async (instanceId: number, actorId: string, job: bigint, kind: string, input: Uint8Array): Promise<void> => {
    const key = `${actorId}#${job}`;
    if (drivingJobs.has(key)) return;
    drivingJobs.add(key);
    const live = (): boolean => !disposing && !closingInstances.has(instanceId) && actorIdByInstance.get(instanceId) === actorId;
    try {
      beginIsolatedJobDrive();
      await serializeCommandIngressForActor(actorId, () => shardClient.startJob(actorId, job, kind, input));
      let step = 0;
      while (live()) {
        const batch = isolatedJobStepsPerSerializedAdmission(PLUGIN_JOB_STEPS_PER_YIELD);
        const progressed = { value: false };
        const outcome = await serializeCommandIngressForActor(actorId, async () => {
          let last: ShardJobStep | undefined;
          for (let i = 0; i < batch; i += 1) {
            last = await shardClient.stepJob(actorId, job, jobStepBudget);
            step += 1;
            if (last.status === "running" && last.progress && last.progress.byteLength > 0) progressed.value = true;
            if (last.status !== "running") return last;
          }
          return last ?? { status: "failed" as const, value: new TextEncoder().encode("plugin.job-step-empty") };
        });
        if (outcome.status !== "running") {
          console.warn(`[DEBUG] job done kind=${kind} job=${job} status=${outcome.status} steps=${step}`);
          if (live()) await deliverJobCompletion(instanceId, actorId, job, outcome);
          return;
        }
        if (progressed.value || isolatedJobUiPollEverySteps(step)) requestIsolatedJobUiPoll();
        await yieldPluginUiContinuation();
      }
    } catch (error) {
      console.warn(`[DEBUG] job drive failed kind=${kind} job=${job}`, error);
      turnOutcomes.push({ instanceId, error });
    } finally {
      endIsolatedJobDrive();
      drivingJobs.delete(key);
    }
  };

  /** 💼️ Reserved-tool Isolated jobs are two dummy steps plus `job-completed` commit. The guest
   * keeps one job-render binding per instance; a later hover/undo spawn steals it and
   * `complete_reserved_spawned_job` never runs. Hold command-ingress through start/step/commit
   * so the binding stays on this job until chrome undo publishes. */
  const commitReservedToolJobWhileSerialized = async (instanceId: number, actorId: string, job: bigint, kind: string, input: Uint8Array): Promise<void> => {
    const key = `${actorId}#${job}`;
    if (drivingJobs.has(key)) return;
    drivingJobs.add(key);
    try {
      beginIsolatedJobDrive();
      await shardClient.startJob(actorId, job, kind, input);
      let step = 0;
      let last: ShardJobStep | undefined;
      const batch = isolatedJobStepsPerSerializedAdmission(PLUGIN_JOB_STEPS_PER_YIELD);
      for (let i = 0; i < batch; i += 1) {
        last = await shardClient.stepJob(actorId, job, jobStepBudget);
        step += 1;
        if (last.status !== "running") break;
      }
      const outcome = last ?? { status: "failed" as const, value: new TextEncoder().encode("plugin.job-step-empty") };
      if (outcome.status === "running") throw new Error(`[DEBUG] reserved-tool job ${job} still running after ${step} Isolated steps`);
      console.warn(`[DEBUG] job done kind=${kind} job=${job} status=${outcome.status} steps=${step}`);
      await deliverJobCompletionTurn(instanceId, actorId, job, outcome);
    } finally {
      endIsolatedJobDrive();
      finishedReservedJobs.add(key);
      drivingJobs.delete(key);
    }
  };
  const commitReservedToolSpawnsWhileSerialized = async (instanceId: number, actorId: string, effects: readonly WireVariant[]): Promise<void> => {
    for (const effect of effects) {
      if (effect.tag !== "spawn-job") continue;
      const value = effect.val as { job?: unknown; kind?: unknown; input?: unknown } | undefined;
      if (typeof value?.job !== "bigint" || value.kind !== "framework.reserved.tool") continue;
      console.warn(`[DEBUG] spawn-job routed kind=${value.kind} job=${value.job} inline=1`);
      await commitReservedToolJobWhileSerialized(instanceId, actorId, value.job, value.kind, coerceWireBytes(value.input));
    }
  };

    const driveReservedToolJob = async (instanceId: number, actorId: string, job: bigint, kind: string, input: Uint8Array): Promise<void> => {
    const key = `${actorId}#${job}`;
    if (drivingJobs.has(key)) return;
    drivingJobs.add(key);
    const live = (): boolean => !disposing && !closingInstances.has(instanceId) && actorIdByInstance.get(instanceId) === actorId;
    try {
      beginIsolatedJobDrive();
      await serializeCommandIngressForActor(actorId, async () => {
        await shardClient.startJob(actorId, job, kind, input);
        let step = 0;
        let last: ShardJobStep | undefined;
        const batch = isolatedJobStepsPerSerializedAdmission(PLUGIN_JOB_STEPS_PER_YIELD);
        for (let i = 0; i < batch; i += 1) {
          last = await shardClient.stepJob(actorId, job, jobStepBudget);
          step += 1;
          if (last.status !== "running") break;
        }
        const outcome = last ?? { status: "failed" as const, value: new TextEncoder().encode("plugin.job-step-empty") };
        if (outcome.status === "running") throw new Error(`[DEBUG] reserved-tool job ${job} still running after ${step} Isolated steps`);
        console.warn(`[DEBUG] job done kind=${kind} job=${job} status=${outcome.status} steps=${step}`);
        if (live()) await deliverJobCompletionTurn(instanceId, actorId, job, outcome);
      });
    } catch (error) {
      console.warn(`[DEBUG] job drive failed kind=${kind} job=${job}`, error);
      turnOutcomes.push({ instanceId, error });
    } finally {
      endIsolatedJobDrive();
      drivingJobs.delete(key);
    }
  };

    /** 🚦 The one choke point every consumer of a turn's `effects` passes through: backbone
   * `send-message`s go to the document port, `spawn-job`/`cancel-job` go to {@link driveSpawnedJob}
   * (they are host WORK, never a `requestedEffects` entry for `applyHostEffects` to branch on), and
   * everything else is returned for the caller's own shell-frame/leftover split. */
  const routeHostEffects = (instanceId: number, effects: readonly WireVariant[], port: ActorDocumentMessagePortV1 | undefined): WireVariant[] => effects.filter(effect => {
    if (effect.tag === "spawn-job") {
      const value = effect.val as { job?: unknown; kind?: unknown; input?: unknown } | undefined;
      if (typeof value?.job !== "bigint" || typeof value.kind !== "string") throw new Error("plugin.spawn-job-authority-invalid");
      const actorId = requireActorId(instanceId);
      if (drivingJobs.has(`${actorId}#${value.job}`) || finishedReservedJobs.has(`${actorId}#${value.job}`)) return false;
      console.warn(`[DEBUG] spawn-job routed kind=${value.kind} job=${value.job}`);
      const input = coerceWireBytes(value.input);
      const drive = value.kind === "framework.reserved.tool"
        ? driveReservedToolJob(instanceId, actorId, value.job, value.kind, input)
        : driveSpawnedJob(instanceId, actorId, value.job, value.kind, input);
      if (value.kind === "framework.reserved.tool") pendingReservedJobDrives.push(drive);
      else void drive;
      return false;
    }
    if (effect.tag === "cancel-job") {
      const value = effect.val as { job?: unknown } | undefined;
      if (typeof value?.job !== "bigint") throw new Error("plugin.cancel-job-authority-invalid");
      const job = value.job;
      const actorId = requireActorId(instanceId);
      void serializeCommandIngressForActor(actorId, () => shardClient.cancelJob(actorId, job)).catch(error => turnOutcomes.push({ instanceId, error }));
      return false;
    }
    const value = effect.val as { target?: WireVariant; payload?: unknown } | undefined;
    if (effect.tag !== "send-message" || value?.target?.tag !== "backbone") return true;
    if (!port) throw new Error("actor-document-port.unbound");
    if (value.target.val !== port.uri) throw new Error("actor-document-port.foreign-uri");
    if (!port.send(port.uri, hotBytes(value.payload))) throw new Error("actor-document-port.retired");
    return false;
  });

  /** 🔀️ The real turn-submission body `enqueue` used to run synchronously inline and return — now
   * run fire-and-forget from `enqueue`, pushing its settlement onto {@link turnOutcomes} instead of
   * resolving a caller's promise directly (a caller's own promise now lives one layer up, in
   * `AppChannelClient.sendCommand`, correlated against this broadcast). A turn-submission failure
   * (NOT an `AppFrame::Error` — that is still an ordinary decoded frame) becomes an `error`-shaped
   * outcome rather than an uncaught rejection, since nothing here awaits this function's own promise. */
  const runQueuedTurn = async (instanceId: number, events: readonly Uint8Array[]): Promise<void> => {
    try {
      const actorId = requireActorId(instanceId);
      const activation = shardClient.captureActorActivation(actorId);
      const documentPort = documentBindings.get(instanceId)?.port;
      const inspected = inspectEncodedAppCommand(events);
      console.warn("[DEBUG] command ingress lane", JSON.stringify({ instanceId, actionId: inspected.actionId, seq: inspected.seq, lane: inspected.lane }));
      const result = await withTypedOperationCall(actorId, `command#${instanceId}`, (call) => serializeCommandIngressForActor(actorId, async (): Promise<WireTurnResult> => {
        activation.assertActive();
        const results: WireTurnResult[] = [];
        let acknowledgements: readonly ShardEventEnvelope[] = [];
        const acceptTurn = async (turn: WireTurnResult): Promise<void> => {
          results.push(turn);
          const accepted = await acceptUiPatches(instanceId, turn);
          results.push(...accepted.turns);
          acknowledgements = [...accepted.acknowledgements, ...typedOperationAcknowledgements(turn), ...accepted.turns.flatMap(typedOperationAcknowledgements)];
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
          for (const commandPage of pages) {
            const pageTurn = await submitTurn(actorId, acknowledgements, { commandPage, activation });
            await acceptTurn(pageTurn);
          }
          let terminal = results.at(-1)?.commandIngress?.tag;
          const observedStatuses = new Set([terminal ?? "missing"]);
          for (let continuation = 0; terminal !== "command-complete" && continuation < 1_024; continuation += 1) {
            if (terminal === "fault") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress fault: ${commandIngressFaultDisplay(results.at(-1)?.commandIngress)}`);
            if (terminal === "backpressure") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress backpressure after serialized submission`);
            const continued = await submitTurn(actorId, acknowledgements, { activation });
            await acceptTurn(continued);
            terminal = continued.commandIngress?.tag;
            observedStatuses.add(terminal ?? "missing");
            if (continuation % 32 === 31) console.warn(`[DEBUG] command ingress continuation ${continuation + 1} status=${terminal ?? "missing"} turn=${wireTurnStatusTag(continued.status)}`);
          }
          console.warn(`[DEBUG] command ingress settled status=${terminal ?? "missing"} observed=${[...observedStatuses].join(",")}`);
          if (terminal !== "command-complete") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress did not complete within 1024 continuations (observed statuses: ${[...observedStatuses].join(", ")})`);
        }
        const settled = await settleAcknowledgedPluginTurns(actorId, results, acknowledgements, (turn) => acceptUiPatches(instanceId, turn), activation, call);
        await commitReservedToolSpawnsWhileSerialized(instanceId, actorId, settled.effects);
        return settled;
      }, inspected.lane));
      requireActorId(instanceId);
      activation.assertActive();
      const outFrames: Uint8Array[] = [];
      const leftover: WireVariant[] = [...(pendingTurnEffects.get(instanceId) ?? [])];
      for (const effect of routeHostEffects(instanceId, result.effects, documentPort)) {
        const frame = shellFrameBytes(effect, instanceId);
        if (frame) outFrames.push(frame);
        else leftover.push(effect);
      }
      const promoted = promoteShellSendMessages(instanceId, leftover, outFrames);
      pendingTurnEffects.set(instanceId, promoted);
      await flushReservedJobDrives();
      if (commandIngressNeedsReplyStampV1(outFrames.map(encodedFrameReplySequence), inspected.seq)) {
        outFrames.push(encodeAppFrame({ Done: { in_reply_to: inspected.seq! } }));
        console.warn("[DEBUG] command ingress stamped Done", JSON.stringify({ instanceId, actionId: inspected.actionId, seq: inspected.seq, leftover: leftover.length, frames: outFrames.length }));
      }
      turnOutcomes.push({ instanceId, frames: outFrames });
      if (wireTurnStatusTag(result.status) === "more-work") void drainTypedOperations(instanceId);
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    }
  };

  /** 🔁️ Keeps an instance's retained typed operations advancing once NO host call is left to drive
   * them. A mounted operation returns its "started" `InvocationResult` on its very first turn and then
   * needs hundreds more reactor turns to reach its terminal result — but every turn in this file is
   * submitted by a host call, so the moment `runQueuedTurn` returns the actor simply stops, its
   * operation frozen mid-flight and its completion never published. This is the poll that replaces
   * that missing driver: one `UserVisible` continuation settle per macrotask (never a `setTimeout`
   * chain — `yieldPluginUiContinuation`'s own doc: a hidden tab throttles those to one tick a second),
   * stopping the instant the actor reports anything other than `more-work`, and re-armed by the next
   * command turn that ends `more-work` again. It shares `serializeCommandIngressForActor` with every
   * other caller, so a real command always wins the actor and this poll waits behind it. */
  const drainingInstances = new Set<number>();
  const drainTypedOperations = async (instanceId: number): Promise<void> => {
    const live = (): boolean => !disposing && !closingInstances.has(instanceId) && actorIdByInstance.has(instanceId);
    if (drainingInstances.has(instanceId) || !live()) return;
    drainingInstances.add(instanceId);
    try {
      const outcome = await drainTypedOperationTurns(PLUGIN_OPERATION_DRAIN_BUDGET, live, async () => {
        const actorId = requireActorId(instanceId);
        const activation = shardClient.captureActorActivation(actorId);
        const documentPort = documentBindings.get(instanceId)?.port;
        const settled = await withTypedOperationCall(actorId, `operation-drain#${instanceId}`, (call) =>
          serializeCommandIngressForActor(actorId, async () => {
            activation.assertActive();
            return settlePluginTurn(actorId, await submitTurn(actorId, [], { lane: "UserVisible", activation }), "UserVisible", new Set(), (turn) => acceptUiPatches(instanceId, turn), true, activation, call);
          }),
        );
        const frames: Uint8Array[] = [];
        const leftover = pendingCompletionEffects.get(instanceId) ?? [];
        for (const effect of routeHostEffects(instanceId, settled.effects, documentPort)) {
          const frame = shellFrameBytes(effect, instanceId);
          if (frame) frames.push(frame);
          else leftover.push(effect);
        }
        if (leftover.length > PLUGIN_OPERATION_EFFECT_CAPACITY) throw new Error(`typed-operation host effects for instance ${instanceId} exceeded their ${PLUGIN_OPERATION_EFFECT_CAPACITY}-entry authority`);
        pendingCompletionEffects.set(instanceId, leftover);
        if (frames.length > 0) turnOutcomes.push({ instanceId, frames });
        return { status: settled.status, nextWake: settled.nextWake };
      });
      if (outcome.stopped === "budget") console.warn(`[DEBUG] typed-operation drain for instance ${instanceId} exhausted its ${PLUGIN_OPERATION_DRAIN_BUDGET}-poll budget`);
      if (outcome.stopped === "idle" && outcome.nextWake !== null && live()) {
        setTimeout(() => void drainTypedOperations(instanceId), Math.min(Math.max(outcome.nextWake, 0), PLUGIN_OPERATION_WAKE_MAX_MS));
      }
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    } finally {
      drainingInstances.delete(instanceId);
    }
  };

  const handle: KernelPluginWasmHandle = {
    manifest: async () => encodePackValue(manifest),
    createApp: (appId) => {
      if (disposing) return Promise.reject(new Error("plugin-handle.closed"));
      const instanceId = nextGlobalInstanceId;
      nextGlobalInstanceId += 1;
      const actorId = `${pluginId}#${instanceId}`;
      actorIdByInstance.set(instanceId, actorId);
      const requireOpening = () => {
        if (disposing || closingInstances.has(instanceId) || actorIdByInstance.get(instanceId) !== actorId) throw new Error("plugin-handle.closed");
      };
      const opening = Promise.resolve().then(async () => {
        requireOpening();
        await registry.activate(pluginId, actorId, "manual" satisfies ActivationReason);
        requireOpening();
        eventSeq += 1;
        const lease = shardClient.captureInstanceLifecycle(actorId, instanceId);
        lifecycleByInstance.set(instanceId, lease);
        registry.touch(actorId);
        const opened = await submitPluginLifecycleTurn(
          lease,
          { kind: "open", input: { appId, actor: currentPluginRuntimeActor, config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})) } },
          "Interactive",
        );
        const captured = lease.pendingReceipt;
        const lifetime = lease.lifetime;
        if (!captured || captured.kind !== "captured" || !lifetime) throw new Error("plugin-ui.native-lifetime-required");
        const owner = new OwnedUiInstance(lease.activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 32 });
        lease.bindHostRetirement(owner);
        uiOwnerByInstance.set(instanceId, owner);
        uiSurfaceByInstance.set(instanceId, new Map());
        uiIntakesByInstance.set(instanceId, new Set());
        requireOpening();
        await withTypedOperationCall(actorId, `open#${instanceId}`, async (call) => {
          await settlePluginTurn(actorId, opened.turn, "Interactive", new Set(), (turn) => acceptUiPatches(instanceId, turn), false, undefined, call);
          requireOpening();
          const acknowledged = await submitPluginLifecycleTurn(lease, { kind: "receipt-ack", receipt: captured }, "Interactive");
          await settlePluginTurn(actorId, acknowledged.turn, "Interactive", new Set(), (turn) => acceptUiPatches(instanceId, turn), false, undefined, call);
        });
        requireOpening();
        return instanceId;
      });
      openingInstances.set(instanceId, opening);
      const forget = () => { if (openingInstances.get(instanceId) === opening) openingInstances.delete(instanceId); };
      return opening.then(value => { forget(); return value; }, error => {
        forget();
        return settleFailedInstanceOpen(error, () => handle.destroyApp(instanceId));
      });
    },
    /** 🧹 Retires one instance. An instance whose `open` never bound a UI owner holds NO retained UI
     * — there is nothing to retire, only an actor to cancel — so a missing owner joins the missing
     * lifecycle branch instead of throwing. Throwing here is what turned a mid-boot
     * `shard 0 terminated` into `Framework OS boot failed: plugin-ui.native-owner-required`: the
     * failed-open cleanup above awaited this call, and its rejection replaced the real cause. */
    destroyApp: (instanceId) => {
      const previous = retiringInstances.get(instanceId);
      if (previous) return previous;
      const actorId = actorIdByInstance.get(instanceId);
      if (!actorId) return Promise.resolve();
      closingInstances.add(instanceId);
      const opening = openingInstances.get(instanceId);
      const retirement = documentBindings.get(instanceId)?.port.retire();
      const retiring = Promise.resolve().then(async () => {
        await Promise.all([opening?.then(() => {}, () => {}), retirement]);
        if (actorIdByInstance.get(instanceId) === actorId) {
          const lease = lifecycleByInstance.get(instanceId);
          const owner = uiOwnerByInstance.get(instanceId);
          if (!lease || !owner) {
            registry.cancel(actorId);
            releaseInstanceMaps(instanceId, actorId);
            return;
          }
          await Promise.allSettled([...(uiReadsByInstance.get(instanceId) ?? [])]);
          await retireInstanceLifecycle(instanceId, lease, owner);
          registry.cancel(actorId);
          releaseInstanceMaps(instanceId, actorId);
        }
      });
      retiringInstances.set(instanceId, retiring);
      const forget = () => { if (retiringInstances.get(instanceId) === retiring) retiringInstances.delete(instanceId); };
      void retiring.then(forget, forget);
      return retiring;
    },
    takeSegmentedDownloadChunk: (instanceId, operationId) => shardClient.takeSegmentedDownloadChunk(requireActorId(instanceId), instanceId, operationId),
    enqueue: (instanceId, events) => {
      requireActorId(instanceId);
      void runQueuedTurn(instanceId, events);
    },
    outcomes: turnOutcomes.stream,
    dispose: () => {
      if (disposal) return disposal;
      disposing = true;
      const retirements = [...actorIdByInstance.keys()].map(instanceId => handle.destroyApp(instanceId));
      disposal = Promise.allSettled(retirements).then(results => {
        const failures = results.filter((result): result is PromiseRejectedResult => result.status === "rejected");
        if (failures.length) throw new AggregateError(failures.map(result => result.reason), "plugin-handle.retirement-failed");
        turnOutcomes.complete();
      });
      return disposal;
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
    const events = uiRefreshSurfaceEvents(instanceId, request);
    if (events.length === 0) return {};
    const actorId = requireActorId(instanceId);
    const activation = shardClient.captureActorActivation(actorId);
    const documentPort = documentBindings.get(instanceId)?.port;
    eventSeq += 1;
    const surfaces = uiSurfaceByInstance.get(instanceId);
    const missingSurfaceIds = new Set(events.map((event) => retainedSurfaceId(instanceId, (event.payload as { readonly surface: { readonly surface: string } }).surface.surface)).filter((surfaceId) => {
      const surface = surfaces?.get(surfaceId);
      return !surface || surface.view.root === null;
    }));
    const result = await withTypedOperationCall(actorId, `refresh-ui#${instanceId}`, (call) => serializeCommandIngressForActor(actorId, async () => {
      const settled = await settlePluginTurn(
        actorId,
        await submitTurn(
          actorId,
          events,
          { lane: "UserVisible", activation },
        ),
        "UserVisible",
        missingSurfaceIds,
        (turn) => acceptUiPatches(instanceId, turn),
        true,
        activation,
        call,
      );
      return settled;
    }));
    requireActorId(instanceId);
    activation.assertActive();
    return ownedUiRefreshResponse(instanceId, actorId, request, routeHostEffects(instanceId, result.effects, documentPort));
  };

  /** 🔁️ Retains one completion's exact activation across evaluation, queueing and publication. */
  const captureExtensionCompletion = (instanceId: number, req: bigint): PluginExtensionCompletion => {
    if (typeof req !== "bigint" || req <= 0n || req > 0xffffffffffffffffn) throw new Error("extension.request-id-invalid");
    const actorId = requireActorId(instanceId);
    const activation = shardClient.captureActorActivation(actorId);
    const documentPort = documentBindings.get(instanceId)?.port;
    let submitted = false;
    const assertActive = (): void => { requireActorId(instanceId); activation.assertActive(); };
    const complete = async (outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array }): Promise<InvocationResponse> => {
      assertActive();
      if (submitted) throw new Error("extension.completion-already-submitted");
      submitted = true;
      const answer = "ok" in outcome ? outcome.ok : outcome.fault;
      if (answer.byteLength > GUEST_HOST_ANSWER_CEILING_BYTES) {
        throw new SemioFaultError({
          origin: "os", code: "extension.answer-too-large", severity: "error",
          message: `extension answer of ${answer.byteLength} B exceeds the ${GUEST_HOST_ANSWER_CEILING_BYTES}-byte host-answer ceiling`,
          scope: { instanceId: String(instanceId), req: String(req) }, retryable: false,
        });
      }
      // 📄️ The guest is handed this answer through ONE `cabi_realloc` per event, and a block its
      // allocator refuses aborts the actor before any guest code runs — no fault, no diagnosis, an
      // endless restore loop (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, boot #12). So the answer
      // crosses as prologue pages inside the guest's declared contiguous-request ceiling, with the
      // terminal page riding the completion that answers `req`, all in ONE turn.
      const { prologue, terminal } = guestAnswerPages(answer);
      const events: ShardEventEnvelope[] = [
        ...prologue.map((page) => ({ kind: "http-chunk" as const, payload: { req, params: { bytes: Array.from(page), done: false } } })),
        { kind: "completed" as const, payload: { req, outcome: "ok" in outcome ? { tag: "ok", val: Array.from(terminal) } : { tag: "fault", val: Array.from(terminal) } } },
      ];
      console.debug("[DEBUG] extension completion submitted", { instanceId, req, status: "ok" in outcome ? "ok" : "fault", bytes: answer.byteLength, pages: events.length });
      return withTypedOperationCall(actorId, `extension-completion#${instanceId}`, (call) => serializeCommandIngressForActor(actorId, async () => {
        assertActive();
        const settled = await settlePluginTurn(
          actorId,
          await submitTurn(actorId, events, { activation }),
          "Interactive",
          new Set(),
          (turn) => { assertActive(); return acceptUiPatches(instanceId, turn); },
          true,
          activation,
          call,
        );
        assertActive();
        const frames: Uint8Array[] = [];
        const effects: WireVariant[] = [];
        for (const effect of routeHostEffects(instanceId, settled.effects, documentPort)) {
          const frame = shellFrameBytes(effect, instanceId);
          if (frame) frames.push(frame);
          else effects.push(effect);
        }
        turnOutcomes.push({ instanceId, frames });
        return invocationFromFrames(frames.map(decodeAppFrame), effects, "extension completion");
      }));
    };
    return Object.freeze({ instanceId, req, assertActive, complete });
  };

  const bindDocumentPort = async (instanceId: number, source: PluginDocumentBindingV1): Promise<ActorDocumentMessagePortV1> => {
    const identity = Object.freeze({ runtimeKey: source.runtimeKey, clientInstanceId: source.clientInstanceId, scope: source.scope === null ? null : Object.freeze({ ...source.scope }) });
    const { current: sourceCurrent, send, merge: receiveMerge, prepared } = source;
    const actorId = requireActorId(instanceId);
    const activation = shardClient.captureActorActivation(actorId);
    const previous = documentBindings.get(instanceId);
    if (previous && !previous.port.closing) throw new Error("actor-document-control.binding-live");
    if (previous) await previous.port.retire();
    if (documentBindings.get(instanceId) !== previous) throw new Error("actor-document-control.binding-collision");
    requireActorId(instanceId);
    activation.assertActive();
    const bindingGeneration = (documentBindingGenerations.get(instanceId) ?? 0n) + 1n;
    const current = () => !closingInstances.has(instanceId) && actorIdByInstance.get(instanceId) === actorId && sourceCurrent();
    const settle = (events: readonly ShardEventEnvelope[]) => withTypedOperationCall(actorId, `document-port#${instanceId}`, (call) => serializeCommandIngressForActor(actorId, async () => {
      activation.assertActive();
      return settlePluginTurn(actorId, await submitTurn(actorId, events, { activation }), "Interactive", new Set(), turn => acceptUiPatches(instanceId, turn), true, activation, call);
    }));
    const binding = new ActorDocumentBindingV1({ ...identity, actorId, activationGeneration: activation.activationGeneration, instanceId }, bindingGeneration, {
      current,
      assertActive: () => activation.assertActive(),
      limits: { messageBytes: BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, pendingBytes: DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes, pendingMessages: DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages },
      send,
      exchange: async command => {
        const result = await settle([{ kind: "message", payload: { source: { tag: "shell", val: String(instanceId) }, payload: Array.from(encodeDocumentBackboneControlV1(command)) } }]);
        return result.effects.flatMap(effect => {
          const bytes = shellFrameBytes(effect, instanceId);
          if (bytes) return [bytes];
          if (effect.tag === "send-message" && (effect.val as { target?: WireVariant } | undefined)?.target?.tag === "backbone") throw new Error("actor-document-control.data-before-receipt");
          return [];
        });
      },
      deliver: async payload => {
        const result = await settle([{ kind: "message", payload: { source: { tag: "backbone", val: binding.port.uri }, payload: Array.from(hotBytes(payload)) } }]);
        if (!current()) return;
        const frames = routeHostEffects(instanceId, result.effects, binding.port).flatMap(effect => {
          const bytes = shellFrameBytes(effect, instanceId);
          return bytes ? [bytes] : [];
        });
        const values = frames.map(decodeAppFrame);
        const failed = values.find(value => "Error" in value);
        if (failed && "Error" in failed) throw new Error(faultDisplayMessage(failed.Error.fault, decodePackValue));
        const merge = values.find(value => "MergeReport" in value);
        const conflicts = values.find(value => "Conflicts" in value);
        if (merge || conflicts) receiveMerge?.(conflicts && "Conflicts" in conflicts ? decodeConflictsFromWire(conflicts.Conflicts.conflicts, decodePackValue) : null, merge && "MergeReport" in merge ? decodeMergeReportFromWire(merge.MergeReport.report, decodePackValue) : null);
        turnOutcomes.push({ instanceId, frames });
      },
    });
    documentBindings.set(instanceId, binding);
    documentBindingGenerations.set(instanceId, bindingGeneration);
    try {
      prepared?.(binding.port);
      await binding.bind();
      return binding.port;
    } catch (error) {
      await binding.port.retire();
      if (documentBindings.get(instanceId) === binding) documentBindings.delete(instanceId);
      throw error;
    }
  };

  return { ...richHandle, refreshUi, captureExtensionCompletion, bindDocumentPort };
}

/** 🩹️ A patch acknowledgement carries the guest's own publication receipt: the guest rejects an ack
 * whose `(lifetime, patch-sequence)` authority it never issued, so the ack is only meaningful for the
 * very turn whose `uiPatchReceipt` produced these patches. */
function patchAckEvents(turn: WireTurnResult, uiPatches: readonly WireUiPatch[]): ShardEventEnvelope[] {
  if (uiPatches.length === 0) return [];
  const receipt = turn.uiPatchReceipt === undefined ? undefined : decodeActorUiPatchReceipt(turn.uiPatchReceipt);
  if (!receipt) return [];
  return uiPatches.flatMap((patch) => (patch.surface ? [{ kind: "patch-ack", payload: { receipt, surface: patch.surface, revision: patch.revision ?? 0 } }] : []));
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

//#region 🔖️ChannelAdapter
/** 🎯️ DslValue may ship `Vec<u8>` as a number array, a Uint8Array, or a `{ kind:"bytes", value }`
 * object — used both for the old DSL-pack byte fields AND (H1-react) for `pack`-typed fields inside a
 * raw WIT `effect`/`patch-op` variant, which jco represents as a plain byte array or `Uint8Array`. */

function leftoverShellInvocationFrames(leftover: readonly WireVariant[]): AppFrameValue[] {
  const frames: AppFrameValue[] = [];
  for (const effect of leftover) {
    const value = effect.val as { target?: WireVariant; payload?: unknown } | undefined;
    if (effect.tag !== "send-message" || value?.target?.tag !== "shell" || value.payload === undefined) continue;
    try {
      const frame = decodeAppFrame(coerceWireBytes(value.payload));
      if ("Invocation" in frame) frames.push(frame);
    } catch {
      /* leftover host effect, not an AppFrame */
    }
  }
  return frames;
}

function leftoverClipboardWriteEffects(leftover: readonly WireVariant[]): WireVariant[] {
  const writes = leftover.filter((effect) => effect.tag === "clipboard-write");
  if (writes.length === 0 && leftover.some((effect) => effect.tag === "send-message")) {
    console.warn(`[DEBUG] leftover clipboard-write missing tags=${leftover.map((effect) => effect.tag).join(",")}`);
  }
  return writes;
}

function promoteShellSendMessages(instanceId: number, leftover: readonly WireVariant[], _outFrames: Uint8Array[]): WireVariant[] {
  for (const frame of leftoverShellInvocationFrames(leftover)) {
    console.warn(`[DEBUG] job-completed leftover frame instance=${instanceId} kind=Invocation history=${frame.Invocation.history_patch.length}`);
  }
  leftoverClipboardWriteEffects(leftover);
  return [...leftover];
}

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
 * `events` has no wire counterpart in this wave (an honest gap `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s
 * native `invocation_from_frames` already flags identically). */
async function performInvocation(client: AppChannelClient, instanceId: number, invocation: unknown, invocationKind: "action" | "command", viewState: unknown): Promise<InvocationResponse> {
  assertAddressedInvocation(invocation, invocationKind, instanceId);
  const invocationRecord = invocation as { readonly address?: { readonly actionId?: unknown; readonly commandId?: unknown }; readonly arguments?: Record<string, unknown> } | null;
  const address = invocationRecord?.address;
  const actionId = address?.actionId ?? address?.commandId ?? null;
  console.warn("[DEBUG] performInvocation", JSON.stringify({ invocationKind, instanceId, actionId }));
  if (actionId === "importFixture") {
    const args = invocationRecord?.arguments;
    const payload = args?.payload;
    console.warn("[DEBUG] importFixture ingress", JSON.stringify({
      name: typeof args?.name === "string" ? args.name : null,
      payloadType: payload === undefined ? "missing" : typeof payload,
      payloadLen: typeof payload === "string" ? payload.length : payload && typeof payload === "object" ? Object.keys(payload as object).length : null,
      argKeys: args ? Object.keys(args) : [],
    }));
  }
  const frames = await client.command(encodePackValue(invocation), viewState);
  const leftover = pendingTurnEffects.get(instanceId) ?? [];
  pendingTurnEffects.delete(instanceId);
  const response = invocationFromFrames(frames, leftover, invocationKind);
  console.warn("[DEBUG] performInvocation settled", JSON.stringify({ invocationKind, instanceId, actionId, frames: frames.length, frameKinds: frames.map((frame) => Object.keys(frame)[0] ?? "?"), historyCursor: response.historyPatch?.cursor ?? null, historyUpserts: response.historyPatch?.upserts?.length ?? 0, historyCanUndo: response.historyPatch?.canUndo ?? null, effects: (response.requestedEffects ?? []).length }));
  return response;
}

/** 🚫️ Refuses an unaddressed invocation at the renderer edge, naming the window kind and instance it
 * was aimed at. Without this the empty id travels to the plugin, which can only answer with the
 * anonymous `window kind <kind> does not own action ""` — a `plugin.internal` fault that names neither
 * the renderer frame that produced it nor the instance it belongs to
 * (`📓️runtime-verification-2026-09-09.md` boot #3, two unhandled rejections per boot). */
export function assertAddressedInvocation(invocation: unknown, invocationKind: "action" | "command", instanceId: number): void {
  const address = (invocation as { readonly address?: Record<string, unknown> } | null)?.address;
  const id = invocationKind === "action" ? address?.actionId : address?.commandId;
  if (typeof id === "string" && id.length > 0) return;
  const windowKindId = typeof address?.windowKindId === "string" ? address.windowKindId : "<unknown>";
  throw new SemioFaultError({
    origin: "renderer",
    code: "renderer.invocation.unaddressed",
    severity: "error",
    message: `${invocationKind} invocation on window kind ${windowKindId} carries no ${invocationKind}Id`,
    scope: {
      pluginId: typeof address?.pluginId === "string" ? address.pluginId : undefined,
      appId: typeof address?.appId === "string" ? address.appId : undefined,
      instanceId: String(instanceId),
    },
    retryable: false,
  });
}

/** 📬️ Decodes the shared invocation publication without losing its host effects or typed fault. */
function invocationFromFrames(frames: readonly AppFrameValue[], leftover: readonly WireVariant[], invocationKind: string): InvocationResponse {
  let output: unknown = null;
  let diagnostics: InvocationResponse["diagnostics"] = [];
  let mutations: InvocationResponse["mutations"] = [];
  let inverseGroup: InvocationResponse["inverseGroup"] = { invocationId: "", mutations: [], inverseMutations: [] };
  let uiScope: InvocationResponse["uiScope"];
  let historyPatch: InvocationResponse["historyPatch"];
  const applyInvocationFrame = (frame: Extract<AppFrameValue, { readonly Invocation: unknown }>): void => {
    if (frame.Invocation.output.length) output = decodePackValue(new Uint8Array(frame.Invocation.output));
    if (frame.Invocation.diagnostics.length) {
      const decodedDiagnostics = decodePackValue(new Uint8Array(frame.Invocation.diagnostics));
      diagnostics = Array.isArray(decodedDiagnostics) ? (decodedDiagnostics as InvocationResponse["diagnostics"]) : [];
    }
    if (frame.Invocation.ui_scope.length) uiScope = decodePackValue(new Uint8Array(frame.Invocation.ui_scope)) as InvocationResponse["uiScope"];
    if (frame.Invocation.history_patch.length) {
      const decodedHistoryPatch = decodePackWire(new Uint8Array(frame.Invocation.history_patch), "$.historyPatch");
      historyPatch = decodedHistoryPatch && typeof decodedHistoryPatch === "object" ? (decodedHistoryPatch as InvocationResponse["historyPatch"]) : undefined;
    }
    ({ mutations, inverseGroup } = decodeInvocationResultPacks(frame.Invocation));
  };
  for (const frame of frames) {
    if ("Invocation" in frame) {
      applyInvocationFrame(frame);
    } else if ("Error" in frame) {
      const fault = decodeFaultFromWire(frame.Error.fault, decodePackValue);
      if (fault) throw new SemioFaultError(fault);
      throw new Error(`${invocationKind} failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
    }
  }
  for (const frame of leftoverShellInvocationFrames(leftover)) {
    if ("Invocation" in frame) applyInvocationFrame(frame);
  }
  if (leftover.some((effect) => effect.tag === TYPED_OPERATION_PENDING_OUTPUT)) throw new Error("directory projection receipt was exposed before typed-operation terminal publication");
  const terminalOutputs = leftover.filter((effect) => effect.tag === TYPED_OPERATION_TERMINAL_OUTPUT);
  if (terminalOutputs.length > 1) throw new Error("typed-operation returned more than one terminal output");
  if (terminalOutputs.length === 1) output = terminalOutputs[0]!.val;
  const requestedEffects = leftover.filter((effect) => effect.tag !== TYPED_OPERATION_TERMINAL_OUTPUT && effect.tag !== TYPED_OPERATION_TERMINAL_SEEN).map(wireEffectToFriendly).filter((effect): effect is Effect => effect !== null);
  return {
    output,
    mutations,
    inverseGroup,
    diagnostics,
    requestedEffects,
    events: [],
    uiScope,
    historyPatch,
  };
}

async function performContextMenu(client: Pick<AppChannelClient, "contextMenu">, request: PluginContextMenuRequest, viewState: ViewModel): Promise<readonly ContextMenuItemSpec[]> {
  const items = await client.contextMenu({ ...request, viewState });
  return Array.isArray(items) ? (items as ContextMenuItemSpec[]) : [];
}

//#region 🔖️ActorIdentity
/** 🪪️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§C3 — the shell's
 * current actor id (`user:{userId}#{shellSessionId}` once identity resolves, `client-<random>` before
 * that), stamped onto every `AppChannelClient` created from here on. Module-level rather than an
 * `adaptPluginHandle` parameter because `PluginWasmHandle` is the kernel's frozen shape (no setter
 * method to add) and every real call site (`loadPluginModuleResilient`, `🐚️🌿️ShellHelpers/🟦️.tsx`)
 * lives outside this ticket's lease — this is the smallest surface that reaches every future
 * `createApp` call without touching a foreign-leased signature. Known limitation: a single JS realm
 * hosting more than one `🏛️ShellHost` (e.g. the multi-pane demonstrator) shares one actor id across
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
export async function adaptPluginHandle(pluginId: string, lease: { readonly handle: KernelPluginWasmHandle; readonly release: () => Promise<void> }): Promise<PluginWasmHandle> {
  const handle = lease.handle;
  const manifest = decodePackValue(await handle.manifest()) as unknown as PluginManifest;
  const channels = new Map<number, AppChannelClient>();
  const channelRequests = new AppChannelRequestSequence();
  let disposal: Promise<void> | null = null;
  let disposing = false;
  const requireChannel = (instanceId: number): AppChannelClient => {
    if (disposing) throw new Error("plugin-handle.closed");
    const client = channels.get(instanceId);
    if (!client) throw new Error(`[DEBUG] program ${pluginId}: no channel for instance ${instanceId} (createApp not called, or already destroyed)`);
    return client;
  };
  return {
    pluginId,
    manifest,
    createApp: async (appId) => {
      if (disposing) throw new Error("plugin-handle.closed");
      const instanceId = await handle.createApp(appId);
      if (disposing) { await handle.destroyApp(instanceId); throw new Error("plugin-handle.closed"); }
      channels.set(instanceId, new AppChannelClient(handle, channelRequests, instanceId, appId, currentPluginRuntimeActor));
      return instanceId;
    },
    destroyApp: async (instanceId) => {
      const channel = channels.get(instanceId);
      await handle.destroyApp(instanceId);
      channel?.dispose();
      if (channels.get(instanceId) === channel) channels.delete(instanceId);
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
    contextMenu: (instanceId, request, viewState) => performContextMenu(requireChannel(instanceId), request, viewState),
    readHistory: async (instanceId) => {
      const frames = await requireChannel(instanceId).readHistory();
      const frame = frames.find((candidate): candidate is Extract<AppFrameValue, { readonly HistorySnapshot: unknown }> => "HistorySnapshot" in candidate);
      if (!frame) throw new Error("[DEBUG] readHistory: missing HistorySnapshot frame");
      return decodePackWire(new Uint8Array(frame.HistorySnapshot.history_patch), "$.historyPatch") as HistoryPatch;
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
    readAppDocumentPack: async (instanceId) => {
      const frames = await requireChannel(instanceId).readDocument();
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`[DEBUG] readAppDocumentPack failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
      const documentFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Document: unknown }> => "Document" in frame);
      return documentFrame
        ? { pack: new Uint8Array(documentFrame.Document.pack), spr: new Uint8Array(documentFrame.Document.spr), ops: documentFrame.Document.ops }
        : null;
    },
    loadAppDocumentPack: async (instanceId, pack, spr) => {
      const frames = await requireChannel(instanceId).loadDocument(pack, spr);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`[DEBUG] loadAppDocumentPack failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    },
    readWindowConfigPacks: (instanceId) => requireChannel(instanceId).readWindowConfigs(),
    loadWindowConfigPack: (instanceId, entry) => requireChannel(instanceId).loadWindowConfig(entry),
    // 🚧️ Same channel-v12 retirement as `attachBackbone`/`detachBackbone` above: the old
    // `AppFrame::Ephemeral` poll was the literal empty-batch drain design-abi.md §4 names as
    // retired outright — `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s native twin (`ephemeral_snapshot`) stubs the
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
    readLocalInteraction: async (instanceId, signal) => {
      const pages: Uint8Array[] = [];
      let length = 0;
      const identity = await requireChannel(instanceId).readLocalInteractionPages(async (page) => {
        length += page.bytes.length;
        if (length > LOCAL_INTERACTION_CAPTURE_MAX_BYTES) throw new Error("local-interaction.capture-length");
        pages.push(Uint8Array.from(page.bytes));
      }, signal);
      const bytes = new Uint8Array(length);
      let offset = 0;
      for (const page of pages) {
        bytes.set(page, offset);
        offset += page.length;
      }
      const capture = decodeLocalInteractionCaptureJson(bytes);
      if (!localInteractionIdentityEquals(capture.identity, identity)) throw new Error("local-interaction.capture-authority");
      return capture;
    },
    subscribeOperationCompletions: (instanceId, listener) =>
      requireChannel(instanceId).onOperationCompleted((completion) => {
        const leftover = pendingCompletionEffects.get(instanceId) ?? [];
        pendingCompletionEffects.delete(instanceId);
        listener({
          instanceId,
          operation: completion.operation,
          revision: completion.revision,
          uiScope: completion.uiScope as InvocationResponse["uiScope"],
          historyPatch: completion.historyPatch as HistoryPatch | undefined,
          requestedEffects: leftover
            .filter((effect) => effect.tag !== TYPED_OPERATION_TERMINAL_OUTPUT && effect.tag !== TYPED_OPERATION_PENDING_OUTPUT && effect.tag !== TYPED_OPERATION_TERMINAL_SEEN)
            .map(wireEffectToFriendly)
            .filter((effect): effect is Effect => effect !== null),
        });
      }),
    dispose: () => {
      if (disposal) return disposal;
      disposing = true;
      disposal = lease.release().then(() => {
        for (const channel of channels.values()) channel.dispose();
        channels.clear();
      });
      return disposal;
    },
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
 * haven't started yet. `🐙️handles` stays in the same topological sequence `order` always had, regardless
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

/** 🧪️ Preserves the exact private dependency types and live state cells across test extraction. */
function pluginRuntimeTestDependenciesV1() {
  const testState = {
    get sharedActivationRegistry() { return sharedActivationRegistry; },
    set sharedActivationRegistry(value: typeof sharedActivationRegistry) { sharedActivationRegistry = value; },
    get sharedShardClient() { return sharedShardClient; },
    set sharedShardClient(value: typeof sharedShardClient) { sharedShardClient = value; },
  };
  return { testState, leftoverShellInvocationFrames, leftoverInspectionRefreshScope, leftoverInspectionPanelHash, windowHostContextBindings, DEFAULT_LEFTOVER_WINDOW_SURFACE, isolatedJobStepsPerSerializedAdmission, isolatedJobUiPollEverySteps, ActivationRegistry, ActorDocumentBindingV1, adaptPluginHandle, assertAddressedInvocation, AppChannelClient, AppChannelRequestSequence, applyRetainedWindowPatches, applyUiPatch, applyUiPatchToRetained, ArtifactMutationRouter, assertShardJspiAvailable, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, buildShardClientOptions, coerceTurnResult, coerceWireBytes, commandIngressFaultDisplay, computeDependencyLevels, consumeTypedOperationEffects, createShardCommandIngressPages, createTurnOutcomeBroadcast, currentPluginRuntimeActor, decodeActorUiPatchReceipt, decodeAppFrame, decodeBackboneMessage, decodeConflictsFromWire, decodeFaultFromWire, decodeForeignStep, decodeInvocationResultPacks, decodeLocalInteractionCaptureJson, decodeMergeReportFromWire, decodeMutationEnvelopesPack, decodePackValue, decodePackWire, decodeWirePack, decodeWirePatchOps, DEFAULT_SHARD_BUDGET, drainTypedOperationTurns, DIRECTORY_PROJECTION_RECEIPT_SCHEMA, emptyUiDocumentState, encodeActorUiPatchReceipt, encodeDocumentBackboneControlV1, encodeMutationOrigin, encodePackValue, enqueuePluginTurn, faultDisplayMessage, fetchDescriptorManifest, fnv1aHex, getActivationRegistry, getPluginTurnScheduler, getShardClient, getThunkScheduler, handlePluginShardLost, hasRequiredUiPatches, InstanceDirectory, invocationFromFrames, isShardLostError, loadPluginModule, loadPluginModulesInDependencyOrder, LOCAL_INTERACTION_CAPTURE_MAX_BYTES, localInteractionIdentityEquals, MAX_TRANSACTION_DEPTH, nextGlobalInstanceId, normalizeWireUiNodeRecord, notePluginLoadProgress, orderPluginRegistryEntries, OwnedResidentLedger, packWireNatural, patchAckEvents, pendingCoalescedTurns, pendingCompletionEffects, pendingLifecycleTurns, pendingTurnEffects, performContextMenu, performInvocation, PLUGIN_BOOT_SHARD_LOST_FAULT, PLUGIN_OPERATION_DRAIN_BUDGET, PLUGIN_OPERATION_EFFECT_CAPACITY, PLUGIN_OPERATION_WAKE_MAX_MS, PLUGIN_TURN_MAILBOX_CAPACITY, PLUGIN_UI_CONTINUATION_BATCH_SIZE, PLUGIN_UI_CONTINUATION_LIMIT, PLUGIN_UI_QUIESCENT_CONTINUATIONS, PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT, PLUGIN_UI_INTAKE_STEP_CEILING, PLUGIN_UI_INTAKE_YIELD_STRIDE, retainedUiIntakeStepCeiling, PluginBootShardLostError, pluginLoadProgress, pluginLoadProgressAt, pluginSurfaceRef, poolConcurrency, rejectionCodeFromBytes, releasePendingLifecycleTurn, rendererResidentLedger, resolveDescriptorBeforeRuntime, retainedSurfaceHash, retainedSurfaceId, retainedSurfacesForActor, retainedSurfaceToBuiltNode, retainedSurfaceToSnapshot, retainedUiRefreshResponse, uiRefreshSectionUnchanged, retainedWindowByActor, retainTurnUiPatches, runBounded, sectionValueFromBuiltNode, runPluginLifecycleTurn, SEGMENTED_DOWNLOAD_MARKER_PREFIX, SemioFaultError, SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY, serializeCommandIngressForActor, serializePerActor, commandIngressLaneForActionV1, commandIngressNeedsReplyStampV1, setPluginRuntimeActor, settleAcknowledgedPluginTurns, settlePluginTurn, SHARD_LIVENESS_POLICY, SHARD_WORKER_URL, ShardClient, sharedPluginTurnScheduler, sharedThunkScheduler, shellFrameBytes, submitPluginLifecycleTurn, submitPluginTurn, teardownPluginActor, tearingDownPluginActors, TransactionCoordinator, TurnScheduler, TYPED_OPERATION_ACK_MAGIC, TYPED_OPERATION_PAGE_MAGIC, TYPED_OPERATION_PARK_CAPACITY, TYPED_OPERATION_PARK_EVICTION_FAULT, TYPED_OPERATION_PENDING_OUTPUT, TYPED_OPERATION_TERMINAL_OUTPUT, TYPED_OPERATION_TERMINAL_SEEN, TYPED_OPERATION_UNATTRIBUTED_FAULT, typedOperationAcknowledgements, TypedOperationCall, TypedOperationRouter, typedOperationResult, uiRefreshBodyKeys, uiRefreshSectionTargets, uiRefreshSurfaceEvents, wireEffectToFriendly, wireExtensionInvocation, wireNatural, wirePatchSurfaceId, wireTurnStatusTag, withTypedOperationCall, yieldPluginUiContinuation };
}

export type PluginRuntimeTestDependenciesV1 = ReturnType<typeof pluginRuntimeTestDependenciesV1>;

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../🧪️tests/🔌️plugin-runtime/🟦️.tsx");
  await registerTests1(import.meta.vitest, pluginRuntimeTestDependenciesV1(), { url: import.meta.url });
}
//#endregion 🧪️Tests
