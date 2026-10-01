/** ✂️ `unbind-node-child` wire twin: the flat `Apply` payload `GltfUnbindNodeChildPayload` and the phase wire `UnbindNodeChildMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindNodeChildPayload {
  parent: bigint;
  child: bigint;
}

export type UnbindNodeChildMutation = GltfPhase<GltfUnbindNodeChildPayload, GltfDiff>;

export const parseGltfUnbindNodeChildPayload = gltfWireObject<GltfUnbindNodeChildPayload>({ parent: gltfWireRequired(gltfWireIndex), child: gltfWireRequired(gltfWireIndex) });
export const parseUnbindNodeChildMutation = gltfWirePhase(parseGltfUnbindNodeChildPayload, parseGltfDiff);
