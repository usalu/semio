/** 🚚️ `move-morph-target` wire twin: the flat `Apply` payload `GltfMoveMorphTargetPayload` and the phase wire `MoveMorphTargetMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveMorphTargetPayload {
  mesh: bigint;
  primitive: bigint;
  target: bigint;
  position: bigint;
}

export type MoveMorphTargetMutation = GltfApplyPhase<GltfMoveMorphTargetPayload>;

export const parseGltfMoveMorphTargetPayload = gltfWireObject<GltfMoveMorphTargetPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMorphTargetMutation = gltfWireApplyPhase(parseGltfMoveMorphTargetPayload);
