import type * as brep from "../../🟦️.ts";

type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "isRenderableMeshTransfer">, source: TestSource): Promise<void> {
  const { isRenderableMeshTransfer } = dependencies;
  type MeshTransfer = brep.MeshTransfer;

  const { describe, expect, it } = vitest;

  describe("@semio-tech/geometry-brep-js", () => {
    it("isRenderableMeshTransfer accepts triangle meshes", () => {
      const mesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [{ start: 0, count: 3, entityId: "face-1" as brep.kernelGeometry.FaceRef }],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      expect(isRenderableMeshTransfer(mesh)).toBe(true);
    });
  });

}
