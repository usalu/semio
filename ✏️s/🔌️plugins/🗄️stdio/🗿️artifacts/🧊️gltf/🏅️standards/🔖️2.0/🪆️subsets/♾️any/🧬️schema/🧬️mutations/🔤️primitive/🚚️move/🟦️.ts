/** 🚚️ `move-primitive-attribute` wire twin: the flat `Apply` payload `GltfMovePrimitiveAttributePayload` and the phase wire `MovePrimitiveAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMovePrimitiveAttributePayload {
  mesh: bigint;
  primitive: bigint;
  semantic: string;
  position: bigint;
}

export type MovePrimitiveAttributeMutation = GltfPhase<GltfMovePrimitiveAttributePayload, GltfDiff>;

export const parseGltfMovePrimitiveAttributePayload = gltfWireObject<GltfMovePrimitiveAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseMovePrimitiveAttributeMutation = gltfWirePhase(parseGltfMovePrimitiveAttributePayload, parseGltfDiff);
