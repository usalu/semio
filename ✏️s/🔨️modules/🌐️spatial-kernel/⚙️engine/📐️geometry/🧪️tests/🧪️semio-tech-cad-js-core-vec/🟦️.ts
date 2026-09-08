type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { AttributeTable, CAD_E2E_ROUTES_MODEL_SPACE_JSON, CAD_GUMBALL_HIDDEN, Model, ModelSpace, SpatialKernel, SpatialPreviewKernel, __geometryTestKernel, __geometryTestRuntime, actionAvailableInModelDefinition, applyModelDiff, applyTransformation, buildModelPrimitiveDocument, cadGumballConfigVisible, collectGeometrySelectionTargets, computeStat, countViewObjectsForModelDefinition, defaultModelDefinitionId, derivePropertyValue, evalExpr, evalGuard, expandSelectionTargetsForAccept, formatStatOutputValue, hashModelPrimitives, hashModelVertices, hashSolidRecord, hashVertexPosition, listApplicablePropertyDefinitionsForModelDefinition, listAttributeDefinitionsForModelDefinitionEntity, listModelDefinitionAttributeDefinitions, listModelDefinitionManifests, listModelDefinitionPropertyDefinitions, listModelDefinitionStatDefinitions, listModelDefinitionTypologies, listModelObjectsForModelDefinition, listPropertyDefinitionsForModelDefinition, listSelectionOperationsForModelDefinition, listStatDefinitionsForModelDefinition, listTransformationsFromModelDefinition, listTransformationsIntoModelDefinition, listTypologiesForModelDefinition, loadAttributeDefinition, loadPropertyDefinition, loadStatDefinition, loadTransformation, loadTypology, modelDefinitionActionRegistry, objectMatchesTypologyPrimitives, objectsForStatCompute, parseModelJson, resolveModelDefinitionScope, resolveTypologyStyle, runRegisteredAction, selectionEventMatches, solidRef, validateAttributeValue } = dependencies;
  type EdgeRecord = any;
  type EdgeRef = any;
  type Expr = any;
  type ModelSpaceJson = any;
  type ObjectRef = any;
  type SelectionEvent = any;
  type SelectionSpec = any;
  type TypologyRef = any;
  type Vec3 = any;
  type VertexRef = any;

  __geometryTestRuntime!.bootstrapCadModules();
  const { BrepjsKernel, preciseSpatialKernelMath } = __geometryTestKernel!;
  const geometryRoutesFixtureJson = JSON.parse(CAD_E2E_ROUTES_MODEL_SPACE_JSON) as ModelSpaceJson;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = vitest;

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
    it("loads geometry and AEC typology assets", () => {
      const typologies = listModelDefinitionTypologies();
      expect(typologies.length).toBeGreaterThanOrEqual(27);
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
      const { resolve } = await import("node:path");
      const fixturePath = resolve(source.directory, "../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🎮️play/🔣️.json");
      const fixtureJson = JSON.parse(await readFile(fixturePath, "utf8")) as ModelSpaceJson;
      const space = ModelSpace.fromJSON(fixtureJson);
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
