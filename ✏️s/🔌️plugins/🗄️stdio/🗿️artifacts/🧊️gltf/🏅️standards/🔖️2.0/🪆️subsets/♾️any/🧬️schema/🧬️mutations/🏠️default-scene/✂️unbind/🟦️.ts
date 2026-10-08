/** ✂️ `unbind-default-scene` wire twin: the flat `Apply` payload `GltfUnbindDefaultScenePayload` and the phase wire `UnbindDefaultSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireObject } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export type GltfUnbindDefaultScenePayload = Record<string, never>;

export type UnbindDefaultSceneMutation = GltfApplyPhase<GltfUnbindDefaultScenePayload>;

export const parseGltfUnbindDefaultScenePayload = gltfWireObject<GltfUnbindDefaultScenePayload>({});
export const parseUnbindDefaultSceneMutation = gltfWireApplyPhase(parseGltfUnbindDefaultScenePayload);
