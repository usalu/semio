import * as ModuleUnderTest from "../../🟦️.ts";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "renderMesh">, source: TestSource): Promise<void> {
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

const moduleUnderTestUrl = pathToFileURL(resolve(import.meta.dirname, "../../🟦️.ts")).href;
const moduleUnderTestDirectory = resolve(import.meta.dirname, "../..");
const vitestApi = await import("vitest");
await registerTests1(vitestApi, { renderMesh: ModuleUnderTest.renderMesh }, { directory: moduleUnderTestDirectory, url: moduleUnderTestUrl });
