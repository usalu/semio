/** 🚚️ `move-primitive` wire twin: the flat `Apply` payload `GltfMovePrimitivePayload` and the phase wire `MovePrimitiveMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMovePrimitivePayload {
  mesh: bigint;
  primitive: bigint;
  position: bigint;
}

export type MovePrimitiveMutation = GltfApplyPhase<GltfMovePrimitivePayload>;

export const parseGltfMovePrimitivePayload = gltfWireObject<GltfMovePrimitivePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMovePrimitiveMutation = gltfWireApplyPhase(parseGltfMovePrimitivePayload);
