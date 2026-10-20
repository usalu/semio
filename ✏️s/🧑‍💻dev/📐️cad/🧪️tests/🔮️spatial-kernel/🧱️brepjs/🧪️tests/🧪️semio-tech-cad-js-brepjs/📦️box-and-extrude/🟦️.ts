/** 📦️ The brepjs box volume, tessellation, solid sync and extrude tests. */
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import type { EdgeRef, VertexRef, WireRef } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { applyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { boxModelDiff } from "@semio-tech/s-spatial-kernel-ts/preview";
import { mesh } from "@semio-tech/cad-engine-ts/brepjs";
import { kernelGeometry } from "@semio-tech/framework-3d-js";
import type { BrepjsTestKit } from "../🧰️brepjs-test-kit/🟦️.ts";
import type { BrepjsKernelHelpers } from "../🛠️kernel-fixture/🟦️.ts";

/** 🧊 Registers the box volume, tessellate, solid sync, extrude wire diff and extrude curve command tests. */
export function registerBoxAndExtrudeTests(kit: BrepjsTestKit, helpers: BrepjsKernelHelpers): void {
  const { Model, applyModelDiff, beforeEach, boxModelDiff, expect, it, kernelGeometry } = kit;
  const { kernel } = helpers;
  beforeEach(async () => {
    await kernel.resetDerivedPipelineForTest();
  });
  it("createBoxFromCorners volume matches axis-aligned footprint×height", async () => {
    const cell = await kernel.createBoxFromCorners({
      cornerA: [0, 0, 0],
      cornerB: [2, 3, 0],
      height: 4,
    });
    const vol = await kernel.volume(cell);
    expect(vol).toBeCloseTo(24, 3);
  });
  it("tessellate returns non-empty mesh for a box", async () => {
    const cell = await kernel.createBoxFromCorners({
      cornerA: [0, 0, 0],
      cornerB: [1, 1, 0],
      height: 1,
    });
    const meshTransfer = await kernel.tessellate(cell, 1e-3);
    expect(meshTransfer.index.length).toBeGreaterThan(0);
    expect(meshTransfer.position.length).toBeGreaterThan(0);
    expect(meshTransfer.faceGroups.length).toBeGreaterThan(0);
  });
  it("syncSolidsFromModel rebuilds box volume after planar vertex move (not stale primitive)", async () => {
    const g = new Model();
    const solid = kernelGeometry.solidRef("moved-box");
    applyModelDiff(g, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
    await kernel.syncSolidsFromModel(g);
    expect(await kernel.volume(solid)).toBeCloseTo(1, 3);
    for (const [id, vert] of Object.entries(g.vertices)) {
      if (!id.includes("moved-box") || vert.position[2] < 0.5) continue;
      g.vertices[id as VertexRef] = { id: vert.id, position: [vert.position[0], vert.position[1], vert.position[2] + 1] };
    }
    g.bump();
    await kernel.syncSolidsFromModel(g);
    expect(await kernel.volume(solid)).toBeCloseTo(2, 2);
    const mesh = await kernel.tessellate(solid, 1e-3, g);
    expect(mesh.index.length).toBeGreaterThan(0);
  });
  it("extrudeWireDiff registers kernel brep for tessellation without meshFaceModelDiff shell", async () => {
    const g = new Model();
    const v0 = "v0" as VertexRef;
    const v1 = "v1" as VertexRef;
    const e0 = "e0" as EdgeRef;
    const w0 = "w0" as WireRef;
    applyModelDiff(g, {
      vertices: {
        added: [
          { id: v0, position: [0, 0, 0] },
          { id: v1, position: [1, 0, 0] },
        ],
      },
      edges: { added: [{ id: e0, vertexIds: [v0, v1] }] },
      wires: { added: [{ id: w0, edgeIds: [e0] }] },
    });
    const { diff, solid } = await kernel.extrudeWireDiff({ wireId: w0, distance: 2, direction: [0, 0, 1], model: g });
    applyModelDiff(g, diff);
    g.bump();
    const mesh = await kernel.tessellate(solid, 1e-3, g);
    expect(mesh.index.length).toBeGreaterThan(0);
    expect(Object.keys(g.faces).some((id) => id.startsWith("cm-"))).toBe(false);
  });
  it("extrudeWireDiff lofts open nurbs interpolate wires to solids", async () => {
    const g = new Model();
    const res = await kernel.executeCommandDiff("curve.interpolateCurve", {
      model: g,
      points: [
        [0, 0, 0],
        [2, 1, 0],
        [4, 0, 0],
      ],
    });
    applyModelDiff(g, res.diff);
    const wireId = res.diff.wires?.added?.[0]?.id;
    expect(wireId).toBeTruthy();
    const { diff, solid } = await kernel.extrudeWireDiff({
      wireId: String(wireId),
      distance: 1.2,
      direction: [0, 0, 1],
      model: g,
    });
    expect(diff.solids?.added?.length).toBe(1);
    applyModelDiff(g, diff);
    g.bump();
    const mesh = await kernel.tessellate(solid, 1e-3, g);
    expect(mesh.index.length).toBeGreaterThan(0);
  });
  it("executeCommandDiff surface.extrudeCrv extrudes selected wires along direction", async () => {
    const g = new Model();
    const v0 = "v0" as VertexRef;
    const v1 = "v1" as VertexRef;
    const e0 = "e0" as EdgeRef;
    const w0 = "w0" as WireRef;
    applyModelDiff(g, {
      vertices: {
        added: [
          { id: v0, position: [0, 0, 0] },
          { id: v1, position: [1, 0, 0] },
        ],
      },
      edges: { added: [{ id: e0, vertexIds: [v0, v1] }] },
      wires: { added: [{ id: w0, edgeIds: [e0] }] },
    });
    const res = await kernel.executeCommandDiff("surface.extrudeCrv", {
      model: g,
      curves: [{ kind: "wire", id: w0 }],
      direction: [0, 0, 1],
      distance: 1.5,
      origin: [0, 0, 0],
      cursor: [0, 0, 1.5],
    });
    expect(res.diff.solids?.added?.length).toBe(1);
  });
}
