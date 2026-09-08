type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { Model, ObjectRef, SelectionTarget, TypologyRef, __spatialCoreTestKernel, __spatialCoreTestRuntime, appendCommittedMeshFaceToModel, applyModelDiff, deletableObjectIdsFromSelection, deleteObjectsFromModel, solidRef } = dependencies;
  type MeshTransfer = any;

  __spatialCoreTestRuntime!.bootstrapCadModules();
  const { preciseSpatialKernelMath } = __spatialCoreTestKernel!;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = vitest;

  describe("@semio-tech/cad-js/core model commit mesh", () => {
    it("appendCommittedMeshFaceToModel adds one mesh face from a triangle mesh", () => {
      const g = new Model();
      const mesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      appendCommittedMeshFaceToModel(g, mesh, "t0", M);
      expect(Object.keys(g.faces).length).toBe(1);
      expect(g.revision).toBeGreaterThan(0);
    });
  });
  describe("@semio-tech/cad-js/core model diff", () => {
    it("applyModelDiff then inverse restores counts", () => {
      const g = new Model();
      const mesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      const d = M.meshFaceModelDiff(mesh, "x");
      const inv = applyModelDiff(g, d);
      expect(Object.keys(g.faces).length).toBe(1);
      applyModelDiff(g, inv);
      expect(Object.keys(g.faces).length).toBe(0);
    });

    it("boxModelDiff creates selectable boundary and volume records", () => {
      const g = new Model();
      applyModelDiff(g, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 4 }, solidRef("box-solid")));
      expect(Object.keys(g.vertices).length).toBe(8);
      expect(Object.keys(g.edges).length).toBe(12);
      expect(Object.keys(g.wires).length).toBe(6);
      expect(Object.keys(g.faces).length).toBe(6);
      expect(Object.keys(g.shells).length).toBe(1);
      expect(Object.keys(g.solids)).toEqual(["box-solid"]);
    });

    it("deleteObjectsFromModel removes object rows but keeps geometry primitives", () => {
      const g = new Model();
      applyModelDiff(g, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-solid")));
      g.objects["box-a"] = { id: "box-a" as ObjectRef, typology: "spatial.shape.primitive.box" as TypologyRef, primitives: { solid: "box-solid" } };
      g.objects["box-b"] = { id: "box-b" as ObjectRef, typology: "spatial.shape.primitive.box" as TypologyRef, primitives: { solid: "box-solid" } };
      const removed = deleteObjectsFromModel(g, ["box-a", "missing"]);
      expect(removed).toEqual(["box-a"]);
      expect(g.objects["box-a"]).toBeUndefined();
      expect(g.objects["box-b"]).toBeTruthy();
      expect(Object.keys(g.solids)).toEqual(["box-solid"]);
    });

    it("deletableObjectIdsFromSelection keeps only object targets", () => {
      const selection: SelectionTarget[] = [
        { kind: "object", id: "box-a", editable: true },
        { kind: "solid", id: "box-solid", editable: true },
        { kind: "object", id: "box-a", editable: true },
      ];
      expect(deletableObjectIdsFromSelection(selection)).toEqual(["box-a"]);
    });
  });

}
