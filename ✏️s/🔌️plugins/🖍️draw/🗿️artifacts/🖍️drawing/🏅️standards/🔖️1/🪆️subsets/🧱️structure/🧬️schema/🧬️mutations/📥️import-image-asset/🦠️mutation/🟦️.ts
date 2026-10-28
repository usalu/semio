import {DrawingAssetsOwner,type DrawingAssets,type DrawingImageAsset,type DrawingLayerNode} from "../../../../../✳️any/🧬️schema/🟦️.ts";
/** 📥️ Image admission owns one validated asset and its sparse inverse. */
export interface ImportImageAsset {readonly assetId:string;readonly asset:DrawingImageAsset}
export interface AssetDocument {readonly assets:DrawingAssets;readonly layers:readonly DrawingLayerNode[]}
export function importImageAssetDiff(snapshot:AssetDocument,mutation:ImportImageAsset):{entries:Record<string,DrawingImageAsset>} {
  const {assetId,asset}=mutation;
  if(!assetId||new TextEncoder().encode(assetId).length>4096||Object.hasOwn(snapshot.assets,assetId)||!Number.isInteger(asset.width)||!Number.isInteger(asset.height)||asset.width<=0||asset.height<=0||asset.width*asset.height!==asset.samples.length||asset.samples.some(sample=>sample.length!==4||sample.some(value=>!Number.isInteger(value)||value<0||value>255)))throw new Error("Invalid or occupied drawing image asset / Ungültige oder belegte Bildressource");
  return {entries:{[assetId]:structuredClone(asset)}};
}
export function foldImageAssetDelta<T extends AssetDocument>(snapshot:T,delta:{readonly entries:Readonly<Record<string,DrawingImageAsset|null>>}):T {
  const changed=new Map(Object.entries(delta.entries));function* entries(){for(const[id,asset]of DrawingAssetsOwner.entries(snapshot.assets)){const next=changed.get(id);if(next===undefined)yield[id,asset]as const;else if(next!==null)yield[id,structuredClone(next)]as const;changed.delete(id);}for(const[id,asset]of changed)if(asset!==null)yield[id,structuredClone(asset)]as const;}const assets=DrawingAssetsOwner.fromEntriesCold(entries());return {...snapshot,assets};
}
