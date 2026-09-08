type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { renderImage } = dependencies;

  const { describe, expect, it } = vitest;
  describe("renderImage", () => {
    it("builds a base64 data URI from mime + base64", () => {
      const node = renderImage({ width: 4, height: 2, mime: "image/png", base64: "QUJD" });
      if (node.component.type !== "image") throw new Error("expected image");
      expect(node.component.src).toBe("data:image/png;base64,QUJD");
    });
  });

}
