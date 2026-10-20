/** 🧰️ The shared brepjs test kit: the destructured vitest, the brepjs kernel dependencies and the bootstrapped cad modules. */
import type { BrepjsTestDependencies } from "../../../🎒️test-dependencies/🟦️.ts";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { BrepjsKernel, deserializeWorkerValue, serializeWorkerValue } from "../../../📨️worker-and-kernel/🟦️.ts";
import { ModelSpace } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import { aabbDifferencePieces, aabbIntersect, modelObjectAabb } from "@semio-tech/s-spatial-kernel-ts/preview/aabb";
import { applyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { boxModelDiff } from "@semio-tech/s-spatial-kernel-ts/preview";
import { defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { face, mesh } from "@semio-tech/cad-engine-ts/brepjs";
import { faceCentroid, fuseSolidsToExternalFaces } from "@semio-tech/s-spatial-kernel-ts/preview/face-geometry";
import { geom } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { kernelGeometry, solidRef } from "@semio-tech/framework-3d-js";

export type TestSource = { readonly url: string };

/** 🏭 Creates the shared brepjs test kit of one registration from the injected dependencies. */
export async function createBrepjsTestKit(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: BrepjsTestDependencies, source: TestSource) {
  const { BrepjsKernel, Model, ModelSpace, aabbDifferencePieces, aabbIntersect, applyModelDiff, boxModelDiff, defaultModelDefinitionId, deserializeWorkerValue, face, faceCentroid, fuseSolidsToExternalFaces, geom, kernelGeometry, mesh, modelObjectAabb, serializeWorkerValue, solidRef } = dependencies;
  const { beforeEach, describe, expect, it } = vitest;
  const { bootstrapCadModules } = await import("@semio-tech/cad-cad-ts/standards/1/subsets/any/editor/engine/runtime");
  const { AEC_BUILDING_MODEL_DEFINITION_ID } = await import("@semio-tech/cad-js-module-aec-building");
  const { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID } = await import("@semio-tech/cad-js-module-aec-building-energy");
  const { AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID } = await import("@semio-tech/cad-js-module-aec-building-structure");
  bootstrapCadModules();
  return { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID, AEC_BUILDING_MODEL_DEFINITION_ID, AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID, BrepjsKernel, Model, ModelSpace, aabbDifferencePieces, aabbIntersect, applyModelDiff, beforeEach, boxModelDiff, defaultModelDefinitionId, describe, deserializeWorkerValue, expect, faceCentroid, fuseSolidsToExternalFaces, geom, it, kernelGeometry, modelObjectAabb, serializeWorkerValue, solidRef, source };
}
export type BrepjsTestKit = Awaited<ReturnType<typeof createBrepjsTestKit>>;
