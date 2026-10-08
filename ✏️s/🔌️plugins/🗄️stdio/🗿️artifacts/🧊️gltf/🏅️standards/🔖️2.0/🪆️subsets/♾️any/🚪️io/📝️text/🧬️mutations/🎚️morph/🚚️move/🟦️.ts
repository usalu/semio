/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveMorphTargetAttributePayload,MoveMorphTargetAttributeMutation} from "../../../../../🧬️schema/🧬️mutations/🎚️morph/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎚️morph/🚚️move/🟦️.ts";
/** 🚚️ `move-morph-target-attribute` wire twin: the flat `Apply` payload `GltfMoveMorphTargetAttributePayload` and the phase wire `MoveMorphTargetAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveMorphTargetAttributePayload = gltfWireObject<GltfMoveMorphTargetAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveMorphTargetAttributeMutation = gltfWireApplyPhase(parseGltfMoveMorphTargetAttributePayload);
