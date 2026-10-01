import { solidRef } from "@semio-tech/s-3d-js";
/// <reference types="vitest/importMeta" />
import { AttributeTable, Model, ModelSpace, actionAvailableInModelDefinition, applyTransformation, buildModelPrimitiveDocument, computeStat, countViewObjectsForModelDefinition, defaultModelDefinitionId, derivePropertyValue, evalExpr, evalGuard, expandSelectionTargetsForAccept, formatStatOutputValue, hashModelPrimitives, hashModelVertices, hashSolidRecord, hashVertexPosition, listApplicablePropertyDefinitionsForModelDefinition, listAttributeDefinitionsForModelDefinitionEntity, listModelDefinitionAttributeDefinitions, listModelDefinitionManifests, listModelDefinitionPropertyDefinitions, listModelDefinitionStatDefinitions, listModelDefinitionTypologies, listModelObjectsForModelDefinition, listPropertyDefinitionsForModelDefinition, listSelectionOperationsForModelDefinition, listStatDefinitionsForModelDefinition, listTransformationsFromModelDefinition, listTransformationsIntoModelDefinition, listTypologiesForModelDefinition, loadAttributeDefinition, loadPropertyDefinition, loadStatDefinition, loadTransformation, loadTypology, objectMatchesTypologyPrimitives, objectsForStatCompute, parseModelJson, resolveModelDefinitionScope, resolveTypologyStyle, selectionEventMatches, validateAttributeValue } from "../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
// #region 🧪️Tests

const __geometryTestRuntime = import.meta.vitest ? await import("../../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts") : null;
const __geometryTestKernel = import.meta.vitest ? await import("../🧱️brepjs/🟦️.ts") : null;
const CAD_E2E_ROUTES_MODEL_SPACE_JSON =
  '{"schema":"spatial.modelspace","revision":1,"models":[{"id":"spatial.shape","model":{"schema":"spatial.model","revision":1,"objects":[{"id":"object-wire-orbit-a","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r10","position":[6,0,0.8]},{"kind":"vertex","id":"r11","position":[7.4,0.6,0.8]},{"kind":"vertex","id":"r12","position":[8.8,0.2,0.8]},{"kind":"vertex","id":"r13","position":[9.9,1.1,0.8]},{"kind":"vertex","id":"r14","position":[10.2,2.6,0.8]},{"kind":"vertex","id":"r15","position":[9.4,3.9,0.8]},{"kind":"vertex","id":"r16","position":[7.8,4.4,0.8]},{"kind":"vertex","id":"r17","position":[6.2,4.1,0.8]},{"kind":"curve","id":"re10","vertexIds":["r10","r11"]},{"kind":"curve","id":"re11","vertexIds":["r11","r12"]},{"kind":"curve","id":"re12","vertexIds":["r12","r13"]},{"kind":"curve","id":"re13","vertexIds":["r13","r14"]},{"kind":"curve","id":"re14","vertexIds":["r14","r15"]},{"kind":"curve","id":"re15","vertexIds":["r15","r16"]},{"kind":"curve","id":"re16","vertexIds":["r16","r17"]},{"kind":"curve","id":"re17","vertexIds":["r17","r10"]},{"kind":"curve","slot":"wire","id":"orbit-a","edgeIds":["re10","re11","re12","re13","re14","re15","re16","re17"]}]},{"id":"object-wire-spine-b","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r18","position":[2,6,1.6]},{"kind":"vertex","id":"r19","position":[3.5,6.8,1.6]},{"kind":"vertex","id":"r20","position":[5.2,6.5,1.6]},{"kind":"vertex","id":"r21","position":[6.8,7.2,1.6]},{"kind":"vertex","id":"r22","position":[7.5,8.4,1.6]},{"kind":"vertex","id":"r23","position":[6.1,9.1,1.6]},{"kind":"curve","id":"re18","vertexIds":["r18","r19"]},{"kind":"curve","id":"re19","vertexIds":["r19","r20"]},{"kind":"curve","id":"re20","vertexIds":["r20","r21"]},{"kind":"curve","id":"re21","vertexIds":["r21","r22"]},{"kind":"curve","id":"re22","vertexIds":["r22","r23"]},{"kind":"curve","slot":"wire","id":"spine-b","edgeIds":["re18","re19","re20","re21","re22"]}]},{"id":"object-wire-stub-wire","typology":"spatial.shape.kernel.wire","primitives":[{"kind":"vertex","id":"r0","position":[0,0,0]},{"kind":"vertex","id":"r1","position":[1.2,0.4,0]},{"kind":"vertex","id":"r2","position":[2.6,0.1,0]},{"kind":"vertex","id":"r3","position":[3.8,0.9,0]},{"kind":"vertex","id":"r4","position":[4.5,2.1,0]},{"kind":"vertex","id":"r5","position":[4.2,3.5,0]},{"kind":"vertex","id":"r6","position":[3.1,4.2,0]},{"kind":"vertex","id":"r7","position":[1.5,4.5,0]},{"kind":"vertex","id":"r8","position":[0.2,3.8,0]},{"kind":"vertex","id":"r9","position":[-0.4,2.2,0]},{"kind":"curve","id":"re0","vertexIds":["r0","r1"]},{"kind":"curve","id":"re1","vertexIds":["r1","r2"]},{"kind":"curve","id":"re2","vertexIds":["r2","r3"]},{"kind":"curve","id":"re3","vertexIds":["r3","r4"]},{"kind":"curve","id":"re4","vertexIds":["r4","r5"]},{"kind":"curve","id":"re5","vertexIds":["r5","r6"]},{"kind":"curve","id":"re6","vertexIds":["r6","r7"]},{"kind":"curve","id":"re7","vertexIds":["r7","r8"]},{"kind":"curve","id":"re8","vertexIds":["r8","r9"]},{"kind":"curve","id":"re9","vertexIds":["r9","r0"]},{"kind":"curve","slot":"wire","id":"stub-wire","edgeIds":["re0","re1","re2","re3","re4","re5","re6","re7","re8","re9"]}]}]}}]}';

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts`. */
export type GeometryTestDependencies = {
  readonly AttributeTable: typeof AttributeTable;
  readonly CAD_E2E_ROUTES_MODEL_SPACE_JSON: typeof CAD_E2E_ROUTES_MODEL_SPACE_JSON;
  readonly Model: typeof Model;
  readonly ModelSpace: typeof ModelSpace;
  readonly __geometryTestKernel: typeof __geometryTestKernel;
  readonly __geometryTestRuntime: typeof __geometryTestRuntime;
  readonly actionAvailableInModelDefinition: typeof actionAvailableInModelDefinition;
  readonly applyTransformation: typeof applyTransformation;
  readonly buildModelPrimitiveDocument: typeof buildModelPrimitiveDocument;
  readonly computeStat: typeof computeStat;
  readonly countViewObjectsForModelDefinition: typeof countViewObjectsForModelDefinition;
  readonly defaultModelDefinitionId: typeof defaultModelDefinitionId;
  readonly derivePropertyValue: typeof derivePropertyValue;
  readonly evalExpr: typeof evalExpr;
  readonly evalGuard: typeof evalGuard;
  readonly expandSelectionTargetsForAccept: typeof expandSelectionTargetsForAccept;
  readonly formatStatOutputValue: typeof formatStatOutputValue;
  readonly hashModelPrimitives: typeof hashModelPrimitives;
  readonly hashModelVertices: typeof hashModelVertices;
  readonly hashSolidRecord: typeof hashSolidRecord;
  readonly hashVertexPosition: typeof hashVertexPosition;
  readonly listApplicablePropertyDefinitionsForModelDefinition: typeof listApplicablePropertyDefinitionsForModelDefinition;
  readonly listAttributeDefinitionsForModelDefinitionEntity: typeof listAttributeDefinitionsForModelDefinitionEntity;
  readonly listModelDefinitionAttributeDefinitions: typeof listModelDefinitionAttributeDefinitions;
  readonly listModelDefinitionManifests: typeof listModelDefinitionManifests;
  readonly listModelDefinitionPropertyDefinitions: typeof listModelDefinitionPropertyDefinitions;
  readonly listModelDefinitionStatDefinitions: typeof listModelDefinitionStatDefinitions;
  readonly listModelDefinitionTypologies: typeof listModelDefinitionTypologies;
  readonly listModelObjectsForModelDefinition: typeof listModelObjectsForModelDefinition;
  readonly listPropertyDefinitionsForModelDefinition: typeof listPropertyDefinitionsForModelDefinition;
  readonly listSelectionOperationsForModelDefinition: typeof listSelectionOperationsForModelDefinition;
  readonly listStatDefinitionsForModelDefinition: typeof listStatDefinitionsForModelDefinition;
  readonly listTransformationsFromModelDefinition: typeof listTransformationsFromModelDefinition;
  readonly listTransformationsIntoModelDefinition: typeof listTransformationsIntoModelDefinition;
  readonly listTypologiesForModelDefinition: typeof listTypologiesForModelDefinition;
  readonly loadAttributeDefinition: typeof loadAttributeDefinition;
  readonly loadPropertyDefinition: typeof loadPropertyDefinition;
  readonly loadStatDefinition: typeof loadStatDefinition;
  readonly loadTransformation: typeof loadTransformation;
  readonly loadTypology: typeof loadTypology;
  readonly objectMatchesTypologyPrimitives: typeof objectMatchesTypologyPrimitives;
  readonly objectsForStatCompute: typeof objectsForStatCompute;
  readonly parseModelJson: typeof parseModelJson;
  readonly resolveModelDefinitionScope: typeof resolveModelDefinitionScope;
  readonly resolveTypologyStyle: typeof resolveTypologyStyle;
  readonly selectionEventMatches: typeof selectionEventMatches;
  readonly solidRef: typeof solidRef;
  readonly validateAttributeValue: typeof validateAttributeValue;
};

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts");
  await registerTests1(import.meta.vitest, { AttributeTable, CAD_E2E_ROUTES_MODEL_SPACE_JSON, Model, ModelSpace, __geometryTestKernel, __geometryTestRuntime, actionAvailableInModelDefinition, applyTransformation, buildModelPrimitiveDocument, computeStat, countViewObjectsForModelDefinition, defaultModelDefinitionId, derivePropertyValue, evalExpr, evalGuard, expandSelectionTargetsForAccept, formatStatOutputValue, hashModelPrimitives, hashModelVertices, hashSolidRecord, hashVertexPosition, listApplicablePropertyDefinitionsForModelDefinition, listAttributeDefinitionsForModelDefinitionEntity, listModelDefinitionAttributeDefinitions, listModelDefinitionManifests, listModelDefinitionPropertyDefinitions, listModelDefinitionStatDefinitions, listModelDefinitionTypologies, listModelObjectsForModelDefinition, listPropertyDefinitionsForModelDefinition, listSelectionOperationsForModelDefinition, listStatDefinitionsForModelDefinition, listTransformationsFromModelDefinition, listTransformationsIntoModelDefinition, listTypologiesForModelDefinition, loadAttributeDefinition, loadPropertyDefinition, loadStatDefinition, loadTransformation, loadTypology, objectMatchesTypologyPrimitives, objectsForStatCompute, parseModelJson, resolveModelDefinitionScope, resolveTypologyStyle, selectionEventMatches, solidRef, validateAttributeValue }, { url: import.meta.url });
}
// #endregion 🧪️Tests
