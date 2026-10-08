/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderTexturesPayload,ReorderTexturesMutation} from "../../../../../🧬️schema/🧬️mutations/🎨️texture/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎨️texture/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-textures` wire twin: the flat `Apply` payload `GltfReorderTexturesPayload` and the phase wire `ReorderTexturesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderTexturesPayload = gltfWireObject<GltfReorderTexturesPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderTexturesMutation = gltfWireApplyPhase(parseGltfReorderTexturesPayload);
