/** 🌱️ `create-material` wire twin: the flat `Apply` payload `GltfCreateMaterialPayload` and the phase wire `CreateMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfMaterial, parseGltfMaterial } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateMaterialPayload {
  position: bigint;
  material?: GltfMaterial;
}

export type CreateMaterialMutation = GltfApplyPhase<GltfCreateMaterialPayload>;

export const parseGltfCreateMaterialPayload = gltfWireObject<GltfCreateMaterialPayload>({ position: gltfWireRequired(gltfWireIndex), material: gltfWireOptional(parseGltfMaterial) });
export const parseCreateMaterialMutation = gltfWireApplyPhase(parseGltfCreateMaterialPayload);
