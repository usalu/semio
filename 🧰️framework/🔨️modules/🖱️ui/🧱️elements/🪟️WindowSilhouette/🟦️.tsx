// #region Header
// framework/ui/elements/🪟️WindowSilhouette/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// Licensed under LGPL-3.0-or-later.
// #endregion Header

//#region 🪟️WindowSilhouetteGeometry
export const WINDOW_SILHOUETTE_GEOMETRY_SCHEMA = "window-silhouette-geometry/v1" as const;

export interface WindowSilhouetteChip {
  readonly left: number;
  readonly right: number;
}

export interface WindowSilhouetteEdge {
  readonly depth: number;
  readonly chips: readonly WindowSilhouetteChip[];
}

export interface WindowSilhouetteMetrics {
  readonly width: number;
  readonly height: number;
  readonly top: WindowSilhouetteEdge;
  readonly bottom: WindowSilhouetteEdge;
}

export interface WindowSilhouettePoint {
  readonly x: number;
  readonly y: number;
}

export type WindowSilhouetteDock = "top" | "bottom";

export interface WindowSilhouetteRegion {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly kind: "body" | "chip";
  readonly dock?: WindowSilhouetteDock;
}

export interface WindowSilhouetteSafeClearances {
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
  readonly left: number;
}

export interface PendingWindowSilhouetteMetrics {
  readonly width: number;
  readonly height: number;
  readonly topClearance: number;
  readonly bottomClearance: number;
}

export interface WindowSilhouetteGeometry {
  readonly schema: typeof WINDOW_SILHOUETTE_GEOMETRY_SCHEMA;
  readonly state: "pending" | "ready";
  readonly metrics: WindowSilhouetteMetrics;
  readonly outline: readonly WindowSilhouettePoint[];
  readonly contentPath: string;
  readonly contentClipPath: string;
  readonly borderPath: string;
  readonly bodyRegion: WindowSilhouetteRegion | null;
  readonly contentRegions: readonly WindowSilhouetteRegion[];
  readonly glassRegions: readonly WindowSilhouetteRegion[];
  readonly safeClearances: WindowSilhouetteSafeClearances;
}

export const WINDOW_SILHOUETTE_PATH_INSET = 1;
export const WINDOW_SILHOUETTE_CHIP_EPSILON = 0.5;

function finiteWindowSilhouetteCoordinate(value: number): number {
  return Number.isFinite(value) ? Math.max(0, value) : 0;
}

export function normalizeWindowSilhouetteChips(chips: readonly WindowSilhouetteChip[], x0: number, x1: number): WindowSilhouetteChip[] {
  const bound0 = Number.isFinite(x0) ? x0 : 0;
  const bound1 = Number.isFinite(x1) ? x1 : bound0;
  const minX = Math.min(bound0, bound1);
  const maxX = Math.max(bound0, bound1);
  const normalized = chips
    .filter((chip) => Number.isFinite(chip.left) && Number.isFinite(chip.right) && chip.right > chip.left)
    .map((chip) => ({ left: Math.max(minX, Math.min(chip.left, maxX)), right: Math.max(minX, Math.min(chip.right, maxX)) }))
    .filter((chip) => chip.right - chip.left > WINDOW_SILHOUETTE_CHIP_EPSILON)
    .sort((a, b) => a.left - b.left || a.right - b.right);
  const merged: { left: number; right: number }[] = [];
  for (const chip of normalized) {
    const last = merged[merged.length - 1];
    if (!last || chip.left > last.right + WINDOW_SILHOUETTE_CHIP_EPSILON) merged.push({ ...chip });
    else last.right = Math.max(last.right, chip.right);
  }
  return merged;
}

export function normalizeWindowSilhouetteMetrics(metrics: WindowSilhouetteMetrics): WindowSilhouetteMetrics {
  const width = finiteWindowSilhouetteCoordinate(metrics.width);
  const height = finiteWindowSilhouetteCoordinate(metrics.height);
  const topDepth = Math.min(finiteWindowSilhouetteCoordinate(metrics.top.depth), height);
  const bottomDepth = Math.min(finiteWindowSilhouetteCoordinate(metrics.bottom.depth), Math.max(0, height - topDepth));
  return {
    width,
    height,
    top: { depth: topDepth, chips: normalizeWindowSilhouetteChips(metrics.top.chips, 0, width) },
    bottom: { depth: bottomDepth, chips: normalizeWindowSilhouetteChips(metrics.bottom.chips, 0, width) },
  };
}

export function windowSilhouetteEdgePoints(edge: WindowSilhouetteEdge, x0: number, x1: number, outer: number, inner: number): WindowSilhouettePoint[] {
  const chips = normalizeWindowSilhouetteChips(edge.chips, x0, x1);
  if (edge.depth <= WINDOW_SILHOUETTE_CHIP_EPSILON) return [{ x: x0, y: outer }, { x: x1, y: outer }];
  if (chips.length === 0) return [{ x: x0, y: inner }, { x: x1, y: inner }];
  const first = chips[0]!;
  const startsWithChip = first.left <= x0 + WINDOW_SILHOUETTE_CHIP_EPSILON;
  let x = x0;
  let y = startsWithChip ? outer : inner;
  const points: WindowSilhouettePoint[] = [{ x, y }];
  const push = (nextX: number, nextY: number) => {
    if (Math.abs(nextX - x) <= WINDOW_SILHOUETTE_CHIP_EPSILON && Math.abs(nextY - y) <= WINDOW_SILHOUETTE_CHIP_EPSILON) return;
    points.push({ x: nextX, y: nextY });
    x = nextX;
    y = nextY;
  };
  if (!startsWithChip) {
    push(first.left, inner);
    push(first.left, outer);
  }
  for (let i = 0; i < chips.length; i++) {
    const chip = chips[i]!;
    const hasNext = i < chips.length - 1;
    push(chip.right, outer);
    if (hasNext) {
      const nextLeft = chips[i + 1]!.left;
      push(chip.right, inner);
      if (chip.right < nextLeft - WINDOW_SILHOUETTE_CHIP_EPSILON) push(nextLeft, inner);
      push(nextLeft, outer);
    } else if (chip.right < x1 - WINDOW_SILHOUETTE_CHIP_EPSILON) {
      push(chip.right, inner);
      push(x1, inner);
    }
  }
  return points;
}

export function windowSilhouetteEdgePointsRtl(edge: WindowSilhouetteEdge, x0: number, x1: number, outer: number, inner: number): WindowSilhouettePoint[] {
  return [...windowSilhouetteEdgePoints(edge, x0, x1, outer, inner)].reverse();
}

export function windowSilhouetteOutline(metrics: WindowSilhouetteMetrics, inset = WINDOW_SILHOUETTE_PATH_INSET): WindowSilhouettePoint[] {
  const normalized = normalizeWindowSilhouetteMetrics(metrics);
  const safeInset = Math.min(finiteWindowSilhouetteCoordinate(inset), normalized.width * 0.5, normalized.height * 0.5);
  const x0 = safeInset;
  const y0 = safeInset;
  const x1 = normalized.width - safeInset;
  const y1 = normalized.height - safeInset;
  const topInner = Math.max(y0, Math.min(y0 + normalized.top.depth, y1));
  const bottomInner = Math.max(y0, Math.min(y1 - normalized.bottom.depth, y1));
  const top = windowSilhouetteEdgePoints(normalized.top, x0, x1, y0, topInner);
  const bottom = windowSilhouetteEdgePointsRtl(normalized.bottom, x0, x1, y1, bottomInner);
  return simplifyWindowSilhouetteOutline([...top, ...bottom]);
}

export function simplifyWindowSilhouetteOutline(points: readonly WindowSilhouettePoint[]): WindowSilhouettePoint[] {
  if (points.length <= 2) return [...points];
  const deduped: WindowSilhouettePoint[] = [];
  for (const point of points) {
    const last = deduped[deduped.length - 1];
    if (!last || Math.abs(last.x - point.x) > WINDOW_SILHOUETTE_CHIP_EPSILON || Math.abs(last.y - point.y) > WINDOW_SILHOUETTE_CHIP_EPSILON) deduped.push(point);
  }
  if (deduped.length <= 2) return deduped;
  const simplified: WindowSilhouettePoint[] = [];
  const count = deduped.length;
  for (let i = 0; i < count; i++) {
    const prev = deduped[(i - 1 + count) % count]!;
    const curr = deduped[i]!;
    const next = deduped[(i + 1) % count]!;
    const horizontal = Math.abs(prev.y - curr.y) <= WINDOW_SILHOUETTE_CHIP_EPSILON && Math.abs(curr.y - next.y) <= WINDOW_SILHOUETTE_CHIP_EPSILON;
    const vertical = Math.abs(prev.x - curr.x) <= WINDOW_SILHOUETTE_CHIP_EPSILON && Math.abs(curr.x - next.x) <= WINDOW_SILHOUETTE_CHIP_EPSILON;
    if (horizontal || vertical) continue;
    simplified.push(curr);
  }
  return simplified.length > 0 ? simplified : [...deduped];
}

export function windowSilhouetteOutlineViolations(points: readonly WindowSilhouettePoint[], bounds?: { readonly x0: number; readonly y0: number; readonly x1: number; readonly y1: number }): string[] {
  if (points.length < 3) return ["outline requires at least three vertices"];
  const violations: string[] = [];
  const count = points.length;
  for (let i = 0; i < count; i++) {
    const prev = points[(i - 1 + count) % count]!;
    const curr = points[i]!;
    const next = points[(i + 1) % count]!;
    const dx = curr.x - prev.x;
    const dy = curr.y - prev.y;
    if (Math.abs(dx) <= WINDOW_SILHOUETTE_CHIP_EPSILON && Math.abs(dy) <= WINDOW_SILHOUETTE_CHIP_EPSILON) violations.push(`zero-length segment at vertex ${i}`);
    if (Math.abs(dx) > WINDOW_SILHOUETTE_CHIP_EPSILON && Math.abs(dy) > WINDOW_SILHOUETTE_CHIP_EPSILON) violations.push(`non-axis-aligned segment at vertex ${i}`);
    const ndx = next.x - curr.x;
    const ndy = next.y - curr.y;
    if (Math.abs(dx) > WINDOW_SILHOUETTE_CHIP_EPSILON && Math.abs(ndx) > WINDOW_SILHOUETTE_CHIP_EPSILON && Math.sign(dx) !== Math.sign(ndx)) violations.push(`horizontal reversal at vertex ${i}`);
    if (Math.abs(dy) > WINDOW_SILHOUETTE_CHIP_EPSILON && Math.abs(ndy) > WINDOW_SILHOUETTE_CHIP_EPSILON && Math.sign(dy) !== Math.sign(ndy)) violations.push(`vertical reversal at vertex ${i}`);
    const horizontal = Math.abs(prev.y - curr.y) <= WINDOW_SILHOUETTE_CHIP_EPSILON && Math.abs(curr.y - next.y) <= WINDOW_SILHOUETTE_CHIP_EPSILON;
    const vertical = Math.abs(prev.x - curr.x) <= WINDOW_SILHOUETTE_CHIP_EPSILON && Math.abs(curr.x - next.x) <= WINDOW_SILHOUETTE_CHIP_EPSILON;
    if (horizontal || vertical) violations.push(`collinear vertex at ${i}`);
    if (bounds && (curr.x < bounds.x0 - WINDOW_SILHOUETTE_CHIP_EPSILON || curr.x > bounds.x1 + WINDOW_SILHOUETTE_CHIP_EPSILON || curr.y < bounds.y0 - WINDOW_SILHOUETTE_CHIP_EPSILON || curr.y > bounds.y1 + WINDOW_SILHOUETTE_CHIP_EPSILON)) violations.push(`vertex ${i} outside bounds`);
  }
  return violations;
}

export function windowSilhouettePathFromOutline(points: readonly WindowSilhouettePoint[]): string {
  if (points.length === 0) return "";
  const parts: string[] = [`M${points[0]!.x},${points[0]!.y}`];
  for (let i = 1; i < points.length; i++) {
    const prev = points[i - 1]!;
    const curr = points[i]!;
    if (Math.abs(curr.x - prev.x) > WINDOW_SILHOUETTE_CHIP_EPSILON) parts.push(`H${curr.x}`);
    if (Math.abs(curr.y - prev.y) > WINDOW_SILHOUETTE_CHIP_EPSILON) parts.push(`V${curr.y}`);
  }
  parts.push("Z");
  return parts.join(" ");
}

export function windowSilhouettePath(metrics: WindowSilhouetteMetrics, inset = WINDOW_SILHOUETTE_PATH_INSET): string {
  return windowSilhouettePathFromOutline(windowSilhouetteOutline(metrics, inset));
}

function serializeWindowSilhouetteCoordinate(value: number): string {
  return Number(finiteWindowSilhouetteCoordinate(value).toFixed(3)).toString();
}

export function windowSilhouetteContentClipPath(metrics: WindowSilhouetteMetrics): string {
  const outline = windowSilhouetteOutline(metrics, 0);
  if (outline.length < 3) return "inset(100%)";
  return `polygon(${outline.map((point) => `${serializeWindowSilhouetteCoordinate(point.x)}px ${serializeWindowSilhouetteCoordinate(point.y)}px`).join(", ")})`;
}

export function windowSilhouetteSafeClearances(metrics: WindowSilhouetteMetrics): WindowSilhouetteSafeClearances {
  const normalized = normalizeWindowSilhouetteMetrics(metrics);
  return { top: normalized.top.depth, right: 0, bottom: normalized.bottom.depth, left: 0 };
}

export function windowSilhouetteBodyRegion(metrics: WindowSilhouetteMetrics): WindowSilhouetteRegion | null {
  const normalized = normalizeWindowSilhouetteMetrics(metrics);
  const height = Math.max(0, normalized.height - normalized.top.depth - normalized.bottom.depth);
  if (normalized.width <= WINDOW_SILHOUETTE_CHIP_EPSILON || height <= WINDOW_SILHOUETTE_CHIP_EPSILON) return null;
  return { x: 0, y: normalized.top.depth, width: normalized.width, height, kind: "body" };
}

export function windowSilhouetteGlassRegions(metrics: WindowSilhouetteMetrics): WindowSilhouetteRegion[] {
  const normalized = normalizeWindowSilhouetteMetrics(metrics);
  const regions: WindowSilhouetteRegion[] = [];
  if (normalized.top.depth > WINDOW_SILHOUETTE_CHIP_EPSILON) {
    for (const chip of normalized.top.chips) regions.push({ x: chip.left, y: 0, width: chip.right - chip.left, height: normalized.top.depth, kind: "chip", dock: "top" });
  }
  if (normalized.bottom.depth > WINDOW_SILHOUETTE_CHIP_EPSILON) {
    const y = normalized.height - normalized.bottom.depth;
    for (const chip of normalized.bottom.chips) regions.push({ x: chip.left, y, width: chip.right - chip.left, height: normalized.bottom.depth, kind: "chip", dock: "bottom" });
  }
  return regions;
}

export function windowSilhouetteContentRegions(metrics: WindowSilhouetteMetrics): WindowSilhouetteRegion[] {
  const body = windowSilhouetteBodyRegion(metrics);
  return body ? [body, ...windowSilhouetteGlassRegions(metrics)] : windowSilhouetteGlassRegions(metrics);
}

export function windowSilhouetteRegionContains(region: WindowSilhouetteRegion, x: number, y: number): boolean {
  if (!Number.isFinite(x) || !Number.isFinite(y)) return false;
  return x >= region.x && x <= region.x + region.width && y >= region.y && y <= region.y + region.height;
}

export function windowSilhouetteContains(metrics: WindowSilhouetteMetrics, x: number, y: number): boolean {
  return windowSilhouetteContentRegions(metrics).some((region) => windowSilhouetteRegionContains(region, x, y));
}

export function pendingWindowSilhouetteMetrics(pending?: Partial<PendingWindowSilhouetteMetrics>): WindowSilhouetteMetrics {
  return normalizeWindowSilhouetteMetrics({
    width: pending?.width ?? 0,
    height: pending?.height ?? 0,
    top: { depth: pending?.topClearance ?? 0, chips: [] },
    bottom: { depth: pending?.bottomClearance ?? 0, chips: [] },
  });
}

export function createWindowSilhouetteGeometry(metrics: WindowSilhouetteMetrics | null, pending?: Partial<PendingWindowSilhouetteMetrics>): WindowSilhouetteGeometry {
  const measured = metrics ? normalizeWindowSilhouetteMetrics(metrics) : null;
  const ready = measured !== null && measured.width > WINDOW_SILHOUETTE_CHIP_EPSILON && measured.height > WINDOW_SILHOUETTE_CHIP_EPSILON;
  const normalized = ready && measured ? measured : pendingWindowSilhouetteMetrics(pending);
  const outline = windowSilhouetteOutline(normalized, 0);
  const bodyRegion = windowSilhouetteBodyRegion(normalized);
  const glassRegions = ready ? windowSilhouetteGlassRegions(normalized) : [];
  const contentRegions = bodyRegion ? [bodyRegion, ...glassRegions] : [...glassRegions];
  return {
    schema: WINDOW_SILHOUETTE_GEOMETRY_SCHEMA,
    state: ready ? "ready" : "pending",
    metrics: normalized,
    outline,
    contentPath: windowSilhouettePathFromOutline(outline),
    contentClipPath: windowSilhouetteContentClipPath(normalized),
    borderPath: windowSilhouettePath(normalized),
    bodyRegion,
    contentRegions,
    glassRegions,
    safeClearances: windowSilhouetteSafeClearances(normalized),
  };
}
//#endregion 🪟️WindowSilhouetteGeometry
