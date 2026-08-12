/** 🔺️ diff fragment for `UnflattenNode`. */
export interface UnflattenNodeDiff {
  layers?: { modified: { index: number; diff: unknown }[] };
}
