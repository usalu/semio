/** 🔗️ `bind-default-scene` wire twin: the flat `Apply` payload `GltfBindDefaultScenePayload` and the phase wire `BindDefaultSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindDefaultScenePayload {
  scene: bigint;
}

export type BindDefaultSceneMutation = GltfApplyPhase<GltfBindDefaultScenePayload>;

export const parseGltfBindDefaultScenePayload = gltfWireObject<GltfBindDefaultScenePayload>({ scene: gltfWireRequired(gltfWireIndex) });
export const parseBindDefaultSceneMutation = gltfWireApplyPhase(parseGltfBindDefaultScenePayload);
