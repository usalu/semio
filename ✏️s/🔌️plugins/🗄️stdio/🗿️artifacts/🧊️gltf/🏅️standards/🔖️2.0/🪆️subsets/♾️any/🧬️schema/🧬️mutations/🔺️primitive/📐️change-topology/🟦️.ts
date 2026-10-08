/** 📐️ `change-primitive-topology-mode` wire twin: the flat `Apply` payload `GltfChangePrimitiveTopologyModePayload` and the phase wire `ChangePrimitiveTopologyModeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireNullable, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangePrimitiveTopologyModePayload {
  mesh: bigint;
  primitive: bigint;
  mode: bigint | null;
}

export type ChangePrimitiveTopologyModeMutation = GltfApplyPhase<GltfChangePrimitiveTopologyModePayload>;

export const parseGltfChangePrimitiveTopologyModePayload = gltfWireObject<GltfChangePrimitiveTopologyModePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), mode: gltfWireRequired(gltfWireNullable(gltfWireIndex)) });
export const parseChangePrimitiveTopologyModeMutation = gltfWireApplyPhase(parseGltfChangePrimitiveTopologyModePayload);
