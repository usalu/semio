type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { SemioBrepKernel } = dependencies;

  const { beforeEach, describe, expect, it } = vitest;
  const { bootstrapCadModules } = await import("../../../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts");
  bootstrapCadModules();

  describe("@semio-tech/cad-js/spatial-kernel/semio", () => {
    const kernel = new SemioBrepKernel();

    beforeEach(async () => {
      await kernel.resetDerivedPipelineForTest();
    });

    it("createBoxFromCorners volume matches axis-aligned footprint×height", async () => {
      const cell = await kernel.createBoxFromCorners({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 4 });
      expect(await kernel.volume(cell)).toBeCloseTo(24, 3);
    });

    it("createBoxFromCornersDiff includes one face bucket and matching FaceRef entity ids on tessellate", async () => {
      const r = await kernel.createBoxFromCornersDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 });
      expect(Object.keys(r.diff.faces?.added ?? {}).length).toBe(6);
      const mesh = await kernel.tessellate(r.solid, 1e-3);
      expect(mesh.index.length).toBeGreaterThan(0);
      const modelFaceIds = new Set((r.diff.faces?.added ?? []).map((f) => String(f.id)));
      for (const info of mesh.faceInfos) expect(modelFaceIds.has(String(info.entityId))).toBe(true);
    });

    it("solid.sphere command creates a solid with the expected volume", async () => {
      const res = await kernel.executeCommandDiff("solid.sphere", { center: [0, 0, 0], radius: 2 });
      const added = res.diff.solids?.added?.[0];
      expect(added).toBeTruthy();
      const vol = await kernel.solidVolume(added!.id);
      expect(vol).toBeCloseTo((4 / 3) * Math.PI * 8, 0);
    });

    it("solid.booleanDifference cuts a sphere out of a box", async () => {
      const box = await kernel.executeCommandDiff("solid.sphere", { center: [0, 0, 0], radius: 5 });
      const boxId = box.diff.solids!.added![0]!.id;
      const sphere = await kernel.executeCommandDiff("solid.sphere", { center: [0, 0, 0], radius: 1 });
      const sphereId = sphere.diff.solids!.added![0]!.id;
      const res = await kernel.executeCommandDiff("solid.booleanDifference", { baseObjects: [{ id: boxId }], cutterObjects: [{ id: sphereId }] });
      const resultId = res.diff.solids?.added?.[0]?.id;
      expect(resultId).toBeTruthy();
      const vol = await kernel.solidVolume(resultId!);
      expect(vol).toBeGreaterThan(0);
      expect(vol).toBeLessThan((4 / 3) * Math.PI * 125);
    });

    it("curve.arc places start/end vertices on the requested circle", async () => {
      const res = await kernel.executeCommandDiff("curve.arc", { center: [0, 0, 0], start: [1, 0, 0], angle: 90 });
      const verts = res.diff.vertices?.added ?? [];
      expect(verts.length).toBe(2);
      expect(verts[0]!.position).toEqual([1, 0, 0]);
    });

    it("energy wall command (…From2PointsAndHeight) builds a box solid", async () => {
      const res = await kernel.executeCommandDiff("energy.energy.constructExternalWallFrom2PointsAndHeight", { pointA: [0, 0, 0], pointB: [4, 0, 0], height: 2.7 });
      expect(res.diff.solids?.added?.length).toBe(1);
      const vol = await kernel.solidVolume(res.diff.solids!.added![0]!.id);
      expect(vol).toBeGreaterThan(0);
    });
  });

}
