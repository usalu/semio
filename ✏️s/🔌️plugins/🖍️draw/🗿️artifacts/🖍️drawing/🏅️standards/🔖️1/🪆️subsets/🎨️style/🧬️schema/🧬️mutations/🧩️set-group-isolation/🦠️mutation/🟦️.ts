/** 🧩️ Immutable, group-only isolation with a compact pass-through default. */
export interface SetGroupIsolation {readonly layerId:string;readonly isolation:boolean}
interface Layer {readonly id:string;readonly kind:string;readonly isolation?:boolean;readonly children?:readonly Layer[]}
interface Document {readonly layers:readonly Layer[]}
export function applyGroupIsolationEdit<T extends Document>(snapshot:T,mutation:SetGroupIsolation):T {
  if(!mutation.layerId||typeof mutation.isolation!=="boolean")throw new Error("Invalid group isolation edit");
  let found=false;
  const visit=(layer:Layer):Layer=>{
    if(layer.id===mutation.layerId){
      if(layer.kind!=="group")throw new Error("Isolation needs a group");
      found=true;const {isolation:_,...rest}=layer;
      return mutation.isolation?{...rest,isolation:true}:rest;
    }
    return layer.kind==="group"&&layer.children?{...layer,children:layer.children.map(visit)}:layer;
  };
  const layers=snapshot.layers.map(visit);
  if(!found)throw new Error("Isolation target is missing");
  return {...snapshot,layers};
}
