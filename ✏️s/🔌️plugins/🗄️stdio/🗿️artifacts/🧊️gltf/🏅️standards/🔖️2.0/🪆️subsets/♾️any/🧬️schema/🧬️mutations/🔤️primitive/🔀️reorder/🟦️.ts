/** 🔀️ `reorder-primitive-attributes` wire twin: the flat `Apply` payload `GltfReorderPrimitiveAttributesPayload` and the phase wire `ReorderPrimitiveAttributesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderPrimitiveAttributesPayload {
  mesh: bigint;
  primitive: bigint;
  order: string[];
}

export type ReorderPrimitiveAttributesMutation = GltfPhase<GltfReorderPrimitiveAttributesPayload, GltfDiff>;

export const parseGltfReorderPrimitiveAttributesPayload = gltfWireObject<GltfReorderPrimitiveAttributesPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireString)) });
export const parseReorderPrimitiveAttributesMutation = gltfWirePhase(parseGltfReorderPrimitiveAttributesPayload, parseGltfDiff);
