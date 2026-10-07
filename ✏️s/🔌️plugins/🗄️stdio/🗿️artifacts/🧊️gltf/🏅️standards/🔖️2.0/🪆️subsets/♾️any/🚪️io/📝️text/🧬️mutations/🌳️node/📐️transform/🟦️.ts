/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfNodeTransform,GltfTransformNodePayload,ChangeNodeTransformMutation} from "../../../../../🧬️schema/🧬️mutations/🌳️node/📐️transform/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌳️node/📐️transform/🟦️.ts";
import type {Binary64} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 📐️ `change-node-transform` wire twin: the flat `Apply` payload `GltfTransformNodePayload` and the phase wire `ChangeNodeTransformMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireLiteral, gltfWireNullable, gltfWireNumber, gltfWireObject, gltfWireRequired, gltfWireTagged, gltfWireTuple } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfNodeTransform = gltfWireTagged<GltfNodeTransform, "kind">("kind", {
  matrix: gltfWireObject<Extract<GltfNodeTransform, { kind: "matrix" }>>({ kind: gltfWireRequired(gltfWireLiteral("matrix")), matrix: gltfWireRequired(gltfWireArray(gltfWireNumber, 16)) }),
  trs: gltfWireObject<Extract<GltfNodeTransform, { kind: "trs" }>>({ kind: gltfWireRequired(gltfWireLiteral("trs")), translation: gltfWireRequired(gltfWireNullable(gltfWireTuple(gltfWireNumber, gltfWireNumber, gltfWireNumber))), rotation: gltfWireRequired(gltfWireNullable(gltfWireTuple(gltfWireNumber, gltfWireNumber, gltfWireNumber, gltfWireNumber))), scale: gltfWireRequired(gltfWireNullable(gltfWireTuple(gltfWireNumber, gltfWireNumber, gltfWireNumber))) }),
});
export const parseGltfTransformNodePayload = gltfWireObject<GltfTransformNodePayload>({ node: gltfWireRequired(gltfWireIndex), transform: gltfWireRequired(parseGltfNodeTransform) });
export const parseChangeNodeTransformMutation = gltfWirePhase(parseGltfTransformNodePayload, parseGltfDiff);
