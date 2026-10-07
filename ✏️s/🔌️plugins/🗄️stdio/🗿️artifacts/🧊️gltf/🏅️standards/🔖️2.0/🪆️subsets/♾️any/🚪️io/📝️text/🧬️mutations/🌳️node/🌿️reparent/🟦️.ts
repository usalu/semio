/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReparentNodePayload,MoveNodeParentMutation} from "../../../../../🧬️schema/🧬️mutations/🌳️node/🌿️reparent/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌳️node/🌿️reparent/🟦️.ts";
/** 🌿️ `move-node-parent` wire twin: the flat `Apply` payload `GltfReparentNodePayload` and the phase wire `MoveNodeParentMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReparentNodePayload = gltfWireObject<GltfReparentNodePayload>({ parent: gltfWireRequired(gltfWireIndex), child: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveNodeParentMutation = gltfWirePhase(parseGltfReparentNodePayload, parseGltfDiff);
