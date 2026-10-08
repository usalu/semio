/** 🔀️ `reorder-skins` wire twin: the flat `Apply` payload `GltfReorderSkinsPayload` and the phase wire `ReorderSkinsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderSkinsPayload {
  order: bigint[];
}

export type ReorderSkinsMutation = GltfApplyPhase<GltfReorderSkinsPayload>;

export const parseGltfReorderSkinsPayload = gltfWireObject<GltfReorderSkinsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderSkinsMutation = gltfWireApplyPhase(parseGltfReorderSkinsPayload);
