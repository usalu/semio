//#region 🧲️PlatformBoot
/** @emoji 🧵️ Browser UI isolate host for the dedicated frame Worker. */

import { BrowserFrameTransport, browserFrameEventFromDom, browserFrameEventIsReplaceable, browserFramePointerDomEvent, browserFrameWheelDomEvent, type BrowserFrameDomEvent, type BrowserFrameFallbackState, type BrowserFrameIntrospectionProbe, type BrowserFrameWorkerFaultCode, type BrowserHubDocumentRemote } from "../🚚️browser-frame-transport/🟦️.ts";
import { createWgpuPageHostIo } from "../🚪️host-io/🟦️.ts";
import { setInteractiveJobPort } from "../../../../../../../../🔨️modules/🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs/🟦️.ts";
import { TURN_DIAGNOSTICS_KEY, setTurnDiagnostics } from "../⏱️turn-budget/🟦️.ts";
import { stampShardWorkerDiagnostics } from "../../../../../../../../🔨️modules/🎭️actor/🩺️diagnostics/🟦️.ts";
import { describeBrowserBootPhase } from "../🫀️boot-liveness/🟦️.ts";
import { WGPU_PREFERS_DARK_MEDIA_QUERY, WGPU_READINESS_BEACON_UNKNOWN_PLUGIN, documentBootMetaReader, readWgpuHostStorageSnapshot, resolveWgpuBootDescriptor, resolveWgpuHostAppearance, resolveWgpuHostPlatform, stripBootBrokerProof, wgpuReadinessBeacon, type WgpuBootDescriptor, type WgpuHostAppearance, type WgpuHostPlatform, type WgpuHostStorageSnapshot } from "../🧭️boot-descriptor/🟦️.ts";
import { DEFAULT_HOST_VARIANT } from "../../../../../🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import { createAccessibilityMirror } from "../♿️accessibility-mirror/🟦️.ts";
import { browserClipboardPasteCandidate, wireBrowserFullscreen, wireBrowserKeyboard } from "../🎮️input-wire/🟦️.ts";

/** 🚏️ Resolves completed renderer artifacts and the generated frame worker through the browser host. */
const RENDERER_MODULE_URL = new URL("../renderer-modules/wgpu/semio-framework-os-renderer-wgpu.js", import.meta.url).href;
const RENDERER_WASM_URL = new URL("../renderer-modules/wgpu/semio-framework-os-renderer-wgpu_bg.wasm", import.meta.url).href;
const FRAME_WORKER_URL = new URL("../🎞️frame-worker.js/🟨️.js", import.meta.url);

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

/** @emoji 🌓️ The page realm's appearance reads — `prefers-color-scheme` plus the persisted
 * `os.config.ui-preferences` appearance. The frame Worker owns neither `window` nor `localStorage`,
 * so these are made HERE and forwarded; see `../🧭️boot-descriptor/🟦️.ts`'s 🌓️HostAppearance region. */
function hostAppearance(): WgpuHostAppearance {
  return resolveWgpuHostAppearance(window);
}

/** @emoji ⌨️ The page realm's platform read — `userAgentData.platform`, else `navigator.platform`.
 * The frame Worker's own `cfg!(target_os = "macos")` is false in every wasm build, so without this a
 * macOS browser formatted `mod` as `Ctrl`; see `../🧭️boot-descriptor/🟦️.ts`'s ⌨️HostPlatform region. */
function hostPlatform(): WgpuHostPlatform {
  return resolveWgpuHostPlatform(window);
}

/** @emoji 🗄️ The page realm's read of every durable preference key the shell owns. It lives HERE for
 * the same reason both appearance reads do: the frame Worker owns no `localStorage`, so its whole
 * preference store — appearance, locale, terminology, themes, keybinding overrides, the compute worker
 * count, the dock skeleton and `ui.introduction.seen.*` — was invisible and unwritable on the browser
 * build. See `../🧭️boot-descriptor/🟦️.ts`'s 🗄️HostStorage census. */
function hostStorage(): WgpuHostStorageSnapshot {
  return readWgpuHostStorageSnapshot(window);
}

/** @emoji 🧭️ The page's own boot axes: `?plugin=&app=&role=&mode=&example=&hub=&user=&dataDir=` over
 * the per-server `<meta name="semio-*">` seeds, plus the one-shot `#semio-broker=` proof — the SAME
 * vocabulary `../🎬️renderer-boot/🟦️.ts` and `../⌨️native-entrypoint/🦀️.rs` build, resolved by the ONE
 * shared resolver so the three doors cannot drift. The proof is read and then removed from the address
 * bar, exactly as React does (`🏛️ShellHost/🟦️.tsx:209-214`). */
function bootDescriptor(): WgpuBootDescriptor {
  const descriptor = resolveWgpuBootDescriptor({ search: window.location.search, hash: window.location.hash, meta: documentBootMetaReader(document), defaultVariant: DEFAULT_HOST_VARIANT });
  stripBootBrokerProof(window.location, window.history);
  return descriptor;
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
export const WGPU_HUB_PROJECTION_GLOBAL = "semioWgpuHubProjection";

type WgpuIntrospection = { readonly dumpStructure: (windowId?: string) => Promise<string>; readonly dumpFrameStats: (windowId?: string) => Promise<string>; readonly dumpAccessibility: (windowId?: string) => Promise<string>; readonly dumpMeshStats: (windowId?: string) => Promise<string>; readonly dumpChrome: (windowId?: string) => Promise<string> };
type WgpuHubProjection = { readonly publishDocumentStatus: (documentKey: string, remote: BrowserHubDocumentRemote | null) => boolean };

function attachIntrospectionBindings(transport: BrowserFrameTransport): () => void {
  const probe = (kind: BrowserFrameIntrospectionProbe) => async (windowId?: string) => (await transport.introspect(kind, windowId)) ?? "";
  const host = window as unknown as { semioWgpuIntrospection?: WgpuIntrospection; semioWgpuHubProjection?: WgpuHubProjection };
  host.semioWgpuIntrospection = { dumpStructure: probe("structure"), dumpFrameStats: probe("frame-stats"), dumpAccessibility: probe("accessibility"), dumpMeshStats: probe("mesh-stats"), dumpChrome: probe("chrome") };
  host.semioWgpuHubProjection = { publishDocumentStatus: (documentKey, remote) => transport.publishHubDocumentStatus(documentKey, remote) };
  return () => {
    delete host.semioWgpuIntrospection;
    delete host.semioWgpuHubProjection;
  };
}

export { WGPU_ACCESSIBILITY_MIRROR_ID } from "../♿️accessibility-mirror/🟦️.ts";


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
  // 🔊️ A dead surface must also be READABLE. The banner is the only place a fault appeared, and a
  // console is what a probe, a CI run and a headless browser can read — so a quarantined renderer used
  // to look exactly like a frame loop that simply stopped, with no error of any kind
  // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-frame-loop-after-selection-2026-09-13.md`).
  console.error(`wgpu renderer fault: ${code}: ${detail}`);
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
function wireInput(root: HTMLElement, canvas: HTMLCanvasElement, transport: BrowserFrameTransport): () => void {
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
  canvas.addEventListener("pointermove", (event) => admit(browserFramePointerDomEvent(event, "pointermove"), performance.now()), options);
  canvas.addEventListener("pointerdown", (event) => {
    const startedAt = performance.now();
    canvas.focus({ preventScroll: true });
    canvas.setPointerCapture(event.pointerId);
    admit(browserFramePointerDomEvent(event, "pointerdown"), startedAt);
  }, options);
  canvas.addEventListener("pointerup", (event) => admit(browserFramePointerDomEvent(event, "pointerup"), performance.now()), options);
  canvas.addEventListener("pointercancel", (event) => admit(browserFramePointerDomEvent(event, "pointercancel"), performance.now()), options);
  canvas.addEventListener("wheel", (event) => {
    const startedAt = performance.now();
    event.preventDefault();
    admit(browserFrameWheelDomEvent(event), startedAt);
  }, { ...options, passive: false });
  const cleanupKeyboard = wireBrowserKeyboard(root, canvas, event => admit(event, performance.now()));
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
    const candidate = browserClipboardPasteCandidate(event.clipboardData?.items);
    if (candidate) {
      if (candidate.kind === "image") {
        event.preventDefault();
        const reader = new FileReader();
        reader.addEventListener("load", () => {
          if (typeof reader.result !== "string") return;
          const handoffStartedAt = performance.now();
          transport.enqueueLossless({ kind: "paste-image-data-url", text: reader.result });
          transport.observeUiTurn("paste-image-handoff", performance.now() - handoffStartedAt);
        }, { once: true });
        reader.readAsDataURL(candidate.file);
        observed("paste", startedAt);
        return;
      }
      event.preventDefault();
      candidate.item.getAsString((text) => {
        const handoffStartedAt = performance.now();
        transport.enqueueLossless({ kind: "paste", text });
        transport.observeUiTurn("paste-handoff", performance.now() - handoffStartedAt);
      });
    }
    observed("paste", startedAt);
  }, options);
  return () => {
    cleanupKeyboard();
    abort.abort();
  };
}
//#endregion 🎮️PlatformInput

//#region 🧵️WorkerLifecycle
async function mount(root: HTMLElement): Promise<void> {
  armUiTurnDiagnostics();
  const descriptor = bootDescriptor();
  const beacon = wgpuReadinessBeacon(document.documentElement, descriptor.pluginVariant);
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
    worker = new Worker(stampShardWorkerDiagnostics(FRAME_WORKER_URL.href), { type: "module", name: "semio-frame-worker" });
  } catch (error) {
    throw new Error(`worker-construction-failed: ${error instanceof Error ? error.message : String(error)}`);
  }
  let cleanupInput = () => {};
  const fullscreenOwner = wireBrowserFullscreen(root, canvas);
  let detachIntrospection = () => {};
  let accessibility: { readonly refresh: () => void; readonly dispose: () => void } | undefined;
  const transport = new BrowserFrameTransport({
    worker,
    boot: { bindingsModuleUrl: RENDERER_MODULE_URL, bindingsWasmUrl: RENDERER_WASM_URL, canvas: offscreen, width, height, dpr, locale: locale(), descriptor, appearance: hostAppearance(), platform: hostPlatform(), storage: hostStorage() },
    requestAnimationFrame: (callback) => window.requestAnimationFrame(callback),
    cancelAnimationFrame: (handle) => window.cancelAnimationFrame(handle),
    // 🚪️ The PAGE half of the shell's file door. The frame Worker owns the whole shell but has no
    // `document`, so `Export Document…`'s `<a download>` and `Import Document…`'s `<input type="file">`
    // live here (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    hostIo: createWgpuPageHostIo(),
    onProgress: (stage, progress, worker) => {
      status.textContent = `${stage} ${Math.round(progress * 100)}%${worker.degraded ? (locale() === "de" ? " · verzögerte Taktung" : " · deferred cadence") : ""}`;
      status.dataset.workerDegraded = worker.degraded ? "true" : "false";
      status.dataset.workerStepOverruns = String(worker.recordedOverruns);
    },
    onUiTurn: (outcome) => {
      canvas.dataset.uiTurn = `${outcome.verdict}:${outcome.site}:${outcome.executingMs.toFixed(3)}`;
    },
    onReady: () => {
      beacon.ready();
      status.remove();
      detachIntrospection = attachIntrospectionBindings(transport);
      accessibility = createAccessibilityMirror(root, transport, locale(), canvas);
      accessibility.refresh();
      cleanupInput = wireInput(root, canvas, transport);
      transport.enqueueReplaceable(browserFrameEventFromDom({ type: "resize", clientWidth: canvas.clientWidth, clientHeight: canvas.clientHeight }, dpr) as Extract<ReturnType<typeof browserFrameEventFromDom>, { kind: "resize" }>);
      canvas.focus({ preventScroll: true });
    },
    // 🖼️ The FRAME channel — raised for every frame the Worker produced and this isolate accepted.
    // 🩸️ The ARIA mirror used to refresh from `onUiTurn`, which the transport raises ONLY for a turn
    // that breached its budget ceiling (`observeUiTurn`: `verdict !== "admitted"`). A healthy shell
    // overruns once, at boot, so the mirror was painted exactly once — from a pull issued before the
    // guest had published any document — and never again: `nodeCount 0` on 6118 where the lane that
    // built it had measured 34, with no source change in between, and every later document (a locale
    // switch, an example switch, a selection) invisible to a reader even when the boot race happened
    // to win (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    onDirectives: ({ cursor, fullscreen }) => {
      canvas.style.cursor = cursor;
      if (typeof fullscreen === "boolean") void fullscreenOwner.set(fullscreen).catch(() => {});
      accessibility?.refresh();
    },
    onFault: (code: BrowserFrameWorkerFaultCode, detail, fallback) => {
      beacon.error();
      cleanupInput();
      fullscreenOwner.dispose();
      detachIntrospection();
      accessibility?.dispose();
      renderFault(root, code, detail, fallback);
    },
  });
  const previousInteractiveJobPort = setInteractiveJobPort(transport.interactiveJobs);
  const publishMetrics = (site: string) => {
    const startedAt = performance.now();
    const nextDpr = window.devicePixelRatio || 1;
    transport.enqueueReplaceable(browserFrameEventFromDom({ type: "resize", clientWidth: canvas.clientWidth, clientHeight: canvas.clientHeight }, nextDpr) as Extract<ReturnType<typeof browserFrameEventFromDom>, { kind: "resize" }>);
    transport.observeUiTurn(site, performance.now() - startedAt);
  };
  const resize = new ResizeObserver(() => publishMetrics("resize-observer"));
  resize.observe(canvas);
  // 📐️ A ResizeObserver never fires when only the DENSITY changes — the window dragged onto a display
  // with a different scale factor, or a browser zoom step — yet the physical surface extent, the
  // projection divisor and the glyph atlas raster all move with it. `(resolution: Ndppx)` matches the
  // CURRENT ratio, so the query has to be re-armed against the new one each time it stops matching.
  // Ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g.
  let resolutionQuery: MediaQueryList | undefined;
  const armResolutionQuery = () => {
    resolutionQuery?.removeEventListener("change", onResolutionChange);
    resolutionQuery = window.matchMedia?.(`(resolution: ${window.devicePixelRatio || 1}dppx)`);
    resolutionQuery?.addEventListener("change", onResolutionChange);
  };
  function onResolutionChange() {
    publishMetrics("resolution-change");
    armResolutionQuery();
  }
  armResolutionQuery();
  // 🌓️ React re-applies every `appearance: "system"` root from ONE shared `matchMedia` `change`
  // listener (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`'s `ensureElementsSurfaceChromeSystemListeners`) and
  // re-reads the persisted preference from the `storage` event (`🎚️UiPreferences/🟦️.ts`'s
  // `installBrowserStorageListener`). Both listeners live here for the same reason both reads do:
  // the Worker realm has neither.
  const republishAppearance = () => transport.setHostAppearance(hostAppearance());
  // 🗄️ A `storage` event fires only for OTHER documents on this origin, so it is exactly the cross-tab
  // case: re-read the whole census and re-seed the Worker's cache. This isolate's own writes never
  // arrive this way — they went out through the door and are already in both halves.
  const republishHostStorage = () => transport.setHostStorage(hostStorage());
  const darkQuery = window.matchMedia?.(WGPU_PREFERS_DARK_MEDIA_QUERY);
  darkQuery?.addEventListener("change", republishAppearance);
  window.addEventListener("storage", republishAppearance);
  window.addEventListener("storage", republishHostStorage);
  window.addEventListener("pagehide", () => {
    beacon.clear();
    resize.disconnect();
    darkQuery?.removeEventListener("change", republishAppearance);
    window.removeEventListener("storage", republishAppearance);
    window.removeEventListener("storage", republishHostStorage);
    resolutionQuery?.removeEventListener("change", onResolutionChange);
    cleanupInput();
    fullscreenOwner.dispose();
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
  wgpuReadinessBeacon(document.documentElement, WGPU_READINESS_BEACON_UNKNOWN_PLUGIN).error();
  renderFault(root, "worker-boot-failed", detail);
  throw error;
}
//#endregion 🧵️WorkerLifecycle
