/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderBuffersPayload,ReorderBuffersMutation} from "../../../../../🧬️schema/🧬️mutations/💿️buffer/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/💿️buffer/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-buffers` wire twin: the flat `Apply` payload `GltfReorderBuffersPayload` and the phase wire `ReorderBuffersMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderBuffersPayload = gltfWireObject<GltfReorderBuffersPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderBuffersMutation = gltfWireApplyPhase(parseGltfReorderBuffersPayload);
