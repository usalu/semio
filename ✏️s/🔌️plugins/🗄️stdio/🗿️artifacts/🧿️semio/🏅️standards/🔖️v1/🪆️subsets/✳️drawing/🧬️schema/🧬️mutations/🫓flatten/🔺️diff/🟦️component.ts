/** 🔺️ diff fragment for `FlattenNode`. */
export interface FlattenNodeDiff {
  layers?: { modified: { index: number; diff: unknown }[] };
}
