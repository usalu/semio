/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveScenePayload,MoveSceneMutation} from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🚚️move/🟦️.ts";
/** 🚚️ `move-scene` wire twin: the flat `Apply` payload `GltfMoveScenePayload` and the phase wire `MoveSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveScenePayload = gltfWireObject<GltfMoveScenePayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveSceneMutation = gltfWireApplyPhase(parseGltfMoveScenePayload);
