/// <reference types="vitest/importMeta" />
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/PluginRuntime/component.tsx
/** @emoji 🐚️ `PluginRuntime` — the `PluginWasmHandle` binary-channel adapter (`loadPluginModule`/
 * `adaptPluginHandle`) that wraps a leased `framework-core` plugin wasm module's 5-function
 * `exchange` ABI behind the wider action/command/refreshUi/contextMenu/document-sync surface the
 * rest of the shell calls, plus the `AppChannelClient` frame-reassembly helpers
 * (`🔖️ChannelAdapter`) that back it.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import {
  type ArtifactInstanceRef,
  ArtifactMutationRouter,
  type ContextMenuItemSpec,
  type HostEffect,
  type HistoryPatch,
  InstanceDirectory,
  type InvocationResponse,
  type PluginContextMenuRequest,
  type PluginGraphError,
  type PluginModuleLease,
  type PluginRegistryEntry,
  type PluginUiRefreshRequest,
  type PluginUiRefreshResponse,
  type PluginUiRefreshSectionResponse,
  SemioFaultError,
  acquirePluginModule,
  orderPluginRegistryEntries,
} from "@semio-tech/framework";
import {
  AppChannelClient,
  type AppFrameValue,
  decodeFaultFromWire,
  decodeMutationEnvelopesPack,
  decodePackValue,
  encodePackValue,
  faultDisplayMessage,
  type SectionProbe,
} from "@semio-tech/framework-os";
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
  readonly applyMutations?: (instanceId: number, mutationsPack: string) => Promise<void>;
  readonly readAppDocument?: (instanceId: number) => Promise<string>;
  readonly loadAppDocument?: (instanceId: number, documentJson: string) => Promise<void>;
  /** 📂️ Binary pack+spr document load (`AppCommand::LoadDocument`) — the Wave-1 channel-native path. */
  readonly loadAppDocumentPack?: (instanceId: number, pack: Uint8Array, spr: Uint8Array) => Promise<void>;
  readonly attachBackbone?: (instanceId: number, uri: string) => Promise<void>;
  readonly detachBackbone?: (instanceId: number) => Promise<void>;
  readonly ephemeralSnapshot?: (instanceId: number) => Promise<{ readonly presence: readonly number[]; readonly presenceGeneration: number; readonly transientGeneration: number } | null>;
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
  readonly dispose: () => void;
};

export type { PluginRegistryEntry };

/** 🐚️ Acquires a refcounted lease on the shared core module (see `acquirePluginModule`) and adapts it —
 * this `PluginWasmHandle`'s own `dispose()` releases that lease rather than tearing down the shared
 * module directly, so several shells (or several call sites within one shell) loading the same
 * `moduleUrl` don't dispose it out from under each other. */
export async function loadPluginModule(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  return adaptPluginHandle(pluginId, await acquirePluginModule(pluginId, moduleUrl));
}

//#region 🔖️ChannelAdapter
/** 🔍️ `SectionProbe.kind` byte convention — mirrors `plugin_exchange`'s `SECTION_KIND_*` consts
 * (`framework/os/module/plugin/rs/lib.rs` `🔖️Exchange` region) byte-for-byte. No shared
 * WIT/protocol_channel enum exists for this yet (Wave 1 scope), so the numbering is duplicated here
 * as the single TS-side consumer. */
const SECTION_KIND_WINDOW = 0;
const SECTION_KIND_PANEL = 1;
const SECTION_KIND_ENGAGEMENTS = 2;
const SECTION_KIND_MEASURES = 3;
const SECTION_KIND_TOOLS = 4;
const SECTION_KIND_LABELS = 5;

/** 🐢️ `PluginUiRefreshSectionRequest.hash`/`PluginUiRefreshSectionResponse.hash` are hex strings
 * (opaque to this file — never parsed by Rust, only echoed back on the next request); the wire
 * `SectionProbe.hash`/`AppFrame.UiSection.hash` are plain `number` u64s (the same JS-`number`
 * convention `readVarintU64`/`writeVarintU64` already use throughout `@semio-tech/framework-os`).
 * These two converters are this adapter's own round-trip only — any consistent base works. */
function hashHexToWire(hex: string | undefined): number | null {
  if (hex === undefined) return null;
  const parsed = Number.parseInt(hex, 16);
  return Number.isFinite(parsed) ? parsed : null;
}
function hashWireToHex(value: number): string {
  return Math.trunc(value).toString(16);
}

/** 🎛️ The fallback envelope `plugin_exchange`'s `dispatch_command_frame` decodes when an app hasn't
 * overridden `DocumentApp::handle_typed_command` yet (`{kind, name, args}`, `store::pack_rt`-wire-value
 * encoded) — see that function's doc comment in `framework/os/module/plugin/rs/lib.rs`. */
/** 🎯️ Shared by `handleAction`/`handleCommand`: encodes `envelope` + `viewState`, sends one
 * `AppCommand::Command` frame, and reassembles the `Invocation`/`Effects`/`Events` frames it produces
 * back into the `InvocationResponse` shape the rest of this file already consumes. `operations`/
 * `inverseGroup` stay at their empty defaults — Wave 1's `AppFrame::Invocation` carries only
 * `output`/`diagnostics` (see `plugin_exchange`'s `AppCommand::Command` arm); no call site in this
 * file reads either field (only `.requestedEffects`/`.uiScope`), confirmed by grep before this
 * adapter was written. The invocation frame now carries the authoritative `uiScope` and optional
 * `historyPatch`, so consumers can apply history before any effects schedule refresh work. */
/** 🎯️ DslValue serde encodes Rust enums as `{ kind, value }` (struct) / plain string (unit). Shell
 * `HostEffect` consumers expect serde externally-tagged JSON (`{ navigate: { uri } }` / `"requestSync"`). */
/** 🎯️ DslValue may ship `Vec<u8>` as a number array, a Uint8Array, or a `{ kind:"bytes", value }` object. */
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

function normalizeWireHostEffect(raw: unknown): HostEffect {
  if (typeof raw === "string") return raw as HostEffect;
  if (!raw || typeof raw !== "object") {
    throw new Error(`[DEBUG] invalid host effect: ${JSON.stringify(raw)}`);
  }
  const record = raw as Record<string, unknown>;
  if (typeof record.kind === "string") {
    const kind = record.kind;
    if ("value" in record) {
      return { [kind]: record.value } as HostEffect;
    }
    if ("fields" in record) {
      return { [kind]: record.fields } as HostEffect;
    }
    return kind as HostEffect;
  }
  return raw as HostEffect;
}

async function performInvocation(client: AppChannelClient, invocation: unknown, invocationKind: "action" | "command", viewState: unknown): Promise<InvocationResponse> {
  const frames = await client.command(encodePackValue(invocation), viewState);
  let output: unknown = null;
  let diagnostics: InvocationResponse["diagnostics"] = [];
  let requestedEffects: HostEffect[] = [];
  let events: InvocationResponse["events"] = [];
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
    } else if ("Effects" in frame) {
      requestedEffects = frame.Effects.effects.map((bytes) => normalizeWireHostEffect(decodePackValue(new Uint8Array(bytes))));
    } else if ("Events" in frame) {
      events = frame.Events.events.map((bytes) => decodePackValue(new Uint8Array(bytes))) as InvocationResponse["events"];
    } else if ("Error" in frame) {
      const fault = decodeFaultFromWire(frame.Error.fault, decodePackValue);
      if (fault) throw new SemioFaultError(fault);
      throw new Error(`${invocationKind} failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
    }
  }
  return {
    output,
    mutations: [],
    inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] },
    diagnostics,
    requestedEffects,
    events,
    uiScope,
    historyPatch,
  };
}

/** 🔄️ One `PluginUiRefreshRequest` section, tagged with where its `AppFrame::UiSection` reply must
 * land in the reassembled `PluginUiRefreshResponse` — `plugin_exchange`'s `AppCommand::RefreshUi`
 * handling always emits exactly one `UiSection` frame per requested `SectionProbe`, in request order
 * (verified against its source), so positional zipping (not `key` round-tripping) is what recovers
 * the window/panel INSTANCE id `SectionProbe.key` can't carry (the wire probe's `key` must instead be
 * the render body-key `plugin_exchange` renders against). */
type RefreshUiTarget =
  | { readonly kind: "window" | "panel"; readonly key: string }
  | { readonly kind: "engagements" | "measures" | "tools" | "labels" };

async function performRefreshUi(client: AppChannelClient, request: PluginUiRefreshRequest): Promise<PluginUiRefreshResponse> {
  const probes: SectionProbe[] = [];
  const targets: RefreshUiTarget[] = [];
  for (const entry of request.windows ?? []) {
    probes.push({ kind: SECTION_KIND_WINDOW, key: entry.bodyKey ?? entry.key, hash: hashHexToWire(entry.hash) });
    targets.push({ kind: "window", key: entry.key });
  }
  for (const entry of request.panels ?? []) {
    probes.push({ kind: SECTION_KIND_PANEL, key: entry.bodyKey ?? entry.key, hash: hashHexToWire(entry.hash) });
    targets.push({ kind: "panel", key: entry.key });
  }
  if (request.engagements) {
    probes.push({ kind: SECTION_KIND_ENGAGEMENTS, key: "engagements", hash: hashHexToWire(request.engagements.hash) });
    targets.push({ kind: "engagements" });
  }
  if (request.measures) {
    probes.push({ kind: SECTION_KIND_MEASURES, key: "measures", hash: hashHexToWire(request.measures.hash) });
    targets.push({ kind: "measures" });
  }
  if (request.tools) {
    probes.push({ kind: SECTION_KIND_TOOLS, key: "tools", hash: hashHexToWire(request.tools.hash) });
    targets.push({ kind: "tools" });
  }
  if (request.labels) {
    probes.push({ kind: SECTION_KIND_LABELS, key: "labels", hash: hashHexToWire(request.labels.hash) });
    targets.push({ kind: "labels" });
  }
  if (probes.length === 0) return {};

  const frames = await client.refreshUi(probes, request.viewState ?? {});
  const sections = frames.filter((frame): frame is Extract<AppFrameValue, { readonly UiSection: unknown }> => "UiSection" in frame);
  let requestedEffects: HostEffect[] = [];
  const refreshSeq = sections[0]?.UiSection.in_reply_to;
  if (refreshSeq !== undefined) {
    for (const frame of frames) {
      if ("Effects" in frame && frame.Effects.in_reply_to === refreshSeq) {
        requestedEffects = frame.Effects.effects.map((bytes) => normalizeWireHostEffect(decodePackValue(new Uint8Array(bytes))));
      }
    }
  }
  if (sections.length !== probes.length) {
    const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    throw new Error(errorFrame ? `[DEBUG] refreshUi failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}` : `[DEBUG] refreshUi: expected ${probes.length} UiSection frames, got ${sections.length}`);
  }

  const windows: PluginUiRefreshSectionResponse[] = [];
  const panels: PluginUiRefreshSectionResponse[] = [];
  let engagements: PluginUiRefreshSectionResponse | undefined;
  let measures: PluginUiRefreshSectionResponse | undefined;
  let tools: PluginUiRefreshSectionResponse | undefined;
  let labels: PluginUiRefreshSectionResponse | undefined;
  sections.forEach((frame, index) => {
    const target = targets[index]!;
    const section = frame.UiSection;
    const response: PluginUiRefreshSectionResponse = {
      key: target.kind === "window" || target.kind === "panel" ? target.key : target.kind,
      hash: hashWireToHex(section.hash),
      value: section.body !== null ? decodePackValue(new Uint8Array(section.body)) : undefined,
    };
    if (target.kind === "window") windows.push(response);
    else if (target.kind === "panel") panels.push(response);
    else if (target.kind === "engagements") engagements = response;
    else if (target.kind === "measures") measures = response;
    else if (target.kind === "tools") tools = response;
    else labels = response;
  });
  return {
    ...(windows.length > 0 ? { windows } : {}),
    ...(panels.length > 0 ? { panels } : {}),
    ...(engagements ? { engagements } : {}),
    ...(measures ? { measures } : {}),
    ...(tools ? { tools } : {}),
    ...(labels ? { labels } : {}),
    ...(requestedEffects.length > 0 ? { requestedEffects } : {}),
  };
}

async function performContextMenu(client: AppChannelClient, request: PluginContextMenuRequest): Promise<readonly ContextMenuItemSpec[]> {
  const items = await client.contextMenu(request);
  return Array.isArray(items) ? (items as ContextMenuItemSpec[]) : [];
}

/** 📡️ Wraps the framework-core `PluginWasmHandle` (the 5-function binary `exchange` ABI) behind the
 * SAME method surface the rest of this file already calls — the compatibility adapter for
 * `HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS`'s ABI flip. One `AppChannelClient` per
 * live instance id (created in `createApp`, dropped in `destroyApp`) frames every call through
 * `AppCommand`/`AppFrame`; no `AppCommand::Hello` handshake is sent — `plugin_exchange` already
 * defaults an un-`Hello`'d instance's actor to `"local"` (see `instance_actor`'s doc), so skipping it
 * avoids the alternative (sending a real `Hello.config`, which would run every migrated app's
 * `apply_config_bytes` against an arbitrary empty/placeholder config — wrong for an app like shooting
 * whose `ShootingConfig` fields have no `#[serde(default)]` and would reject `{}`). */
export async function adaptPluginHandle(pluginId: string, lease: PluginModuleLease): Promise<PluginWasmHandle> {
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
      channels.set(instanceId, new AppChannelClient(handle, instanceId, appId));
      return instanceId;
    },
    destroyApp: async (instanceId) => {
      channels.delete(instanceId);
      await handle.destroyApp(instanceId);
    },
    handleAction: (instanceId, actionJson, viewState) => performInvocation(requireChannel(instanceId), JSON.parse(actionJson), "action", viewState),
    handleCommand: (instanceId, commandJson, viewState) => performInvocation(requireChannel(instanceId), JSON.parse(commandJson), "command", viewState),
    refreshUi: (instanceId, request) => performRefreshUi(requireChannel(instanceId), request),
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
    },
    readAppDocument: undefined,
    loadAppDocument: undefined,
    loadAppDocumentPack: async (instanceId, pack, spr) => {
      const frames = await requireChannel(instanceId).loadDocument(pack, spr);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`[DEBUG] loadAppDocumentPack failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    },
    attachBackbone: async (instanceId, uri) => {
      const frames = await requireChannel(instanceId).attachBackbone(uri);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) console.error(`[DEBUG] program ${pluginId}: attachBackbone failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    },
    detachBackbone: async (instanceId) => {
      const frames = await requireChannel(instanceId).detachBackbone();
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) console.error(`[DEBUG] program ${pluginId}: detachBackbone failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    },
    ephemeralSnapshot: async (instanceId) => {
      const frames = await requireChannel(instanceId).drain();
      const frame = frames.find((candidate): candidate is Extract<AppFrameValue, { readonly Ephemeral: unknown }> => "Ephemeral" in candidate);
      return frame ? { presence: frame.Ephemeral.presence, presenceGeneration: frame.Ephemeral.presence_generation, transientGeneration: frame.Ephemeral.transient_generation } : null;
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
      const fakeLease: PluginModuleLease = {
        handle: {
          manifest: async () => encodePackValue({ pluginId: "b-plugin", label: "B", version: "1.0.0", apps: [], workflows: [], examples: [] }),
          createApp: async () => 20,
          destroyApp: async () => {},
          exchange: async (_instanceId, frames) => {
            const commands = frames.map((frame) => decodeAppCommand(frame));
            seenCommands.push(...commands);
            return commands.map((command) => {
              if (command !== "Bye" && "transactionPrepare" in command) {
                return encodeAppFrame({ transactionPrepared: { txn_id: command.transactionPrepare.txn_id, foreign: [], rejection: [] } });
              }
              if (command !== "Bye" && "transactionCommit" in command) {
                return encodeAppFrame({ transactionCommitted: { txn_id: command.transactionCommit.txn_id, edit_id: "edit-1" } });
              }
              if (command !== "Bye" && "ReadDocument" in command) {
                return encodeAppFrame({ Document: { in_reply_to: command.ReadDocument.seq, pack: [5, 5], spr: [6], ops: "" } });
              }
              return encodeAppFrame({ Done: { in_reply_to: 0 } });
            });
          },
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
      const fakeLease: PluginModuleLease = {
        handle: {
          manifest: async () => encodePackValue({ pluginId: "b-plugin", label: "B", version: "1.0.0", apps: [], workflows: [], examples: [] }),
          createApp: async () => 20,
          destroyApp: async () => {},
          exchange: async (_instanceId, frames) => {
            const commands = frames.map((frame) => decodeAppCommand(frame));
            return commands.map((command) => (command !== "Bye" && "LoadDocument" in command ? encodeAppFrame({ Done: { in_reply_to: command.LoadDocument.seq } }) : encodeAppFrame({ Done: { in_reply_to: 0 } })));
          },
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
