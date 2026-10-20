/** 🧊️ The attribute validation, edge and solid geometry, expr, model json, metadata and selection filter tests. */
import type { EdgeRecord, EdgeRef, ObjectRef, TypologyRef, VertexRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { ModelSpaceJson } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import type { Expr } from "@semio-tech/s-spatial-kernel-ts/geometry/expressions";
import type { SelectionEvent, SelectionSpec } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import type { Vec3 } from "@semio-tech/framework-3d-js";
import type { VecTestKit } from "../🧰️vec-test-kit/🟦️.ts";

/** 🏷 Registers the attribute validation, edge and solid geometry, expr, model json, metadata and selection filter tests. */
export function registerValidationGeometryAndMetadataTests(kit: VecTestKit): void {
  const { AttributeTable, CAD_E2E_ROUTES_MODEL_SPACE_JSON, M, Model, ModelSpace, describe, evalExpr, evalGuard, expandSelectionTargetsForAccept, expect, it, loadAttributeDefinition, parseModelJson, selectionEventMatches, validateAttributeValue } = kit;
  const geometryRoutesFixtureJson = JSON.parse(CAD_E2E_ROUTES_MODEL_SPACE_JSON) as ModelSpaceJson;
  describe("@semio-tech/cad-js/core attribute validation", () => {
    it("validates opening attribute options", () => {
      const defn = loadAttributeDefinition("spatial.shape.opening")!;
      expect(validateAttributeValue(defn, "window")).toBe(true);
      expect(validateAttributeValue(defn, "invalid")).toBe(false);
    });
  });
  describe("@semio-tech/cad-js/core edge and solid geometry", () => {
    it("arcEndOnCircle projects off-circle pick onto arc", () => {
      const end = M.arcEndOnCircle([0, 0, 0], [2, 0, 0], [0, 3, 0]);
      expect(end[0]).toBeCloseTo(0, 5);
      expect(end[1]).toBeCloseTo(2, 5);
      expect(M.vec3Distance([0, 0, 0], end)).toBeCloseTo(2, 5);
    });
    it("arcSamplePoints quarter arc from center start end", () => {
      const pts = M.arcSamplePoints([0, 0, 0], [2, 0, 0], [0, 2, 0], 4);
      expect(pts[0]).toEqual([2, 0, 0]);
      expect(pts[pts.length - 1]![0]).toBeCloseTo(0, 5);
      expect(pts[pts.length - 1]![1]).toBeCloseTo(2, 5);
    });
    it("circleSamplePoints and edgeCurveLength for circle curves", () => {
      const pts = M.circleSamplePoints([0, 0, 0], [0, 0, 1], 2, 64);
      expect(pts.length).toBeGreaterThan(8);
      expect(
        M.edgeCurveLength({ kind: "circle", center: [0, 0, 0], normal: [0, 0, 1], radius: 2 }, [
          [2, 0, 0],
          [2, 0, 0],
        ]),
      ).toBeCloseTo(M.cos(0) * 0 + M.sin(0) * 0 + 4 * 3.141592653589793, 3);
    });
    it("nurbs edge samples through control poles", () => {
      const curve = M.nurbsCurveFromPoles([
        [0, 0, 0],
        [1, 2, 0],
        [3, 0, 0],
      ] as Vec3[])!;
      const v0 = "v0" as VertexRef;
      const v1 = "v1" as VertexRef;
      const verts = {
        [v0]: { id: v0, position: [0, 0, 0] as Vec3 },
        [v1]: { id: v1, position: [3, 0, 0] as Vec3 },
      };
      const edge: EdgeRecord = { id: "e" as EdgeRef, vertexIds: [v0, v1], curve };
      const pts = M.edgeSamplePoints(verts, edge, 24);
      expect(pts.length).toBeGreaterThan(4);
    });
    it("solidPrimitiveAabb sphere bounds", () => {
      const b = M.solidPrimitiveAabb({ kind: "sphere", center: [1, 2, 3], radius: 5 });
      expect(b.min).toEqual([-4, -3, -2]);
      expect(b.max).toEqual([6, 7, 8]);
    });
  });
  describe("@semio-tech/cad-js/core expr", () => {
    it("evaluates numeric fold min expr", () => {
      const e: Expr = {
        kind: "fold",
        operation: "min",
        args: [
          { kind: "const", value: 3 },
          { kind: "const", value: 7 },
        ],
      };
      expect(evalExpr(e, { context: {}, preview: M })).toBe(3);
    });
    it("evaluates guards used by box factory", () => {
      const g: Expr = {
        kind: "all",
        args: [
          { kind: "exists", target: { root: "context", segments: [{ kind: "field", name: "origin" }] } },
          {
            kind: "binop",
            operation: ">",
            left: { kind: "path", root: "context", segments: [{ kind: "field", name: "height" }] },
            right: { kind: "const", value: 0 },
          },
        ],
      };
      expect(evalGuard(g, { context: { origin: [0, 0, 0], height: 2 } })).toBe(true);
    });
  });
  describe("@semio-tech/cad-js/core model json", () => {
    it("materializeInlineObjectPrimitives promotes play fixture wires into model geometry", () => {
      const space = ModelSpace.fromJSON(geometryRoutesFixtureJson as ModelSpaceJson);
      const model = space.models["spatial.shape"]!;
      expect(model.wires["stub-wire"]?.edgeIds.length).toBeGreaterThan(0);
      expect(model.objects["object-wire-stub-wire"]?.primitives.wire).toBe("stub-wire");
    });

    it("parseModelJson fills missing entity arrays with empty lists", () => {
      const model = parseModelJson({
        schema: "spatial.model",
        revision: 1,
        objects: [],
        geometry: {
          vertices: [{ id: "v0", position: [0, 0, 0] }],
          edges: [{ id: "e0", vertexIds: ["v0", "v0"] }],
        },
      });
      expect(model).not.toBeNull();
      expect(Object.keys(model!.anchors).length).toBe(0);
      expect(Object.keys(model!.vertices).length).toBe(1);
      expect(Object.keys(model!.edges).length).toBe(1);
    });
  });
  describe("@semio-tech/cad-js/core metadata", () => {
    it("AttributeTable setField bumps model revision", () => {
      const g = new Model();
      const r0 = g.revision;
      g.metadata.setField("e1", "exposure", "external");
      expect(g.revision).toBeGreaterThan(r0);
      expect(g.metadata.get("e1")?.exposure).toBe("external");
    });

    it("AttributeTable entries and JSON roundtrip", () => {
      const store = new AttributeTable(() => {});
      store.setField("face-1", "exposure", "external");
      store.setField("face-2", "uValue", 0.25);
      const json = store.toJSON();
      expect(json).toHaveLength(2);
      const restored = AttributeTable.fromJSON(json);
      expect(restored.get("face-1")?.exposure).toBe("external");
      expect(restored.get("face-2")?.uValue).toBe(0.25);
    });

    it("ModelJson metadata roundtrip", () => {
      const g = new Model();
      g.metadata.setField("solid-a", "tag", "roof");
      const back = Model.fromJSON(g.toJSON());
      expect(back.metadata.get("solid-a")?.tag).toBe("roof");
    });

    it("AttributeTable getEntityFlags and setEntityFlag roundtrip", () => {
      const g = new Model();
      expect(g.getEntityFlags("obj-1")).toEqual({});
      g.setEntityFlag("obj-1", "hidden", true);
      g.setEntityFlag("face-2", "locked", true);
      expect(g.getEntityFlags("obj-1")).toEqual({ hidden: true });
      expect(g.getEntityFlags("face-2")).toEqual({ locked: true });
      g.setEntityFlag("obj-1", "hidden", false);
      expect(g.getEntityFlags("obj-1")).toEqual({});
      const back = Model.fromJSON(g.toJSON());
      expect(back.getEntityFlags("face-2")).toEqual({ locked: true });
    });
  });
  describe("@semio-tech/cad-js/core selection filter", () => {
    it("selectionEventMatches rejects kinds outside accept", () => {
      const spec: SelectionSpec = { accept: ["face"], multiple: false };
      const ok: SelectionEvent = {
        kind: "selection.changed",
        targets: [{ kind: "face", id: "f1", editable: true }],
      };
      const bad: SelectionEvent = {
        kind: "selection.changed",
        targets: [{ kind: "edge", id: "s1", editable: false }],
      };
      expect(selectionEventMatches(spec, ok)).toBe(true);
      expect(selectionEventMatches(spec, bad)).toBe(false);
    });

    it("expandSelectionTargetsForAccept maps object picks to wire primitives", () => {
      const model = new Model();
      const typology = "spatial.shape.curve.interpolate-curve";
      model.objects[typology as ObjectRef] = {
        id: typology as ObjectRef,
        typology: typology as TypologyRef,
        primitives: { curve: "w-interp" },
      };
      const spec: SelectionSpec = { accept: ["wire", "edge"], multiple: true };
      const expanded = expandSelectionTargetsForAccept(model, spec, [{ kind: "object", id: typology, editable: true }]);
      expect(expanded).toEqual([{ kind: "wire", id: "w-interp", editable: true }]);
    });
  });
}
