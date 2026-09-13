//#region 🧲️PlatformBoot
/** @emoji 🧵️ Browser UI isolate host for the dedicated frame Worker. */

import { BrowserFrameTransport, browserFrameEventFromDom, browserFrameEventIsReplaceable, type BrowserFrameDomEvent, type BrowserFrameFallbackState, type BrowserFrameIntrospectionProbe, type BrowserFrameWorkerFaultCode } from "../🚚️browser-frame-transport/🟦️.ts";
import { setInteractiveJobPort } from "../../../../../../../../🔨️modules/🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs/🟦️.ts";
import { TURN_DIAGNOSTICS_KEY, setTurnDiagnostics } from "../⏱️turn-budget/🟦️.ts";
import { describeBrowserBootPhase } from "../🫀️boot-liveness/🟦️.ts";
import { DEFAULT_HOST_VARIANT } from "../../../../../🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";

/** 🚏️ Resolves completed renderer artifacts and the generated frame worker through the browser host. */
const RENDERER_MODULE_URL = new URL("../renderer-modules/wgpu/semio-framework-os-renderer-wgpu.js", import.meta.url).href;
const RENDERER_WASM_URL = new URL("../renderer-modules/wgpu/semio-framework-os-renderer-wgpu_bg.wasm", import.meta.url).href;
const FRAME_WORKER_URL = new URL("../🎞️frame-worker.js/🟨️.js", import.meta.url);
const BOOT_FIELD_CAPACITY = 2048;
const LOCATION_SEARCH_CAPACITY = 8192;

await new Promise<void>((resolve) => {
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", () => resolve(), { once: true });
  else resolve();
});

/** @emoji 🩺️ Resolves a stored `SEMIO_RUNTIME_DIAGNOSTICS` preference and hands it to the UI-turn
 * ledger. The read lives HERE, in the UI isolate, and not in `../⏱️turn-budget/🟦️.ts`: that module is
 * also bundled into `🎞️frame-worker.js`, whose carrier census forbids credential-bearing storage. Wrapped
 * because a sandboxed page throws on `localStorage`; an absent value leaves the build-time switch to
 * decide. */
function armUiTurnDiagnostics(): void {
  try {
    const stored = globalThis.localStorage?.getItem(TURN_DIAGNOSTICS_KEY);
    if (stored !== null && stored !== undefined) setTurnDiagnostics(["1", "true", "on", "yes"].includes(stored.trim().toLowerCase()));
  } catch {
    setTurnDiagnostics(undefined);
  }
}

function locale(): "en" | "de" {
  return navigator.language.toLowerCase().startsWith("de") ? "de" : "en";
}

function bounded(value: string, field: string): string {
  if (value.length > BOOT_FIELD_CAPACITY) throw new Error(`boot-descriptor-overflow: ${field} exceeds ${BOOT_FIELD_CAPACITY} code units`);
  return value;
}

function bootDescriptor(): { pluginVariant: string; appRole: string; appMode: string; appExample: string; hub?: { hubUrl: string; user: string; dataDir: string } } {
  if (window.location.search.length > LOCATION_SEARCH_CAPACITY) throw new Error(`boot-descriptor-overflow: location.search exceeds ${LOCATION_SEARCH_CAPACITY} code units`);
  const params = new URLSearchParams(window.location.search);
  const hubUrl = params.get("hub");
  return {
    pluginVariant: bounded(params.get("plugin") ?? document.querySelector<HTMLMetaElement>('meta[name="semio-plugin"]')?.content ?? DEFAULT_HOST_VARIANT, "plugin"),
    appRole: params.get("role") === "viewer" ? "viewer" : "editor",
    appMode: bounded(params.get("mode") ?? "", "mode"),
    appExample: bounded(params.get("example") ?? "", "example"),
    ...(hubUrl ? { hub: { hubUrl: bounded(hubUrl, "hub"), user: bounded(params.get("user") ?? "", "user"), dataDir: bounded(params.get("dataDir") ?? "", "dataDir") } } : {}),
  };
}

/** @emoji 🪪️ The trunk shell is single-mount by construction (`#root`, one transferred `OffscreenCanvas`),
 * so its canvas carries the fixed `#semio-wgpu-canvas` identity `🌐️.html`'s own stylesheet and the
 * parity harness's wgpu boot gate (`🧑‍💻dev/…/📜️script.ts` `triageParityBoot`) both address it by. The
 * multi-mount library path (`../🎬️renderer-boot/🟦️.ts`'s `bootFrameworkOsWgpu`) deliberately stays
 * id-less — several independently-rooted mounts coexist on one page there. */
export const WGPU_CANVAS_ID = "semio-wgpu-canvas";

function canvasElement(): HTMLCanvasElement {
  const canvas = document.createElement("canvas");
  canvas.id = WGPU_CANVAS_ID;
  canvas.tabIndex = 0;
  canvas.setAttribute("aria-label", locale() === "de" ? "Semio Arbeitsfläche" : "Semio workspace");
  canvas.style.cssText = "display:block;width:100%;height:100%;touch-action:none;outline:none;";
  return canvas;
}

/** @emoji 🔬️ `window.semioWgpuIntrospection.dumpStructure()`/`dumpFrameStats()` — the readiness beacon and
 * the structural oracle the parity harness (`🧑‍💻dev/…/📜️script.ts` `triageParityBoot`/`dumpWgpuStructure`)
 * addresses. Deliberately NOT `window.wasmBindings`: Trunk publishes its own UI-thread instantiation of the
 * renderer under that name (see `🌐️.html`), and that instance never boots, so its `dumpStructure` traps on
 * an uninitialised `UI_ENGINE`. The real exports live in `semio-frame-worker` beside the thread-local they
 * read, so this is an async shim over the transport's introspection pair. It is attached only once the
 * Worker reports `booted`, which is what makes waiting for the function a truthful boot gate; each call
 * answers `""` rather than throwing when the dump is unavailable, so a probe reads an empty dump instead of
 * a page error. */
export const WGPU_INTROSPECTION_GLOBAL = "semioWgpuIntrospection";

type WgpuIntrospection = { readonly dumpStructure: (windowId?: string) => Promise<string>; readonly dumpFrameStats: (windowId?: string) => Promise<string>; readonly dumpAccessibility: (windowId?: string) => Promise<string> };

function attachIntrospectionBindings(transport: BrowserFrameTransport): () => void {
  const probe = (kind: BrowserFrameIntrospectionProbe) => async (windowId?: string) => (await transport.introspect(kind, windowId)) ?? "";
  const host = window as unknown as { semioWgpuIntrospection?: WgpuIntrospection };
  host.semioWgpuIntrospection = { dumpStructure: probe("structure"), dumpFrameStats: probe("frame-stats"), dumpAccessibility: probe("accessibility") };
  return () => delete host.semioWgpuIntrospection;
}

//#region ♿️AccessibilityMirror
/** @emoji ♿️ The id of the ARIA subtree this host maintains beside the canvas. Fixed, so a probe (and
 * a screen-reader user's own tooling) can address it. */
export const WGPU_ACCESSIBILITY_MIRROR_ID = "semio-wgpu-accessibility";

/** ♿️ How long the mirror waits after one refresh before answering another, however much input
 * arrives in between. The projection crosses the Worker seam, so refreshing per input turn would put
 * a message round-trip on every pointer move; coalescing to this floor keeps it event-driven (nothing
 * ticks while nothing happens) without paying per event. */
const ACCESSIBILITY_REFRESH_FLOOR_MS = 400;

/** ♿️ One node of the renderer's published accessibility tree — `ui_contract`'s own
 * `AccessibilityProjectionNode` wire shape (`🖱️ui/🧬️contract/♿️accessibility/🦀️.rs`), not a
 * host-private one, so a second renderer publishing the same tree needs no new mirror. */
type AccessibilityProjectionNode = {
  readonly nodeId: number;
  readonly key: string;
  readonly role: string;
  readonly depth: number;
  readonly label?: string;
  readonly description?: string;
  readonly live: string;
  readonly shortcut?: string;
  readonly hidden?: boolean;
  readonly disabled?: boolean;
  readonly focusable?: boolean;
  readonly actionable?: boolean;
  readonly focused?: boolean;
};

/**
 * @emoji ♿️ The ARIA subtree that gives a GPU canvas an accessibility tree at all.
 *
 * React's Interpreter writes `aria-label`/`aria-describedby`/`aria-live`/`aria-keyshortcuts` straight
 * onto the element it renders per `UiNodeRecord`. A wgpu canvas renders no elements, so the renderer
 * publishes the same information as data (`dumpAccessibility`, the `accessibility` introspection
 * probe) and THIS is where it gets elements again.
 *
 * Visually hidden by the standard clip rule, never `display:none`/`visibility:hidden` — those two
 * remove a subtree from the accessibility tree as well as from the page, which would defeat the whole
 * point. Every mirrored element carries `tabindex="-1"`: keyboard focus belongs to the canvas, which
 * owns the renderer's own focus ring and Tab traversal, so the mirror must be readable without ever
 * stealing a Tab stop from it. `data-*` attributes carry the raw projection fields so a probe can
 * read the tree without parsing ARIA.
 *
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap #3 of `📓️audit-wgpu-parity-2026-09-13.md`.
 */
function accessibilityMirror(root: HTMLElement, transport: BrowserFrameTransport): { readonly refresh: () => void; readonly dispose: () => void } {
  const mirror = document.createElement("div");
  mirror.id = WGPU_ACCESSIBILITY_MIRROR_ID;
  mirror.setAttribute("role", "region");
  mirror.setAttribute("aria-label", locale() === "de" ? "Semio Bedienelemente" : "Semio controls");
  mirror.style.cssText = "position:absolute;width:1px;height:1px;margin:-1px;padding:0;overflow:hidden;clip:rect(0 0 0 0);clip-path:inset(50%);white-space:nowrap;border:0;";
  root.appendChild(mirror);
  let published = "";
  let lastAt = 0;
  let pending = false;
  let disposed = false;

  const paint = (nodes: readonly AccessibilityProjectionNode[]): void => {
    const elements = nodes.map((node) => {
      const element = document.createElement("div");
      element.setAttribute("role", node.role);
      element.tabIndex = -1;
      element.dataset.nodeId = String(node.nodeId);
      element.dataset.nodeKey = node.key;
      element.dataset.depth = String(node.depth);
      if (node.label !== undefined) element.setAttribute("aria-label", node.label);
      if (node.live !== "off") element.setAttribute("aria-live", node.live);
      if (node.shortcut !== undefined) element.setAttribute("aria-keyshortcuts", node.shortcut);
      if (node.hidden === true) element.setAttribute("aria-hidden", "true");
      if (node.disabled === true) element.setAttribute("aria-disabled", "true");
      if (node.focused === true) element.dataset.focused = "true";
      if (node.focusable === true) element.dataset.focusable = "true";
      if (node.actionable === true) element.dataset.actionable = "true";
      if (node.description !== undefined) {
        const description = document.createElement("span");
        description.id = `${WGPU_ACCESSIBILITY_MIRROR_ID}-${node.nodeId}-desc`;
        description.textContent = node.description;
        element.setAttribute("aria-describedby", description.id);
        element.appendChild(description);
      }
      return element;
    });
    mirror.replaceChildren(...elements);
    mirror.dataset.nodeCount = String(elements.length);
  };

  const pull = async (): Promise<void> => {
    lastAt = performance.now();
    const json = await transport.introspect("accessibility");
    if (disposed || json === null || json === published) return;
    published = json;
    let dump: { readonly nodes?: readonly AccessibilityProjectionNode[] };
    try {
      dump = JSON.parse(json) as { readonly nodes?: readonly AccessibilityProjectionNode[] };
    } catch {
      return;
    }
    paint(dump.nodes ?? []);
  };

  /** ♿️ Asks for a refresh, coalesced onto the floor above — an input burst produces one pull, not one per event. */
  const refresh = (): void => {
    if (disposed || pending) return;
    const waited = performance.now() - lastAt;
    if (waited >= ACCESSIBILITY_REFRESH_FLOOR_MS) {
      void pull();
      return;
    }
    pending = true;
    window.setTimeout(() => {
      pending = false;
      if (!disposed) void pull();
    }, ACCESSIBILITY_REFRESH_FLOOR_MS - waited);
  };

  return {
    refresh,
    dispose: () => {
      disposed = true;
      mirror.remove();
    },
  };
}
//#endregion ♿️AccessibilityMirror

function statusElement(root: HTMLElement): HTMLElement {
  const status = document.createElement("div");
  status.setAttribute("role", "status");
  status.setAttribute("aria-live", "polite");
  status.style.cssText = "position:fixed;left:12px;bottom:12px;padding:6px 9px;background:#001117cc;color:#d7f7ff;font:12px monospace;z-index:9997;";
  root.appendChild(status);
  return status;
}

/** @emoji 🪂️ The REAL fallback state in words. The banner used to assert a static
 * "No UI-thread frame fallback was attempted", which was both untrue as a claim about intent and
 * useless as a diagnosis: once `transferControlToOffscreen()` succeeds there IS no UI-thread frame path
 * to attempt, and what the reader needs instead is what the surface actually did — who owns the canvas
 * now, whether input is still admitted, and what the UI-turn ledger measured. */
function fallbackLines(state: BrowserFrameFallbackState | undefined, tongue: "en" | "de"): string {
  if (!state) {
    return tongue === "de"
      ? "Oberfläche: vor der Übergabe der Zeichenfläche an den Worker gescheitert. Kein Frame-Pfad war je aktiv."
      : "Surface: failed before the canvas reached the Worker. No frame path was ever live.";
  }
  const turns = state.uiTurns;
  const steps = state.workerSteps;
  const ledger = `${turns.recordedOverruns}/${turns.sustainedOverruns} (p99 ${turns.p99Ms.toFixed(3)} ms, worst ${turns.worstExecutingMs.toFixed(3)} ms @ ${turns.worstSite || "—"})`;
  const workerLedger = `${steps.recordedOverruns}/${steps.sustainedOverruns} (worst ${steps.worstStepMs.toFixed(3)} ms @ ${steps.worstStepSite || "—"})`;
  const phaseLine = describeBrowserBootPhase(state.bootPhase, state.bootPhaseElapsedMs, tongue);
  if (tongue === "de") {
    return [
      `Oberfläche: ${state.surface}${state.deferredCadence ? " · verzögerte Taktung" : ""}`,
      `Boot-Stufe: ${state.bootStage || "—"} · still seit ${Math.round(state.bootSilentForMs)} ms`,
      phaseLine,
      `UI-Thread-Frames: nicht verfügbar — die Zeichenfläche gehört dem Frame-Worker (OffscreenCanvas übergeben)`,
      `Worker beendet: ${state.workerTerminated ? "ja" : "nein"} · Eingaben angenommen: ${state.inputAccepted ? "ja" : "nein"}`,
      `UI-Takte über dem Budget (erfasst/anhaltend): ${ledger}`,
      `Worker-Schritte über dem Budget (erfasst/anhaltend): ${workerLedger}${steps.degraded ? " · verzögerte Taktung" : ""}`,
    ].join("\n");
  }
  return [
    `Surface: ${state.surface}${state.deferredCadence ? " · deferred cadence" : ""}`,
    `Boot stage: ${state.bootStage || "—"} · silent for ${Math.round(state.bootSilentForMs)} ms`,
    phaseLine,
    `UI-thread frames: unavailable — the canvas belongs to the frame Worker (OffscreenCanvas transferred)`,
    `Worker terminated: ${state.workerTerminated ? "yes" : "no"} · input accepted: ${state.inputAccepted ? "yes" : "no"}`,
    `UI turns over budget (recorded/sustained): ${ledger}`,
    `Worker steps over budget (recorded/sustained): ${workerLedger}${steps.degraded ? " · deferred cadence" : ""}`,
  ].join("\n");
}

function renderFault(root: HTMLElement, code: string, detail: string, state?: BrowserFrameFallbackState): void {
  const banner = document.createElement("div");
  banner.setAttribute("role", "alert");
  banner.style.cssText = "position:fixed;inset:0;padding:24px;background:#2a0a0acc;color:#ffb4b4;font:14px monospace;white-space:pre-wrap;overflow:auto;z-index:9999;";
  const tongue = locale();
  const title = tongue === "de" ? "wgpu-Renderer-Fehler" : "wgpu renderer fault";
  banner.textContent = `${title}:\n\n${code}: ${detail}\n\n${fallbackLines(state, tongue)}`;
  root.appendChild(banner);
}
//#endregion 🧲️PlatformBoot

//#region 🎮️PlatformInput
function wireInput(canvas: HTMLCanvasElement, transport: BrowserFrameTransport): () => void {
  const abort = new AbortController();
  const options = { signal: abort.signal };
  const observed = (site: string, startedAt: number) => void transport.observeUiTurn(site, performance.now() - startedAt);
  /** 🎮️ Hands ONE observed DOM input to the transport through the shared projection, on the lane that
   * projection names. Every input listener below is therefore focus/capture plus this call. */
  const admit = (dom: BrowserFrameDomEvent, startedAt: number): void => {
    const event = browserFrameEventFromDom(dom, window.devicePixelRatio || 1);
    if (browserFrameEventIsReplaceable(event)) transport.enqueueReplaceable(event);
    else transport.enqueueLossless(event);
    observed(dom.type, startedAt);
  };
  const pointer = (event: PointerEvent, type: "pointermove" | "pointerdown" | "pointerup"): BrowserFrameDomEvent => ({
    type,
    pointerId: event.pointerId,
    pointerType: event.pointerType,
    offsetX: event.offsetX,
    offsetY: event.offsetY,
    pressure: event.pressure,
    tiltX: event.tiltX,
    tiltY: event.tiltY,
    button: event.button,
  });
  canvas.addEventListener("pointermove", (event) => admit(pointer(event, "pointermove"), performance.now()), options);
  canvas.addEventListener("pointerdown", (event) => {
    const startedAt = performance.now();
    canvas.focus({ preventScroll: true });
    canvas.setPointerCapture(event.pointerId);
    admit(pointer(event, "pointerdown"), startedAt);
  }, options);
  canvas.addEventListener("pointerup", (event) => admit(pointer(event, "pointerup"), performance.now()), options);
  canvas.addEventListener("wheel", (event) => {
    const startedAt = performance.now();
    event.preventDefault();
    admit({ type: "wheel", offsetX: event.offsetX, offsetY: event.offsetY, deltaX: event.deltaX, deltaY: event.deltaY }, startedAt);
  }, { ...options, passive: false });
  const key = (event: KeyboardEvent, type: "keydown" | "keyup") => admit({ type, key: event.key, shift: event.shiftKey, ctrl: event.ctrlKey, alt: event.altKey, meta: event.metaKey }, performance.now());
  canvas.addEventListener("keydown", (event) => void key(event, "keydown"), options);
  canvas.addEventListener("keyup", (event) => void key(event, "keyup"), options);
  canvas.addEventListener("compositionstart", () => {
    const startedAt = performance.now();
    transport.enqueueLossless({ kind: "ime-start" });
    observed("ime-start", startedAt);
  }, options);
  canvas.addEventListener("compositionupdate", (event) => {
    const startedAt = performance.now();
    transport.enqueueLossless({ kind: "ime-update", text: event.data, cursor: event.data.length });
    observed("ime-update", startedAt);
  }, options);
  canvas.addEventListener("compositionend", (event) => {
    const startedAt = performance.now();
    transport.enqueueLossless({ kind: "ime-commit", text: event.data });
    observed("ime-commit", startedAt);
  }, options);
  canvas.addEventListener("paste", (event) => {
    const startedAt = performance.now();
    const items = event.clipboardData?.items;
    if (items) {
      const count = Math.min(items.length, 16);
      for (let index = 0; index < count; index++) {
        const item = items[index];
        if (item?.kind !== "string" || item.type !== "text/plain") continue;
        item.getAsString((text) => {
          const handoffStartedAt = performance.now();
          transport.enqueueLossless({ kind: "paste", text });
          transport.observeUiTurn("paste-handoff", performance.now() - handoffStartedAt);
        });
        break;
      }
    }
    observed("paste", startedAt);
  }, options);
  return () => abort.abort();
}
//#endregion 🎮️PlatformInput

//#region 🧵️WorkerLifecycle
async function mount(root: HTMLElement): Promise<void> {
  armUiTurnDiagnostics();
  const descriptor = bootDescriptor();
  if (typeof Worker === "undefined") throw new Error("worker-unavailable: Dedicated Worker is not supported");
  const canvas = canvasElement();
  if (typeof canvas.transferControlToOffscreen !== "function") throw new Error("offscreen-canvas-unavailable: OffscreenCanvas transfer is not supported");
  root.replaceChildren(canvas);
  const status = statusElement(root);
  const dpr = window.devicePixelRatio || 1;
  const width = Math.max(1, Math.round(canvas.clientWidth * dpr));
  const height = Math.max(1, Math.round(canvas.clientHeight * dpr));
  canvas.width = width;
  canvas.height = height;
  let offscreen: OffscreenCanvas;
  try {
    offscreen = canvas.transferControlToOffscreen();
  } catch (error) {
    throw new Error(`offscreen-transfer-failed: ${error instanceof Error ? error.message : String(error)}`);
  }
  let worker: Worker;
  try {
    worker = new Worker(FRAME_WORKER_URL, { type: "module", name: "semio-frame-worker" });
  } catch (error) {
    throw new Error(`worker-construction-failed: ${error instanceof Error ? error.message : String(error)}`);
  }
  let cleanupInput = () => {};
  let detachIntrospection = () => {};
  let accessibility: { readonly refresh: () => void; readonly dispose: () => void } | undefined;
  const transport = new BrowserFrameTransport({
    worker,
    boot: { bindingsModuleUrl: RENDERER_MODULE_URL, bindingsWasmUrl: RENDERER_WASM_URL, canvas: offscreen, width, height, dpr, pluginVariant: descriptor.pluginVariant, locale: locale(), appRole: descriptor.appRole, appMode: descriptor.appMode, appExample: descriptor.appExample, hub: descriptor.hub },
    requestAnimationFrame: (callback) => window.requestAnimationFrame(callback),
    cancelAnimationFrame: (handle) => window.cancelAnimationFrame(handle),
    onProgress: (stage, progress, worker) => {
      status.textContent = `${stage} ${Math.round(progress * 100)}%${worker.degraded ? (locale() === "de" ? " · verzögerte Taktung" : " · deferred cadence") : ""}`;
      status.dataset.workerDegraded = worker.degraded ? "true" : "false";
      status.dataset.workerStepOverruns = String(worker.recordedOverruns);
    },
    onUiTurn: (outcome) => {
      canvas.dataset.uiTurn = `${outcome.verdict}:${outcome.site}:${outcome.executingMs.toFixed(3)}`;
      accessibility?.refresh();
    },
    onReady: () => {
      status.remove();
      detachIntrospection = attachIntrospectionBindings(transport);
      accessibility = accessibilityMirror(root, transport);
      accessibility.refresh();
      cleanupInput = wireInput(canvas, transport);
      transport.enqueueReplaceable(browserFrameEventFromDom({ type: "resize", clientWidth: canvas.clientWidth, clientHeight: canvas.clientHeight }, dpr) as Extract<ReturnType<typeof browserFrameEventFromDom>, { kind: "resize" }>);
      canvas.focus({ preventScroll: true });
    },
    onDirectives: ({ cursor, fullscreen }) => {
      canvas.style.cursor = cursor;
      if (fullscreen === true) void canvas.requestFullscreen().catch(() => {});
      if (fullscreen === false && document.fullscreenElement) void document.exitFullscreen().catch(() => {});
    },
    onFault: (code: BrowserFrameWorkerFaultCode, detail, fallback) => {
      cleanupInput();
      detachIntrospection();
      accessibility?.dispose();
      renderFault(root, code, detail, fallback);
    },
  });
  const previousInteractiveJobPort = setInteractiveJobPort(transport.interactiveJobs);
  const resize = new ResizeObserver(() => {
    const startedAt = performance.now();
    const nextDpr = window.devicePixelRatio || 1;
    transport.enqueueReplaceable(browserFrameEventFromDom({ type: "resize", clientWidth: canvas.clientWidth, clientHeight: canvas.clientHeight }, nextDpr) as Extract<ReturnType<typeof browserFrameEventFromDom>, { kind: "resize" }>);
    transport.observeUiTurn("resize-observer", performance.now() - startedAt);
  });
  resize.observe(canvas);
  window.addEventListener("pagehide", () => {
    resize.disconnect();
    cleanupInput();
    detachIntrospection();
    accessibility?.dispose();
    setInteractiveJobPort(previousInteractiveJobPort);
    transport.close();
  }, { once: true });
}

const root = document.getElementById("root");
if (!root) throw new Error("missing-root: #root is unavailable");
try {
  await mount(root);
} catch (error) {
  const detail = error instanceof Error ? error.message : String(error);
  renderFault(root, "worker-boot-failed", detail);
  throw error;
}
//#endregion 🧵️WorkerLifecycle
