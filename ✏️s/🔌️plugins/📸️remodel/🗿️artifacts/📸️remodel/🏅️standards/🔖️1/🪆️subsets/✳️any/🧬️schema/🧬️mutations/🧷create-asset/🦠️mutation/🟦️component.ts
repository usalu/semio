/** 🧷 create-asset mutation payload — upserts one key-addressed image asset. */
export interface CreateAsset {
  key: string;
  asset: { mime: string; data: string; width: number; height: number };
}
