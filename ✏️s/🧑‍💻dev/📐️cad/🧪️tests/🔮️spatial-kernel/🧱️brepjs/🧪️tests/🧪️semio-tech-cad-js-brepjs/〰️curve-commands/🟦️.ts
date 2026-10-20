/** 〰️ The brepjs arc, circle, sphere, control point, interpolate, typology and fixture roundtrip tests. */
import type { ModelSpaceJson } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import type { Vec3 } from "@semio-tech/framework-3d-js";
import { deserializeWorkerValue, serializeWorkerValue } from "../../../📨️worker-and-kernel/🟦️.ts";
import { ModelSpace } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import { defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { geom } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import type { BrepjsTestKit } from "../🧰️brepjs-test-kit/🟦️.ts";
import type { BrepjsKernelHelpers } from "../🛠️kernel-fixture/🟦️.ts";

/** 🌀 Registers the arc, circle, sphere, control point curve, worker argument, interpolate curve, typology and concrete forest fixture tests. */
export function registerCurveCommandTests(kit: BrepjsTestKit, helpers: BrepjsKernelHelpers): void {
  const { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID, AEC_BUILDING_MODEL_DEFINITION_ID, AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID, Model, ModelSpace, defaultModelDefinitionId, deserializeWorkerValue, expect, geom, it, serializeWorkerValue, source } = kit;
  const { kernel } = helpers;
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
    const fixturePath = new URL("../../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🎮️play/🔣️.json", source.url);
    const snapshotJson = JSON.parse(await readFile(fixturePath, "utf8")) as ModelSpaceJson;
    const space = ModelSpace.fromJSON(snapshotJson);
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
}
