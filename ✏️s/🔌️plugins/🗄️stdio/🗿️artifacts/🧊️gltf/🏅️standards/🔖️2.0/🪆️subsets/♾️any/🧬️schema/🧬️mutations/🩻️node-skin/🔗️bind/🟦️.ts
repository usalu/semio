/** 🔗️ `bind-node-skin` wire twin: the flat `Apply` payload `GltfBindNodeSkinPayload` and the phase wire `BindNodeSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindNodeSkinPayload {
  node: bigint;
  skin: bigint;
}

export type BindNodeSkinMutation = GltfPhase<GltfBindNodeSkinPayload, GltfDiff>;

export const parseGltfBindNodeSkinPayload = gltfWireObject<GltfBindNodeSkinPayload>({ node: gltfWireRequired(gltfWireIndex), skin: gltfWireRequired(gltfWireIndex) });
export const parseBindNodeSkinMutation = gltfWirePhase(parseGltfBindNodeSkinPayload, parseGltfDiff);
