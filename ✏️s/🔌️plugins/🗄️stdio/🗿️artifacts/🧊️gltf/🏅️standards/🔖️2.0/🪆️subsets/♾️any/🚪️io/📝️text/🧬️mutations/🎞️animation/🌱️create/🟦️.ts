/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateAnimationPayload,CreateAnimationMutation} from "../../../../../🧬️schema/🧬️mutations/🎞️animation/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎞️animation/🌱️create/🟦️.ts";
/** 🌱️ `create-animation` wire twin: the flat `Apply` payload `GltfCreateAnimationPayload` and the phase wire `CreateAnimationMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateAnimationPayload = gltfWireObject<GltfCreateAnimationPayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateAnimationMutation = gltfWirePhase(parseGltfCreateAnimationPayload, parseGltfDiff);
