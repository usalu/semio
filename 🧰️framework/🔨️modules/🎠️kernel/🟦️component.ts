// #region 🎠️Kernel
/// <reference types="vitest/importMeta" />
/** @emoji 🎠️ `@semio-tech/framework` — plugin runtime, leases, invocation responses, and playground boot. */
import { PLAYGROUND_BUILD_TARGETS, type PlaygroundBuildTarget } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️playgrounds.ts";
import { PLUGIN_BUILD_TARGETS, PLUGIN_HOST_CONFIGS, EXTENSION_TARGETS, pluginModuleUrl, extensionModuleUrl } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️plugins.ts";
import type { IconName } from "@semio-tech/assets";
import type { ShellLocale, ShellTerminology, LocalizedLabel } from "../🛂️manifest/🤖️generated/🟦️ui-axes.ts";

import type {
  PluginManifest,
  PluginUiNode,
  PluginViewState,
  ProgramContributionEntry,
  WindowLayout,
  NamedLayout,
} from "../🛂️manifest/🟦️component.ts";
import type { StoragePort } from "../🖥️platform/🟦️component.ts";

//#region EphemeralLane
/** 🫧 Process-local box for module ephemeral values — sole lane until OS draft projection owns these keys. */
export type EphemeralBox<T> = { current: T };

const ephemeralBoxes = new Map<string, EphemeralBox<unknown>>();
const ephemeralMaps = new Map<string, Map<unknown, unknown>>();
const ephemeralSets = new Map<string, Set<unknown>>();

/** 🫧 Get-or-create a mutable box keyed for OS draft projection.
 * Init is stored as-is — never treat a function-typed `T` as a lazy factory (that would
 * invoke identity/no-op resolvers and leave `.current` undefined). */
export function ephemeralBox<T>(key: string, init: T): EphemeralBox<T> {
  let box = ephemeralBoxes.get(key) as EphemeralBox<T> | undefined;
  if (!box) {
    box = { current: init };
    ephemeralBoxes.set(key, box as EphemeralBox<unknown>);
  }
  return box;
}

/** 🫧 Get-or-create a process-local Map owned by the ephemeral lane. */
export function ephemeralMap<K, V>(key: string): Map<K, V> {
  let map = ephemeralMaps.get(key) as Map<K, V> | undefined;
  if (!map) {
    map = new Map();
    ephemeralMaps.set(key, map as Map<unknown, unknown>);
  }
  return map;
}

/** 🫧 Get-or-create a process-local Set owned by the ephemeral lane. */
export function ephemeralSet<T>(key: string): Set<T> {
  let set = ephemeralSets.get(key) as Set<T> | undefined;
  if (!set) {
    set = new Set();
    ephemeralSets.set(key, set as Set<unknown>);
  }
  return set;
}

const ephemeralWeakMaps = new Map<string, WeakMap<object, unknown>>();

/** 🫧 Get-or-create a process-local WeakMap owned by the ephemeral lane. */
export function ephemeralWeakMap<K extends object, V>(key: string): WeakMap<K, V> {
  let map = ephemeralWeakMaps.get(key) as WeakMap<K, V> | undefined;
  if (!map) {
    map = new WeakMap();
    ephemeralWeakMaps.set(key, map as WeakMap<object, unknown>);
  }
  return map;
}
//#endregion EphemeralLane

export type PluginWasmHandle = {
  readonly manifest: () => Promise<Uint8Array>;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly exchange: (instanceId: number, frames: Uint8Array[]) => Promise<Uint8Array[]>;
  readonly dispose: () => void;
};

export function buildContributionsJson(loaded: ReadonlyArray<{ readonly pluginId: string; readonly manifest: PluginManifest }>): string {
  const entries: ProgramContributionEntry[] = [];
  for (const entry of loaded) {
    for (const contribution of entry.manifest.contributions ?? []) {
      entries.push({ pluginId: entry.pluginId, contribution });
    }
  }
  return JSON.stringify(entries);
}

export function resolveLayoutForMode(
  app: { readonly defaultLayout?: WindowLayout; readonly namedLayouts?: readonly NamedLayout[]; readonly modes: readonly { readonly id: string; readonly layoutId?: string }[] },
  modeId: string,
): WindowLayout | undefined {
  const mode = app.modes.find((entry) => entry.id === modeId);
  if (mode?.layoutId) {
    const named = app.namedLayouts?.find((entry) => entry.id === mode.layoutId);
    if (named) return named.layout;
  }
  return app.defaultLayout;
}



/**
 * 🧩️ Expands a plugin registry for a primary plugin: `primaryPluginId` is matched directly
 * against entry `pluginId` (no registry-id indirection), then every other entry whose
 * `contributes` intersects the primary entry's `consumes` is appended. Studio mode, or the
 * absence of a primary id, passes the full registry through unchanged.
 */
export function expandPluginRegistry(plugins: readonly PluginRegistryEntry[], primaryPluginId?: string, studioMode = false): readonly PluginRegistryEntry[] {
  if (studioMode || !primaryPluginId) return plugins;
  const primaryEntries = plugins.filter((entry) => entry.pluginId === primaryPluginId);
  const consumes = new Set(primaryEntries.flatMap((entry) => entry.consumes ?? []));
  const contributorEntries = plugins.filter((entry) => entry.pluginId !== primaryPluginId && (entry.contributes ?? []).some((tag) => consumes.has(tag)));
  return [...primaryEntries, ...contributorEntries];
}

export type ExternalSlotResolverContext = {
  readonly plugins: ReadonlyMap<string, PluginWasmHandle>;
  readonly contributorInstances: Map<string, number>;
  readonly viewState: PluginViewState;
};

export async function ensureContributorInstance(pluginId: string, appId: string, context: ExternalSlotResolverContext): Promise<number | null> {
  const existing = context.contributorInstances.get(pluginId);
  if (existing != null) return existing;
  const handle = context.plugins.get(pluginId);
  if (!handle) return null;
  const instanceId = await handle.createApp(appId);
  context.contributorInstances.set(pluginId, instanceId);
  return instanceId;
}

export async function resolveExternalSlots(node: PluginUiNode, context: ExternalSlotResolverContext): Promise<PluginUiNode> {
  if (node.type === "externalSlot") {
    const pluginId = String(node.pluginId ?? "");
    const appId = String(node.appId ?? pluginId);
    const handle = context.plugins.get(pluginId);
    if (!handle) {
      return { type: "text", value: `Extension unavailable: ${pluginId}` };
    }
    const instanceId = await ensureContributorInstance(pluginId, appId, context);
    if (instanceId == null) {
      return { type: "text", value: `Extension unavailable: ${pluginId}` };
    }
    // 🚧️ Rendering a contributor's UI body now goes through `AppChannelClient.refreshUi`
    // (`RefreshUi` → `UiSection` over `exchange`, os-product `🔖️AppChannelClient` region) instead
    // of the removed per-verb `render`/`renderWithDocument`. Wiring that dispatch loop into this
    // exact call site is the dedicated follow-up work package this ticket flags for the React
    // renderer's dispatch/refresh loops — until then an external slot degrades to unavailable
    // rather than silently guessing at `SectionProbe.kind`/body-key framing.
    return { type: "text", value: `Extension unavailable: ${pluginId}` };
  }
  if (node.type === "stack" && Array.isArray(node.children)) {
    const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child as PluginUiNode, context)));
    return { ...node, children };
  }
  if (node.type === "section" && Array.isArray(node.children)) {
    const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child as PluginUiNode, context)));
    return { ...node, children };
  }
  return node;
}

export type PluginRegistryEntry = {
  readonly pluginId: string;
  readonly moduleUrl: string;
  readonly contributes?: readonly string[];
  readonly consumes?: readonly string[];
};

//#region InvocationResponse
/** @emoji 🕰️ Hybrid logical clock stamp carried by every kernel operation. */
export type HybridLogicalTimestamp = { readonly wall: number; readonly counter: number };

/** @emoji 🩹️ A schema-tagged document mutation payload (forward diff or inverse diff). */
export type DocumentDiff = { readonly schemaId: string; readonly payload: unknown };

/** @emoji ↩️ Undo semantics for a single kernel operation. */
export type UndoPolicy = "exactBaseOnly" | "transformAgainstConcurrent" | "semanticUndo" | "compensatingAction";

/** @emoji ↩️ The true inverse of a kernel operation, recorded from the store's `Edit.backwards`. */
export type InverseMutation = {
  readonly targetOperation: string;
  readonly inverseDiff: DocumentDiff;
  readonly baseVersion: number;
  readonly dependencies?: readonly string[];
  readonly undoPolicy: UndoPolicy;
};

/** @emoji 🔁️ One typed document operation with its true inverse — the CQRS wire unit. */
export type KernelMutation = {
  readonly id: string;
  readonly document: number;
  readonly baseVersion: number;
  readonly invocationId: string;
  readonly diff: DocumentDiff;
  readonly inverse: InverseMutation;
  readonly dependencies?: readonly string[];
  readonly author: string;
  readonly timestamp: HybridLogicalTimestamp;
};

/** @emoji 🎁️ The undo group binding an invocation (action or command) to its operations + inverses. */
export type UndoGroup = {
  readonly invocationId: string;
  readonly mutations: readonly string[];
  readonly inverseMutations: readonly InverseMutation[];
};

/** @emoji 📣️ An out-of-band app event surfaced to the shell (e.g. history changed). */
export type AppEvent = { readonly kind: string; readonly payload: unknown };

/** @emoji 🩺️ Canonical severity for faults and diagnostics. */
export type Severity = "fatal" | "error" | "warning" | "hint";

/** @emoji 🧭️ Layer that produced a fault. */
export type FaultOrigin = "edge" | "renderer" | "os" | "module" | "plugin" | "app" | "extension";

export type FaultScope = {
  readonly pluginId?: string;
  readonly appId?: string;
  readonly instanceId?: string;
  readonly module?: string;
  readonly bodyKey?: string;
};

export type FaultCause = { readonly message: string; readonly code?: string };

export type TextSpan = { readonly line: number; readonly column: number; readonly length: number };

/** @emoji 🧯️ Structured abort report shared across Rust, WIT, and TypeScript. */
export type Fault = {
  readonly origin: FaultOrigin;
  readonly code: string;
  readonly severity: Severity;
  readonly message: string;
  readonly scope: FaultScope;
  readonly span?: TextSpan;
  readonly causes?: readonly FaultCause[];
  readonly retryable: boolean;
};

/** @emoji 🩺️ A diagnostic emitted alongside an action result. */
export type Diagnostic = {
  readonly code: string;
  readonly severity: Severity;
  readonly message: string;
  readonly scope?: FaultScope;
  readonly span?: TextSpan;
};

/** @emoji 🧯️ Error subclass carrying a structured {@link Fault}. */
export class SemioFaultError extends Error {
  readonly fault: Fault;
  constructor(fault: Fault) {
    super(fault.message);
    this.name = "SemioFaultError";
    this.fault = fault;
  }
}

/**
 * @emoji 🐚️ A typed side effect the shell performs on the app's behalf. Mirrors the Rust
 * `HostEffect` enum (externally tagged: unit variants are the plain tag string, struct variants are
 * a single-key object keyed by the camelCase variant name).
 */
export type HostEffect =
  | "requestSync"
  | { readonly openWindow: { readonly kind: string; readonly params: unknown } }
  | { readonly closeWindow: { readonly window: number } }
  | { readonly notify: { readonly message: string } }
  | { readonly navigate: { readonly uri: string } }
  /** @emoji 📂️ Replaces the active app instance's document with a VCS envelope JSON — host-owned
   * counterpart of `loadAppDocument` for catalog/example studio opens. */
  | { readonly loadDocument: { readonly pack?: readonly number[]; readonly spr?: readonly number[]; readonly documentJson?: string } }
  | { readonly openExternalUrl: { readonly url: string } }
  | { readonly setPanel: { readonly panelJson: string } }
  | { readonly downloadMediaExport: { readonly filename: string; readonly mimeType: string; readonly data: string; readonly encoding?: string } }
  | { readonly iconRenderExport: { readonly items: readonly { readonly filename: string; readonly request: unknown }[] } }
  | { readonly requestFileOpen: { readonly accept: string; readonly readAs?: string; readonly importAction: string; readonly multiple?: boolean } }
  /** @emoji 🎞️ Asks the shell to decode a video (file picker, or `payload` bytes already in hand)
   * and re-dispatch `frameAction` once per sampled frame with `{payload: dataUrl(image/jpeg), name,
   * frameIndex, timestampMs, index, total, width, height, ...args}`, then `doneAction` once with
   * `{name, durationMs, frameCount, sampledCount, width, height, codec, ...args}`; if the host can't
   * decode it, `fallbackAction` fires once with `{payload: dataUrl(raw bytes), name, ...args}`. The
   * numeric hints (`sampleStride`/`maxFrames`/`maxLongEdgePx`/`fpsHint`) are 0 when the caller wants
   * the host default. */
  | {
      readonly requestMediaFrames: {
        readonly accept: string;
        readonly frameAction: string;
        readonly doneAction: string;
        readonly fallbackAction: string;
        readonly sampleStride?: number;
        readonly maxFrames?: number;
        readonly maxLongEdgePx?: number;
        readonly fpsHint?: number;
        readonly payload?: string;
        readonly args?: unknown;
      };
    }
  | { readonly spawnPluginInstance: { readonly pluginId: string; readonly appId: string; readonly osInstanceId?: string; readonly label?: string; readonly documentJson?: string } }
  | { readonly openPluginInstance: { readonly pluginId: string; readonly appId: string; readonly osInstanceId?: string } }
  | { readonly setActiveUtility: { readonly windowId: string; readonly utilityId: string } }
  /** 🛠️ Programmatically switches the host-owned active tool of the active mode — the effect form of
   * `setActiveTool`. Empty `toolId` deactivates the current tool. */
  | { readonly setActiveTool: { readonly toolId: string } }
  | { readonly openDialog: { readonly dialogId: string; readonly args?: Record<string, unknown> } }
  /** @emoji 🔁️ Re-dispatches `action` onto the same plugin instance after `delayMs` — lets a program
   * advance staged/progressive work over several ticks without blocking the host; the response's own
   * `requestedEffects` are fed back through `applyHostEffects` recursively. */
  | { readonly dispatchAction: { readonly action: string; readonly args?: unknown; readonly delayMs: number } }
  /** @emoji 🎯️ Patches world-3d selection chrome and document-tree `selectedIds` without a composite re-render. */
  | {
      readonly patchWorld3dChrome: {
        readonly selectionJson: string;
        readonly vorticesJson?: string;
        readonly documentSelectedIds: readonly string[];
        readonly documentHighlightedIds?: readonly string[];
      };
    }
  | { readonly clipboardWrite: { readonly fragment: unknown } }
  | { readonly replayShellCommand: { readonly actionId: string; readonly args?: unknown } }
  | {
      readonly invokeExtension: {
        readonly extensionId: string;
        readonly capability: string;
        readonly requestJson: string;
        readonly responseAction: string;
      };
    };

/**
 * @emoji 🐢️ Mirrors the Rust `UiDirtyScope` — which rendered UI sections an action actually
 * invalidates. Absent (`undefined`) on an `InvocationResponse` means the same as the Rust side's missing
 * field: treat as `{kind: "full"}` (see {@link resolveUiDirtyScope}) — every program that doesn't emit
 * this yet keeps today's whole-shell-refresh behavior.
 */
export type UiDirtyScope =
  | { readonly kind: "full" }
  | { readonly kind: "none" }
  | {
      readonly kind: "partial";
      readonly windowBodies?: readonly string[];
      readonly panelBodies?: readonly string[];
      readonly utilities?: boolean;
      readonly tools?: boolean;
      readonly engagements?: boolean;
      readonly measures?: boolean;
      readonly labels?: boolean;
    };

/** @emoji 🐢️ Normalizes a possibly-absent `UiDirtyScope` — missing (older program, or a response built without one) means `full`. */
export function resolveUiDirtyScope(scope: UiDirtyScope | undefined): UiDirtyScope {
  return scope ?? { kind: "full" };
}

/**
 * @emoji 📤️ Typed result of a plugin `handle-action`/`handle-command` call — mirrors the Rust
 * `InvocationResult`. Replaces the legacy `string[]` JSON-patch shape: operations are now typed
 * `KernelMutation`s with true inverses, and the shell applies `requestedEffects` through
 * `applyHostEffects` (WS-E).
 */
export type InvocationResponse = {
  readonly output: unknown;
  readonly mutations: readonly KernelMutation[];
  readonly inverseGroup: UndoGroup;
  readonly diagnostics?: readonly Diagnostic[];
  readonly requestedEffects?: readonly HostEffect[];
  readonly events?: readonly AppEvent[];
  readonly uiScope?: UiDirtyScope;
};

// 🐢️ `uiScope` deliberately left unset here (not `{kind: "none"}`) — `resolveUiDirtyScope` treats a
// missing scope as `full`, the safe default for the rare failure paths that return this constant
// (unparseable response, stub module missing `handleAction`/`handleCommand`).
const EMPTY_INVOCATION_RESPONSE: InvocationResponse = {
  output: null,
  mutations: [],
  inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] },
};

/** @emoji 📥️ Parses a raw program `handle-action`/`handle-command` response string into a typed {@link InvocationResponse}. */
export function parseInvocationResponse(raw: string): InvocationResponse {
  try {
    const parsed = JSON.parse(raw) as Partial<InvocationResponse> | null;
    if (parsed && typeof parsed === "object" && Array.isArray(parsed.mutations)) {
      return parsed as InvocationResponse;
    }
  } catch {
    // fall through to the empty response
  }
  return EMPTY_INVOCATION_RESPONSE;
}
//#endregion InvocationResponse

//#region SerializedPluginWasm
/** @emoji 🧾️ Flattens jco/component errors — message is often `[object Object] (see error.payload)` while the real text lives on `payload.val`. */
export function pluginErrorText(error: unknown): string {
  if (error instanceof Error) {
    const withPayload = error as Error & { payload?: unknown };
    const payload = withPayload.payload;
    if (payload && typeof payload === "object") {
      const record = payload as { val?: unknown; tag?: unknown; message?: unknown };
      if (typeof record.val === "string" && record.val.length > 0) {
        return `${withPayload.message} payload=${JSON.stringify(payload)}`;
      }
      if (typeof record.message === "string" && record.message.length > 0) {
        return `${withPayload.message} payload=${JSON.stringify(payload)}`;
      }
    }
    return withPayload.message;
  }
  if (error && typeof error === "object" && "payload" in error) {
    try {
      return JSON.stringify(error);
    } catch {
      return String(error);
    }
  }
  return String(error);
}

/** @emoji 🔒️ True when a plugin call hit the single-flight instance lock (or a poisoned guard after a trap). */
export function isPluginInstanceBusyError(error: unknown): boolean {
  const message = pluginErrorText(error);
  return message.includes("plugin instance busy") || message.includes("plugin busy");
}

/** @emoji 🔒️ Serializes wasm program entry points — the host keeps instances in one RefCell. */
export function withSerializedPluginWasmHandle(handle: PluginWasmHandle): PluginWasmHandle {
  let tail: Promise<void> = Promise.resolve();
  const runSerialized = <T>(fn: () => Promise<T>): Promise<T> => {
    const job = tail.then(async () => {
      for (let attempt = 0; attempt < 8; attempt += 1) {
        try {
          return await fn();
        } catch (error) {
          if (!isPluginInstanceBusyError(error)) throw error;
          await new Promise((resolve) => setTimeout(resolve, attempt + 1));
        }
      }
      return fn();
    });
    tail = job.then(
      () => undefined,
      () => undefined,
    );
    return job;
  };
  return {
    manifest: () => runSerialized(() => handle.manifest()),
    createApp: (appId) => runSerialized(() => handle.createApp(appId)),
    destroyApp: (instanceId) => runSerialized(() => handle.destroyApp(instanceId)),
    exchange: (instanceId, frames) => runSerialized(() => handle.exchange(instanceId, frames)),
    dispose: handle.dispose,
  };
}
//#endregion SerializedPluginWasm

//#region PluginWorkerClient
/** @emoji 🧵️ Message types the generated `🟨️plugin-worker.js` dispatches (framework/os/dev/script.ts `pluginWorkerSource`). */
type PluginWorkerMessageType = "init" | "manifest" | "createApp" | "destroy" | "exchange" | "error";

/** @emoji ⏱️ Logs only, never kills the worker — a plugin action owns in-flight, possibly undo-relevant
 * state, so abandoning it mid-call (the wgpu renderer's timeout+restart policy) would corrupt it. */
const PLUGIN_WORKER_UNRESPONSIVE_MS = 10000;

/** @emoji 🔌️ Derives the generic worker bootstrap script's URL from a plugin module URL — same directory,
 * `🟨️plugin-worker.js` instead of the plugin's own bridge filename. The bootstrap script itself never
 * needs cache-busting (it's plugin-version-agnostic; the *actual* module URL, `?v=`-busted or not, only
 * ever travels as the `init` request's `moduleUrl` payload — see `start()` below), so any `?query` or
 * `#hash` on `moduleUrl` (from `PluginSource.moduleUrl`'s hot-reload cache-busting) is stripped first —
 * otherwise the trailing `.js` no longer sits at the string's end and the replace silently no-ops,
 * pointing the worker at the plugin's own module instead of its bootstrap script. */
/** @emoji 🪶️ GUESTSLIM: the typst default font set (see `infinite_canvas`'s `render` feature doc),
 * static-served alongside every plugin's own output at `_vendor/guestslim-typst-fonts.bin`
 * (`📇️registry/📜️script.ts`'s `ensureGuestSlimTypstFontsAsset`). Fetched once and reused across every
 * plugin worker this tab spins up — the file itself never changes at runtime (pinned crate version). */
const GUESTSLIM_TYPST_DEFAULT_FONTS_ASSET_HANDLE = 1;
let guestSlimTypstFontsPromise: Promise<ArrayBuffer> | null = null;

/** @emoji 🛡️ Best-effort: most plugins never call `read-asset` at all, and the guest-side Rust already
 * degrades gracefully (empty font list → typst compile yields no glyphs → `BoardResolvedIcon::None`)
 * when no reader is registered — so a fetch hiccup here must never block a plugin worker from booting. */
async function guestSlimAssetsForModule(moduleUrl: string): Promise<ReadonlyArray<readonly [number, ArrayBuffer]>> {
  guestSlimTypstFontsPromise ??= (async () => {
    const vendorUrl = moduleUrl.split(/[?#]/)[0]!.replace(/\/[^/]+\/[^/]+\.js$/, "/_vendor/guestslim-typst-fonts.bin");
    const response = await fetch(vendorUrl);
    if (!response.ok) throw new Error(`GuestSlim typst fonts asset fetch failed: ${response.status} ${vendorUrl}`);
    return response.arrayBuffer();
  })();
  try {
    const buffer = await guestSlimTypstFontsPromise;
    return [[GUESTSLIM_TYPST_DEFAULT_FONTS_ASSET_HANDLE, buffer]];
  } catch (error) {
    console.warn("[DEBUG] GuestSlim typst fonts asset unavailable; affected plugins fall back to blank typst/emoji/text icons", error);
    guestSlimTypstFontsPromise = null;
    return [];
  }
}

export function pluginWorkerUrl(moduleUrl: string): string {
  const bare = moduleUrl.split(/[?#]/)[0]!;
  return bare.replace(/\/[^/]+\.js$/, "/🟨️plugin-worker.js");
}

/**
 * @emoji 🧵️ Runs a component-model plugin's WASM inside a Web Worker so `handleAction` — including
 * long-running precompute — never blocks the UI thread. Mirrors `framework/os/renderer/wgpu/js/🟦️boot.ts`'s
 * `PluginWorkerClient`, minus its 5s timeout+restart.
 */
class PluginWorkerClient {
  private worker: Worker | null = null;
  private readonly pending = new Map<string, { resolve: (value: Record<string, unknown>) => void; reject: (error: Error) => void; watchdog: number }>();
  onBackboneOutbound?: (uri: string, message: Uint8Array) => void;

  constructor(
    private readonly pluginId: string,
    private readonly moduleUrl: string,
  ) {}

  private clearPending(error: Error): void {
    for (const [requestId, entry] of this.pending) {
      window.clearTimeout(entry.watchdog);
      entry.reject(error);
      this.pending.delete(requestId);
    }
  }

  private attachWorker(worker: Worker): void {
    worker.onmessage = (event: MessageEvent) => {
      const message = event.data as {
        requestId?: string;
        type?: PluginWorkerMessageType | "backboneOutbound";
        uri?: string;
        message?: string;
      };
      if (message.type === "backboneOutbound" && message.uri && message.message != null) {
        const bytes = message.message instanceof Uint8Array ? message.message : new Uint8Array(message.message as ArrayBuffer);
        this.onBackboneOutbound?.(message.uri, bytes);
        return;
      }
      const requestId = message.requestId;
      if (!requestId) return;
      const entry = this.pending.get(requestId);
      if (!entry) return;
      window.clearTimeout(entry.watchdog);
      this.pending.delete(requestId);
      if (message.type === "error") {
        entry.reject(new Error(message.message ?? `program worker ${this.pluginId} error`));
        return;
      }
      entry.resolve(message);
    };
    worker.onerror = (error) => {
      console.error(`[DEBUG] program worker ${this.pluginId} crashed`, error);
      this.worker = null;
      this.clearPending(new Error(`program worker ${this.pluginId} crashed`));
    };
  }

  async start(): Promise<void> {
    const worker = new Worker(pluginWorkerUrl(this.moduleUrl), { type: "module" });
    this.attachWorker(worker);
    this.worker = worker;
    // 🪶️ GUESTSLIM: structured-clone copy, not a transfer — `guestSlimAssetsForModule` caches and
    // reuses the same master `ArrayBuffer` across every plugin worker this tab starts; transferring
    // it would detach (neuter) it after the first worker, breaking every subsequent one.
    const guestSlimAssets = await guestSlimAssetsForModule(this.moduleUrl);
    await this.request("init", { moduleUrl: this.moduleUrl, guestSlimAssets });
  }

  private request(type: PluginWorkerMessageType, payload: Record<string, unknown>): Promise<Record<string, unknown>> {
    return new Promise((resolve, reject) => {
      if (!this.worker) {
        reject(new Error(`program worker ${this.pluginId} is not running`));
        return;
      }
      const requestId = crypto.randomUUID();
      const watchdog = window.setTimeout(() => {
        console.warn(`[DEBUG] program worker ${this.pluginId} unresponsive for ${PLUGIN_WORKER_UNRESPONSIVE_MS}ms: ${type}`);
      }, PLUGIN_WORKER_UNRESPONSIVE_MS);
      this.pending.set(requestId, { resolve, reject, watchdog });
      this.worker.postMessage({ type, requestId, ...payload });
    });
  }

  async manifest(): Promise<Uint8Array> {
    return ((await this.request("manifest", {})).value as Uint8Array | undefined) ?? new Uint8Array();
  }

  async createApp(appId: string): Promise<number> {
    return Number((await this.request("createApp", { appId })).instanceId);
  }

  async destroyApp(instanceId: number): Promise<void> {
    await this.request("destroy", { instanceId });
  }

  async exchange(instanceId: number, frames: Uint8Array[]): Promise<Uint8Array[]> {
    return ((await this.request("exchange", { instanceId, frames })).value as Uint8Array[] | undefined) ?? [];
  }

  dispose(): void {
    this.clearPending(new Error(`program worker ${this.pluginId} disposed`));
    this.worker?.terminate();
    this.worker = null;
  }

  postBackboneInbound(uri: string, messages: readonly Uint8Array[]): void {
    this.worker?.postMessage({ type: "backboneInbound", uri, messages });
  }
}

/**
 * @emoji 🧵️ Worker-backed `PluginWasmHandle` for component-model plugins (the ABI the generated
 * `🟨️plugin-worker.js` supports). Caller falls back to the direct main-thread import on failure (no
 * `🟨️plugin-worker.js` alongside this module, wasm-bindgen-only program, or `Worker` unavailable).
 *
 * Keyed by `moduleUrl` (not `pluginId`): a hot reload acquires a *second* worker at a fresh
 * cache-busted URL for the same `pluginId` while the old one still serves live instances, so a
 * `pluginId`-keyed map would have the new worker's `set()` silently clobber the old entry and then
 * the old worker's `dispose()` delete the new one out from under it. `activeWorkerByPluginId` tracks
 * which of a plugin's (possibly several, during a swap) worker clients is the one inbound backbone
 * traffic should reach.
 */
const pluginWorkerClients = new Map<string, PluginWorkerClient>();
const activeWorkerByPluginId = new Map<string, PluginWorkerClient>();

async function loadPluginModuleViaWorker(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  const client = new PluginWorkerClient(pluginId, moduleUrl);
  pluginWorkerClients.set(moduleUrl, client);
  client.onBackboneOutbound = (uri, message) => relayPluginBackboneOutbound(uri, message);
  await client.start();
  activeWorkerByPluginId.set(pluginId, client);
  console.log(`[DEBUG] plugin worker + ${pluginId} (${pluginWorkerClients.size} live)`);
  return withSerializedPluginWasmHandle({
    manifest: () => client.manifest(),
    createApp: (appId) => client.createApp(appId),
    destroyApp: (instanceId) => client.destroyApp(instanceId),
    exchange: (instanceId, frames) => client.exchange(instanceId, frames),
    dispose: () => {
      if (pluginWorkerClients.get(moduleUrl) === client) pluginWorkerClients.delete(moduleUrl);
      if (activeWorkerByPluginId.get(pluginId) === client) activeWorkerByPluginId.delete(pluginId);
      client.dispose();
      console.log(`[DEBUG] plugin worker - ${pluginId} (${pluginWorkerClients.size} live)`);
    },
  });
}
//#endregion PluginWorkerClient

export function relayPluginBackboneOutbound(uri: string, message: Uint8Array): void {
  pluginBackboneRoutes.get(pluginBackboneDocumentIdFromUri(uri))?.(uri, message);
}

/** @emoji 🌉️ A direct-import (main-thread, no-worker) plugin's generated `🟨️host-shim.js` runs in this
 * same realm but can't import from this module, so it reaches the outbound relay through this
 * well-known global instead — the same relay a worker-backed program reaches via `postMessage`. */
(globalThis as unknown as { __semioMainThreadPluginBackboneOutbound?: (uri: string, message: Uint8Array) => void }).__semioMainThreadPluginBackboneOutbound = relayPluginBackboneOutbound;

/** @emoji 🌉️ Inbound counterpart: pushes straight into the same global queue a direct-import plugin's
 * `🟨️host-shim.js` `backbonePoll` drains, keyed by `uri` (globally unique per document, so no pluginId
 * scoping is needed even though several plugins may share this realm). */
function pushMainThreadPluginBackboneInbound(uri: string, messages: readonly Uint8Array[]): void {
  const bridge = globalThis as unknown as { __semioBackboneInbound?: Map<string, Uint8Array[]> };
  const queue = bridge.__semioBackboneInbound ?? new Map<string, Uint8Array[]>();
  queue.set(uri, [...(queue.get(uri) ?? []), ...messages]);
  bridge.__semioBackboneInbound = queue;
}

export function postPluginBackboneInbound(pluginId: string, uri: string, messages: readonly Uint8Array[]): void {
  const client = activeWorkerByPluginId.get(pluginId);
  if (client) {
    client.postBackboneInbound(uri, messages);
    return;
  }
  pushMainThreadPluginBackboneInbound(uri, messages);
}

//#region 🐚️PluginBackboneRouting
/** @emoji 🐚️ Extracts the `<documentId>` a plugin's `actor://<documentId>` backbone uri names — the
 * `framework/sync` `ChannelBackbone::pair` convention (see the react renderer's `openDocument`). Falls
 * back to the whole uri for any other scheme so an unrecognized realm still gets a routing key instead
 * of being silently dropped. */
function pluginBackboneDocumentIdFromUri(uri: string): string {
  return uri.startsWith("actor://") ? uri.slice("actor://".length) : uri;
}

const pluginBackboneRoutes = new Map<string, (uri: string, message: Uint8Array) => void>();

/**
 * @emoji 🐚️ Routes a plugin's outbound backbone bytes for one document to whichever shell instance owns
 * it — replaces the old page-global relay slot (`setPluginBackboneOutboundRelay`), which a second
 * mounted shell silently overwrote: misrouting the first shell's document sync into the second shell's
 * backbone worker, then severing it entirely the moment that second shell unmounted (it cleared the
 * slot to `null`). Register at the same point a shell learns it owns `documentId` (the react renderer's
 * `openDocument`) and call the returned unregister function at the matching `closeDocument`/unmount.
 */
export function registerPluginBackboneRoute(documentId: string, relay: (uri: string, message: Uint8Array) => void): () => void {
  pluginBackboneRoutes.set(documentId, relay);
  return () => {
    if (pluginBackboneRoutes.get(documentId) === relay) pluginBackboneRoutes.delete(documentId);
  };
}
//#endregion 🐚️PluginBackboneRouting

//#region 🪶️LeasePool
/** @emoji 🪶️ One caller's reference to a {@link LeasePool}-managed resource. `release()` is idempotent —
 * a second call is a no-op — and drops this caller's refcount; the pool only disposes the underlying
 * resource once every issued lease on that key has released (and, unless `lingerMs` is 0, only after
 * the linger window below elapses with no re-acquire). */
export interface Lease<T> {
  readonly value: T;
  release(): void;
}

export interface LeasePoolStats {
  readonly key: string;
  readonly refs: number;
  readonly state: "loading" | "resident" | "lingering";
}

export interface LeasePool<T> {
  acquire(key: string): Promise<Lease<T>>;
  /** Forces disposal of `key` (or every entry when omitted) right now, bypassing any linger timer.
   * A no-op (logged, not thrown) for a key with active leases — evicting a resource a caller still
   * holds would leave that caller's `Lease.value` silently dead underneath it. */
  evictNow(key?: string): void;
  stats(): readonly LeasePoolStats[];
}

type LeasePoolEntry<T> = {
  readonly promise: Promise<T>;
  refs: number;
  lingerTimer: ReturnType<typeof setTimeout> | null;
  settled: T | undefined;
};

/**
 * @emoji 🪶️ Generic refcounted resource pool with linger-based eviction — the shared mechanism both
 * {@link acquirePluginModule} (plugin worker modules) and the renderer's engine-session cache build on
 * top of, instead of each hand-rolling its own refcounting. A resource loads once per `key` and is
 * shared by every caller; when the last lease on a key releases, the resource isn't disposed
 * immediately — it lingers for `lingerMs` (default 30s) so a caller that re-acquires the same key
 * shortly after (e.g. reopening a just-closed window) reuses the still-live resource instead of paying
 * full reload cost. `lingerMs: 0` disposes the instant refs hit zero, matching the pre-`LeasePool`
 * `acquirePluginModule` behavior exactly.
 */
export function createLeasePool<T>(load: (key: string) => Promise<T>, dispose: (value: T) => void, options?: { readonly lingerMs?: number; readonly label?: string }): LeasePool<T> {
  const lingerMs = options?.lingerMs ?? 30_000;
  const label = options?.label ?? "resource";
  const entries = new Map<string, LeasePoolEntry<T>>();

  function disposeEntry(key: string, entry: LeasePoolEntry<T>): void {
    if (entries.get(key) !== entry) return;
    entries.delete(key);
    if (entry.settled !== undefined) {
      console.log(`[DEBUG] ${label} evicted ${key}`);
      dispose(entry.settled);
    }
  }

  return {
    async acquire(key: string): Promise<Lease<T>> {
      let entry = entries.get(key);
      if (!entry) {
        const created: LeasePoolEntry<T> = { promise: load(key), refs: 0, lingerTimer: null, settled: undefined };
        created.promise.then(
          (value) => {
            created.settled = value;
          },
          () => {
            if (entries.get(key) === created) entries.delete(key);
          },
        );
        entries.set(key, created);
        entry = created;
      }
      const active = entry;
      if (active.lingerTimer !== null) {
        clearTimeout(active.lingerTimer);
        active.lingerTimer = null;
      }
      active.refs += 1;
      try {
        const value = await active.promise;
        let released = false;
        return {
          value,
          release: () => {
            if (released) return;
            released = true;
            active.refs -= 1;
            if (active.refs > 0) return;
            if (lingerMs <= 0) {
              disposeEntry(key, active);
              return;
            }
            active.lingerTimer = setTimeout(() => disposeEntry(key, active), lingerMs);
          },
        };
      } catch (error) {
        active.refs -= 1;
        throw error;
      }
    },
    evictNow(key?: string): void {
      for (const [entryKey, entry] of key ? ([[key, entries.get(key)]] as const) : entries) {
        if (!entry) continue;
        if (entry.refs > 0) {
          console.warn(`[DEBUG] ${label} evictNow(${entryKey}) skipped — ${entry.refs} active lease(s)`);
          continue;
        }
        if (entry.lingerTimer !== null) clearTimeout(entry.lingerTimer);
        disposeEntry(entryKey, entry);
      }
    },
    stats(): readonly LeasePoolStats[] {
      return Array.from(entries.entries()).map(([key, entry]) => ({
        key,
        refs: entry.refs,
        state: entry.settled === undefined ? "loading" : entry.lingerTimer !== null ? "lingering" : "resident",
      }));
    },
  };
}
//#endregion 🪶️LeasePool

//#region 🐚️PluginModuleLease
export interface PluginModuleLease {
  readonly handle: PluginWasmHandle;
  /** Releases this caller's reference to the shared module — idempotent, a second call is a no-op.
   * The underlying worker/module disposes once every lease on this `moduleUrl` has released and the
   * pool's linger window (see {@link createLeasePool}) elapses with no re-acquire. */
  release(): void;
}

// 🐚️ The pool's `load` callback only receives the key (`moduleUrl` — already globally unique per
// plugin, matching the pre-pool cache's key exactly), but `loadPluginModuleUncached` also wants a
// human-readable `pluginId` for its worker/log labels. `acquirePluginModule` records that association
// here just before acquiring; safe as a plain overwrite since a given `moduleUrl` only ever maps to
// one `pluginId` in practice.
const pluginModuleIdByUrl = new Map<string, string>();
const pluginModulePool = createLeasePool<PluginWasmHandle>((moduleUrl) => loadPluginModuleUncached(pluginModuleIdByUrl.get(moduleUrl) ?? moduleUrl, moduleUrl), (handle) => handle.dispose(), { label: "plugin module" });

/**
 * @emoji 🐚️ Refcounted replacement for the old `loadPluginModule` — several shells (or several plugin
 * instances within one shell) loading the SAME `moduleUrl` share one worker/module, but each caller
 * gets its own {@link PluginModuleLease} and must `release()` it on unmount/teardown. Built on
 * {@link createLeasePool}: the shared module lingers briefly after the last lease releases (a shell
 * closed and immediately reopened reuses it) rather than disposing that instant — under the pre-pool
 * cache, a loaded module was in practice *never* disposed at all (its promise was cached forever with
 * nothing to evict it; `dispose()` was only ever reachable on load *failure*), so this is strictly a
 * bugfix on top of a lifecycle improvement.
 */
export async function acquirePluginModule(pluginId: string, moduleUrl: string): Promise<PluginModuleLease> {
  pluginModuleIdByUrl.set(moduleUrl, pluginId);
  const lease = await pluginModulePool.acquire(moduleUrl);
  return { handle: lease.value, release: lease.release };
}

/** @emoji 🔁️ Forces immediate disposal of a stale `moduleUrl` after a hot reload has released its last
 * lease — a no-op with a `[DEBUG]` warning (see {@link createLeasePool.evictNow}) if a caller still
 * holds the old lease, so a reload sequence must release before evicting. Skipping this after a
 * cache-busted reload would leave the old worker lingering for the pool's full 30s window per swap. */
export function evictPluginModule(moduleUrl: string): void {
  pluginModulePool.evictNow(moduleUrl);
}

/** @emoji 🔭️ Debug-only runtime snapshot — live plugin worker ids and the plugin module pool's lease
 * states — for verifying eager-boot-vs-lazy-residency changes from devtools without instrumenting call
 * sites by hand. Intentionally global rather than exported: this is a console/devtools aid, not API. */
(globalThis as unknown as { __semioPluginRuntimeStats?: () => unknown }).__semioPluginRuntimeStats = () => ({
  workerModuleUrls: Array.from(pluginWorkerClients.keys()),
  workerCount: pluginWorkerClients.size,
  activePluginIds: Array.from(activeWorkerByPluginId.keys()),
  modulePool: pluginModulePool.stats(),
});
//#endregion 🐚️PluginModuleLease

/**
 * 🌉️ Direct main-thread import fallback for {@link loadPluginModuleViaWorker} (no `Worker` global —
 * vitest/node — or no `🟨️plugin-worker.js` alongside this module). Only the component-model
 * `createPluginApi` ABI is supported: the pre-ABI-flip flat `semio_plugin_*` wasm-bindgen export
 * surface (one JS function per verb: `semio_plugin_handle_action`, `semio_plugin_render`, ...)
 * predates the binary `exchange` ABI entirely and has no equivalent under it, so it is dropped
 * rather than adapted — this is a greenfield codebase with no legacy-ABI support obligation.
 */
async function loadPluginModuleUncached(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  // 🧵️ Worker-backed by default so a plugin's `exchange` (e.g. puzzle-3d's collision precompute) can
  // never block the UI thread. Falls back to the direct main-thread import below when unavailable: no
  // `Worker` global (vitest/node) or no `🟨️plugin-worker.js` alongside this module.
  if (typeof Worker !== "undefined") {
    try {
      return await loadPluginModuleViaWorker(pluginId, moduleUrl);
    } catch (error) {
      console.warn(`[DEBUG] program ${pluginId} worker-backed load failed, falling back to main thread: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  const module = (await import(/* @vite-ignore */ moduleUrl)) as {
    default?: () => Promise<void> | void;
    createPluginApi?: () => Promise<{
      manifest: () => Promise<Uint8Array>;
      createApp: (appId: string) => Promise<number>;
      destroyApp?: (instanceId: number) => Promise<void>;
      exchange: (instanceId: number, frames: Uint8Array[]) => Promise<Uint8Array[]>;
    }>;
  };
  if (module.default) await module.default();
  if (!module.createPluginApi) {
    throw new Error(`[DEBUG] program ${pluginId} missing createPluginApi export`);
  }
  const api = await module.createPluginApi();
  return withSerializedPluginWasmHandle({
    manifest: () => api.manifest(),
    createApp: (appId) => api.createApp(appId),
    destroyApp: async (instanceId) => {
      await api.destroyApp?.(instanceId);
    },
    exchange: (instanceId, frames) => api.exchange(instanceId, frames),
    dispose() {},
  });
}

/** 🌉️ Adapts a {@link PluginWasmHandle} to a plain-object shape safe to close over across a
 * `postMessage`/global-bridge boundary (see the wgpu renderer's own program-worker embedding) — a
 * pass-through now that the whole ABI is already binary (`manifest`/`exchange` bytes cross
 * structured clone natively, same as `Uint8Array` payloads elsewhere on this bridge). */
export function pluginHandleForBridge(handle: PluginWasmHandle) {
  return {
    manifest: () => handle.manifest(),
    createApp: (appId: string) => handle.createApp(appId),
    destroyApp: (instanceId: number) => handle.destroyApp(instanceId),
    exchange: (instanceId: number, frames: Uint8Array[]) => handle.exchange(instanceId, frames),
  };
}
//#endregion PluginRuntime

//#region 🔌️PluginSource
/** @emoji 🔌️ Dev-server SSE endpoint a `PluginSource` availability stream connects to (see
 * {@link createDevPluginSource}) — mounted by the dev runner's `semioPluginHotSwapVitePlugin`
 * alongside the `/plugin-modules` static alias it watches. Shared here (rather than duplicated as a
 * literal in both the dev vite plugin and the shell) so the two ends can't drift apart. */
export const PLUGIN_SOURCE_WATCH_PATH = "/plugin-modules/watch";

/** @emoji 🔌️ One entry of an availability stream: either the full set of currently-built plugins sent
 * once on connect (a reconnecting/late-connecting browser must not miss builds that already finished),
 * or a single plugin's rebuild landing. `rebuiltAt` is the artifact's build timestamp and doubles as
 * the cache-busting query value {@link PluginSource.moduleUrl} mints. */
export type PluginSourceEvent = { readonly kind: "snapshot"; readonly plugins: readonly { readonly pluginId: string; readonly rebuiltAt: number }[] } | { readonly kind: "built"; readonly pluginId: string; readonly rebuiltAt: number };

/**
 * @emoji 🔌️ Where the shell's incremental plugin runtime (install/uninstall/reload — see the react
 * renderer's plugin panel) gets its catalog and availability notifications from. `createDevPluginSource`
 * is the only implementation today; a future `HubPluginSource` (fetching manifests and artifacts from
 * the plugin hub over HTTP/SSE instead of the local dev server) implements the same three methods and
 * needs no changes anywhere else — the shell only ever depends on this interface.
 */
export interface PluginSource {
  readonly id: string;
  /** Every plugin this source can currently install (built or not — the panel shows "available"
   * entries that haven't finished their first build yet). */
  list(): Promise<readonly PluginRegistryEntry[]>;
  /** Mints a concrete, cache-busted module URL for one install/reload of `pluginId`. Omitting
   * `rebuiltAt` (initial install, before any `built` event) falls back to the registry's own
   * `moduleUrl`, unbusted — correct for a first load, where there is nothing stale to bust. */
  moduleUrl(pluginId: string, rebuiltAt?: number): string;
  /** Subscribes to availability events; returns an unsubscribe function. Fires an immediate `snapshot`
   * on subscribe against sources that support it (the dev source's SSE endpoint always sends one). */
  subscribe(listener: (event: PluginSourceEvent) => void): () => void;
}

/** @emoji 🔌️ `PluginSource` backed by the dev server's static `/plugin-modules` output and its
 * {@link PLUGIN_SOURCE_WATCH_PATH} SSE stream. `EventSource` is unavailable under vitest/node, so
 * `subscribe` there is a harmless no-op (matches every other browser-only feature detection in this
 * module, e.g. {@link loadPluginModuleUncached}'s `Worker` check). */
export function createDevPluginSource(registry: readonly PluginRegistryEntry[]): PluginSource {
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry] as const));
  return {
    id: "dev",
    async list() {
      return registry;
    },
    moduleUrl(pluginId, rebuiltAt) {
      const entry = byId.get(pluginId);
      if (!entry) throw new Error(`[DEBUG] plugin source "dev" has no registry entry for ${pluginId}`);
      return rebuiltAt === undefined ? entry.moduleUrl : `${entry.moduleUrl}?v=${rebuiltAt}`;
    },
    subscribe(listener) {
      if (typeof EventSource === "undefined") return () => {};
      const source = new EventSource(PLUGIN_SOURCE_WATCH_PATH);
      source.onmessage = (event) => {
        try {
          listener(JSON.parse(event.data) as PluginSourceEvent);
        } catch (error) {
          console.warn(`[DEBUG] plugin source "dev" malformed event: ${error instanceof Error ? error.message : String(error)}`);
        }
      };
      return () => source.close();
    },
  };
}

/** @emoji 🧩️ Dev-server SSE endpoint for {@link createExtensionSource} — paired with the `/extensions`
 * static route the extension store materializes at install time. */
export const EXTENSION_SOURCE_WATCH_PATH = "/extensions/watch";

/** @emoji 🧩️ `PluginSource` backed by the extension store's `/extensions` HTTP tree and its watch SSE
 * stream. Catalog rows come from generated {@link EXTENSION_TARGETS}; runtime installs add artifacts
 * under each extension id without changing this list. */
export function createExtensionSource(): PluginSource {
  const registry: readonly PluginRegistryEntry[] = EXTENSION_TARGETS.map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: extensionModuleUrl(target.pluginId, target.wasmOut),
    contributes: target.contributes,
    consumes: target.consumes,
  }));
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry] as const));
  return {
    id: "extensions",
    async list() {
      return registry;
    },
    moduleUrl(pluginId, rebuiltAt) {
      const entry = byId.get(pluginId);
      if (!entry) throw new Error(`[DEBUG] plugin source "extensions" has no registry entry for ${pluginId}`);
      return rebuiltAt === undefined ? entry.moduleUrl : `${entry.moduleUrl}?v=${rebuiltAt}`;
    },
    subscribe(listener) {
      if (typeof EventSource === "undefined") return () => {};
      const source = new EventSource(EXTENSION_SOURCE_WATCH_PATH);
      source.onmessage = (event) => {
        try {
          listener(JSON.parse(event.data) as PluginSourceEvent);
        } catch (error) {
          console.warn(`[DEBUG] plugin source "extensions" malformed event: ${error instanceof Error ? error.message : String(error)}`);
        }
      };
      return () => source.close();
    },
  };
}

/** @emoji 🔌️ Merges multiple {@link PluginSource} implementations — dev `/plugin-modules` plus extension
 * `/extensions` — into one catalog the shell's incremental runtime can treat as a single source. */
export function multiplexPluginSources(...sources: readonly PluginSource[]): PluginSource {
  if (sources.length === 0) throw new Error("[DEBUG] multiplexPluginSources requires at least one source");
  if (sources.length === 1) return sources[0];
  return {
    id: sources.map((source) => source.id).join("+"),
    async list() {
      const merged = new Map<string, PluginRegistryEntry>();
      for (const entries of await Promise.all(sources.map((source) => source.list()))) {
        for (const entry of entries) merged.set(entry.pluginId, entry);
      }
      return [...merged.values()];
    },
    moduleUrl(pluginId, rebuiltAt) {
      for (const source of sources) {
        try {
          return source.moduleUrl(pluginId, rebuiltAt);
        } catch {
          continue;
        }
      }
      throw new Error(`[DEBUG] multiplexed plugin sources have no registry entry for ${pluginId}`);
    },
    subscribe(listener) {
      const unsubscribes = sources.map((source) => source.subscribe(listener));
      return () => {
        for (const unsubscribe of unsubscribes) unsubscribe();
      };
    },
  };
}
//#endregion 🔌️PluginSource

// #region 🎮️PlaygroundResolution
/** @emoji 🎮️ Finds the generated playground catalog row for a variant id or one of its aliases. */
function findPlaygroundVariant(playgroundPluginId: string): PlaygroundBuildTarget | undefined {
  return PLAYGROUND_BUILD_TARGETS.find((entry) => entry.variant === playgroundPluginId || entry.aliases.includes(playgroundPluginId));
}

/** @emoji 🎯️ Resolves a playground filter/alias (e.g. "3d", "sourcing") to its underlying wasm component registry id. */
export function resolvePluginRegistryId(playgroundPluginId: string): string {
  return findPlaygroundVariant(playgroundPluginId)?.pluginId ?? playgroundPluginId;
}

/** @emoji 🎯️ Resolves a playground filter/alias to the app id that should be instantiated by default within its plugin's manifest. */
export function resolvePlaygroundDefaultAppId(playgroundPluginId: string): string | undefined {
  return findPlaygroundVariant(playgroundPluginId)?.app;
}

export type PlaygroundBootSession = {
  readonly variant: string;
  readonly defaultAppId?: string;
  readonly plugins: readonly PluginRegistryEntry[];
};

export type PlaygroundBoot = {
  readonly variant: string;
  readonly defaultAppId?: string;
  readonly plugins: readonly PluginRegistryEntry[];
};

/** @emoji 🎮️ Resolves the wasm plugin list and default app for one playground variant; when the on-disk
 * `generated/🟦️session.ts` was overwritten by another concurrent dev variant, rebuilds from the generated
 * program catalog instead of trusting the stale program rows. */
export function resolvePlaygroundBoot(variant: string, session?: PlaygroundBootSession): PlaygroundBoot {
  const defaultAppId = resolvePlaygroundDefaultAppId(variant);
  if (session?.variant === variant) {
    return { variant, defaultAppId: session.defaultAppId ?? defaultAppId, plugins: session.plugins };
  }
  const registryPluginId = resolvePluginRegistryId(variant);
  const studioMode = resolvePluginHostConfig(variant) !== undefined;
  const catalogPlugins: PluginRegistryEntry[] = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: target.role === "extension" ? extensionModuleUrl(target.pluginId, target.wasmOut) : pluginModuleUrl(target.pluginId, target.wasmOut),
    contributes: target.contributes,
    consumes: target.consumes,
  }));
  return {
    variant,
    defaultAppId,
    plugins: expandPluginRegistry(catalogPlugins, studioMode ? undefined : registryPluginId, studioMode),
  };
}

//#region 🏠️🧳️PluginHostConfig
/** 🏠️🧳️ Declares, for a plugin whose manifest offers a host-style multi-app experience (one app is the
 * landing/default view, another hosts other apps as spawned sub-instances — e.g. "s"'s home/studio
 * pair), which app ids play which role. Callers resolve controller ids and default panel tabs from
 * the *loaded manifest*'s own `controllerId`/`panelTabs` on those apps rather than hardcoding separate
 * literals — this table only ever needs to carry app-id role assignments. A pluginFilter absent here
 * simply boots through the ordinary single-app path (`resolvePlaygroundDefaultAppId`). Mirrored by
 * `PLUGIN_HOST_CONFIGS`/`resolve_plugin_host_config` in `framework/os/renderer/wgpu/rs/lib.rs`'s
 * `program_bridge` module for the WGPU renderer. */
export type PluginHostConfig = {
  readonly pluginId: string;
  readonly landingAppId: string;
  readonly hostAppId: string;
};

/** 🎯️ Resolves a playground filter/alias to its plugin's host config, or `undefined` when that program doesn't offer a host-style multi-app experience. */
export function resolvePluginHostConfig(playgroundPluginId: string): PluginHostConfig | undefined {
  const registryId = resolvePluginRegistryId(playgroundPluginId);
  return PLUGIN_HOST_CONFIGS.find((entry) => entry.pluginId === registryId);
}
//#endregion 🏠️🧳️PluginHostConfig
// #endregion 🎮️PlaygroundResolution
// #endregion 🎠️Kernel
