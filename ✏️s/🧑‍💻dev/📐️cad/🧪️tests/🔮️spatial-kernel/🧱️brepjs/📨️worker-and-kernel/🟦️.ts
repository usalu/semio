/** 📨️ The worker protocol, the worker client and the brepjs spatial kernel that fronts them.
 *
 * @see ../🟦️.ts — the brepjs kernel module, kept ONLY as a vitest differential oracle
 */
import { type SpatialKernel, type ModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { type ModelJson } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { ModelSpace } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { type MeshTransfer, type Vec3 } from "@semio-tech/framework-3d-js";
import { PreciseSpatialKernelMath } from "@semio-tech/s-spatial-kernel-ts/preview";
import { type EdgeRef, type FaceRef, type SolidRef, type VertexRef } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { isBrepjsTestRun, openCascadeWasmNeedsNodeResolve } from "../🧩️opencascade-and-scratch/🟦️.ts";
import { BrepjsWasmEngine } from "../⚙️wasm-engine/🟦️.ts";

// #region 📨️WorkerProtocol
export type BrepjsWorkerRequest = { readonly type: "init" } | { readonly type: "rpc"; readonly id: string; readonly method: string; readonly args: readonly unknown[] };

export type BrepjsWorkerResponse =
  | { readonly type: "init-done" }
  | { readonly type: "init-error"; readonly error: string }
  | { readonly type: "rpc-result"; readonly id: string; readonly result: unknown }
  | { readonly type: "rpc-error"; readonly id: string; readonly error: string };

/** 📨️ Walks RPC payloads so nested `Model` / `ModelSpace` survive worker `postMessage`. */
export function serializeWorkerValue(value: unknown): unknown {
  if (value instanceof Model) return { __modelJson: value.toJSON() };
  if (value instanceof ModelSpace) return { __modelSpaceJson: value.toJSON() };
  if (Array.isArray(value)) return value.map(serializeWorkerValue);
  if (!value || typeof value !== "object") return value;
  const row = value as Record<string, unknown>;
  const out: Record<string, unknown> = {};
  for (const [key, entry] of Object.entries(row)) out[key] = serializeWorkerValue(entry);
  return out;
}

function serializeWorkerArg(arg: unknown): unknown {
  return serializeWorkerValue(arg);
}

/** 📨️ Restores `Model` / `ModelSpace` instances from worker RPC payloads. */
export function deserializeWorkerValue(value: unknown): unknown {
  if (value && typeof value === "object" && "__modelJson" in value) {
    return Model.fromJSON((value as { readonly __modelJson: ModelJson }).__modelJson);
  }
  if (value && typeof value === "object" && "__modelSpaceJson" in value) {
    return ModelSpace.fromJSON((value as { readonly __modelSpaceJson: Parameters<typeof ModelSpace.fromJSON>[0] }).__modelSpaceJson);
  }
  if (Array.isArray(value)) return value.map(deserializeWorkerValue);
  if (!value || typeof value !== "object") return value;
  const row = value as Record<string, unknown>;
  const out: Record<string, unknown> = {};
  for (const [key, entry] of Object.entries(row)) out[key] = deserializeWorkerValue(entry);
  return out;
}

export function deserializeWorkerArg(arg: unknown): unknown {
  return deserializeWorkerValue(arg);
}

// #endregion 📨️WorkerProtocol
// #region 🎬️BrepjsWorkerClient
/** 🎬️ Routes brepjs RPC to a dedicated worker or local `BrepjsWasmEngine` (vitest / no Worker). */
class BrepjsWorkerClient {
  private localEngine: BrepjsWasmEngine | null = null;
  private worker: Worker | null = null;
  private initPromise: Promise<void> | null = null;
  private readonly pending = new Map<string, { readonly resolve: (v: unknown) => void; readonly reject: (e: Error) => void }>();

  private localWasmEngine(): BrepjsWasmEngine {
    if (!this.localEngine) this.localEngine = new BrepjsWasmEngine();
    return this.localEngine;
  }

  constructor() {
    const workerDisabled = isBrepjsTestRun || openCascadeWasmNeedsNodeResolve;
    if (!workerDisabled && typeof Worker !== "undefined") {
      try {
        this.worker = new Worker(new URL("../🟦️.ts", import.meta.url), { type: "module" });
        this.worker.onmessage = (event: MessageEvent<BrepjsWorkerResponse>) => this.onWorkerMessage(event.data);
      } catch {
        this.worker = null;
      }
    }
  }

  private onWorkerMessage(msg: BrepjsWorkerResponse): void {
    if (msg.type === "init-done") {
      const init = this.pending.get("init");
      init?.resolve(undefined);
      this.pending.delete("init");
      return;
    }
    if (msg.type === "init-error") {
      const init = this.pending.get("init");
      init?.reject(new Error(msg.error));
      this.pending.delete("init");
      return;
    }
    if (msg.type === "rpc-result") {
      const p = this.pending.get(msg.id);
      p?.resolve(msg.result);
      this.pending.delete(msg.id);
      return;
    }
    if (msg.type === "rpc-error") {
      const p = this.pending.get(msg.id);
      p?.reject(new Error(msg.error));
      this.pending.delete(msg.id);
    }
  }

  async ensureReady(): Promise<void> {
    if (!this.initPromise) this.initPromise = this.boot();
    await this.initPromise;
  }

  private async boot(): Promise<void> {
    if (!this.worker) {
      await this.localWasmEngine().ensureInit();
      return;
    }
    await new Promise<void>((resolve, reject) => {
      this.pending.set("init", { resolve: () => resolve(), reject });
      this.worker!.postMessage({ type: "init" } satisfies BrepjsWorkerRequest);
    });
  }

  async rpc<T>(method: string, args: readonly unknown[]): Promise<T> {
    await this.ensureReady();
    const serialized = args.map(serializeWorkerArg);
    if (!this.worker) {
      const hydrated = serialized.map(deserializeWorkerArg);
      const engine = this.localWasmEngine();
      const fn = (engine as unknown as Record<string, unknown>)[method];
      if (typeof fn !== "function") throw new Error(`brepjs rpc: unknown method ${method}`);
      return (await (fn as (...a: unknown[]) => unknown).apply(engine, hydrated)) as T;
    }
    return new Promise<T>((resolve, reject) => {
      const id = crypto.randomUUID();
      this.pending.set(id, {
        resolve: (v) => resolve(v as T),
        reject,
      });
      this.worker!.postMessage({ type: "rpc", id, method, args: serialized } satisfies BrepjsWorkerRequest);
    });
  }
}

// #endregion 🎬️BrepjsWorkerClient
// #region 🔌️BrepjsKernel
/** 🔌️ `SpatialKernel` facade: preview math on main thread, WASM in worker via `BrepjsWorkerClient`. */
export class BrepjsKernel extends PreciseSpatialKernelMath implements SpatialKernel {
  readonly id: string = "brepjs-opencascade";
  private readonly wasm = new BrepjsWorkerClient();

  readonly operations: readonly string[] = ["solid.createBox", "wire.extrudeToSolid", "face.offset", "entity.tessellate", "measure.distance", "measure.area", "measure.volume"];

  async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef> {
    return this.wasm.rpc("createBoxFromCorners", [input]);
  }

  async volume(cell: SolidRef): Promise<number> {
    return this.wasm.rpc("volume", [cell]);
  }

  async tessellate(solid: SolidRef, tolerance: number, model?: Model): Promise<MeshTransfer> {
    return this.wasm.rpc("tessellate", [solid, tolerance, model]);
  }

  /** 🧪️ Clears worker solids cache between vitest cases. */
  async resetDerivedPipelineForTest(): Promise<void> {
    return this.wasm.rpc("resetDerivedPipeline", []);
  }

  async executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: ModelDiff }> {
    return this.wasm.rpc("executeCommandDiff", [commandId, params]);
  }

  async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    return this.wasm.rpc("createBoxFromCornersDiff", [input]);
  }

  async extrudeWireDiff(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    return this.wasm.rpc("extrudeWireDiff", [input]);
  }

  async offsetFacesDiff(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<{ readonly diff: ModelDiff }> {
    return this.wasm.rpc("offsetFacesDiff", [input]);
  }

  async vertexDistance(a: VertexRef, b: VertexRef, model: Model): Promise<number> {
    return this.wasm.rpc("vertexDistance", [a, b, model]);
  }

  async edgeLength(e: EdgeRef, model: Model): Promise<number> {
    return this.wasm.rpc("edgeLength", [e, model]);
  }

  async faceArea(f: FaceRef, model: Model): Promise<number> {
    return this.wasm.rpc("faceArea", [f, model]);
  }

  async solidVolume(c: SolidRef): Promise<number> {
    return this.wasm.rpc("solidVolume", [c]);
  }

  async adjacentSolids(cell: SolidRef, model: Model): Promise<readonly SolidRef[]> {
    return this.wasm.rpc("adjacentSolids", [cell, model]);
  }

  async sharedFacesBetween(a: SolidRef, b: SolidRef, model: Model): Promise<readonly FaceRef[]> {
    return this.wasm.rpc("sharedFacesBetween", [a, b, model]);
  }

  async extrudeWire(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<SolidRef> {
    return this.wasm.rpc("extrudeWire", [input]);
  }

  async offsetFaces(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<void> {
    await this.wasm.rpc("offsetFaces", [input]);
  }

  disposeSolid(cell: SolidRef): void {
    void this.wasm.rpc("disposeSolid", [cell]);
  }

  async syncSolidsFromModel(model: Model): Promise<void> {
    await this.wasm.rpc("syncSolidsFromModel", [model]);
  }

  /** 🪜️ Exports a linked `ModelSpace` to AP242 STEP. */
  async exportModelSpaceToStep(space: ModelSpace, modelSpaceId = "default"): Promise<string> {
    return this.wasm.rpc("exportModelSpaceToStep", [space, modelSpaceId]);
  }

  /** 🪜️ Exports one `Model` to AP242 STEP. */
  async exportModelToStep(model: Model, modelId = "model"): Promise<string> {
    return this.wasm.rpc("exportModelToStep", [model, modelId]);
  }

  /** 🪜️ Imports AP242 STEP into a `ModelSpace`. */
  async importStepToModelSpace(stepText: string): Promise<ModelSpace> {
    return this.wasm.rpc("importStepToModelSpace", [stepText]);
  }

  /** 🪜️ Imports raw AP242 BREP STEP (no spatial UDA) into a `ModelSpace`. */
  async importStepBrepToModelSpace(stepText: string, options?: { readonly prefix?: string; readonly lengthScale?: number }): Promise<ModelSpace> {
    return this.wasm.rpc("importStepBrepToModelSpace", [stepText, options]);
  }

  /** 🏗️ Imports AP242 BREP STEP with presentation layers into a building `ModelSpace`. */
  async importStepBimToModelSpace(stepText: string, options?: { readonly prefix?: string; readonly lengthScale?: number; readonly modelDefinitionId?: string }): Promise<ModelSpace> {
    return this.wasm.rpc("importStepBimToModelSpace", [stepText, options]);
  }
}
// #endregion 🔌️BrepjsKernel
