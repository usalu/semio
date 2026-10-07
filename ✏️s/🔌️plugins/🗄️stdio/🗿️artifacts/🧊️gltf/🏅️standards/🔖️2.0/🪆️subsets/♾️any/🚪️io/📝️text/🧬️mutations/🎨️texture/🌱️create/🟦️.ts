/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateTexturePayload,CreateTextureMutation} from "../../../../../🧬️schema/🧬️mutations/🎨️texture/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎨️texture/🌱️create/🟦️.ts";
/** 🌱️ `create-texture` wire twin: the flat `Apply` payload `GltfCreateTexturePayload` and the phase wire `CreateTextureMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateTexturePayload = gltfWireObject<GltfCreateTexturePayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateTextureMutation = gltfWirePhase(parseGltfCreateTexturePayload, parseGltfDiff);
