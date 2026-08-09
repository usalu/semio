/** 🧬️ Raster diff schema — sparse field delta. */

export interface RasterDiff {
  artifact?: RasterArtifact;
  schema?: string;
  id?: string;
  title?: string | null;
  layers?: RasterLayersDelta;
  assets?: RasterAssetsDelta;
  selectedIds?: RasterStringList;
  activeUtilityId?: string;
  brushSize?: number;
  brushOpacity?: number;
  compositeViewport?: RasterViewportSize | null;
  cameraX?: number;
  cameraY?: number;
  cameraZoom?: number;
  locale?: string;
  hoveredId?: string | null;
}

export interface RasterStringList {
  values: string[];
}

export interface RasterAssetsDelta {
  entries: Record<string, RasterImageAsset | null>;
}

export interface RasterLayersDelta {
  added?: RasterLayerNode[];
  removed?: string[];
  patched?: RasterLayerPatchEntry[];
  reordered?: string[];
}

export interface RasterLayerPatchEntry {
  id: string;
  patch: RasterLayerPatch;
}

export interface RasterLayerPatch {
  name?: string;
  visible?: boolean;
  opacity?: number;
  blendMode?: string;
  transformX?: number;
  transformY?: number;
  width?: number;
  height?: number;
  adjustmentKind?: string;
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
