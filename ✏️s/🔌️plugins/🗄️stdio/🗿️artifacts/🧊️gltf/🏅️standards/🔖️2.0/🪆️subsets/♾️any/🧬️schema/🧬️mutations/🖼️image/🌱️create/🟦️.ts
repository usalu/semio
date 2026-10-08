/** 🌱️ `create-image` wire twin: the flat `Apply` payload `GltfCreateImagePayload` and the phase wire `CreateImageMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfImage, parseGltfImage } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateImagePayload {
  position: bigint;
  image?: GltfImage;
}

export type CreateImageMutation = GltfApplyPhase<GltfCreateImagePayload>;

export const parseGltfCreateImagePayload = gltfWireObject<GltfCreateImagePayload>({ position: gltfWireRequired(gltfWireIndex), image: gltfWireOptional(parseGltfImage) });
export const parseCreateImageMutation = gltfWireApplyPhase(parseGltfCreateImagePayload);
