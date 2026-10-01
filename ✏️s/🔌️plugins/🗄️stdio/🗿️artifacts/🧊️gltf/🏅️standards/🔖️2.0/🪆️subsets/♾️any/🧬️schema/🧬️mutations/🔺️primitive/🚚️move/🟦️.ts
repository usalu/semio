/** 🚚️ `move-primitive` wire twin: the flat `Apply` payload `GltfMovePrimitivePayload` and the phase wire `MovePrimitiveMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMovePrimitivePayload {
  mesh: bigint;
  primitive: bigint;
  position: bigint;
}

export type MovePrimitiveMutation = GltfPhase<GltfMovePrimitivePayload, GltfDiff>;

export const parseGltfMovePrimitivePayload = gltfWireObject<GltfMovePrimitivePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMovePrimitiveMutation = gltfWirePhase(parseGltfMovePrimitivePayload, parseGltfDiff);
