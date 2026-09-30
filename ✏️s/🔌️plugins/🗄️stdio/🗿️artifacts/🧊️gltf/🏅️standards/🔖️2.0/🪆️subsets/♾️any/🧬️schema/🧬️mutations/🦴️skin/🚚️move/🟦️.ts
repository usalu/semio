/** 🚚️ `move-skin` wire twin: the flat `Apply` payload `GltfMoveSkinPayload` and the phase wire `MoveSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveSkinPayload {
  index: number;
  position: number;
}

export type MoveSkinMutation = GltfPhase<GltfMoveSkinPayload, GltfDiff>;

export const parseGltfMoveSkinPayload = gltfWireObject<GltfMoveSkinPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveSkinMutation = gltfWirePhase(parseGltfMoveSkinPayload, parseGltfDiff);
