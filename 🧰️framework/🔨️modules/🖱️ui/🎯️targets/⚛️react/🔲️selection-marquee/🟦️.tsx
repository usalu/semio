import * as React from "react";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import type { MergeMode } from "@semio-tech/framework/interaction";

/** 👉️ ⬚️ Canonical area-select overlay coverage (drag right-to-left = partial). */
export type SelectionMarqueeCoverage = "partial" | "full";

export type SelectionMarqueeRect = {
  readonly x: number | string;
  readonly y: number | string;
  readonly width: number | string;
  readonly height: number | string;
};

export type SelectionMarqueePoint = {
  readonly x: number;
  readonly y: number;
};

export type SelectionMarqueeProps = {
  readonly coverage: SelectionMarqueeCoverage;
  readonly className?: string;
} & ({ readonly shape: "rect"; readonly rect: SelectionMarqueeRect } | { readonly shape: "polygon"; readonly points: readonly SelectionMarqueePoint[] });

/** ☑️ ⬚️ Shared SVG marquee for spatial area selection (primary fill/stroke; dashed when partial). */
export function SelectionMarquee(props: SelectionMarqueeProps): React.ReactElement {
  const { coverage, className } = props;
  const svgClass = cn("selection-marquee pointer-events-none absolute inset-0 h-full w-full overflow-visible", className);
  if (props.shape === "rect") {
    const { rect } = props;
    return (
      <svg className={svgClass} data-coverage={coverage} aria-hidden>
        <rect x={rect.x} y={rect.y} width={rect.width} height={rect.height} />
      </svg>
    );
  }
  const points = props.points.map((point) => `${point.x},${point.y}`).join(" ");
  return (
    <svg className={svgClass} data-coverage={coverage} aria-hidden>
      <polygon points={points} />
    </svg>
  );
}

export const SELECTION_DRAG_DIRECTION_THRESHOLD_PX = 2;

export type SelectionMarqueeMethod = "lasso" | "rectangle";

/** 🖱️ Crossing selection when the drag ends left of the start (partial overlap). */
export function marqueeIsCrossing(startX: number, endX: number): boolean {
  return endX < startX;
}

/** 🖱️ Lasso uses the first horizontal step; rectangle compares start vs end. */
export function marqueeIsCrossingFromPath(path: readonly SelectionMarqueePoint[], method: SelectionMarqueeMethod = "rectangle"): boolean {
  const start = path[0];
  if (!start) return false;
  if (method === "lasso") {
    for (const point of path.slice(1)) {
      const dx = point.x - start.x;
      if (Math.abs(dx) < SELECTION_DRAG_DIRECTION_THRESHOLD_PX) continue;
      return dx < 0;
    }
  }
  const end = path[path.length - 1] ?? start;
  return marqueeIsCrossing(start.x, end.x);
}

/** 🖱️ Maps drag direction to marquee coverage (rectangle endpoints). */
export function marqueeCoverageFromDrag(startX: number, endX: number): SelectionMarqueeCoverage {
  return marqueeIsCrossing(startX, endX) ? "partial" : "full";
}

/** 🖱️ Maps gesture path to marquee coverage (lasso first horizontal step). */
export function marqueeCoverageFromPath(path: readonly SelectionMarqueePoint[], method: SelectionMarqueeMethod = "rectangle"): SelectionMarqueeCoverage {
  return marqueeIsCrossingFromPath(path, method) ? "partial" : "full";
}

/** 🖱️ Resolves marquee coverage for rectangle or lasso gestures. */
export function marqueeCoverageFromGesture(input: { readonly method: SelectionMarqueeMethod; readonly startX: number; readonly endX: number; readonly path: readonly SelectionMarqueePoint[] }): SelectionMarqueeCoverage {
  if (input.method === "lasso" && input.path.length > 0) {
    return marqueeCoverageFromPath(input.path, "lasso");
  }
  return marqueeCoverageFromDrag(input.startX, input.endX);
}

/** 🎯️ Maps shift/ctrl modifiers to a marquee-drag `MergeMode` (ctrl+shift → invertive).
 * `persistentMode` — a shell's own {@link SelectionModeStore} value, e.g. `useShellScope().selection.get()`
 * — takes precedence when set to something other than `"replace"`, mirroring the toolbar toggle in
 * `SelectionUtilityOptions`; omitted, behaves as if the toolbar is at its `"replace"` setting (was: read a
 * page-global `(globalThis).__selectionMode`, so one shell's toolbar choice silently applied to every
 * other mounted shell's marquee gestures too). ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM
 * W3a: unified onto the `🕹️interaction` module's `MergeMode` (was a bespoke `SelectionMergeMode` union
 * with `"default"` instead of `"replace"`) — never produces `"range"`, since a marquee drag has no
 * ordered/anchored topology to range over (unlike `interactionMergeFromModifiers`'s click-select shift
 * mapping below). */
export function marqueeModeFromModifiers(modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean }, persistentMode?: MergeMode): MergeMode {
  if (persistentMode && persistentMode !== "replace") {
    return persistentMode;
  }
  const shift = modifiers.shiftKey === true;
  const ctrl = modifiers.ctrlKey === true || modifiers.metaKey === true;
  if (shift && ctrl) return "invertive";
  if (shift) return "additive";
  if (ctrl) return "subtractive";
  return "replace";
}

/** 🎯️ Applies a marquee-drag `MergeMode` when committing ids — `"range"` has no meaning without
 * an ordered topology here (see `marqueeModeFromModifiers`'s doc), so it falls back to `"replace"`. */
export function selectionMergeIds(mode: MergeMode, current: readonly string[], incoming: readonly string[]): string[] {
  const currentSet = new Set(current);
  const incomingSet = new Set(incoming);
  if (mode === "replace" || mode === "range") return [...incomingSet];
  if (mode === "additive") {
    for (const id of incomingSet) currentSet.add(id);
    return [...currentSet];
  }
  if (mode === "subtractive") {
    for (const id of incomingSet) currentSet.delete(id);
    return [...currentSet];
  }
  for (const id of incomingSet) {
    if (currentSet.has(id)) currentSet.delete(id);
    else currentSet.add(id);
  }
  return [...currentSet];
}

export type ScreenRect = { readonly x: number; readonly y: number; readonly width: number; readonly height: number };

export function screenRectContainsRect(outer: ScreenRect, inner: ScreenRect): boolean {
  return inner.x >= outer.x && inner.y >= outer.y && inner.x + inner.width <= outer.x + outer.width && inner.y + inner.height <= outer.y + outer.height;
}

export function screenRectIntersectsRect(a: ScreenRect, b: ScreenRect): boolean {
  return a.x <= b.x + b.width && a.x + a.width >= b.x && a.y <= b.y + b.height && a.y + a.height >= b.y;
}

export function screenRectFromPoints(points: readonly { readonly x: number; readonly y: number }[]): ScreenRect | null {
  if (!points.length) return null;
  const xs = points.map((point) => point.x);
  const ys = points.map((point) => point.y);
  const minX = Math.min(...xs);
  const minY = Math.min(...ys);
  const maxX = Math.max(...xs);
  const maxY = Math.max(...ys);
  return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}
