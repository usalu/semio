/** 🔗️ `bind-default-scene` wire twin: the flat `Apply` payload `GltfBindDefaultScenePayload` and the phase wire `BindDefaultSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindDefaultScenePayload {
  scene: bigint;
}

export type BindDefaultSceneMutation = GltfPhase<GltfBindDefaultScenePayload, GltfDiff>;

export const parseGltfBindDefaultScenePayload = gltfWireObject<GltfBindDefaultScenePayload>({ scene: gltfWireRequired(gltfWireIndex) });
export const parseBindDefaultSceneMutation = gltfWirePhase(parseGltfBindDefaultScenePayload, parseGltfDiff);
