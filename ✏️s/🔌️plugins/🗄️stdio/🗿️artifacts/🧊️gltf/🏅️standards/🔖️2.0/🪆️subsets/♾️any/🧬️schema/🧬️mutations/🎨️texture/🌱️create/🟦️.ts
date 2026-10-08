/** 🌱️ `create-texture` wire twin: the flat `Apply` payload `GltfCreateTexturePayload` and the phase wire `CreateTextureMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfTexture, parseGltfTexture } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateTexturePayload {
  position: bigint;
  texture?: GltfTexture;
}

export type CreateTextureMutation = GltfApplyPhase<GltfCreateTexturePayload>;

export const parseGltfCreateTexturePayload = gltfWireObject<GltfCreateTexturePayload>({ position: gltfWireRequired(gltfWireIndex), texture: gltfWireOptional(parseGltfTexture) });
export const parseCreateTextureMutation = gltfWireApplyPhase(parseGltfCreateTexturePayload);
