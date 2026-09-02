/** 🧬️ Raster artifact schema — every field with its state class. */

export interface RasterArtifact {
  schema: string;
  id: string;
  title?: string;
  layers: RasterLayerNode[];
  assets: Record<string, RasterImageAsset>;
  selectedIds: string[];
  activeUtilityId: string;
  brushSize: number;
  brushOpacity: number;
  compositeViewport?: RasterViewportSize;
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
  locale: string;
  hoveredId?: string;
}

export interface RasterLayerNode {
  kind: string;
  [key: string]: unknown;
}

export interface RasterImageAsset {
  mime: string;
  data: string;
}

export interface RasterViewportSize {
  width: number;
  height: number;
}
