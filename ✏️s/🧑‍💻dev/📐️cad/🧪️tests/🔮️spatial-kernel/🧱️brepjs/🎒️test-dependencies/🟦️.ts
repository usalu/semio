/** 🎒️ The values the module hands its extracted suites, owned by the module directory so that the suites never import the module back. */

import type { face, mesh } from "@semio-tech/cad-engine-ts/brepjs";
import type { applyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import type { ModelSpace } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import type { defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import type { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import type { kernelGeometry, solidRef } from "@semio-tech/framework-3d-js";
import type { boxModelDiff } from "@semio-tech/s-spatial-kernel-ts/preview";
import type { aabbDifferencePieces, aabbIntersect, modelObjectAabb } from "@semio-tech/s-spatial-kernel-ts/preview/aabb";
import type { faceCentroid, fuseSolidsToExternalFaces } from "@semio-tech/s-spatial-kernel-ts/preview/face-geometry";
import type { geom } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import type { BrepjsKernel, deserializeWorkerValue, serializeWorkerValue } from "../📨️worker-and-kernel/🟦️.ts";

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-brepjs/🟦️.ts`. */
export type BrepjsTestDependencies = {
  readonly BrepjsKernel: typeof BrepjsKernel;
  readonly Model: typeof Model;
  readonly ModelSpace: typeof ModelSpace;
  readonly aabbDifferencePieces: typeof aabbDifferencePieces;
  readonly aabbIntersect: typeof aabbIntersect;
  readonly applyModelDiff: typeof applyModelDiff;
  readonly boxModelDiff: typeof boxModelDiff;
  readonly defaultModelDefinitionId: typeof defaultModelDefinitionId;
  readonly deserializeWorkerValue: typeof deserializeWorkerValue;
  readonly face: typeof face;
  readonly faceCentroid: typeof faceCentroid;
  readonly fuseSolidsToExternalFaces: typeof fuseSolidsToExternalFaces;
  readonly geom: typeof geom;
  readonly kernelGeometry: typeof kernelGeometry;
  readonly mesh: typeof mesh;
  readonly modelObjectAabb: typeof modelObjectAabb;
  readonly serializeWorkerValue: typeof serializeWorkerValue;
  readonly solidRef: typeof solidRef;
};
