/** 📐️ `change-primitive-topology-mode` wire twin: the flat `Apply` payload `GltfChangePrimitiveTopologyModePayload` and the phase wire `ChangePrimitiveTopologyModeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangePrimitiveTopologyModePayload {
  mesh: number;
  primitive: number;
  mode: number;
}

export type ChangePrimitiveTopologyModeMutation = GltfPhase<GltfChangePrimitiveTopologyModePayload, GltfDiff>;

export const parseGltfChangePrimitiveTopologyModePayload = gltfWireObject<GltfChangePrimitiveTopologyModePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), mode: gltfWireRequired(gltfWireIndex) });
export const parseChangePrimitiveTopologyModeMutation = gltfWirePhase(parseGltfChangePrimitiveTopologyModePayload, parseGltfDiff);
