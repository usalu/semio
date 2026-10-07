/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveTexturePayload,MoveTextureMutation} from "../../../../../🧬️schema/🧬️mutations/🎨️texture/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎨️texture/🚚️move/🟦️.ts";
/** 🚚️ `move-texture` wire twin: the flat `Apply` payload `GltfMoveTexturePayload` and the phase wire `MoveTextureMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveTexturePayload = gltfWireObject<GltfMoveTexturePayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveTextureMutation = gltfWirePhase(parseGltfMoveTexturePayload, parseGltfDiff);
