type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { canvasDrawingPngExportPort, drawingSceneFromPreviewPayload, isDrawingRef } = dependencies;
  type DrawingScene = any;

  const { describe, expect, it } = vitest;

  describe("@semio-tech/s-2d-js", () => {
    it("recognizes drawing refs", () => {
      expect(isDrawingRef("drawing-1")).toBe(true);
      expect(isDrawingRef("solid-1")).toBe(false);
    });

    it("parses drawing scene preview payloads", () => {
      const scene = drawingSceneFromPreviewPayload({ width: 10, height: 20, nodes: [] });
      expect(scene).toEqual({ width: 10, height: 20, nodes: [] });
      expect(drawingSceneFromPreviewPayload({ error: "missing" })).toBeUndefined();
    });

    it("rasterizes a rect scene to png data url", () => {
      if (typeof document === "undefined") return;
      const scene: DrawingScene = {
        width: 100,
        height: 100,
        nodes: [
          {
            transform: [1, 0, 0, 1, 0, 0],
            node: { kind: "rect", x: 10, y: 10, width: 30, height: 20 },
            fill: { kind: "solid", color: [1, 0, 0, 1] },
          },
        ],
      };
      const png = canvasDrawingPngExportPort.exportPng(scene);
      expect(png.startsWith("data:image/png")).toBe(true);
    });
  });

}
