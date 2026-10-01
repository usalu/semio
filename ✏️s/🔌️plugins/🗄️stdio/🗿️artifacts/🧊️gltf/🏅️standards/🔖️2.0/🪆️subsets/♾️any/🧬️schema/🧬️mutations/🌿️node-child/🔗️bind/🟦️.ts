/** 🔗️ `bind-node-child` wire twin: the flat `Apply` payload `GltfBindNodeChildPayload` and the phase wire `BindNodeChildMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindNodeChildPayload {
  parent: bigint;
  child: bigint;
  position: bigint;
}

export type BindNodeChildMutation = GltfPhase<GltfBindNodeChildPayload, GltfDiff>;

export const parseGltfBindNodeChildPayload = gltfWireObject<GltfBindNodeChildPayload>({ parent: gltfWireRequired(gltfWireIndex), child: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseBindNodeChildMutation = gltfWirePhase(parseGltfBindNodeChildPayload, parseGltfDiff);
