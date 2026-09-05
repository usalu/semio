//#region 🧲️PlatformBoot
/** @emoji 🧵️ Browser UI isolate host for the dedicated frame Worker. */

import { BrowserFrameTransport, type BrowserFramePointer, type BrowserFrameWorkerFaultCode } from "../🚚️browser-frame-transport/🟦️.ts";
import { setInteractiveJobPort } from "../../../../../../../../🔨️modules/🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs.ts";

const RENDERER_MODULE_URL = new URL("./semio-framework-os-renderer-wgpu.js", import.meta.url).href;
const RENDERER_WASM_URL = new URL("./semio-framework-os-renderer-wgpu_bg.wasm", import.meta.url).href;
const FRAME_WORKER_URL = new URL("./🎞️frame-worker.js", import.meta.url);
const BOOT_FIELD_CAPACITY = 2048;
const LOCATION_SEARCH_CAPACITY = 8192;

await new Promise<void>((resolve) => {
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", () => resolve(), { once: true });
  else resolve();
});

function locale(): "en" | "de" {
  return navigator.language.toLowerCase().startsWith("de") ? "de" : "en";
}

function bounded(value: string, field: string): string {
  if (value.length > BOOT_FIELD_CAPACITY) throw new Error(`boot-descriptor-overflow: ${field} exceeds ${BOOT_FIELD_CAPACITY} code units`);
  return value;
}

function bootDescriptor(): { pluginVariant: string; appRole: string; hub?: { hubUrl: string; user: string; dataDir: string } } {
  if (window.location.search.length > LOCATION_SEARCH_CAPACITY) throw new Error(`boot-descriptor-overflow: location.search exceeds ${LOCATION_SEARCH_CAPACITY} code units`);
  const params = new URLSearchParams(window.location.search);
  const hubUrl = params.get("hub");
  return {
    pluginVariant: bounded(params.get("plugin") ?? "s", "plugin"),
    appRole: params.get("role") === "viewer" ? "viewer" : "editor",
    ...(hubUrl ? { hub: { hubUrl: bounded(hubUrl, "hub"), user: bounded(params.get("user") ?? "", "user"), dataDir: bounded(params.get("dataDir") ?? "", "dataDir") } } : {}),
  };
}

function canvasElement(): HTMLCanvasElement {
  const canvas = document.createElement("canvas");
  canvas.tabIndex = 0;
  canvas.setAttribute("aria-label", locale() === "de" ? "Semio Arbeitsfläche" : "Semio workspace");
  canvas.style.cssText = "display:block;width:100%;height:100%;touch-action:none;outline:none;";
  return canvas;
}

function statusElement(root: HTMLElement): HTMLElement {
  const status = document.createElement("div");
  status.setAttribute("role", "status");
  status.setAttribute("aria-live", "polite");
  status.style.cssText = "position:fixed;left:12px;bottom:12px;padding:6px 9px;background:#001117cc;color:#d7f7ff;font:12px monospace;z-index:9997;";
  root.appendChild(status);
  return status;
}

function renderFault(root: HTMLElement, code: string, detail: string): void {
  const banner = document.createElement("div");
  banner.setAttribute("role", "alert");
  banner.style.cssText = "position:fixed;inset:0;padding:24px;background:#2a0a0acc;color:#ffb4b4;font:14px monospace;white-space:pre-wrap;overflow:auto;z-index:9999;";
  banner.textContent = `wgpu renderer fault:\n\n${code}: ${detail}\n\nNo UI-thread frame fallback was attempted.`;
  root.appendChild(banner);
}
//#endregion 🧲️PlatformBoot

//#region 🎮️PlatformInput
function wireInput(canvas: HTMLCanvasElement, transport: BrowserFrameTransport): () => void {
  const abort = new AbortController();
  const options = { signal: abort.signal };
  const pointer = (event: PointerEvent): BrowserFramePointer => ({
    pointerId: event.pointerId,
    pointerKind: event.pointerType === "touch" || event.pointerType === "pen" ? event.pointerType : "mouse",
    x: event.offsetX * window.devicePixelRatio,
    y: event.offsetY * window.devicePixelRatio,
    pressure: event.pressure || undefined,
    tiltX: event.tiltX || undefined,
    tiltY: event.tiltY || undefined,
  });
  const observed = (site: string, startedAt: number) => void transport.observeUiTurn(site, performance.now() - startedAt);
  canvas.addEventListener("pointermove", (event) => {
    const startedAt = performance.now();
    transport.enqueueReplaceable({ kind: "pointer-move", ...pointer(event) });
    observed("pointer-move", startedAt);
  }, options);
  canvas.addEventListener("pointerdown", (event) => {
    const startedAt = performance.now();
    canvas.focus({ preventScroll: true });
    canvas.setPointerCapture(event.pointerId);
    transport.enqueueLossless({ kind: "pointer-down", ...pointer(event), button: event.button === 2 ? "secondary" : event.button === 1 ? "middle" : "primary" });
    observed("pointer-down", startedAt);
  }, options);
  canvas.addEventListener("pointerup", (event) => {
    const startedAt = performance.now();
    transport.enqueueLossless({ kind: "pointer-up", ...pointer(event), button: event.button === 2 ? "secondary" : event.button === 1 ? "middle" : "primary" });
    observed("pointer-up", startedAt);
  }, options);
  canvas.addEventListener("wheel", (event) => {
    const startedAt = performance.now();
    event.preventDefault();
    transport.enqueueReplaceable({ kind: "wheel", x: event.offsetX * window.devicePixelRatio, y: event.offsetY * window.devicePixelRatio, deltaX: event.deltaX, deltaY: event.deltaY });
    observed("wheel", startedAt);
  }, { ...options, passive: false });
  const key = (event: KeyboardEvent, kind: "key-down" | "key-up") => {
    const startedAt = performance.now();
    transport.enqueueLossless({ kind, key: event.key, shift: event.shiftKey, ctrl: event.ctrlKey, alt: event.altKey, meta: event.metaKey });
    observed(kind, startedAt);
  };
  canvas.addEventListener("keydown", (event) => void key(event, "key-down"), options);
  canvas.addEventListener("keyup", (event) => void key(event, "key-up"), options);
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
  const transport = new BrowserFrameTransport({
    worker,
    boot: { bindingsModuleUrl: RENDERER_MODULE_URL, bindingsWasmUrl: RENDERER_WASM_URL, canvas: offscreen, width, height, dpr, pluginVariant: descriptor.pluginVariant, locale: locale(), appRole: descriptor.appRole, hub: descriptor.hub },
    requestAnimationFrame: (callback) => window.requestAnimationFrame(callback),
    cancelAnimationFrame: (handle) => window.cancelAnimationFrame(handle),
    onProgress: (stage, progress) => { status.textContent = `${stage} ${Math.round(progress * 100)}%`; },
    onReady: () => {
      status.remove();
      cleanupInput = wireInput(canvas, transport);
      transport.enqueueReplaceable({ kind: "resize", width, height, dpr });
      canvas.focus({ preventScroll: true });
    },
    onDirectives: ({ cursor, fullscreen }) => {
      canvas.style.cursor = cursor;
      if (fullscreen === true) void canvas.requestFullscreen().catch(() => {});
      if (fullscreen === false && document.fullscreenElement) void document.exitFullscreen().catch(() => {});
    },
    onFault: (code: BrowserFrameWorkerFaultCode, detail) => {
      cleanupInput();
      renderFault(root, code, detail);
    },
  });
  const previousInteractiveJobPort = setInteractiveJobPort(transport.interactiveJobs);
  const resize = new ResizeObserver(() => {
    const startedAt = performance.now();
    const nextDpr = window.devicePixelRatio || 1;
    transport.enqueueReplaceable({ kind: "resize", width: Math.max(1, Math.round(canvas.clientWidth * nextDpr)), height: Math.max(1, Math.round(canvas.clientHeight * nextDpr)), dpr: nextDpr });
    transport.observeUiTurn("resize-observer", performance.now() - startedAt);
  });
  resize.observe(canvas);
  window.addEventListener("pagehide", () => {
    resize.disconnect();
    cleanupInput();
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
