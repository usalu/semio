/** 🌱️ `create-skin` wire twin: the flat `Apply` payload `GltfCreateSkinPayload` and the phase wire `CreateSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfSkin, parseGltfSkin } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateSkinPayload {
  position: bigint;
  skin?: GltfSkin;
}

export type CreateSkinMutation = GltfApplyPhase<GltfCreateSkinPayload>;

export const parseGltfCreateSkinPayload = gltfWireObject<GltfCreateSkinPayload>({ position: gltfWireRequired(gltfWireIndex), skin: gltfWireOptional(parseGltfSkin) });
export const parseCreateSkinMutation = gltfWireApplyPhase(parseGltfCreateSkinPayload);
