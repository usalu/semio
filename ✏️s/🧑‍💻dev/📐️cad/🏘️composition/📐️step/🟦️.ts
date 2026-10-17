/** 📐️ CAD development composition explicitly binds STEP geometry operations and browser publication. */
import { SemioBrepKernel } from "@semio-tech/s-spatial-kernel-ts/semio";
import { SemioGeometrySession, requireBrowserGeometrySessionV1 } from "@semio-tech/s-spatial-kernel-semio-session-ts";
import type { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
async function initializeBindings() {
  const bindings=await import("../../📦️packages/🦀️rust/🕸️bindings/cad_geometry.js");
  const source=new URL("../../📦️packages/🦀️rust/🕸️bindings/cad_geometry_bg.wasm",import.meta.url);
  const module_or_path=source.protocol === "file:" ? await (await import("node:fs/promises")).readFile(source) : source;
  await bindings.default({ module_or_path });
  return bindings;
}
let initialization:ReturnType<typeof initializeBindings> | undefined;
/** 🌊️ One explicit outward session owns both general geometry authority and STEP codec operations. */
export function createStepGeometrySession():SemioGeometrySession { return new SemioGeometrySession(async() => requireBrowserGeometrySessionV1(new (await(initialization ??= initializeBindings())).BrowserSession())); }
/** 🧊️ Constructs a kernel with explicit format contribution ownership. */
export function createStepGeometryKernel():SemioBrepKernel { return new SemioBrepKernel(createStepGeometrySession()); }
/** 📤️ Exports a model through the STEP owner while retaining the kernel's derived geometry claims. */
export async function exportModelToStep(kernel:SemioBrepKernel,model:Model):Promise<string> {
  const shapes=await kernel.geometryHandles(model);
  if (!shapes.length) return "";
  return (await kernel.invokeGeometry<{value:string}>("exportStep",{shapes})).value;
}
/** 📥️ Imports STEP shapes into the same explicit retained geometry authority. */
export async function importStepHandles(kernel:SemioBrepKernel,data:string):Promise<readonly string[]> { return (await kernel.invokeGeometry<{handles:readonly string[]}>("importStep",{data})).handles; }
