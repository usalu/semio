/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderNodesPayload,ReorderNodesMutation} from "../../../../../🧬️schema/🧬️mutations/🌳️node/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌳️node/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-nodes` wire twin: the flat `Apply` payload `GltfReorderNodesPayload` and the phase wire `ReorderNodesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderNodesPayload = gltfWireObject<GltfReorderNodesPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderNodesMutation = gltfWireApplyPhase(parseGltfReorderNodesPayload);
