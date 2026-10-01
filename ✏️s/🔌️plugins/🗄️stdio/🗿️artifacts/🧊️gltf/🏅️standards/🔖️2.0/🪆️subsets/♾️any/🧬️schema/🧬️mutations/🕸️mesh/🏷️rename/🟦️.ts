/** 🏷️ `change-mesh-name` wire twin: the flat `Apply` payload `GltfChangeMeshNamePayload` and the phase wire `ChangeMeshNameMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireNullable, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeMeshNamePayload {
  mesh: bigint;
  value: string | null;
}

export type ChangeMeshNameMutation = GltfPhase<GltfChangeMeshNamePayload, GltfDiff>;

export const parseGltfChangeMeshNamePayload = gltfWireObject<GltfChangeMeshNamePayload>({ mesh: gltfWireRequired(gltfWireIndex), value: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseChangeMeshNameMutation = gltfWirePhase(parseGltfChangeMeshNamePayload, parseGltfDiff);
