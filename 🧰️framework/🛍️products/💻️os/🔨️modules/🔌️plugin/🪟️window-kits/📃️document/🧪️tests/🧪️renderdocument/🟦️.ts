type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { renderDocument } = dependencies;

  const { describe, expect, it } = vitest;
  describe("renderDocument", () => {
    it("renders one child per page", () => {
      const node = renderDocument({ pages: [{ text: "p1" }, { text: "p2" }] });
      if (node.component.type !== "container") throw new Error("expected container");
      expect(node.children.length).toBe(2);
    });
  });

}
