/** 🌱️ `create-morph-target` wire twin: the flat `Apply` payload `GltfCreateMorphTargetPayload` and the phase wire `CreateMorphTargetMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfMorphTarget, parseGltfMorphTarget } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateMorphTargetPayload {
  mesh: bigint;
  primitive: bigint;
  position: bigint;
  target?: GltfMorphTarget;
}

export type CreateMorphTargetMutation = GltfApplyPhase<GltfCreateMorphTargetPayload>;

export const parseGltfCreateMorphTargetPayload = gltfWireObject<GltfCreateMorphTargetPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex), target: gltfWireOptional(parseGltfMorphTarget) });
export const parseCreateMorphTargetMutation = gltfWireApplyPhase(parseGltfCreateMorphTargetPayload);
