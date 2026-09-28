/** 🧩️ Ungroup selected containers while preserving their children's parent-space geometry. */
import type {DrawingTransform} from "../../../../🧬️schema/🧬️mutations/🟦️.ts";
import {drawingTransformToMatrix,drawingMatrixToTransform} from "../../../../🧬️schema/🧮️geometry/↗️affine/🟦️.ts";
import {multiply} from "../../../../🧬️schema/🧮️geometry/🟦️.ts";

export interface UngroupLayer {
  id:string;kind:string;transform:DrawingTransform;visible:boolean;locked:boolean;opacity:number;blendMode:string;children?:UngroupLayer[];[key:string]:unknown;
}

export function ungroup(source:readonly UngroupLayer[],ids:readonly string[]):{layers:UngroupLayer[];selection:string[]} {
  const wanted=new Set(ids),groups:UngroupLayer[]=[];
  function inspect(layers:readonly UngroupLayer[],locked=false) {
    for(const layer of layers) {
      if(wanted.has(layer.id)) {
        if(layer.kind!=="group" || !layer.children) throw new Error("Select groups to ungroup");
        if(locked || layer.locked) throw new Error("Unlock selected groups before ungrouping");
        if(layer.isolation===true || layer.opacity!==1 || layer.blendMode!=="normal") throw new Error("Group compositing must be resolved before ungrouping");
        groups.push(layer);
      }
      if(layer.children) inspect(layer.children,locked||layer.locked);
    }
  }
  inspect(source);
  if(groups.length===0 || groups.length!==wanted.size) throw new Error("A selected group no longer exists");
  const selection:string[]=[],seen=new Set<string>();
  function select(layer:UngroupLayer) {
    if(wanted.has(layer.id)) for(const child of layer.children!) select(child);
    else if(!seen.has(layer.id)) {seen.add(layer.id);selection.push(layer.id);}
  }
  groups.forEach(select);
  const layers=structuredClone(source) as UngroupLayer[];
  function locate(nodes:UngroupLayer[],id:string):{siblings:UngroupLayer[];index:number}|null {
    for(let index=0;index<nodes.length;index++) {
      const layer=nodes[index]!;
      if(layer.id===id) return {siblings:nodes,index};
      const found=layer.children?locate(layer.children,id):null;
      if(found) return found;
    }
    return null;
  }
  for(const group of groups.toReversed()) {
    const location=locate(layers,group.id)!;
    const current=location.siblings[location.index]!,matrix=drawingTransformToMatrix(current.transform);
    for(const child of current.children!) {
      const combined=multiply(matrix,drawingTransformToMatrix(child.transform));
      if(!combined.every(Number.isFinite)) throw new Error("Ungrouping produced a nonfinite transform");
      child.transform=drawingMatrixToTransform(combined);
      if(!Object.values(child.transform).every(Number.isFinite)) throw new Error("Ungrouping produced a nonfinite transform");
      child.visible=current.visible&&child.visible;
    }
    location.siblings.splice(location.index,1,...current.children!);
  }
  return {layers,selection};
}
