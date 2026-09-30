/** 🚚️ `move-required-extension` wire twin: the flat `Apply` payload `GltfMoveRequiredExtensionPayload` and the phase wire `MoveRequiredExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveRequiredExtensionPayload {
  extension: string;
  position: number;
}

export type MoveRequiredExtensionMutation = GltfPhase<GltfMoveRequiredExtensionPayload, GltfDiff>;

export const parseGltfMoveRequiredExtensionPayload = gltfWireObject<GltfMoveRequiredExtensionPayload>({ extension: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveRequiredExtensionMutation = gltfWirePhase(parseGltfMoveRequiredExtensionPayload, parseGltfDiff);
