/** 🌱️ `create-primitive` wire twin: the flat `Apply` payload `GltfCreatePrimitivePayload` and the phase wire `CreatePrimitiveMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfPrimitive, parseGltfPrimitive } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreatePrimitivePayload {
  mesh: bigint;
  position: bigint;
  primitive?: GltfPrimitive;
}

export type CreatePrimitiveMutation = GltfApplyPhase<GltfCreatePrimitivePayload>;

export const parseGltfCreatePrimitivePayload = gltfWireObject<GltfCreatePrimitivePayload>({ mesh: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex), primitive: gltfWireOptional(parseGltfPrimitive) });
export const parseCreatePrimitiveMutation = gltfWireApplyPhase(parseGltfCreatePrimitivePayload);
