/** 🏷️ `change-mesh-name` wire twin: the flat `Apply` payload `GltfChangeMeshNamePayload` and the phase wire `ChangeMeshNameMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireNullable, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeMeshNamePayload {
  mesh: bigint;
  value: string | null;
}

export type ChangeMeshNameMutation = GltfApplyPhase<GltfChangeMeshNamePayload>;

export const parseGltfChangeMeshNamePayload = gltfWireObject<GltfChangeMeshNamePayload>({ mesh: gltfWireRequired(gltfWireIndex), value: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseChangeMeshNameMutation = gltfWireApplyPhase(parseGltfChangeMeshNamePayload);
