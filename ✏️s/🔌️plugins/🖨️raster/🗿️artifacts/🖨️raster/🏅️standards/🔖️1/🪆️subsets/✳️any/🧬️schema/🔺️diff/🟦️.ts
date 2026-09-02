/** 🧬️ Raster diff schema — sparse field delta over the artifact. */

export interface RasterDiff {
  /** @state artifact */
  artifact?: RasterArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  layers?: RasterLayersDelta;
  /** @state artifact */
  assets?: RasterAssetsDelta;
  /** @state presence */
  selectedIds?: RasterStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  brushSize?: number;
  /** @state config */
  brushOpacity?: number;
  /** @state config */
  compositeViewport?: RasterViewportSize | null;
  /** @state config */
  cameraX?: number;
  /** @state config */
  cameraY?: number;
  /** @state config */
  cameraZoom?: number;
  /** @state config */
  locale?: string;
  /** @state artifact */
  hoveredId?: string | null;
}

export interface RasterArtifact { [key: string]: unknown; }

export interface RasterAssetsDelta {
  entries: Record<string, RasterImageAsset | null>;
}

export interface RasterImageAsset {
  mime: string;
  data: string;
}

export interface RasterStringList {
  values: string[];
}

export interface RasterLayersDelta {
  added: RasterLayerInsertion[];
  removed: string[];
  patched: RasterLayerPatchEntry[];
  moved: RasterLayerMove[];
}

export interface RasterLayerInsertion {
  parentId?: string;
  index: number;
  layer: RasterLayerNode;
}

export interface RasterLayerMove {
  id: string;
  parentId?: string;
  index: number;
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

export interface RasterLayerNode { [key: string]: unknown; }

export interface RasterViewportSize {
  width: number;
  height: number;
}
