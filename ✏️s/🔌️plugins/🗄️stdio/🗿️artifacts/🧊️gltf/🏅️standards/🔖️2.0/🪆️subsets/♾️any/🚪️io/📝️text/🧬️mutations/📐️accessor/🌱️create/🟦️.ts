/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateAccessorPayload,CreateAccessorMutation} from "../../../../../🧬️schema/🧬️mutations/📐️accessor/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📐️accessor/🌱️create/🟦️.ts";
/** 🌱️ `create-accessor` wire twin: the flat `Apply` payload `GltfCreateAccessorPayload` and the phase wire `CreateAccessorMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfAccessorType, type GltfComponentType, gltfWireIndex, gltfWireObject, gltfWireRequired, parseGltfAccessorType, parseGltfComponentType, gltfWireOptional, parseGltfAccessor } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateAccessorPayload = gltfWireObject<GltfCreateAccessorPayload>({ position: gltfWireRequired(gltfWireIndex), componentType: gltfWireRequired(parseGltfComponentType), count: gltfWireRequired(gltfWireIndex), kind: gltfWireRequired(parseGltfAccessorType), accessor: gltfWireOptional(parseGltfAccessor) });
export const parseCreateAccessorMutation = gltfWireApplyPhase(parseGltfCreateAccessorPayload);
