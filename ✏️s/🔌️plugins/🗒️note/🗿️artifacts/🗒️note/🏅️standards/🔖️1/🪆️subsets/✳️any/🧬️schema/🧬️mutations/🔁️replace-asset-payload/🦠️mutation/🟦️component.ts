/** 🔁 `replace-asset-payload` mutation payload. */
export interface ReplaceAssetPayload {
  key: string;
  newAsset: { mime: string; data: string; width?: number | null; height?: number | null };
}
