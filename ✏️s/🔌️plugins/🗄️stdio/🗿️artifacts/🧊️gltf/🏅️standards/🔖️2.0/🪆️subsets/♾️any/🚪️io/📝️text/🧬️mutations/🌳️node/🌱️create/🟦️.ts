/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateNodePayload,CreateNodeMutation} from "../../../../../🧬️schema/🧬️mutations/🌳️node/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌳️node/🌱️create/🟦️.ts";
/** 🌱️ `create-node` wire twin: the flat `Apply` payload `GltfCreateNodePayload` and the phase wire `CreateNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, parseGltfNode } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateNodePayload = gltfWireObject<GltfCreateNodePayload>({ position: gltfWireRequired(gltfWireIndex), node: gltfWireOptional(parseGltfNode) });
export const parseCreateNodeMutation = gltfWireApplyPhase(parseGltfCreateNodePayload);
