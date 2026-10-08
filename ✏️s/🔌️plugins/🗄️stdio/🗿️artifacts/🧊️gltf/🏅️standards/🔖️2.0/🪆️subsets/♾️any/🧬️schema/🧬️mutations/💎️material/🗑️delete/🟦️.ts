/** 🗑️ `delete-material` wire twin: the flat `Apply` payload `GltfDeleteMaterialPayload` and the phase wire `DeleteMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteMaterialPayload {
  index: bigint;
}

export type DeleteMaterialMutation = GltfApplyPhase<GltfDeleteMaterialPayload>;

export const parseGltfDeleteMaterialPayload = gltfWireObject<GltfDeleteMaterialPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteMaterialMutation = gltfWireApplyPhase(parseGltfDeleteMaterialPayload);
