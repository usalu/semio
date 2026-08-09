/** 🧬️ Raster snapshot schema — persistent fields only. */

export interface RasterSnapshot {
  schema: string;
  id: string;
  title?: string;
  layers: RasterLayerNode[];
  assets: Record<string, RasterImageAsset>;
}

export interface RasterLayerNode {
  kind: string;
  [key: string]: unknown;
}

export interface RasterImageAsset {
  mime: string;
  data: string;
}
