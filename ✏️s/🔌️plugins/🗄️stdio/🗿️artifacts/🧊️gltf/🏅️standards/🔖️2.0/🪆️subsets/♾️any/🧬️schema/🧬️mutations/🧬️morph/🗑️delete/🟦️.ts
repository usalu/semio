/** 🗑️ `delete-morph-target` wire twin: the flat `Apply` payload `GltfDeleteMorphTargetPayload` and the phase wire `DeleteMorphTargetMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteMorphTargetPayload {
  mesh: bigint;
  primitive: bigint;
  target: bigint;
}

export type DeleteMorphTargetMutation = GltfApplyPhase<GltfDeleteMorphTargetPayload>;

export const parseGltfDeleteMorphTargetPayload = gltfWireObject<GltfDeleteMorphTargetPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex) });
export const parseDeleteMorphTargetMutation = gltfWireApplyPhase(parseGltfDeleteMorphTargetPayload);
