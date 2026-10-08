/** 🌱️ `create-scene` wire twin: the flat `Apply` payload `GltfCreateScenePayload` and the phase wire `CreateSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireInteger, gltfWireObject, gltfWireRequired, gltfWireOptional, type GltfScene, parseGltfScene } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateScenePayload {
  position: number;
  scene?: GltfScene;
}

export type CreateSceneMutation = GltfApplyPhase<GltfCreateScenePayload>;

export const parseGltfCreateScenePayload = gltfWireObject<GltfCreateScenePayload>({ position: gltfWireRequired(gltfWireInteger(4294967295)), scene: gltfWireOptional(parseGltfScene) });
export const parseCreateSceneMutation = gltfWireApplyPhase(parseGltfCreateScenePayload);
