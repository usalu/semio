/** ✂️ `unbind-morph-target-attribute` wire twin: the flat `Apply` payload `GltfUnbindMorphTargetAttributePayload` and the phase wire `UnbindMorphTargetAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindMorphTargetAttributePayload {
  mesh: bigint;
  primitive: bigint;
  target: bigint;
  semantic: string;
}

export type UnbindMorphTargetAttributeMutation = GltfApplyPhase<GltfUnbindMorphTargetAttributePayload>;

export const parseGltfUnbindMorphTargetAttributePayload = gltfWireObject<GltfUnbindMorphTargetAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString) });
export const parseUnbindMorphTargetAttributeMutation = gltfWireApplyPhase(parseGltfUnbindMorphTargetAttributePayload);
