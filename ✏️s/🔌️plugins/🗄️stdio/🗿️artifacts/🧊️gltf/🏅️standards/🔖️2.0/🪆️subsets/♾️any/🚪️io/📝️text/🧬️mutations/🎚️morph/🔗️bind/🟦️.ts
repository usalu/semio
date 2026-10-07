/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfBindMorphTargetAttributePayload,BindMorphTargetAttributeMutation} from "../../../../../🧬️schema/🧬️mutations/🎚️morph/🔗️bind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎚️morph/🔗️bind/🟦️.ts";
/** 🔗️ `bind-morph-target-attribute` wire twin: the flat `Apply` payload `GltfBindMorphTargetAttributePayload` and the phase wire `BindMorphTargetAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfBindMorphTargetAttributePayload = gltfWireObject<GltfBindMorphTargetAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString), accessor: gltfWireRequired(gltfWireIndex) });
export const parseBindMorphTargetAttributeMutation = gltfWirePhase(parseGltfBindMorphTargetAttributePayload, parseGltfDiff);
