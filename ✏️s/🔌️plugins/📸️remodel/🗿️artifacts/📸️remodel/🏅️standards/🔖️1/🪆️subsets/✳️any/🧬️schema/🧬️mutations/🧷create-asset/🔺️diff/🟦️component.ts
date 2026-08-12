/** 🔺 create-asset diff — populates `RemodelDiff.assets` with the full post-upsert asset map. */
export interface CreateAssetDiff {
  assets: Record<string, { mime: string; data: string; width: number; height: number }>;
}
