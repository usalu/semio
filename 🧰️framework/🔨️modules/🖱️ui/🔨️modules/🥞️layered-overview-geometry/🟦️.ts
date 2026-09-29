// #region 🧲️Header
// 💻️ framework/ui/modules/🥞️layered-overview-geometry/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🧬️LayeredContracts
/** 📍️ Zero-based strip cell of a pane. */
export interface LayeredCell {
  readonly column: number;
  readonly row: number;
}

/** 🧭️ A position on the strip in cell units: `{ x: 0, y: 0 }` shows the top-left cell, `{ x: 1.5, y: 0 }` half of the second and half of the third. */
export interface LayeredOffset {
  readonly x: number;
  readonly y: number;
}

/** 📐️ Columns and rows of a strip. */
export interface LayeredGrid {
  readonly columns: number;
  readonly rows: number;
}

/** 📏️ The columns a row of the strip actually holds. */
export interface LayeredSpan {
  readonly first: number;
  readonly last: number;
}

/** ▭️ A rectangle in fractions of the view (`{ x: 0, y: 0, width: 1, height: 1 }` is the whole view). */
export interface LayeredRect {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

/** 📏️ Relative track sizes of the grid rest (`fr` weights per strip column and row; missing ones weigh 1), so each page lies exactly behind a card
 * cell of an overlay grid built from the same tracks. */
export interface LayeredTracks {
  readonly columns?: readonly number[];
  readonly rows?: readonly number[];
}

/** ⏱️ When panes are mounted and released; `budget` is the most panes mounted at once, every other value is in ms. */
export interface LayeredLifecycle {
  readonly budget: number;
  readonly warmStartMs: number;
  readonly warmIntervalMs: number;
  readonly suspendIdleMs: number;
  readonly suspendOffscreenMs: number;
  readonly suspendHiddenMs: number;
  readonly sweepMs: number;
}
// #endregion 🧬️LayeredContracts

// #region 🔢️LayeredGrid
/** 📐️ How much wider than tall {@link nearSquareGrid} may get before a card column gets too narrow to read. */
export const LAYERED_MAX_COLUMN_SURPLUS = 2;

/** 🔢️ Of every near-square shape that holds `count` panes, the one leaving the fewest empty cells, ties going to the squarest. */
export function nearSquareGrid(count: number): LayeredGrid {
  if (!Number.isInteger(count) || count < 1) throw new Error(`A layered grid needs at least one pane: ${count}`);
  let best: LayeredGrid | undefined;
  for (let columns = 1; columns <= count; columns += 1) {
    const rows = Math.ceil(count / columns);
    if (columns < rows || columns - rows > LAYERED_MAX_COLUMN_SURPLUS) continue;
    const empty = columns * rows - count;
    if (best === undefined || empty < best.columns * best.rows - count || (empty === best.columns * best.rows - count && columns - rows < best.columns - best.rows)) best = { columns, rows };
  }
  if (best === undefined) throw new Error(`A layered grid has no shape for ${count} panes`);
  return best;
}

/** 📏️ The columns row `row` of {@link nearSquareGrid}`(count)` holds: every row but the last is full, a short last row is centred. */
export function centeredRowSpan(row: number, count: number): LayeredSpan {
  const { columns, rows } = nearSquareGrid(count);
  if (!Number.isInteger(row) || row < 0 || row >= rows) throw new Error(`Row outside the layered grid: ${row}`);
  const inRow = Math.min(columns, count - row * columns);
  const first = Math.floor((columns - inRow) / 2);
  return { first, last: first + inRow - 1 };
}

/** 📍️ The cell of every pane, row-major over {@link centeredRowSpan}, so the strip never trails off into empty cells on one side. */
export function centeredLastRowCells(count: number): readonly LayeredCell[] {
  const { columns } = nearSquareGrid(count);
  return Array.from({ length: count }, (_, index) => {
    const row = Math.floor(index / columns);
    return { column: centeredRowSpan(row, count).first + (index % columns), row };
  });
}

/** 📐️ The strip that holds `cells`: one past the largest column and row. */
export function stripGrid(cells: readonly LayeredCell[]): LayeredGrid {
  return cells.reduce<LayeredGrid>((grid, cell) => ({ columns: Math.max(grid.columns, cell.column + 1), rows: Math.max(grid.rows, cell.row + 1) }), { columns: 1, rows: 1 });
}

/** 🧭️ The offset that shows `cell` exactly. */
export function cellOffset(cell: LayeredCell): LayeredOffset {
  return { x: cell.column, y: cell.row };
}

/** 📏️ The occupied columns of the row filling most of a view whose top edge sits `y` rows into the strip — a free pan rests there, so an empty flank beside a short row is never a view of its own. */
export function occupiedColumns(cells: readonly LayeredCell[], y: number): LayeredSpan {
  const { columns, rows } = stripGrid(cells);
  const row = Math.min(rows - 1, Math.max(0, Math.round(y)));
  const inRow = cells.filter((cell) => cell.row === row).map((cell) => cell.column);
  return inRow.length === 0 ? { first: 0, last: columns - 1 } : { first: Math.min(...inRow), last: Math.max(...inRow) };
}

/** 🧭️ Keeps an offset inside the strip and on occupied columns ({@link occupiedColumns}). */
export function clampOffset(offset: LayeredOffset, cells: readonly LayeredCell[]): LayeredOffset {
  const { rows } = stripGrid(cells);
  const y = Math.min(rows - 1, Math.max(0, offset.y));
  const { first, last } = occupiedColumns(cells, y);
  return { x: Math.min(last, Math.max(first, offset.x)), y };
}

/** 🖱️ The offset a pointer at the fractions `fx`, `fy` of the view pans to: the top-left corner shows the first cell, the bottom-right the last. */
export function pointerOffset(fx: number, fy: number, grid: LayeredGrid): LayeredOffset {
  const unit = (value: number): number => Math.min(1, Math.max(0, value));
  return { x: unit(fx) * (grid.columns - 1), y: unit(fy) * (grid.rows - 1) };
}
// #endregion 🔢️LayeredGrid

// #region 🎞️LayeredMotion
/** 🎞️ Duration of every programmatic glide (reveal, open, deep link). */
export const LAYERED_GLIDE_MS = 500;

/** 🎞️ Per-frame follow factor of the pointer pan. */
export const LAYERED_FOLLOW_LERP = 0.12;

/** 🎞️ The follow settles once both axes are this close to the target, in cells. */
export const LAYERED_FOLLOW_EPSILON = 1e-4;

/** 🎞️ Cubic ease-in-out. */
export function easeInOutCubic(t: number): number {
  return t < 0.5 ? 4 * t * t * t : 1 - (-2 * t + 2) ** 3 / 2;
}

/** 🧭️ Linear blend of two offsets. */
export function lerpOffset(from: LayeredOffset, to: LayeredOffset, t: number): LayeredOffset {
  return { x: from.x + (to.x - from.x) * t, y: from.y + (to.y - from.y) * t };
}

/** 🎞️ Where a glide from `from` to `to` is after `elapsedMs`; it lands exactly on `to`. */
export function glideOffset(from: LayeredOffset, to: LayeredOffset, elapsedMs: number, durationMs: number = LAYERED_GLIDE_MS): { readonly offset: LayeredOffset; readonly done: boolean } {
  const t = Math.min(1, Math.max(0, elapsedMs / durationMs));
  return t >= 1 ? { offset: to, done: true } : { offset: lerpOffset(from, to, easeInOutCubic(t)), done: false };
}

/** 🎞️ One frame of the pointer follow; it lands exactly on `target` once within {@link LAYERED_FOLLOW_EPSILON}. */
export function followStep(current: LayeredOffset, target: LayeredOffset): { readonly offset: LayeredOffset; readonly settled: boolean } {
  if (Math.abs(target.x - current.x) < LAYERED_FOLLOW_EPSILON && Math.abs(target.y - current.y) < LAYERED_FOLLOW_EPSILON) return { offset: target, settled: true };
  return { offset: lerpOffset(current, target, LAYERED_FOLLOW_LERP), settled: false };
}

/** 🧱️ The strip's CSS transform for `offset`, in percent of the strip itself. */
export function stripTransform(offset: LayeredOffset, grid: LayeredGrid): string {
  const percent = (value: number, count: number): number => Math.round((-value * 1e6) / count) / 1e4 + 0;
  return `translate3d(${percent(offset.x, grid.columns)}%, ${percent(offset.y, grid.rows)}%, 0)`;
}
// #endregion 🎞️LayeredMotion

// #region 🕳️LayeredVeil
/** 📐️ One axis of a cell in the view after panning, as fractions of the view. */
export interface PaneAxisBounds {
  readonly start: number;
  readonly end: number;
  readonly visible: boolean;
}

/** 📐️ Where a span starting at `start` (fractions of the view) and `size` long lies inside the view along one axis. */
export function spanAxisBounds(start: number, size: number): PaneAxisBounds {
  const inside = Math.max(0, start);
  const end = Math.min(1, start + size);
  return { start: inside, end, visible: end - inside > LAYERED_VEIL_EPSILON };
}

/** 📐️ Where cell `index` lies along one axis of a view panned to `offset` on that axis. */
export function paneAxisBounds(index: number, offset: number): PaneAxisBounds {
  return spanAxisBounds(index - offset, 1);
}

/** 🕳️ Slivers and overhangs below this fraction of the view count as nothing. */
export const LAYERED_VEIL_EPSILON = 1e-6;

/** 🕳️ The glass over the strip: `whole` (no hole), `clear` (the revealed pane fills the view, the glass is hidden) or a `hole` in fractions of the view. */
export type LayeredVeil = { readonly kind: "whole" } | { readonly kind: "clear" } | { readonly kind: "hole"; readonly left: number; readonly top: number; readonly right: number; readonly bottom: number };

/** 🕳️ The glass for a revealed `cell` (or none) at the CURRENT `offset` of the panorama. */
export function veilClip(cell: LayeredCell | null, offset: LayeredOffset): LayeredVeil {
  return veilForRect(cell === null ? null : { x: cell.column - offset.x, y: cell.row - offset.y, width: 1, height: 1 });
}

/** 🕳️ The glass for the revealed page showing at `rect` (fractions of the view), or none. */
export function veilForRect(rect: LayeredRect | null): LayeredVeil {
  if (rect === null) return { kind: "whole" };
  const horizontal = spanAxisBounds(rect.x, rect.width);
  const vertical = spanAxisBounds(rect.y, rect.height);
  if (!horizontal.visible || !vertical.visible) return { kind: "whole" };
  const covers = (bounds: PaneAxisBounds): boolean => bounds.start <= LAYERED_VEIL_EPSILON && bounds.end >= 1 - LAYERED_VEIL_EPSILON;
  if (covers(horizontal) && covers(vertical)) return { kind: "clear" };
  return { kind: "hole", left: horizontal.start, top: vertical.start, right: horizontal.end, bottom: vertical.end };
}

/** 🔺️ The `evenodd` vertex list of a veil with a hole, in percent of the view: the view's outline, then the hole's. */
export function veilPolygon(veil: Extract<LayeredVeil, { kind: "hole" }>): readonly (readonly [number, number])[] {
  const p = (value: number): number => Math.round(value * 1e6) / 1e4;
  const [l, t, r, b] = [p(veil.left), p(veil.top), p(veil.right), p(veil.bottom)];
  return [[0, 0], [100, 0], [100, 100], [0, 100], [0, 0], [l, t], [r, t], [r, b], [l, b], [l, t]];
}

/** ✂️ The veil's CSS `clip-path`. */
export function veilClipPath(veil: LayeredVeil): string {
  return veil.kind === "hole" ? `polygon(evenodd, ${veilPolygon(veil).map(([x, y]) => `${x}% ${y}%`).join(", ")})` : "none";
}
// #endregion 🕳️LayeredVeil

// #region 🔍️LayeredGridRest
/** ▭️ The whole view. */
export const LAYERED_VIEW: LayeredRect = { x: 0, y: 0, width: 1, height: 1 };

/** 📏️ Start and size of every track along one axis, as fractions of the view, from `fr` weights (missing ones weigh 1). */
export function trackSpans(weights: readonly number[] | undefined, count: number): readonly { readonly start: number; readonly size: number }[] {
  const sizes = Array.from({ length: count }, (_, index) => weights?.[index] ?? 1);
  if (sizes.some((size) => !(size > 0) || !Number.isFinite(size))) throw new Error(`Layered grid tracks must be positive: ${sizes.join(" ")}`);
  const total = sizes.reduce((sum, size) => sum + size, 0);
  return sizes.map((size, index) => ({ start: sizes.slice(0, index).reduce((sum, before) => sum + before, 0) / total, size: size / total }));
}

/** 🔳️ The CSS track list of one axis (`minmax(0, 1fr) minmax(0, 1.5fr) …`) — the overlay grid that puts each card over its page. */
export function trackTemplate(weights: readonly number[] | undefined, count: number): string {
  return Array.from({ length: count }, (_, index) => `minmax(0, ${weights?.[index] ?? 1}fr)`).join(" ");
}

/** 🔳️ Where `cell` rests in the grid rest: its track rectangle. */
export function restRect(cell: LayeredCell, grid: LayeredGrid, tracks: LayeredTracks = {}): LayeredRect {
  const column = trackSpans(tracks.columns, grid.columns)[cell.column]!;
  const row = trackSpans(tracks.rows, grid.rows)[cell.row]!;
  return { x: column.start, y: row.start, width: column.size, height: row.size };
}

/** 🧭️ Linear blend of two rectangles. */
export function lerpRect(from: LayeredRect, to: LayeredRect, t: number): LayeredRect {
  return { x: from.x + (to.x - from.x) * t, y: from.y + (to.y - from.y) * t, width: from.width + (to.width - from.width) * t, height: from.height + (to.height - from.height) * t };
}

/** 🎞️ Where a zoom of the camera from `from` to `to` (the part of the grid rest shown full size) is after `elapsedMs`; it lands exactly on `to`. */
export function glideRect(from: LayeredRect, to: LayeredRect, elapsedMs: number, durationMs: number = LAYERED_GLIDE_MS): { readonly rect: LayeredRect; readonly done: boolean } {
  const t = Math.min(1, Math.max(0, elapsedMs / durationMs));
  return t >= 1 ? { rect: to, done: true } : { rect: lerpRect(from, to, easeInOutCubic(t)), done: false };
}

/** 🔍️ Where a page resting at `rest` shows while the camera frames `camera` (the grid rest's `LAYERED_VIEW` at rest, the page's own rest when it is zoomed to full size). */
export function viewRect(rest: LayeredRect, camera: LayeredRect): LayeredRect {
  return { x: (rest.x - camera.x) / camera.width, y: (rest.y - camera.y) / camera.height, width: rest.width / camera.width, height: rest.height / camera.height };
}

/** 🖼️ How a view-sized page covers `view` without distortion: uniformly scaled to cover it, anchored at the top centre, the rest cut off — the
 * page's CSS `transform` (origin `0 0`) and `clip-path`. A full-view rectangle is the identity. */
export function coverPlacement(view: LayeredRect): { readonly transform: string; readonly clipPath: string } {
  const scale = Math.max(view.width, view.height);
  const side = (1 - view.width / scale) / 2;
  const bottom = 1 - view.height / scale;
  const percent = (value: number): number => Math.round(value * 1e6) / 1e4 + 0;
  return {
    transform: `translate3d(${percent(view.x - side * scale)}%, ${percent(view.y)}%, 0) scale(${Math.round(scale * 1e6) / 1e6})`,
    clipPath: `inset(0% ${percent(side)}% ${percent(bottom)}% ${percent(side)}%)`,
  };
}
// #endregion 🔍️LayeredGridRest

// #region 🪟️LayeredWindowing
/** 🪟️ The cells whose placeholder or poster exists while the view sits at an offset. */
export interface LayeredWindow {
  readonly firstColumn: number;
  readonly lastColumn: number;
  readonly firstRow: number;
  readonly lastRow: number;
}

/** 🪟️ Every cell the view at `offset` touches, widened by `radius` cells. */
export function windowAround(offset: LayeredOffset, radius: number): LayeredWindow {
  return { firstColumn: Math.floor(offset.x) - radius, lastColumn: Math.ceil(offset.x) + radius, firstRow: Math.floor(offset.y) - radius, lastRow: Math.ceil(offset.y) + radius };
}

/** 🪟️ Whether `cell` lies in `window`. */
export function inWindow(cell: LayeredCell, window: LayeredWindow): boolean {
  return cell.column >= window.firstColumn && cell.column <= window.lastColumn && cell.row >= window.firstRow && cell.row <= window.lastRow;
}
// #endregion 🪟️LayeredWindowing

// #region 🧮️LayeredLifecycle
/** ⏱️ Defaults of {@link LayeredLifecycle}: four panes, the first warm boot after 1.5 s, one per 35 s (one plugin-load budget), never released by time. */
export const LAYERED_DEFAULT_LIFECYCLE: LayeredLifecycle = { budget: 4, warmStartMs: 1_500, warmIntervalMs: 35_000, suspendIdleMs: Number.POSITIVE_INFINITY, suspendOffscreenMs: Number.POSITIVE_INFINITY, suspendHiddenMs: Number.POSITIVE_INFINITY, sweepMs: 5_000 };

/** ⏱️ A complete lifecycle: the offscreen and hidden-tab thresholds follow the idle one unless given. */
export function resolveLifecycle(lifecycle: Partial<LayeredLifecycle> = {}): LayeredLifecycle {
  const suspendIdleMs = lifecycle.suspendIdleMs ?? LAYERED_DEFAULT_LIFECYCLE.suspendIdleMs;
  return { ...LAYERED_DEFAULT_LIFECYCLE, suspendOffscreenMs: suspendIdleMs, suspendHiddenMs: suspendIdleMs, ...lifecycle, suspendIdleMs };
}

/** 🌡️ One step of the warm queue: the pane it boots next, or why it holds. */
export type LayeredWarmStep = { readonly kind: "boot"; readonly id: string } | { readonly kind: "hold"; readonly reason: "opened" | "budget" | "complete" };

/** 🐢️ The warm queue boots never-booted panes in cell order while nothing is opened, and pre-boots at most `budget` panes over the page's life — beyond that only a reveal or an open boots one, so warming never evicts a pane the user touched. */
export function nextWarmBoot(ids: readonly string[], booted: ReadonlySet<string>, liveCount: number, openedId: string | null, budget: number): LayeredWarmStep {
  if (openedId !== null) return { kind: "hold", reason: "opened" };
  const next = ids.find((id) => !booted.has(id));
  if (next === undefined) return { kind: "hold", reason: "complete" };
  if (liveCount >= budget || booted.size >= budget) return { kind: "hold", reason: "budget" };
  return { kind: "boot", id: next };
}

/** 🐢️ The wait before the next warm boot: the start grace for the first, a full interval for every later one. */
export function warmDelay(warmedBefore: boolean, lifecycle: LayeredLifecycle): number {
  return warmedBefore ? lifecycle.warmIntervalMs : lifecycle.warmStartMs;
}

/** 🧮️ The panes to release so at most `budget` stay mounted: the least recently touched first, never one in `keep`. */
export function panesOverBudget(liveByRecency: readonly string[], keep: ReadonlySet<string>, budget: number): readonly string[] {
  const excess = liveByRecency.length - budget;
  return excess > 0 ? liveByRecency.filter((id) => !keep.has(id)).slice(0, excess) : [];
}

/** 🕰️ A mounted pane and the last time it was booted or touched. */
export interface LayeredLivePane {
  readonly id: string;
  readonly since: number;
}

/** 🕰️ The mounted panes to release by time: a hidden tab uses `suspendHiddenMs`, an opened pane leaves the others `suspendOffscreenMs`, the overview `suspendIdleMs`; never one in `keep`. */
export function panesToRelease(live: readonly LayeredLivePane[], keep: ReadonlySet<string>, now: number, context: { readonly opened: boolean; readonly hidden: boolean }, lifecycle: LayeredLifecycle): readonly string[] {
  const threshold = context.hidden ? lifecycle.suspendHiddenMs : context.opened ? lifecycle.suspendOffscreenMs : lifecycle.suspendIdleMs;
  return live.filter((pane) => !keep.has(pane.id) && now - pane.since >= threshold).map((pane) => pane.id);
}

/** ⏱️ The browser timing surface {@link scheduleIdle} runs on. */
export interface LayeredIdleScheduler {
  readonly setTimeout: (callback: () => void, delayMs: number) => number;
  readonly clearTimeout: (handle: number) => void;
  readonly requestIdleCallback?: (callback: () => void, options?: { readonly timeout: number }) => number;
  readonly cancelIdleCallback?: (handle: number) => void;
}

/** 🐢️ Runs `callback` in the first idle period after at least `delayMs` (at most 1 s later); returns the cancel. */
export function scheduleIdle(callback: () => void, delayMs: number, scheduler: LayeredIdleScheduler): () => void {
  let idle: number | null = null;
  const timeout = scheduler.setTimeout(() => {
    if (scheduler.requestIdleCallback) idle = scheduler.requestIdleCallback(callback, { timeout: 1_000 });
    else callback();
  }, delayMs);
  return () => {
    scheduler.clearTimeout(timeout);
    if (idle !== null) scheduler.cancelIdleCallback?.(idle);
  };
}
// #endregion 🧮️LayeredLifecycle
