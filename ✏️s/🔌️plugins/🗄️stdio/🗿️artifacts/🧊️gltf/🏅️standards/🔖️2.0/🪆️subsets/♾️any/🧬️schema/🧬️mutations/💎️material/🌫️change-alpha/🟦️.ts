/** 🌫️ `change-material-alpha-mode` wire twin: the flat `Apply` payload `GltfChangeMaterialAlphaModePayload` and the phase wire `ChangeMaterialAlphaModeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfAlphaMode, gltfWireIndex, gltfWireObject, gltfWireRequired, parseGltfAlphaMode } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeMaterialAlphaModePayload {
  material: bigint;
  alphaMode: GltfAlphaMode;
}

export type ChangeMaterialAlphaModeMutation = GltfPhase<GltfChangeMaterialAlphaModePayload, GltfDiff>;

export const parseGltfChangeMaterialAlphaModePayload = gltfWireObject<GltfChangeMaterialAlphaModePayload>({ material: gltfWireRequired(gltfWireIndex), alphaMode: gltfWireRequired(parseGltfAlphaMode) });
export const parseChangeMaterialAlphaModeMutation = gltfWirePhase(parseGltfChangeMaterialAlphaModePayload, parseGltfDiff);
