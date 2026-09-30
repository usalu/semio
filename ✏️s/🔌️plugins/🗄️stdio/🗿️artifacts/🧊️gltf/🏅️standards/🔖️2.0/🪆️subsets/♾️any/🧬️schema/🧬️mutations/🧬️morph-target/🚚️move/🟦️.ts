/** 🚚️ `move-morph-target` wire twin: the flat `Apply` payload `GltfMoveMorphTargetPayload` and the phase wire `MoveMorphTargetMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveMorphTargetPayload {
  mesh: number;
  primitive: number;
  target: number;
  position: number;
}

export type MoveMorphTargetMutation = GltfPhase<GltfMoveMorphTargetPayload, GltfDiff>;

export const parseGltfMoveMorphTargetPayload = gltfWireObject<GltfMoveMorphTargetPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMorphTargetMutation = gltfWirePhase(parseGltfMoveMorphTargetPayload, parseGltfDiff);
