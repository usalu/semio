/** 🔺️ StlDiff — handcrafted sparse diff. `solidName` plus an index-keyed `triangles` triple. */

export interface StlTriangle {
  normal: [number, number, number];
  vertices: [[number, number, number], [number, number, number], [number, number, number]];
}

/** 🔺️ Sparse per-field patch for one `StlTriangle`; both fields whole-value replace. */
export interface StlTriangleDiff {
  normal?: [number, number, number];
  vertices?: [[number, number, number], [number, number, number], [number, number, number]];
}

/** 📦️ One `triangles.modified[]` entity — `index` is the triangle's position in BASE. */
export interface StlTriangleModified {
  index: number;
  diff: StlTriangleDiff;
}

/** 📦️ One `triangles.added[]` entity — `index` is the triangle's position in the FINAL sequence. */
export interface StlTriangleAdded {
  index: number;
  triangle: StlTriangle;
}

export interface StlTrianglesDiff {
  removed: number[];
  modified: StlTriangleModified[];
  added: StlTriangleAdded[];
}

/** 🔺️ Diff for `stdio.stl`. `schema` is an identity field and never appears here. */
export interface StlDiff {
  solidName?: string;
  triangles?: StlTrianglesDiff;
}
