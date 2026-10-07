/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveRequiredExtensionPayload,MoveRequiredExtensionMutation} from "../../../../../🧬️schema/🧬️mutations/✅️required/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/✅️required/🚚️move/🟦️.ts";
/** 🚚️ `move-required-extension` wire twin: the flat `Apply` payload `GltfMoveRequiredExtensionPayload` and the phase wire `MoveRequiredExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveRequiredExtensionPayload = gltfWireObject<GltfMoveRequiredExtensionPayload>({ extension: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveRequiredExtensionMutation = gltfWirePhase(parseGltfMoveRequiredExtensionPayload, parseGltfDiff);
