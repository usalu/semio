/** 🌱️ `create-animation` wire twin: the flat `Apply` payload `GltfCreateAnimationPayload` and the phase wire `CreateAnimationMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfAnimation, parseGltfAnimation } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateAnimationPayload {
  position: bigint;
  animation?: GltfAnimation;
}

export type CreateAnimationMutation = GltfApplyPhase<GltfCreateAnimationPayload>;

export const parseGltfCreateAnimationPayload = gltfWireObject<GltfCreateAnimationPayload>({ position: gltfWireRequired(gltfWireIndex), animation: gltfWireOptional(parseGltfAnimation) });
export const parseCreateAnimationMutation = gltfWireApplyPhase(parseGltfCreateAnimationPayload);
