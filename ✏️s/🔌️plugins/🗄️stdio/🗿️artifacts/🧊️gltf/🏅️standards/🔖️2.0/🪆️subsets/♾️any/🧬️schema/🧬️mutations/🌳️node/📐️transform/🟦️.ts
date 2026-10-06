import type {Binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 📐️ `change-node-transform` wire twin: the flat `Apply` payload `GltfTransformNodePayload` and the phase wire `ChangeNodeTransformMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireLiteral, gltfWireNullable, gltfWireNumber, gltfWireObject, gltfWireRequired, gltfWireTagged, gltfWireTuple } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export type GltfNodeTransform =
  | { kind: "matrix"; matrix: Binary64[] }
  | { kind: "trs"; translation: [Binary64, Binary64, Binary64] | null; rotation: [Binary64, Binary64, Binary64, Binary64] | null; scale: [Binary64, Binary64, Binary64] | null };

export interface GltfTransformNodePayload {
  node: bigint;
  transform: GltfNodeTransform;
}

export type ChangeNodeTransformMutation = GltfPhase<GltfTransformNodePayload, GltfDiff>;

export const parseGltfNodeTransform = gltfWireTagged<GltfNodeTransform, "kind">("kind", {
  matrix: gltfWireObject<Extract<GltfNodeTransform, { kind: "matrix" }>>({ kind: gltfWireRequired(gltfWireLiteral("matrix")), matrix: gltfWireRequired(gltfWireArray(gltfWireNumber, 16)) }),
  trs: gltfWireObject<Extract<GltfNodeTransform, { kind: "trs" }>>({ kind: gltfWireRequired(gltfWireLiteral("trs")), translation: gltfWireRequired(gltfWireNullable(gltfWireTuple(gltfWireNumber, gltfWireNumber, gltfWireNumber))), rotation: gltfWireRequired(gltfWireNullable(gltfWireTuple(gltfWireNumber, gltfWireNumber, gltfWireNumber, gltfWireNumber))), scale: gltfWireRequired(gltfWireNullable(gltfWireTuple(gltfWireNumber, gltfWireNumber, gltfWireNumber))) }),
});
export const parseGltfTransformNodePayload = gltfWireObject<GltfTransformNodePayload>({ node: gltfWireRequired(gltfWireIndex), transform: gltfWireRequired(parseGltfNodeTransform) });
export const parseChangeNodeTransformMutation = gltfWirePhase(parseGltfTransformNodePayload, parseGltfDiff);
