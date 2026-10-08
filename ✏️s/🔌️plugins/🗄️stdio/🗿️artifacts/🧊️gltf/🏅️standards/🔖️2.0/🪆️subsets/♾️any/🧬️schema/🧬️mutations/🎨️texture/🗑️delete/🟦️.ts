/** 🗑️ `delete-texture` wire twin: the flat `Apply` payload `GltfDeleteTexturePayload` and the phase wire `DeleteTextureMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteTexturePayload {
  index: bigint;
}

export type DeleteTextureMutation = GltfApplyPhase<GltfDeleteTexturePayload>;

export const parseGltfDeleteTexturePayload = gltfWireObject<GltfDeleteTexturePayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteTextureMutation = gltfWireApplyPhase(parseGltfDeleteTexturePayload);
