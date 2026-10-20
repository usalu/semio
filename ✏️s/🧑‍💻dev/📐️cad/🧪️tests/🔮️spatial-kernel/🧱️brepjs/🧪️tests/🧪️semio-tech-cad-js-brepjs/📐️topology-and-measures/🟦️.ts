/** 📐️ The brepjs topology preview, aabb, measure and adjacency tests. */
import { aabbVolume } from "@semio-tech/s-spatial-kernel-ts/preview/aabb";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import type { EdgeRef, FaceRef, MutableSolidRecord, ShellRef, SolidRef, VertexRef, WireRef } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import type { Vec3 } from "@semio-tech/framework-3d-js";
import { aabbDifferencePieces, aabbIntersect, modelObjectAabb } from "@semio-tech/s-spatial-kernel-ts/preview/aabb";
import { applyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { boxModelDiff } from "@semio-tech/s-spatial-kernel-ts/preview";
import { face } from "@semio-tech/cad-engine-ts/brepjs";
import { faceCentroid, fuseSolidsToExternalFaces } from "@semio-tech/s-spatial-kernel-ts/preview/face-geometry";
import { geom } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { kernelGeometry, solidRef } from "@semio-tech/framework-3d-js";
import type { BrepjsTestKit } from "../🧰️brepjs-test-kit/🟦️.ts";
import type { BrepjsKernelHelpers } from "../🛠️kernel-fixture/🟦️.ts";

/** 📏 Registers the topology preview, moved shell aabb, distance, area, volume, hull fuse, adjacency and aabb difference tests. */
export function registerTopologyAndMeasureTests(kit: BrepjsTestKit, helpers: BrepjsKernelHelpers): void {
  const { Model, aabbDifferencePieces, aabbIntersect, applyModelDiff, boxModelDiff, expect, faceCentroid, fuseSolidsToExternalFaces, geom, it, kernelGeometry, modelObjectAabb, solidRef } = kit;
  const { kernel } = helpers;
  it("topology preview geometry resolves face centroid and fuse external faces", () => {
    const model = new Model();
    const west = solidRef("west");
    const east = solidRef("east");
    applyModelDiff(model, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, west));
    applyModelDiff(model, boxModelDiff({ cornerA: [0, 0, 1], cornerB: [1, 1, 1], height: 1 }, east));
    const westTop = Object.keys(model.faces).find((id) => id.includes("face-top") && id.includes("west"))!;
    const face = model.faces[westTop as FaceRef]!;
    const centroid = faceCentroid(model, face);
    expect(centroid).not.toBeNull();
    expect(centroid![2]).toBeCloseTo(1, 3);
    const fused = fuseSolidsToExternalFaces(model, [west, east], {
      hullSolidId: "hull",
      contactPairs: [
        ["face-top", "face-bottom"],
        ["face-bottom", "face-top"],
      ],
      maxSeparation: 0.05,
    });
    const westTopFace = Object.keys(model.faces).find((id) => id.includes("west") && id.includes("face-top"))!;
    const eastBottomFace = Object.keys(model.faces).find((id) => id.includes("east") && id.includes("face-bottom"))!;
    expect(fused.externalFaces.map(String)).not.toContain(westTopFace);
    expect(fused.externalFaces.map(String)).not.toContain(eastBottomFace);
    expect(fused.externalFaces.some((id) => String(id).includes("west") && String(id).includes("face-bottom"))).toBe(true);
  });
  it("modelObjectAabb follows moved shell vertices when SolidPrimitive is stale", () => {
    const model = new Model();
    const cell = kernelGeometry.solidRef("box");
    applyModelDiff(model, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
    const rec = geom(model).solids[cell]! as MutableSolidRecord;
    rec.solid = { kind: "box", cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 };
    const before = modelObjectAabb(model, rec)!;
    let topId = Object.keys(geom(model).vertices)[0]!;
    let topZ = geom(model).vertices[topId]!.position[2];
    for (const [id, vert] of Object.entries(geom(model).vertices)) {
      if (vert.position[2] > topZ) {
        topZ = vert.position[2];
        topId = id;
      }
    }
    const top = geom(model).vertices[topId]!;
    geom(model).vertices[topId] = { id: top.id, position: [top.position[0], top.position[1], top.position[2] + 2] };
    const after = modelObjectAabb(model, rec)!;
    expect(after.max[2]).toBeGreaterThan(before.max[2] + 1);
  });
  it("vertexDistance matches graph positions", async () => {
    const g = new Model();
    const va = "va" as VertexRef;
    const vb = "vb" as VertexRef;
    g.vertices[va] = { id: va, position: [0, 0, 0] };
    g.vertices[vb] = { id: vb, position: [3, 4, 0] };
    expect(await kernel.vertexDistance(va, vb, g)).toBe(5);
  });
  it("faceArea sums boundary wire triangles", async () => {
    const g = new Model();
    const fid = "f0" as FaceRef;
    const wid = "w0" as WireRef;
    const v0 = "v0" as VertexRef;
    const v1 = "v1" as VertexRef;
    const v2 = "v2" as VertexRef;
    const e0 = "e0" as EdgeRef;
    const e1 = "e1" as EdgeRef;
    const e2 = "e2" as EdgeRef;
    g.vertices[v0] = { id: v0, position: [0, 0, 0] };
    g.vertices[v1] = { id: v1, position: [1, 0, 0] };
    g.vertices[v2] = { id: v2, position: [0, 1, 0] };
    g.edges[e0] = { id: e0, vertexIds: [v0, v1] };
    g.edges[e1] = { id: e1, vertexIds: [v1, v2] };
    g.edges[e2] = { id: e2, vertexIds: [v2, v0] };
    g.wires[wid] = { id: wid, edgeIds: [e0, e1, e2] };
    g.faces[fid] = {
      id: fid,
      wireIds: [wid],
    };
    const a = await kernel.faceArea(fid, g);
    expect(a).toBeCloseTo(0.5, 5);
  });
  it("solidVolume matches volume", async () => {
    const cell = await kernel.createBoxFromCorners({
      cornerA: [0, 0, 0],
      cornerB: [1, 1, 0],
      height: 1,
    });
    expect(await kernel.solidVolume(cell)).toBeCloseTo(await kernel.volume(cell), 6);
  });
  it("syncSolidsFromModel fuses from_geometry hull metadata into one solid volume", async () => {
    const g = new Model();
    applyModelDiff(g, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("west")));
    applyModelDiff(g, boxModelDiff({ cornerA: [3, 0, 0], cornerB: [4, 1, 0], height: 1 }, solidRef("east")));
    g.metadata.setField("from_geometry-hull", "fuseSourceSolidIds", ["west", "east"]);
    g.solids["from_geometry-hull" as SolidRef] = { id: "from_geometry-hull" as SolidRef, shellIds: [] };
    await kernel.syncSolidsFromModel(g);
    expect(await kernel.volume("from_geometry-hull" as SolidRef)).toBeCloseTo(2, 3);
  });
  it("adjacentSolids lists other solids sharing any face", async () => {
    const g = new Model();
    const f = "fs" as FaceRef;
    g.faces[f] = { id: f, wireIds: [] };
    const s0 = "s0" as ShellRef;
    const s1 = "s1" as ShellRef;
    g.shells[s0] = { id: s0, faceIds: [f] };
    g.shells[s1] = { id: s1, faceIds: [f] };
    g.solids["c0" as SolidRef] = { id: "c0" as SolidRef, shellIds: [s0] };
    g.solids["c1" as SolidRef] = { id: "c1" as SolidRef, shellIds: [s1] };
    const adj = await kernel.adjacentSolids("c0" as SolidRef, g);
    expect(adj.map(String).sort()).toEqual(["c1"]);
  });
  it("sharedFacesBetween returns shared face ids", async () => {
    const g = new Model();
    const f = "fx" as FaceRef;
    g.faces[f] = { id: f, wireIds: [] };
    const sa = "sa" as ShellRef;
    const sb = "sb" as ShellRef;
    g.shells[sa] = { id: sa, faceIds: [f] };
    g.shells[sb] = { id: sb, faceIds: [f] };
    g.solids["ca" as SolidRef] = { id: "ca" as SolidRef, shellIds: [sa] };
    g.solids["cb" as SolidRef] = { id: "cb" as SolidRef, shellIds: [sb] };
    const xs = await kernel.sharedFacesBetween("ca" as SolidRef, "cb" as SolidRef, g);
    expect(xs).toEqual([f]);
  });
  it("aabbDifferencePieces volume equals solid minus intersection overlap", () => {
    const cell = { min: [0, 0, 0] as Vec3, max: [2, 2, 2] as Vec3 };
    const other = { min: [1, 1, 0] as Vec3, max: [3, 3, 2] as Vec3 };
    const inter = aabbIntersect(cell, other)!;
    const pieces = aabbDifferencePieces(cell, [inter]);
    const pieceVol = pieces.reduce((acc, p) => acc + aabbVolume(p), 0);
    expect(pieceVol).toBeCloseTo(aabbVolume(cell) - aabbVolume(inter), 4);
  });
}
