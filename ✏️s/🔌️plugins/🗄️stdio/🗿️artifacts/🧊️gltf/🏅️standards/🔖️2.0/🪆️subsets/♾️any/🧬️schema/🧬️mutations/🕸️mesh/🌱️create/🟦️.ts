/** 🌱️ `create-mesh` wire twin: the flat `Apply` payload `GltfCreateMeshPayload` and the phase wire `CreateMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfMesh, parseGltfMesh } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateMeshPayload {
  position: bigint;
  mesh?: GltfMesh;
}

export type CreateMeshMutation = GltfApplyPhase<GltfCreateMeshPayload>;

export const parseGltfCreateMeshPayload = gltfWireObject<GltfCreateMeshPayload>({ position: gltfWireRequired(gltfWireIndex), mesh: gltfWireOptional(parseGltfMesh) });
export const parseCreateMeshMutation = gltfWireApplyPhase(parseGltfCreateMeshPayload);
