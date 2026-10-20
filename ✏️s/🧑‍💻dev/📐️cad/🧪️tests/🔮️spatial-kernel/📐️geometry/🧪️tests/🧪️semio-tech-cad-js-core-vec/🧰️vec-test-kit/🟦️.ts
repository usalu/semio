/** 🧰️ The shared cad-js core test kit: the destructured vitest, the core dependencies, the Brepjs kernel and the precise spatial math. */
import type { GeometryTestDependencies } from "../../../🎒️test-dependencies/🟦️.ts";
import { preciseSpatialKernelMath } from "@semio-tech/s-spatial-kernel-ts/preview";

export type TestSource = { readonly url: string };
export const MODEL_DEFINITION_ASSETS = "../../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/";
/** 🗂️ The typology assets on disk: every `<model definition>/🗂️typologies/<typology>/` must hold exactly the fixed
 * `🔣️typology.json` the runtime glob and the artifact projection golden declare — `misnamed` lists every other file, `ids`
 * the sorted ids of the fixed-name assets. */
export async function typologyAssetCensus(source: TestSource): Promise<{ readonly misnamed: readonly string[]; readonly ids: readonly string[] }> {
  const { readdir, readFile } = await import("node:fs/promises");
  const root = new URL(MODEL_DEFINITION_ASSETS, source.url);
  const misnamed: string[] = [];
  const ids: string[] = [];
  for (const modelDefinition of await readdir(root, { withFileTypes: true })) {
    if (!modelDefinition.isDirectory()) continue;
    const typologies = new URL(`${encodeURIComponent(modelDefinition.name)}/🗂️typologies/`, root);
    const folders = await readdir(typologies, { withFileTypes: true }).catch(() => []);
    for (const folder of folders) {
      const folderUrl = new URL(`${encodeURIComponent(folder.name)}/`, typologies);
      for (const file of await readdir(folderUrl)) {
        if (file !== "🔣️typology.json") misnamed.push(`${modelDefinition.name}/🗂️typologies/${folder.name}/${file}`);
        else ids.push((JSON.parse(await readFile(new URL(encodeURIComponent(file), folderUrl), "utf8")) as { readonly id: string }).id);
      }
    }
  }
  return { misnamed, ids: ids.sort() };
}

/** 🏭 Creates the shared vec test kit of one registration from the injected dependencies. */
export function createVecTestKit(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: GeometryTestDependencies, source: TestSource) {
  const { AttributeTable, CAD_E2E_ROUTES_MODEL_SPACE_JSON, Model, ModelSpace, __geometryTestKernel, __geometryTestRuntime, actionAvailableInModelDefinition, applyTransformation, buildModelPrimitiveDocument, computeStat, countViewObjectsForModelDefinition, defaultModelDefinitionId, derivePropertyValue, evalExpr, evalGuard, expandSelectionTargetsForAccept, formatStatOutputValue, hashModelPrimitives, hashModelVertices, hashSolidRecord, hashVertexPosition, listApplicablePropertyDefinitionsForModelDefinition, listAttributeDefinitionsForModelDefinitionEntity, listModelDefinitionAttributeDefinitions, listModelDefinitionManifests, listModelDefinitionPropertyDefinitions, listModelDefinitionStatDefinitions, listModelDefinitionTypologies, listModelObjectsForModelDefinition, listPropertyDefinitionsForModelDefinition, listSelectionOperationsForModelDefinition, listStatDefinitionsForModelDefinition, listTransformationsFromModelDefinition, listTransformationsIntoModelDefinition, listTypologiesForModelDefinition, loadAttributeDefinition, loadPropertyDefinition, loadStatDefinition, loadTransformation, loadTypology, objectMatchesTypologyPrimitives, objectsForStatCompute, parseModelJson, resolveModelDefinitionScope, resolveTypologyStyle, selectionEventMatches, solidRef, validateAttributeValue } = dependencies;
  const { describe, expect, it } = vitest;
  __geometryTestRuntime!.bootstrapCadModules();
  const { BrepjsKernel } = __geometryTestKernel!;
  const M = preciseSpatialKernelMath;
  return { AttributeTable, BrepjsKernel, CAD_E2E_ROUTES_MODEL_SPACE_JSON, M, Model, ModelSpace, actionAvailableInModelDefinition, applyTransformation, buildModelPrimitiveDocument, computeStat, countViewObjectsForModelDefinition, defaultModelDefinitionId, derivePropertyValue, describe, evalExpr, evalGuard, expandSelectionTargetsForAccept, expect, formatStatOutputValue, hashModelPrimitives, hashModelVertices, hashSolidRecord, hashVertexPosition, it, listApplicablePropertyDefinitionsForModelDefinition, listAttributeDefinitionsForModelDefinitionEntity, listModelDefinitionAttributeDefinitions, listModelDefinitionManifests, listModelDefinitionPropertyDefinitions, listModelDefinitionStatDefinitions, listModelDefinitionTypologies, listModelObjectsForModelDefinition, listPropertyDefinitionsForModelDefinition, listSelectionOperationsForModelDefinition, listStatDefinitionsForModelDefinition, listTransformationsFromModelDefinition, listTransformationsIntoModelDefinition, listTypologiesForModelDefinition, loadAttributeDefinition, loadPropertyDefinition, loadStatDefinition, loadTransformation, loadTypology, objectMatchesTypologyPrimitives, objectsForStatCompute, parseModelJson, resolveModelDefinitionScope, resolveTypologyStyle, selectionEventMatches, solidRef, source, validateAttributeValue };
}
export type VecTestKit = ReturnType<typeof createVecTestKit>;
