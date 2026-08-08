/** 🧬️ Draw diff schema — sparse field delta. */

export interface DrawDiff {
  /** @state persistent */
  artifact?: DrawArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  id?: string;
  /** @state persistent */
  title?: string | null;
  /** @state persistent */
  layers?: DrawLayersDelta;
  /** @state persistent */
  assets?: DrawAssetsDelta;
  /** @state persistent */
  artboard?: DrawArtboard | null;
  /** @state shared-ui */
  selectedIds?: DrawStringList;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state local-ui */
  engagementInput?: string;
  /** @state local-ui */
  cameraX?: number;
  /** @state local-ui */
  cameraY?: number;
  /** @state local-ui */
  cameraZoom?: number;
  /** @state local-ui */
  locale?: string;
  /** @state preview */
  hoveredId?: string | null;
}

export interface DrawArtifact {
  schema: string;
  id: string;
  title?: string;
  layers: DrawLayerNode[];
  assets: Record<string, DrawImageAsset>;
  artboard?: DrawArtboard;
  selectedIds: string[];
  activeUtilityId: string;
  engagementInput: string;
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
  locale: string;
  hoveredId?: string;
}

export interface DrawAssetsDelta {
  entries: Record<string, DrawImageAsset | null>;
}

export interface DrawStringList {
  values: string[];
}

export interface DrawLayersDelta {
  added: DrawLayerNode[];
  removed: string[];
  patched: DrawLayerPatchEntry[];
  reordered?: string[];
}

export interface DrawLayerPatchEntry {
  id: string;
  patch: DrawLayerPatch;
}

export interface DrawLayerPatch {
  visible?: boolean;
  locked?: boolean;
  name?: string;
  opacity?: number;
  blendMode?: string;
  transformJson?: string;
  fillJson?: string;
  strokeJson?: string;
  booleanOperation?: string;
  traceParamsJson?: string;
  layerJson?: string;
}

export interface DrawLayerNode {
  kind: string;
  [key: string]: unknown;
}

export interface DrawImageAsset {
  mime: string;
  data: string;
  width?: number;
  height?: number;
}

export interface DrawArtboard {
  width: number;
  height: number;
}
