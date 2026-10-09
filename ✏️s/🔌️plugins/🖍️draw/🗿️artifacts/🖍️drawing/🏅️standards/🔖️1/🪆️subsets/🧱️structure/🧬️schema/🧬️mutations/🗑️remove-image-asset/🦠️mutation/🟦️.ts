import type {AssetDocument,ImportImageAsset} from "../../📥️import-image-asset/🦠️mutation/🟦️.ts";
/** 🗑️ Removal rejects live image and trace references and restores the exact prior asset. */
export interface RemoveImageAsset {readonly assetId:string}
export function removeImageAssetDiff(snapshot:AssetDocument,mutation:RemoveImageAsset):{entries:Record<string,null>} {
  if(!Object.hasOwn(snapshot.assets,mutation.assetId))throw new Error("Image asset no longer exists / Bildressource existiert nicht mehr");
  const pending=[...snapshot.layers];while(pending.length){const layer=pending.pop()!;if((layer.kind==="image"&&layer.imageKey===mutation.assetId)||(layer.kind==="trace"&&layer.sourceKey===mutation.assetId))throw new Error("Image asset remains referenced / Bildressource wird noch verwendet");if(layer.kind==="group")pending.push(...layer.children);}
  return {entries:{[mutation.assetId]:null}};
}
export function removeImageAssetInverse(snapshot:AssetDocument,mutation:RemoveImageAsset):ImportImageAsset {
  removeImageAssetDiff(snapshot,mutation);return {assetId:mutation.assetId,asset:structuredClone(snapshot.assets[mutation.assetId]!)};
}
