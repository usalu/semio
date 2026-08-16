// #region 📊️TableWindowKit
/// <reference types="vitest/importMeta" />
/** @emoji 📊️ `@semio-tech/plugin-window-kits` — TS twin of Rust `TableWindowKit` (`framework.window.table`). */
import type { TableScene, UiComponentSceneNode, UiNode } from "@semio-tech/framework";

/** 🆔️ Frozen kind id — twin of Rust `TableWindowKit::KIND_ID`. */
export const TABLE_WINDOW_KIND_ID = "framework.window.table";

/** 📊️ Flat column/row grid of plain string cells — twin of Rust `TableView`. */
export type TableView = {
  readonly columns: readonly string[];
  readonly rows: readonly (readonly string[])[];
};

/** 📊️ Twin of Rust `TableWindowKit::render` — builds a `table` component scene from `view`. */
export function renderTable(view: TableView): UiNode {
  const scene: TableScene = { columnsJson: JSON.stringify(view.columns), rowsJson: JSON.stringify(view.rows) };
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
}
//#endregion 🧪️Tests
// #endregion 📊️TableWindowKit
