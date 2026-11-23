// #region Header
// framework/ui/elements/🔲️WindowFrame/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// Licensed under LGPL-3.0-or-later.
// #endregion Header

//#region 🔲️WindowFrameGeometry
export const WINDOW_FRAME_GEOMETRY_SCHEMA = "window-frame-geometry/v1" as const;
export const WINDOW_FRAME_PATH_INSET = 1;
export const WINDOW_FRAME_CHIP_EPSILON = 0.5;

export interface WindowFrameChip {
  readonly left: number;
  readonly right: number;
}

export interface WindowFrameEdge {
  readonly depth: number;
  readonly chips: readonly WindowFrameChip[];
}

export interface WindowFrameMetrics {
  readonly width: number;
  readonly height: number;
  readonly top: WindowFrameEdge;
  readonly bottom: WindowFrameEdge;
}

export type WindowFrameDock = "top" | "bottom";

export interface WindowFrameRect {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

export interface WindowFrameChipRegion extends WindowFrameRect {
  readonly dock: WindowFrameDock;
}

export interface WindowFrameSafeClearances {
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
  readonly left: number;
}

export interface PendingWindowFrameMetrics {
  readonly width: number;
  readonly height: number;
  readonly topClearance: number;
  readonly bottomClearance: number;
}

export interface WindowFrameGeometry {
  readonly schema: typeof WINDOW_FRAME_GEOMETRY_SCHEMA;
  readonly state: "pending" | "ready";
  readonly metrics: WindowFrameMetrics;
  readonly frame: WindowFrameRect;
  readonly borderPath: string;
  readonly chipRegions: readonly WindowFrameChipRegion[];
  readonly safeClearances: WindowFrameSafeClearances;
}

function finiteWindowFrameCoordinate(value: number): number {
  return Number.isFinite(value) ? Math.max(0, value) : 0;
}

/** 🔲️ Clamps, sorts and merges chip spans of one frame edge; spans not wider than the epsilon vanish.
 * @see ./frame.cases.json
 */
export function normalizeWindowFrameChips(chips: readonly WindowFrameChip[], x0: number, x1: number): WindowFrameChip[] {
  const bound0 = Number.isFinite(x0) ? x0 : 0;
  const bound1 = Number.isFinite(x1) ? x1 : bound0;
  const minX = Math.min(bound0, bound1);
  const maxX = Math.max(bound0, bound1);
  const normalized = chips
    .filter((chip) => Number.isFinite(chip.left) && Number.isFinite(chip.right) && chip.right > chip.left)
    .map((chip) => ({ left: Math.max(minX, Math.min(chip.left, maxX)), right: Math.max(minX, Math.min(chip.right, maxX)) }))
    .filter((chip) => chip.right - chip.left > WINDOW_FRAME_CHIP_EPSILON)
    .sort((a, b) => a.left - b.left || a.right - b.right);
  const merged: { left: number; right: number }[] = [];
  for (const chip of normalized) {
    const last = merged[merged.length - 1];
    if (!last || chip.left > last.right + WINDOW_FRAME_CHIP_EPSILON) merged.push({ ...chip });
    else last.right = Math.max(last.right, chip.right);
  }
  return merged;
}

export function normalizeWindowFrameMetrics(metrics: WindowFrameMetrics): WindowFrameMetrics {
  const width = finiteWindowFrameCoordinate(metrics.width);
  const height = finiteWindowFrameCoordinate(metrics.height);
  const topDepth = Math.min(finiteWindowFrameCoordinate(metrics.top.depth), height);
  const bottomDepth = Math.min(finiteWindowFrameCoordinate(metrics.bottom.depth), Math.max(0, height - topDepth));
  return {
    width,
    height,
    top: { depth: topDepth, chips: normalizeWindowFrameChips(metrics.top.chips, 0, width) },
    bottom: { depth: bottomDepth, chips: normalizeWindowFrameChips(metrics.bottom.chips, 0, width) },
  };
}

/** 🖼️ Inset rectangle path of the frame outline. */
export function windowFrameBorderPath(metrics: WindowFrameMetrics, inset = WINDOW_FRAME_PATH_INSET): string {
  const { width, height } = normalizeWindowFrameMetrics(metrics);
  if (width <= WINDOW_FRAME_CHIP_EPSILON || height <= WINDOW_FRAME_CHIP_EPSILON) return "";
  const i = Math.min(finiteWindowFrameCoordinate(inset), width / 2, height / 2);
  return `M${i},${i} H${width - i} V${height - i} H${i} Z`;
}

/** 🧊 Glass overlays of the chips, inside the frame. */
export function windowFrameChipRegions(metrics: WindowFrameMetrics): WindowFrameChipRegion[] {
  const normalized = normalizeWindowFrameMetrics(metrics);
  const regions: WindowFrameChipRegion[] = [];
  if (normalized.top.depth > WINDOW_FRAME_CHIP_EPSILON) {
    for (const chip of normalized.top.chips) regions.push({ x: chip.left, y: 0, width: chip.right - chip.left, height: normalized.top.depth, dock: "top" });
  }
  if (normalized.bottom.depth > WINDOW_FRAME_CHIP_EPSILON) {
    const y = normalized.height - normalized.bottom.depth;
    for (const chip of normalized.bottom.chips) regions.push({ x: chip.left, y, width: chip.right - chip.left, height: normalized.bottom.depth, dock: "bottom" });
  }
  return regions;
}

export function windowFrameSafeClearances(metrics: WindowFrameMetrics): WindowFrameSafeClearances {
  const normalized = normalizeWindowFrameMetrics(metrics);
  return { top: normalized.top.depth, right: 0, bottom: normalized.bottom.depth, left: 0 };
}

export function windowFrameContains(metrics: WindowFrameMetrics, x: number, y: number): boolean {
  if (!Number.isFinite(x) || !Number.isFinite(y)) return false;
  const { width, height } = normalizeWindowFrameMetrics(metrics);
  return x >= 0 && x <= width && y >= 0 && y <= height;
}

export function pendingWindowFrameMetrics(pending?: Partial<PendingWindowFrameMetrics>): WindowFrameMetrics {
  return normalizeWindowFrameMetrics({
    width: pending?.width ?? 0,
    height: pending?.height ?? 0,
    top: { depth: pending?.topClearance ?? 0, chips: [] },
    bottom: { depth: pending?.bottomClearance ?? 0, chips: [] },
  });
}

export function createWindowFrameGeometry(metrics: WindowFrameMetrics | null, pending?: Partial<PendingWindowFrameMetrics>): WindowFrameGeometry {
  const measured = metrics ? normalizeWindowFrameMetrics(metrics) : null;
  const ready = measured !== null && measured.width > WINDOW_FRAME_CHIP_EPSILON && measured.height > WINDOW_FRAME_CHIP_EPSILON;
  const normalized = ready && measured ? measured : pendingWindowFrameMetrics(pending);
  return {
    schema: WINDOW_FRAME_GEOMETRY_SCHEMA,
    state: ready ? "ready" : "pending",
    metrics: normalized,
    frame: { x: 0, y: 0, width: normalized.width, height: normalized.height },
    borderPath: windowFrameBorderPath(normalized),
    chipRegions: ready ? windowFrameChipRegions(normalized) : [],
    safeClearances: windowFrameSafeClearances(normalized),
  };
}
//#endregion 🔲️WindowFrameGeometry
