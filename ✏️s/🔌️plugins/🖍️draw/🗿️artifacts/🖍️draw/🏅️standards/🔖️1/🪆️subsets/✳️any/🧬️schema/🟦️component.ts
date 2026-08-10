/** 🧬️ Draw artifact schema — every field with its state class. */

export interface DrawArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  id: string;
  /** @state persistent */
  title?: string;
  /** @state persistent */
  layers: DrawLayerNode[];
  /** @state persistent */
  assets: Record<string, DrawImageAsset>;
  /** @state persistent */
  artboard?: DrawArtboard;
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  cameraX: number;
  /** @state local-ui */
  cameraY: number;
  /** @state local-ui */
  cameraZoom: number;
  /** @state local-ui */
  locale: string;
  /** @state preview */
  hoveredId?: string;
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
