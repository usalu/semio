/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteAnimationPayload,DeleteAnimationMutation} from "../../../../../🧬️schema/🧬️mutations/🎞️animation/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎞️animation/🗑️delete/🟦️.ts";
/** 🗑️ `delete-animation` wire twin: the flat `Apply` payload `GltfDeleteAnimationPayload` and the phase wire `DeleteAnimationMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteAnimationPayload = gltfWireObject<GltfDeleteAnimationPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteAnimationMutation = gltfWirePhase(parseGltfDeleteAnimationPayload, parseGltfDiff);
