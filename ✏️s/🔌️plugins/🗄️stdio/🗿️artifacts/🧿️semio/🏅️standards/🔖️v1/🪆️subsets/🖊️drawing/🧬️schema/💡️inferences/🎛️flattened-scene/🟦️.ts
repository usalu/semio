/** 🎛 `flattened-scene` — one named inference: world transform (composed down through nested
 * groups) + resolved style per scene-graph entity, keyed by its structural
 * `"<layer>:<p0>.<p1>..."` address. */
import type { DrawStyle, Transform } from "../../📸️snapshot/🟦️";

export interface FlattenedNode {
  worldTransform: Transform;
  resolvedStyle?: DrawStyle;
}
