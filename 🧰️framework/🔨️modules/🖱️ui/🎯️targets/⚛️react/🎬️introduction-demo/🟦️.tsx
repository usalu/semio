import type { IntroductionGesture, IntroductionCursor, IntroductionKeyModifier, IntroductionDemonstration, IntroductionPoint } from "@semio-tech/framework-onboarding/declaration";
import * as React from "react";
import { reactHostPort } from "@semio-tech/ui/elements/Ports/react";
import { useShellScopeOptional } from "@semio-tech/ui/elements/ShellScope";
import { cn } from "@semio-tech/ui/modules/class-name-composition";
import { INTRODUCTION_DEMO_IDLE_THRESHOLD_MS, type IntroductionResolvedPoint, resolveIntroductionPoint, useIntroductionPointerIdle } from "../🧭️introduction-resolve/🟦️.ts";
import { introductionPointRelativeToHost } from "../📐️introduction-geometry/🟦️.ts";

export const INTRODUCTION_DEMO_GESTURE_DEFAULT_CURSOR: Record<IntroductionGesture["kind"], IntroductionCursor> = {
  leftClick: "pointer",
  rightClick: "pointer",
  doubleClick: "pointer",
  drag: "grab",
  scroll: "default",
  orbit: "move",
};

type IntroductionDemoPointerButton = "left" | "middle" | "right" | "wheel";

type IntroductionDemoFeedbackKind = "leftClick" | "rightClick" | "doubleClick" | "scroll" | "dragLeft" | "dragMiddle" | "dragRight" | "orbit";

type IntroductionDemoVisual = {
  readonly button: IntroductionDemoPointerButton;
  readonly modifiers: readonly IntroductionKeyModifier[];
  readonly feedback: IntroductionDemoFeedbackKind;
  readonly showDoubleChip: boolean;
};

/** 🖱️ Resolves the mini-mouse highlight, modifier chips, and press/trail family for a gesture. */
export function introductionDemoResolveVisual(gesture: IntroductionGesture): IntroductionDemoVisual {
  switch (gesture.kind) {
    case "leftClick":
      return { button: "left", modifiers: [], feedback: "leftClick", showDoubleChip: false };
    case "rightClick":
      return { button: "right", modifiers: [], feedback: "rightClick", showDoubleChip: false };
    case "doubleClick":
      return { button: "left", modifiers: [], feedback: "doubleClick", showDoubleChip: true };
    case "scroll":
      return { button: "wheel", modifiers: [], feedback: "scroll", showDoubleChip: false };
    case "drag": {
      const button = gesture.button ?? "left";
      const feedback = button === "middle" ? "dragMiddle" : button === "right" ? "dragRight" : "dragLeft";
      return { button, modifiers: gesture.modifiers ?? [], feedback, showDoubleChip: false };
    }
    case "orbit":
      return {
        button: gesture.button ?? "right",
        modifiers: gesture.modifiers ?? ["alt"],
        feedback: "orbit",
        showDoubleChip: false,
      };
  }
}

function introductionDemoRippleClass(feedback: IntroductionDemoFeedbackKind, double = false): string {
  const tone = feedback === "rightClick" || feedback === "dragRight" || feedback === "orbit" ? "right" : feedback === "dragMiddle" ? "middle" : "left";
  const base = `introduction-demo-ripple introduction-demo-ripple--${tone}`;
  return double ? `${base} introduction-demo-ripple--double` : base;
}

/** 🌐️ A point along the quadratic-bezier arc from `from` to `to`, bulged perpendicular to the
 * straight line between them — an `orbit` gesture reads as a curved rotation around a pivot, visually
 * distinct from `drag`'s straight-line pan/reposition. */
export function introductionDemoArcPoint(from: IntroductionResolvedPoint, to: IntroductionResolvedPoint, t: number): IntroductionResolvedPoint {
  const dx = to.x - from.x;
  const dy = to.y - from.y;
  const distance = Math.hypot(dx, dy) || 1;
  const bulge = Math.min(64, distance * 0.35);
  const controlX = (from.x + to.x) / 2 + (-dy / distance) * bulge;
  const controlY = (from.y + to.y) / 2 + (dx / distance) * bulge;
  const oneMinusT = 1 - t;
  return {
    x: oneMinusT * oneMinusT * from.x + 2 * oneMinusT * t * controlX + t * t * to.x,
    y: oneMinusT * oneMinusT * from.y + 2 * oneMinusT * t * controlY + t * t * to.y,
  };
}

const INTRODUCTION_DEMO_PHASE_MS = {
  appear: 250,
  travel: 600,
  press: 180,
  dragMove: 900,
  release: 180,
  linger: 500,
  fadeOut: 250,
  pause: 900,
} as const;

type IntroductionDemoPhase = keyof typeof INTRODUCTION_DEMO_PHASE_MS;

const INTRODUCTION_DEMO_NEXT_PHASE: Record<IntroductionDemoPhase, IntroductionDemoPhase> = {
  appear: "travel",
  travel: "press",
  press: "dragMove",
  dragMove: "release",
  release: "linger",
  linger: "fadeOut",
  fadeOut: "pause",
  pause: "appear",
};

function introductionDemoEaseInOutCubic(t: number): number {
  return t < 0.5 ? 4 * t * t * t : 1 - (-2 * t + 2) ** 3 / 2;
}

function introductionDemoLerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

const INTRODUCTION_DEMO_APPEAR_OFFSET = { x: -32, y: -32 } as const;

/** 🎬️ Ghost-cursor gesture demonstration(s) for an introduction step, mounted by `UIIntroduction`
 * whenever it has one or more effective demonstrations. Plays them in order, one full gesture-loop each,
 * then wraps back to the first — e.g. a viewport step showing zoom, then pan, then orbit, repeating.
 * Plays only while {@link useIntroductionPointerIdle} is true — any real pointer movement hides it and
 * restores the real cursor instantly (`data-introduction-demonstrating` cleared, see `🎨️ui.css`); going
 * idle again restarts the sequence from its first demonstration's `appear` phase. Never dispatches real
 * pointer events — purely visual, driven by an imperative rAF loop (no per-frame React state) that
 * re-resolves its `IntroductionPoint` endpoints every frame via {@link resolveIntroductionPoint} so it
 * stays glued to a moving/orbiting target. Renders nothing under `prefers-reduced-motion: reduce` — the
 * info box's "perform to continue" text remains the fallback. */
export const IntroductionDemonstrationOverlay: React.FC<{ readonly demonstrations: readonly IntroductionDemonstration[] }> = ({ demonstrations }) => {
  const overlayRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const ghostRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const calloutRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const trailPathRef = reactHostPort.useRef<SVGPathElement | null>(null);
  const rippleHostRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const shellScope = useShellScopeOptional();
  const demoHost = shellScope?.portalLayerRef.current ?? null;
  const demoPositionClass = demoHost ? "absolute" : "fixed";
  const demonstratingRoot = reactHostPort.useCallback((): Element | null => (shellScope ? shellScope.rootRef.current : document.documentElement), [shellScope]);
  const hide = reactHostPort.useCallback(() => {
    demonstratingRoot()?.removeAttribute("data-introduction-demonstrating");
    if (overlayRef.current) overlayRef.current.style.visibility = "hidden";
  }, [demonstratingRoot]);
  const { idle, lastPositionRef } = useIntroductionPointerIdle(true, INTRODUCTION_DEMO_IDLE_THRESHOLD_MS, hide);
  const mouseClipId = reactHostPort.useId().replace(/:/g, "");
  const reducedMotion = reactHostPort.useMemo(() => typeof window !== "undefined" && window.matchMedia?.("(prefers-reduced-motion: reduce)")?.matches === true, []);

  reactHostPort.useEffect(() => {
    if (reducedMotion || !idle || demonstrations.length === 0) {
      hide();
      return;
    }
    if (overlayRef.current) overlayRef.current.style.visibility = "visible";
    demonstratingRoot()?.setAttribute("data-introduction-demonstrating", "true");
    const origin = lastPositionRef.current;

    const setGlyph = (glyph: IntroductionCursor) => {
      const ghost = ghostRef.current;
      if (ghost) ghost.style.backgroundImage = `var(--cursor-ghost-${glyph})`;
    };
    const toOverlayPoint = (x: number, y: number) => {
      if (!demoHost) return { x, y };
      const hostRect = demoHost.getBoundingClientRect();
      return introductionPointRelativeToHost({ x, y }, hostRect);
    };
    const setPosition = (x: number, y: number) => {
      const ghost = ghostRef.current;
      if (!ghost) return;
      const local = toOverlayPoint(x, y);
      ghost.style.transform = `translate3d(${local.x}px, ${local.y}px, 0)`;
    };
    const setOpacity = (opacity: number) => {
      const ghost = ghostRef.current;
      if (ghost) ghost.style.opacity = String(opacity);
      const callout = calloutRef.current;
      if (callout) callout.style.opacity = String(opacity);
    };
    const setCallout = (x: number, y: number, visual: IntroductionDemoVisual, pressed: boolean, scrollDirection?: number) => {
      const callout = calloutRef.current;
      if (!callout) return;
      const local = toOverlayPoint(x, y);
      callout.style.transform = `translate3d(${local.x + 28}px, ${local.y + 8}px, 0)`;
      callout.dataset.button = visual.button;
      callout.dataset.feedback = visual.feedback;
      if (visual.modifiers.length > 0) callout.setAttribute("data-modifiers", visual.modifiers.join(" "));
      else callout.removeAttribute("data-modifiers");
      if (visual.showDoubleChip) callout.setAttribute("data-double", "true");
      else callout.removeAttribute("data-double");
      if (pressed) callout.setAttribute("data-pressed", "true");
      else callout.removeAttribute("data-pressed");
      if (scrollDirection !== undefined) callout.dataset.scrollDirection = scrollDirection >= 0 ? "down" : "up";
      else callout.removeAttribute("data-scroll-direction");
    };
    const clearTrail = () => {
      const path = trailPathRef.current;
      if (!path) return;
      path.removeAttribute("d");
      path.removeAttribute("class");
    };
    const updateTrail = (from: IntroductionResolvedPoint, to: IntroductionResolvedPoint, orbit: boolean, feedback: IntroductionDemoFeedbackKind) => {
      const path = trailPathRef.current;
      if (!path) return;
      const localFrom = toOverlayPoint(from.x, from.y);
      const localTo = toOverlayPoint(to.x, to.y);
      if (orbit) {
        const dx = localTo.x - localFrom.x;
        const dy = localTo.y - localFrom.y;
        const distance = Math.hypot(dx, dy) || 1;
        const bulge = Math.min(64, distance * 0.35);
        const controlX = (localFrom.x + localTo.x) / 2 + (-dy / distance) * bulge;
        const controlY = (localFrom.y + localTo.y) / 2 + (dx / distance) * bulge;
        path.setAttribute("d", `M ${localFrom.x} ${localFrom.y} Q ${controlX} ${controlY} ${localTo.x} ${localTo.y}`);
      } else {
        path.setAttribute("d", `M ${localFrom.x} ${localFrom.y} L ${localTo.x} ${localTo.y}`);
      }
      path.setAttribute("class", `introduction-demo-trail-path introduction-demo-trail-path--${feedback}`);
      const length = path.getTotalLength();
      path.style.setProperty("--introduction-demo-trail-length", String(length));
    };
    const spawnRipple = (x: number, y: number, feedback: IntroductionDemoFeedbackKind, double = false) => {
      const host = rippleHostRef.current;
      if (!host || feedback === "scroll") return;
      const local = toOverlayPoint(x, y);
      const ripple = document.createElement("div");
      ripple.className = `${introductionDemoRippleClass(feedback, double)} pointer-events-none ${demoPositionClass} rounded-full`;
      ripple.style.left = `${local.x - 12}px`;
      ripple.style.top = `${local.y - 12}px`;
      ripple.style.width = "24px";
      ripple.style.height = "24px";
      host.appendChild(ripple);
      ripple.addEventListener("animationend", () => ripple.remove());
    };
    const spawnZoomRing = (x: number, y: number, zoomIn: boolean) => {
      const host = rippleHostRef.current;
      if (!host) return;
      const local = toOverlayPoint(x, y);
      const ring = document.createElement("div");
      ring.className = `introduction-demo-zoom-ring introduction-demo-zoom-ring--${zoomIn ? "in" : "out"} pointer-events-none ${demoPositionClass}`;
      ring.style.left = `${local.x}px`;
      ring.style.top = `${local.y}px`;
      host.appendChild(ring);
      ring.addEventListener("animationend", () => ring.remove());
    };

    let cancelled = false;
    let rafId = 0;
    let demoIndex = 0;
    let phase: IntroductionDemoPhase = "appear";
    let phaseStart = performance.now();
    let rippleSpawned = false;
    let secondRippleSpawned = false;
    let thirdZoomSpawned = false;

    const tick = (now: number) => {
      if (cancelled) return;
      const demonstration=demonstrations[demoIndex];if(!demonstration)return;
      const gesture = demonstration.gesture;
      const visual = introductionDemoResolveVisual(gesture);
      const isDragLike = gesture.kind === "drag" || gesture.kind === "orbit";
      const isScroll = gesture.kind === "scroll";
      const hoverCursor = demonstration.cursor ?? INTRODUCTION_DEMO_GESTURE_DEFAULT_CURSOR[gesture.kind];
      const dragCursor = demonstration.cursor ?? (gesture.kind === "orbit" ? "move" : "grabbing");
      const targets: { readonly start: IntroductionPoint; readonly end: IntroductionPoint | null } = isDragLike ? { start: gesture.from, end: gesture.to } : { start: gesture.at, end: null };

      const start = resolveIntroductionPoint(targets.start, demonstratingRoot());
      const end = targets.end ? resolveIntroductionPoint(targets.end, demonstratingRoot()) : null;
      if (!start || (targets.end && !end)) {
        rafId = requestAnimationFrame(tick);
        return;
      }

      const duration = isScroll && phase === "press" ? 900 : INTRODUCTION_DEMO_PHASE_MS[phase];
      const t = Math.min(1, (now - phaseStart) / duration);
      const eased = introductionDemoEaseInOutCubic(t);
      const anchor = origin ?? { x: start.x + INTRODUCTION_DEMO_APPEAR_OFFSET.x, y: start.y + INTRODUCTION_DEMO_APPEAR_OFFSET.y };
      const pressed = !isScroll && (phase === "press" || phase === "dragMove");

      switch (phase) {
        case "appear":
          setGlyph("default");
          setOpacity(t);
          setPosition(anchor.x, anchor.y);
          setCallout(anchor.x, anchor.y, visual, false);
          clearTrail();
          break;
        case "travel":
          setOpacity(1);
          setPosition(introductionDemoLerp(anchor.x, start.x, eased), introductionDemoLerp(anchor.y, start.y, eased));
          setCallout(introductionDemoLerp(anchor.x, start.x, eased), introductionDemoLerp(anchor.y, start.y, eased), visual, false);
          if (t > 0.7) setGlyph(hoverCursor);
          clearTrail();
          break;
        case "press":
          setGlyph(hoverCursor);
          if (isScroll) {
            const zoomIn = gesture.deltaY < 0;
            const direction = zoomIn ? -1 : 1;
            setPosition(start.x, start.y);
            setCallout(start.x, start.y, visual, false, direction);
            if (!rippleSpawned) {
              spawnZoomRing(start.x, start.y, zoomIn);
              rippleSpawned = true;
            }
            if (t > 0.33 && !secondRippleSpawned) {
              spawnZoomRing(start.x, start.y, zoomIn);
              secondRippleSpawned = true;
            }
            if (t > 0.66 && !thirdZoomSpawned) {
              spawnZoomRing(start.x, start.y, zoomIn);
              thirdZoomSpawned = true;
            }
          } else {
            setPosition(start.x, start.y);
            setCallout(start.x, start.y, visual, true);
            if (!rippleSpawned) {
              spawnRipple(start.x, start.y, visual.feedback);
              rippleSpawned = true;
            }
            if (gesture.kind === "doubleClick" && t > 0.5 && !secondRippleSpawned) {
              spawnRipple(start.x, start.y, visual.feedback, true);
              secondRippleSpawned = true;
            }
          }
          clearTrail();
          break;
        case "dragMove": {
          setGlyph(dragCursor);
          if (end) {
            const point = gesture.kind === "orbit" ? introductionDemoArcPoint(start, end, eased) : { x: introductionDemoLerp(start.x, end.x, eased), y: introductionDemoLerp(start.y, end.y, eased) };
            setPosition(point.x, point.y);
            setCallout(point.x, point.y, visual, true);
            updateTrail(start, point, gesture.kind === "orbit", visual.feedback);
          } else {
            setPosition(start.x, start.y);
            setCallout(start.x, start.y, visual, true);
            clearTrail();
          }
          break;
        }
        case "release": {
          const point = end ?? start;
          setGlyph(hoverCursor);
          setPosition(point.x, point.y);
          setCallout(point.x, point.y, visual, false);
          clearTrail();
          break;
        }
        case "linger": {
          const point = end ?? start;
          setPosition(point.x, point.y);
          setCallout(point.x, point.y, visual, false);
          clearTrail();
          break;
        }
        case "fadeOut": {
          const point = end ?? start;
          setOpacity(1 - t);
          setPosition(point.x, point.y);
          setCallout(point.x, point.y, visual, pressed);
          clearTrail();
          break;
        }
        case "pause":
          setOpacity(0);
          clearTrail();
          break;
      }

      if (t >= 1) {
        phaseStart = now;
        rippleSpawned = false;
        secondRippleSpawned = false;
        thirdZoomSpawned = false;
        if (phase === "pause") demoIndex = (demoIndex + 1) % demonstrations.length;
        phase = phase === "press" && !isDragLike ? "release" : INTRODUCTION_DEMO_NEXT_PHASE[phase];
      }
      rafId = requestAnimationFrame(tick);
    };

    rafId = requestAnimationFrame(tick);
    return () => {
      cancelled = true;
      cancelAnimationFrame(rafId);
      hide();
    };
  }, [demonstratingRoot, demonstrations, hide, idle, reducedMotion, demoHost, demoPositionClass]);

  if (reducedMotion) return null;

  return (
    <div ref={overlayRef} data-slot="introduction-demonstration" className={cn("pointer-events-none inset-0", demoPositionClass)} style={{ visibility: "hidden", zIndex: "calc(var(--z-tutorial) + 2)" }}>
      <svg className={cn("pointer-events-none inset-0 h-full w-full overflow-visible", demoPositionClass)} aria-hidden="true">
        <path ref={trailPathRef} />
      </svg>
      <div
        ref={ghostRef}
        data-slot="introduction-demonstration-cursor"
        className={cn("pointer-events-none h-6 w-6 bg-contain bg-no-repeat opacity-0", demoPositionClass)}
        style={{ backgroundImage: "var(--cursor-ghost-default)", willChange: "transform, opacity" }}
      />
      <div ref={calloutRef} data-slot="introduction-demonstration-callout" className={cn("introduction-demo-callout pointer-events-none opacity-0 text-foreground", demoPositionClass)} data-button="left" data-feedback="leftClick">
        <svg className="introduction-demo-mouse" viewBox="0 0 48 72" aria-hidden="true">
          <defs>
            <clipPath id={mouseClipId}>
              <path d="M24 4C12.954 4 4 12.954 4 24v24c0 11.046 8.954 20 20 20s20-8.954 20-20V24C44 12.954 35.046 4 24 4Z" />
            </clipPath>
          </defs>
          <g clipPath={`url(#${mouseClipId})`}>
            <path className="introduction-demo-mouse-button" data-part="left" d="M24 4C12.954 4 4 12.954 4 24V28H24Z" />
            <path className="introduction-demo-mouse-button" data-part="right" d="M24 4C35.046 4 44 12.954 44 24V28H24Z" />
            <path className="introduction-demo-mouse-button" data-part="middle" d="M21 12H27V22H21Z" />
          </g>
          <path className="introduction-demo-mouse-body" d="M24 4C12.954 4 4 12.954 4 24v24c0 11.046 8.954 20 20 20s20-8.954 20-20V24C44 12.954 35.046 4 24 4Z" />
          <path className="introduction-demo-mouse-divider" d="M24 4V28" />
          <path className="introduction-demo-mouse-seam" d="M8 28H40" />
          <path className="introduction-demo-mouse-wheel" d="M24 14v10" />
        </svg>
        <div className="introduction-demo-callout-chips">
          <kbd className="introduction-demo-callout-chip" data-chip="zoom-in">
            +
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="zoom-out">
            −
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="double">
            2×
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="alt">
            Alt
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="shift">
            Shift
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="control">
            Ctrl
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="meta">
            ⌘️
          </kbd>
        </div>
      </div>
      <div ref={rippleHostRef} className={cn("pointer-events-none inset-0", demoPositionClass)} />
    </div>
  );
};

/** 🎬️ The default demonstration for a purely informational step (`interactions.length === 0`, the
 * Next/Done button is the only way to continue) — clicking `ui.introduction.next`, the stable id every
 * such button renders under regardless of its Next/Done label. Used by `UIIntroduction` whenever the step
 * doesn't declare its own `demonstration`, so authors never have to spell this out per step. */
export const INTRODUCTION_DEMO_NEXT_BUTTON_DEMONSTRATION: IntroductionDemonstration = { gesture: { kind: "leftClick", at: { kind: "element", id: "ui.introduction.next" } } };
