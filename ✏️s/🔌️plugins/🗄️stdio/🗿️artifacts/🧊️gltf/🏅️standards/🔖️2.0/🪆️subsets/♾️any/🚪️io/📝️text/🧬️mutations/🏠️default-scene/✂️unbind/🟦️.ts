/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfUnbindDefaultScenePayload,UnbindDefaultSceneMutation} from "../../../../../🧬️schema/🧬️mutations/🏠️default-scene/✂️unbind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🏠️default-scene/✂️unbind/🟦️.ts";
/** ✂️ `unbind-default-scene` wire twin: the flat `Apply` payload `GltfUnbindDefaultScenePayload` and the phase wire `UnbindDefaultSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireObject } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfUnbindDefaultScenePayload = gltfWireObject<GltfUnbindDefaultScenePayload>({});
export const parseUnbindDefaultSceneMutation = gltfWireApplyPhase(parseGltfUnbindDefaultScenePayload);
