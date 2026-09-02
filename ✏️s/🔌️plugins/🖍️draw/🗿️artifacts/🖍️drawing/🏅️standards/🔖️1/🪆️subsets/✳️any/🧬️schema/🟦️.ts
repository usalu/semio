/** 🧬️ Drawing artifact schema — every field with its state class. */

export interface DrawingArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  layers: DrawingLayerNode[];
  /** @state artifact */
  assets: Record<string, DrawingImageAsset>;
  /** @state artifact */
  artboard?: DrawingArtboard;
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  activeUtilityId: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  cameraX: number;
  /** @state config */
  cameraY: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
  locale: string;
  /** @state artifact */
  hoveredId?: string;
}

export interface DrawingLayerNode {
  kind: string;
  [key: string]: unknown;
}

export interface DrawingImageAsset {
  mime: string;
  data: string;
  width?: number;
  height?: number;
}

export interface DrawingArtboard {
  width: number;
  height: number;
}
