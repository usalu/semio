/** 🔗️ `bind-node-mesh` wire twin: the flat `Apply` payload `GltfBindNodeMeshPayload` and the phase wire `BindNodeMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindNodeMeshPayload {
  node: bigint;
  mesh: bigint;
}

export type BindNodeMeshMutation = GltfApplyPhase<GltfBindNodeMeshPayload>;

export const parseGltfBindNodeMeshPayload = gltfWireObject<GltfBindNodeMeshPayload>({ node: gltfWireRequired(gltfWireIndex), mesh: gltfWireRequired(gltfWireIndex) });
export const parseBindNodeMeshMutation = gltfWireApplyPhase(parseGltfBindNodeMeshPayload);
