/** 🌱️ `create-node` wire twin: the flat `Apply` payload `GltfCreateNodePayload` and the phase wire `CreateNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfNode, parseGltfNode } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateNodePayload {
  position: bigint;
  node?: GltfNode;
}

export type CreateNodeMutation = GltfApplyPhase<GltfCreateNodePayload>;

export const parseGltfCreateNodePayload = gltfWireObject<GltfCreateNodePayload>({ position: gltfWireRequired(gltfWireIndex), node: gltfWireOptional(parseGltfNode) });
export const parseCreateNodeMutation = gltfWireApplyPhase(parseGltfCreateNodePayload);
