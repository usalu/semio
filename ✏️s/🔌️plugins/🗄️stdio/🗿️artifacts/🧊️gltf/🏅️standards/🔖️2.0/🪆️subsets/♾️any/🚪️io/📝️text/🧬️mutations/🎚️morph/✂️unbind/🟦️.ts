/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfUnbindMorphTargetAttributePayload,UnbindMorphTargetAttributeMutation} from "../../../../../🧬️schema/🧬️mutations/🎚️morph/✂️unbind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎚️morph/✂️unbind/🟦️.ts";
/** ✂️ `unbind-morph-target-attribute` wire twin: the flat `Apply` payload `GltfUnbindMorphTargetAttributePayload` and the phase wire `UnbindMorphTargetAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfUnbindMorphTargetAttributePayload = gltfWireObject<GltfUnbindMorphTargetAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString) });
export const parseUnbindMorphTargetAttributeMutation = gltfWirePhase(parseGltfUnbindMorphTargetAttributePayload, parseGltfDiff);
