/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderUsedExtensionsPayload,ReorderUsedExtensionsMutation} from "../../../../../🧬️schema/🧬️mutations/📣️used/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📣️used/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-used-extensions` wire twin: the flat `Apply` payload `GltfReorderUsedExtensionsPayload` and the phase wire `ReorderUsedExtensionsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderUsedExtensionsPayload = gltfWireObject<GltfReorderUsedExtensionsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireString)) });
export const parseReorderUsedExtensionsMutation = gltfWirePhase(parseGltfReorderUsedExtensionsPayload, parseGltfDiff);
