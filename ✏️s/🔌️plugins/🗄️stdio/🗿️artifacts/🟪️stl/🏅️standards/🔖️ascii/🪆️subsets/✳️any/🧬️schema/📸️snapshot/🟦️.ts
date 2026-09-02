/** 🧬️ StlSnapshot schema — complete per the ASCII STL spec
 * (https://en.wikipedia.org/wiki/STL_(file_format)): `solid`/`endsolid` header/trailer name plus
 * an ordered, self-contained (non-index-shared) triangle list. */

/** 🔺️ One STL facet. Normal is persisted exactly as read, never recomputed. */
export interface StlTriangle {
  normal: [number, number, number];
  vertices: [[number, number, number], [number, number, number], [number, number, number]];
}

/** 📸️ Persisted `stdio.stl` snapshot. */
export interface StlSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact — the `solid <name>`/`endsolid <name>` header/trailer token. */
  solidName: string;
  /** @state artifact */ triangles: StlTriangle[];
}
