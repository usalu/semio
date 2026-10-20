import type {DrawingLayerNode,DrawingArtifact} from "../../../../../✳️any/🧬️schema/🟦️.ts";
import {setShapeCoordinate,type ShapeCoordinateField} from "../../../../../✳️any/🧬️schema/🔷️shape/✏️coordinates/🟦️.ts";
/** 🔷️ Sparse authored coordinates preserve shape identity and appearance. */
export interface SetShapeCoordinate {readonly layerId:string;readonly field:ShapeCoordinateField;readonly index?:number|null;readonly value:number}
export function applyShapeCoordinate(snapshot:DrawingArtifact,mutation:SetShapeCoordinate):DrawingArtifact {
 let found=false;
 const visit=(layer:DrawingLayerNode):DrawingLayerNode=>{
  if(layer.id===mutation.layerId){if(layer.kind!=="shape")throw new Error("Shape target has another kind");found=true;return setShapeCoordinate(layer,mutation.field,mutation.index??undefined,mutation.value);}
  return layer.kind==="group"?{...layer,children:layer.children.map(visit)}:layer;
 };
 const layers=snapshot.layers.map(visit);if(!found)throw new Error("Shape target is missing");return {...snapshot,layers};
}
