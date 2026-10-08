/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteTexturePayload,DeleteTextureMutation} from "../../../../../🧬️schema/🧬️mutations/🎨️texture/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎨️texture/🗑️delete/🟦️.ts";
/** 🗑️ `delete-texture` wire twin: the flat `Apply` payload `GltfDeleteTexturePayload` and the phase wire `DeleteTextureMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteTexturePayload = gltfWireObject<GltfDeleteTexturePayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteTextureMutation = gltfWireApplyPhase(parseGltfDeleteTexturePayload);
