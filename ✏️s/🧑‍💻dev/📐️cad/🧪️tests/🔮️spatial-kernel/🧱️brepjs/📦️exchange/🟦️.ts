/** 📦️ The exchange functions of the brepjs kernel: model space to STEP, OBJ and GLB and STEP back into model space.
 *
 * @see ../🟦️.ts — the brepjs kernel module, kept ONLY as a vitest differential oracle
 */
import { ModelSpace } from "@semio-tech/s-spatial-kernel-ts/geometry/model-space";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { BrepjsKernel } from "../📨️worker-and-kernel/🟦️.ts";

// #region 🔌️BrepjsKernel
/** 🪜️ Exports `space` via a fresh `BrepjsKernel` (convenience). */
export async function exportModelSpaceToStep(space: ModelSpace, modelSpaceId = "default"): Promise<string> {
  const kernel = new BrepjsKernel();
  return kernel.exportModelSpaceToStep(space, modelSpaceId);
}

/** 💾️ Exports `space` solids as merged OBJ via tessellation. */
export async function exportModelSpaceToObj(space: ModelSpace, deflection = 0.1): Promise<string> {
  const { meshTransferToObj, mergeMeshTransfers } = await import("@semio-tech/framework-3d-js");
  const kernel = new BrepjsKernel();
  const meshes = [];
  for (const model of Object.values(space.models)) {
    for (const solid of Object.values(model.solids)) {
      const mesh = await kernel.tessellate(solid.id, deflection, model);
      if (mesh.position.length > 0 && mesh.index.length > 0) meshes.push(mesh);
    }
  }
  if (meshes.length === 0) return "# empty model space\n";
  return meshTransferToObj(mergeMeshTransfers(meshes));
}

/** 💾️ Exports `space` solids as merged GLB via tessellation. */
export async function exportModelSpaceToGlb(space: ModelSpace, deflection = 0.1): Promise<Uint8Array> {
  const { meshTransferToGlb, mergeMeshTransfers } = await import("@semio-tech/framework-3d-js");
  const kernel = new BrepjsKernel();
  const meshes = [];
  for (const model of Object.values(space.models)) {
    for (const solid of Object.values(model.solids)) {
      const mesh = await kernel.tessellate(solid.id, deflection, model);
      if (mesh.position.length > 0 && mesh.index.length > 0) meshes.push(mesh);
    }
  }
  if (meshes.length === 0) return new Uint8Array([0x67, 0x6c, 0x54, 0x46, 0x02, 0x00, 0x00, 0x00]);
  return meshTransferToGlb(mergeMeshTransfers(meshes));
}

/** 🪜️ Exports `model` via a fresh `BrepjsKernel` (convenience). */
export async function exportModelToStep(model: Model, modelId = "model"): Promise<string> {
  const kernel = new BrepjsKernel();
  return kernel.exportModelToStep(model, modelId);
}

/** 🪜️ Imports STEP text via a fresh `BrepjsKernel` (convenience). */
export async function importStepToModelSpace(stepText: string): Promise<ModelSpace> {
  const kernel = new BrepjsKernel();
  return kernel.importStepToModelSpace(stepText);
}

/** 🪜️ Imports raw BREP STEP text via a fresh `BrepjsKernel` (convenience). */
export async function importStepBrepToModelSpace(stepText: string, options?: { readonly prefix?: string; readonly lengthScale?: number }): Promise<ModelSpace> {
  const kernel = new BrepjsKernel();
  return kernel.importStepBrepToModelSpace(stepText, options);
}

/** 🏗️ Imports presentation-layer BIM STEP text via a fresh `BrepjsKernel` (convenience). */
export async function importStepBimToModelSpace(stepText: string, options?: { readonly prefix?: string; readonly lengthScale?: number; readonly modelDefinitionId?: string }): Promise<ModelSpace> {
  const kernel = new BrepjsKernel();
  return kernel.importStepBimToModelSpace(stepText, options);
}
// #endregion 🔌️BrepjsKernel
