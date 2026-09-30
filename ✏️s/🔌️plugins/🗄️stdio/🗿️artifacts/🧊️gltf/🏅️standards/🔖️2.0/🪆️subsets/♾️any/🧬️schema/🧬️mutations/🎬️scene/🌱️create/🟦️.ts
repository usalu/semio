/** 🌱️ `create-scene` wire twin: the flat `Apply` payload `GltfCreateScenePayload` and the phase wire `CreateSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireInteger, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateScenePayload {
  position: number;
}

export type CreateSceneMutation = GltfPhase<GltfCreateScenePayload, GltfDiff>;

export const parseGltfCreateScenePayload = gltfWireObject<GltfCreateScenePayload>({ position: gltfWireRequired(gltfWireInteger(4294967295)) });
export const parseCreateSceneMutation = gltfWirePhase(parseGltfCreateScenePayload, parseGltfDiff);
