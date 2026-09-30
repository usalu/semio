/** 🔗️ `bind-node-mesh` wire twin: the flat `Apply` payload `GltfBindNodeMeshPayload` and the phase wire `BindNodeMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindNodeMeshPayload {
  node: number;
  mesh: number;
}

export type BindNodeMeshMutation = GltfPhase<GltfBindNodeMeshPayload, GltfDiff>;

export const parseGltfBindNodeMeshPayload = gltfWireObject<GltfBindNodeMeshPayload>({ node: gltfWireRequired(gltfWireIndex), mesh: gltfWireRequired(gltfWireIndex) });
export const parseBindNodeMeshMutation = gltfWirePhase(parseGltfBindNodeMeshPayload, parseGltfDiff);
