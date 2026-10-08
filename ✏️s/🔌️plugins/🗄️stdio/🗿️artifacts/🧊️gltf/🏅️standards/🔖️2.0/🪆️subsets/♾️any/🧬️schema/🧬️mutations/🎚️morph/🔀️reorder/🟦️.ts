/** 🔀️ `reorder-morph-target-attributes` wire twin: the flat `Apply` payload `GltfReorderMorphTargetAttributesPayload` and the phase wire `ReorderMorphTargetAttributesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderMorphTargetAttributesPayload {
  mesh: bigint;
  primitive: bigint;
  target: bigint;
  order: string[];
}

export type ReorderMorphTargetAttributesMutation = GltfApplyPhase<GltfReorderMorphTargetAttributesPayload>;

export const parseGltfReorderMorphTargetAttributesPayload = gltfWireObject<GltfReorderMorphTargetAttributesPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireString)) });
export const parseReorderMorphTargetAttributesMutation = gltfWireApplyPhase(parseGltfReorderMorphTargetAttributesPayload);
