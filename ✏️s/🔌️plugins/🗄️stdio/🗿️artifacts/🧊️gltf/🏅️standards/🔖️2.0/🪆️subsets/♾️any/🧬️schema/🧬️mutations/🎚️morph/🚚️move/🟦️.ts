/** 🚚️ `move-morph-target-attribute` wire twin: the flat `Apply` payload `GltfMoveMorphTargetAttributePayload` and the phase wire `MoveMorphTargetAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveMorphTargetAttributePayload {
  mesh: bigint;
  primitive: bigint;
  target: bigint;
  semantic: string;
  position: bigint;
}

export type MoveMorphTargetAttributeMutation = GltfApplyPhase<GltfMoveMorphTargetAttributePayload>;

export const parseGltfMoveMorphTargetAttributePayload = gltfWireObject<GltfMoveMorphTargetAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMorphTargetAttributeMutation = gltfWireApplyPhase(parseGltfMoveMorphTargetAttributePayload);
