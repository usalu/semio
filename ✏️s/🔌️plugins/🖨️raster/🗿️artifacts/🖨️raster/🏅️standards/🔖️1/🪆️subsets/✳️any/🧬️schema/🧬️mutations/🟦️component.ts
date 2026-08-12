/** 🖼️ Minimal local mirror of `RasterLayerNode` — the snapshot facet's own `🟦️component.ts` is a
 *  stale generic scaffold (out of this facet's scope), so `create-layer`'s payload type is defined
 *  inline here instead of importing it. */
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

export type RasterLayerNode =
  | { kind: 'pixel'; id: string; name: string; visible: boolean; opacity: number; blend: string; transform: RasterTransform; mask?: RasterLayerMask; width?: number; height?: number; image?: string }
  | { kind: 'group'; id: string; name: string; visible: boolean; opacity: number; blend: string; transform: RasterTransform; mask?: RasterLayerMask; children: RasterLayerNode[] }
  | { kind: 'adjustment'; id: string; name: string; visible: boolean; opacity: number; blend: string; transform: RasterTransform; adjustmentKind: string; params: Record<string, unknown> };

export interface RasterImageAsset {
  mime: string;
  data: string;
}

/** 🧬️ RasterMutation union — closed semantic mutation vocabulary for the raster document. */
export type RasterMutation =
  | { mutation: 'createLayer'; parentId?: string; index: number; layer: RasterLayerNode }
  | { mutation: 'deleteLayer'; layerId: string }
  | { mutation: 'reorderLayers'; layerId: string; parentId?: string; index: number }
  | { mutation: 'renameLayer'; layerId: string; newName: string }
  | { mutation: 'changeLayerVisible'; layerId: string; newVisible: boolean }
  | { mutation: 'changeLayerOpacity'; layerId: string; newOpacity: number }
  | { mutation: 'changeLayerBlendMode'; layerId: string; newBlendMode: string }
  | { mutation: 'moveLayer'; layerId: string; newX: number; newY: number }
  | { mutation: 'resizeLayer'; layerId: string; newWidth: number; newHeight: number }
  | { mutation: 'changeLayerAdjustmentKind'; layerId: string; newAdjustmentKind: string }
  | { mutation: 'addLayerAsset'; assetId: string; asset: RasterImageAsset }
  | { mutation: 'removeLayerAsset'; assetId: string };
