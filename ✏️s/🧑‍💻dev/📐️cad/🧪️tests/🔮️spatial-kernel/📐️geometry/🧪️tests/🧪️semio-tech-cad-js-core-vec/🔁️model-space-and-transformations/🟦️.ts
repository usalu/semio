/** 🔁️ The model space, hashing and transformation tests. */
import { applyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { CAD_GUMBALL_HIDDEN, cadGumballConfigVisible, collectGeometrySelectionTargets, modelDefinitionActionRegistry, runRegisteredAction } from "@semio-tech/cad-cad-ts/standards/1/subsets/any/editor/engine/actions";
import type { ObjectRef, TypologyRef } from "@semio-tech/s-spatial-kernel-ts/geometry";
import type { ModelSpaceJson } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import type { SpatialKernel, SpatialPreviewKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { VecTestKit } from "../🧰️vec-test-kit/🟦️.ts";

/** 🧮 Registers the model space and hashing tests and the transformation tests. */
export function registerModelSpaceAndTransformationTests(kit: VecTestKit): void {
  const { BrepjsKernel, M, Model, ModelSpace, actionAvailableInModelDefinition, applyTransformation, buildModelPrimitiveDocument, countViewObjectsForModelDefinition, defaultModelDefinitionId, derivePropertyValue, describe, expect, hashModelPrimitives, hashModelVertices, hashSolidRecord, hashVertexPosition, it, listAttributeDefinitionsForModelDefinitionEntity, listModelDefinitionManifests, listModelObjectsForModelDefinition, listPropertyDefinitionsForModelDefinition, listSelectionOperationsForModelDefinition, listTransformationsFromModelDefinition, listTransformationsIntoModelDefinition, listTypologiesForModelDefinition, loadPropertyDefinition, loadTransformation, resolveModelDefinitionScope, solidRef, source } = kit;
  describe("@semio-tech/cad-js/core model space and hashing", () => {
    it("hashes vertex positions stably", () => {
      const a = hashVertexPosition([1, 2, 3]);
      const b = hashVertexPosition([1.0000000004, 2, 3]);
      expect(a).toBe(b);
      expect(hashVertexPosition([1, 2, 4])).not.toBe(a);
    });
    it("links models in a model space", () => {
      const space = new ModelSpace();
      const m = new Model();
      applyModelDiff(m, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      space.link("primary", m);
      expect(space.get("primary")).toBe(m);
      const hashes = space.vertexHashesByModel();
      expect(Object.keys(hashes.primary ?? {}).length).toBe(8);
      const geo = space.geometryHashesByModel().primary;
      expect(Object.keys(geo?.solid ?? {}).length).toBeGreaterThanOrEqual(1);
      const roundTrip = ModelSpace.fromJSON(space.toJSON());
      expect(roundTrip.get("primary")?.revision).toBe(m.revision);
      expect(hashModelVertices(roundTrip.get("primary")!)).toEqual(hashes.primary);
    });
    it("hashes edges and solids deterministically", () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-a")));
      const hashes = hashModelPrimitives(model);
      const solidId = Object.keys(model.solids)[0]!;
      const edgeId = Object.keys(model.edges)[0]!;
      expect(hashes.solid?.[solidId]).toBeTruthy();
      expect(hashes.edge?.[edgeId]).toBeTruthy();
      expect(hashes.solid?.[solidId]).toBe(hashSolidRecord(model.solids[solidId]!));
    });
  });
  describe("@semio-tech/cad-js/core transformations", () => {
    it("detects visible CAD gumball configs", () => {
      expect(cadGumballConfigVisible(CAD_GUMBALL_HIDDEN)).toBe(false);
      expect(cadGumballConfigVisible({ ...CAD_GUMBALL_HIDDEN, rotate: true })).toBe(true);
      expect(cadGumballConfigVisible(null)).toBe(false);
    });

    it("lists model definition manifests and transformation directions", () => {
      const manifests = listModelDefinitionManifests();
      expect(manifests.some((row) => row.id === "spatial.shape")).toBe(true);
      expect(manifests.some((row) => row.id === "aec.building.energy")).toBe(true);
      expect(listTransformationsIntoModelDefinition("aec.building.energy").some((row) => row.id === "from_geometry")).toBe(true);
      expect(listTransformationsFromModelDefinition("spatial.shape").some((row) => row.target.modelDefinition === "aec.building.energy")).toBe(true);
    });
    it("scopes catalogs to active model definition", () => {
      const shape = resolveModelDefinitionScope(defaultModelDefinitionId());
      const energy = resolveModelDefinitionScope("aec.building.energy");
      expect(shape.interactions.some((row) => row.id === "primitive.box")).toBe(true);
      expect(energy.interactions.length).toBe(5);
      expect(energy.interactions.every((row) => row.id.startsWith("energy.energy.construct"))).toBe(true);
      expect(energy.interactions.some((row) => row.id === "primitive.box")).toBe(false);
      expect(energy.typologies.some((row) => row.id === "energy.energy.hull")).toBe(true);
      expect(energy.selectionEntityKinds).toContain("object");
      expect(energy.selectionEntityKinds).toContain("solid");
      expect(shape.selectionEntityKinds).toContain("vertex");
      expect(shape.selectionEntityKinds).toContain("face");
      const structure = resolveModelDefinitionScope("aec.building.structure");
      expect(structure.selectionEntityKinds).toContain("face");
      expect(structure.selectionEntityKinds).toContain("object");
      expect(listAttributeDefinitionsForModelDefinitionEntity("aec.building.structure", "face").some((row) => row.field === "exposure")).toBe(true);
      expect(listPropertyDefinitionsForModelDefinition("aec.building.energy").some((row) => row.id === "energy.heatedvolume")).toBe(true);
      expect(actionAvailableInModelDefinition("primitive.createBoxFromCorners", defaultModelDefinitionId())).toBe(true);
      expect(actionAvailableInModelDefinition("primitive.createBoxFromCorners", "aec.building.energy")).toBe(false);
      expect(listSelectionOperationsForModelDefinition(defaultModelDefinitionId()).some((row) => row.id === "selection.selectVertices")).toBe(true);
      expect(listSelectionOperationsForModelDefinition("aec.building.energy").length).toBe(0);
    });
    it("buildModelPrimitiveDocument nests shell through vertex under solid", () => {
      const model = new Model();
      const solid = solidRef("box-solid");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      const tree = buildModelPrimitiveDocument(model, String(solid));
      expect(tree?.kind).toBe("solid");
      expect(tree?.children).toHaveLength(1);
      const shell = tree!.children[0]!;
      expect(shell.kind).toBe("shell");
      expect(shell.children.length).toBeGreaterThan(0);
      const face = shell.children[0]!;
      expect(face.kind).toBe("face");
      expect(face.children.some((row) => row.kind === "wire")).toBe(true);
      const wire = face.children.find((row) => row.kind === "wire")!;
      expect(wire.children.some((row) => row.kind === "edge")).toBe(true);
      const edge = wire.children.find((row) => row.kind === "edge")!;
      expect(edge.children.every((row) => row.kind === "vertex")).toBe(true);
      expect(edge.children.length).toBe(2);
    });

    it("listModelObjectsForModelDefinition filters objects by typology ownership", () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["hull"] = { id: "hull" as ObjectRef, typology: "energy.energy.hull" as TypologyRef, primitives: { solid: String(cell) } };
      model.objects["other"] = { id: "other" as ObjectRef, typology: "spatial.shape.primitive.box" as TypologyRef, primitives: { solid: String(cell) } };
      expect(listModelObjectsForModelDefinition(model, "aec.building.energy").map((row) => String(row.id))).toEqual(["hull"]);
      expect(countViewObjectsForModelDefinition(model, "aec.building.energy")).toBe(1);
    });

    it("listModelObjectsForModelDefinition includes kernel typology objects on shape model definition", () => {
      const model = new Model();
      const cell = solidRef("imported");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["imported"] = {
        id: "imported" as ObjectRef,
        typology: "spatial.shape.kernel.solid" as TypologyRef,
        primitives: { solid: String(cell) },
      };
      expect(listModelObjectsForModelDefinition(model, defaultModelDefinitionId()).map((row) => String(row.id))).toEqual(["imported"]);
    });
    it("listModelObjectsForModelDefinition lists BIM class objects for aec.building", async () => {
      const { readFile } = await import("node:fs/promises");
      const fixturePath = new URL("../../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🎮️play/🔣️.json", source.url);
      const snapshotJson = JSON.parse(await readFile(fixturePath, "utf8")) as ModelSpaceJson;
      const space = ModelSpace.fromJSON(snapshotJson);
      const building = space.models["aec.building"]!;
      expect(listTypologiesForModelDefinition("aec.building").some((row) => row.id === "building.building.column")).toBe(true);
      expect(listModelObjectsForModelDefinition(building, "aec.building")).toHaveLength(11);
    });
    it("collectGeometrySelectionTargets scopes object rows to model definition typologies", () => {
      const model = new Model();
      const cell = solidRef("c0");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      model.objects["hull"] = { id: "hull" as ObjectRef, typology: "energy.energy.hull" as TypologyRef, primitives: { solid: String(cell) } };
      model.objects["other"] = { id: "other" as ObjectRef, typology: "spatial.shape.primitive.box" as TypologyRef, primitives: { solid: String(cell) } };
      const all = collectGeometrySelectionTargets(model, ["object"], "aec.building.energy");
      expect(all.map((row) => row.id)).toEqual(["hull"]);
    });
    it("ActionRegistry rejects out-of-scope actions for active model definition", async () => {
      const actions = modelDefinitionActionRegistry();
      const model = new Model();
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      await expect(
        runRegisteredAction(
          actions,
          "primitive.createBoxFromCorners",
          { cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1, __context: {}, __event: { kind: "test" } },
          { kernel, preview: kernel as unknown as SpatialPreviewKernel, model, activeModelDefinitionId: "aec.building.energy" },
        ),
      ).rejects.toThrow(/not available in model definition aec\.building\.energy/);
    });
    it("loads and applies from_geometry transformation", () => {
      const spec = loadTransformation("aec.building.energy.from_geometry");
      expect(spec?.source.modelDefinition).toBe("spatial.shape");
      expect(spec?.target.modelDefinition).toBe("aec.building.energy");
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      source.objects["geom"] = {
        id: "geom" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "box" },
      };
      const target = applyTransformation(spec!, source, M);
      expect(target.objects["energy.energy.hull"]?.typology).toBe("energy.energy.hull");
      expect(target.objects["energy.energy.hull"]?.primitives).toEqual({ solid: "box" });
      expect(target.solids.box).toBe(source.solids.box);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.roof")).toHaveLength(1);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.baseplate")).toHaveLength(1);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.externalwall")).toHaveLength(4);
      expect(target.objects["energy.energy.roof"]?.primitives.surface).toBe("box-box-face-top");
      expect(target.objects["energy.energy.baseplate"]?.primitives.surface).toBe("box-box-face-bottom");
      const space = new ModelSpace();
      space.link("geometry", source);
      space.transfer("geometry", "energy", spec!, M);
      expect(space.get("energy")?.objects["energy.energy.windows"]).toBeTruthy();
    });
    it("loads and applies from_building transformation with shared solid geometry", () => {
      const spec = loadTransformation("aec.building.structure.from_building");
      expect(spec?.source.modelDefinition).toBe("aec.building");
      expect(spec?.target.modelDefinition).toBe("aec.building.structure");
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [0.4, 0.4, 3], height: 3 }, solidRef("column-solid")));
      source.objects["col"] = {
        id: "col" as ObjectRef,
        typology: "building.building.column" as TypologyRef,
        primitives: { solid: "column-solid" },
      };
      const target = applyTransformation(spec!, source, M);
      expect(listModelObjectsForModelDefinition(target, "aec.building.structure.classic")).toHaveLength(1);
      expect(target.objects["structure.structure.reinforcedconcretecolumn"]?.typology).toBe("structure.structure.reinforcedconcretecolumn");
      expect(target.objects["structure.structure.reinforcedconcretecolumn"]?.primitives.solid).toBe("column-solid");
      expect(target.solids["column-solid"]).toBe(source.solids["column-solid"]);
    });
    it("from_geometry fuses touching shape solids and drops internal faces", () => {
      const spec = loadTransformation("aec.building.energy.from_geometry")!;
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("lower")));
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 1], cornerB: [1, 1, 1], height: 1 }, solidRef("upper")));
      source.objects["lower"] = {
        id: "lower" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "lower" },
      };
      source.objects["upper"] = {
        id: "upper" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "upper" },
      };
      const target = applyTransformation(spec, source, M);
      expect(target.solids["from_geometry-hull"]?.shellIds).toEqual([]);
      expect(target.metadata.get("from_geometry-hull")?.fuseSourceSolidIds).toEqual(["lower", "upper"]);
      expect(target.objects["energy.energy.hull"]?.primitives.solid).toBe("from_geometry-hull");
      expect(target.objects["energy.energy.roof"]?.primitives.surface).toBe("box-upper-face-top");
      expect(target.objects["energy.energy.baseplate"]?.primitives.surface).toBe("box-lower-face-bottom");
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.externalwall")).toHaveLength(4);
      expect(listModelObjectsForModelDefinition(target, "aec.building.energy").filter((row) => row.typology === "energy.energy.hull" && row.id !== "energy.energy.hull")).toHaveLength(0);
      expect(target.objects["box-lower-face-top"]).toBeUndefined();
      expect(target.objects["box-upper-face-bottom"]).toBeUndefined();
    });
    it("from_geometry hull heated volume is boolean union not vertex AABB", async () => {
      const spec = loadTransformation("aec.building.energy.from_geometry")!;
      const source = new Model();
      applyModelDiff(source, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("west")));
      applyModelDiff(source, M.boxModelDiff({ cornerA: [3, 0, 0], cornerB: [4, 1, 0], height: 1 }, solidRef("east")));
      source.objects["west"] = {
        id: "west" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "west" },
      };
      source.objects["east"] = {
        id: "east" as ObjectRef,
        typology: "spatial.shape.primitive.box" as TypologyRef,
        primitives: { solid: "east" },
      };
      const target = applyTransformation(spec, source, M);
      const kernel = new BrepjsKernel() as unknown as SpatialKernel;
      const hull = target.objects["energy.energy.hull"]!;
      const heated = await derivePropertyValue(loadPropertyDefinition("energy.heatedvolume")!, { model: target, kernel, object: hull });
      expect(heated.heatedvolume).toBeCloseTo(2, 3);
    });
  });
}
