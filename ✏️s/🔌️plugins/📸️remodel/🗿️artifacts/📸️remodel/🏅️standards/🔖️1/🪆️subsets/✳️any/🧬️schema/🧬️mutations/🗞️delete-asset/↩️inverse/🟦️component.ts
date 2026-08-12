/** ↩️ delete-asset inverse — a `create-asset` restoring the captured BASE value. */
export interface DeleteAssetInverse {
  key: string;
  asset: { mime: string; data: string; width: number; height: number };
}
