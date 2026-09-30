/** 🗑️ `delete-skin` wire twin: the flat `Apply` payload `GltfDeleteSkinPayload` and the phase wire `DeleteSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteSkinPayload {
  index: number;
}

export type DeleteSkinMutation = GltfPhase<GltfDeleteSkinPayload, GltfDiff>;

export const parseGltfDeleteSkinPayload = gltfWireObject<GltfDeleteSkinPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteSkinMutation = gltfWirePhase(parseGltfDeleteSkinPayload, parseGltfDiff);
