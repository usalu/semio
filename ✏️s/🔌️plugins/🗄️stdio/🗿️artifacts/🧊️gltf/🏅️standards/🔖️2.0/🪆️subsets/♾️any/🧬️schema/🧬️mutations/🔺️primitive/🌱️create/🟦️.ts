/** 🌱️ `create-primitive` wire twin: the flat `Apply` payload `GltfCreatePrimitivePayload` and the phase wire `CreatePrimitiveMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreatePrimitivePayload {
  mesh: bigint;
  position: bigint;
}

export type CreatePrimitiveMutation = GltfPhase<GltfCreatePrimitivePayload, GltfDiff>;

export const parseGltfCreatePrimitivePayload = gltfWireObject<GltfCreatePrimitivePayload>({ mesh: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseCreatePrimitiveMutation = gltfWirePhase(parseGltfCreatePrimitivePayload, parseGltfDiff);
