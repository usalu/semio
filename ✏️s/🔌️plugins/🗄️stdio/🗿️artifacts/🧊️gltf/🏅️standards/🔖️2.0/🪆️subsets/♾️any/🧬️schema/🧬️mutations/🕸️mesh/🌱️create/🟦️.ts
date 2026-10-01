/** 🌱️ `create-mesh` wire twin: the flat `Apply` payload `GltfCreateMeshPayload` and the phase wire `CreateMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateMeshPayload {
  position: bigint;
}

export type CreateMeshMutation = GltfPhase<GltfCreateMeshPayload, GltfDiff>;

export const parseGltfCreateMeshPayload = gltfWireObject<GltfCreateMeshPayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateMeshMutation = gltfWirePhase(parseGltfCreateMeshPayload, parseGltfDiff);
