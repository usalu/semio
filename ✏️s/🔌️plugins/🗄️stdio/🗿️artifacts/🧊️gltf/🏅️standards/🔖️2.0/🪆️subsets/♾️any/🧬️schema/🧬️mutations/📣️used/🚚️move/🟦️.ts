/** 🚚️ `move-used-extension` wire twin: the flat `Apply` payload `GltfMoveUsedExtensionPayload` and the phase wire `MoveUsedExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveUsedExtensionPayload {
  extension: string;
  position: bigint;
}

export type MoveUsedExtensionMutation = GltfApplyPhase<GltfMoveUsedExtensionPayload>;

export const parseGltfMoveUsedExtensionPayload = gltfWireObject<GltfMoveUsedExtensionPayload>({ extension: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveUsedExtensionMutation = gltfWireApplyPhase(parseGltfMoveUsedExtensionPayload);
