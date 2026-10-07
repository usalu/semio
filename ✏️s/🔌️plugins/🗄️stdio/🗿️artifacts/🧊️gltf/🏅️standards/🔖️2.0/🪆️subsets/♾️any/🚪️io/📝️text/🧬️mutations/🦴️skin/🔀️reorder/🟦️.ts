/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderSkinsPayload,ReorderSkinsMutation} from "../../../../../🧬️schema/🧬️mutations/🦴️skin/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🦴️skin/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-skins` wire twin: the flat `Apply` payload `GltfReorderSkinsPayload` and the phase wire `ReorderSkinsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderSkinsPayload = gltfWireObject<GltfReorderSkinsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderSkinsMutation = gltfWirePhase(parseGltfReorderSkinsPayload, parseGltfDiff);
