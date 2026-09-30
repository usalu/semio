/** 🏷️ `change-node-name` wire twin: the flat `Apply` payload `GltfChangeNodeNamePayload` and the phase wire `ChangeNodeNameMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireInteger, gltfWireNullable, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfPhase, gltfWirePhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeNodeNameRestore {
  node: number;
  before: string | null;
  after: string | null;
}

export interface GltfChangeNodeNamePayload {
  node: number;
  value: string | null;
}

export type ChangeNodeNameMutation = GltfPhase<GltfChangeNodeNamePayload, GltfChangeNodeNameRestore>;

export const parseGltfChangeNodeNameRestore = gltfWireObject<GltfChangeNodeNameRestore>({ node: gltfWireRequired(gltfWireInteger(4294967295)), before: gltfWireRequired(gltfWireNullable(gltfWireString)), after: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseGltfChangeNodeNamePayload = gltfWireObject<GltfChangeNodeNamePayload>({ node: gltfWireRequired(gltfWireInteger(4294967295)), value: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseChangeNodeNameMutation = gltfWirePhase(parseGltfChangeNodeNamePayload, parseGltfChangeNodeNameRestore);
