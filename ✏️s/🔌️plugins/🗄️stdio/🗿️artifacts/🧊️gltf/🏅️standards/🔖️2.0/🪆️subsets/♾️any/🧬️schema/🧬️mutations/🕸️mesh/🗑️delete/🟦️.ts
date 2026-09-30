/** 🗑️ `delete-mesh` wire twin: the flat `Apply` payload `GltfDeleteMeshPayload` and the phase wire `DeleteMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteMeshPayload {
  index: number;
}

export type DeleteMeshMutation = GltfPhase<GltfDeleteMeshPayload, GltfDiff>;

export const parseGltfDeleteMeshPayload = gltfWireObject<GltfDeleteMeshPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteMeshMutation = gltfWirePhase(parseGltfDeleteMeshPayload, parseGltfDiff);
