/** 🔁 replace-dense mutation payload — whole-value swap of `ReconstructionResults.dense`. */
export interface ReplaceDense {
  dense?: { positions: string; colors?: string; confidence?: string; classification?: string };
}
