import { type Effect, GUEST_HOST_ANSWER_CEILING_BYTES, guestAnswerPages, type InvocationResponse, type PluginDispatchHintV1, type PluginManifest, SemioFaultError } from "@semio-tech/framework";
import { AppChannelClient, type AppFrameValue, decodeAppFrame, decodeFaultFromWire, decodeInvocationResultPacks, decodePackValue, decodePackWire, encodePackValue, faultDisplayMessage, type WindowConfigPackEntry } from "@semio-tech/framework-os";
import { leftoverShellInvocationFrames, scanTypedOperationPages, type TypedOperationAckEvent, type WireTurnResult, wireTurnStatusTag, type WireVariant } from "../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import { pendingTurnEffects, wgpuHostEffect } from "../🎛️contributions-and-effects/🟦️.ts";
import type { MediaTransportPort } from "../../../../🎬️media/🚚️lifecycle/🟦️.ts";
import type { ActorCodecRequest, ActorCodecSuccess } from "../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧬️codec/🟦️.ts";

export type WgpuPluginExtensionCompletion = {
  readonly instanceId: number;
  readonly req: bigint;
  assertActive(): void;
  complete(outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array }): Promise<InvocationResponse>;
};

/** 📄️ Cuts one host answer into guest-safe pages — the wgpu twin of PluginRuntime's capture. */
export function wgpuGuestAnswerPages(answer: Uint8Array): { readonly prologue: readonly Uint8Array[]; readonly terminal: Uint8Array } {
  return guestAnswerPages(answer);
}

export function wgpuHostAnswerCeilingBytes(): number {
  return GUEST_HOST_ANSWER_CEILING_BYTES;
}

/** 📏️ Continuation turns ONE host call may spend settling the typed operations it started before
 * it hands the rest to {@link drainTypedOperationTurns}. A `Migrated` interactive job returns its
 * "started" reply immediately, so this only has to carry the operation far enough to publish its
 * first pages and their acknowledgements. */
export const WGPU_TYPED_OPERATION_SETTLE_LIMIT = 64;

/** 📏️ Continuation polls the standing drain submits for one instance before it gives up — the wgpu
 * twin of `PLUGIN_OPERATION_DRAIN_BUDGET`. */
export const WGPU_TYPED_OPERATION_DRAIN_BUDGET = 4_096;

/** 📏️ Retained authority for the host effects a drain accumulates between drains. An operation
 * publishes at most one host effect per continuation turn, so a completion that never arrives would
 * otherwise grow without bound; exceeding it fails closed. */
export const WGPU_TYPED_OPERATION_EFFECT_CAPACITY = 4_096;

/** 📬️ The wgpu host's half of the typed-operation result exchange.
 *
 * ⚖️ `route_exchange_output` (`⚛️reactor/🔄️turn/🦀️.rs`) publishes a mounted operation's result pages as
 * `Effect::SendMessage{Shell{instance}}` beside the ordinary `AppFrame` replies, and the guest PARKS
 * the operation until the host answers each page with its ACK token. A host that does not answer
 * freezes the operation mid-flight — which is why `flowEvalTick` could reach the guest's command door
 * on this target and never produce a `flowEvalResolve`. This type is the whole obligation in one
 * place: collect the pages out of a turn's effects, hand back the acknowledgements the next
 * submission owes, surface a fault page by name, and leave the remaining effects untouched for the
 * host's own routing. The page codec itself is `🖼️wire-turn/🟦️.ts`'s, shared with React. */
export class WgpuTypedOperationDrive {
  #acknowledgements: TypedOperationAckEvent[] = [];
  #pages = 0;
  #terminal = false;
  readonly #hostEffects = new WeakMap<WireTurnResult, readonly WireVariant[]>();

  constructor(private readonly instanceId: number) {}

  /** 📬️ Scans every accepted turn once, keeping its non-page effects and queueing its ACKs. */
  observe(turns: readonly WireTurnResult[]): void {
    for (const turn of turns) {
      if (this.#hostEffects.has(turn)) continue;
      const scan = scanTypedOperationPages(turn.effects);
      this.#hostEffects.set(turn, scan.kept);
      this.#acknowledgements.push(...scan.acknowledgements);
      this.#pages += scan.pages.length;
      this.#terminal ||= scan.terminal;
    }
  }

  /** 📬️ The acknowledgements the NEXT submitted turn owes, handed over exactly once. */
  takeAcknowledgements(): readonly TypedOperationAckEvent[] {
    const owed = this.#acknowledgements;
    this.#acknowledgements = [];
    return owed;
  }

  /** 🧩️ Every effect of `turns` that was NOT transport — pages already consumed by {@link observe}. */
  hostEffects(turns: readonly WireTurnResult[]): WireVariant[] {
    return turns.flatMap((turn) => this.#hostEffects.get(turn) ?? turn.effects);
  }

  /** 🔄️ Whether the actor still owes this call a turn: an unanswered page, or a guest that says it
   * has work left this activation. */
  owesASettle(last: WireTurnResult | undefined): boolean {
    return this.#acknowledgements.length > 0 || wireTurnStatusTag(last?.status) === "more-work";
  }

  /** 🔄️ Whether the turns just accepted did anything at all — a settle loop that keeps submitting
   * against a silent actor is the drain's job, not a host call's. */
  progressed(turns: readonly WireTurnResult[]): boolean {
    return this.#acknowledgements.length > 0 || turns.some((turn) => turn.uiPatches.length > 0 || turn.effects.length > 0);
  }

  report(_phase: string): void {}
}

/** 🏁️ One guest publication that answers NO host call, as the shell takes it between two dispatches.
 *
 * ⚖️ A typed operation outlives the command that started it: the admitting reply carries no outcome, and the
 * operation's own `UiDirtyScope` and command-log delta arrive turns later — on an `AppFrame::OperationCompleted`, the
 * ONLY carrier of both, and on the throttled UI-progress `Invocation` frames a long run pushes meanwhile. React hears
 * them through `subscribeOperationCompletions` and `subscribeOperationProgress`. This target heard neither: the
 * completion matched no waiter and fell into the channel's completion lane nothing subscribed to, the progress scope
 * waited in the leftover ledger for a dispatch that had no reason to come. An operation that ended on the standing
 * drain — an example load of a few hundred one-item steps, the replay behind an accepted history edit — therefore
 * changed the document and left the surface and the History rows as they were
 * (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, live fault F11).
 *
 * The law of what each publication owes the shell is the shared corpus
 * `🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧫️operation-publication/🔣️.json`. */
export type WgpuOperationPublication = {
  /** 🏁️ `true` for an `AppFrame::OperationCompleted`; `false` for a UI-progress frame and for the drain's own report. */
  readonly completed: boolean;
  readonly uiScope?: NonNullable<InvocationResponse["uiScope"]>;
  readonly historyPatch?: NonNullable<InvocationResponse["historyPatch"]>;
  /** 🧯️ Why the standing drain stopped — the shell tells it as a notice; a refused completion is never silence. */
  readonly fault?: string;
  /** 🧹️ The bounded lane gave up entries before this one: the shell re-reads the whole history projection. */
  readonly resync?: true;
};

/** 🏁️ Per-instance publications the shell has not taken yet, oldest first ({@link takeOperationPublications}). */
export const pendingOperationPublications = new Map<number, WgpuOperationPublication[]>();

/** 📏️ How many untaken publications one instance keeps. A history patch is a DELTA — its rows exist nowhere else — so
 * a lane that outgrows this authority never drops its oldest entries: it collapses into one `resync` publication. */
export const WGPU_OPERATION_PUBLICATION_CAPACITY = 64;

/** 🏁️ Queues one publication for the shell's next take. A publication that dirtied nothing, patched no history and
 * reports no fault is bookkeeping — a retained operation completes on every drain poll — and is not queued at all. */
export function stashOperationPublication(instanceId: number, publication: WgpuOperationPublication): void {
  if (publication.fault === undefined && publication.historyPatch === undefined && publication.resync !== true && (publication.uiScope === undefined || publication.uiScope.kind === "none")) return;
  const queue = [...(pendingOperationPublications.get(instanceId) ?? []), publication];
  if (queue.length <= WGPU_OPERATION_PUBLICATION_CAPACITY) {
    pendingOperationPublications.set(instanceId, queue);
    return;
  }
  const fault = queue.reduce<string | undefined>((last, entry) => entry.fault ?? last, undefined);
  pendingOperationPublications.set(instanceId, [{ completed: queue.some((entry) => entry.completed), uiScope: { kind: "full" }, resync: true, ...(fault === undefined ? {} : { fault }) }]);
}

/** 🏁️ The publication one `AppChannelClient.onOperationCompleted` completion is — its scope and history patch, each
 * only when the frame carried one. */
export function operationCompletionPublication(completion: { readonly uiScope?: unknown; readonly historyPatch?: unknown }): WgpuOperationPublication {
  const uiScope = completion.uiScope as InvocationResponse["uiScope"] | null;
  const historyPatch = completion.historyPatch as InvocationResponse["historyPatch"] | null;
  return { completed: true, ...(uiScope ? { uiScope } : {}), ...(historyPatch && typeof historyPatch === "object" ? { historyPatch } : {}) };
}

/** 🎞️ Queues the scope and history patch of one uncorrelated UI-progress `Invocation` frame the standing drain
 * uncovered, and answers whether the lane now carries EVERYTHING that frame said. A frame with an output, diagnostics
 * or mutations still owes the next dispatch its fold ({@link wgpuInvocationFromFrames}); a pure progress frame owes it
 * nothing, and left in the leftover ledger it would be folded a second time over that dispatch's own scope. */
export function stashOperationProgress(instanceId: number, frame: Uint8Array): boolean {
  const decoded = decodeAppFrame(frame) as AppFrameValue;
  if (!("Invocation" in decoded) || decoded.Invocation.in_reply_to !== 0) return false;
  const carried = decoded.Invocation;
  if (carried.ui_scope.length === 0 && carried.history_patch.length === 0) return false;
  const uiScope = carried.ui_scope.length > 0 ? (decodePackWire(new Uint8Array(carried.ui_scope), "progress.uiScope") as InvocationResponse["uiScope"] | null) : null;
  const historyPatch = carried.history_patch.length > 0 ? (decodePackWire(new Uint8Array(carried.history_patch), "progress.historyPatch") as InvocationResponse["historyPatch"] | null) : null;
  stashOperationPublication(instanceId, { completed: false, ...(uiScope ? { uiScope } : {}), ...(historyPatch && typeof historyPatch === "object" ? { historyPatch } : {}) });
  return carried.output.length === 0 && carried.diagnostics.length === 0 && carried.mutations.length === 0 && carried.inverse_group.length === 0;
}

/** 🏁️ Hands the shell every publication queued for `instanceId` since its last take, oldest first. */
export function takeOperationPublications(instanceId: number): readonly WgpuOperationPublication[] {
  const queue = pendingOperationPublications.get(instanceId) ?? [];
  pendingOperationPublications.delete(instanceId);
  return queue;
}

/** 📥️ Projects the four `pack`-typed payloads of one `Invocation` reply frame onto exact JSON. The wire
 * codec returns lossless integer carriers, so a raw decode would hand the shell `{kind, value}` objects
 * wherever a plugin returned a `u64` — the `render`/`handleAction` output, the diagnostics list, the UI
 * scope and the history patch all cross this one boundary. */
export type AppFrameInvocationPayload = { readonly output: ArrayLike<number>; readonly diagnostics: ArrayLike<number>; readonly ui_scope: ArrayLike<number>; readonly history_patch: ArrayLike<number>; readonly mutations: ArrayLike<number>; readonly inverse_group: ArrayLike<number> };

export function decodeInvocationPayloads(frame: AppFrameInvocationPayload): Pick<InvocationResponse, "output" | "diagnostics" | "uiScope" | "historyPatch" | "mutations" | "inverseGroup"> {
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

/** 📬️ Folds ONE call's answer out of the frames it resolved with AND the leftover effects the same
 * outcome carried — the wgpu twin of `PluginRuntime`'s `invocationFromFrames`, and the only place
 * this target decides what a dispatch answered.
 *
 * 🕹️ The leftover half is not an extra: a framework-reserved tool verb (`interactionSelect`,
 * `interactionHover`, `clearSelection`, `selectAll`, `setSelectionMode`, `setGranularity`) is
 * ADMITTED by the dispatch turn with an empty `InvocationResult` and a `SpawnJob`, and its real
 * answer — `output.interactionView` plus the app-declared refresh scope — is published by the job's
 * completion turn, which lands in the leftover lane. Reading the frames alone therefore answers
 * `uiScope: undefined` (the shell reads `UiDirtyScope::None`), so `refresh` renders NOTHING, the
 * guest is never asked to re-render, and its `World3dScene.selectionJson` stays `[]` however the
 * pick went — the wgpu selection round trip's own defect, measured on 6118 across 7 of 8
 * generation3d examples in both roles (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-selection-roundtrip-2026-09-15.md`).
 *
 * Per-field, never wholesale: an admission that already carried mutations or a history patch must
 * not be blanked by a completion frame that carries neither. */
export function wgpuInvocationFromFrames(frames: readonly AppFrameValue[], leftover: readonly WireVariant[]): InvocationResponse {
  let output: unknown = null;
  let diagnostics: InvocationResponse["diagnostics"] = [];
  let uiScope: InvocationResponse["uiScope"];
  let historyPatch: InvocationResponse["historyPatch"];
  let mutations: InvocationResponse["mutations"] = [];
  let inverseGroup: InvocationResponse["inverseGroup"] = { invocationId: "", mutations: [], inverseMutations: [] };
  const applyInvocationFrame = (frame: AppFrameInvocationPayload): void => {
    if (frame.output.length) output = decodePackWire(new Uint8Array(frame.output), "invocation.output");
    if (frame.diagnostics.length) {
      const decoded = decodePackWire(new Uint8Array(frame.diagnostics), "invocation.diagnostics");
      diagnostics = Array.isArray(decoded) ? (decoded as InvocationResponse["diagnostics"]) : [];
    }
    if (frame.ui_scope.length) uiScope = decodePackWire(new Uint8Array(frame.ui_scope), "invocation.uiScope") as InvocationResponse["uiScope"];
    if (frame.history_patch.length) {
      const decoded = decodePackWire(new Uint8Array(frame.history_patch), "invocation.historyPatch");
      historyPatch = decoded && typeof decoded === "object" ? (decoded as InvocationResponse["historyPatch"]) : undefined;
    }
    if (frame.mutations.length || frame.inverse_group.length) ({ mutations, inverseGroup } = decodeInvocationResultPacks(frame));
  };
  for (const frame of frames) {
    if ("Invocation" in frame) {
      applyInvocationFrame(frame.Invocation);
    } else if ("Error" in frame) {
      const fault = decodeFaultFromWire(frame.Error.fault, decodePackValue);
      if (fault) throw new SemioFaultError(fault);
      throw new Error(`invocation failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
    }
  }
  for (const frame of leftoverShellInvocationFrames<AppFrameValue>(leftover, decodeAppFrame)) {
    if ("Invocation" in frame) applyInvocationFrame(frame.Invocation);
  }
  const requestedEffects = leftover.map((effect) => wgpuHostEffect(effect)).filter((effect): effect is Effect => effect !== null);
  return { output, mutations, inverseGroup, diagnostics, requestedEffects, events: [], uiScope, historyPatch };
}

export async function performInvocation(client: AppChannelClient, instanceId: number, invocation: unknown, viewState: unknown, dispatch?: PluginDispatchHintV1): Promise<InvocationResponse> {
  const frames = await client.command(encodePackValue(invocation), viewState, dispatch);
  const leftover = pendingTurnEffects.get(instanceId) ?? [];
  pendingTurnEffects.delete(instanceId);
  return wgpuInvocationFromFrames(frames, leftover);
}

/** 🎛️ Optional steering for one {@link WgpuPluginHandle.invoke} call — the wgpu twin of
 * `PluginExtensionInvokeContext` (`🔌️PluginRuntime/🟦️.tsx`). */
export interface WgpuPluginInvokeContext {
  readonly originInstanceId?: number;
  readonly signal?: AbortSignal;
  readonly onProgress?: (progress: { readonly capability: string; readonly turns: number; readonly budget: number }) => void;
}

/** 🐚️ The typed handle this file hands to a `bootFrameworkOsWgpu`/`🟦️.ts` caller — narrower than
 * `PluginRuntime`'s wide `PluginWasmHandle` (no transactions/merge/conflicts/backbone/presence): only
 * the surface `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s `wasm32` branch actually calls. */
export interface WgpuPluginHandle extends MediaTransportPort {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  /** 🪪️ The package this module was admitted as, off its served descriptor — the identity a hub
   * execution-target lease must name before the shell binds a hub document to this program. */
  readonly packageId: string;
  readonly componentSha256: string;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly readAppDocumentIdentity: (instanceId: number) => Promise<{ readonly appInstanceId: number; readonly parentDocumentId: string | null }>;
  /** 🎯️ `dispatch` (`PluginDispatchHintV1`, kernel) is the input ledger's causal `order` for this one
   * call, threaded into {@link serializeWgpuActorCall} — see its `## order` section; absent ⇒ arrival order. */
  readonly handleAction: (instanceId: number, invocation: unknown, viewState: unknown, dispatch?: PluginDispatchHintV1) => Promise<InvocationResponse>;
  readonly handleCommand: (instanceId: number, invocation: unknown, viewState: unknown, dispatch?: PluginDispatchHintV1) => Promise<InvocationResponse>;
  readonly render: (instanceId: number, surfaceId: string, bodyKey: string, viewState: unknown) => Promise<unknown>;
  readonly renderDocument: (instanceId: number, surfaceId: string, bodyKey: string, viewState: unknown) => Promise<string>;
  readonly contextMenu: (instanceId: number, request: unknown) => Promise<unknown>;
  readonly readWindowConfigPacks: (instanceId: number) => Promise<readonly WindowConfigPackEntry[]>;
  readonly loadWindowConfigPack: (instanceId: number, entry: WindowConfigPackEntry) => Promise<void>;
  readonly captureExtensionCompletion: (instanceId: number, req: bigint) => WgpuPluginExtensionCompletion;
  /** 📥️ Calls INTO this program — the wgpu counterpart of `PluginRuntime`'s own `invoke`, driving the
   * SAME `Event::Request`/`respond` protocol through the SAME `driveInboundRequest` so the two
   * renderer targets cannot diverge into two inbound-call ABIs. */
  readonly invoke: (capability: string, request: Uint8Array | string, context?: WgpuPluginInvokeContext) => Promise<Uint8Array>;
  readonly dispatchInvokeExtension: (instanceId: number, extensionId: string, capability: string, requestJson: string, req: bigint) => Promise<InvocationResponse>;
  readonly pushScopedContributions: (instanceId: number, appId: string, viewStateJson: string) => Promise<InvocationResponse>;
  /** 📡️ Binds or retires this instance's document backbone at `uri` for `bindingGeneration` — the
   * browser twin of the native `DocumentBackboneBindingCommandV1` exchange. Resolves the host effects
   * the guest left beside its verified receipt; a refusal rejects with the guest's own code. */
  readonly documentBackbone: (instanceId: number, operation: "bind" | "retire", bindingGeneration: bigint, uri: string) => Promise<InvocationResponse>;
  /** 📥️ Delivers one hot backbone message from the document actor at `uri` to this instance. */
  readonly receiveDocumentBackbone: (instanceId: number, uri: string, payload: Uint8Array) => Promise<InvocationResponse>;
  /** 📥️ Force-applies one `protocol::encode_envelopes` causal batch (`AppCommand::ApplyEnvelopes`). */
  readonly applyMutations: (instanceId: number, operations: Uint8Array) => Promise<void>;
  /** 🗃️ Restores one encoded document archive — the one whole-document load (a plain document is an archive without members):
   * stepped admit, poll and acknowledge through `AppChannelClient.loadDocumentArchive`, so a long history folds across turns. */
  readonly loadAppDocumentArchive: (instanceId: number, archive: Uint8Array) => Promise<void>;
  /** 📚️ Captures the complete drawing, history and owned members before a surface transfer. */
  readonly readAppDocumentArchive: (instanceId: number) => Promise<Uint8Array>;
  /** 🧾️ The instance's whole history projection (`AppCommand::ReadHistory` → `HistorySnapshot`), the browser twin of the
   * native shell's `read_history` — what a restored document archive re-reads its rows and head from. */
  readonly readHistory: (instanceId: number) => Promise<unknown>;
  /** 🧬️ Calls this program's component `codec` interface on a live instance's actor, serialized with
   * its turns: the browser twin of the native `OwnedComponentDocumentCodec`. A guest fault rejects
   * with the fault's display message; a program with no live instance refuses. */
  readonly codec: <Request extends ActorCodecRequest>(request: Request) => Promise<ActorCodecSuccess<Request["operation"]>>;
  /** 👥️ The last `AppFrame::Ephemeral` this instance's guest appended to an answer (`AppChannelClient.ephemeral`),
   * or `null` — what the shell's presence heartbeat carries as the peer's app presence pack and interaction. */
  readonly ephemeralSnapshot: (instanceId: number) => WgpuEphemeralSnapshot | null;
  /** 🏁️ What the guest published between two host calls since the last take — typed-operation completions, UI-progress
   * frames and the standing drain's own fault: the wgpu twin of React's `subscribeOperationCompletions` and
   * `subscribeOperationProgress` ({@link takeOperationPublications}). */
  readonly takeOperationPublications: (instanceId: number) => readonly WgpuOperationPublication[];
  readonly dispose: () => Promise<void>;
}

/** 👥️ One instance's published ephemeral state as the JS bridge hands it to Rust: pack bytes, never number arrays —
 * the app presence pack, the interaction slice, and the guest's own tool run and history edit (`AppFrame::Ephemeral`
 * `tool_run` / `history_edit`, empty when it has none), which the presence heartbeat publishes like React's. */
export type WgpuEphemeralSnapshot = { readonly presence: Uint8Array; readonly presenceGeneration: number; readonly interaction: Uint8Array; readonly toolRun: Uint8Array; readonly historyEdit: Uint8Array };

/** 👥️ Projects the app channel's last ephemeral frame onto the bridge's byte shape. */
export function wgpuEphemeralSnapshot(ephemeral: { readonly presence: readonly number[]; readonly presenceGeneration: number; readonly interaction: readonly number[]; readonly toolRun: readonly number[]; readonly historyEdit: readonly number[] } | null): WgpuEphemeralSnapshot | null {
  return ephemeral === null
    ? null
    : { presence: Uint8Array.from(ephemeral.presence), presenceGeneration: ephemeral.presenceGeneration, interaction: Uint8Array.from(ephemeral.interaction), toolRun: Uint8Array.from(ephemeral.toolRun), historyEdit: Uint8Array.from(ephemeral.historyEdit) };
}
