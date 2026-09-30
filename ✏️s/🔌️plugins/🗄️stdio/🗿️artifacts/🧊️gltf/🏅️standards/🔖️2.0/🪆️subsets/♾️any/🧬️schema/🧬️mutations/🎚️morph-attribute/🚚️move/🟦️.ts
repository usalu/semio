/** 🚚️ `move-morph-target-attribute` wire twin: the flat `Apply` payload `GltfMoveMorphTargetAttributePayload` and the phase wire `MoveMorphTargetAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveMorphTargetAttributePayload {
  mesh: number;
  primitive: number;
  target: number;
  semantic: string;
  position: number;
}

export type MoveMorphTargetAttributeMutation = GltfPhase<GltfMoveMorphTargetAttributePayload, GltfDiff>;

export const parseGltfMoveMorphTargetAttributePayload = gltfWireObject<GltfMoveMorphTargetAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMorphTargetAttributeMutation = gltfWirePhase(parseGltfMoveMorphTargetAttributePayload, parseGltfDiff);
