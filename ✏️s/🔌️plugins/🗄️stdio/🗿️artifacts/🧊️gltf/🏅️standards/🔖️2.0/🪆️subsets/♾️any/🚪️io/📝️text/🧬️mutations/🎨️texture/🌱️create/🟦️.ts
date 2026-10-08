/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateTexturePayload,CreateTextureMutation} from "../../../../../🧬️schema/🧬️mutations/🎨️texture/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎨️texture/🌱️create/🟦️.ts";
/** 🌱️ `create-texture` wire twin: the flat `Apply` payload `GltfCreateTexturePayload` and the phase wire `CreateTextureMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, parseGltfTexture } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateTexturePayload = gltfWireObject<GltfCreateTexturePayload>({ position: gltfWireRequired(gltfWireIndex), texture: gltfWireOptional(parseGltfTexture) });
export const parseCreateTextureMutation = gltfWireApplyPhase(parseGltfCreateTexturePayload);
