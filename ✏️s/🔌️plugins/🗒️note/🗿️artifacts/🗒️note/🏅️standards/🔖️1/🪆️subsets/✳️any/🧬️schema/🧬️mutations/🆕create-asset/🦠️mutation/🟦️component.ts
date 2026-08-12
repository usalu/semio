/** 🆕 `create-asset` mutation payload. */
export interface CreateAsset {
  key: string;
  asset: { mime: string; data: string; width?: number | null; height?: number | null };
}
