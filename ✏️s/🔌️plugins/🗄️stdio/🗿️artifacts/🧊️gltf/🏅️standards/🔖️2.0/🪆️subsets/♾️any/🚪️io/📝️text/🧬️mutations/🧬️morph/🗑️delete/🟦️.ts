/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteMorphTargetPayload,DeleteMorphTargetMutation} from "../../../../../🧬️schema/🧬️mutations/🧬️morph/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🧬️morph/🗑️delete/🟦️.ts";
/** 🗑️ `delete-morph-target` wire twin: the flat `Apply` payload `GltfDeleteMorphTargetPayload` and the phase wire `DeleteMorphTargetMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteMorphTargetPayload = gltfWireObject<GltfDeleteMorphTargetPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex) });
export const parseDeleteMorphTargetMutation = gltfWirePhase(parseGltfDeleteMorphTargetPayload, parseGltfDiff);
