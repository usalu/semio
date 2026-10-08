/** 🚚️ `move-texture` wire twin: the flat `Apply` payload `GltfMoveTexturePayload` and the phase wire `MoveTextureMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveTexturePayload {
  index: bigint;
  position: bigint;
}

export type MoveTextureMutation = GltfApplyPhase<GltfMoveTexturePayload>;

export const parseGltfMoveTexturePayload = gltfWireObject<GltfMoveTexturePayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveTextureMutation = gltfWireApplyPhase(parseGltfMoveTexturePayload);
