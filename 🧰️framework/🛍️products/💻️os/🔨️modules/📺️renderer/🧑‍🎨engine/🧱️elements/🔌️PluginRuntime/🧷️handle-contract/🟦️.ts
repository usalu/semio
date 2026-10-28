import type { PluginOperationDiagnostic } from "../📬️typed-result/🟦️.ts";
import type {OwnedUiSurfaceLease} from '../🧺️handle-ui-intake/lease.ts';
import type {PluginUiRefreshSectionResponse} from '@semio-tech/framework';
export type {OwnedUiSurfaceLease,OwnedUiSurfaceReadSource} from '../🧺️handle-ui-intake/lease.ts';
/** 🧷️ The public type contract of the plugin handle: `PluginWasmHandle`, the extension-completion and document-binding shapes, the operation-completion publication and the transaction proposal/outcome types.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  type AppDefinition,
  type CommandDefinition,
  type Conflict,
  type ConflictId,
  type ConflictResolution,
  type ContextMenuItemSpec,
  type Effect,
  type HistoryPatch,
  type InvocationResponse,
  type MergePolicy,
  type ManifestExample,
  type MergeReport,
  type PluginContextMenuRequest,
  type PluginDependency,
  type PluginUiRefreshRequest,
  type PluginUiRefreshResponse,
  type PluginViewState,
  type TopicContribution
} from "@semio-tech/framework";
import type { MediaExportHandle, MediaExportStatus } from "@semio-tech/framework/wire-contracts";
// #endregion 🔌️Adapters
import {
  type AppDocumentIdentity,
  type DocumentArchiveLoadStatus,
  type DocumentArchivePack,
  type WindowConfigPackEntry
} from "@semio-tech/framework/wire-contracts";
import {
  type DocumentArchiveMergeV1
} from "@semio-tech/framework/app-channel";
import { type ArtifactPresencePeer, type LocalInteractionCapture } from "@semio-tech/framework/replication";
import { type PluginDispatchHintV1 } from "@semio-tech/framework";
import type { IconName } from "@semio-tech/ui-react";
import { type ActorDocumentMessagePortV1, type ActorDocumentSourceV1 } from "@semio-tech/framework/backbone/binding";

/** 🎭️ Renderer-side view state passed to program wasm calls — structurally mirrors `@semio-tech/framework`'s {@link PluginViewState}, kept as a distinct local alias since `ViewModel` is the established name used throughout this file. */
export type ViewModel = PluginViewState;

/** ⚠️ Not folded into `@semio-tech/framework`'s `PluginManifest`: this shell-local shape types `apps` richly (`AppDefinition[]`) where core intentionally keeps the wasm-boundary shape loose (`Record<string, unknown>[]`) for other consumers (e.g. compose, coda). Left for a human to decide whether to widen core's `PluginManifest` itself. */
export type PluginManifest = {
  readonly pluginId: string;
  readonly label: string;
  readonly version: string;
  readonly apps: readonly AppDefinition[];
  readonly examples: readonly ManifestExample[];
  readonly contributions?: readonly {
    readonly kind: "playbookBlockKind";
    readonly appId: string;
    readonly blockKind: string;
    readonly label: string;
    readonly iconId: IconName;
    readonly defaultValueJson?: string;
    readonly paramsBodyKey: string;
    readonly previewBodyKey: string;
  }[];
  /** 🎛️ Plugin-scope commands this plugin exposes — apply whenever any of its apps is focused. */
  readonly commands?: readonly CommandDefinition[];
  /** 🗂️ This plugin's OWNED artifact kinds — mirrors Rust `PluginManifest.artifact_kinds`. Feeds
   * {@link AppRouter.build}'s "owner plugin's surface first" ordering and its
   * `surface.missing-owner-surface`/`surface.contribution-not-permitted` checks (contract freeze §3).
   * Optional: a wasm build that predates the C3 router still decodes, just with no owner ordering. */
  readonly artifactKinds?: readonly { readonly id: string }[];
  /** 🔗️ This plugin's direct dependencies — mirrors Rust `PluginManifest.dependencies`, gates a
   * contributed surface through {@link AppRouter.build}'s `surface.contribution-not-permitted` check. */
  readonly dependencies?: readonly PluginDependency[];
  /** 🗂️ Open plugin contributions — the SAME rows core's `PluginManifest.topicContributions` carries.
   * The host→guest contributions push (`scopeContributionsJson`) reads exactly this field off a
   * loaded program, so leaving it off this shape made that push typecheck against nothing. */
  readonly topicContributions?: readonly TopicContribution[];
};

//#region 🔖️plugin-runtime
/** 🎟️ Captures requester activation and request identity before extension work or queue admission. */
export interface PluginExtensionCompletion {
  readonly instanceId: number;
  readonly req: bigint;
  assertActive(): void;
  complete(outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array }): Promise<InvocationResponse>;
}

/** 🎛️ Optional steering for one {@link PluginWasmHandle.invoke} call. `originInstanceId` names the
 * requesting instance the answer is being fetched FOR — it becomes the request event's
 * `message-endpoint::shell` origin, so a guest that ever routes by caller has the truth rather than a
 * placeholder; `0` (the default) means the shell itself asked, owning no instance of the callee.
 * `signal`/`onProgress` are the expensive-operation contract: an inbound request is drained over a
 * bounded number of guest turns, and both are read at each turn boundary. */
export interface PluginExtensionInvokeContext {
  readonly originInstanceId?: number;
  readonly signal?: AbortSignal;
  readonly onProgress?: (progress: { readonly capability: string; readonly turns: number; readonly budget: number }) => void;
}

export type PluginDocumentBindingV1 = ActorDocumentSourceV1 & Readonly<{
  current(): boolean;
  send(payload: Uint8Array): void;
  prepared?(port: ActorDocumentMessagePortV1): void;
  merge?(conflicts: readonly Conflict[] | null, report: MergeReport | null): void;
}>;

//#region 🔖️Merge
export type PluginWasmHandle = {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  /** 🧵 Drains one operation-owned export chunk; `undefined` is the sealed end marker. */
  readonly takeSegmentedDownloadChunk: (instanceId: number, operationId: bigint) => Promise<Uint8Array | undefined>;
  /** 🎯️ Dispatches one action. `dispatch` ({@link PluginDispatchHintV1}, kernel) is the input ledger's
   * causal `order` for this one call — forwarded down to the actor's command-ingress queue so a guest
   * follow-up of input N runs before the already-queued input N+1 (law L2); absent ⇒ arrival order. */
  readonly handleAction: (instanceId: number, actionJson: string, viewState: ViewModel, dispatch?: PluginDispatchHintV1) => Promise<InvocationResponse>;
  /** 🎛️ Dispatches a scoped command (os/plugin/app/mode) — optional since not every program declares commands.
   * `dispatch` as on {@link handleAction}. */
  readonly handleCommand?: (instanceId: number, commandJson: string, viewState: ViewModel, dispatch?: PluginDispatchHintV1) => Promise<InvocationResponse>;
  readonly refreshUi: (instanceId: number, request: PluginUiRefreshRequest) => Promise<PluginUiRefreshResponse>;
  /** 🎟️ Only an exact internally certified ordinary response can lend its acknowledged original source. */
  readonly acquireOwnedUiSurface:(instanceId:number,target:PluginUiRefreshSectionResponse)=>OwnedUiSurfaceLease|undefined;
  readonly contextMenu: (instanceId: number, request: PluginContextMenuRequest, viewState: ViewModel) => Promise<readonly ContextMenuItemSpec[]>;
  /** 🧾️ Complete projection used to seed or resynchronize host-owned history state. */
  readonly readHistory: (instanceId: number) => Promise<HistoryPatch>;
  /** 🔗️ The `DocumentApp` document-sync surface (WS-D) — optional since not every program has migrated onto it yet (WS-F).
   * `protocol_channel::AppCommand` carries binary `pack`/`spr` document-container bytes only
   * (`LoadDocumentArchive`/`ReadDocument`, backed by `store::print_document_pack`/`parse_document_pack`'s
   * deflate+BLAKE3 `.spk` container) — there is no JSON-text document command on the channel. The OLD
   * `readAppDocument`/`loadAppDocument` pair (plain JSON text — `MutationEnvelope[]` / a VCS envelope
   * string) has been retired along with every call site that used to feature-detect it;
   * {@link readAppDocumentPack} reads the same `.spk` container `documentPack` caches, and every whole-document load is the
   * stepped, cancellable {@link loadAppDocumentArchive} (a plain document is an archive with no members). */
  /** ⚖️ `AppCommand::ApplyEnvelopes`'s reply batches `MergeReport`/`Conflicts` frames alongside the
   * ingest itself (contract freeze §C6/§C9 "pushed unsolicited after every ingest") — decoded here,
   * same shape as {@link resolveConflict}'s reply, so a REMOTE peer's quarantined/degraded merge
   * reaches the caller instead of being dropped after the `Error` check. */
  readonly applyMutations?: (
    instanceId: number,
    mutationsPack: string,
  ) => Promise<{ readonly mergeReport: MergeReport | null; readonly conflicts: readonly Conflict[] | null }>;
  /** 📖️ Binary pack+spr document read (`AppCommand::ReadDocument`); `null` when the reply carries no `AppFrame::Document`
   * frame. */
  readonly readAppDocumentPack?: (instanceId: number) => Promise<{ readonly pack: Uint8Array; readonly spr: Uint8Array; readonly ops?: string } | null>;
  /** 📤️ Media OUT port read (`AppCommand::MediaOut`) — the agent-facing export counterpart to
   * {@link readAppDocumentPack}: it returns the guest's own exported bytes over the bridge instead
   * of driving the UI's download path, which hands the HUMAN a file and is the wrong outcome for an
   * agent (`📓️lb1-live-bridge-action-routing.md` §11.1, `📓️wr3-headless-routes-view-state-export.md`
   * §4.4). `null` when the reply carries no `AppFrame::Media` frame. */
  readonly exportAppMedia?: (instanceId: number, port: string) => Promise<{ readonly port: string; readonly descriptor: Uint8Array; readonly data: Uint8Array } | null>;
  /** 📂️ Delivers one declared natural media artifact and waits for its mutation commit. */
  readonly importAppMedia?: (instanceId: number, port: string, descriptor: unknown, data: Uint8Array) => Promise<void>;
  /** 🎬️ Starts one document-revision-owned media export without collecting its output. */
  readonly submitMediaExport: (instanceId: number, port: string, parentDocumentId: string, revision: bigint) => Promise<MediaExportHandle>;
  /** ⏱️ Advances one bounded media export turn and returns exact progress. */
  readonly pollMediaExport: (instanceId: number, handle: MediaExportHandle) => Promise<MediaExportStatus>;
  /** 🛑️ Cancels only the media operation identified by the complete handle. */
  readonly cancelMediaExport: (instanceId: number, handle: MediaExportHandle) => Promise<void>;
  /** 📥️ Takes one bounded media page and its explicit terminal marker. */
  readonly takeMediaExportChunk: (instanceId: number, handle: MediaExportHandle) => Promise<{ readonly handle: MediaExportHandle; readonly data: Uint8Array; readonly terminal: boolean }>;
  /** 🗃️ Complete root plus recursive owned-member closure for durable document persistence. */
  readonly readAppDocumentArchive?: (instanceId: number) => Promise<DocumentArchivePack>;
  /** 🪪️ The instance's own document identity — the id its store stamps on every envelope it publishes — without exporting
   * its bytes; `parent_document_id` is `null` for an app without a document. */
  readonly readAppDocumentIdentity: (instanceId: number) => Promise<AppDocumentIdentity>;
  /** 🗃️ Atomically restores a complete recursive document archive; `signal` cancels the load, `progress` hears every
   * polled status (`completed` of `total`). */
  readonly loadAppDocumentArchive?: (instanceId: number, archive: DocumentArchivePack, signal?: AbortSignal, progress?: (status: DocumentArchiveLoadStatus) => void) => Promise<void>;
  /** 🔀️ Merges a read-back of the document the instance already shows: its events join the instance's log and nothing is
   * replaced; answers what was taken and what the instance is ahead by, or the code of an archive it cannot merge. */
  readonly mergeAppDocumentArchive?: (instanceId: number, archive: DocumentArchivePack, signal?: AbortSignal, progress?: (status: DocumentArchiveLoadStatus) => void) => Promise<DocumentArchiveMergeV1>;
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
  ) => Promise<{ readonly presence: readonly number[]; readonly presenceGeneration: number; readonly transientGeneration: number; readonly interaction: readonly number[]; readonly toolRun: readonly number[]; readonly historyEdit: readonly number[] } | null>;
  /** 👥️ Pushes the document-wide presence roster into this instance's plugin app — the ONLY plugin
   * ingress for peers (contract-freeze §C7.6). `ownColor` is this actor's own hub-assigned palette
   * index (`null` for a folder-only session with no hub); `peers` is the whole roster with the
   * caller's own actor already dropped. */
  readonly pushPresence?: (instanceId: number, ownColor: number | null, peers: readonly ArtifactPresencePeer[]) => Promise<void>;
  /** 🔁️ Binds one canonical Completed submission to the originating activation before evaluation. */
  readonly captureExtensionCompletion?: (instanceId: number, req: bigint) => PluginExtensionCompletion;
  /** 📥️ Calls INTO this program: submits one `Event::Request{req, params:{origin, capability,
   * payload}}` on the program's own request actor and returns the `respond` effect's OK bytes, or
   * throws the guest's decoded fault. The ONE inbound-call door the ABI has — `extension.invoke`,
   * `artifact-compose` and the guest `io-run`/`io-sniff` exports were all replaced by it
   * (`🔌️plugin/🧬️schema/📜️.wit`'s `request-event`). Optional only because a test double may omit
   * it; every handle `loadPluginModule` returns defines it. */
  readonly invoke?: (capability: string, request: Uint8Array | string, context?: PluginExtensionInvokeContext) => Promise<Uint8Array>;
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
  readonly subscribeOperationDiagnostics: (instanceId: number, listener: (receipt: PluginOperationDiagnostic) => void) => () => void;
  readonly subscribeOperationCompletions: (instanceId: number, listener: (completion: PluginOperationCompletion) => void) => () => void;
  /** 🎞️ Subscribes to what a running operation hands the shell mid-operation: the `UiDirtyScope` to refresh (a tool run's
   * live process) and the `HistoryPatch` a running history replay publishes (its stage and `done/total`), each absent
   * when the frame carried none. Best effort: the shell's refresh coalescer drops what it cannot keep up with. Returns
   * the unsubscribe. */
  readonly subscribeOperationProgress: (instanceId: number, listener: (progress: { readonly uiScope?: InvocationResponse["uiScope"]; readonly historyPatch?: HistoryPatch }) => void) => () => void;
  /** 💼️ Fires (coalesced) while an Isolated spawned job of `instanceId` reports `step-job` progress — the
   * shell answers with a full refresh so surfaces adopting the job's retained output re-render as it advances. */
  readonly subscribeSpawnedJobProgress?: (instanceId: number, listener: () => void) => () => void;
  readonly dispose: () => Promise<void>;
};
//#endregion 🔖️Merge

//#region 🔖️ActorAdapter
/** 🏁️ One typed operation's terminal publication, delivered to every subscriber of
 * {@link PluginWasmHandle.subscribeOperationCompletions}. A mounted operation finishes turns long
 * after the command that started it resolved, so NOTHING in the invocation response describes its
 * outcome — `uiScope`, `historyPatch`, `requestedEffects` and `terminalOutput` are that outcome, and
 * this is their only delivery path.
 *
 * 🧾️ `terminalOutput` is the operation's terminal result value, the exact carrier
 * {@link invocationFromFrames} substitutes into `InvocationResponse.output` when the terminal page is
 * drained inside a host call's own turn. A job-routed verb never drains it there — its admitting reply
 * is the `{ operationId, generation }` handle — so without this field the value a caller dispatched
 * the verb FOR (the Home directory-projection receipt) was decoded off the wire, carried in
 * `pendingCompletionEffects`, and then dropped unread. `undefined` when the operation published none. */
export type PluginOperationCompletion = Readonly<{
  instanceId: number;
  operation: bigint;
  revision: bigint;
  uiScope: InvocationResponse["uiScope"];
  historyPatch: HistoryPatch | undefined;
  requestedEffects: readonly Effect[];
  terminalOutput: unknown;
}>;
//#endregion 🔖️ActorAdapter

//#region 🔖️Transaction
/** 🎫️ One member of a resolved transaction — mirrors the Rust host's transaction member shape
 * (contract freeze §5). */
export type TransactionMember = {
  readonly pluginId: string;
  readonly instanceId: number;
  readonly artifactId: string;
};

/** 🎫️ `AppCommand::TransactionPrepare`'s two frozen wire forms (contract freeze §2): owner-mutation
 * (`mutationId`+`payload`, single op) or pre-planned (`preparedOps`+`origin`, an op list). */
export type TransactionPrepareRequest =
  | { readonly form: "owner"; readonly mutationId: string; readonly payload: Uint8Array }
  | { readonly form: "planned"; readonly preparedOps: readonly Uint8Array[]; readonly origin: Uint8Array };

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
  readonly foreign: readonly Uint8Array[];
};

export type TransactionOutcome = { readonly ok: true; readonly txnId: string; readonly editIds: ReadonlyMap<string, string> } | { readonly ok: false; readonly code: string };
//#endregion 🔖️Transaction
//#endregion 🔖️plugin-runtime
