type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { renderMedia } = dependencies;

  const { describe, expect, it } = vitest;
  describe("renderMedia", () => {
    it("renders duration, position, and kind as key-value entries", () => {
      const node = renderMedia({ durationMs: 60_000, positionMs: 1_500, kind: "video" });
      if (node.component.type !== "keyValueList") throw new Error("expected keyValueList");
      expect(node.component.entries.map((entry) => entry.value)).toEqual(["60000", "1500", "video"]);
    });
  });

}
