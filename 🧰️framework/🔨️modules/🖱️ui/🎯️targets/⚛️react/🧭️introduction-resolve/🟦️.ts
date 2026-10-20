import type { IntroductionPoint } from "@semio-tech/framework-onboarding/declaration";
import { elementIdSelector } from "@semio-tech/ui/elements/ElementId";
import * as React from "react";
import { reactHostPort } from "@semio-tech/ui/elements/Ports/react";
import { introductionSurfaceResolvers } from "../📐️introduction-geometry/🟦️.ts";

//#region 🎬️DemonstrationResolve
export type IntroductionResolvedPoint = { readonly x: number; readonly y: number };

/** 🪡️ A point at `t` (0–1, clamped) along `points` by arc length — linearly interpolates between
 * the two points straddling `t`'s cumulative distance. Degenerate inputs (0 or 1 points, zero-length
 * polyline) return the first point (or the origin) for every `t`, never throw. */
export function polylinePointAt(points: readonly { readonly x: number; readonly y: number }[], t: number): { readonly x: number; readonly y: number } {
  if (points.length === 0) return { x: 0, y: 0 };
  const first=points[0];if(!first)throw Error("Complete polyline point sequence required");
  if(points.length===1)return first;
  const segmentLengths: number[] = [];
  let totalLength = 0;
  for (let i = 1; i < points.length; i++) {
    const point=points[i],prior=points[i-1];if(!point||!prior)throw Error("Complete polyline segment required");
    const length=Math.hypot(point.x-prior.x,point.y-prior.y);
    segmentLengths.push(length);
    totalLength += length;
  }
  if(totalLength===0)return first;
  const targetDistance = Math.min(1, Math.max(0, t)) * totalLength;
  let accumulated = 0;
  for (let i = 0; i < segmentLengths.length; i++) {
    const segmentLength=segmentLengths[i],point=points[i],next=points[i+1];if(segmentLength===undefined||!point||!next)throw Error("Complete polyline length and points required");
    if (accumulated + segmentLength >= targetDistance || i === segmentLengths.length - 1) {
      const segmentT = segmentLength === 0 ? 0 : (targetDistance - accumulated) / segmentLength;
      return {
        x:point.x+(next.x-point.x)*segmentT,
        y:point.y+(next.y-point.y)*segmentT,
      };
    }
    accumulated += segmentLength;
  }
  const last=points.at(-1);if(!last)throw Error("Complete polyline terminal point required");return last;
}

/** 🪡️ Samples a canvas-layer path (`move`/`line`/`quad`/`cubic` segments, in whatever local
 * coordinate space the caller's points are already expressed in — apply layer transforms before calling)
 * into a flat polyline via de Casteljau evaluation — the one parametric-curve evaluator this codebase has,
 * needed because `IntroductionPoint.Curve` resolves through arc-length interpolation over a polyline, not
 * a true parametric curve. `arc`/`close` segments are not sampled (curve targeting degrades to the
 * straight gap between their neighbors); rare enough for a demonstration target to not warrant full SVG
 * arc-to-bezier conversion. */
export function sampleBezierSegments(
  segments: readonly {
    readonly kind?: string;
    readonly to?: readonly [number, number];
    readonly ctrl?: readonly [number, number];
    readonly ctrl1?: readonly [number, number];
    readonly ctrl2?: readonly [number, number];
  }[],
  samplesPerCurve = 16,
): readonly { readonly x: number; readonly y: number }[] {
  const points: { x: number; y: number }[] = [];
  let current = { x: 0, y: 0 };
  for (const segment of segments) {
    if ((segment.kind === "move" || segment.kind === "line") && segment.to) {
      current = { x: segment.to[0], y: segment.to[1] };
      points.push(current);
    } else if (segment.kind === "quad" && segment.ctrl && segment.to) {
      const ctrl = { x: segment.ctrl[0], y: segment.ctrl[1] };
      const to = { x: segment.to[0], y: segment.to[1] };
      for (let i = 1; i <= samplesPerCurve; i++) {
        const t = i / samplesPerCurve;
        const oneMinusT = 1 - t;
        points.push({
          x: oneMinusT * oneMinusT * current.x + 2 * oneMinusT * t * ctrl.x + t * t * to.x,
          y: oneMinusT * oneMinusT * current.y + 2 * oneMinusT * t * ctrl.y + t * t * to.y,
        });
      }
      current = to;
    } else if (segment.kind === "cubic" && segment.ctrl1 && segment.ctrl2 && segment.to) {
      const ctrl1 = { x: segment.ctrl1[0], y: segment.ctrl1[1] };
      const ctrl2 = { x: segment.ctrl2[0], y: segment.ctrl2[1] };
      const to = { x: segment.to[0], y: segment.to[1] };
      for (let i = 1; i <= samplesPerCurve; i++) {
        const t = i / samplesPerCurve;
        const oneMinusT = 1 - t;
        points.push({
          x: oneMinusT ** 3 * current.x + 3 * oneMinusT * oneMinusT * t * ctrl1.x + 3 * oneMinusT * t * t * ctrl2.x + t ** 3 * to.x,
          y: oneMinusT ** 3 * current.y + 3 * oneMinusT * oneMinusT * t * ctrl1.y + 3 * oneMinusT * t * t * ctrl2.y + t ** 3 * to.y,
        });
      }
      current = to;
    }
  }
  return points;
}

/** 📌️ Resolves an `IntroductionPoint` to a live viewport pixel. Called every animation frame by a
 * playing demonstration — re-resolving (rather than caching) keeps a drag path glued to a target that's
 * still moving (a resizing panel, an orbiting 3D camera, a panning 2D canvas). `null` means "not
 * resolvable yet" (element not mounted, no surface resolver registered, entity not found/hidden, or a 3D
 * point off-camera) — callers wait and retry next frame rather than treating it as an error. */
export function resolveIntroductionPoint(point: IntroductionPoint, root: ParentNode | null = document): IntroductionResolvedPoint | null {
  if (!root) return null;
  switch (point.kind) {
    case "element": {
      const element = root.querySelector(elementIdSelector(point.id));
      if (!element) return null;
      const rect = element.getBoundingClientRect();
      const [offsetX, offsetY] = point.offset ?? [0.5, 0.5];
      return { x: rect.left + offsetX * rect.width, y: rect.top + offsetY * rect.height };
    }
    case "screen":
      return { x: point.x, y: point.y };
    case "screenNormalized":
      return { x: point.x * window.innerWidth, y: point.y * window.innerHeight };
    case "window": {
      const element = root.querySelector(elementIdSelector(point.id));
      if (!element) return null;
      const rect = element.getBoundingClientRect();
      return { x: rect.left + point.x, y: rect.top + point.y };
    }
    case "windowNormalized": {
      const element = root.querySelector(elementIdSelector(point.id));
      if (!element) return null;
      const rect = element.getBoundingClientRect();
      return { x: rect.left + point.x * rect.width, y: rect.top + point.y * rect.height };
    }
    case "scene": {
      const resolver = introductionSurfaceResolvers.get(point.id);
      const projected = resolver?.scenePoint?.(point.position);
      if (!projected || !projected.visible) return null;
      return { x: projected.x, y: projected.y };
    }
    case "canvas": {
      const resolver = introductionSurfaceResolvers.get(point.id);
      const projected = resolver?.canvasPoint?.(point.x, point.y);
      if (!projected || !projected.visible) return null;
      return { x: projected.x, y: projected.y };
    }
    case "entity": {
      const resolver = introductionSurfaceResolvers.get(point.id);
      const geometry = resolver?.entity?.(point.domain, point.entity);
      if (!geometry || !geometry.visible) return null;
      if (!geometry.rect) return geometry.point;
      const [offsetX, offsetY] = point.offset ?? [0.5, 0.5];
      return { x: geometry.rect.x + offsetX * geometry.rect.width, y: geometry.rect.y + offsetY * geometry.rect.height };
    }
    case "curve": {
      const resolver = introductionSurfaceResolvers.get(point.id);
      const geometry = resolver?.entity?.(point.domain, point.entity);
      if (!geometry || !geometry.visible || !geometry.polyline || geometry.polyline.length === 0) return null;
      return polylinePointAt(geometry.polyline, point.t);
    }
    case "domain": {
      const resolver = introductionSurfaceResolvers.get(point.id);
      const geometry = resolver?.entity?.(point.domain, point.entity);
      if (!geometry || !geometry.visible || !geometry.rect || !geometry.domain) return null;
      const { min, max, axis } = geometry.domain;
      const t = max === min ? 0 : (Math.min(max, Math.max(min, point.value)) - min) / (max - min);
      const rect = geometry.rect;
      return axis === "x" ? { x: rect.x + t * rect.width, y: rect.y + rect.height / 2 } : { x: rect.x + rect.width / 2, y: rect.y + (1 - t) * rect.height };
    }
    default:
      return null;
  }
}
//#endregion 🎬️DemonstrationResolve

export const INTRODUCTION_DEMO_IDLE_THRESHOLD_MS = 1600;

type IntroductionPointerPosition = { readonly x: number; readonly y: number };

type IntroductionPointerIdleState = {
  readonly idle: boolean;
  /** 🎬️ The real pointer's last observed viewport position (`null` until it has moved at least once) —
   * lets a demonstration open by traveling from where the user's actual cursor is resting, instead of
   * materializing out of nowhere near its target. Stable ref identity; read `.current`, don't watch it. */
  readonly lastPositionRef: React.RefObject<IntroductionPointerPosition | null>;
};

/** 💤️ Tracks whether the user's real pointer has been still for `thresholdMs` — the gate a
 * demonstration plays behind — and where it last was. Any `pointerdown`/`wheel`/`keydown`, or a
 * `pointermove` with different coordinates or non-zero movement deltas, resets the timer and flips back
 * to not-idle immediately. Same-coordinate, zero-delta moves are ignored because browsers re-fire them
 * when the DOM mutates under a stationary cursor. `onActivity` runs synchronously inside the native
 * event handler so the visual disappears before React commits the idle-state update. Starts not-idle:
 * a demonstration should only appear once the user has first settled, not the instant a step mounts
 * mid-motion. */
export function useIntroductionPointerIdle(enabled: boolean, thresholdMs: number = INTRODUCTION_DEMO_IDLE_THRESHOLD_MS, onActivity?: () => void): IntroductionPointerIdleState {
  const [idle, setIdle] = reactHostPort.useState(false);
  const lastPositionRef = reactHostPort.useRef<IntroductionPointerPosition | null>(null);
  const onActivityRef = reactHostPort.useRef(onActivity);
  onActivityRef.current = onActivity;

  reactHostPort.useEffect(() => {
    if (!enabled) {
      setIdle(false);
      return;
    }
    let timer: ReturnType<typeof setTimeout> | null = null;

    const settle = () => {
      timer = setTimeout(() => setIdle(true), thresholdMs);
    };
    const unsettle = () => {
      onActivityRef.current?.();
      setIdle(false);
      if (timer) clearTimeout(timer);
      settle();
    };
    const onPointerMove = (event: PointerEvent) => {
      const previous = lastPositionRef.current;
      const coordinatesChanged = !previous || event.clientX !== previous.x || event.clientY !== previous.y;
      const movementChanged = event.movementX !== 0 || event.movementY !== 0;
      if (!coordinatesChanged && !movementChanged) return;
      lastPositionRef.current = { x: event.clientX, y: event.clientY };
      unsettle();
    };
    const onOtherInput = () => unsettle();

    window.addEventListener("pointermove", onPointerMove, true);
    window.addEventListener("pointerdown", onOtherInput, true);
    window.addEventListener("wheel", onOtherInput, true);
    window.addEventListener("keydown", onOtherInput, true);
    settle();
    return () => {
      if (timer) clearTimeout(timer);
      window.removeEventListener("pointermove", onPointerMove, true);
      window.removeEventListener("pointerdown", onOtherInput, true);
      window.removeEventListener("wheel", onOtherInput, true);
      window.removeEventListener("keydown", onOtherInput, true);
    };
  }, [enabled, thresholdMs]);

  return { idle, lastPositionRef };
}
