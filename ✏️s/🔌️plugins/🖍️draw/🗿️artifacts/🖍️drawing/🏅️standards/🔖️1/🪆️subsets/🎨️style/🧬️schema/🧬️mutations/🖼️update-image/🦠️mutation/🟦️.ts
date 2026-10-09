/** 🖼️ Authored image facet edits preserve identity and appearance. */
export interface UpdateImage {readonly layerId:string;readonly imageKey:string;readonly width:number;readonly height:number}
interface ImageLayer {readonly id:string;readonly kind:string;readonly children?:readonly ImageLayer[]}
interface ImageDocument {readonly layers:readonly ImageLayer[]}
export function applyImageEdit<T extends ImageDocument>(snapshot:T,mutation:UpdateImage):T {
  if(!mutation.layerId||typeof mutation.imageKey!=="string"||![mutation.width,mutation.height].every(value=>Number.isFinite(value)&&value>0))throw new Error("Invalid image edit");
  let found=false;
  const visit=(layer:ImageLayer):ImageLayer=>{
    if(layer.id===mutation.layerId){if(layer.kind!=="image")throw new Error("Image target has another kind");found=true;return {...layer,...{imageKey:mutation.imageKey,width:mutation.width,height:mutation.height}};}
    return layer.kind==="group"&&layer.children?{...layer,children:layer.children.map(visit)}:layer;
  };
  const layers=snapshot.layers.map(visit);
  if(!found)throw new Error("Image target is missing");
  return {...snapshot,layers};
}
