type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { renderTree } = dependencies;

  const { describe, expect, it } = vitest;
  describe("renderTree", () => {
    it("expands nested children recursively", () => {
      const node = renderTree({ roots: [{ id: "root", label: "Root", children: [{ id: "child", label: "Child" }] }] });
      if (node.component.type !== "tree") throw new Error("expected tree");
      expect(node.children.length).toBe(1);
      const rootItem = node.children[0]!.children[0]!;
      expect(rootItem.key).toBe("root");
      expect(rootItem.children[0]?.key).toBe("child");
    });
  });

}
