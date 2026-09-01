/** 🧬️ Raster snapshot schema — artifact-lane fields only. */

export interface RasterSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  layers: RasterLayerNode[];
  /** @state artifact */
  assets: Record<string, RasterAssetChild>;
}

export type RasterLayerNode = RasterLayerPixel | RasterLayerGroup | RasterLayerAdjustment;

export interface RasterLayerPixel {
  kind: "pixel";
  id: string;
  name: string;
  visible: boolean;
  opacity: number;
  blendMode: string;
  transform: RasterTransform;
  mask?: RasterLayerMask;
  width?: number;
  height?: number;
  imageKey?: string;
}

export interface RasterLayerGroup {
  kind: "group";
  id: string;
  name: string;
  visible: boolean;
  opacity: number;
  blendMode: string;
  transform: RasterTransform;
  mask?: RasterLayerMask;
  children: RasterLayerNode[];
}

export interface RasterLayerAdjustment {
  kind: "adjustment";
  id: string;
  name: string;
  visible: boolean;
  opacity: number;
  blendMode: string;
  transform: RasterTransform;
  adjustmentKind: string;
  params: Record<string, RasterDslValue>;
}

export type RasterDslValue = Record<string, unknown>;

export interface RasterTransform {
  x: number;
  y: number;
  scaleX: number;
  scaleY: number;
  rotation: number;
}

export interface RasterLayerMask {
  enabled: boolean;
  linked: boolean;
  invert: boolean;
  width?: number;
  height?: number;
}

export interface RasterAssetChild {
  childId: string;
  target: string;
}
