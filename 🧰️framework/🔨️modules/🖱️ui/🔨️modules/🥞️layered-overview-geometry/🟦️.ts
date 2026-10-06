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

/** 📍️ The occupied cell nearest to `offset`: rounded, then clamped onto occupied columns ({@link clampOffset}). */
export function nearestCell(offset: LayeredOffset, cells: readonly LayeredCell[]): LayeredCell {
  const { x, y } = clampOffset({ x: Math.round(offset.x), y: Math.round(offset.y) }, cells);
  return { column: Math.round(x), row: Math.round(y) };
}

/** 📍️ The cell a view rests on exactly, or `null` while it lies between cells. */
export function restingCell(offset: LayeredOffset): LayeredCell | null {
  const column = Math.round(offset.x);
  const row = Math.round(offset.y);
  return Math.abs(offset.x - column) < LAYERED_FOLLOW_EPSILON && Math.abs(offset.y - row) < LAYERED_FOLLOW_EPSILON ? { column, row } : null;
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

/** 🎞️ Follow factor of the pointer pan per frame of a 60 Hz display: the share of the remaining gap one such frame closes. */
export const LAYERED_FOLLOW_LERP = 0.12;

/** ⏲️ The frame {@link LAYERED_FOLLOW_LERP} is defined for, in ms. */
export const LAYERED_FOLLOW_FRAME_MS = 1000 / 60;

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

/** 🧲️ The share of the remaining gap a follow closes within `elapsedMs`: {@link LAYERED_FOLLOW_LERP} per 60 Hz frame, compounded, so the pan
 * takes the same time at every refresh rate and under a browser that paints slowly. */
export function followFactor(elapsedMs: number = LAYERED_FOLLOW_FRAME_MS): number {
  return 1 - (1 - LAYERED_FOLLOW_LERP) ** (Math.max(0, elapsedMs) / LAYERED_FOLLOW_FRAME_MS);
}

/** 🎞️ The pointer follow `elapsedMs` after its last frame (one 60 Hz frame unless given); it lands exactly on `target` once within {@link LAYERED_FOLLOW_EPSILON}. */
export function followStep(current: LayeredOffset, target: LayeredOffset, elapsedMs?: number): { readonly offset: LayeredOffset; readonly settled: boolean } {
  if (Math.abs(target.x - current.x) < LAYERED_FOLLOW_EPSILON && Math.abs(target.y - current.y) < LAYERED_FOLLOW_EPSILON) return { offset: target, settled: true };
  return { offset: lerpOffset(current, target, followFactor(elapsedMs)), settled: false };
}

/** 🎞️ Duration of the settle that carries a released swipe or a wheel step onto its cell. */
export const LAYERED_SETTLE_MS = 320;

/** 🎞️ Cubic ease-out: a settle leaves at the speed it was let go with and comes to rest. */
export function easeOutCubic(t: number): number {
  return 1 - (1 - t) ** 3;
}

/** 🎞️ Where a settle from `from` to `to` is after `elapsedMs`; it lands exactly on `to`. */
export function settleOffset(from: LayeredOffset, to: LayeredOffset, elapsedMs: number, durationMs: number = LAYERED_SETTLE_MS): { readonly offset: LayeredOffset; readonly done: boolean } {
  const t = Math.min(1, Math.max(0, elapsedMs / durationMs));
  return t >= 1 ? { offset: to, done: true } : { offset: lerpOffset(from, to, easeOutCubic(t)), done: false };
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

/** 📐️ Where cell `index` lies along one axis of a view panned to `offset` on that axis. */
export function paneAxisBounds(index: number, offset: number): PaneAxisBounds {
  const start = Math.max(0, index - offset);
  const end = Math.min(1, index - offset + 1);
  return { start, end, visible: end - start > LAYERED_VEIL_EPSILON };
}

/** 🕳️ Slivers and overhangs below this fraction of the view count as nothing. */
export const LAYERED_VEIL_EPSILON = 1e-6;

/** 🕳️ The glass over the strip: `whole` (no hole), `clear` (the revealed pane fills the view, the glass is hidden) or a `hole` in fractions of the view. */
export type LayeredVeil = { readonly kind: "whole" } | { readonly kind: "clear" } | { readonly kind: "hole"; readonly left: number; readonly top: number; readonly right: number; readonly bottom: number };

/** 🕳️ The glass for a revealed `cell` (or none) at the CURRENT `offset` of the strip. */
export function veilClip(cell: LayeredCell | null, offset: LayeredOffset): LayeredVeil {
  if (cell === null) return { kind: "whole" };
  const horizontal = paneAxisBounds(cell.column, offset.x);
  const vertical = paneAxisBounds(cell.row, offset.y);
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

// #region 👆️LayeredSwipe
/** 👆️ How far (px) a touch travels before it is a swipe along one axis rather than a tap. */
export const LAYERED_SWIPE_SLOP_PX = 10;

/** 👆️ The share of the view a released swipe must have carried the strip to move on to the neighbouring cell. */
export const LAYERED_SWIPE_DISTANCE = 0.25;

/** 👆️ The release speed (px/ms) at which a swipe moves on however short it was: a flick. */
export const LAYERED_SWIPE_FLICK_PX_PER_MS = 0.3;

/** 🪀️ The stiffness of the rubber band past the last cell of a row or column — UIKit's 0.55. */
export const LAYERED_RUBBER_BAND = 0.55;

/** 🧈️ The share of its speed a flung scroll keeps per ms — UIKit's normal deceleration rate. */
export const LAYERED_DECELERATION = 0.998;

/** 🧈️ A flung scroll stops below this speed (px/ms). */
export const LAYERED_FLING_STOP_PX_PER_MS = 0.02;

/** 🛞️ The wheel distance (px) one step to the neighbouring cell needs. */
export const LAYERED_WHEEL_STEP_PX = 40;

/** 🛞️ The pause (ms) that ends a wheel gesture, so the trailing inertia of a touchpad never takes a second step. */
export const LAYERED_WHEEL_QUIET_MS = 250;

/** ↔️ An axis of the strip: `x` along the rows, `y` along the columns. */
export type LayeredAxis = "x" | "y";

/** ↔️ The axis a touch that moved `dx`, `dy` px swipes along, or `null` while it is still within {@link LAYERED_SWIPE_SLOP_PX}; the longer leg wins, a tie goes to `x`. */
export function swipeAxis(dx: number, dy: number): LayeredAxis | null {
  if (Math.max(Math.abs(dx), Math.abs(dy)) < LAYERED_SWIPE_SLOP_PX) return null;
  return Math.abs(dx) >= Math.abs(dy) ? "x" : "y";
}

/** 👆️ The cell a released swipe moves the view by along its axis: `movedPx` is how far the finger carried the strip from its cell (negative =
 * left or up, toward the next cell), `velocityPxPerMs` its speed at release with the same sign; a flick moves the way it flicks, a long swipe the
 * way it went, anything else stays. */
export function swipeStep(movedPx: number, velocityPxPerMs: number, viewPx: number): -1 | 0 | 1 {
  const direction = Math.abs(velocityPxPerMs) >= LAYERED_SWIPE_FLICK_PX_PER_MS ? velocityPxPerMs : Math.abs(movedPx) >= LAYERED_SWIPE_DISTANCE * viewPx ? movedPx : 0;
  return direction > 0 ? -1 : direction < 0 ? 1 : 0;
}

/** 🧭️ The occupied cell one `step` from `from` along `axis`, landing on occupied columns like a pan ({@link clampOffset}); `null` where the
 * strip ends or the cell holds no pane. */
export function swipeTarget(from: LayeredCell, axis: LayeredAxis, step: number, cells: readonly LayeredCell[]): LayeredCell | null {
  if (step === 0) return null;
  const target = nearestCell(axis === "x" ? { x: from.column + step, y: from.row } : { x: from.column, y: from.row + step }, cells);
  if (target.column === from.column && target.row === from.row) return null;
  return cells.some((cell) => cell.column === target.column && cell.row === target.row) ? target : null;
}

/** 🧭️ Where a neighbouring page lies from the view: `left` is reached by swiping right, `up` by swiping down. */
export type LayeredDirection = "up" | "left" | "right" | "down";

/** 🧭️ The directions in the order a reader meets their hints: above, beside, below. */
export const LAYERED_DIRECTIONS: readonly { readonly direction: LayeredDirection; readonly axis: LayeredAxis; readonly step: -1 | 1 }[] = [
  { direction: "up", axis: "y", step: -1 },
  { direction: "left", axis: "x", step: -1 },
  { direction: "right", axis: "x", step: 1 },
  { direction: "down", axis: "y", step: 1 },
];

/** 🍩️ Where a swipe lands on the wrapping strip: the occupied `cell`, and the `slot` — the offset next to the view where that page comes in
 * from. Inside the strip the slot is the cell itself; across an edge the index wraps (past the last occupied column of a row to its first,
 * past the bottom row to the top one, onto its occupied columns like a pan) while the slot stays beside the view, so the strip seems an
 * endless canvas. */
export interface LayeredLanding {
  readonly cell: LayeredCell;
  readonly slot: LayeredOffset;
}

/** 🍩️ The occupied cell a swipe past the edge wraps to: the far end of the row's occupied columns, or the far row on its occupied columns. */
function wrappedCell(from: LayeredCell, axis: LayeredAxis, step: number, cells: readonly LayeredCell[]): LayeredCell {
  if (axis === "y") return nearestCell({ x: from.column, y: step > 0 ? 0 : stripGrid(cells).rows - 1 }, cells);
  const columns = cells.filter((other) => other.row === from.row).map((other) => other.column);
  return { column: step > 0 ? Math.min(...columns) : Math.max(...columns), row: from.row };
}

/** 🍩️ The landing one `step` from `from` along `axis` on the wrapping strip ({@link LayeredLanding}); `null` without a step or where the row
 * (or the column) holds no other page. */
export function swipeWrapTarget(from: LayeredCell, axis: LayeredAxis, step: number, cells: readonly LayeredCell[]): LayeredLanding | null {
  if (step === 0) return null;
  const inside = swipeTarget(from, axis, step, cells);
  if (inside !== null) return { cell: inside, slot: cellOffset(inside) };
  const cell = wrappedCell(from, axis, step, cells);
  if (cell.column === from.column && cell.row === from.row) return null;
  return { cell, slot: axis === "x" ? { x: from.column + step, y: from.row } : { x: cell.column, y: from.row + step } };
}

/** 🧭️ The landings one swipe away from `from`, by the direction they lie in — exactly where {@link swipeWrapTarget} lands a swipe, so a hint
 * never promises a page the swipe does not reach. Directions without another page are left out. */
export function swipeNeighbours(from: LayeredCell, cells: readonly LayeredCell[]): readonly ({ readonly direction: LayeredDirection } & LayeredLanding)[] {
  return LAYERED_DIRECTIONS.flatMap(({ direction, axis, step }) => {
    const landing = swipeWrapTarget(from, axis, step, cells);
    return landing === null ? [] : [{ direction, ...landing }];
  });
}

/** 🪀️ How far (in cells) the strip follows a finger that pulls it `overshoot` cells past the last cell: ever less, never a whole cell. */
export function rubberBand(overshoot: number): number {
  return 1 - 1 / (Math.max(0, overshoot) * LAYERED_RUBBER_BAND + 1);
}

/** 👆️ Where the strip lies while a finger carries it `shift` cells from `from` along `axis` (positive = toward the next cell): toward the slot
 * the neighbouring page comes in from ({@link swipeWrapTarget}, across an edge too) at most all the way, on a rubber band where the row or
 * column holds no other page. */
export function swipeDragOffset(from: LayeredCell, axis: LayeredAxis, shift: number, cells: readonly LayeredCell[]): LayeredOffset {
  const step = Math.sign(shift);
  const base = cellOffset(from);
  const landing = swipeWrapTarget(from, axis, step, cells);
  if (landing !== null) return lerpOffset(base, landing.slot, Math.min(1, Math.abs(shift)));
  const pull = step * rubberBand(Math.abs(shift));
  return axis === "x" ? { x: base.x + pull, y: base.y } : { x: base.x, y: base.y + pull };
}

/** 🧈️ A flung scroll `elapsedMs` on: the distance it travelled (px), its speed then (px/ms) and whether it has come to rest — exponential
 * decay by {@link LAYERED_DECELERATION} per ms, exact at any frame rate. */
export function flingStep(velocityPxPerMs: number, elapsedMs: number): { readonly distance: number; readonly velocity: number; readonly done: boolean } {
  const kept = LAYERED_DECELERATION ** Math.max(0, elapsedMs);
  const velocity = velocityPxPerMs * kept;
  return { distance: (velocityPxPerMs * (kept - 1)) / Math.log(LAYERED_DECELERATION), velocity, done: Math.abs(velocity) < LAYERED_FLING_STOP_PX_PER_MS };
}
// #endregion 👆️LayeredSwipe

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

/** 🐢️ While the view rests on cell `at` of a swiped strip, the panes one swipe away — its neighbours on the wrapping strip
 * ({@link swipeNeighbours}), in pane order — are booted one at a time while fewer than `budget` panes live, so the next swipe lands on a live
 * page and warming never evicts one. */
export function neighbourWarmBoot(ids: readonly string[], cells: readonly LayeredCell[], at: LayeredCell, live: ReadonlySet<string>, openedId: string | null, budget: number): LayeredWarmStep {
  if (openedId !== null) return { kind: "hold", reason: "opened" };
  const around = new Set(swipeNeighbours(at, cells).map(({ cell }) => `${cell.column}:${cell.row}`));
  const next = ids.find((id, index) => !live.has(id) && around.has(`${cells[index]!.column}:${cells[index]!.row}`));
  if (next === undefined) return { kind: "hold", reason: "complete" };
  if (live.size >= budget) return { kind: "hold", reason: "budget" };
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
