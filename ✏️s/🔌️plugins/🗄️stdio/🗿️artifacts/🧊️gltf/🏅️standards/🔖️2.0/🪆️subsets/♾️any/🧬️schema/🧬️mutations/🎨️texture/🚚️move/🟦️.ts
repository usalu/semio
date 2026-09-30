/** 🚚️ `move-texture` wire twin: the flat `Apply` payload `GltfMoveTexturePayload` and the phase wire `MoveTextureMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveTexturePayload {
  index: number;
  position: number;
}

export type MoveTextureMutation = GltfPhase<GltfMoveTexturePayload, GltfDiff>;

export const parseGltfMoveTexturePayload = gltfWireObject<GltfMoveTexturePayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveTextureMutation = gltfWirePhase(parseGltfMoveTexturePayload, parseGltfDiff);
