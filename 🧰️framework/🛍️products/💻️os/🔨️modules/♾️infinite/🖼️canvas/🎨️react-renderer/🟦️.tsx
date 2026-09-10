// #region 🧲️Header
/** @emoji 🖼️ `@semio-tech/infinite-canvas-react-renderer` — React host for tile-based infinite canvases (WASM bridge supplied by leaf bundles). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ContextMenuController, decodeIcon, encodeIcon, resolveIconUrlsInBoardJson, reactHostPort, type ContextMenuItem, type Icon, type IconSelectorMode } from "@semio-tech/ui-react";
import React from "react";
// #endregion 🔌️Adapters

export {
  ContextMenuController,
  decodeIcon,
  encodeIcon,
  resolveIconUrlsInBoardJson,
  reactHostPort,
  type ContextMenuItem,
  type Icon,
  type IconSelectorMode,
  React,
};

// #region 🔖️EventBinding
export type CanvasListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

/** @emoji 🎧️ Tracks DOM listeners for deterministic teardown on canvas unmount. */
export class CanvasEventBindingController {
  private readonly cleanups: Array<() => void> = [];

  listen(target: CanvasListenerTarget | null | undefined, kind: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions): void {
    if (!target) return;
    target.addEventListener(kind, listener, options);
    this.cleanups.push(() => target.removeEventListener(kind, listener, options));
  }

  dispose(): void {
    while (this.cleanups.length > 0) {
      this.cleanups.pop()?.();
    }
  }
}
// #endregion 🔖️EventBinding

// #region 🔖️CanvasWasmBridge
/** @emoji 🌐️ Leaf bundles (e.g. puzzle/2d) implement this against their `cdylib` session type. */
export interface CanvasWasmBridge<Session> {
  ensureLoaded(): Promise<void>;
  createSession(): Session;
}
// #endregion 🔖️CanvasWasmBridge

// #region 🔖️GraphWasmCanvas
/** @emoji ⌨️ Modifier keys held during a pointer gesture (shift/ctrl/meta/alt). */
export type CanvasInputModifiers = {
  readonly shift: boolean;
  readonly ctrl: boolean;
  readonly meta: boolean;
  readonly alt: boolean;
};

/** @emoji ⏳️ How long {@link GraphWasmCanvas} waits for a container that never reports a real layout
 * size before attaching the GPU surface at whatever degenerate size it does report. */
export const DEGENERATE_LAYOUT_ATTACH_MS = 2000;

/** @emoji 🙈️ Frame period used while no animation clock is running. */
export const HIDDEN_DEMAND_FRAME_MS = 32;

/** @emoji 🎞️ Schedules one canvas frame. A hidden/background tab — and any DOM host without an
 * animation clock, e.g. a jsdom test — never runs a frame callback, so an `requestAnimationFrame`-only
 * loop silently stops repainting there; that is the class of defect a blank node-graph window in a
 * hidden tab reduces to. Falls back to the timer clock, which keeps ticking while hidden. Shared by
 * every wasm surface loop so the fallback lives in exactly one place. */
export function scheduleDemandFrame(tick: () => void): { readonly cancel: () => void } {
  if (globalThis.document?.hidden === true || typeof globalThis.requestAnimationFrame !== "function") {
    const timer = setTimeout(tick, HIDDEN_DEMAND_FRAME_MS);
    return { cancel: () => clearTimeout(timer) };
  }
  const frame = requestAnimationFrame(tick);
  return { cancel: () => cancelAnimationFrame(frame) };
}

/** @emoji 🕸️ Minimal WASM graph session surface (attach, resize, RAF, optional pointer). */
export interface GraphWasmSession {
  attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  detachGpu?(): void;
  pointerDown?(x: number, y: number, button: number, extend: boolean, modifiers?: CanvasInputModifiers): void;
  pointerMove?(x: number, y: number): void;
  pointerUp?(x: number, y: number, modifiers?: CanvasInputModifiers): void;
  doubleClick?(x: number, y: number): void;
  wheel?(x: number, y: number, deltaY: number): void;
}

export interface GraphWasmCanvasProps {
  readonly className?: string;
  readonly sessionFactory: () => GraphWasmSession;
  readonly onSessionReady?: (session: GraphWasmSession) => void;
  readonly enablePointer?: boolean;
}

export function GraphWasmCanvas({ className, sessionFactory, onSessionReady, enablePointer = true }: GraphWasmCanvasProps): React.JSX.Element {
  const containerRef = React.useRef<HTMLDivElement>(null);
  const canvasRef = React.useRef<HTMLCanvasElement>(null);
  const sessionRef = React.useRef<GraphWasmSession | null>(null);

  const renderFrame = React.useCallback(() => {
    try {
      sessionRef.current?.renderFrame();
    } catch {
      /* gpu not ready */
    }
  }, []);

  React.useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;
    let torndown = false;
    let localRaf: { readonly cancel: () => void } | null = null;
    let localRo: ResizeObserver | null = null;
    let layoutRo: ResizeObserver | null = null;
    let degenerateTimer: ReturnType<typeof setTimeout> | null = null;
    const session = sessionFactory();
    sessionRef.current = session;
    onSessionReady?.(session);
    const modifiersOf = (ev: PointerEvent | MouseEvent): CanvasInputModifiers => ({
      shift: ev.shiftKey,
      ctrl: ev.ctrlKey,
      meta: ev.metaKey,
      alt: ev.altKey,
    });
    const onPointerDown = (ev: PointerEvent) => {
      const rect = canvas.getBoundingClientRect();
      session.pointerDown?.(ev.clientX - rect.left, ev.clientY - rect.top, ev.button, ev.shiftKey, modifiersOf(ev));
      renderFrame();
    };
    const onPointerMove = (ev: PointerEvent) => {
      const rect = canvas.getBoundingClientRect();
      session.pointerMove?.(ev.clientX - rect.left, ev.clientY - rect.top);
      renderFrame();
    };
    const onPointerUp = (ev: PointerEvent) => {
      const rect = canvas.getBoundingClientRect();
      session.pointerUp?.(ev.clientX - rect.left, ev.clientY - rect.top, modifiersOf(ev));
      renderFrame();
    };
    const onDoubleClick = (ev: MouseEvent) => {
      const rect = canvas.getBoundingClientRect();
      session.doubleClick?.(ev.clientX - rect.left, ev.clientY - rect.top);
      renderFrame();
    };
    const onWheel = (ev: WheelEvent) => {
      ev.preventDefault();
      const rect = canvas.getBoundingClientRect();
      session.wheel?.(ev.clientX - rect.left, ev.clientY - rect.top, ev.deltaY);
      renderFrame();
    };
    const attach = (initW: number, initH: number, dpr: number) => {
      canvas.width = Math.round(initW * dpr);
      canvas.height = Math.round(initH * dpr);
      canvas.style.width = `${initW}px`;
      canvas.style.height = `${initH}px`;
      const attachResult = typeof session.attachCanvas === "function" ? session.attachCanvas(canvas, initW, initH, dpr) : Promise.resolve();
      void Promise.resolve(attachResult).then(() => {
        if (torndown) return;
        const resize = () => {
          const rect = container.getBoundingClientRect();
          const dpr = globalThis.devicePixelRatio || 1;
          const w = Math.max(1, Math.round(rect.width));
          const h = Math.max(1, Math.round(rect.height));
          canvas.width = Math.round(w * dpr);
          canvas.height = Math.round(h * dpr);
          canvas.style.width = `${w}px`;
          canvas.style.height = `${h}px`;
          session.setSize(w, h, dpr);
          renderFrame();
        };
        resize();
        localRo = new ResizeObserver(resize);
        localRo.observe(container);
        const tick = () => {
          renderFrame();
          localRaf = scheduleDemandFrame(tick);
        };
        localRaf = scheduleDemandFrame(tick);
        if (enablePointer) {
          canvas.addEventListener("pointerdown", onPointerDown);
          canvas.addEventListener("pointermove", onPointerMove);
          canvas.addEventListener("pointerup", onPointerUp);
          canvas.addEventListener("pointerleave", onPointerUp);
          canvas.addEventListener("dblclick", onDoubleClick);
          canvas.addEventListener("wheel", onWheel, { passive: false });
        }
      });
    };
    // Waits for the container to report a real (non-degenerate) layout size before the first GPU attach —
    // attaching at a stale 1x1 rect (common on first paint, before flex/grid layout settles) leaves the
    // WebGPU surface configured at a bogus size; WebGPU surface errors are async/out-of-band and never
    // surface as a JS exception, so a botched first attach silently renders nothing forever after.
    // The wait is driven by `ResizeObserver` (plus one synchronous measure), never by an animation
    // frame: a hidden/background tab never ticks `requestAnimationFrame`, so the rAF poll this replaces
    // never attached at all there and every canvas stayed at the HTML default 300x150.
    const attachWhenLaidOut = (): boolean => {
      if (torndown) return true;
      const rect = container.getBoundingClientRect();
      if (rect.width < 8 || rect.height < 8) return false;
      attach(Math.round(rect.width), Math.round(rect.height), globalThis.devicePixelRatio || 1);
      return true;
    };
    if (!attachWhenLaidOut()) {
      layoutRo = new ResizeObserver(() => {
        if (!attachWhenLaidOut()) return;
        layoutRo?.disconnect();
        layoutRo = null;
      });
      layoutRo.observe(container);
      // Last resort for a container that never reaches a real size (a permanently collapsed pane):
      // attach at whatever it reports rather than leaving the surface unattached forever.
      degenerateTimer = setTimeout(() => {
        if (torndown || layoutRo === null) return;
        layoutRo.disconnect();
        layoutRo = null;
        const rect = container.getBoundingClientRect();
        attach(Math.max(1, Math.round(rect.width)), Math.max(1, Math.round(rect.height)), globalThis.devicePixelRatio || 1);
      }, DEGENERATE_LAYOUT_ATTACH_MS);
    }
    return () => {
      torndown = true;
      if (degenerateTimer != null) clearTimeout(degenerateTimer);
      layoutRo?.disconnect();
      localRo?.disconnect();
      if (enablePointer) {
        canvas.removeEventListener("pointerdown", onPointerDown);
        canvas.removeEventListener("pointermove", onPointerMove);
        canvas.removeEventListener("pointerup", onPointerUp);
        canvas.removeEventListener("pointerleave", onPointerUp);
        canvas.removeEventListener("dblclick", onDoubleClick);
        canvas.removeEventListener("wheel", onWheel);
      }
      localRaf?.cancel();
      sessionRef.current?.detachGpu?.();
      sessionRef.current = null;
    };
  }, [enablePointer, onSessionReady, renderFrame, sessionFactory]);

  return (
    <div ref={containerRef} className={className ?? "relative h-full w-full min-h-0 min-w-0"}>
      <canvas ref={canvasRef} className="block h-full w-full touch-none" />
    </div>
  );
}
// #endregion 🔖️GraphWasmCanvas

// #region 🔖️ReconcilerReexports
export type RenderMode = "main-thread" | "worker-offscreen" | "headless-test";
// #endregion 🔖️ReconcilerReexports

// #region 🔖️Vitest
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️canvaseventbindingcontroller/🟦️.tsx");
  await registerTests1(import.meta.vitest, { CanvasEventBindingController }, { directory: import.meta.dir, url: import.meta.url });
}
// #endregion 🔖️Vitest
