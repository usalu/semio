// #region 🧲️Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** 🧭️ `@semio-tech/cad-js/brepjs` — `SpatialKernel` backed by brepjs + OpenCascade WASM, kept ONLY as a vitest differential oracle (see `import.meta.vitest`-guarded exports below and `🎫️tickets/…/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME/📓️w4a-spatial-kernel-first-party.md`). The production runtime kernel is `🧠️semio/🟦️.ts`; pure preview math moved to the `🧮️preview` modules. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { face, mesh } from "@semio-tech/cad-engine-ts/brepjs";
import { applyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { ModelSpace } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import { defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { kernelGeometry, type MeshTransfer, solidRef } from "@semio-tech/framework-3d-js";
import { boxModelDiff } from "@semio-tech/s-spatial-kernel-ts/preview";
import { aabbDifferencePieces, aabbIntersect, modelObjectAabb } from "@semio-tech/s-spatial-kernel-ts/preview/aabb";
import { faceCentroid, fuseSolidsToExternalFaces } from "@semio-tech/s-spatial-kernel-ts/preview/face-geometry";
import { geom } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { meshTransferTransferables } from "./🧱️entity-maps/🟦️.ts";
import { BrepjsWasmEngine } from "./⚙️wasm-engine/🟦️.ts";
import { BrepjsKernel, type BrepjsWorkerRequest, type BrepjsWorkerResponse, deserializeWorkerArg, deserializeWorkerValue, serializeWorkerValue } from "./📨️worker-and-kernel/🟦️.ts";
export { kernelGeometry };

// #endregion 🔌️Adapters
// #region 🌐️BrepjsWorkerEntry
function isBrepjsDedicatedWorker(): boolean {
  return typeof self !== "undefined" && self.constructor?.name === "DedicatedWorkerGlobalScope";
}

if (isBrepjsDedicatedWorker()) {
  let engine: BrepjsWasmEngine | null = null;
  self.addEventListener("message", (event: MessageEvent<BrepjsWorkerRequest>) => {
    const msg = event.data;
    if (msg.type === "init") {
      void (async () => {
        try {
          engine = new BrepjsWasmEngine();
          await engine.ensureInit();
          self.postMessage({ type: "init-done" } satisfies BrepjsWorkerResponse);
        } catch (e) {
          self.postMessage({
            type: "init-error",
            error: e instanceof Error ? e.message : String(e),
          } satisfies BrepjsWorkerResponse);
        }
      })();
      return;
    }
    if (msg.type === "rpc" && engine) {
      void (async () => {
        try {
          const args = msg.args.map(deserializeWorkerArg);
          const fn = (engine as unknown as Record<string, unknown>)[msg.method];
          if (typeof fn !== "function") throw new Error(`brepjs worker: unknown method ${msg.method}`);
          const result = await (fn as (...a: unknown[]) => unknown).apply(engine, args);
          const transfers = result && typeof result === "object" && "position" in result ? meshTransferTransferables(result as MeshTransfer) : undefined;
          if (transfers?.length) {
            self.postMessage({ type: "rpc-result", id: msg.id, result } satisfies BrepjsWorkerResponse, {
              transfer: transfers,
            });
          } else {
            self.postMessage({ type: "rpc-result", id: msg.id, result } satisfies BrepjsWorkerResponse);
          }
        } catch (e) {
          self.postMessage({
            type: "rpc-error",
            id: msg.id,
            error: e instanceof Error ? e.message : String(e),
          } satisfies BrepjsWorkerResponse);
        }
      })();
    }
  });
}

// #endregion 🌐️BrepjsWorkerEntry
// #region 🧪️Tests
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

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️semio-tech-cad-js-brepjs/🟦️.ts");
  await registerTests1(import.meta.vitest, { BrepjsKernel, Model, ModelSpace, aabbDifferencePieces, aabbIntersect, applyModelDiff, boxModelDiff, defaultModelDefinitionId, deserializeWorkerValue, face, faceCentroid, fuseSolidsToExternalFaces, geom, kernelGeometry, mesh, modelObjectAabb, serializeWorkerValue, solidRef }, { url: import.meta.url });
}
// #endregion 🧪️Tests
