/** 🔺️ diff fragment for `DeleteNode` — a real cascade: `nodes` AND `edges` both change. */
export interface DeleteNodeDiff {
  nodes?: unknown[];
  edges?: unknown[];
}
