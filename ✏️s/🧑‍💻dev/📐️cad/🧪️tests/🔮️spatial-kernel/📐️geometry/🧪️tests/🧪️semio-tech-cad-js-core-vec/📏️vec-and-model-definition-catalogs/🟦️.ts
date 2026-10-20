/** 📏️ The vec and model definition catalog tests. */
import { applyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { ObjectRef, TypologyRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { SpatialKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { typologyAssetCensus } from "../🧰️vec-test-kit/🟦️.ts";
import type { VecTestKit } from "../🧰️vec-test-kit/🟦️.ts";

/** 📚 Registers the vec arithmetic tests and the model definition catalog tests. */
export function registerVecAndModelDefinitionCatalogTests(kit: VecTestKit): void {
  const { BrepjsKernel, M, Model, computeStat, defaultModelDefinitionId, derivePropertyValue, describe, expect, formatStatOutputValue, it, listApplicablePropertyDefinitionsForModelDefinition, listModelDefinitionAttributeDefinitions, listModelDefinitionPropertyDefinitions, listModelDefinitionStatDefinitions, listModelDefinitionTypologies, listStatDefinitionsForModelDefinition, loadAttributeDefinition, loadPropertyDefinition, loadStatDefinition, loadTypology, objectMatchesTypologyPrimitives, objectsForStatCompute, resolveModelDefinitionScope, resolveTypologyStyle, solidRef, source } = kit;
  describe("@semio-tech/cad-js/core vec", () => {
    it("adds and distances", () => {
      expect(M.vec3Distance([0, 0, 0], [3, 4, 0])).toBe(5);
    });
  });
  describe("@semio-tech/cad-js/core model definition catalogs", () => {
    it("loads attribute and property definition assets", () => {
      const attributes = listModelDefinitionAttributeDefinitions();
      const properties = listModelDefinitionPropertyDefinitions();
      expect(attributes.length).toBeGreaterThanOrEqual(6);
      expect(properties.some((row) => row.id === "spatial.shape.volume")).toBe(true);
      expect(loadAttributeDefinition("spatial.shape.material")?.field).toBe("material");
      expect(loadPropertyDefinition("spatial.shape.volume")?.unit).toBe("volume");
    });
    it("loads stat definition assets", () => {
      const stats = listModelDefinitionStatDefinitions();
      expect(stats.some((row) => row.id === "spatial.shape.geometry")).toBe(true);
      expect(stats.some((row) => row.id === "energy.demand")).toBe(true);
      expect(stats.some((row) => row.id === "structure.stability")).toBe(true);
      expect(loadStatDefinition("spatial.shape.geometry")?.outputs.some((row) => row.key === "totalVolume")).toBe(true);
      expect(listStatDefinitionsForModelDefinition("aec.building.energy").map((row) => row.id)).toContain("energy.demand");
      expect(resolveModelDefinitionScope(defaultModelDefinitionId()).statDefinitions.map((row) => row.id)).toContain("spatial.shape.geometry");
      expect(formatStatOutputValue(0.456, "percent")).toBe("45.6%");
    });
    it("computes spatial.shape.geometry stats for solid-backed objects", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      model.objects["obj-a"] = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      const defn = loadStatDefinition("spatial.shape.geometry")!;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const out = await computeStat(defn, {
        model,
        kernel,
        modelDefinitionId: defaultModelDefinitionId(),
        scope: "model",
        objects: objectsForStatCompute(model, defaultModelDefinitionId(), defn, "model", []),
      });
      expect(out.objectCount).toBe(1);
      expect(out.solidCount).toBe(1);
      expect(out.totalVolume).toBeCloseTo(1, 3);
      expect(out.sizeX).toBeCloseTo(1, 3);
      expect(out.sizeY).toBeCloseTo(1, 3);
      expect(out.sizeZ).toBeCloseTo(1, 3);
    });
    it("computes energy and structure stats with finite outputs", async () => {
      const energyModel = new Model();
      applyModelDiff(energyModel, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("hull-solid")));
      energyModel.objects["hull"] = {
        id: "hull" as ObjectRef,
        typology: "energy.energy.hull" as TypologyRef,
        primitives: { solid: "hull-solid" },
      };
      const energyDefn = loadStatDefinition("energy.demand")!;
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const energyOut = await computeStat(energyDefn, {
        model: energyModel,
        kernel,
        modelDefinitionId: "aec.building.energy",
        scope: "model",
        objects: objectsForStatCompute(energyModel, "aec.building.energy", energyDefn, "model", []),
      });
      expect(Number.isFinite(energyOut.heatedVolume)).toBe(true);
      expect(Number.isFinite(energyOut.annualHeatingDemand)).toBe(true);
      expect(energyOut.heatedVolume).toBeGreaterThan(0);

      const structureModel = new Model();
      applyModelDiff(structureModel, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [0.4, 0.4, 0], height: 3 }, solidRef("column-solid")));
      structureModel.objects["column"] = {
        id: "column" as ObjectRef,
        typology: "structure.structure.reinforcedconcretecolumn" as TypologyRef,
        primitives: { solid: "column-solid" },
      };
      const structureDefn = loadStatDefinition("structure.stability")!;
      const structureOut = await computeStat(structureDefn, {
        model: structureModel,
        kernel,
        modelDefinitionId: "aec.building.structure",
        scope: "model",
        objects: objectsForStatCompute(structureModel, "aec.building.structure", structureDefn, "model", []),
      });
      expect(structureOut.elementCount).toBe(1);
      expect(Number.isFinite(structureOut.estimatedMass)).toBe(true);
      expect(structureOut.stabilityIndex).toBeGreaterThan(0);
      expect(structureOut.stabilityIndex).toBeLessThanOrEqual(1);
    });
    it("loads geometry and AEC typology assets", async () => {
      const census = await typologyAssetCensus(source);
      expect(census.misnamed).toEqual([]);
      expect(listModelDefinitionTypologies().map((row) => row.id).sort()).toEqual(census.ids);
      expect(loadTypology("energy.energy.hull")?.properties).toContain("energy.heatedvolume");
      expect(loadTypology("energy.energy.hull")?.properties).toContain("spatial.shape.volume");
    });
    it("assigns primitiveKinds to geometry typologies", () => {
      const box = loadTypology("spatial.shape.primitive.box");
      const line = loadTypology("spatial.shape.curve.line");
      const plane = loadTypology("spatial.shape.surface.plane");
      expect(box?.primitiveKinds).toEqual(["solid"]);
      expect(line?.primitiveKinds).toEqual(["curve"]);
      expect(plane?.primitiveKinds).toEqual(["surface"]);
    });
    it("assigns primitiveKinds to AEC typologies", () => {
      expect(loadTypology("building.building.slab")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("building.building.wall")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("building.building.beam")?.primitiveKinds).toEqual(["curve"]);
      expect(loadTypology("building.building.column")?.primitiveKinds).toEqual(["solid"]);
      expect(loadTypology("energy.energy.externalwall")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("energy.energy.hull")?.primitiveKinds).toEqual(["solid"]);
      expect(loadTypology("structure.structure.onewayreinforcedconcreteslab")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("structure.linefem.lineelement")?.primitiveKinds).toEqual(["curve"]);
      expect(loadTypology("structure.surfacefem.surfaceelement")?.primitiveKinds).toEqual(["surface"]);
      expect(loadTypology("structure.solidfem.solidelement")?.primitiveKinds).toEqual(["solid"]);
    });
    it("resolveTypologyStyle is deterministic and distinct per typology id", () => {
      const a = resolveTypologyStyle("spatial.shape.primitive.box");
      const b = resolveTypologyStyle("spatial.shape.primitive.box");
      const c = resolveTypologyStyle("spatial.shape.curve.line");
      expect(a).toEqual(b);
      expect(a.color).toMatch(/^#[0-9a-f]{6}$/i);
      expect(c.color).not.toBe(a.color);
      expect(a.pattern.kind).toBe("none");
    });
    it("resolveTypologyStyle merges authored typology style overrides", () => {
      const slab = resolveTypologyStyle("structure.structure.onewayreinforcedconcreteslab");
      expect(slab.color).toBe("#8B7355");
      expect(slab.pattern.kind).toBe("hatch");
      expect(slab.pattern.direction).toBe(0);
      const wall = resolveTypologyStyle("structure.structure.reinforcedconcreteexternalwall");
      expect(wall.pattern.kind).toBe("crosshatch");
      expect(wall.edgeColor).toBe("#3D5560");
      const roof = resolveTypologyStyle("energy.energy.roof");
      expect(roof.pattern.kind).toBe("hatch");
      expect(roof.pattern.direction).toBe(90);
    });
    it("derives spatial.shape.volume for solid-backed objects", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      const object = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      const defn = loadPropertyDefinition("spatial.shape.volume")!;
      const kernel = {
        syncSolidsFromModel: async () => {},
        solidVolume: async () => 42,
      } as unknown as SpatialKernel;
      const out = await derivePropertyValue(defn, { model, kernel, object });
      expect(out.volume).toBe(42);
      expect(listApplicablePropertyDefinitionsForModelDefinition(defaultModelDefinitionId(), model, object).map((row) => row.id)).toContain("spatial.shape.volume");
    });
    it("validates object geometry against typology primitiveKinds", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const solidId = Object.keys(model.solids)[0]!;
      model.objects["obj-a"] = {
        id: "obj-a" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: solidId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-a"]!)).toBe(true);
      model.objects["obj-b"] = {
        id: "obj-b" as ObjectRef,
        typology: "spatial.shape.selection.select-all" as TypologyRef,
        primitives: { solid: solidId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-b"]!)).toBe(false);
      model.objects["obj-c"] = {
        id: "obj-c" as ObjectRef,
        typology: "building.building.slab" as TypologyRef,
        primitives: { solid: solidId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-c"]!)).toBe(false);
      const faceId = Object.keys(model.faces)[0]!;
      model.objects["obj-d"] = {
        id: "obj-d" as ObjectRef,
        typology: "building.building.slab" as TypologyRef,
        primitives: { surface: faceId },
      };
      expect(objectMatchesTypologyPrimitives(model, model.objects["obj-d"]!)).toBe(true);
    });
  });
}
