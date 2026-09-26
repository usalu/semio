// #region 🧲️Header
/** @emoji 🖼️ `@semio-tech/infinite-canvas-react-renderer` — React host for tile-based infinite canvases (WASM bridge supplied by leaf bundles). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ContextMenuController, decodeIcon, encodeIcon, resolveIconUrlsInBoardJson, reactHostPort, useCanvasAppearanceSync, type ContextMenuItem, type Icon, type IconSelectorMode } from "@semio-tech/ui-react";
import React from "react";
import { GestureRecognizer, type PinchStep } from "@semio-tech/framework";
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

/** @emoji ⏱️ A render-on-demand frame clock for one canvas. `invalidate` marks the canvas dirty and paints it at the next
 * frame (any number of invalidations before that frame coalesce into ONE paint); `paintNow` paints synchronously and
 * satisfies every invalidation that preceded it, so a pending frame paints nothing twice; `beginContinuous`/
 * `endContinuous` bracket genuinely continuous work (a gesture whose moves do not invalidate one by one). */
export type DemandFrameSchedulerV1 = {
  invalidate(): void;
  paintNow(): void;
  beginContinuous(reason: string): void;
  endContinuous(reason: string): void;
  dispose(): void;
};

/** @emoji 🌗️ The trailing window of a surface whose wasm session eases its own state after an input (a flow graph's or a
 * tiled map's camera settling): it keeps painting this long after its last invalidation. */
export const EASED_SURFACE_TRAILING_WINDOW_MS = 250;

/** @emoji 🎚️ Options of {@link createDemandFrameScheduler}. `trailingWindowMs` is an EXPLICIT, bounded animation window:
 * a surface whose wasm session eases state on its own (a camera settling after a gesture) keeps painting that long after
 * its last invalidation. A surface without self-animating state declares none and paints exactly once per demand. */
export type DemandFrameSchedulerOptionsV1 = {
  readonly trailingWindowMs?: number;
};

/** @emoji 🪶️ The one render-on-demand scheduler of every wasm canvas surface (flow node-graph, tiled-map,
 * {@link GraphWasmCanvas}); each of them used to run an unconditional animation-frame loop that held the tab at 60 fps
 * fully idle (REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT; ticket 26/09/23 S15 for {@link GraphWasmCanvas}). A frame paints
 * only when the canvas is dirty, a continuous reason is held, or an explicit trailing window runs; a synchronous
 * `paintNow` clears the dirt, so an owner that paints on its own never gets the same frame painted again by the clock
 * (ticket 26/09/23 F1: a typed character cost 2–3 paints of ~90 ms each in the trinity query editor, and a node-graph
 * drag painted every frame twice — once by its own clock, once by the canvas's). */
export function createDemandFrameScheduler(render: () => void, opts?: DemandFrameSchedulerOptionsV1): DemandFrameSchedulerV1 {
  const trailingWindowMs = opts?.trailingWindowMs ?? 0;
  const continuousReasons = new Set<string>();
  let handle: { readonly cancel: () => void } | null = null;
  let trailingUntil = 0;
  let dirty = false;
  let disposed = false;

  const animating = () => continuousReasons.size > 0 || Date.now() < trailingUntil;
  const tick = () => {
    handle = null;
    if (disposed) return;
    if (dirty || animating()) {
      dirty = false;
      render();
    }
    if (animating()) schedule();
  };
  const schedule = () => {
    handle = scheduleDemandFrame(tick);
  };
  const ensureScheduled = () => {
    if (disposed || handle !== null) return;
    schedule();
  };

  return {
    invalidate() {
      dirty = true;
      if (trailingWindowMs > 0) trailingUntil = Date.now() + trailingWindowMs;
      ensureScheduled();
    },
    paintNow() {
      if (disposed) return;
      dirty = false;
      render();
    },
    beginContinuous(reason: string) {
      continuousReasons.add(reason);
      ensureScheduled();
    },
    endContinuous(reason: string) {
      continuousReasons.delete(reason);
      if (continuousReasons.size > 0) return;
      dirty = true;
      if (trailingWindowMs > 0) trailingUntil = Date.now() + trailingWindowMs;
      ensureScheduled();
    },
    dispose() {
      disposed = true;
      continuousReasons.clear();
      handle?.cancel();
      handle = null;
    },
  };
}

/** @emoji 🎯️ The owner's handle on a canvas session: every call except `renderFrame` invalidates the canvas (painted once
 * at the next frame, coalesced with every other call before it), and `renderFrame` paints at once and satisfies those
 * invalidations. So a canvas repaints exactly when its owner changed something (a scene sync, a caret, a theme, a camera),
 * never twice for one change, and an idle canvas paints nothing. It replaces an unconditional per-frame repaint that held
 * the main thread at 100 % for every node-graph and text-editor window (measured ticket 26/09/23 S15: an idle trinity jack
 * editor painted its query text at ~98 ms per frame, so every host continuation waited a frame and one retained patch
 * intake took 20 s). */
export function frameDemandingSessionV1<Session extends object>(session: Session, scheduler: Pick<DemandFrameSchedulerV1, "invalidate" | "paintNow">): Session {
  return new Proxy(session, {
    get(target, key) {
      const value: unknown = Reflect.get(target, key, target);
      if (typeof value !== "function") return value;
      const method = value as (...args: unknown[]) => unknown;
      if (key === "renderFrame") return () => scheduler.paintNow();
      return (...args: unknown[]) => {
        scheduler.invalidate();
        return method.apply(target, args);
      };
    },
  });
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
  /** 🚫️ The pointer left the canvas, the browser took the pointer (`pointercancel`) or capture was lost
   * mid-gesture. This is a CANCEL, never a release: {@link GraphWasmCanvas} used to map `pointerleave` to
   * `pointerUp`, which forged a `canvasPointerUp` for a gesture the user never ended (and a second one
   * after every real release outside the canvas). A session without a gesture treats it as a no-op. */
  pointerCancel?(): void;
  doubleClick?(x: number, y: number): void;
  wheel?(x: number, y: number, deltaY: number): void;
  /** 🤏️ One two-finger step from the shared `👆️gesture` recognizer (surface pixels). A session without it
   * still gets the recognizer's suppression: its single-pointer lane never sees a pinch's contacts. */
  pinch?(step: PinchStep): void;
}

export interface GraphWasmCanvasProps {
  readonly className?: string;
  readonly sessionFactory: () => GraphWasmSession;
  /** The owner's session handle ({@link frameDemandingSessionV1}): calls through it repaint the canvas on demand. */
  readonly onSessionReady?: (session: GraphWasmSession) => void;
  readonly enablePointer?: boolean;
}

export function GraphWasmCanvas({ className, sessionFactory, onSessionReady, enablePointer = true }: GraphWasmCanvasProps): React.JSX.Element {
  const containerRef = React.useRef<HTMLDivElement>(null);
  const canvasRef = React.useRef<HTMLCanvasElement>(null);
  const sessionRef = React.useRef<GraphWasmSession | null>(null);
  const schedulerRef = React.useRef<DemandFrameSchedulerV1 | null>(null);
  useCanvasAppearanceSync(() => schedulerRef.current?.invalidate(), true);

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
    let localRo: ResizeObserver | null = null;
    let layoutRo: ResizeObserver | null = null;
    let degenerateTimer: ReturnType<typeof setTimeout> | null = null;
    const session = sessionFactory();
    sessionRef.current = session;
    const scheduler = createDemandFrameScheduler(renderFrame);
    schedulerRef.current = scheduler;
    onSessionReady?.(frameDemandingSessionV1(session, scheduler));
    const modifiersOf = (ev: PointerEvent | MouseEvent): CanvasInputModifiers => ({
      shift: ev.shiftKey,
      ctrl: ev.ctrlKey,
      meta: ev.metaKey,
      alt: ev.altKey,
    });
    const gestures = new GestureRecognizer();
    const gesturePoint = (ev: PointerEvent) => {
      const rect = canvas.getBoundingClientRect();
      return { pointerId: ev.pointerId, x: ev.clientX - rect.left, y: ev.clientY - rect.top };
    };
    const onPointerDown = (ev: PointerEvent) => {
      if (enablePointer) canvas.setPointerCapture(ev.pointerId);
      const verdict = gestures.down(gesturePoint(ev));
      if (verdict.kind === "pinchBegin") {
        session.pointerCancel?.();
        scheduler.paintNow();
        return;
      }
      if (verdict.kind !== "single") return;
      const rect = canvas.getBoundingClientRect();
      session.pointerDown?.(ev.clientX - rect.left, ev.clientY - rect.top, ev.button, ev.shiftKey, modifiersOf(ev));
      scheduler.paintNow();
    };
    const onPointerMove = (ev: PointerEvent) => {
      const verdict = gestures.move(gesturePoint(ev));
      if (verdict.kind === "pinch") {
        session.pinch?.(verdict.step);
        scheduler.paintNow();
        return;
      }
      if (verdict.kind !== "single") return;
      const rect = canvas.getBoundingClientRect();
      session.pointerMove?.(ev.clientX - rect.left, ev.clientY - rect.top);
      scheduler.paintNow();
    };
    const onPointerUp = (ev: PointerEvent) => {
      if (canvas.hasPointerCapture(ev.pointerId)) canvas.releasePointerCapture(ev.pointerId);
      if (gestures.up(ev.pointerId).kind !== "single") return;
      const rect = canvas.getBoundingClientRect();
      session.pointerUp?.(ev.clientX - rect.left, ev.clientY - rect.top, modifiersOf(ev));
      scheduler.paintNow();
    };
    // 🚫️ `pointerleave` / `pointercancel` / `lostpointercapture` are gesture CANCELS. While the canvas holds
    // capture, `pointerleave` and `lostpointercapture` only fire after `pointerup` has already closed the
    // gesture, so the session sees them as no-ops; only a capture lost mid-gesture (or a pointer the
    // browser reclaims for scrolling) reaches the session as a cancel.
    const onPointerCancel = (ev: PointerEvent) => {
      if (ev.type !== "lostpointercapture" && canvas.hasPointerCapture(ev.pointerId)) canvas.releasePointerCapture(ev.pointerId);
      if (ev.type !== "pointerleave" && gestures.up(ev.pointerId).kind !== "single") return;
      if (gestures.latched) return;
      session.pointerCancel?.();
      scheduler.paintNow();
    };
    const onDoubleClick = (ev: MouseEvent) => {
      const rect = canvas.getBoundingClientRect();
      session.doubleClick?.(ev.clientX - rect.left, ev.clientY - rect.top);
      scheduler.invalidate();
    };
    const onWheel = (ev: WheelEvent) => {
      ev.preventDefault();
      const rect = canvas.getBoundingClientRect();
      session.wheel?.(ev.clientX - rect.left, ev.clientY - rect.top, ev.deltaY);
      scheduler.invalidate();
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
          scheduler.paintNow();
        };
        resize();
        localRo = new ResizeObserver(resize);
        localRo.observe(container);
        if (enablePointer) {
          canvas.addEventListener("pointerdown", onPointerDown);
          canvas.addEventListener("pointermove", onPointerMove);
          canvas.addEventListener("pointerup", onPointerUp);
          canvas.addEventListener("pointerleave", onPointerCancel);
          canvas.addEventListener("pointercancel", onPointerCancel);
          canvas.addEventListener("lostpointercapture", onPointerCancel);
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
        canvas.removeEventListener("pointerleave", onPointerCancel);
        canvas.removeEventListener("pointercancel", onPointerCancel);
        canvas.removeEventListener("lostpointercapture", onPointerCancel);
        canvas.removeEventListener("dblclick", onDoubleClick);
        canvas.removeEventListener("wheel", onWheel);
      }
      scheduler.dispose();
      if (schedulerRef.current === scheduler) schedulerRef.current = null;
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
  const { registerTests2 } = await import("./🧪️tests/🪶️demand-frames/🟦️.tsx");
  await registerTests2(import.meta.vitest, { GraphWasmCanvas, React, createDemandFrameScheduler }, { directory: import.meta.dir, url: import.meta.url });
}
// #endregion 🔖️Vitest
