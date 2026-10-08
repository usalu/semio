/** 🗑️ `delete-mesh` wire twin: the flat `Apply` payload `GltfDeleteMeshPayload` and the phase wire `DeleteMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteMeshPayload {
  index: bigint;
}

export type DeleteMeshMutation = GltfApplyPhase<GltfDeleteMeshPayload>;

export const parseGltfDeleteMeshPayload = gltfWireObject<GltfDeleteMeshPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteMeshMutation = gltfWireApplyPhase(parseGltfDeleteMeshPayload);
