/** 🎭️ Canonical document types shared by mutation payloads. */
import type {RasterLayerNode, RasterLayerMask, RasterTransform, RasterImageAsset} from "../🟦️.ts";
export type {RasterLayerNode, RasterLayerMask, RasterTransform, RasterImageAsset} from "../🟦️.ts";
export interface RasterPixelContent { imageKey: string | null; width: number | null; height: number | null }

/** 🧬️ RasterMutation union — closed semantic mutation vocabulary for the raster document. */
export type RasterMutation =
  | {mutation:'changeLayerAdjustmentParameter'; layerId:string; parameter:'brightness'|'contrast'; expected:number|null; value:number|null}
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
  | { mutation: 'removeLayerAsset'; assetId: string }
  | { mutation: 'changeLayerPixels'; layerId: string; expectedImageKey: string | null; content: RasterPixelContent; transform?: RasterTransform | null }
  | { mutation: 'changeLayerMask'; layerId: string; expected: RasterLayerMask | null; mask: RasterLayerMask | null };
