/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveMorphTargetPayload,MoveMorphTargetMutation} from "../../../../../🧬️schema/🧬️mutations/🧬️morph/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🧬️morph/🚚️move/🟦️.ts";
/** 🚚️ `move-morph-target` wire twin: the flat `Apply` payload `GltfMoveMorphTargetPayload` and the phase wire `MoveMorphTargetMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveMorphTargetPayload = gltfWireObject<GltfMoveMorphTargetPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMorphTargetMutation = gltfWirePhase(parseGltfMoveMorphTargetPayload, parseGltfDiff);
