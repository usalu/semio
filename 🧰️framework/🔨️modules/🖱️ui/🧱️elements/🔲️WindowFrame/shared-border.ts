import { resolveBorderSegments, type BorderPoint, type BorderSegment, type BorderState } from "../../🔨️modules/📏️border-presentation/🟦️.ts";
import { WINDOW_FRAME_CHIP_EPSILON, WINDOW_FRAME_PATH_INSET, type WindowFrameChipRegion } from "./🟦️.tsx";

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
  readonly follows?: true;
}

export interface SharedBorderClip {
  readonly left: number;
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
}

/** ⚓ Visible state-only edges can recolour a permanent line without shifting it. */
export function leadingSharedOutlineSegments(sources: readonly SharedOutlineSegment[]): readonly SharedOutlineSegment[] {
  const permanent = sources.filter(source => !source.follows);
  return permanent.length ? permanent : sources;
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
      const values = leadingSharedOutlineSegments(contributors).map(source => (horizontal ? source.transform.top : source.transform.left) + source.paintFrom[normal] * (horizontal ? source.transform.scaleY : source.transform.scaleX));
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
      const values = normalSources.length ? leadingSharedOutlineSegments(normalSources).map(source => screen(source, source.paintFrom)) : leadingSharedOutlineSegments(tangentSources).flatMap(source => {
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

/** 🖌️ Connects an ordered span's snapped junctions to its centerline. Crossing junctions consume the span, and an isolated move does not paint.
 * @see ./junctions.cases.json
 */
export function sharedBorderPaintPoints(from: BorderPoint, to: BorderPoint, normal: "x" | "y", centerline: number): BorderPoint[] {
  const tangent = normal === "x" ? "y" : "x";
  if (from[tangent] > to[tangent]) return [];
  const points = [from, { ...from, [normal]: centerline }, { ...to, [normal]: centerline }, to]
    .filter((point, index, all) => index === 0 || point.x !== all[index - 1]!.x || point.y !== all[index - 1]!.y);
  return points.length > 1 ? points : [];
}

/** 📍️ Projects split logical edges onto the unchanged inset frame stroke. */
export function projectSharedBorderPoint(point: BorderPoint, from: BorderPoint, to: BorderPoint, paintFrom: BorderPoint, paintTo: BorderPoint): BorderPoint {
  const ratio = from.x === to.x ? (point.y - from.y) / (to.y - from.y) : (point.x - from.x) / (to.x - from.x);
  return { x: paintFrom.x + ratio * (paintTo.x - paintFrom.x), y: paintFrom.y + ratio * (paintTo.y - paintFrom.y) };
}

/** 🧲️ Maps a viewport coordinate to its edge cluster: the box edges of one commit lying within `tolerance` px of the cluster's first edge are one line (layout leftovers such as a scroller's rounded scroll range leave edges a fraction of a pixel apart that must still share one line). */
export type SharedBorderSnap = (axis: "x" | "y", value: number) => number;

/** 🧲️ Builds the {@link SharedBorderSnap} of the box edges `xs` and `ys`; pure, language-agnostic (see `snap.cases.json`). */
export function createSharedBorderSnap(xs: readonly number[], ys: readonly number[], tolerance = 0.75): SharedBorderSnap {
  const cluster = (values: readonly number[]) => {
    const sorted = [...new Set(values.map(value => Math.round(value * 64) / 64))].sort((a, b) => a - b);
    const heads: number[] = [], members: number[] = [];
    for (const value of sorted) {
      if (!heads.length || value - heads[heads.length - 1]! > tolerance) heads.push(value);
      members.push(heads[heads.length - 1]!);
    }
    return { sorted, members };
  };
  const axes = { x: cluster(xs), y: cluster(ys) };
  return (axis, value) => {
    const { sorted, members } = axes[axis];
    let low = 0, high = sorted.length - 1, best = -1;
    while (low <= high) {
      const mid = (low + high) >> 1;
      if (Math.abs(sorted[mid]! - value) <= tolerance && (best < 0 || Math.abs(sorted[mid]! - value) < Math.abs(sorted[best]! - value))) best = mid;
      if (sorted[mid]! < value) low = mid + 1; else high = mid - 1;
    }
    for (const index of [best - 1, best + 1]) if (index >= 0 && index < sorted.length && Math.abs(sorted[index]! - value) < Math.abs(sorted[best]! - value)) best = index;
    return best < 0 ? value : members[best]!;
  };
}

/** 🧩️ Registers the four frame edges and the inner edges of every chip region in snapped viewport coordinates; `logical` and `paint` are the frame's corner rings, chip regions share the frame's local space and paint their edges one inset inside the chip. `source` names the box whose edges these are when it differs from the painting `owner` (a cell painted in its owner's overlay).
 * @see ../🔲️WindowFrame/frame.cases.json
 */
export function sharedOutlineSegments(owner: string, logical: readonly BorderPoint[], paint: readonly BorderPoint[], state: BorderState, transform: SharedBorderTransform, snap?: SharedBorderSnap, source = owner, chips: readonly WindowFrameChipRegion[] = []): SharedOutlineSegment[] {
  if (logical.length !== paint.length) return [];
  const screen = (point: BorderPoint): BorderPoint => {
    const x = Math.round((transform.left + point.x * transform.scaleX) * 64) / 64, y = Math.round((transform.top + point.y * transform.scaleY) * 64) / 64;
    return snap ? { x: snap("x", x), y: snap("y", y) } : { x, y };
  };
  const edge = (index: number, point: BorderPoint, next: BorderPoint, paintFrom: BorderPoint, paintTo: BorderPoint): SharedOutlineSegment[] => {
    const from = screen(point), end = screen(next);
    const to = { x: Math.abs(point.x - next.x) <= WINDOW_FRAME_CHIP_EPSILON ? from.x : end.x,
      y: Math.abs(point.y - next.y) <= WINDOW_FRAME_CHIP_EPSILON ? from.y : end.y };
    if (from.x === to.x && from.y === to.y) return [];
    return [{ id: `${source}:${index}`, owner, from, to, state, logicalFrom: from, logicalTo: to, paintFrom, paintTo, transform }];
  };
  const frame = logical.flatMap((point, index) => edge(index, point, logical[(index + 1) % logical.length]!, paint[index]!, paint[(index + 1) % paint.length]!));
  if (!chips.length || !logical.length) return frame;
  const minX = Math.min(...logical.map(point => point.x)), maxX = Math.max(...logical.map(point => point.x));
  const inner = chips.flatMap((chip, chipIndex) => {
    const x0 = chip.x, x1 = chip.x + chip.width, y0 = chip.y, y1 = chip.y + chip.height, i = Math.min(WINDOW_FRAME_PATH_INSET, chip.width / 2, chip.height / 2);
    const corners: BorderPoint[] = [{ x: x0, y: y0 }, { x: x1, y: y0 }, { x: x1, y: y1 }, { x: x0, y: y1 }];
    const insets: BorderPoint[] = [{ x: x0 + i, y: y0 + i }, { x: x1 - i, y: y0 + i }, { x: x1 - i, y: y1 - i }, { x: x0 + i, y: y1 - i }];
    const sides = [chip.dock === "bottom", x1 < maxX - WINDOW_FRAME_CHIP_EPSILON, chip.dock === "top", x0 > minX + WINDOW_FRAME_CHIP_EPSILON];
    return sides.flatMap((present, side) => present ? edge(logical.length + chipIndex * 4 + side, corners[side]!, corners[(side + 1) % 4]!, insets[side]!, insets[(side + 1) % 4]!) : []);
  });
  return [...frame, ...inner];
}
