/** 🔺️ diff fragment for `DragNodes` — folded per-node `moveNode` diffs. */
export interface DragNodesDiff {
  layers?: { modified: { index: number; diff: unknown }[] };
}
