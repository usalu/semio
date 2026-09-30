/** 🗑️ `delete-material` wire twin: the flat `Apply` payload `GltfDeleteMaterialPayload` and the phase wire `DeleteMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteMaterialPayload {
  index: number;
}

export type DeleteMaterialMutation = GltfPhase<GltfDeleteMaterialPayload, GltfDiff>;

export const parseGltfDeleteMaterialPayload = gltfWireObject<GltfDeleteMaterialPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteMaterialMutation = gltfWirePhase(parseGltfDeleteMaterialPayload, parseGltfDiff);
