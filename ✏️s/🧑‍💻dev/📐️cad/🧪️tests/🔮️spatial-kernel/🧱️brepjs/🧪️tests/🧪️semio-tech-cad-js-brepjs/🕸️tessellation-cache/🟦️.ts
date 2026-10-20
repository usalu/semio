/** 🕸️ The brepjs sheared shell sync, tessellation cache and dispose tests. */
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import type { VertexRef } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { applyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { boxModelDiff } from "@semio-tech/s-spatial-kernel-ts/preview";
import { mesh } from "@semio-tech/cad-engine-ts/brepjs";
import { kernelGeometry } from "@semio-tech/framework-3d-js";
import type { BrepjsTestKit } from "../🧰️brepjs-test-kit/🟦️.ts";
import type { BrepjsKernelHelpers } from "../🛠️kernel-fixture/🟦️.ts";

/** 🧵 Registers the sheared shell sync, face entity mapping, cached mesh, dispose and face bucket tests. */
export function registerTessellationCacheTests(kit: BrepjsTestKit, helpers: BrepjsKernelHelpers): void {
  const { Model, applyModelDiff, boxModelDiff, expect, it, kernelGeometry } = kit;
  const { kernel } = helpers;
  it("syncSolidsFromModel follows sheared box shell (not axis-aligned primitive proxy)", async () => {
    const g = new Model();
    const solid = kernelGeometry.solidRef("sheared-box");
    applyModelDiff(g, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
    const corner = g.vertices["box-sheared-box-v111" as VertexRef];
    expect(corner).toBeDefined();
    g.vertices["box-sheared-box-v111" as VertexRef] = { id: corner!.id, position: [1.4, 1.2, 1] };
    g.bump();
    await kernel.syncSolidsFromModel(g);
    const vol = await kernel.volume(solid);
    expect(vol).toBeGreaterThan(1.05);
    expect(vol).toBeLessThan(1.35);
    const mesh = await kernel.tessellate(solid, 1e-3, g);
    expect(mesh.index.length).toBeGreaterThan(0);
  });
  it("tessellate maps brep faces to model FaceRef entityIds when model is provided", async () => {
    const g = new Model();
    const solid = kernelGeometry.solidRef("box-pick");
    applyModelDiff(g, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
    await kernel.syncSolidsFromModel(g);
    const meshTransfer = await kernel.tessellate(solid, 1e-3, g);
    expect(meshTransfer.faceInfos.length).toBeGreaterThan(0);
    const modelFaceIds = new Set(Object.keys(g.faces));
    for (const info of meshTransfer.faceInfos) {
      expect(typeof info.entityId).toBe("string");
      expect(modelFaceIds.has(String(info.entityId))).toBe(true);
    }
    for (const group of meshTransfer.faceGroups) {
      expect(modelFaceIds.has(String(group.entityId))).toBe(true);
    }
  });
  it("tessellate returns cached mesh with equal buffers for same solid and tolerance", async () => {
    const cell = await kernel.createBoxFromCorners({
      cornerA: [0, 0, 0],
      cornerB: [1, 1, 0],
      height: 1,
    });
    const tol = 1e-3;
    const a = await kernel.tessellate(cell, tol);
    const b = await kernel.tessellate(cell, tol);
    expect(a.index.length).toBe(b.index.length);
    expect(a.position.length).toBe(b.position.length);
    expect([...a.index]).toEqual([...b.index]);
  });
  it("disposeSolid clears tessellation cache for that solid", async () => {
    const cell = await kernel.createBoxFromCorners({
      cornerA: [0, 0, 0],
      cornerB: [1, 1, 0],
      height: 1,
    });
    const before = await kernel.tessellate(cell, 1e-3);
    kernel.disposeSolid(cell);
    const after = await kernel.tessellate(cell, 1e-3);
    expect(after.index.length).toBe(0);
    expect(before.index.length).toBeGreaterThan(0);
  });
  it("createBoxFromCornersDiff includes one face bucket", async () => {
    const r = await kernel.createBoxFromCornersDiff({
      cornerA: [0, 0, 0],
      cornerB: [1, 1, 0],
      height: 1,
    });
    expect(r.solid).toBeDefined();
    expect(Object.keys(r.diff.faces?.added ?? {}).length).toBeGreaterThan(0);
    expect(await kernel.volume(r.solid)).toBeGreaterThan(0);
  });
}
