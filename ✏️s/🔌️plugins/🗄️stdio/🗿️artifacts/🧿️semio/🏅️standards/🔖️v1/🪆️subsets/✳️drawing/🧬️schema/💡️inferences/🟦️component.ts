/** 💡️ SemioDrawing inference schema — flattenedScene (world transform + resolved style) per
 * scene-graph entity, keyed by the same `"<layer>:<p0>.<p1>..."` structural address every
 * mutation triad in this facet uses in place of a stable node id. */
import type { DrawStyle, Transform } from "../📸️snapshot/🟦️component";

export interface FlattenedNode {
  worldTransform: Transform;
  resolvedStyle?: DrawStyle;
}

export interface SemioDrawingInference {
  /** @state inferred */
  flattenedScene: Record<string, FlattenedNode>;
}
