/** 🏷️ `change-scene-name` wire twin: the flat `Apply` payload `GltfChangeSceneNamePayload` and the phase wire `ChangeSceneNameMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireNullable, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeSceneNamePayload {
  scene: number;
  value: string | null;
}

export type ChangeSceneNameMutation = GltfPhase<GltfChangeSceneNamePayload, GltfDiff>;

export const parseGltfChangeSceneNamePayload = gltfWireObject<GltfChangeSceneNamePayload>({ scene: gltfWireRequired(gltfWireIndex), value: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseChangeSceneNameMutation = gltfWirePhase(parseGltfChangeSceneNamePayload, parseGltfDiff);
