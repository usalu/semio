/** 📊️ The windowed table rules: next row and column, scroll offsets, viewport row cap and the column template.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { TABLE_COLUMN_WINDOW_WIDTH_PX } from "../🪟️tree-windows/🟦️.ts";

//#region 🔖️UiInterpreter
//#region 📊️TableWindow
/** 📊️ Keys that move the active row of a windowed table, and how far. `page` is the viewport's own row
 * count, so PageDown lands on the row the reader sees at the bottom — the ARIA grid pattern's row
 * navigation over the table's WHOLE logical extent, not over the rows that happen to be materialised. */
export function tableWindowNextRowV1(key: string, current: number, total: number, page: number): number | null {
  if (total <= 0) return null;
  const last = total - 1;
  const clamp = (value: number) => Math.min(last, Math.max(0, value));
  switch (key) {
    case "ArrowDown":
      return clamp(current + 1);
    case "ArrowUp":
      return clamp(current - 1);
    case "PageDown":
      return clamp(current + Math.max(1, page));
    case "PageUp":
      return clamp(current - Math.max(1, page));
    case "Home":
      return 0;
    case "End":
      return last;
    default:
      return null;
  }
}

/** 📊️ The scroll position that brings logical row `index` fully into a viewport of `height` pixels at
 * `scrollTop`, rows pitched `rowPx` apart — unchanged when it already is. Spacers keep every row at
 * `index * rowPx` whether or not it is materialised, so this is exact for rows the host has not streamed. */
export function tableWindowScrollTopForRowV1(index: number, rowPx: number, scrollTop: number, height: number): number {
  const top = index * rowPx;
  if (top < scrollTop) return top;
  if (top + rowPx > scrollTop + height) return Math.max(0, top + rowPx - height);
  return scrollTop;
}

/** 📊️ How many rows a windowed table's scroll viewport may show at most: the capacity the guest's short answers taught
 * (`learned`, {@link treeWindowServedRequestsV1}), else the one it taught before (a new `total` forgets the capacity
 * until the next short answer re-teaches it, and the viewport must not flash open meanwhile), while the list is longer
 * than that, one row short of it (a viewport scrolled between rows shows a partial row at each edge, and the served
 * window must cover both) — `null` = uncapped. A viewport taller than the rows the guest can materialise at once shows blank rows no
 * scroll reaches: the whole list fits the viewport, so the scroll range is a row or two, the window never moves, and
 * every row past the capacity stays unreachable by wheel, keyboard and screen reader alike (Home on hub 7800, 11 of 29
 * spaces, ticket 26/09/23 S18 §14c / C12). Capped, the viewport scrolls over the list and the window follows it.
 * Rows: `🖱️ui/🧬️contract/🧫️fixtures/🪟️tree-window-served.json` `tableViewportCaps`. */
export function tableWindowViewportCapRowsV1(learned: number | null, previous: number | null, total: number): number | null {
  const servable = learned ?? previous;
  return servable !== null && servable > 0 && servable < total ? Math.max(1, servable - 1) : null;
}

/** 📊️ The shared column track of a table's header and rows: the first (name) column twice as wide as the
 * others, the actions column exactly as wide as the widest materialised action strip — header and rows
 * read the SAME string, so their columns line up however the rows stream. */
export function tableWindowColumnTemplate(columns: number, actions: number, leading: number, trailing: number): string {
  const spacer = (count: number) => `minmax(${count * TABLE_COLUMN_WINDOW_WIDTH_PX}px, ${count * TABLE_COLUMN_WINDOW_WIDTH_PX}px)`;
  return [...(leading > 0 ? [spacer(leading)] : []), ...Array.from({ length: columns }, () => `minmax(${TABLE_COLUMN_WINDOW_WIDTH_PX}px, ${TABLE_COLUMN_WINDOW_WIDTH_PX}px)`), ...(actions > 0 ? [`calc(${actions} * var(--size-medium) + ${actions} * var(--spacing-single))`] : []), ...(trailing > 0 ? [spacer(trailing)] : [])].join(" ");
}

/** ↔️ Grid-column navigation over the complete logical extent. */
export function tableWindowNextColumnV1(key: string, current: number, total: number): number | null {
  if (total <= 0) return null;
  if (key === "ArrowRight") return Math.min(total - 1, current + 1);
  if (key === "ArrowLeft") return Math.max(0, current - 1);
  if (key === "Home") return 0;
  if (key === "End") return total - 1;
  return null;
}

/** ↔️ Horizontal counterpart of `tableWindowScrollTopForRowV1`. */
export function tableWindowScrollLeftForColumnV1(index: number, columnPx: number, scrollLeft: number, width: number): number {
  const left = index * columnPx;
  if (left < scrollLeft) return left;
  if (left + columnPx > scrollLeft + width) return Math.max(0, left + columnPx - width);
  return scrollLeft;
}
//#endregion 📊️TableWindow
//#endregion 🔖️UiInterpreter
