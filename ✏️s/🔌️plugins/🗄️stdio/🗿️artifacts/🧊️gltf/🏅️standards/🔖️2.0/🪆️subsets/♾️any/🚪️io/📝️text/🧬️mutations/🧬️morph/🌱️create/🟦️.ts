/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateMorphTargetPayload,CreateMorphTargetMutation} from "../../../../../🧬️schema/🧬️mutations/🧬️morph/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🧬️morph/🌱️create/🟦️.ts";
/** 🌱️ `create-morph-target` wire twin: the flat `Apply` payload `GltfCreateMorphTargetPayload` and the phase wire `CreateMorphTargetMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateMorphTargetPayload = gltfWireObject<GltfCreateMorphTargetPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseCreateMorphTargetMutation = gltfWirePhase(parseGltfCreateMorphTargetPayload, parseGltfDiff);
