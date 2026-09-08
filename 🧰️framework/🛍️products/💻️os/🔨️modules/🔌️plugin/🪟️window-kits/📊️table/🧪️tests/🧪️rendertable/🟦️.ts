type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { renderTable, renderTableRows } = dependencies;
  type ActionDescriptor = any;

  const { describe, expect, it } = vitest;
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
