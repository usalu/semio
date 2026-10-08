/** 🏷️ `change-node-name` wire twin: the flat `Apply` payload `GltfChangeNodeNamePayload` and the phase wire `ChangeNodeNameMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireInteger, gltfWireNullable, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeNodeNamePayload {
  node: number;
  value: string | null;
}

export type ChangeNodeNameMutation = GltfApplyPhase<GltfChangeNodeNamePayload>;

export const parseGltfChangeNodeNamePayload = gltfWireObject<GltfChangeNodeNamePayload>({ node: gltfWireRequired(gltfWireInteger(4294967295)), value: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseChangeNodeNameMutation = gltfWireApplyPhase(parseGltfChangeNodeNamePayload);
