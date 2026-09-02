/** 🧬️ Draw artifact schema — every field with its state class. */

export interface DrawArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  layers: DrawLayerNode[];
  /** @state artifact */
  assets: Record<string, DrawImageAsset>;
  /** @state artifact */
  artboard?: DrawArtboard;
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
