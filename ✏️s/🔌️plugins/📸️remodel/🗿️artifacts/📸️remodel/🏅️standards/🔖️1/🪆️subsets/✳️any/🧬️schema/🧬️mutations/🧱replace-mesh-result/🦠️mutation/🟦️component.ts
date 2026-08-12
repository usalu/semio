/** 🧱 replace-mesh-result mutation payload — whole-value swap of the reconstructed mesh. */
export interface ReplaceMeshResult {
  mesh: {
    source: "placeholder" | "reconstructed" | "imported";
    textureAssetId?: string;
    geometry: Record<string, unknown>;
    watertight?: Record<string, unknown>;
  };
}
