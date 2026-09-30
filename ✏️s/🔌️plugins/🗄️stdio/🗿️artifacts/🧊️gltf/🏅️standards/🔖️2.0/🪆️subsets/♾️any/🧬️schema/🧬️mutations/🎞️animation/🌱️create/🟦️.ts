/** 🌱️ `create-animation` wire twin: the flat `Apply` payload `GltfCreateAnimationPayload` and the phase wire `CreateAnimationMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateAnimationPayload {
  position: number;
}

export type CreateAnimationMutation = GltfPhase<GltfCreateAnimationPayload, GltfDiff>;

export const parseGltfCreateAnimationPayload = gltfWireObject<GltfCreateAnimationPayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateAnimationMutation = gltfWirePhase(parseGltfCreateAnimationPayload, parseGltfDiff);
