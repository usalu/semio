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
  type ContextMenuItemSpec,
  type HostEffect,
  type InvocationResponse,
  type PluginContextMenuRequest,
  type PluginModuleLease,
  type PluginRegistryEntry,
  type PluginUiRefreshRequest,
  type PluginUiRefreshResponse,
  type PluginUiRefreshSectionResponse,
  SemioFaultError,
  acquirePluginModule,
} from "@semio-tech/framework-core";
import {
  AppChannelClient,
  type AppFrameValue,
  decodeActionWire,
  decodeFaultFromWire,
  decodeOperationEnvelopesPack,
  decodePackValue,
  encodePackValue,
  faultDisplayMessage,
  type SectionProbe,
} from "@semio-tech/framework-os-core";
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
  /** 🔗️ The `DocumentApp` document-sync surface (WS-D) — optional since not every program has migrated onto it yet (WS-F).
   * 🚧️ Wave 1 gap (documented, not silently dropped): `protocol_channel::AppCommand` only carries
   * binary `pack`/`spr` document-container bytes (`LoadDocument`/`ReadDocument`, backed by
   * `store::print_document_pack`/`parse_document_pack`'s deflate+BLAKE3 `.spk` container) — there is
   * no JSON-text document command on the new channel, and no TS-side encoder for that container
   * format (deliberately out of scope for `🔖️PackValueCodec`, see its header doc). The OLD
   * `applyOperations`/`readAppDocument`/`loadAppDocument` all carried plain JSON text
   * (`OperationEnvelope[]` / a VCS envelope string), so they cannot be rebuilt on top of the binary
   * channel without a real pack encoder in TS (a separate, much larger work package). Every call
   * site already feature-detects these (`if (plugin.loadAppDocument) ...`), so leaving them
   * `undefined` here fails loud-but-inert (a `console.error`/no-op at the call site) rather than
   * silently miscoding a `.spk` container. */
  readonly applyOperations?: (instanceId: number, operationsPack: string) => Promise<void>;
  readonly readAppDocument?: (instanceId: number) => Promise<string>;
  readonly loadAppDocument?: (instanceId: number, documentJson: string) => Promise<void>;
  /** 📂️ Binary pack+spr document load (`AppCommand::LoadDocument`) — the Wave-1 channel-native path. */
  readonly loadAppDocumentPack?: (instanceId: number, pack: Uint8Array, spr: Uint8Array) => Promise<void>;
  readonly attachBackbone?: (instanceId: number, uri: string) => Promise<void>;
  readonly detachBackbone?: (instanceId: number) => Promise<void>;
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
 * convention `readVarintU64`/`writeVarintU64` already use throughout `@semio-tech/framework-os-core`).
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
type WireCommandEnvelope = { readonly kind: "action" | "command"; readonly name: string; readonly args: unknown };

/** 🎯️ Shared by `handleAction`/`handleCommand`: encodes `envelope` + `viewState`, sends one
 * `AppCommand::Command` frame, and reassembles the `Invocation`/`Effects`/`Events` frames it produces
 * back into the `InvocationResponse` shape the rest of this file already consumes. `operations`/
 * `inverseGroup` stay at their empty defaults — Wave 1's `AppFrame::Invocation` carries only
 * `output`/`diagnostics` (see `plugin_exchange`'s `AppCommand::Command` arm); no call site in this
 * file reads either field (only `.requestedEffects`/`.uiScope`), confirmed by grep before this
 * adapter was written. `uiScope` stays `undefined` too (not sent over the wire yet), which
 * `resolveUiDirtyScope` already treats as `full` — the safe default. */
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

async function performCommand(client: AppChannelClient, envelope: WireCommandEnvelope, viewState: unknown): Promise<InvocationResponse> {
  const frames = await client.command(encodePackValue(envelope), viewState);
  let output: unknown = null;
  let diagnostics: InvocationResponse["diagnostics"] = [];
  let requestedEffects: HostEffect[] = [];
  let events: InvocationResponse["events"] = [];
  for (const frame of frames) {
    if ("Invocation" in frame) {
      output = decodePackValue(new Uint8Array(frame.Invocation.output));
      const decodedDiagnostics = decodePackValue(new Uint8Array(frame.Invocation.diagnostics));
      diagnostics = Array.isArray(decodedDiagnostics) ? (decodedDiagnostics as InvocationResponse["diagnostics"]) : [];
    } else if ("Effects" in frame) {
      requestedEffects = frame.Effects.effects.map((bytes) => normalizeWireHostEffect(decodePackValue(new Uint8Array(bytes))));
    } else if ("Events" in frame) {
      events = frame.Events.events.map((bytes) => decodePackValue(new Uint8Array(bytes))) as InvocationResponse["events"];
    } else if ("Error" in frame) {
      const fault = decodeFaultFromWire(frame.Error.fault, decodePackValue);
      if (fault) throw new SemioFaultError(fault);
      throw new Error(`[DEBUG] ${envelope.kind} '${envelope.name}' failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
    }
  }
  return {
    output,
    operations: [],
    inverseGroup: { invocationId: "", operations: [], inverseOperations: [] },
    diagnostics,
    requestedEffects,
    events,
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
    handleAction: (instanceId, actionJson, viewState) => {
      const parsed = decodeActionWire(actionJson);
      return performCommand(requireChannel(instanceId), { kind: "action", name: parsed.action, args: parsed.args }, viewState);
    },
    handleCommand: (instanceId, commandJson, viewState) => {
      const parsed = decodeActionWire(commandJson);
      return performCommand(requireChannel(instanceId), { kind: "command", name: parsed.action, args: parsed.args }, viewState);
    },
    refreshUi: (instanceId, request) => performRefreshUi(requireChannel(instanceId), request),
    contextMenu: (instanceId, request) => performContextMenu(requireChannel(instanceId), request),
    applyOperations: async (instanceId, operationsPack) => {
      const envelopes = decodeOperationEnvelopesPack(operationsPack);
      const frames = await requireChannel(instanceId).applyEnvelopes(envelopes);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`[DEBUG] applyOperations failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
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
    dispose: () => lease.release(),
  };
}
//#endregion 🔖️ChannelAdapter
//#endregion 🔖️plugin-runtime
