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
  const { describe, expect, it } = import.meta.vitest;
  describe("renderTable", () => {
    it("serializes columns and rows into the table scene", () => {
      const node = renderTable({ columns: ["a", "b"], rows: [["1", "2"]] });
      if (node.type !== "componentScene") throw new Error("expected componentScene");
      expect(node.table?.columnsJson).toBe('["a","b"]');
      expect(node.table?.rowsJson).toBe('[["1","2"]]');
    });
  });
  describe("renderTableRows", () => {
    it("stamps a stable row id and omits the actions column when no row has one", () => {
      const node = renderTableRows({ columns: ["Name"], rows: [{ id: "space:abc", cells: ["Atelier"] }] });
      if (node.type !== "componentScene") throw new Error("expected componentScene");
      const columns = JSON.parse(node.table?.columnsJson ?? "[]") as { id: string }[];
      const rows = JSON.parse(node.table?.rowsJson ?? "[]") as { id: string; col0: { kind: string; value: string } }[];
      expect(rows[0]?.id).toBe("space:abc");
      expect(rows[0]?.col0).toEqual({ kind: "text", value: "Atelier" });
      expect(columns.some((column) => column.id === "actions")).toBe(false);
    });
    it("renders row action buttons carrying their dispatchable descriptor", () => {
      const action: ActionDescriptor = { controllerId: "s.space.home", action: "delete-space" };
      const node = renderTableRows({ columns: ["Name"], rows: [{ id: "space:abc", cells: ["Atelier"], actions: [{ iconId: "trash-2", action }] }] });
      if (node.type !== "componentScene") throw new Error("expected componentScene");
      const columns = JSON.parse(node.table?.columnsJson ?? "[]") as { id: string }[];
      const rows = JSON.parse(node.table?.rowsJson ?? "[]") as { actions: { buttons: { iconId: string; action: ActionDescriptor }[] } }[];
      expect(columns.some((column) => column.id === "actions")).toBe(true);
      expect(rows[0]?.actions.buttons[0]?.iconId).toBe("trash-2");
      expect(rows[0]?.actions.buttons[0]?.action).toEqual(action);
    });
  });
}
//#endregion 🧪️Tests
// #endregion 📊️TableWindowKit
