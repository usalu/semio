/** 🫳️ Plans a merge within one sibling stack, without changing document ownership. */
export type MergeLayer={id:string;kind:string;visible?:boolean;locked?:boolean;blendMode?:string;children?:MergeLayer[]};
export function mergeDownPlan(layers:MergeLayer[],layerId:string):{parentId:string|null;index:number;layers:MergeLayer[]} {
  if(!layerId.trim()||[...layerId].length>128)throw new Error("raster.merge-target-invalid");
  const visit=(siblings:MergeLayer[],parentId:string|null,depth:number,inherited:boolean):ReturnType<typeof mergeDownPlan>|undefined=>{
    if(depth>32)throw new Error("raster.merge-depth");
    const index=siblings.findIndex(layer=>layer.id===layerId);
    if(index!==-1){
      if(index===0)throw new Error("raster.merge-no-lower-layer");
      const pair=siblings.slice(index-1,index+1);
      if(pair.some(layer=>layer.visible===false||(layer.blendMode??"normal")!=="normal"||!["pixel","group"].includes(layer.kind)))throw new Error("raster.merge-backdrop-dependent");
      const protectedTree=(layer:MergeLayer):boolean=>layer.locked===true||(layer.children??[]).some(protectedTree);
      if(inherited||pair.some(protectedTree))throw new Error("raster-layer-locked");
      return {parentId,index:index-1,layers:pair};
    }
    for(const layer of siblings)if(layer.kind==="group"){
      const result=visit(layer.children??[],layer.id,depth+1,inherited||layer.locked===true);if(result)return result;
    }
  };
  const result=visit(layers,null,0,false);if(!result)throw new Error("raster.merge-target-missing");return result;
}
