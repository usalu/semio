/** 🕳️ Field admission compares only the addressed facet before publishing history. */
import type {DrawingArtifact,DrawingLayerNode} from "../../../🧬️schema/🟦️.ts";
import type {DrawingMutation} from "../../../🧬️schema/🧬️mutations/🟦️.ts";
import {shapeCoordinate} from "../../../🧬️schema/🔷️shape/✏️coordinates/🟦️.ts";
import {binary64Value} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";

function equivalent(current:unknown,proposed:unknown):boolean {
 if(current===proposed||current==null&&proposed==null)return true;
 if(typeof current!=="object"||current===null)return false;
 if("bits" in current&&typeof current.bits==="bigint")return binary64Value({bits:current.bits})===proposed;
 if(Array.isArray(current))return Array.isArray(proposed)&&current.length===proposed.length&&current.every((value,index)=>equivalent(value,proposed[index]));
 if(typeof proposed!=="object"||proposed===null||Array.isArray(proposed))return false;
 const before=current as Record<string,unknown>,after=proposed as Record<string,unknown>;
 const keys=Object.keys(before).filter(key=>before[key]!==undefined),other=Object.keys(after).filter(key=>after[key]!==undefined);
 return keys.length===other.length&&keys.every(key=>Object.hasOwn(after,key)&&equivalent(before[key],after[key]));
}
/** 🎛️ Requires an admitted field mutation; unsupported structural verbs never masquerade as no-ops. */
export function hasLayerFieldChange(document:DrawingArtifact,mutation:DrawingMutation):boolean {
 if(!("layerId" in mutation))throw Error("Expected a layer field mutation");
 const pending:DrawingLayerNode[]=[...document.layers];let layer:DrawingLayerNode|undefined;
 while(pending.length){const candidate=pending.pop()!;if(candidate.id===mutation.layerId){layer=candidate;break;}if(candidate.kind==="group")pending.push(...candidate.children);}
 if(!layer)throw Error("The layer field target is missing");
 switch(mutation.mutation) {
  case "renameLayer":return layer.name!==mutation.newName;
  case "setLayerVisible":return layer.visible!==mutation.visible;
  case "setLayerLocked":return layer.locked!==mutation.locked;
  case "setLayerOpacity":return !equivalent(layer.opacity,mutation.opacity);
  case "setLayerBlendMode":return layer.blendMode!==mutation.blendMode;
  case "updateLayerTransform":return !equivalent(layer.transform,mutation.transform);
  case "replaceLayerFill":return !equivalent(layer.attributes.fill,mutation.fill);
  case "replaceLayerStroke":return !equivalent(layer.attributes.stroke,mutation.stroke);
  case "setLayerFillRule":return layer.attributes.fillRule!==mutation.fillRule;
  case "setGroupIsolation":if(layer.kind==="group")return layer.isolation!==mutation.isolation;break;
  case "setLayerBooleanOperation":if(layer.kind==="boolean")return layer.operation!==mutation.booleanOperation;break;
  case "updateLayerTraceParams":if(layer.kind==="trace")return !equivalent(layer.params,mutation.params);break;
  case "updateText":if(layer.kind==="text")return layer.content!==mutation.content||!equivalent(layer.size,mutation.size);break;
  case "updateImage":if(layer.kind==="image")return layer.imageKey!==mutation.imageKey||!equivalent(layer.width,mutation.width)||!equivalent(layer.height,mutation.height);break;
  case "setShapeCoordinate":if(layer.kind==="shape")return shapeCoordinate(layer,mutation.field,mutation.index??undefined)!==mutation.value;break;
 }
 throw Error("Expected a matching layer field mutation");
}

/** 🛂️ Resolves every bulk target before publication, preserving document order and unique identity. */
export function selectFieldTargets(document:DrawingArtifact,ids:readonly string[]):DrawingLayerNode[] {
 const remaining=new Set(ids),selected:DrawingLayerNode[]=[];
 const visit=(layers:readonly DrawingLayerNode[]):void=>{
  for(const layer of layers) {
   const row="drawing-play-layers."+layer.kind+"."+layer.id;
   if(remaining.delete(layer.id)||remaining.has(row)){selected.push(layer);remaining.delete(row);}
   if(layer.kind==="group")visit(layer.children);
  }
 };
 visit(document.layers);
 if(remaining.size)throw Error("A selected layer field target is missing");
 return selected;
}
