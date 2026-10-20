import type { BrepjsTestDependencies } from "../../🎒️test-dependencies/🟦️.ts";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import * as vitestRuntime from "vitest";
import { BrepjsKernel, deserializeWorkerValue, serializeWorkerValue } from "../../📨️worker-and-kernel/🟦️.ts";
import { ModelSpace } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import { aabbDifferencePieces, aabbIntersect, modelObjectAabb } from "@semio-tech/s-spatial-kernel-ts/preview/aabb";
import { applyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { boxModelDiff } from "@semio-tech/s-spatial-kernel-ts/preview";
import { defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { face, mesh } from "@semio-tech/cad-engine-ts/brepjs";
import { faceCentroid, fuseSolidsToExternalFaces } from "@semio-tech/s-spatial-kernel-ts/preview/face-geometry";
import { geom } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { kernelGeometry, solidRef } from "@semio-tech/framework-3d-js";
import { createBrepjsTestKit } from "./🧰️brepjs-test-kit/🟦️.ts";
import type { TestSource } from "./🧰️brepjs-test-kit/🟦️.ts";
import { registerBrepjsSuite } from "./🧱️brepjs-suite/🟦️.ts";

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: BrepjsTestDependencies, source: TestSource): Promise<void> {
  const kit = await createBrepjsTestKit(vitest, dependencies, source);
  registerBrepjsSuite(kit);
}

const dependencies: BrepjsTestDependencies = { BrepjsKernel, Model, ModelSpace, aabbDifferencePieces, aabbIntersect, applyModelDiff, boxModelDiff, defaultModelDefinitionId, deserializeWorkerValue, face, faceCentroid, fuseSolidsToExternalFaces, geom, kernelGeometry, mesh, modelObjectAabb, serializeWorkerValue, solidRef };
await registerTests1(vitestRuntime, dependencies, { url: new URL("../../🟦️.ts", import.meta.url).href });
