import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "renderImage" | "renderImagePreview">, source: TestSource): Promise<void> {
  const { renderImage, renderImagePreview } = dependencies;

  const { describe, expect, it } = vitest;
  describe("renderImage", () => {
    it("builds a base64 data URI from mime + base64", () => {
      const node = renderImage({ width: 4, height: 2, mime: "image/png", base64: "QUJD" });
      if (node.component.type !== "image") throw new Error("expected image");
      expect(node.component.src).toBe("data:image/png;base64,QUJD");
    });

    it("matches the neutral EN/DE unavailable fixture while keeping the window mounted", () => {
      const fixture = JSON.parse(readFileSync(fileURLToPath(new URL("./🧫️fixtures/🚫️unavailable/🔣️.json", source.url)), "utf8"));
      for (const locale of ["en", "de"] as const) {
        const node = renderImagePreview({ availability: "unavailable" }, locale);
        expect(node.key).toBe(fixture.windowKey);
        expect(node.children).toHaveLength(1);
        expect(node.children[0]?.key).toBe(fixture.unavailableNodeKey);
        const component = node.children[0]?.component;
        if (component?.type !== "text") throw new Error("expected explicit unavailable text");
        expect(component.value).toBe(fixture.labels[locale]);
      }
    });
  });

}
