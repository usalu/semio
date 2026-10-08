/** 🔗️ `bind-node-skin` wire twin: the flat `Apply` payload `GltfBindNodeSkinPayload` and the phase wire `BindNodeSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindNodeSkinPayload {
  node: bigint;
  skin: bigint;
}

export type BindNodeSkinMutation = GltfApplyPhase<GltfBindNodeSkinPayload>;

export const parseGltfBindNodeSkinPayload = gltfWireObject<GltfBindNodeSkinPayload>({ node: gltfWireRequired(gltfWireIndex), skin: gltfWireRequired(gltfWireIndex) });
export const parseBindNodeSkinMutation = gltfWireApplyPhase(parseGltfBindNodeSkinPayload);
