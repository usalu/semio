import type {Binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** ⚖️ `change-node-morph-weights` wire twin: the flat `Apply` payload `GltfChangeNodeMorphWeightsPayload` and the phase wire `ChangeNodeMorphWeightsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireNumber, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeNodeMorphWeightsPayload {
  node: bigint;
  weights: Binary64[];
}

export type ChangeNodeMorphWeightsMutation = GltfApplyPhase<GltfChangeNodeMorphWeightsPayload>;

export const parseGltfChangeNodeMorphWeightsPayload = gltfWireObject<GltfChangeNodeMorphWeightsPayload>({ node: gltfWireRequired(gltfWireIndex), weights: gltfWireRequired(gltfWireArray(gltfWireNumber)) });
export const parseChangeNodeMorphWeightsMutation = gltfWireApplyPhase(parseGltfChangeNodeMorphWeightsPayload);
