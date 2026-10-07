/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderImagesPayload,ReorderImagesMutation} from "../../../../../🧬️schema/🧬️mutations/🖼️image/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🖼️image/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-images` wire twin: the flat `Apply` payload `GltfReorderImagesPayload` and the phase wire `ReorderImagesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderImagesPayload = gltfWireObject<GltfReorderImagesPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderImagesMutation = gltfWirePhase(parseGltfReorderImagesPayload, parseGltfDiff);
