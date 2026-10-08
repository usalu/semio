/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeNodeMorphWeightsPayload,ChangeNodeMorphWeightsMutation} from "../../../../../🧬️schema/🧬️mutations/🌳️node/⚖️change-weights/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌳️node/⚖️change-weights/🟦️.ts";
import type {Binary64} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** ⚖️ `change-node-morph-weights` wire twin: the flat `Apply` payload `GltfChangeNodeMorphWeightsPayload` and the phase wire `ChangeNodeMorphWeightsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireNumber, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeNodeMorphWeightsPayload = gltfWireObject<GltfChangeNodeMorphWeightsPayload>({ node: gltfWireRequired(gltfWireIndex), weights: gltfWireRequired(gltfWireArray(gltfWireNumber)) });
export const parseChangeNodeMorphWeightsMutation = gltfWireApplyPhase(parseGltfChangeNodeMorphWeightsPayload);
