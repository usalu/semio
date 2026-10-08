/** 🗑️ `delete-skin` wire twin: the flat `Apply` payload `GltfDeleteSkinPayload` and the phase wire `DeleteSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteSkinPayload {
  index: bigint;
}

export type DeleteSkinMutation = GltfApplyPhase<GltfDeleteSkinPayload>;

export const parseGltfDeleteSkinPayload = gltfWireObject<GltfDeleteSkinPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteSkinMutation = gltfWireApplyPhase(parseGltfDeleteSkinPayload);
