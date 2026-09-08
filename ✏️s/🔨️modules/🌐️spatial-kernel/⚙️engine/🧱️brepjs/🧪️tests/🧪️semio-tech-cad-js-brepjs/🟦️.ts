type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { BrepjsKernel, Model, ModelSpace, aabbDifferencePieces, aabbIntersect, applyModelDiff, boxModelDiff, defaultModelDefinitionId, deserializeWorkerValue, face, faceCentroid, fuseSolidsToExternalFaces, geom, kernelGeometry, mesh, modelObjectAabb, serializeWorkerValue, solidRef } = dependencies;
  type EdgeRef = any;
  type FaceRef = any;
  type ModelSpaceJson = any;
  type MutableSolidRecord = any;
  type ShellRef = any;
  type SolidRef = any;
  type Vec3 = any;
  type VertexRef = any;
  type WireRef = any;

  const { beforeEach, describe, expect, it } = vitest;
  const { bootstrapCadModules } = await import("../../../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts");
  const { AEC_BUILDING_MODEL_DEFINITION_ID } = await import("@semio-tech/cad-js-module-aec-building");
  const { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID } = await import("@semio-tech/cad-js-module-aec-building-energy");
  const { AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID } = await import("@semio-tech/cad-js-module-aec-building-structure");

  bootstrapCadModules();

  describe("@semio-tech/cad-js/brepjs", () => {
    const kernel = new BrepjsKernel();

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

    it("executeCommandDiff curve.arc places end vertex on circle not off-circle pick", async () => {
      const res = await kernel.executeCommandDiff("curve.arc", {
        center: [0, 0, 0],
        start: [2, 0, 0],
        end: [0, 3, 0],
      });
      const verts = res.diff.vertices?.added ?? [];
      expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
      expect(verts[1]!.position[1]).toBeCloseTo(2, 5);
    });

    it("executeCommandDiff curve.arc creates one arc edge between start and end", async () => {
      const res = await kernel.executeCommandDiff("curve.arc", {
        center: [0, 0, 0],
        start: [2, 0, 0],
        end: [0, 2, 0],
      });
      const verts = res.diff.vertices?.added ?? [];
      const edges = res.diff.edges?.added ?? [];
      const wires = res.diff.wires?.added ?? [];
      expect(verts).toHaveLength(2);
      expect(edges).toHaveLength(1);
      expect(wires).toHaveLength(1);
      expect(verts[0]!.position).toEqual([2, 0, 0]);
      expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
      expect(verts[1]!.position[1]).toBeCloseTo(2, 5);
      expect(verts[1]!.position[2]).toBeCloseTo(0, 5);
      expect(edges[0]!.curve).toEqual({ kind: "arc", center: [0, 0, 0] });
      expect(edges[0]!.vertexIds).toHaveLength(2);
    });

    it("executeCommandDiff curve.arc computes end from angle when end is missing", async () => {
      const res = await kernel.executeCommandDiff("curve.arc", {
        center: [0, 0, 0],
        start: [1, 0, 0],
        angle: 90,
      });
      const verts = res.diff.vertices?.added ?? [];
      expect(verts).toHaveLength(2);
      expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
      expect(verts[1]!.position[1]).toBeCloseTo(1, 5);
      expect(res.diff.edges?.added?.[0]?.curve).toEqual({ kind: "arc", center: [0, 0, 0] });
    });

    it("executeCommandDiff curve.circle creates closed circle edge with circle metadata", async () => {
      const res = await kernel.executeCommandDiff("curve.circle", {
        center: [1, 2, 0],
        radiusPoint: [4, 2, 0],
      });
      const verts = res.diff.vertices?.added ?? [];
      const edges = res.diff.edges?.added ?? [];
      expect(verts).toHaveLength(1);
      expect(verts[0]!.position).toEqual([4, 2, 0]);
      expect(edges[0]!.curve).toEqual({ kind: "circle", center: [1, 2, 0], normal: [0, 0, 1], radius: 3 });
      expect(edges[0]!.vertexIds[0]).toBe(edges[0]!.vertexIds[1]);
    });

    it("executeCommandDiff solid.sphere stores SolidPrimitive and brepjs solid", async () => {
      const res = await kernel.executeCommandDiff("solid.sphere", {
        center: [0, 0, 0],
        radius: 2,
      });
      const solids = res.diff.solids?.added ?? [];
      expect(solids[0]!.solid).toEqual({ kind: "sphere", center: [0, 0, 0], radius: 2 });
      const vol = await kernel.volume(solids[0]!.id);
      expect(vol).toBeCloseTo((4 / 3) * Math.PI * 8, 1);
    });

    it("executeCommandDiff curve.controlPointCurve creates nurbs edge", async () => {
      const res = await kernel.executeCommandDiff("curve.controlPointCurve", {
        points: [
          [0, 0, 0],
          [1, 2, 0],
          [3, 1, 0],
        ],
      });
      const edges = res.diff.edges?.added ?? [];
      expect(edges[0]!.curve?.kind).toBe("nurbs");
      if (edges[0]!.curve?.kind === "nurbs") {
        expect(edges[0]!.curve.poles).toHaveLength(3);
        expect(edges[0]!.curve.through).toBe(false);
      }
    });

    it("worker arg serialization roundtrips nested model in command params", () => {
      const g = new Model();
      const bag = serializeWorkerValue({
        model: g,
        points: [
          [0, 0, 0],
          [2, 1, 0],
        ],
      }) as Record<string, unknown>;
      expect(bag.model).toEqual(expect.objectContaining({ __modelJson: expect.objectContaining({ schema: "spatial.model" }) }));
      const restored = deserializeWorkerValue(bag) as { model: Model; points: readonly Vec3[] };
      expect(restored.model).toBeInstanceOf(Model);
      expect(restored.points).toHaveLength(2);
    });

    it("executeCommandDiff curve.interpolateCurve marks through-points nurbs", async () => {
      const g = new Model();
      const res = await kernel.executeCommandDiff("curve.interpolateCurve", {
        model: g,
        points: [
          [0, 0, 0],
          [2, 1, 0],
          [4, 0, 0],
        ],
      });
      const edges = res.diff.edges?.added ?? [];
      expect(edges[0]!.curve?.kind).toBe("nurbs");
      if (edges[0]!.curve?.kind === "nurbs") {
        expect(edges[0]!.curve.through).toBe(true);
        expect(edges[0]!.curve.poles).toHaveLength(3);
      }
      expect((res.diff.wires?.added ?? []).length).toBe(1);
    });

    it("executeCommandDiff typology constructFrom2PointsAndHeight builds a solid", async () => {
      const res = await kernel.executeCommandDiff("energy.energy.constructExternalWallFrom2PointsAndHeight", {
        pointA: [0, 0, 0],
        pointB: [4, 3, 0],
        height: 2.5,
      });
      expect((res.diff.solids?.added ?? []).length).toBeGreaterThanOrEqual(1);
    });

    it("concrete forest left play fixture roundtrips shape, building, energy, and structure models", async () => {
      const { readFile } = await import("node:fs/promises");
      const { resolve } = await import("node:path");
      const fixturePath = resolve(source.directoryname, "../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🎮️play/🔣️.json");
      const fixtureJson = JSON.parse(await readFile(fixturePath, "utf8")) as ModelSpaceJson;
      const space = ModelSpace.fromJSON(fixtureJson);
      const shape = space.models[defaultModelDefinitionId()]!;
      const building = space.models[AEC_BUILDING_MODEL_DEFINITION_ID]!;
      const energy = space.models[AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID]!;
      const structure = space.models[AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID]!;
      expect(Object.keys(shape.objects)).toHaveLength(1);
      expect(Object.keys(building.objects)).toHaveLength(11);
      expect(Object.keys(energy.objects)).toHaveLength(1);
      expect(Object.keys(structure.objects)).toHaveLength(11);
      expect(Object.keys(geom(shape).vertices).length).toBeGreaterThan(0);
      expect(Object.keys(geom(building).vertices).length).toBeGreaterThan(0);
      expect(Object.keys(geom(energy).vertices).length).toBeGreaterThan(0);
      expect(Object.keys(geom(structure).vertices).length).toBeGreaterThan(0);
      expect(Object.keys(geom(structure).solids)).toHaveLength(0);
      expect(Object.keys(geom(energy).solids)).toHaveLength(0);
    });

  });

}
