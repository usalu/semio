/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderNodeChildrenPayload,ReorderNodeChildrenMutation} from "../../../../../🧬️schema/🧬️mutations/🌿️node-child/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌿️node-child/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-node-children` wire twin: the flat `Apply` payload `GltfReorderNodeChildrenPayload` and the phase wire `ReorderNodeChildrenMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderNodeChildrenPayload = gltfWireObject<GltfReorderNodeChildrenPayload>({ parent: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderNodeChildrenMutation = gltfWirePhase(parseGltfReorderNodeChildrenPayload, parseGltfDiff);
