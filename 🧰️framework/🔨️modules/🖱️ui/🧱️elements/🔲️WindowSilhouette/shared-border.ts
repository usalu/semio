import { resolveBorderSegments, type BorderPoint, type BorderSegment, type BorderState } from "../../🔨️modules/📏️border-presentation/🟦️.ts";
import { WINDOW_SILHOUETTE_CHIP_EPSILON } from "./🟦️.tsx";

/** 🔍️ Shares measurement's scale policy: integer offset-size rounding is not a transform. */
export function sharedBorderAxisScale(painted: number, layout: number): number {
  return layout > 0 && Math.abs(painted - layout) > 1 ? painted / layout : 1;
}

export interface SharedBorderTransform {
  readonly left: number;
  readonly top: number;
  readonly scaleX: number;
  readonly scaleY: number;
}

export interface SharedOutlineSegment extends BorderSegment {
  readonly logicalFrom: BorderPoint;
  readonly logicalTo: BorderPoint;
  readonly paintFrom: BorderPoint;
  readonly paintTo: BorderPoint;
  readonly transform: SharedBorderTransform;
  readonly clip?: SharedBorderClip;
}

export interface SharedBorderClip {
  readonly left: number;
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
}

/** ✂️ Excludes invisible source paint before it can suppress a visible incident owner. */
export function clipSharedOutlineSegments(sources: readonly SharedOutlineSegment[], clip: SharedBorderClip): SharedOutlineSegment[] {
  return sources.flatMap(source => {
    const { transform: t } = source;
    const a = { x: t.left + source.paintFrom.x * t.scaleX, y: t.top + source.paintFrom.y * t.scaleY };
    const horizontal = source.from.y === source.to.y;
    if (horizontal ? a.y < clip.top || a.y > clip.bottom : a.x < clip.left || a.x > clip.right) return [];
    const low = horizontal ? clip.left : clip.top, high = horizontal ? clip.right : clip.bottom;
    if (low > high) return [];
    const axis = horizontal ? "x" : "y";
    const start = source.logicalFrom[axis], end = source.logicalTo[axis];
    const ratios = [(source.from[axis] - start) / (end - start), (source.to[axis] - start) / (end - start)];
    const limits = [(low - start) / (end - start), (high - start) / (end - start)];
    const from = Math.max(0, Math.min(...ratios), Math.min(...limits));
    const to = Math.min(1, Math.max(...ratios), Math.max(...limits));
    if (from >= to) return [];
    const point = (ratio: number): BorderPoint => ({ x: source.logicalFrom.x + ratio * (source.logicalTo.x - source.logicalFrom.x), y: source.logicalFrom.y + ratio * (source.logicalTo.y - source.logicalFrom.y) });
    return [{ ...source, from: point(from), to: point(to) }];
  });
}

/** 👁️ Removes clipped candidates at the canonical centerline before resolving paint ownership. */
export function visibleSharedOutlineSegments(input: readonly SharedOutlineSegment[]): SharedOutlineSegment[] {
  let sources = [...input];
  if (!sources.some(source => source.clip && Object.values(source.clip).some(Number.isFinite))) return sources;
  for (;;) {
    const byId = new Map(sources.map(source => [source.id, source]));
    const next: SharedOutlineSegment[] = [];
    let removed = false;
    for (const span of resolveBorderSegments(sources)) {
      const horizontal = span.from.y === span.to.y, normal = horizontal ? "y" : "x";
      const contributors = span.contributors.map(id => byId.get(id)!);
      const values = contributors.map(source => (horizontal ? source.transform.top : source.transform.left) + source.paintFrom[normal] * (horizontal ? source.transform.scaleY : source.transform.scaleX));
      const centerline = (Math.min(...values) + Math.max(...values)) / 2;
      for (const source of contributors) {
        const clip = source.clip;
        if (clip && (horizontal ? centerline < clip.top || centerline > clip.bottom : centerline < clip.left || centerline > clip.right)) { removed = true; continue; }
        next.push({ ...source, id: `${source.id}:${next.length}`, from: span.from, to: span.to });
      }
    }
    if (!removed) return sources;
    sources = next;
  }
}

/** 🕸️ Intersects canonical incident paint lines, independent of winning state or registration order. */
export function createSharedBorderJunctionResolver(sources: readonly SharedOutlineSegment[]): (point: BorderPoint) => BorderPoint {
  const horizontal = new Map<number, SharedOutlineSegment[]>(), vertical = new Map<number, SharedOutlineSegment[]>();
  for (const source of sources) {
    const h = source.from.y === source.to.y, lines = h ? horizontal : vertical, coordinate = h ? source.from.y : source.from.x;
    const list = lines.get(coordinate) ?? []; list.push(source); lines.set(coordinate, list);
  }
  const cache = new Map<string, BorderPoint>();
  return point => {
    const key = `${point.x}:${point.y}`, previous = cache.get(key);
    if (previous) return previous;
    const incident = (lines: Map<number, SharedOutlineSegment[]>, normal: "x" | "y", tangent: "x" | "y") => (lines.get(point[normal]) ?? []).filter(source => point[tangent] >= Math.min(source.from[tangent], source.to[tangent]) && point[tangent] <= Math.max(source.from[tangent], source.to[tangent]));
    const hs = incident(horizontal, "y", "x"), vs = incident(vertical, "x", "y");
    const coordinate = (normalSources: readonly SharedOutlineSegment[], tangentSources: readonly SharedOutlineSegment[], normal: "x" | "y"): number => {
      const screen = (source: SharedOutlineSegment, paint: BorderPoint) => source.transform[normal === "x" ? "left" : "top"] + paint[normal] * source.transform[normal === "x" ? "scaleX" : "scaleY"];
      const values = normalSources.length ? normalSources.map(source => screen(source, source.paintFrom)) : tangentSources.flatMap(source => {
        if (point.x === source.logicalFrom.x && point.y === source.logicalFrom.y) return [screen(source, source.paintFrom)];
        if (point.x === source.logicalTo.x && point.y === source.logicalTo.y) return [screen(source, source.paintTo)];
        return [point[normal]];
      });
      return values.length ? (Math.min(...values) + Math.max(...values)) / 2 : point[normal];
    };
    const result = { x: coordinate(vs, hs, "x"), y: coordinate(hs, vs, "y") };
    cache.set(key, result); return result;
  };
}

/** 🪢️ Pure junction convenience entry for fixtures and independent implementations. */
export function sharedBorderJunction(point: BorderPoint, sources: readonly SharedOutlineSegment[]): BorderPoint {
  return createSharedBorderJunctionResolver(sources)(point);
}

/** 📍️ Projects split logical edges onto the unchanged inset silhouette stroke. */
export function projectSharedBorderPoint(point: BorderPoint, from: BorderPoint, to: BorderPoint, paintFrom: BorderPoint, paintTo: BorderPoint): BorderPoint {
  const ratio = from.x === to.x ? (point.y - from.y) / (to.y - from.y) : (point.x - from.x) / (to.x - from.x);
  return { x: paintFrom.x + ratio * (paintTo.x - paintFrom.x), y: paintFrom.y + ratio * (paintTo.y - paintFrom.y) };
}

/** 🧩️ Registers every original rectilinear silhouette edge in snapped viewport coordinates. */
export function sharedOutlineSegments(owner: string, logical: readonly BorderPoint[], paint: readonly BorderPoint[], state: BorderState, transform: SharedBorderTransform): SharedOutlineSegment[] {
  if (logical.length !== paint.length) return [];
  const screen = (point: BorderPoint): BorderPoint => ({ x: Math.round((transform.left + point.x * transform.scaleX) * 64) / 64, y: Math.round((transform.top + point.y * transform.scaleY) * 64) / 64 });
  return logical.flatMap((point, index) => {
    const next = logical[(index + 1) % logical.length]!;
    const from = screen(point), end = screen(next);
    const to = { x: Math.abs(point.x - next.x) <= WINDOW_SILHOUETTE_CHIP_EPSILON ? from.x : end.x,
      y: Math.abs(point.y - next.y) <= WINDOW_SILHOUETTE_CHIP_EPSILON ? from.y : end.y };
    if (from.x === to.x && from.y === to.y) return [];
    return [{ id: `${owner}:${index}`, owner, from, to, state, logicalFrom: from, logicalTo: to, paintFrom: paint[index]!, paintTo: paint[(index + 1) % paint.length]!, transform }];
  });
}
