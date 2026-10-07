/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfBindNodeMeshPayload,BindNodeMeshMutation} from "../../../../../🧬️schema/🧬️mutations/🏗️node-mesh/🔗️bind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🏗️node-mesh/🔗️bind/🟦️.ts";
/** 🔗️ `bind-node-mesh` wire twin: the flat `Apply` payload `GltfBindNodeMeshPayload` and the phase wire `BindNodeMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfBindNodeMeshPayload = gltfWireObject<GltfBindNodeMeshPayload>({ node: gltfWireRequired(gltfWireIndex), mesh: gltfWireRequired(gltfWireIndex) });
export const parseBindNodeMeshMutation = gltfWirePhase(parseGltfBindNodeMeshPayload, parseGltfDiff);
