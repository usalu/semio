type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { renderMesh } = dependencies;

  const { describe, expect, it } = vitest;
  describe("renderMesh", () => {
    it("carries the JSON blobs into the world3d scene", () => {
      const node = renderMesh({ cameraJson: "{}", meshesJson: "[]", instancesJson: "[]", selectionJson: "[]" });
      if (node.type !== "componentScene") throw new Error("expected componentScene");
      expect(node.world3d?.cameraJson).toBe("{}");
    });
  });

}
