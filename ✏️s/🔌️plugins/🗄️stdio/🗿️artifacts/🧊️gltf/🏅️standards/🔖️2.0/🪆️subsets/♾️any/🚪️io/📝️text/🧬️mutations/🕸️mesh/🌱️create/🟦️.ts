/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateMeshPayload,CreateMeshMutation} from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🕸️mesh/🌱️create/🟦️.ts";
/** 🌱️ `create-mesh` wire twin: the flat `Apply` payload `GltfCreateMeshPayload` and the phase wire `CreateMeshMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, parseGltfMesh } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateMeshPayload = gltfWireObject<GltfCreateMeshPayload>({ position: gltfWireRequired(gltfWireIndex), mesh: gltfWireOptional(parseGltfMesh) });
export const parseCreateMeshMutation = gltfWireApplyPhase(parseGltfCreateMeshPayload);
