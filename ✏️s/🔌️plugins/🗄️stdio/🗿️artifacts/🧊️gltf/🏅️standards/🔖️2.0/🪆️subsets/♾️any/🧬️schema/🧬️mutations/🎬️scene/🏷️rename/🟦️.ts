/** 🏷️ `change-scene-name` wire twin: the flat `Apply` payload `GltfChangeSceneNamePayload` and the phase wire `ChangeSceneNameMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireNullable, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeSceneNamePayload {
  scene: bigint;
  value: string | null;
}

export type ChangeSceneNameMutation = GltfApplyPhase<GltfChangeSceneNamePayload>;

export const parseGltfChangeSceneNamePayload = gltfWireObject<GltfChangeSceneNamePayload>({ scene: gltfWireRequired(gltfWireIndex), value: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseChangeSceneNameMutation = gltfWireApplyPhase(parseGltfChangeSceneNamePayload);
