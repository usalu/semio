/** ✂️ `unbind-node-child` wire twin: the flat `Apply` payload `GltfUnbindNodeChildPayload` and the phase wire `UnbindNodeChildMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindNodeChildPayload {
  parent: bigint;
  child: bigint;
}

export type UnbindNodeChildMutation = GltfApplyPhase<GltfUnbindNodeChildPayload>;

export const parseGltfUnbindNodeChildPayload = gltfWireObject<GltfUnbindNodeChildPayload>({ parent: gltfWireRequired(gltfWireIndex), child: gltfWireRequired(gltfWireIndex) });
export const parseUnbindNodeChildMutation = gltfWireApplyPhase(parseGltfUnbindNodeChildPayload);
