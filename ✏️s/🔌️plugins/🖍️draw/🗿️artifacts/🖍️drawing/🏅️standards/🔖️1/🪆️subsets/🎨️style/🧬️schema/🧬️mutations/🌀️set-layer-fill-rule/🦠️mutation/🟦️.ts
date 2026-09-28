/** 🌀️ An immutable semantic fill-rule change on one identified layer. */
import {parseFillRule,type FillRule} from "../../../../../✳️any/🧬️schema/🎨️fill/🌀️rule/🟦️.ts";
export interface SetLayerFillRule {readonly layerId:string;readonly fillRule:FillRule}
interface Layer {readonly id:string;readonly kind:string;readonly attributes?:Record<string,unknown>;readonly children?:readonly Layer[]}
interface Document {readonly layers:readonly Layer[]}
export function applyFillRuleEdit<T extends Document>(snapshot:T,mutation:SetLayerFillRule):T {
  const fillRule=parseFillRule(mutation.fillRule);let found=false;
  const visit=(layer:Layer):Layer=>{
    if(layer.id===mutation.layerId){found=true;return {...layer,attributes:{...layer.attributes,fillRule}};}
    return layer.kind==="group" && layer.children ? {...layer,children:layer.children.map(visit)} : layer;
  };
  const layers=snapshot.layers.map(visit);
  if(!found)throw new Error("Fill-rule target is missing");
  return {...snapshot,layers};
}
