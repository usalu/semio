/** 🔗️ `bind-morph-target-attribute` wire twin: the flat `Apply` payload `GltfBindMorphTargetAttributePayload` and the phase wire `BindMorphTargetAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindMorphTargetAttributePayload {
  mesh: bigint;
  primitive: bigint;
  target: bigint;
  semantic: string;
  accessor: bigint;
}

export type BindMorphTargetAttributeMutation = GltfPhase<GltfBindMorphTargetAttributePayload, GltfDiff>;

export const parseGltfBindMorphTargetAttributePayload = gltfWireObject<GltfBindMorphTargetAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString), accessor: gltfWireRequired(gltfWireIndex) });
export const parseBindMorphTargetAttributeMutation = gltfWirePhase(parseGltfBindMorphTargetAttributePayload, parseGltfDiff);
