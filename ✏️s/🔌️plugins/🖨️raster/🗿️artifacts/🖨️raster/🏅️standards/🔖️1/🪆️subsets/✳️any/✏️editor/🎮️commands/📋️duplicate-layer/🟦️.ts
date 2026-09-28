/** 📋️ Duplicate into the source sibling stack without changing its inherited coordinate frame. */
export type DuplicateLayerNode= {id:string;kind:string;locked?:boolean;children?:DuplicateLayerNode[]};
export function duplicateLayerPlan(layers:DuplicateLayerNode[],layerId:string):{parentId:string|null;index:number} {
  const visit=(siblings:DuplicateLayerNode[],parentId:string|null,inherited:boolean):ReturnType<typeof duplicateLayerPlan>|undefined=>{
    for(const [index,layer] of siblings.entries()){
      if(layer.id===layerId){if(inherited)throw new Error("raster-layer-locked");return {parentId,index:index+1};}
      if(layer.kind==="group"&&layer.children){const found=visit(layer.children,layer.id,inherited||layer.locked===true);if(found)return found;}
    }
  };
  const plan=visit(layers,null,false);if(!plan)throw new Error("raster-layer-not-found");return plan;
}
