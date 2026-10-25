import type { TreeWindowRowExtent } from "@semio-tech/framework-manifest/ui-contract-and-canvas-types";
import { domSizePx, uiSpacingPx, STYLING_METRICS } from "@semio-tech/ui-styling";
import * as React from "react";

/**
 * 🪟️ The materialised slice of a logically {@link TreeDataWindow.total}-long child list: the container's
 * rendered `items` are entries `[offset, offset + items.length)` of that list. `total > 0` with no items is
 * expandable-but-not-yet-streamed, NOT empty — the rows above and below the slice are stood in for by
 * `data-slot="tree-window-spacer"` blocks of exactly {@link treeRowHeightPx} pitch each, so the scrollbar
 * always spans the whole document and a scroll lands on the real row index.
 * @see 🎫️ 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING · 📓️design-virtualised-tree.md §6.1
 **/
export interface TreeDataWindow {
  readonly total: number;
  readonly offset: number;
  readonly rowExtent: TreeWindowRowExtent;
}

/** 📏️ Resolves the declared closed-row geometry of a virtual Tree window. */
export function treeWindowRowExtentPx(extent: TreeWindowRowExtent): number {
  const exactMetric = (value: number) => Math.round(value * 1_000) / 1_000;
  switch (extent) {
    case "standard":
      return domSizePx("treeRowUiSpacing");
    case "compactText":
      return exactMetric(uiSpacingPx(STYLING_METRICS.chrome.sizeTinyUiSpacing) * 1.5);
    case "compactSmallControl":
      return exactMetric(uiSpacingPx(STYLING_METRICS.chrome.controlHeightSmallUiSpacing));
    case "compactControl":
      return exactMetric(uiSpacingPx(STYLING_METRICS.chrome.controlHeightUiSpacing));
  }
}

/** 🪜️ Every declared closed-row geometry token — the one runtime list a renderer validates a DOM or wire token against before pricing rows with {@link treeWindowRowExtentPx}. A token absent from this list has no pitch, so the container's whole geometry would be `NaN`. */
export const TREE_WINDOW_ROW_EXTENTS = ["standard", "compactText", "compactSmallControl", "compactControl"] as const satisfies readonly TreeWindowRowExtent[];

/** 🔭️ Rows requested beyond each edge of the viewport, so a scroll of up to this many rows paints from what is already materialised. */
export const TREE_WINDOW_OVERSCAN_ROWS = 8;

/** 🏔️ Hard ceiling on one window request — the guest's `UI_BUILT_CHILDREN_MAX`/`UI_DOCUMENT_NODES` fan-out of one child list. */
export const TREE_WINDOW_ROWS_MAX = 128;

/**
 * 💰️ The nodes ALL windowed containers of one panel body may spend together, and the ONE cost model both
 * sides price a body with: **every container costs `1 + rows`** — its own container node plus the rows it
 * materialises — summed over every windowed container in the body, nested ones included, each counted once.
 *
 * 🧯️ This is the guest's own ledger, not an independently chosen number: `TreeWindows`
 * (`🔨️modules/🔌️plugin/🦀️.rs`, region `🔖️PanelWindowing`) runs
 * `UI_DOCUMENT_NODES - 1 - TREE_WINDOW_FIXED_NODE_HEADROOM` records and `debit(1)`s each container before
 * `grant()`ing its rows out of the same pool. The previous host number (112 "rows") priced only the rows
 * and only the visible containers, so a body with one open section asked 112 and was answered 110 on every
 * single render — a permanent, silent under-materialisation with no fault to notice it by
 * (📓️s3-review-streaming-loop.md §1). Two magic numbers across a wasm boundary cannot be kept in step by
 * hand, so this line is the parity anchor: the Rust side greps this file for
 * `TREE_WINDOW_BODY_NODE_BUDGET = <n>` and fails if it disagrees. Keep the spelling on one line.
 *
 * 🧾️ `128 - 1 - 24`: the record arena, less the tree root, less `TREE_WINDOW_FIXED_NODE_HEADROOM` — the
 * reserve for rows a panel builds WITHOUT a window (the energy inspector's sixteen fenestration fields
 * being the fleet's measured worst case). Raised from 16 to 24, and this number from 111 to 103, by F2
 * after 📓️r2-fem-energy-listapps-reverify.md §1.5 measured that body at 21 un-ledgered records.
 * @see 🎫️ 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING · 📓️design-virtualised-tree.md §7
 **/
export const TREE_WINDOW_BODY_NODE_BUDGET = 103;

/** 📐️ One MATERIALISED row of a windowed container as the DOM measured it: which entry of the child list it is, and where its own top edge sits — in the same space as the container's `top`. */
export interface TreeWindowRowMeasure {
  readonly index: number;
  readonly top: number;
}

/** 📦️ One windowed container as the DOM measured it: its authored key, its window, and the full virtual extent (`top`/`height` cover spacers, rows and any expanded nested content); `rows` are its own materialised rows' real tops, ascending, when the observer could read them. */
export interface TreeWindowContainerMeasure {
  readonly key: string;
  readonly total: number;
  readonly offset: number;
  readonly length: number;
  readonly top: number;
  readonly height: number;
  readonly rowExtent: TreeWindowRowExtent;
  readonly rows?: readonly TreeWindowRowMeasure[];
}

/** 📬️ One container's next window: materialise `rows` entries starting at `offset`. */
export interface TreeWindowRequest {
  readonly key: string;
  readonly offset: number;
  readonly rows: number;
}

/** 🎦️ What one on-screen container contributes to the viewport rule: the rows of it the viewport actually covers, where inside its child list they start, and how far its centre sits from the viewport's — the order a body-wide budget trims in. */
export interface TreeWindowVisibleRows {
  readonly total: number;
  readonly visibleRows: number;
  readonly firstVisibleRow: number;
  readonly distancePx: number;
}

/**
 * 🎯️ Maps a pixel position onto the index of the child row of `container` drawn there.
 *
 * 🧯️ Not `floor(y / rowHeightPx)`: a container's own rows are NOT all one row tall. A materialised row can
 * itself be an open windowed group rendering its whole subtree, so every row after it sits at a pixel offset
 * no uniform pitch can predict — and a parent asked for a window computed that way is asked for rows that are
 * nowhere near the viewport, which evicts the very subtree the user just opened and then oscillates as the
 * eviction changes the geometry again (📓️s3-review-streaming-loop.md §2).
 *
 * So the materialised band is read from the rows' REAL tops (`data-tree-window-row`, direct children only,
 * ascending — the observer measures them exactly as it measures the containers). Only the two spacer bands
 * use the uniform pitch, and there it is exact by construction: a spacer IS `rows × rowHeightPx` of nothing.
 * Without row measurements (a test's stubbed rect, a target that cannot read them) the whole container falls
 * back to the flat pitch, which is correct for a container none of whose rows is expanded.
 **/
function treeWindowRowIndexAt(container: TreeWindowContainerMeasure, y: number, total: number, offset: number, length: number, rowHeightPx: number): number {
  const clamp = (index: number) => Math.min(total - 1, Math.max(0, index));
  const localPx = y - container.top;
  const rows = container.rows;
  if (!rows || rows.length === 0) return clamp(Math.floor(localPx / rowHeightPx));
  if (y < rows[0]!.top) return Math.min(offset, clamp(Math.floor(localPx / rowHeightPx)));
  const trailingPx = (total - offset - length) * rowHeightPx;
  const rowsEnd = container.top + Math.max(0, container.height) - trailingPx;
  if (y >= rowsEnd) return clamp(offset + length + Math.floor((y - rowsEnd) / rowHeightPx));
  let low = 0;
  let high = rows.length - 1;
  while (low < high) {
    const middle = Math.ceil((low + high) / 2);
    if (rows[middle]!.top <= y) low = middle;
    else high = middle - 1;
  }
  return clamp(rows[low]!.index);
}

/**
 * 🔬️ Measures every container against the viewport once, so the request rule and the body-wide cap read the
 * SAME geometry instead of each deriving its own. A container that does not overlap the viewport at all is
 * absent from the map — it is dropped entirely (no request, so a scrolled-away branch keeps whatever it last
 * materialised instead of churning). `visibleRows` is counted in the container's OWN child rows
 * ({@link treeWindowRowIndexAt}), never in viewport pixels: a section whose whole visible extent is one
 * expanded child needs one row of itself, not a viewport's worth, and the body-wide budget is spent where the
 * rows actually are.
 * @see 🎫️ 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING · 📓️design-virtualised-tree.md §6.1
 **/
export function treeWindowVisibleRowsForViewport(containers: readonly TreeWindowContainerMeasure[], viewportTop: number, viewportHeight: number): ReadonlyMap<string, TreeWindowVisibleRows> {
  const visible = new Map<string, TreeWindowVisibleRows>();
  const height = Math.max(0, viewportHeight);
  const viewportBottom = viewportTop + height;
  const viewportCentre = viewportTop + height / 2;
  for (const container of containers) {
    const rowHeightPx = treeWindowRowExtentPx(container.rowExtent);
    const total = Math.max(0, Math.floor(container.total));
    if (total === 0) continue;
    const containerHeight = Math.max(0, container.height);
    const overlapPx = Math.min(container.top + containerHeight, viewportBottom) - Math.max(container.top, viewportTop);
    if (overlapPx <= 0) continue;
    const offset = Math.min(Math.max(0, Math.floor(container.offset)), total);
    const length = Math.min(Math.max(0, Math.floor(container.length)), total - offset);
    const firstVisibleRow = treeWindowRowIndexAt(container, Math.max(container.top, viewportTop), total, offset, length, rowHeightPx);
    const lastVisibleRow = treeWindowRowIndexAt(container, Math.min(container.top + containerHeight, viewportBottom) - 1, total, offset, length, rowHeightPx);
    visible.set(container.key, { total, visibleRows: Math.max(1, lastVisibleRow - firstVisibleRow + 1), firstVisibleRow, distancePx: Math.abs(container.top + containerHeight / 2 - viewportCentre) });
  }
  return visible;
}

/** 📍️ One container's window at an EXACT row count: the slice starts `overscan` rows before the first visible one and always ends inside `total`. */
function treeWindowRequestAtRows(key: string, metrics: TreeWindowVisibleRows, rows: number, overscan: number): TreeWindowRequest {
  const bounded = Math.max(1, Math.min(Math.floor(rows), metrics.total));
  return { key, offset: Math.min(Math.max(0, metrics.firstVisibleRow - Math.max(0, overscan)), metrics.total - bounded), rows: bounded };
}

/**
 * 👀️ The windows the given viewport wants, for every container that intersects it — the ONE pure rule both the
 * React observer and its tests read. Off-screen containers are dropped entirely (no request, so a scrolled-away
 * branch keeps whatever it last materialised instead of churning). `offset` is the first visible row less the
 * overscan, clamped so the window always ends inside `total`; `rows` is the visible run plus one overscan per
 * edge, clamped to `[1, min(total, TREE_WINDOW_ROWS_MAX)]`. Per-container only — a body asking several
 * containers at once must run the answer through {@link capTreeWindowRequests}.
 * @see 🎫️ 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING · 📓️design-virtualised-tree.md §6.1
 **/
export function treeWindowRequestsForViewport(
  containers: readonly TreeWindowContainerMeasure[],
  viewportTop: number,
  viewportHeight: number,
  overscanRows: number,
): readonly TreeWindowRequest[] {
  const visible = treeWindowVisibleRowsForViewport(containers, viewportTop, viewportHeight);
  const overscan = Math.max(0, Math.floor(overscanRows));
  const requests: TreeWindowRequest[] = [];
  for (const container of containers) {
    const metrics = visible.get(container.key);
    if (!metrics) continue;
    requests.push(treeWindowRequestAtRows(container.key, metrics, Math.min(metrics.visibleRows + 2 * overscan, TREE_WINDOW_ROWS_MAX), overscan));
  }
  return requests;
}

/**
 * ✂️ Fits the WHOLE body's request inside {@link TREE_WINDOW_BODY_NODE_BUDGET}, in the guest's own cost model:
 * every container in the request costs `1 + rows` — its own node plus the rows it materialises — so a request
 * that fits here is a request the guest can answer in full, and the host never asks for rows it will silently
 * not be given (📓️s3-review-streaming-loop.md §1).
 *
 * Overscan goes first — uniformly, down to zero, so every container keeps its whole visible run before any of
 * them loses a row the user can see — and only then are rows trimmed, hardest from the containers whose centre
 * sits furthest from the viewport's, down to one row and finally to none. A container trimmed to zero rows stays
 * in the answer as a spacer-only window (it exists in the body either way and still costs its one node; dropping
 * it from the wire would only hand the guest back its own default). Requests keep their input order (the report
 * signature must not churn on a re-sort) and every answer still satisfies `offset + rows ≤ total`.
 * @see 🎫️ 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING · 📓️design-virtualised-tree.md §7
 **/
export function capTreeWindowRequests(requests: readonly TreeWindowRequest[], containersVisibleRows: ReadonlyMap<string, TreeWindowVisibleRows>, bodyNodeBudget: number = TREE_WINDOW_BODY_NODE_BUDGET): readonly TreeWindowRequest[] {
  const budget = Math.max(0, Math.floor(bodyNodeBudget));
  const cost = (entries: readonly { readonly rows: number }[]) => entries.length + entries.reduce((carry, entry) => carry + entry.rows, 0);
  if (cost(requests) <= budget) return requests;
  const entries = requests.map((request, index) => ({
    index,
    key: request.key,
    ceiling: request.rows,
    rows: request.rows,
    overscan: TREE_WINDOW_OVERSCAN_ROWS,
    metrics: containersVisibleRows.get(request.key) ?? { total: request.offset + request.rows, visibleRows: request.rows, firstVisibleRow: request.offset, distancePx: Number.POSITIVE_INFINITY },
  }));
  for (let overscan = TREE_WINDOW_OVERSCAN_ROWS - 1; overscan >= 0; overscan -= 1) {
    for (const entry of entries) {
      const shrunk = Math.max(1, entry.metrics.visibleRows + 2 * overscan);
      entry.rows = Math.min(entry.ceiling, shrunk);
      entry.overscan = shrunk < entry.ceiling ? overscan : TREE_WINDOW_OVERSCAN_ROWS;
    }
    if (cost(entries) <= budget) break;
  }
  let excess = cost(entries) - budget;
  const furthestFirst = [...entries].sort((left, right) => right.metrics.distancePx - left.metrics.distancePx || right.index - left.index);
  for (const entry of furthestFirst) {
    if (excess <= 0) break;
    const given = Math.min(excess, entry.rows - 1);
    if (given <= 0) continue;
    entry.rows -= given;
    entry.overscan = 0;
    excess -= given;
  }
  for (const entry of furthestFirst) {
    if (excess <= 0) break;
    if (entry.rows <= 0) continue;
    entry.rows = 0;
    excess -= 1;
  }
  return entries.map((entry) => (entry.rows > 0 ? treeWindowRequestAtRows(entry.key, entry.metrics, entry.rows, entry.overscan) : { key: entry.key, offset: Math.min(entry.metrics.firstVisibleRow, Math.max(0, entry.metrics.total - 1)), rows: 0 }));
}

/**
 * 📇️ Which entry of the child list the `position`-th RENDERED row of a windowed container is —
 * `undefined` for an unwindowed container, whose rows carry no index and are never measured as rows.
 * A `direction === "up"` tree renders the slice reversed, so the indices count back from its end.
 **/
export function treeWindowRowIndexOf(childWindow: TreeDataWindow | undefined, materialisedCount: number, position: number, direction: "down" | "up"): number | undefined {
  if (!childWindow) return undefined;
  const { leading } = treeWindowSpacerRows(childWindow, materialisedCount);
  return leading + (direction === "up" ? materialisedCount - 1 - position : position);
}

/** ♿️ What a windowed row tells assistive technology about its place among ALL its siblings: `aria-posinset` (its entry
 * index, one-based) and `aria-setsize` (the window's total, never fewer than the row's own position). The rows outside the
 * streamed slice are blank `aria-hidden` spacers, so without the pair a reader counts only the materialised rows and never
 * learns that a history row holds 120 mutations. `undefined` for an unwindowed row, whose siblings are all in the DOM. */
export function treeWindowRowAriaAttributes(windowRowIndex: number | undefined, windowRowTotal: number | undefined): { readonly "aria-posinset": number; readonly "aria-setsize": number } | undefined {
  if (windowRowIndex === undefined || windowRowTotal === undefined) return undefined;
  const position = Math.max(0, Math.floor(windowRowIndex)) + 1;
  return { "aria-posinset": position, "aria-setsize": Math.max(position, Math.floor(windowRowTotal)) };
}

/** 🕳️ Spacer row counts for a container that materialised `materialisedCount` rows of {@link TreeDataWindow}. */
export function treeWindowSpacerRows(childWindow: TreeDataWindow | undefined, materialisedCount: number): { readonly leading: number; readonly trailing: number } {
  if (!childWindow) return { leading: 0, trailing: 0 };
  const total = Math.max(0, Math.floor(childWindow.total));
  const leading = Math.min(Math.max(0, Math.floor(childWindow.offset)), total);
  return { leading, trailing: Math.max(0, total - leading - materialisedCount) };
}

/**
 * ⛓️ Joins one windowed container's `windowKey` onto its enclosing windowed containers' — U+241F SYMBOL FOR
 * UNIT SEPARATOR: the PRINTABLE glyph, not the C0 control character it depicts.
 *
 * 🧯️ The control character itself (U+001F) is what a separator is semantically for, and it does not work
 * here: every window path crosses the wasm boundary inside `PluginViewState.treeWindows`, and
 * `parseResolvedPluginViewState` (`🛂️manifest/🟦️.ts:977`) refuses any identifier matching
 * `[\u0000-\u001f\u007f]`. A single nested path therefore took the WHOLE view context down — every
 * `refreshUi` failed with `view context: invalid identifier` and every unrelated `interactionSelect` was
 * refused with `dispatch-failed`, measured on the fem3d House lane. U+241F is one code point, printable,
 * greppable, absent from every authored node key, and passes that law.
 *
 * 🧯️ A window is addressed by its PATH, never by its node key alone, because the node key is also the pick
 * target id the tree-level `interactionSelect` dispatches (`targets: [{granularity, id: record.key}]`) and so
 * cannot be namespaced to make it unique. Two containers legitimately share a key across a body — `📐️cad`
 * builds the same `object.id` under four pane sections, `🏗️fem` builds `case.id` and `combination.id` in one
 * body — and under key identity they would share one open state, one window, and each other's measurements
 * (📓️f2-sdk-body-node-ledger.md §10). The path is unique by construction and costs the ids nothing.
 *
 * 🔗️ Mirror of the ui-contract's Rust `TREE_WINDOW_PATH_SEPARATOR = "\u{1f}"`; kept on one line, as the
 * escape ``, for the parity law to read.
 * @see 🎫️ 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING · 📓️f2-sdk-body-node-ledger.md §10
 **/
export const TREE_WINDOW_PATH_SEPARATOR = "␟";

/** 🛤️ One windowed container's path: its enclosing windowed containers' keys, outermost first, then its own. A top-level section's path IS its key, so a flat body is unchanged. `undefined` for an unwindowed container, which has no window identity at all. */
export function treeWindowPathOf(parentWindowPath: string | undefined, windowKey: string | undefined): string | undefined {
  if (windowKey === undefined || windowKey.length === 0) return undefined;
  return parentWindowPath ? `${parentWindowPath}${TREE_WINDOW_PATH_SEPARATOR}${windowKey}` : windowKey;
}

/** 📮️ DOM mirror of a container's window, stamped on its branch content element — the element whose top edge IS row 0 of the child list, so a measurement of it plus these numbers is the whole observer contract. */
export interface TreeWindowDomAttributes {
  readonly "data-tree-window-key"?: string;
  readonly "data-tree-window-path"?: string;
  readonly "data-tree-window-total": number;
  readonly "data-tree-window-offset": number;
  readonly "data-tree-window-length": number;
  readonly "data-tree-window-row-extent": TreeWindowRowExtent;
}

/** 🏷️ Builds {@link TreeWindowDomAttributes} for a windowed container; `undefined` for an unwindowed one (no attributes, no spacers). `-key` stays the AUTHORED node key (and the pick target id); `-path` is the window's identity — what the host keys its state by and sends back on the wire. */
export function treeWindowDomAttributes(childWindow: TreeDataWindow | undefined, materialisedCount: number, windowKey: string | undefined, windowPath?: string): TreeWindowDomAttributes | undefined {
  if (!childWindow) return undefined;
  const total = Math.max(0, Math.floor(childWindow.total));
  const offset = Math.min(Math.max(0, Math.floor(childWindow.offset)), total);
  const path = windowPath ?? treeWindowPathOf(undefined, windowKey);
  return {
    ...(windowKey === undefined ? {} : { "data-tree-window-key": windowKey }),
    ...(path === undefined ? {} : { "data-tree-window-path": path }),
    "data-tree-window-total": total,
    "data-tree-window-offset": offset,
    "data-tree-window-length": materialisedCount,
    "data-tree-window-row-extent": childWindow.rowExtent,
  };
}

/**
 * 🖌️ The blank extent standing in for `rows` unmaterialised rows at one edge of a window. A plain block inside
 * the branch content — no {@link TreeAlignedRow}, so it claims no guide gutter and the branch's own indentation
 * lines paint straight through it — sized at exactly the row pitch so the rows below keep their real index.
 * Zero rows render nothing at all.
 **/
export function renderTreeWindowSpacer(rows: number, edge: "leading" | "trailing", rowExtent: TreeWindowRowExtent | undefined): React.ReactElement | null {
  if (rows <= 0) return null;
  if (rowExtent === undefined) throw new Error("A virtual Tree spacer requires its window's closed row extent.");
  return <div key={`tree-window-spacer-${edge}`} data-slot="tree-window-spacer" data-tree-window-spacer={edge} data-tree-window-rows={rows} aria-hidden="true" className="w-full min-w-0 shrink-0" style={{ height: `${rows * treeWindowRowExtentPx(rowExtent)}px` }} />;
}
