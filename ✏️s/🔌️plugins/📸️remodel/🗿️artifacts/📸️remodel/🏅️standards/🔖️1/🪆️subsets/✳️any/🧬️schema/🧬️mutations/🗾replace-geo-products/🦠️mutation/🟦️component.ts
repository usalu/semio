/** 🔁 replace-geo-products mutation payload — whole-value swap of `ReconstructionResults.geo`. */
export interface ReplaceGeoProducts {
  geo?: { dsmAssetId?: string; dtmAssetId?: string; orthoAssetId?: string };
}
