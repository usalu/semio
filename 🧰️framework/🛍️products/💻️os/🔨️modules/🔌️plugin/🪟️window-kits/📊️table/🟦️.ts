// #region 📊️TableWindowKit
/// <reference types="vitest/importMeta" />
/** @emoji 📊️ `@semio-tech/plugin-window-kits` — TS twin of Rust `TableWindowKit` (`framework.window.table`). */
import type { ActionDescriptor, TableScene, UiComponentSceneNode } from "@semio-tech/framework";

/** 🆔️ Frozen kind id — twin of Rust `TableWindowKit::KIND_ID`. */
export const TABLE_WINDOW_KIND_ID = "framework.window.table";

/** 📊️ Flat column/row grid of plain string cells — twin of Rust `TableView`. */
export type TableView = {
  readonly columns: readonly string[];
  readonly rows: readonly (readonly string[])[];
};

/** 📊️ Twin of Rust `TableWindowKit::render` — builds a `table` component scene from `view`. */
export function renderTable(view: TableView): UiComponentSceneNode {
  const scene: TableScene = { columnsJson: JSON.stringify(view.columns), rowsJson: JSON.stringify(view.rows) };
  const node: UiComponentSceneNode = { type: "componentScene", surfaceId: TABLE_WINDOW_KIND_ID, controllerId: TABLE_WINDOW_KIND_ID, componentKind: "table", table: scene };
  return node;
}

/** 🆔️ One row-scoped action button for `renderTableRows` — twin of Rust `TableRowAction`. Dispatches
 * `action` (a normal `ActionDescriptor`, already carrying whatever args its handler needs, e.g. the
 * row's own id) on click, rendered via the renderer's existing `TableCell` "buttons" cell kind. */
export type TableRowAction = { readonly iconId: string; readonly label?: string; readonly action: ActionDescriptor };

/** 🆔️ One identified, actionable row for `renderTableRows` — twin of Rust `TableRow`. `id` reaches the
 * React DOM as `data-row-id` (`Table/component.tsx`'s `getRowId` reads `row.id`); `cells` are plain
 * text, positional to `TableRowsView.columns`; `actions` render as one trailing "actions" column of
 * row buttons. */
export type TableRow = { readonly id: string; readonly cells: readonly string[]; readonly actions?: readonly TableRowAction[] };

/** 📊️ Identified-rows sibling of `TableView` — twin of Rust `TableRowsView`. `actionsLabel` is the
 * header for the trailing actions column (ignored when no row has an action); omit for an icon-only
 * header, matching the framework's own `sourcing::curation` precedent. */
export type TableRowsView = { readonly columns: readonly string[]; readonly rows: readonly TableRow[]; readonly actionsLabel?: string };

/** 🆔️ Twin of Rust `TableWindowKit::render_rows` — stamps a real per-row `id` and, when any row
 * declares one, a trailing actions column of button cells, instead of `renderTable`'s flat
 * positional-string grid. */
export function renderTableRows(view: TableRowsView): UiComponentSceneNode {
  const hasActions = view.rows.some((row) => (row.actions?.length ?? 0) > 0);
  const columns: { id: string; label: string }[] = view.columns.map((label, index) => ({ id: `col${index}`, label }));
  if (hasActions) columns.push({ id: "actions", label: view.actionsLabel ?? "" });
  const rows = view.rows.map((row) => {
    const record: Record<string, unknown> = { id: row.id };
    row.cells.forEach((value, index) => {
      record[`col${index}`] = { kind: "text", value };
    });
    if (hasActions) {
      record.actions = { kind: "buttons", buttons: (row.actions ?? []).map((action) => ({ iconId: action.iconId, label: action.label, action: action.action })) };
    }
    return record;
  });
  const scene: TableScene = { columnsJson: JSON.stringify(columns), rowsJson: JSON.stringify(rows) };
  const node: UiComponentSceneNode = { type: "componentScene", surfaceId: TABLE_WINDOW_KIND_ID, controllerId: TABLE_WINDOW_KIND_ID, componentKind: "table", table: scene };
  return node;
}

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️rendertable/🟦️.ts");
  await registerTests1(import.meta.vitest, { renderTable, renderTableRows }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
// #endregion 📊️TableWindowKit
