/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeMaterialAlphaModePayload,ChangeMaterialAlphaModeMutation} from "../../../../../🧬️schema/🧬️mutations/💎️material/🌫️change-alpha/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/💎️material/🌫️change-alpha/🟦️.ts";
/** 🌫️ `change-material-alpha-mode` wire twin: the flat `Apply` payload `GltfChangeMaterialAlphaModePayload` and the phase wire `ChangeMaterialAlphaModeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfAlphaMode, gltfWireIndex, gltfWireObject, gltfWireRequired, parseGltfAlphaMode } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeMaterialAlphaModePayload = gltfWireObject<GltfChangeMaterialAlphaModePayload>({ material: gltfWireRequired(gltfWireIndex), alphaMode: gltfWireRequired(parseGltfAlphaMode) });
export const parseChangeMaterialAlphaModeMutation = gltfWireApplyPhase(parseGltfChangeMaterialAlphaModePayload);
