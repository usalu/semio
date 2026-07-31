// #region 🧲️Header
/** @emoji 🖼️ `@semio-tech/infinite-canvas-react-renderer` — r3f-style reconciler host for tile-based infinite canvases (WASM bridge supplied by leaf bundles). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ContextMenuController, decodeIcon, encodeIcon, resolveIconUrlsInBoardJson, reactHostPort, type ContextMenuItem, type Icon, type IconSelectorMode } from "@semio-tech/ui-react";
import React from "react";
import Reconciler from "react-reconciler";
import { ContinuousEventPriority, DefaultEventPriority, DiscreteEventPriority, LegacyRoot, NoEventPriority } from "react-reconciler/constants";
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
  Reconciler,
  ContinuousEventPriority,
  DefaultEventPriority,
  DiscreteEventPriority,
  LegacyRoot,
  NoEventPriority,
};

// #region 🔖️EventBinding
export type CavasListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

/** @emoji 🎧️ Tracks DOM listeners for deterministic teardown on canvas unmount. */
export class CavasEventBindingController {
  private readonly cleanups: Array<() => void> = [];

  listen(target: CavasListenerTarget | null | undefined, kind: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions): void {
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
  const rafRef = React.useRef<number | null>(null);

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
    let waitRaf: number | null = null;
    let localRaf: number | null = null;
    let localRo: ResizeObserver | null = null;
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
      void session.attachCanvas(canvas, initW, initH, dpr).then(() => {
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
          localRaf = requestAnimationFrame(tick);
        };
        localRaf = requestAnimationFrame(tick);
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
    let attempts = 0;
    const waitForLayout = () => {
      if (torndown) return;
      const rect = container.getBoundingClientRect();
      const dpr = globalThis.devicePixelRatio || 1;
      attempts += 1;
      if (rect.width >= 8 && rect.height >= 8) {
        attach(Math.round(rect.width), Math.round(rect.height), dpr);
        return;
      }
      if (attempts > 120) {
        attach(Math.max(1, Math.round(rect.width)), Math.max(1, Math.round(rect.height)), dpr);
        return;
      }
      waitRaf = requestAnimationFrame(waitForLayout);
    };
    waitForLayout();
    return () => {
      torndown = true;
      if (waitRaf != null) cancelAnimationFrame(waitRaf);
      localRo?.disconnect();
      if (enablePointer) {
        canvas.removeEventListener("pointerdown", onPointerDown);
        canvas.removeEventListener("pointermove", onPointerMove);
        canvas.removeEventListener("pointerup", onPointerUp);
        canvas.removeEventListener("pointerleave", onPointerUp);
        canvas.removeEventListener("dblclick", onDoubleClick);
        canvas.removeEventListener("wheel", onWheel);
      }
      if (localRaf != null) cancelAnimationFrame(localRaf);
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
  const { describe, expect, it } = import.meta.vitest;

  describe("CavasEventBindingController", () => {
    it("disposes registered listeners", () => {
      const ctrl = new CavasEventBindingController();
      let count = 0;
      const target = {
        addEventListener: () => {
          count += 1;
        },
        removeEventListener: () => {
          count -= 1;
        },
      };
      ctrl.listen(target, "pointermove", () => {});
      expect(count).toBe(1);
      ctrl.dispose();
      expect(count).toBe(0);
    });
  });
}
// #endregion 🔖️Vitest
