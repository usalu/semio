/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderRequiredExtensionsPayload,ReorderRequiredExtensionsMutation} from "../../../../../🧬️schema/🧬️mutations/✅️required/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/✅️required/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-required-extensions` wire twin: the flat `Apply` payload `GltfReorderRequiredExtensionsPayload` and the phase wire `ReorderRequiredExtensionsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderRequiredExtensionsPayload = gltfWireObject<GltfReorderRequiredExtensionsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireString)) });
export const parseReorderRequiredExtensionsMutation = gltfWirePhase(parseGltfReorderRequiredExtensionsPayload, parseGltfDiff);
