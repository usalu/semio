/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateSkinPayload,CreateSkinMutation} from "../../../../../🧬️schema/🧬️mutations/🦴️skin/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🦴️skin/🌱️create/🟦️.ts";
/** 🌱️ `create-skin` wire twin: the flat `Apply` payload `GltfCreateSkinPayload` and the phase wire `CreateSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, parseGltfSkin } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateSkinPayload = gltfWireObject<GltfCreateSkinPayload>({ position: gltfWireRequired(gltfWireIndex), skin: gltfWireOptional(parseGltfSkin) });
export const parseCreateSkinMutation = gltfWireApplyPhase(parseGltfCreateSkinPayload);
