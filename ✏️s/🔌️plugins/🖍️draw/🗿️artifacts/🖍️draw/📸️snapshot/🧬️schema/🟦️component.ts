/** 🧬️ Draw snapshot schema — persistent fields only. */

export interface DrawSnapshot {
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
