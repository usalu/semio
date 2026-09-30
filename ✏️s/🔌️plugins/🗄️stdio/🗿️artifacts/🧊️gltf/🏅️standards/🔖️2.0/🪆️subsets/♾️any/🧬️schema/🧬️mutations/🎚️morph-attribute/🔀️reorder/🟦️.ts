/** 🔀️ `reorder-morph-target-attributes` wire twin: the flat `Apply` payload `GltfReorderMorphTargetAttributesPayload` and the phase wire `ReorderMorphTargetAttributesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderMorphTargetAttributesPayload {
  mesh: number;
  primitive: number;
  target: number;
  order: string[];
}

export type ReorderMorphTargetAttributesMutation = GltfPhase<GltfReorderMorphTargetAttributesPayload, GltfDiff>;

export const parseGltfReorderMorphTargetAttributesPayload = gltfWireObject<GltfReorderMorphTargetAttributesPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireString)) });
export const parseReorderMorphTargetAttributesMutation = gltfWirePhase(parseGltfReorderMorphTargetAttributesPayload, parseGltfDiff);
