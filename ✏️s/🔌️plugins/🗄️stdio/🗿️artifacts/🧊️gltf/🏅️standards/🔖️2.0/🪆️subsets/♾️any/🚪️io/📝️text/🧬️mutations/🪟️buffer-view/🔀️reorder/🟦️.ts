/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderBufferViewsPayload,ReorderBufferViewsMutation} from "../../../../../🧬️schema/🧬️mutations/🪟️buffer-view/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🪟️buffer-view/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-buffer-views` wire twin: the flat `Apply` payload `GltfReorderBufferViewsPayload` and the phase wire `ReorderBufferViewsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderBufferViewsPayload = gltfWireObject<GltfReorderBufferViewsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderBufferViewsMutation = gltfWirePhase(parseGltfReorderBufferViewsPayload, parseGltfDiff);
